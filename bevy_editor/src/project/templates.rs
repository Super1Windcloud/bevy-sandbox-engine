//! Module to handle Bevy Sandbox Engine project templates.

use std::path::{Path, PathBuf};

use serde::Deserialize;

/// The path to the folder containing the templates project
const TEMPLATE_FOLDER_PATH: &str = "project_templates/";
const TEMPLATE_MANIFEST_NAME: &str = "template.toml";
const TEMPLATE_IGNORED_DIRECTORIES: &[&str] = &["target"];

/// Template category displayed by the launcher.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateKind {
    /// Standard project template.
    Project,
    /// Mod-oriented template.
    Mod,
}

impl Default for TemplateKind {
    fn default() -> Self {
        Self::Project
    }
}

/// Preview illustration style used by the launcher card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplatePreviewStyle {
    /// Generic card preview.
    Generic,
    /// Minimal sandbox grid preview.
    Grid,
    /// Top-down shooter preview.
    Shooter,
}

impl Default for TemplatePreviewStyle {
    fn default() -> Self {
        Self::Generic
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A template discovered from the local `project_templates/` directory.
pub struct TemplateDefinition {
    /// Stable template identifier derived from the directory name.
    pub id: String,
    /// Absolute or workspace-relative path to the template directory.
    pub path: PathBuf,
    /// Launcher category for the template.
    pub kind: TemplateKind,
    /// Display title for Simplified Chinese users.
    pub title_zh: String,
    /// Display title for English users.
    pub title_en: String,
    /// Short description for Simplified Chinese users.
    pub subtitle_zh: String,
    /// Short description for English users.
    pub subtitle_en: String,
    /// Preview card top color in hex format.
    pub preview_top_color: Option<String>,
    /// Preview card bottom color in hex format.
    pub preview_bottom_color: Option<String>,
    /// Optional preview image path relative to the template directory.
    pub preview_image: Option<PathBuf>,
    /// Preview card illustration style.
    pub preview_style: TemplatePreviewStyle,
}

/// Enumerate all project templates available under the local `project_templates/` directory.
pub fn list_templates() -> std::io::Result<Vec<TemplateDefinition>> {
    let template_root = Path::new(TEMPLATE_FOLDER_PATH);
    let mut templates = Vec::new();

    for entry in std::fs::read_dir(template_root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let Some(id) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        templates.push(read_template_definition(id.to_string(), path)?);
    }

    templates.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(templates)
}

pub(super) async fn copy_template(template_id: &str, to: &Path) -> std::io::Result<()> {
    let template_path = Path::new(TEMPLATE_FOLDER_PATH).join(template_id);
    clone_directory(template_path, to)?;
    Ok(())
}

#[derive(Debug, Default, Deserialize)]
struct TemplateManifest {
    kind: TemplateKind,
    title_zh: Option<String>,
    title_en: Option<String>,
    subtitle_zh: Option<String>,
    subtitle_en: Option<String>,
    preview_top_color: Option<String>,
    preview_bottom_color: Option<String>,
    preview_image: Option<String>,
    preview_style: TemplatePreviewStyle,
}

fn read_template_definition(id: String, path: PathBuf) -> std::io::Result<TemplateDefinition> {
    let manifest_path = path.join(TEMPLATE_MANIFEST_NAME);
    let manifest = if manifest_path.exists() {
        let manifest_text = std::fs::read_to_string(&manifest_path)?;
        toml::from_str::<TemplateManifest>(&manifest_text).map_err(|error| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid template manifest {}: {error}",
                    manifest_path.display()
                ),
            )
        })?
    } else {
        TemplateManifest::default()
    };

    let default_title = title_case_from_id(&id);
    let preview_image = manifest
        .preview_image
        .as_deref()
        .map(|relative_path| path.join(relative_path));

    Ok(TemplateDefinition {
        id,
        path,
        kind: manifest.kind,
        title_zh: manifest.title_zh.unwrap_or_else(|| default_title.clone()),
        title_en: manifest.title_en.unwrap_or_else(|| default_title.clone()),
        subtitle_zh: manifest
            .subtitle_zh
            .unwrap_or_else(|| "从 project_templates 目录自动加载。".to_string()),
        subtitle_en: manifest
            .subtitle_en
            .unwrap_or_else(|| "Auto-loaded from the project_templates directory.".to_string()),
        preview_top_color: manifest.preview_top_color,
        preview_bottom_color: manifest.preview_bottom_color,
        preview_image,
        preview_style: manifest.preview_style,
    })
}

fn title_case_from_id(id: &str) -> String {
    id.split(['_', '-'])
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => {
                    let mut word = first.to_uppercase().to_string();
                    word.push_str(chars.as_str());
                    word
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn clone_directory<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> std::io::Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        let file_name_str = file_name.to_string_lossy();

        if path.is_dir() && TEMPLATE_IGNORED_DIRECTORIES.contains(&file_name_str.as_ref()) {
            continue;
        }

        let new_path = to.join(file_name);
        if path.is_dir() {
            clone_directory(path, new_path)?;
        } else {
            std::fs::copy(path, new_path)?;
        }
    }
    Ok(())
}
