# Bevy Sandbox Engine

`bevy-sandbox-engine` is a sandbox-game-focused engine layer built on top of Bevy.
It packages editor-facing tooling, runtime scaffolding, project templates, and reusable UI/pane crates into one workspace aimed at rapid sandbox game iteration.

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

- `crates/bevy_editor`: the main `bevy-sandbox-engine` crate and engine entry point
- `crates/bevy_editor_launcher`: launcher binary for opening and creating projects
- `bevy_editor_panes/*`: viewport and tool panes
- `bevy_widgets/*`: reusable UI widgets
- `templates/*`: starter projects copied by the launcher
- `design-book/*`: design notes and long-form architecture docs

## Running

Run the launcher:

```bash
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
bevy_sandbox_engine = { package = "bevy-sandbox-engine", path = "../../crates/bevy_editor" }
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
