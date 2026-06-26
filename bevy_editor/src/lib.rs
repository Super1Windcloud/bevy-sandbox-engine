//! The main Bevy Sandbox Engine application.
//!
//! This crate contains a standalone application that can be used to build sandbox game workflows on top of Bevy.
//! Virtually all of the underlying logic and functionality of the engine should be backed by the assorted crates in the workspace;
//! this crate is simply responsible for orchestrating those crates and providing a user interface for them.
//!
//! The exact nature of this crate will be in flux for a while:
//!
//! - Initially, this will be a standard Bevy application that simply edits scenes with `DefaultPlugins`.
//! - Then, it will be a statically linked plugin that can be added to any Bevy game at compile time,
//!   which transforms the user's application into an editor that runs their game.
//! - Finally, it will be a standalone application that communicates with a running Bevy game via the Bevy Remote Protocol.

use std::f32::consts::TAU;
use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::app::App as BevyApp;
use bevy::asset::UnapprovedPathMode;
use bevy::color::palettes::tailwind;
use bevy::feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme};
use bevy::gilrs::GilrsPlugin;
use bevy::prelude::*;
use bevy::render::{
    RenderPlugin,
    settings::{Backends, RenderCreation, WgpuSettings},
};
use bevy::window::{MonitorSelection, WindowMode, WindowPlugin, WindowPosition};
use bevy::window::{WindowCloseRequested, WindowClosed};
// Re-export Bevy for project use
pub use bevy;

use bevy::winit::{UpdateMode, WinitSettings};
use bevy_context_menu::ContextMenuPlugin;
use bevy_editor_core::EditorCorePlugin;
use bevy_editor_core::selection::Selectable;
use bevy_editor_styles::StylesPlugin;
use bevy_egui::EguiPlugin;
use bevy_toolbar::ActiveTool;
use bevy_transform_gizmos::{GizmoTransformable, TransformGizmoPlugin};

// Panes
use bevy_2d_viewport::Viewport2dPanePlugin;
use bevy_3d_viewport::Viewport3dPanePlugin;
use bevy_asset_browser::AssetBrowserPanePlugin;

use crate::compat::CompatProjectPlugin;
use crate::load_gltf::LoadGltfPlugin;
use crate::window_icon::WindowIconPlugin;

mod compat;
mod load_gltf;
pub mod locale_env;
pub mod project;
mod ui;
pub mod window_icon;

const APP_WINDOW_BG: Color = Color::srgb(0.039, 0.047, 0.063);

#[derive(Resource, Default, Clone)]
pub(crate) struct LaunchOptions {
    pub(crate) project_path: Option<PathBuf>,
    editor_mode: bool,
}

#[derive(Resource)]
struct DelayedWindowReveal {
    remaining_frames: u8,
}

fn log_window_close_requested(mut events: MessageReader<WindowCloseRequested>) {
    for event in events.read() {
        warn!(
            "Editor window close requested for entity {:?}",
            event.window
        );
    }
}

fn log_window_closed(mut events: MessageReader<WindowClosed>) {
    for event in events.read() {
        warn!("Editor window closed for entity {:?}", event.window);
    }
}

/// The plugin that handle the bare minimum to run the application
pub struct RuntimePlugin;

impl Plugin for RuntimePlugin {
    fn build(&self, bevy_app: &mut BevyApp) {
        let launch_options = bevy_app
            .world()
            .get_resource::<LaunchOptions>()
            .cloned()
            .unwrap_or_default();
        let render_plugin = RenderPlugin {
            render_creation: RenderCreation::Automatic(Box::new(WgpuSettings {
                backends: Some(default_render_backends()),
                ..default()
            })),
            ..default()
        };

        bevy_app
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Bevy Sandbox Engine".to_string(),
                            resolution: bevy::window::WindowResolution::new(1440, 900),
                            position: WindowPosition::Centered(MonitorSelection::Primary),
                            mode: WindowMode::Windowed,
                            visible: !launch_options.editor_mode,
                            ..default()
                        }),
                        ..default()
                    })
                    .disable::<GilrsPlugin>()
                    .set(AssetPlugin {
                        file_path: asset_root_for_launch_options(&launch_options),
                        unapproved_path_mode: UnapprovedPathMode::Deny,
                        ..default()
                    })
                    .set(render_plugin),
            )
            .add_plugins(WindowIconPlugin)
            .insert_resource(ClearColor(APP_WINDOW_BG))
            .add_systems(Update, (log_window_close_requested, log_window_closed));
    }
}

