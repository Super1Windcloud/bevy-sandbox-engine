# Bevy Sandbox Engine

`bevy-sandbox-engine` is a sandbox-game-focused engine layer built on top of Bevy.
It packages editor-facing tooling, runtime scaffolding, project templates, and reusable UI/pane crates into one workspace aimed at rapid sandbox game iteration.
The launcher UI now uses `egui`, while the larger editor/runtime stack still contains legacy Bevy-native UI infrastructure that can be migrated incrementally.

## Positioning

This project is not a general-purpose replacement for Bevy itself.
It is a higher-level foundation for building sandbox game workflows:

- in-editor scene and content iteration
- launcher-driven project management
- reusable viewport, pane, and widget systems
- starter templates for new sandbox projects

The current codebase still contains many crates and modules inherited from the original editor prototype layout.
The external identity has been renamed to `bevy-sandbox-engine`, while internal crate names are being migrated incrementally to avoid breaking the workspace all at once.

## Workspace Layout

- `bevy_editor`: the main `bevy-sandbox-engine` crate and engine entry point
- `bevy_editor_launcher`: `egui` launcher binary for opening and creating projects
- `bevy_*`: all workspace crates now live directly under the repository root
- `templates/*`: starter projects copied by the launcher
- `design-book/*`: design notes and long-form architecture docs

## Architecture

The workspace is organized in layers rather than as one monolithic editor crate.

### 1. Launcher layer

- `bevy_editor_launcher` is the executable entry point with `src/main.rs`
- it provides the project picker / project creation flow
- it reads and writes local project metadata, copies templates, and launches editor projects
- the launcher UI is currently built with `egui`

This crate is intentionally small: it is responsible for bootstrapping workflows, not for containing editor logic.

### 2. Engine orchestration layer

- `bevy_editor` is a library crate, not a binary crate
- it exposes the public `bevy-sandbox-engine` API used by templates and downstream projects
- it assembles the runtime/editor plugin graph through `RuntimePlugin`, `EditorPlugin`, and `App`

In practice, this crate is the composition root of the workspace. It pulls together panes, widgets, editor systems, styling, asset helpers, and runtime behavior into one coherent engine layer.

### 3. Editor core layer

These crates provide shared editor capabilities used across the UI:

- `bevy_editor_core`: selection, actions, keybinding, and shared editor state
- `bevy_pane_layout`: docking / pane container model and related UI plumbing
- `bevy_transform_gizmos`: transform editing tools
- `bevy_editor_styles`: shared theme tokens, icons, and visual assets
- `bevy_context_menu`, `bevy_toolbar`, `bevy_gizmo_indicator`: reusable editor-facing interaction components

This layer should contain reusable editor primitives, not app-specific entry logic.

### 4. Pane layer

Panes are feature modules mounted into the editor shell:

- `bevy_2d_viewport`
- `bevy_3d_viewport`
- `bevy_scene_tree`
- `bevy_properties_pane`
- `bevy_asset_browser`
- `bevy_preferences`
- `bevy_marketplace_viewer`

Each pane owns one focused area of editor behavior and can be composed into the workspace UI through the pane layout system.

### 5. Widget and utility layer

The lower-level `bevy_*` support crates are shared building blocks used by panes and editor systems:

- widget crates such as `bevy_text_editing`, `bevy_field_forms`, `bevy_menu_bar`, `bevy_scroll_box`, `bevy_tooltips`
- runtime helpers such as `bevy_clipboard`, `bevy_asset_preview`, `bevy_infinite_grid`, `bevy_undo`
- parsing / macro infrastructure such as `bevy_proto_bsn`, `bevy_proto_bsn_ast`, `bevy_proto_bsn_macros`

These crates exist to keep pane code thinner and to prevent the top-level engine crate from accumulating unrelated responsibilities.

### Runtime flow

The current runtime flow is:

1. `bevy_editor_launcher` starts the launcher UI.
2. The launcher creates or opens a sandbox project from `templates/*`.
3. The generated project depends on `bevy-sandbox-engine` from `bevy_editor`.
4. `bevy_editor::App` builds a Bevy app, installs the runtime plugin set, and optionally attaches editor plugins.
5. Pane crates and widget crates are mounted as part of that editor plugin graph.

### Naming note

The public product identity is `bevy-sandbox-engine`, but several internal crate names still carry `bevy_editor_*` or older prototype-era names.
That is expected for now: the architecture is already layered, while naming cleanup is being done incrementally to avoid unnecessary churn.

## Running

Run the launcher:

```bash
cargo run
```

On Windows, both the launcher and the editor now default to `dx12` if `WGPU_BACKEND` is not set.
Override it manually when you want to test another backend:

```powershell
$env:WGPU_BACKEND="vulkan"
cargo run
```

Run the simple embedded example directly:

```bash
cargo run --example simple_editor -p bevy-sandbox-engine-launcher
```

## Using In Templates

Starter templates now depend on the local workspace crate:

```toml
[dependencies]
bevy_sandbox_engine = { package = "bevy-sandbox-engine", path = "../../bevy_editor" }
```

Minimal app entry:

```rust
use bevy_sandbox_engine::App;

fn main() {
    App::new().run();
}
```

## Roadmap Direction

Near-term focus:

- stabilize the launcher-to-project workflow
- reshape editor-first systems into sandbox-engine primitives
- reduce legacy naming across internal crates
- expand templates toward actual sandbox gameplay foundations

## License

Except where otherwise noted, code in this repository is dual-licensed under:

- MIT ([LICENSE-MIT](LICENSE-MIT))
- Apache-2.0 ([LICENSE-APACHE](LICENSE-APACHE))

See [CREDITS.md](CREDITS.md) for third-party asset attributions.
