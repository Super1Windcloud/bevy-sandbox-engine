//! This module contains project management functionalities for Bevy Sandbox Engine.

use bevy::log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};
use templates::copy_template;
use toml::{Table, Value};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use crate::compat::{COMPAT_PROJECT_ROOT_ENV, is_compat_project_root};

mod cache;
pub mod templates;

/// Supported project kinds in the launcher.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProjectKind {
    #[default]
    Rust,
    BlockmanCompat,
}

/// Basic information about a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    /// The path to the root of the project.
    pub path: PathBuf,
    /// The detected project kind.
    #[serde(default)]
    pub kind: ProjectKind,
    /// Optional display name used by the launcher.
    #[serde(default)]
    pub display_name: Option<String>,
    /// The last time the project was opened.
    pub last_opened: SystemTime,
}

impl PartialEq for ProjectInfo {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl ProjectInfo {
    /// Get the name of the project.
    pub fn name(&self) -> Option<String> {
        self.display_name
            .clone()
            .or_else(|| Some(self.path.file_name()?.to_str()?.to_string()))
    }
}

/// Create a new project with the given name and path.
/// Copy the blank project template from the local templates folder
pub async fn create_new_project(
    template_id: String,
    path: PathBuf,
) -> std::io::Result<ProjectInfo> {
    let path = path;

    if let Err(error) = copy_template(&template_id, path.as_path()).await {
        error!("Failed to create new project");
        return Err(error);
    }

    if let Err(error) = rewrite_template_dependency_paths(path.as_path()) {
        error!("Failed to rewrite template dependency paths");
        return Err(error);
    }

    let kind = detect_project_kind(path.as_path()).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "Template `{template_id}` did not produce a supported project layout at {}",
                path.display()
            ),
        )
    })?;

    let info = ProjectInfo {
        path,
        kind,
        display_name: None,
        last_opened: SystemTime::now(),
    };

    let mut projects = get_local_projects();
    projects.push(info.clone());
    set_project_list(projects);

    Ok(info)
}

/// Get all projects that have been opened in the engine launcher.
pub fn get_local_projects() -> Vec<ProjectInfo> {
    let mut projects = cache::load_projects().unwrap_or_else(|error| {
        warn!("Failed to load projects from cache file: {:?}", error);
        Vec::new()
    });
    let original_len = projects.len();
    projects.retain_mut(|project| {
        if !project.path.is_dir() {
            return false;
        }

        let Some(kind) = detect_project_kind(&project.path) else {
            return false;
        };
        project.kind = kind;
        true
    });

    if projects.len() != original_len {
        if let Err(error) = cache::save_projects(projects.clone()) {
            error!("Couldn't prune missing projects from cache: {:?}", error);
        }
    }

    projects
}

/// Update the current project info or create new ones if doesn't exist.
pub fn update_project_info() {
    let mut projects = get_local_projects();
    let current_dir = std::env::current_dir().unwrap();

    match projects.iter_mut().find(|p| p.path == current_dir) {
        Some(project) => {
            // Update info
            project.last_opened = SystemTime::now();
            project.kind = detect_project_kind(&current_dir).unwrap_or(ProjectKind::Rust);
        }
        None => {
            // Create new info
            let project = ProjectInfo {
                path: current_dir.clone(),
                kind: detect_project_kind(&current_dir).unwrap_or(ProjectKind::Rust),
                display_name: None,
                last_opened: SystemTime::now(),
            };
            projects.push(project);
        }
    }

    if let Err(error) = cache::save_projects(projects) {
        error!("Couldn't update project info: {:?}", error);
    }
}

/// Set the project list to the given list of projects.
pub fn set_project_list(projects: Vec<ProjectInfo>) {
    if let Err(error) = cache::save_projects(projects) {
        error!("Unable to save project list: {:?}", error);
    }
}

