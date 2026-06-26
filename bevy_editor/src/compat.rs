//! Compatibility layer for importing external non-Rust projects.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::tailwind,
    log::{error, info, warn},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use regex::Regex;
use rquickjs::{Context, Runtime, function::Func};
use serde_json::{Value, json};
use walkdir::WalkDir;

use crate::LaunchOptions;

pub const COMPAT_PROJECT_ROOT_ARG: &str = "--project";

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

    get normalized() {
        const length = this.magnitude;
        if (length === 0) {
            return new Vector3();
        }

        return new Vector3(this.x / length, this.y / length, this.z / length);
    }

    Mul(scale) {
        return new Vector3(this.x * scale, this.y * scale, this.z * scale);
    }

    Add(other) {
        return new Vector3(this.x + other.x, this.y + other.y, this.z + other.z);
    }

    EqualsTo(other) {
        return this.x === other.x && this.y === other.y && this.z === other.z;
    }

    static Distance(a, b) {
        return new Vector3(a.x - b.x, a.y - b.y, a.z - b.z).magnitude;
    }
}

class Vector2 {
    constructor(x = 0, y = 0) {
        this.x = x;
        this.y = y;
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

    static get identity() {
        return new Quaternion();
    }

    static FromEuler(_euler) {
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
        this.parent = null;
        this.children = [];
        this.gameObject = null;
    }

    get childCount() {
        return this.children.length;
    }

    FindChild(name) {
        return this.children.find((child) => child.name === name) ?? new Transform();
    }

    GetChild(index) {
        return this.children[index] ?? new Transform();
    }

    addChild(transform) {
        transform.parent = this;
        this.children.push(transform);
    }

    SetParent(parent, _keepWorldPosition = false) {
        if (parent) {
            parent.addChild(this);
        }
    }

    LookAt(_pos, _up = new Vector3(0, 1, 0)) {}

    SetPositionAndRotation(pos, rot) {
        this.position = pos;
        this.rotation = rot;
    }
}

class GameObject extends EngineObject {
    constructor(name = "GameObject") {
        super(name);
        this.transform = new Transform();
        this.transform.gameObject = this;
        this.transform.name = name;
        this.enable = true;
        this._components = new Map();
    }

    GetComponent(type) {
        const typeName = typeof type === "string" ? type : type?.name;
        if (!typeName) {
            return null;
        }

        if (this._components.has(typeName)) {
            return this._components.get(typeName);
        }

        const fallback = __compat.createNativeComponent(typeName, this);
        if (fallback) {
            this._components.set(typeName, fallback);
            return fallback;
        }

        return null;
    }

    AddComponent(type) {
        const typeName = typeof type === "string" ? type : type?.name;
        const ctor = globalThis[typeName];
        if (typeof ctor !== "function") {
            return null;
        }

        const instance = new ctor();
        __compat.attachComponent(this, instance, typeName);
        return instance;
    }

    static Instantiate(_origin, _position, _rotation, _parent) {
        return new GameObject("Instantiated");
    }

    static DestroyGameObject(_go) {}

