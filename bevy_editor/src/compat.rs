//! Compatibility layer for importing external non-Rust projects.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use bevy::{
    log::{error, info, warn},
    prelude::*,
};
use regex::Regex;
use walkdir::WalkDir;

use crate::LaunchOptions;

mod render;
mod script;

pub const COMPAT_PROJECT_ROOT_ARG: &str = "--project";

#[derive(Debug, Clone, Resource)]
pub struct CompatProjectResource {
    pub manifest: CompatProjectManifest,
    pub script_registry: ScriptRegistry,
    pub scenes: Vec<SceneSummary>,
}

#[derive(Debug, Clone)]
pub struct CompatProjectManifest {
    pub root: PathBuf,
    pub assets_root: PathBuf,
    pub scripts: Vec<PathBuf>,
    pub declarations: Vec<PathBuf>,
    pub prefabs: Vec<PrefabSummary>,
    pub scenes: Vec<PathBuf>,
    pub materials: Vec<PathBuf>,
    pub meshes: Vec<PathBuf>,
    pub animations: Vec<PathBuf>,
    pub skeletons: Vec<PathBuf>,
    pub navmeshes: Vec<PathBuf>,
    pub textures: Vec<PathBuf>,
    pub audio: Vec<PathBuf>,
    pub ui: Vec<PathBuf>,
    pub uuid_index: BTreeMap<String, PathBuf>,
}

#[derive(Debug, Clone)]
pub struct PrefabSummary {
    pub path: PathBuf,
    pub name: String,
    pub uuid: Option<String>,
    pub ts_components: Vec<String>,
    pub render_nodes: Vec<PrefabRenderNode>,
}

#[derive(Debug, Clone)]
pub struct PrefabRenderNode {
    pub name: String,
    pub transform: Transform,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptRegistry {
    pub script_files: Vec<PathBuf>,
    pub component_classes: Vec<String>,
    pub transpile_failures: Vec<String>,
    pub eval_failures: Vec<String>,
    pub script_host_summaries: Vec<ScriptHostSummary>,
}

#[derive(Debug, Clone)]
pub struct SceneSummary {
    pub path: PathBuf,
    pub name: String,
    pub nodes: Vec<CompatNode>,
}

#[derive(Debug, Clone)]
pub struct CompatNode {
    pub name: String,
    pub source: CompatNodeSource,
    pub script_components: Vec<String>,
    pub transform: Transform,
}

#[derive(Debug, Clone)]
pub enum CompatNodeSource {
    NativeSceneObject,
    PrefabInstance {
        prefab_path: Option<PathBuf>,
        prefab_name: Option<String>,
        prefab_uuid: Option<String>,
    },
}

#[derive(Component, Debug, Clone)]
pub struct CompatSceneRoot {
    pub path: PathBuf,
}

#[derive(Component, Debug, Clone)]
pub struct CompatNodeMarker {
    pub source_label: String,
}

#[derive(Component, Debug, Clone)]
pub struct CompatScriptList(pub Vec<String>);

#[derive(Debug, Clone)]
pub struct ScriptHostSummary {
    pub scene_name: String,
    pub node_name: String,
    pub components: Vec<String>,
}

pub struct CompatProjectPlugin;

impl Plugin for CompatProjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_compat_project)
            .add_systems(PostStartup, render::migrate_default_scene);
    }
}

