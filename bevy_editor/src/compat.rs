//! Compatibility layer for importing external non-Rust projects.

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use bevy::{
    log::{error, info, warn},
    prelude::*,
};
use regex::Regex;
use rquickjs::{Context, Runtime, function::Func};
use walkdir::WalkDir;

pub const COMPAT_PROJECT_ROOT_ENV: &str = "BEVY_COMPAT_PROJECT_ROOT";

const QUICKJS_BOOTSTRAP: &str = r#"
globalThis.console = {
    log: (...args) => __rust_log("info", args.join(" ")),
    warn: (...args) => __rust_log("warn", args.join(" ")),
    error: (...args) => __rust_log("error", args.join(" "))
};

class EngineObject {
    constructor(name = "EngineObject") {
        this._name = name;
    }

    get name() {
        return this._name;
    }
}

class Vector3 {
    constructor(x = 0, y = 0, z = 0) {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    static get zero() {
        return new Vector3(0, 0, 0);
    }

    get magnitude() {
        return Math.sqrt(this.x * this.x + this.y * this.y + this.z * this.z);
    }

    Mul(scale) {
        return new Vector3(this.x * scale, this.y * scale, this.z * scale);
    }
}

class Quaternion {
    constructor(x = 0, y = 0, z = 0, w = 1) {
        this.x = x;
        this.y = y;
        this.z = z;
        this.w = w;
    }

    static LookRotation(_direction) {
        return new Quaternion();
    }

    static Slerp(from, to, t) {
        if (t <= 0) {
            return from;
        }

        if (t >= 1) {
            return to;
        }

        return to;
    }
}

class Transform extends EngineObject {
    constructor() {
        super("Transform");
        this.position = Vector3.zero;
        this.localPosition = Vector3.zero;
        this.localScale = new Vector3(1, 1, 1);
        this.rotation = new Quaternion();
        this.localRotation = new Quaternion();
        this.forward = new Vector3(0, 0, 1);
    }

    FindChild(_name) {
        return new Transform();
    }
}

class GameObject extends EngineObject {
    constructor(name = "GameObject") {
        super(name);
        this.transform = new Transform();
        this.transform.gameObject = this;
    }

    GetComponent(_type) {
        return null;
    }

    AddComponent(_type) {
        return null;
    }

    static Instantiate(_origin, _position, _rotation, _parent) {
        return new GameObject("Instantiated");
    }

    static DestroyGameObject(_go) {}
}

class Component extends EngineObject {
    constructor() {
        super("Component");
        this._gameObject = new GameObject("ScriptHost");
        this._transform = this._gameObject.transform;
    }

    get transform() {
        return this._transform;
    }

    get gameObject() {
        return this._gameObject;
    }
}

class CharacterController extends Component {
    SimpleMove(direction) {
        __rust_log("info", "CharacterController.SimpleMove " + JSON.stringify(direction));
    }
}

globalThis.Debug = {
    Log: (...args) => __rust_log("info", args.join(" ")),
    Warning: (...args) => __rust_log("warn", args.join(" ")),
    Error: (...args) => __rust_log("error", args.join(" "))
};

globalThis.EditorComponentSettings = {
    DecorateName: (_name) => () => {}
};

globalThis.GlobalEvent = {
    Instance: {
        Subscribe: () => {},
        UnSubscribe: () => {},
        Dispatch: () => {}
    }
};

globalThis.Time = {
    deltaTime: 1 / 60,
    fixedDeltaTime: 1 / 60
};
"#;

#[derive(Debug, Clone, Resource)]
pub struct CompatProjectResource {
    pub manifest: CompatProjectManifest,
    pub script_registry: ScriptRegistry,
}

#[derive(Debug, Clone)]
pub struct CompatProjectManifest {
    pub root: PathBuf,
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
}

#[derive(Debug, Clone)]
pub struct PrefabSummary {
    pub path: PathBuf,
    pub ts_components: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptRegistry {
    pub script_files: Vec<PathBuf>,
    pub component_classes: Vec<String>,
    pub transpile_failures: Vec<String>,
    pub eval_failures: Vec<String>,
}

pub struct CompatProjectPlugin;

impl Plugin for CompatProjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_compat_project);
    }
}

fn initialize_compat_project(mut commands: Commands) {
    let Some(root) = std::env::var_os(COMPAT_PROJECT_ROOT_ENV).map(PathBuf::from) else {
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

pub fn is_compat_project_root(path: &Path) -> bool {
    path.join("Assets").is_dir()
        && path.join("Assets").join("Scripts").is_dir()
        && path.join("ProjectSettings").is_dir()
        && path.join("tsconfig.json").is_file()
}

fn load_compat_project(root: &Path) -> std::io::Result<CompatProjectResource> {
    let manifest = scan_project(root)?;
    let script_registry = build_script_registry(&manifest);
    Ok(CompatProjectResource {
        manifest,
        script_registry,
    })
}

fn scan_project(root: &Path) -> std::io::Result<CompatProjectManifest> {
    let root = fs::canonicalize(root)?;
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
                    path: relative_path,
                    ts_components: extract_ts_component_names(&bytes),
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
    })
}