    static DestroyComponent(component) {
        if (component) {
            component.enable = false;
        }
    }
}

class Component extends EngineObject {
    constructor() {
        super("Component");
        this._gameObject = new GameObject("ScriptHost");
        this._transform = this._gameObject.transform;
        this.enable = true;
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

class CharacterHealth extends Component {
    constructor() {
        super();
        this.IsDead = false;
    }

    RefresHealth() {
        this.IsDead = false;
    }
}

class Ray {
    constructor() {
        this.direction = new Vector3(0, 0, 1);
    }
}

class Camera extends Component {
    ScreenPointToRay(_point) {
        return new Ray();
    }

    WorldPointToScreen(point) {
        return new Vector3(point.x, point.y, point.z);
    }

    static get mainCamera() {
        if (!Camera.__mainCamera) {
            Camera.__mainCamera = new Camera();
        }
        return Camera.__mainCamera;
    }
}

class RectTransform extends Transform {}

class Image extends Component {
    constructor() {
        super();
        this.rectTransform = new RectTransform();
    }
}

class UIComponent extends Component {
    constructor() {
        super();
        this.canvas = {
            FindChild: (_type, name) => {
                const image = new Image();
                image.name = name;
                return image;
            }
        };
    }
}

class ParticleSystem extends Component {
    Play() {}
}

class AudioSource extends Component {
    Play() {}
}

class FABRIK extends Component {
    constructor() {
        super();
        this.target = null;
    }
}

class Prefab extends EngineObject {
    constructor(name = "Prefab") {
        super(name);
    }

    Instance() {
        return new GameObject(`${this.name}_Instance`);
    }
}

class List {
    constructor(_type) {
        this._items = [];
    }

    get count() {
        return this._items.length;
    }

    Add(value) {
        this._items.push(value);
    }

    get(index) {
        return this._items[index];
    }
}

class Input {
    static get mousePosition() {
        return new Vector2(0, 0);
    }

    static GetKey(_key) {
        return false;
    }

    static GetKeyDown(_key) {
        return false;
    }

    static GetKeyUp(_key) {
        return false;
    }

    static GetMouseButton(_button) {
        return false;
    }

    static GetMouseButtonDown(_button) {
        return false;
    }

    static GetMouseButtonUp(_button) {
        return false;
    }
}

globalThis.KeyCode = {
    W: "W",
    A: "A",
    S: "S",
    D: "D"
};

globalThis.MouseButton = {
    LeftButton: "LeftButton"
};

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

globalThis.__compat = {
    attachComponent(gameObject, component, typeName) {
        component._gameObject = gameObject;
        component._transform = gameObject.transform;
        component.name = typeName;
        gameObject._components.set(typeName, component);
    },

    createNativeComponent(typeName, gameObject) {
        const ctor = globalThis[typeName];
        if (typeof ctor !== "function") {
            return null;
        }

        if (!(ctor.prototype instanceof Component)) {
            return null;
        }

        const instance = new ctor();
        this.attachComponent(gameObject, instance, typeName);
        return instance;
    },

    createNodeHost(nodeName, componentNames) {
        const gameObject = new GameObject(nodeName);
        const created = [];
        for (const componentName of componentNames) {
            const ctor = globalThis[componentName];
            if (typeof ctor !== "function") {
                throw new Error(`Missing script class: ${componentName}`);
            }

            const instance = new ctor();
            this.attachComponent(gameObject, instance, componentName);
            created.push(instance);
        }

        return { gameObject, created };
    },

    initializeNodeHost(host) {
        for (const instance of host.created) {
            if (typeof instance.OnEnable === "function") {
                instance.OnEnable();
            }
        }

        for (const instance of host.created) {
            if (typeof instance.OnStart === "function") {
                instance.OnStart();
            }
        }

        return host;
    }
};
"#;

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
            .add_systems(PostStartup, migrate_default_scene);
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
    let script_registry = build_script_registry(&manifest, &scenes);
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

fn build_script_registry(
    manifest: &CompatProjectManifest,
    scenes: &[SceneSummary],
) -> ScriptRegistry {
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
    registry.script_host_summaries = initialize_script_hosts(&context, scenes, &mut registry);
    registry
}

fn initialize_script_hosts(
    context: &Context,
    scenes: &[SceneSummary],
    registry: &mut ScriptRegistry,
) -> Vec<ScriptHostSummary> {
    let mut summaries = Vec::new();

    for scene in scenes {
        for node in &scene.nodes {
            if node.script_components.is_empty() {
                continue;
            }

            let expression = json!({
                "node": node.name,
                "components": node.script_components,
            })
            .to_string();

            let eval_result = context.with(|ctx| {
                let script = format!(
                    "(function(payload) {{
                        const host = __compat.createNodeHost(payload.node, payload.components);
                        __compat.initializeNodeHost(host);
                    }})({expression});"
                );
                ctx.eval::<(), _>(script.as_str())?;
                Ok::<(), rquickjs::Error>(())
            });

            match eval_result {
                Ok(()) => summaries.push(ScriptHostSummary {
                    scene_name: scene.name.clone(),
                    node_name: node.name.clone(),
                    components: node.script_components.clone(),
                }),
                Err(error) => registry.eval_failures.push(format!(
                    "QuickJS failed to initialize node host '{}:{}': {error}",
                    scene.name, node.name
                )),
            }
        }
    }

