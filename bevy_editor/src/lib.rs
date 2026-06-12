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
use std::time::Duration;
use std::{path::PathBuf, sync::OnceLock};

use bevy::app::App as BevyApp;
use bevy::asset::UnapprovedPathMode;
use bevy::color::palettes::tailwind;
use bevy::gilrs::GilrsPlugin;
use bevy::prelude::*;
use bevy::render::{
    RenderPlugin,
    settings::{Backends, RenderCreation, WgpuSettings},
};
use bevy::window::{MonitorSelection, WindowMode, WindowPlugin, WindowPosition};
use bevy::window::{PrimaryWindow, WindowCloseRequested, WindowClosed, WindowCreated};
use bevy::{
    feathers::{FeathersPlugin, dark_theme::create_dark_theme, theme::UiTheme},
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    ui_widgets::UiWidgetsPlugins,
};
// Re-export Bevy for project use
pub use bevy;

use bevy::winit::{UpdateMode, WinitSettings};
#[cfg(target_os = "windows")]
use bevy::winit::WINIT_WINDOWS;
use bevy_context_menu::ContextMenuPlugin;
use bevy_editor_core::EditorCorePlugin;
use bevy_editor_core::selection::Selectable;
use bevy_editor_styles::StylesPlugin;
use bevy_egui::EguiPlugin;
use bevy_toolbar::ActiveTool;
use bevy_transform_gizmos::{GizmoTransformable, TransformGizmoPlugin};
use image::ImageReader;
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;
use winit::window::Icon;

// Panes
use bevy_2d_viewport::Viewport2dPanePlugin;
use bevy_3d_viewport::Viewport3dPanePlugin;
use bevy_asset_browser::AssetBrowserPanePlugin;

use crate::compat::CompatProjectPlugin;
use crate::load_gltf::LoadGltfPlugin;

mod compat;
mod load_gltf;
pub mod project;
mod ui;

const APP_WINDOW_BG: Color = Color::srgb(0.039, 0.047, 0.063);
static APP_ICON: OnceLock<Option<Icon>> = OnceLock::new();

#[derive(Resource, Default, Clone)]
struct LaunchOptions {
    project_path: Option<PathBuf>,
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

fn load_app_icon() -> Option<Icon> {
    let icon_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("assets")
        .join("logo.png");

    let image = ImageReader::open(&icon_path)
        .ok()?
        .decode()
        .ok()?
        .into_rgba8();
    let (width, height) = image.dimensions();

    Icon::from_rgba(image.into_raw(), width, height).ok()
}

fn set_app_icon_for_window(window_entity: Entity) -> bool {
    let Some(icon) = APP_ICON.get_or_init(load_app_icon).clone() else {
        warn!("Failed to load editor icon from assets/logo.png");
        return false;
    };

    #[cfg(target_os = "windows")]
    {
        WINIT_WINDOWS.with_borrow(|winit_windows| {
            let Some(window_id) = winit_windows.entity_to_winit.get(&window_entity) else {
                return false;
            };

            if let Some(window) = winit_windows.windows.get(window_id) {
                window.set_window_icon(Some(icon.clone()));
                window.set_taskbar_icon(Some(icon));
                true
            } else {
                false
            }
        })
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = window_entity;
        let _ = icon;
        false
    }
}

fn set_app_icon(mut window_created_events: MessageReader<WindowCreated>) {
    for event in window_created_events.read() {
        let _ = set_app_icon_for_window(event.window);
    }
}

fn ensure_primary_window_icon(primary_window_entity: Single<Entity, With<PrimaryWindow>>) {
    let _ = set_app_icon_for_window(*primary_window_entity);
}

/// The plugin that handle the bare minimum to run the application
pub struct RuntimePlugin;

impl Plugin for RuntimePlugin {
    fn build(&self, bevy_app: &mut BevyApp) {
        let render_plugin = RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(default_render_backends()),
                ..default()
            }),
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
                            visible: true,
                            ..default()
                        }),
                        ..default()
                    })
                    .disable::<GilrsPlugin>()
                    .set(AssetPlugin {
                        file_path: "../assets".to_string(),
                        unapproved_path_mode: UnapprovedPathMode::Deny,
                        ..default()
                    })
                    .set(render_plugin),
            )
            .insert_resource(ClearColor(APP_WINDOW_BG))
            .add_systems(
                Update,
                (
                    ensure_primary_window_icon,
                    set_app_icon,
                    log_window_close_requested,
                    log_window_closed,
                ),
            );
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
                UiWidgetsPlugins,
                InputDispatchPlugin,
                TabNavigationPlugin,
                FeathersPlugin,
            ))
            .add_plugins(CompatProjectPlugin)
            .insert_resource(WinitSettings {
                focused_mode: UpdateMode::reactive(Duration::from_secs_f64(1.0 / 60.0)),
                unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs(1)),
            })
            .insert_resource(UiTheme(create_dark_theme()))
            .init_resource::<ActiveTool>()
            .add_systems(Startup, dummy_setup);
    }
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
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_3d: ResMut<Assets<StandardMaterial>>,
) {
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
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(vec3(-1., -1., 1.), Vec3::Y),
        GizmoTransformable,
        Name::new("DirectionalLight"),
    ));
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