fn initialize_compat_project(mut commands: Commands, launch_options: Option<Res<LaunchOptions>>) {
    let Some(root) = compat_project_root_from_launch_options(launch_options.as_deref())
        .or_else(compat_project_root_from_args)
    else {
        return;
    };

    match load_compat_project(&root) {
        Ok(resource) => {
            info!(
                "Loaded compatibility project '{}' with {} scripts, {} prefabs and {} scenes",
                resource.manifest.root.display(),
                resource.manifest.scripts.len(),
                resource.manifest.prefabs.len(),
                resource.manifest.scenes.len()
            );

            if !resource.script_registry.component_classes.is_empty() {
                info!(
                    "Discovered script component classes: {}",
                    resource.script_registry.component_classes.join(", ")
                );
            }

            for failure in &resource.script_registry.transpile_failures {
                warn!("{failure}");
            }

            for failure in &resource.script_registry.eval_failures {
                warn!("{failure}");
            }

            for host in &resource.script_registry.script_host_summaries {
                info!(
                    "Initialized script host [{}] {} => {}",
                    host.scene_name,
                    host.node_name,
                    host.components.join(", ")
                );
            }

            commands.insert_resource(resource);
        }
        Err(error) => {
            error!(
                "Failed to initialize compatibility project '{}': {error}",
                root.display()
            );
        }
    }
}

fn compat_project_root_from_launch_options(
    launch_options: Option<&LaunchOptions>,
) -> Option<PathBuf> {
    launch_options
        .and_then(|launch_options| launch_options.project_path.clone())
        .filter(|path| is_compat_project_root(path))
}

fn compat_project_root_from_args() -> Option<PathBuf> {
    let mut args = std::env::args_os();
    while let Some(arg) = args.next() {
        if arg == COMPAT_PROJECT_ROOT_ARG {
            return args.next().map(PathBuf::from);
        }
    }

    None
}

pub fn is_compat_project_root(path: &Path) -> bool {
    path.join("Assets").is_dir()
        && path.join("Assets").join("Scripts").is_dir()
        && path.join("ProjectSettings").is_dir()
        && path.join("tsconfig.json").is_file()
}

fn load_compat_project(root: &Path) -> std::io::Result<CompatProjectResource> {
    let manifest = scan_project(root)?;
    let scenes = build_scene_summaries(&manifest);
    let script_registry = script::build_script_registry(&manifest, &scenes);
    Ok(CompatProjectResource {
        manifest,
        script_registry,
        scenes,
    })
}

fn scan_project(root: &Path) -> std::io::Result<CompatProjectManifest> {
    let root = fs::canonicalize(root)?;
    let assets_root = root.join("Assets");
    let mut scripts = Vec::new();
    let mut declarations = Vec::new();
    let mut prefabs = Vec::new();
    let mut scenes = Vec::new();
    let mut materials = Vec::new();
    let mut meshes = Vec::new();
    let mut animations = Vec::new();
    let mut skeletons = Vec::new();
    let mut navmeshes = Vec::new();
    let mut textures = Vec::new();
    let mut audio = Vec::new();
    let mut ui = Vec::new();
    let mut uuid_index = BTreeMap::new();

    for entry in WalkDir::new(&root) {
        let Ok(entry) = entry else {
            continue;
        };
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let relative_path = match path.strip_prefix(&root) {
            Ok(relative) => relative.to_path_buf(),
            Err(_) => continue,
        };
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default();

        if extension == "meta"
            && let Some(asset_path) = asset_path_from_meta(path)
            && asset_path.is_file()
            && let Some(uuid) = load_uuid_from_meta_for_meta_path(path)
            && let Ok(relative_asset_path) = asset_path.strip_prefix(&assets_root)
        {
            uuid_index.insert(uuid, relative_asset_path.to_path_buf());
        }

        match extension {
            "ts" => {
                if relative_path.starts_with("Assets") {
                    scripts.push(relative_path);
                } else {
                    declarations.push(relative_path);
                }
            }
            "d.ts" => declarations.push(relative_path),
            "prefab" => {
                let bytes = fs::read(path)?;
                prefabs.push(PrefabSummary {
                    name: prefab_name_from_bytes(&bytes).unwrap_or_else(|| {
                        path.file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string()
                    }),
                    path: relative_path,
                    uuid: load_uuid_from_meta(path),
                    ts_components: extract_ts_component_names(&bytes),
                    render_nodes: extract_prefab_render_nodes_from_bytes(&bytes),
                });
            }
            "scene" => scenes.push(relative_path),
            "mat" => materials.push(relative_path),
            "mesh" => meshes.push(relative_path),
            "clip" => animations.push(relative_path),
            "skeleton" => skeletons.push(relative_path),
            "nmmesh" => navmeshes.push(relative_path),
            "ui" => ui.push(relative_path),
            "png" | "jpg" | "jpeg" | "tga" | "bmp" | "cubemap" => textures.push(relative_path),
            "wav" | "mp3" | "ogg" => audio.push(relative_path),
            _ => {}
        }
    }

    scripts.sort();
    declarations.sort();
    prefabs.sort_by(|left, right| left.path.cmp(&right.path));
    scenes.sort();
    materials.sort();
    meshes.sort();
    animations.sort();
    skeletons.sort();
    navmeshes.sort();
    textures.sort();
    audio.sort();
    ui.sort();

    Ok(CompatProjectManifest {
        root,
        assets_root,
        scripts,
        declarations,
        prefabs,
        scenes,
        materials,
        meshes,
        animations,
        skeletons,
        navmeshes,
        textures,
        audio,
        ui,
        uuid_index,
    })
}