    summaries
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

fn find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
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

fn find_field(block: &[u8], field: &[u8]) -> Option<usize> {
    block
        .windows(field.len() + 1)
        .position(|window| window[0] as usize == field.len() && &window[1..] == field)
}

fn read_u32_le(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        bytes.get(offset..offset + 4)?.try_into().ok()?,
    ))
}

fn read_f32_le(bytes: &[u8], offset: usize) -> Option<f32> {
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

fn normalize_lookup_key(value: &str) -> String {
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

fn migrate_default_scene(
    mut commands: Commands,
    compat_project: Option<Res<CompatProjectResource>>,
    existing_roots: Query<Entity, With<CompatSceneRoot>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let Some(compat_project) = compat_project else {
        return;
    };
    if !existing_roots.is_empty() {
        return;
    }

    let Some(scene) = compat_project
        .scenes
        .iter()
        .find(|scene| scene.name.eq_ignore_ascii_case("DefaultScene"))
        .or_else(|| compat_project.scenes.first())
    else {
        return;
    };

    let root = commands
        .spawn((
            Name::new(format!("CompatScene: {}", scene.name)),
            CompatSceneRoot {
                path: scene.path.clone(),
            },
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    for node in &scene.nodes {
        let source_label = match &node.source {
            CompatNodeSource::NativeSceneObject => "scene-object".to_string(),
            CompatNodeSource::PrefabInstance {
                prefab_path,
                prefab_name,
                prefab_uuid,
            } => format!(
                "prefab:{}:{}:{}",
                prefab_name.clone().unwrap_or_else(|| "unknown".to_string()),
                prefab_path
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "unresolved".to_string()),
                prefab_uuid.clone().unwrap_or_else(|| "no-uuid".to_string())
            ),
        };

        let entity = commands
            .spawn((
                Name::new(node.name.clone()),
                CompatNodeMarker { source_label },
                CompatScriptList(node.script_components.clone()),
                node.transform,
                Visibility::default(),
            ))
            .id();

        commands.entity(root).add_child(entity);

        let spawned_render_assets = spawn_prefab_render_assets(
            &mut commands,
            &compat_project.manifest,
            node,
            entity,
            &mut meshes,
            &mut materials,
            &asset_server,
        );

        if !spawned_render_assets {
            let color = if node.script_components.is_empty() {
                tailwind::SLATE_500
            } else {
                tailwind::EMERALD_500
            };
            commands.entity(entity).insert((
                Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(0.35)))),
                MeshMaterial3d(materials.add(Color::from(color))),
            ));
        }
    }

    info!(
        "Migrated compatibility scene '{}' into {} Bevy placeholder entities",
        scene.path.display(),
        scene.nodes.len()
    );
}

fn spawn_prefab_render_assets(
    commands: &mut Commands,
    manifest: &CompatProjectManifest,
    node: &CompatNode,
    parent: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
) -> bool {
    let CompatNodeSource::PrefabInstance {
        prefab_path: Some(prefab_path),
        ..
    } = &node.source
    else {
        return false;
    };

    let Some(prefab) = manifest
        .prefabs
        .iter()
        .find(|prefab| &prefab.path == prefab_path)
    else {
        return false;
    };

    let prefab_asset_dir = prefab_path.parent().unwrap_or_else(|| Path::new(""));
    let mesh_dir = prefab_asset_dir.join("Mesh");
    let material_dir = prefab_asset_dir.join("Material");
    let mut spawned_any = false;

    for render_node in &prefab.render_nodes {
        let Some(mesh_path) =
            find_named_asset(&manifest.assets_root, &mesh_dir, &render_node.name, "mesh")
        else {
            continue;
        };

        let absolute_mesh_path = manifest.assets_root.join(&mesh_path);
        let Some(mesh) = load_compat_mesh(&absolute_mesh_path) else {
            warn!(
                "Failed to decode compatibility mesh '{}'",
                absolute_mesh_path.display()
            );
            continue;
        };

        let material = find_named_asset(
            &manifest.assets_root,
            &material_dir,
            &render_node.name,
            "mat",
        )
        .and_then(|path| load_compat_material(manifest, &path, materials, asset_server))
        .unwrap_or_else(|| materials.add(default_preview_material(&render_node.name)));

        commands.entity(parent).with_children(|parent| {
            parent.spawn((
                Name::new(format!("{} mesh", render_node.name)),
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material),
                render_node.transform,
                Visibility::default(),
            ));
        });
        spawned_any = true;
    }

    spawned_any
}