fn build_script_registry(manifest: &CompatProjectManifest) -> ScriptRegistry {
    let mut registry = ScriptRegistry {
        script_files: manifest.scripts.clone(),
        ..Default::default()
    };

    let runtime = match Runtime::new() {
        Ok(runtime) => runtime,
        Err(error) => {
            registry
                .eval_failures
                .push(format!("Failed to create QuickJS runtime: {error}"));
            return registry;
        }
    };
    let context = match Context::full(&runtime) {
        Ok(context) => context,
        Err(error) => {
            registry
                .eval_failures
                .push(format!("Failed to create QuickJS context: {error}"));
            return registry;
        }
    };

    if let Err(error) = context.with(|ctx| {
        let globals = ctx.globals();
        globals.set(
            "__rust_log",
            Func::from(|level: String, message: String| match level.as_str() {
                "error" => error!("ts> {message}"),
                "warn" => warn!("ts> {message}"),
                _ => info!("ts> {message}"),
            }),
        )?;
        ctx.eval::<(), _>(QUICKJS_BOOTSTRAP)?;
        Ok::<(), rquickjs::Error>(())
    }) {
        registry.eval_failures.push(format!(
            "Failed to bootstrap QuickJS compatibility runtime: {error}"
        ));
        return registry;
    }

    let mut discovered_classes = BTreeSet::new();
    for script_path in &manifest.scripts {
        let absolute_path = manifest.root.join(script_path);
        let source = match fs::read_to_string(&absolute_path) {
            Ok(source) => source,
            Err(error) => {
                registry.transpile_failures.push(format!(
                    "Failed to read script '{}': {error}",
                    absolute_path.display()
                ));
                continue;
            }
        };

        let transpiled = transpile_typescript_compat(&source);
        if !transpiled.converted {
            registry.transpile_failures.push(format!(
                "Script '{}' still contains unsupported TypeScript syntax after compatibility transpile",
                script_path.display()
            ));
        }

        for class_name in find_component_classes(&transpiled.code) {
            discovered_classes.insert(class_name);
        }

        let eval_result = context.with(|ctx| {
            ctx.eval::<(), _>(transpiled.code.as_str())?;
            Ok::<(), rquickjs::Error>(())
        });

        if let Err(error) = eval_result {
            registry.eval_failures.push(format!(
                "QuickJS failed to evaluate '{}': {error}",
                script_path.display()
            ));
        }
    }

    registry.component_classes = discovered_classes.into_iter().collect();
    registry
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

fn is_component_name(value: &str) -> bool {
    !value.starts_with("BLOCKMAN3.")
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.'))
        && value.chars().any(|ch| ch.is_ascii_alphabetic())
}

#[derive(Debug)]
struct TranspileOutput {
    code: String,
    converted: bool,
}

fn transpile_typescript_compat(source: &str) -> TranspileOutput {
    let mut code = source.replace('\u{feff}', "");

    let enum_pattern =
        Regex::new(r"(?s)\benum\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{(.*?)\}").expect("enum regex");
    code = enum_pattern
        .replace_all(&code, |captures: &regex::Captures<'_>| {
            transpile_enum_block(&captures[1], &captures[2])
        })
        .into_owned();

    let decorator_pattern = Regex::new(r"(?m)^[ \t]*@[^\r\n]+(?:\r?\n)?").expect("decorator regex");
    code = decorator_pattern.replace_all(&code, "").into_owned();

    let generic_call_pattern =
        Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)<[^>\r\n]+>\(").expect("generic call regex");
    code = generic_call_pattern.replace_all(&code, "$1(").into_owned();

    let class_generic_pattern =
        Regex::new(r"\b(class\s+[A-Za-z_][A-Za-z0-9_]*)<[^>\r\n]+>").expect("class generic regex");
    code = class_generic_pattern.replace_all(&code, "$1").into_owned();

    let implements_pattern = Regex::new(r"\s+implements\s+[^{]+").expect("implements regex");
    code = implements_pattern.replace_all(&code, "").into_owned();

    let modifier_pattern = Regex::new(r"\b(public|private|protected|readonly|declare|abstract)\s+")
        .expect("modifier regex");
    code = modifier_pattern.replace_all(&code, "").into_owned();

    let export_pattern = Regex::new(r"\bexport\s+").expect("export regex");
    code = export_pattern.replace_all(&code, "").into_owned();

    code = remove_type_annotations(&code);

    TranspileOutput {
        converted: !contains_obvious_typescript_syntax(&code),
        code,
    }
}