fn build_scene_summaries(manifest: &CompatProjectManifest) -> Vec<SceneSummary> {
    let prefab_by_name = manifest
        .prefabs
        .iter()
        .map(|prefab| (normalize_lookup_key(&prefab.name), prefab))
        .collect::<BTreeMap<_, _>>();
    let prefab_by_uuid = manifest
        .prefabs
        .iter()
        .filter_map(|prefab| prefab.uuid.as_ref().map(|uuid| (uuid.clone(), prefab)))
        .collect::<BTreeMap<_, _>>();

    let mut scenes = Vec::new();
    for scene_path in &manifest.scenes {
        let absolute_path = manifest.root.join(scene_path);
        let Ok(bytes) = fs::read(&absolute_path) else {
            continue;
        };
        let scene_name = scene_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let nodes = extract_scene_nodes_from_bytes(&bytes, &prefab_by_name, &prefab_by_uuid);

        scenes.push(SceneSummary {
            path: scene_path.clone(),
            name: scene_name,
            nodes,
        });
    }

    scenes
}

fn extract_ts_component_names(bytes: &[u8]) -> Vec<String> {
    let mut strings = extract_readable_strings(bytes);
    if strings.is_empty() {
        return Vec::new();
    }

    strings.dedup();
    let mut components = BTreeSet::new();
    for pair in strings.windows(2) {
        if pair[0] == "com_type" && is_component_name(&pair[1]) {
            components.insert(pair[1].clone());
        }
    }

    components.into_iter().collect()
}

fn extract_prefab_render_nodes_from_bytes(bytes: &[u8]) -> Vec<PrefabRenderNode> {
    extract_game_object_nodes_from_bytes(bytes)
        .into_iter()
        .filter(|node| !node.name.is_empty())
        .map(|node| PrefabRenderNode {
            name: node.name,
            transform: node.transform,
        })
        .collect()
}

fn prefab_name_from_bytes(bytes: &[u8]) -> Option<String> {
    let strings = extract_readable_strings(bytes);
    strings.windows(2).find_map(|pair| {
        if pair[0] == "name" && !pair[1].starts_with("BLOCKMAN3.") {
            Some(pair[1].clone())
        } else {
            None
        }
    })
}

fn extract_readable_strings(bytes: &[u8]) -> Vec<String> {
    let mut current = String::new();
    let mut strings = Vec::new();
    for byte in bytes {
        let ch = *byte as char;
        if ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '/' | '\\' | '#' | '-' | ' ') {
            current.push(ch);
        } else if current.len() >= 3 {
            strings.push(current.trim().to_string());
            current.clear();
        } else {
            current.clear();
        }
    }

    if current.len() >= 3 {
        strings.push(current.trim().to_string());
    }

    strings
}

fn load_uuid_from_meta(path: &Path) -> Option<String> {
    let meta_path = path.with_extension(format!(
        "{}.meta",
        path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default()
    ));
    load_uuid_from_meta_for_meta_path(&meta_path)
}