fn find_named_asset(
    assets_root: &Path,
    preferred_dir: &Path,
    name: &str,
    extension: &str,
) -> Option<PathBuf> {
    let direct = preferred_dir.join(format!("{name}.{extension}"));
    if assets_root.join(&direct).is_file() {
        return Some(direct);
    }

    let normalized_name = normalize_lookup_key(name);
    WalkDir::new(assets_root)
        .into_iter()
        .filter_map(Result::ok)
        .find_map(|entry| {
            if !entry.file_type().is_file() {
                return None;
            }
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some(extension) {
                return None;
            }
            let stem = path.file_stem()?.to_string_lossy();
            if normalize_lookup_key(&stem) == normalized_name {
                path.strip_prefix(assets_root).ok().map(Path::to_path_buf)
            } else {
                None
            }
        })
}

fn default_preview_material(name: &str) -> StandardMaterial {
    let palette = [
        tailwind::SKY_500,
        tailwind::EMERALD_500,
        tailwind::AMBER_500,
        tailwind::ROSE_500,
        tailwind::VIOLET_500,
    ];
    let hash = name.bytes().fold(0usize, |acc, byte| {
        acc.wrapping_mul(31).wrapping_add(byte as usize)
    });
    StandardMaterial {
        base_color: Color::from(palette[hash % palette.len()]),
        perceptual_roughness: 0.8,
        ..default()
    }
}

fn load_compat_material(
    manifest: &CompatProjectManifest,
    relative_path: &Path,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
) -> Option<Handle<StandardMaterial>> {
    let text = fs::read_to_string(manifest.assets_root.join(relative_path)).ok()?;
    let value = serde_json::from_str::<Value>(&text).ok()?;
    let base_color = material_color(&value).unwrap_or(Color::WHITE);
    let texture_path = material_main_texture_path(manifest, &value);
    let base_color_texture = texture_path.map(|path| {
        let asset_path = path.to_string_lossy().replace('\\', "/");
        asset_server.load(asset_path)
    });

    Some(materials.add(StandardMaterial {
        base_color,
        base_color_texture,
        perceptual_roughness: 0.8,
        ..default()
    }))
}

fn material_color(value: &Value) -> Option<Color> {
    let vectors = value.get("vectors")?.as_array()?;
    let color = vectors.iter().find(|entry| {
        entry
            .get("name")
            .and_then(Value::as_str)
            .is_some_and(|name| name == "_Color")
    })?;
    let rgba = color.get("value")?.as_array()?;
    let r = rgba.first()?.as_f64()? as f32;
    let g = rgba.get(1)?.as_f64()? as f32;
    let b = rgba.get(2)?.as_f64()? as f32;
    let a = rgba.get(3).and_then(Value::as_f64).unwrap_or(1.0) as f32;
    Some(Color::srgba(r, g, b, a))
}

fn material_main_texture_path(manifest: &CompatProjectManifest, value: &Value) -> Option<PathBuf> {
    let textures = value.get("textures")?.as_array()?;
    let texture = textures.iter().find(|entry| {
        entry
            .get("name")
            .and_then(Value::as_str)
            .is_some_and(|name| name == "_MainTex")
    })?;
    let uuid = texture
        .get("value")?
        .get("id")?
        .as_str()?
        .to_ascii_uppercase();
    manifest.uuid_index.get(&uuid).cloned()
}

fn load_compat_mesh(path: &Path) -> Option<Mesh> {
    let bytes = fs::read(path).ok()?;
    let indices = read_mesh_indices(&bytes)?;
    let vertex_data_start = find_bytes(&bytes, b"vertex_data", 0)?;
    let data_field = find_bytes(&bytes, b"data", vertex_data_start)?;
    let vertex_bytes_start = data_field + b"data".len() + 4;
    let max_index = indices.iter().copied().max()? as usize;
    let vertex_count = max_index + 1;

    find_positions_in_vertex_blob(&bytes[vertex_bytes_start..], vertex_count, &indices)
        .map(|positions| mesh_from_positions_and_indices(positions, indices))
}