/// Detect the project kind for a path.
pub fn detect_project_kind(path: &Path) -> Option<ProjectKind> {
    if is_rust_project(path) {
        Some(ProjectKind::Rust)
    } else if is_compat_project_root(path) {
        Some(ProjectKind::BlockmanCompat)
    } else {
        None
    }
}

/// Run a project in editor mode.
pub fn run_project(project: &ProjectInfo) -> std::io::Result<()> {
    match project.kind {
        ProjectKind::Rust => run_rust_project(project),
        ProjectKind::BlockmanCompat => run_compat_project(project),
    }
}

fn run_rust_project(project: &ProjectInfo) -> std::io::Result<()> {
    // Make sure the project folder exist
    if !project.path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Project root folder not found",
        ));
    }

    // Make sure it has the minimum file to be a valid project
    let cargo_toml = project.path.join("Cargo.toml");
    let src_folder = project.path.join("src");
    let main_rs = src_folder.join("main.rs");
    if !cargo_toml.exists() || !src_folder.exists() || !main_rs.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Project isn't a valid one of the following missing: Cargo.toml, src folder or main.rs file",
        ));
    }

    #[cfg(target_os = "windows")]
    const CREATE_NEW_CONSOLE: u32 = 0x0000_0010;

    let mut command = Command::new("cargo");
    command.current_dir(&project.path).arg("run");

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NEW_CONSOLE);

    let child = command
        .spawn()
        .map_err(|error| std::io::Error::other(format!("Failed to run project: {error}")))?;

    info!("Project process started successfully (pid: {})", child.id());
    Ok(())
}

fn run_compat_project(project: &ProjectInfo) -> std::io::Result<()> {
    if !project.path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Compatibility project root folder not found",
        ));
    }

    if !is_compat_project_root(&project.path) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Project isn't a valid compatibility project: missing Assets/Scripts, ProjectSettings or tsconfig.json",
        ));
    }

    #[cfg(target_os = "windows")]
    const CREATE_NEW_CONSOLE: u32 = 0x0000_0010;

    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| std::io::Error::other("Unable to locate workspace root"))?;

    let mut command = Command::new("cargo");
    command
        .current_dir(workspace_root)
        .arg("run")
        .arg("-p")
        .arg("bevy-sandbox-engine-launcher")
        .arg("--example")
        .arg("simple_editor")
        .env(COMPAT_PROJECT_ROOT_ENV, &project.path);

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NEW_CONSOLE);

    let child = command.spawn().map_err(|error| {
        std::io::Error::other(format!("Failed to run compatibility project: {error}"))
    })?;

    info!(
        "Compatibility project process started successfully (pid: {})",
        child.id()
    );
    Ok(())
}

fn is_rust_project(path: &Path) -> bool {
    path.join("Cargo.toml").exists()
        && path.join("src").is_dir()
        && path.join("src").join("main.rs").exists()
}

fn rewrite_template_dependency_paths(project_root: &Path) -> std::io::Result<()> {
    let cargo_toml_path = project_root.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        return Ok(());
    }

    let cargo_toml_text = std::fs::read_to_string(&cargo_toml_path)?;
    let mut cargo_toml = cargo_toml_text.parse::<Table>().map_err(|error| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse {}: {error}", cargo_toml_path.display()),
        )
    })?;

    let Some(Value::Table(dependencies)) = cargo_toml.get_mut("dependencies") else {
        return Ok(());
    };

    let Some(Value::Table(engine_dependency)) = dependencies.get_mut("bevy_sandbox_engine") else {
        return Ok(());
    };

    let engine_path = std::fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    engine_dependency.insert(
        "path".to_string(),
        Value::String(engine_path.display().to_string()),
    );

    let updated_text = toml::to_string_pretty(&cargo_toml).map_err(|error| {
        std::io::Error::other(format!("Failed to serialize Cargo.toml: {error}"))
    })?;
    std::fs::write(cargo_toml_path, updated_text)?;
    Ok(())
}