fn load_uuid_from_meta_for_meta_path(meta_path: &Path) -> Option<String> {
    let text = fs::read_to_string(meta_path).ok()?;
    let uuid_pattern = Regex::new(r#""uuid"\s*:\s*"([A-Fa-f0-9]{32})""#).expect("uuid regex");
    uuid_pattern
        .captures(&text)
        .map(|capture| capture[1].to_ascii_uppercase())
}

fn asset_path_from_meta(meta_path: &Path) -> Option<PathBuf> {
    let file_name = meta_path.file_name()?.to_string_lossy();
    let asset_name = file_name.strip_suffix(".meta")?;
    Some(meta_path.with_file_name(asset_name))
}

fn is_component_name(value: &str) -> bool {
    !value.starts_with("BLOCKMAN3.")
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.'))
        && value.chars().any(|ch| ch.is_ascii_alphabetic())
}

fn extract_scene_nodes_from_bytes(
    bytes: &[u8],
    prefab_by_name: &BTreeMap<String, &PrefabSummary>,
    prefab_by_uuid: &BTreeMap<String, &PrefabSummary>,
) -> Vec<CompatNode> {
    let nodes = extract_game_object_nodes_from_bytes(bytes)
        .into_iter()
        .filter_map(|node| {
            let prefab_uuid = node.prefab_uuid;
            let prefab = prefab_uuid
                .as_ref()
                .and_then(|uuid| prefab_by_uuid.get(uuid).copied())
                .or_else(|| resolve_prefab_by_name(&node.name, prefab_by_name));

            let source = if let Some(prefab) = prefab {
                CompatNodeSource::PrefabInstance {
                    prefab_path: Some(prefab.path.clone()),
                    prefab_name: Some(prefab.name.clone()),
                    prefab_uuid: prefab.uuid.clone().or(prefab_uuid.clone()),
                }
            } else {
                CompatNodeSource::NativeSceneObject
            };

            let script_components = prefab
                .map(|prefab| prefab.ts_components.clone())
                .unwrap_or_default();

            Some(CompatNode {
                name: node.name,
                source,
                script_components,
                transform: node.transform,
            })
        })
        .collect();

    dedupe_nodes(nodes)
}

#[derive(Debug)]
struct RawGameObjectNode {
    name: String,
    transform: Transform,
    prefab_uuid: Option<String>,
}

fn extract_game_object_nodes_from_bytes(bytes: &[u8]) -> Vec<RawGameObjectNode> {
    let mut nodes = Vec::new();
    let mut cursor = 0;

    while let Some(type_offset) = find_bytes(bytes, b"BLOCKMAN3.GameObject", cursor) {
        let next_offset =
            find_bytes(bytes, b"BLOCKMAN3.GameObject", type_offset + 1).unwrap_or(bytes.len());
        let block = &bytes[type_offset..next_offset];
        cursor = next_offset;

        let Some(name) = read_string_field(block, b"name") else {
            continue;
        };
        if !should_capture_scene_name(&name) {
            continue;
        }

        let transform = Transform {
            translation: read_vec3_field(block, b"t").unwrap_or(Vec3::ZERO),
            rotation: read_quat_field(block, b"r").unwrap_or(Quat::IDENTITY),
            scale: read_vec3_field(block, b"s").unwrap_or(Vec3::ONE),
        };

        let prefab_uuid = read_uuid_after_field(block, b"id");

        nodes.push(RawGameObjectNode {
            name,
            transform,
            prefab_uuid,
        });
    }

    nodes
}

pub(super) fn find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    haystack
        .get(start..)?
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|offset| start + offset)
}

fn read_string_field(block: &[u8], field: &[u8]) -> Option<String> {
    let offset = find_field(block, field)?;
    let length_offset = offset + 1 + field.len();
    let length = read_u32_le(block, length_offset)? as usize;
    let value_start = length_offset + 4;
    let value = block.get(value_start..value_start + length)?;
    String::from_utf8(value.to_vec()).ok()
}