fn read_mesh_indices(bytes: &[u8]) -> Option<Vec<u32>> {
    let field_offset = find_field(bytes, b"indices")?;
    let byte_len_offset = field_offset + 1 + b"indices".len();
    let byte_len = read_u32_le(bytes, byte_len_offset)? as usize;
    let data_start = byte_len_offset + 8;
    let index_byte_len = byte_len.checked_sub(4)?;
    let data = bytes.get(data_start..data_start + index_byte_len)?;
    if data.len() % 2 != 0 {
        return None;
    }
    Some(
        data.chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]) as u32)
            .collect(),
    )
}

fn find_positions_in_vertex_blob(
    blob: &[u8],
    vertex_count: usize,
    indices: &[u32],
) -> Option<Vec<[f32; 3]>> {
    let byte_len = vertex_count.checked_mul(12)?;
    if byte_len > blob.len() {
        return None;
    }

    let mut best: Option<(usize, Vec<[f32; 3]>, f32)> = None;
    let search_len = blob.len().saturating_sub(byte_len).min(4096);
    for offset in (0..search_len).step_by(4) {
        let Some(positions) = read_position_candidate(blob, offset, vertex_count) else {
            continue;
        };
        let score = score_position_candidate(&positions, indices);
        if score <= 0.0 {
            continue;
        }
        if best
            .as_ref()
            .is_none_or(|(_, _, best_score)| score > *best_score)
        {
            best = Some((offset, positions, score));
        }
    }

    best.map(|(_, positions, _)| positions)
}

fn read_position_candidate(
    blob: &[u8],
    offset: usize,
    vertex_count: usize,
) -> Option<Vec<[f32; 3]>> {
    let mut positions = Vec::with_capacity(vertex_count);
    for vertex in 0..vertex_count {
        let base = offset + vertex * 12;
        let x = read_f32_le(blob, base)?;
        let y = read_f32_le(blob, base + 4)?;
        let z = read_f32_le(blob, base + 8)?;
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return None;
        }
        positions.push([x, y, z]);
    }
    Some(positions)
}

fn score_position_candidate(positions: &[[f32; 3]], indices: &[u32]) -> f32 {
    if positions.is_empty() {
        return 0.0;
    }

    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);
    let mut finite_count = 0usize;
    for position in positions {
        let p = Vec3::from_array(*position);
        if p.length_squared() > 10_000.0 {
            return 0.0;
        }
        min = min.min(p);
        max = max.max(p);
        finite_count += 1;
    }

    let extent = max - min;
    if extent.min_element() < 0.0001 || extent.max_element() > 100.0 {
        return 0.0;
    }

    let mut triangle_area = 0.0;
    let mut triangle_count = 0usize;
    for tri in indices.chunks_exact(3).take(128) {
        let a = Vec3::from_array(positions[tri[0] as usize]);
        let b = Vec3::from_array(positions[tri[1] as usize]);
        let c = Vec3::from_array(positions[tri[2] as usize]);
        triangle_area += (b - a).cross(c - a).length();
        triangle_count += 1;
    }

    if triangle_count == 0 || triangle_area <= 0.0001 {
        return 0.0;
    }

    finite_count as f32 + triangle_area.min(1000.0)
}

fn mesh_from_positions_and_indices(positions: Vec<[f32; 3]>, indices: Vec<u32>) -> Mesh {
    let normals = generate_normals(&positions, &indices);
    let uvs = vec![[0.0, 0.0]; positions.len()];
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn generate_normals(positions: &[[f32; 3]], indices: &[u32]) -> Vec<[f32; 3]> {
    let mut normals = vec![Vec3::ZERO; positions.len()];
    for tri in indices.chunks_exact(3) {
        let ia = tri[0] as usize;
        let ib = tri[1] as usize;
        let ic = tri[2] as usize;
        if ia >= positions.len() || ib >= positions.len() || ic >= positions.len() {
            continue;
        }
        let a = Vec3::from_array(positions[ia]);
        let b = Vec3::from_array(positions[ib]);
        let c = Vec3::from_array(positions[ic]);
        let normal = (b - a).cross(c - a);
        normals[ia] += normal;
        normals[ib] += normal;
        normals[ic] += normal;
    }

    normals
        .into_iter()
        .map(|normal| normal.try_normalize().unwrap_or(Vec3::Y).to_array())
        .collect()
}