fn transpile_enum_block(name: &str, body: &str) -> String {
    let members = body
        .lines()
        .filter_map(|line| {
            let cleaned = line
                .split("//")
                .next()
                .unwrap_or_default()
                .trim()
                .trim_end_matches(',');
            if cleaned.is_empty() {
                return None;
            }

            cleaned
                .split_once('=')
                .map(|(member, value)| format!("{}: {}", member.trim(), value.trim()))
        })
        .collect::<Vec<_>>()
        .join(",\n    ");

    format!("const {name} = Object.freeze({{\n    {members}\n}});")
}

fn remove_type_annotations(input: &str) -> String {
    let chars = input.chars().collect::<Vec<_>>();
    let mut output = String::with_capacity(input.len());
    let mut index = 0;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut string_delim = '\0';

    while index < chars.len() {
        let current = chars[index];
        let next = chars.get(index + 1).copied();

        if in_line_comment {
            output.push(current);
            if current == '\n' {
                in_line_comment = false;
            }
            index += 1;
            continue;
        }

        if in_block_comment {
            output.push(current);
            if current == '*' && next == Some('/') {
                output.push('/');
                index += 2;
                in_block_comment = false;
            } else {
                index += 1;
            }
            continue;
        }

        if string_delim != '\0' {
            output.push(current);
            if current == '\\' {
                if let Some(next_char) = next {
                    output.push(next_char);
                    index += 2;
                } else {
                    index += 1;
                }
                continue;
            }
            if current == string_delim {
                string_delim = '\0';
            }
            index += 1;
            continue;
        }

        if current == '/' && next == Some('/') {
            output.push(current);
            output.push('/');
            index += 2;
            in_line_comment = true;
            continue;
        }

        if current == '/' && next == Some('*') {
            output.push(current);
            output.push('*');
            index += 2;
            in_block_comment = true;
            continue;
        }

        if matches!(current, '"' | '\'' | '`') {
            output.push(current);
            string_delim = current;
            index += 1;
            continue;
        }

        if current == ':' {
            let previous = last_significant_char(&output);
            let mut cursor = index + 1;
            while cursor < chars.len() && chars[cursor].is_whitespace() {
                cursor += 1;
            }

            if matches!(previous, Some(ch) if ch.is_ascii_alphanumeric() || matches!(ch, '_' | ')' | '?'))
                && let Some((end_index, valid_type)) = scan_type_annotation(&chars, cursor)
                && valid_type
            {
                index = end_index;
                continue;
            }
        }

        output.push(current);
        index += 1;
    }

    output
}

fn last_significant_char(text: &str) -> Option<char> {
    text.chars().rev().find(|ch| !ch.is_whitespace())
}

fn scan_type_annotation(chars: &[char], start: usize) -> Option<(usize, bool)> {
    let mut index = start;
    let mut angle_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut seen_content = false;

    while index < chars.len() {
        let current = chars[index];
        match current {
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '(' => paren_depth += 1,
            ')' => {
                if angle_depth == 0 && bracket_depth == 0 && paren_depth == 0 {
                    return Some((index, seen_content));
                }
                paren_depth = paren_depth.saturating_sub(1);
            }
            '=' | ';' | '{' | ',' => {
                if angle_depth == 0 && bracket_depth == 0 && paren_depth == 0 {
                    return Some((index, seen_content));
                }
            }
            '\n' => {
                if angle_depth == 0 && bracket_depth == 0 && paren_depth == 0 {
                    return Some((index, seen_content));
                }
            }
            '\'' | '"' | '`' | '!' => return None,
            _ => {}
        }

        if !current.is_whitespace() {
            if !(current.is_ascii_alphanumeric()
                || matches!(
                    current,
                    '_' | '.' | '|' | '&' | '?' | '[' | ']' | '<' | '>' | ',' | ' '
                ))
            {
                return None;
            }
            seen_content = true;
        }

        index += 1;
    }

    Some((index, seen_content))
}

fn contains_obvious_typescript_syntax(code: &str) -> bool {
    code.contains("@EditorComponentSettings")
        || code.contains(": boolean")
        || code.contains(": number")
        || code.contains(": string")
        || code.contains(": Vector3")
        || code.contains(": Quaternion")
        || code.contains(" as ")
}

fn find_component_classes(code: &str) -> Vec<String> {
    let pattern = Regex::new(r"\bclass\s+([A-Za-z_][A-Za-z0-9_]*)\s+extends\s+Component\b")
        .expect("class regex");
    pattern
        .captures_iter(code)
        .map(|capture| capture[1].to_string())
        .collect()
}