fn read_vec3_field(block: &[u8], field: &[u8]) -> Option<Vec3> {
    let offset = find_field(block, field)?;
    let value_start = offset + 1 + field.len() + 4;
    let x = read_f32_le(block, value_start)?;
    let y = read_f32_le(block, value_start + 4)?;
    let z = read_f32_le(block, value_start + 8)?;
    Some(Vec3::new(x, y, z))
}

fn read_quat_field(block: &[u8], field: &[u8]) -> Option<Quat> {
    let offset = find_field(block, field)?;
    let value_start = offset + 1 + field.len() + 4;
    let x = read_f32_le(block, value_start)?;
    let y = read_f32_le(block, value_start + 4)?;
    let z = read_f32_le(block, value_start + 8)?;
    let w = read_f32_le(block, value_start + 12)?;
    Some(Quat::from_xyzw(x, y, z, w).normalize())
}

fn read_uuid_after_field(block: &[u8], field: &[u8]) -> Option<String> {
    let offset = find_field(block, field)?;
    let length_offset = offset + 1 + field.len();
    let length = read_u32_le(block, length_offset)? as usize;
    if length != 16 {
        return None;
    }
    let value_start = length_offset + 4;
    let bytes = block.get(value_start..value_start + 16)?;
    Some(uuid_bytes_to_string(bytes))
}

pub(super) fn find_field(block: &[u8], field: &[u8]) -> Option<usize> {
    find_field_from(block, field, 0)
}

pub(super) fn find_field_from(block: &[u8], field: &[u8], start: usize) -> Option<usize> {
    block
        .get(start..)?
        .windows(field.len() + 1)
        .position(|window| window[0] as usize == field.len() && &window[1..] == field)
        .map(|offset| start + offset)
}

pub(super) fn read_u32_le(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        bytes.get(offset..offset + 4)?.try_into().ok()?,
    ))
}

pub(super) fn read_f32_le(bytes: &[u8], offset: usize) -> Option<f32> {
    Some(f32::from_le_bytes(
        bytes.get(offset..offset + 4)?.try_into().ok()?,
    ))
}

fn uuid_bytes_to_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<String>()
}

fn resolve_prefab_by_name<'a>(
    scene_object_name: &str,
    prefab_by_name: &'a BTreeMap<String, &'a PrefabSummary>,
) -> Option<&'a PrefabSummary> {
    let normalized = normalize_lookup_key(scene_object_name);
    prefab_by_name.get(&normalized).copied().or_else(|| {
        let simplified = strip_numeric_suffix(scene_object_name);
        prefab_by_name
            .get(&normalize_lookup_key(&simplified))
            .copied()
    })
}

pub(super) fn normalize_lookup_key(value: &str) -> String {
    value.to_ascii_lowercase().replace([' ', '-'], "")
}

fn strip_numeric_suffix(value: &str) -> String {
    if let Some((prefix, suffix)) = value.rsplit_once('_')
        && suffix.chars().all(|ch| ch.is_ascii_digit())
    {
        return prefix.to_string();
    }
    value.to_string()
}

fn should_capture_scene_name(value: &str) -> bool {
    !matches!(
        value,
        "enable"
            | "inst_id"
            | "file_id"
            | "layer"
            | "flags"
            | "component_count"
            | "child_count"
            | "data"
            | "type"
            | "sig"
            | "ptype"
            | "shared"
            | "state"
            | "exist"
            | "container"
            | "_name"
            | "_type"
            | "name"
    ) && !value.starts_with("BLOCKMAN3.")
        && value.chars().any(|ch| ch.is_ascii_alphabetic())
}

fn dedupe_nodes(nodes: Vec<CompatNode>) -> Vec<CompatNode> {
    let mut counts = BTreeMap::<String, usize>::new();
    let mut deduped = Vec::new();
    for node in nodes {
        let counter = counts.entry(node.name.clone()).or_default();
        *counter += 1;
        if *counter > 1 && !matches!(node.source, CompatNodeSource::PrefabInstance { .. }) {
            continue;
        }
        deduped.push(node);
    }
    deduped
}
