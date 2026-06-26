use std::{collections::BTreeSet, fs, path::PathBuf};

use bevy::log::{error, info, warn};
use regex::Regex;
use rquickjs::{CatchResultExt, Context, Runtime, function::Func};
use serde_json::json;
#[cfg(test)]
use walkdir::WalkDir;

use super::{CompatProjectManifest, SceneSummary, ScriptHostSummary, ScriptRegistry};

struct PreparedScript {
    path: PathBuf,
    code: String,
    classes: Vec<String>,
    base_classes: Vec<String>,
}

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

pub(super) fn build_script_registry(
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

    let mut prepared_scripts = Vec::new();
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

        prepared_scripts.push(PreparedScript {
            path: script_path.clone(),
            classes: find_declared_classes(&transpiled.code),
            base_classes: find_base_classes(&transpiled.code),
            code: transpiled.code,
        });
    }

    let mut discovered_classes = BTreeSet::new();
    for script in &prepared_scripts {
        for class_name in find_component_classes(&script.code) {
            discovered_classes.insert(class_name);
        }
    }

    let prepared_scripts = order_scripts_by_class_dependencies(prepared_scripts);

    for script in &prepared_scripts {
        let eval_result = context.with(|ctx| {
            ctx.eval::<(), _>(script.code.as_str())
                .catch(&ctx)
                .map_err(|error| error.to_string())
        });

        if let Err(error) = eval_result {
            registry.eval_failures.push(format!(
                "QuickJS failed to evaluate '{}': {error}",
                script.path.display()
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
                ctx.eval::<(), _>(script.as_str())
                    .catch(&ctx)
                    .map_err(|error| error.to_string())
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

    let type_alias_pattern =
        Regex::new(r"(?m)^[ \t]*type\s+[A-Za-z_][A-Za-z0-9_]*\s*=.*?;[ \t]*(?:\r?\n)?")
            .expect("type alias regex");
    code = type_alias_pattern.replace_all(&code, "").into_owned();

    let definite_assignment_pattern =
        Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)!\s*:").expect("definite assignment regex");
    code = definite_assignment_pattern
        .replace_all(&code, "$1:")
        .into_owned();

    let optional_annotation_pattern =
        Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)\?\s*:").expect("optional annotation regex");
    code = optional_annotation_pattern
        .replace_all(&code, "$1:")
        .into_owned();

    let abstract_method_pattern =
        Regex::new(r"(?m)^([ \t]*)(?:protected|public|private)?\s*abstract\s+([A-Za-z_][A-Za-z0-9_]*)\s*(\([^;\r\n]*\))\s*:\s*[^;\r\n]+;")
            .expect("abstract method regex");
    code = abstract_method_pattern
        .replace_all(&code, "$1$2$3 {}")
        .into_owned();

    let new_generic_pattern =
        Regex::new(r"\bnew\s+([A-Za-z_][A-Za-z0-9_]*)<[^>\r\n]+>").expect("new generic regex");
    code = new_generic_pattern
        .replace_all(&code, "new $1")
        .into_owned();

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

    code = remove_initializer_type_annotations(&code);
    code = remove_type_annotations(&code);
    code = remove_as_assertions(&code);
    code = remove_non_null_assertions(&code);
    code = strip_comments(&code);

    TranspileOutput {
        converted: !contains_obvious_typescript_syntax(&code),
        code,
    }
}

fn remove_initializer_type_annotations(input: &str) -> String {
    let mut code = input.to_string();
    let map_initializer_pattern =
        Regex::new(r":\s*Map<[^=\r\n]+>\s*=\s*new\s+Map\b").expect("map initializer regex");
    code = map_initializer_pattern
        .replace_all(&code, " = new Map")
        .into_owned();

    let array_initializer_pattern =
        Regex::new(r":\s*Array<[^=\r\n]+>\s*=\s*new\s+Array\b").expect("array initializer regex");
    array_initializer_pattern
        .replace_all(&code, " = new Array")
        .into_owned()
}

fn remove_as_assertions(input: &str) -> String {
    let as_pattern = Regex::new(r"\s+as\s+[A-Za-z_][A-Za-z0-9_]*(?:\[\])?").expect("as regex");
    as_pattern.replace_all(input, "").into_owned()
}

fn remove_non_null_assertions(input: &str) -> String {
    let non_null_pattern = Regex::new(r"([A-Za-z0-9_\]\)])!").expect("non-null regex");
    non_null_pattern.replace_all(input, "$1").into_owned()
}

fn strip_comments(input: &str) -> String {
    let block_comment_pattern = Regex::new(r"(?s)/\*.*?\*/").expect("block comment regex");
    let code = block_comment_pattern.replace_all(input, "");
    let line_comment_pattern = Regex::new(r"(?m)//[^\r\n]*").expect("line comment regex");
    line_comment_pattern.replace_all(&code, "").into_owned()
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

            if chars
                .get(cursor)
                .is_some_and(|ch| ch.is_ascii_digit() || matches!(ch, '-' | '('))
            {
                output.push(current);
                index += 1;
                continue;
            }

            if matches!(previous, Some(ch) if ch.is_ascii_alphanumeric() || matches!(ch, '_' | ')'))
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
    let mut brace_depth = 0usize;
    let mut seen_content = false;

    while index < chars.len() {
        let current = chars[index];
        match current {
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '{' => {
                if angle_depth == 0 && bracket_depth == 0 && paren_depth == 0 && seen_content {
                    return Some((index, seen_content));
                }
                brace_depth += 1;
            }
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '(' => paren_depth += 1,
            ')' => {
                if angle_depth == 0 && bracket_depth == 0 && paren_depth == 0 && brace_depth == 0 {
                    return Some((index, seen_content));
                }
                paren_depth = paren_depth.saturating_sub(1);
            }
            '=' | ';' | ',' => {
                if angle_depth == 0 && bracket_depth == 0 && paren_depth == 0 && brace_depth == 0 {
                    return Some((index, seen_content));
                }
            }
            '\n' => {
                if angle_depth == 0 && bracket_depth == 0 && paren_depth == 0 && brace_depth == 0 {
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
                    '_' | '.'
                        | '|'
                        | '&'
                        | '?'
                        | '['
                        | ']'
                        | '<'
                        | '>'
                        | ','
                        | ' '
                        | '{'
                        | '}'
                        | ':'
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
        || code.contains("| null")
        || code.contains("| undefined")
        || code.contains("!:")
        || code.contains("?:")
        || code.contains(" abstract ")
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

fn find_declared_classes(code: &str) -> Vec<String> {
    let pattern = Regex::new(r"\bclass\s+([A-Za-z_][A-Za-z0-9_]*)\b").expect("class regex");
    pattern
        .captures_iter(code)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn find_base_classes(code: &str) -> Vec<String> {
    let pattern =
        Regex::new(r"\bclass\s+[A-Za-z_][A-Za-z0-9_]*\s+extends\s+([A-Za-z_][A-Za-z0-9_]*)\b")
            .expect("extends regex");
    pattern
        .captures_iter(code)
        .map(|capture| capture[1].to_string())
        .filter(|class_name| class_name != "Component")
        .collect()
}

fn order_scripts_by_class_dependencies(scripts: Vec<PreparedScript>) -> Vec<PreparedScript> {
    let mut remaining = scripts;
    let mut ordered = Vec::new();
    let mut available = BTreeSet::new();

    while !remaining.is_empty() {
        let ready_index = remaining.iter().position(|script| {
            script
                .base_classes
                .iter()
                .all(|base_class| available.contains(base_class))
        });

        let index = ready_index.unwrap_or(0);
        let script = remaining.remove(index);
        for class_name in &script.classes {
            available.insert(class_name.clone());
        }
        ordered.push(script);
    }

    ordered
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn transpiles_blockman_scripts_without_obvious_typescript_syntax() {
        let script_root = Path::new(r"C:\Users\superuse\Downloads\shooting\Assets\Scripts");
        if !script_root.exists() {
            return;
        }

        for entry in WalkDir::new(script_root).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file()
                || entry.path().extension().and_then(|value| value.to_str()) != Some("ts")
            {
                continue;
            }

            let source = fs::read_to_string(entry.path()).expect("script should be readable");
            let transpiled = transpile_typescript_compat(&source);
            assert!(
                transpiled.converted,
                "{}\n{}",
                entry.path().display(),
                transpiled.code
            );
        }
    }

    #[test]
    fn evaluates_blockman_scripts() {
        let script_root = Path::new(r"C:\Users\superuse\Downloads\shooting\Assets\Scripts");
        if !script_root.exists() {
            return;
        }

        let runtime = Runtime::new().expect("QuickJS runtime");
        let context = Context::full(&runtime).expect("QuickJS context");
        context
            .with(|ctx| ctx.eval::<(), _>(QUICKJS_BOOTSTRAP))
            .expect("bootstrap should evaluate");

        let mut scripts = Vec::new();
        for entry in WalkDir::new(script_root).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file()
                || entry.path().extension().and_then(|value| value.to_str()) != Some("ts")
            {
                continue;
            }

            let source = fs::read_to_string(entry.path()).expect("script should be readable");
            let transpiled = transpile_typescript_compat(&source);
            scripts.push(PreparedScript {
                path: entry.path().to_path_buf(),
                classes: find_declared_classes(&transpiled.code),
                base_classes: find_base_classes(&transpiled.code),
                code: transpiled.code,
            });
        }

        for script in order_scripts_by_class_dependencies(scripts) {
            let result = context.with(|ctx| {
                ctx.eval::<(), _>(script.code.as_str())
                    .catch(&ctx)
                    .map_err(|error| error.to_string())
            });
            assert!(
                result.is_ok(),
                "{}\n{:?}\n{}",
                script.path.display(),
                result,
                script.code
            );
        }
    }
}
