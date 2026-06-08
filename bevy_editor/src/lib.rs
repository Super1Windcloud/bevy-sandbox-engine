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

use bevy::app::App as BevyApp;
use bevy::asset::UnapprovedPathMode;
use bevy::color::palettes::tailwind;
use bevy::diagnostic::FrameCount;
use bevy::gilrs::GilrsPlugin;
use bevy::prelude::*;
use bevy::render::{
    RenderPlugin,
    settings::{RenderCreation, WgpuSettings},
};
use bevy::window::{MonitorSelection, PrimaryWindow, WindowMode, WindowPlugin, WindowPosition};
use bevy::{
    feathers::{FeathersPlugin, dark_theme::create_dark_theme, theme::UiTheme},
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    ui_widgets::UiWidgetsPlugins,
};
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

use crate::load_gltf::LoadGltfPlugin;

mod load_gltf;
pub mod project;
mod ui;

const APP_WINDOW_BG: Color = Color::srgb(0.039, 0.047, 0.063);
const SHOW_WINDOW_AFTER_FRAMES: u32 = 3;

fn show_primary_window_when_ready(
    mut primary_window: Single<&mut Window, With<PrimaryWindow>>,
    frame_count: Res<FrameCount>,
) {
    if !primary_window.visible && frame_count.0 >= SHOW_WINDOW_AFTER_FRAMES {
        primary_window.visible = true;
    }
}

/// The plugin that handle the bare minimum to run the application
pub struct RuntimePlugin;

impl Plugin for RuntimePlugin {
    fn build(&self, bevy_app: &mut BevyApp) {
        #[cfg(target_os = "windows")]
        let render_plugin = RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(
                    bevy::render::settings::Backends::from_env()
                        .unwrap_or(bevy::render::settings::Backends::DX12),
                ),
                ..default()
            }),
            ..default()
        };

        #[cfg(not(target_os = "windows"))]
        let render_plugin = RenderPlugin::default();

        bevy_app
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Bevy Sandbox Engine".to_string(),
                            resolution: bevy::window::WindowResolution::new(1440, 900),
                            position: WindowPosition::Centered(MonitorSelection::Primary),
                            mode: WindowMode::Windowed,
                            visible: false,
                            ..default()
                        }),
                        ..default()
                    })
                    .disable::<GilrsPlugin>()
                    .set(AssetPlugin {
                        unapproved_path_mode: UnapprovedPathMode::Deny,
                        ..default()
                    })
                    .set(render_plugin),
            )
            .insert_resource(ClearColor(APP_WINDOW_BG))
            .add_systems(Update, show_primary_window_when_ready);
    }
}

/// The plugin that attach your editor to the application
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, bevy_app: &mut BevyApp) {
        // Update/register this project to the local project list
        project::update_project_info();
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

        let mut bevy_app = BevyApp::new();
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