/// The plugin that attach your editor to the application
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, bevy_app: &mut BevyApp) {
        let launch_options = bevy_app
            .world()
            .get_resource::<LaunchOptions>()
            .cloned()
            .unwrap_or_default();
        // Update/register this project to the local project list
        project::update_project_info(launch_options.project_path.as_deref());
        info!("Loading Bevy Sandbox Engine");
        bevy_app
            .add_plugins((
                EditorCorePlugin,
                ContextMenuPlugin,
                StylesPlugin,
                EguiPlugin::default(),
                Viewport2dPanePlugin,
                Viewport3dPanePlugin,
                ui::EditorUIPlugin,
                AssetBrowserPanePlugin,
                LoadGltfPlugin,
                MeshPickingPlugin,
                TransformGizmoPlugin,
                FeathersPlugins,
            ))
            .add_plugins(CompatProjectPlugin)
            .insert_resource(DelayedWindowReveal {
                remaining_frames: 3,
            })
            .insert_resource(WinitSettings {
                focused_mode: UpdateMode::reactive(Duration::from_secs_f64(1.0 / 60.0)),
                unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs(1)),
            })
            .insert_resource(UiTheme(create_dark_theme()))
            .init_resource::<ActiveTool>()
            .add_systems(Startup, dummy_setup)
            .add_systems(Update, reveal_editor_window_when_ready);
    }
}

fn reveal_editor_window_when_ready(
    reveal: Option<ResMut<DelayedWindowReveal>>,
    root_ui: Query<(), With<ui::RootUINode>>,
    mut primary_window: Single<&mut Window, With<bevy::window::PrimaryWindow>>,
) {
    let Some(mut reveal) = reveal else {
        return;
    };

    if root_ui.is_empty() {
        return;
    }

    if reveal.remaining_frames > 0 {
        reveal.remaining_frames -= 1;
        return;
    }

    primary_window.visible = true;
}

/// Your game application
/// This appllication allow your game to run, and the editor to be attached to it
#[derive(Default)]
pub struct App;

impl App {
    /// create new instance of [`App`]
    pub fn new() -> Self {
        Self
    }

    /// Run the application
    pub fn run(&self) -> AppExit {
        let args = std::env::args().collect::<Vec<String>>();
        let editor_mode = !args.iter().any(|arg| arg == "-game");
        let launch_options = LaunchOptions {
            project_path: resolve_project_path(&args, editor_mode),
            editor_mode,
        };

        let mut bevy_app = BevyApp::new();
        bevy_app.insert_resource(launch_options);
        bevy_app.add_plugins(RuntimePlugin);
        if editor_mode {
            bevy_app.add_plugins(EditorPlugin);
        }

        bevy_app.run()
    }
}

/// This is temporary, until we can load maps from the asset browser
fn dummy_setup(
    launch_options: Res<LaunchOptions>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_3d: ResMut<Assets<StandardMaterial>>,
) {
    if launch_options.project_path.is_some() {
        return;
    }

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(2.5)))),
        MeshMaterial3d(materials_3d.add(Color::WHITE)),
        Name::new("Plane"),
        Selectable,
        GizmoTransformable,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.)))),
        MeshMaterial3d(materials_3d.add(Color::from(tailwind::BLUE_500))),
        Transform::from_translation(vec3(1.1, 0.5, -1.3))
            .with_rotation(Quat::from_rotation_y(TAU * 0.05)),
        Name::new("Box"),
        Selectable,
        GizmoTransformable,
    ));

    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::default().looking_to(vec3(-1., -1., 1.), Vec3::Y),
        GizmoTransformable,
        Name::new("DirectionalLight"),
    ));
}

fn asset_root_for_launch_options(launch_options: &LaunchOptions) -> String {
    launch_options
        .project_path
        .as_deref()
        .and_then(project_asset_root)
        .unwrap_or_else(|| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("assets")
        })
        .display()
        .to_string()
}

fn project_asset_root(project_path: &Path) -> Option<PathBuf> {
    let assets = project_path.join("Assets");
    assets.is_dir().then_some(assets)
}

fn parse_project_path_argument(args: &[String]) -> Option<PathBuf> {
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--project" {
            return iter.next().map(PathBuf::from);
        }
    }

    None
}

fn resolve_project_path(args: &[String], editor_mode: bool) -> Option<PathBuf> {
    parse_project_path_argument(args).or_else(|| {
        if editor_mode {
            project::get_most_recent_project().map(|project| project.path)
        } else {
            None
        }
    })
}

fn default_render_backends() -> Backends {
    Backends::from_env().unwrap_or_else(|| {
        #[cfg(target_os = "windows")]
        {
            Backends::VULKAN
        }

        #[cfg(target_os = "macos")]
        {
            Backends::METAL
        }

        #[cfg(all(
            unix,
            not(target_os = "macos"),
            not(target_os = "android"),
            not(target_family = "wasm")
        ))]
        {
            Backends::VULKAN
        }

        #[cfg(target_os = "android")]
        {
            Backends::VULKAN
        }

        #[cfg(target_family = "wasm")]
        {
            Backends::BROWSER_WEBGPU
        }
    })
}
