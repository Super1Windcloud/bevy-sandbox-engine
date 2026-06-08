//! The launcher for Bevy Sandbox Engine.
//!
//! The launcher provide a bunch of functionalities to manage your projects.

use std::{path::PathBuf, sync::OnceLock};

use bevy::{
    diagnostic::FrameCount,
    ecs::schedule::common_conditions::any_with_component,
    prelude::*,
    render::{
        RenderPlugin,
        settings::{RenderCreation, WgpuSettings},
    },
    tasks::{IoTaskPool, Task, block_on, futures_lite::future},
    window::{MonitorSelection, PrimaryWindow, WindowCreated, WindowMode, WindowPosition},
    winit::WINIT_WINDOWS,
};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use image::ImageReader;
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;
use winit::window::Icon;

use bevy_sandbox_engine::project::{
    ProjectInfo, create_new_project, get_local_projects, set_project_list, templates::Templates,
};

mod ui;

const SHOW_WINDOW_AFTER_FRAMES: u32 = 5;

fn show_primary_window_when_ready(
    mut primary_window: Single<&mut Window, With<PrimaryWindow>>,
    frame_count: Res<FrameCount>,
) {
    if !primary_window.visible && frame_count.0 >= SHOW_WINDOW_AFTER_FRAMES {
        primary_window.visible = true;
    }
}

/// The Task that creates a new project
#[derive(Component)]
struct CreateProjectTask(Task<std::io::Result<ProjectInfo>>);

/// Check on the status of the [`CreateProjectTask`] and handle the result when done
fn poll_create_project_task(
    mut commands: Commands,
    mut task_query: Query<(Entity, &mut CreateProjectTask)>,
    mut project_list: ResMut<ProjectInfoList>,
    mut ui_state: ResMut<ui::LauncherUiState>,
) {
    let (task_entity, mut task) = task_query.single_mut().unwrap();
    if let Some(result) = block_on(future::poll_once(&mut task.0)) {
        let locale_strings = ui::strings(ui_state.locale);
        match result {
            Ok(project_info) => {
                project_list.0.push(project_info.clone());
                set_project_list(project_list.0.clone());
                ui_state.notifications.push(ui::Notification {
                    text: format!(
                        "{}: {}",
                        locale_strings.created_project,
                        project_info.name().unwrap_or_else(|| "Unknown".to_string())
                    ),
                    ttl: Timer::from_seconds(3.0, TimerMode::Once),
                });
                commands.entity(task_entity).despawn();
            }
            Err(error) => {
                error!("Failed to create new project: {:?}", error);
                ui_state.notifications.push(ui::Notification {
                    text: format!("{}: {error}", locale_strings.failed_to_create_project),
                    ttl: Timer::from_seconds(3.0, TimerMode::Once),
                });
                commands.entity(task_entity).despawn();
            }
        }
    }
}

/// Spawn a new [`CreateProjectTask`] to create a new project
fn spawn_create_new_project_task(commands: &mut Commands, template: Templates, path: PathBuf) {
    let task = IoTaskPool::get().spawn(async move { create_new_project(template, path).await });
    commands.spawn(CreateProjectTask(task));
}

#[derive(Resource)]
struct ProjectInfoList(Vec<ProjectInfo>);

static APP_ICON: OnceLock<Option<Icon>> = OnceLock::new();

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

fn set_app_icon_for_window(window_entity: Entity) {
    let Some(icon) = APP_ICON.get_or_init(load_app_icon).clone() else {
        warn!("Failed to load app icon from assets/logo.png");
        return;
    };

    WINIT_WINDOWS.with_borrow(|winit_windows| {
        let Some(window_id) = winit_windows.entity_to_winit.get(&window_entity) else {
            return;
        };

        if let Some(window) = winit_windows.windows.get(window_id) {
            window.set_window_icon(Some(icon.clone()));

            #[cfg(target_os = "windows")]
            window.set_taskbar_icon(Some(icon));
        }
    });
}

fn set_app_icon(mut window_created_events: MessageReader<WindowCreated>) {
    for event in window_created_events.read() {
        set_app_icon_for_window(event.window);
    }
}

fn main() {
    #[cfg(target_os = "windows")]
    let render_plugin = RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
            backends: Some(
                bevy::render::settings::Backends::from_env()
                    .unwrap_or(bevy::render::settings::Backends::VULKAN),
            ),
            ..default()
        }),
        ..default()
    };

    #[cfg(not(target_os = "windows"))]
    let render_plugin = RenderPlugin::default();

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Sandbox Engine Launcher".to_string(),
                        resolution: bevy::window::WindowResolution::new(1320, 860),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        mode: WindowMode::Windowed,
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(render_plugin),
            EguiPlugin::default(),
        ))
        .insert_resource(ClearColor(Color::srgb(0.039, 0.047, 0.063)))
        .insert_resource(ProjectInfoList(get_local_projects()))
        .insert_resource(ui::LauncherUiState::default())
        .add_systems(Startup, ui::setup)
        .add_systems(Update, show_primary_window_when_ready)
        .add_systems(
            Update,
            (
                set_app_icon,
                ui::sync_system_locale,
                poll_create_project_task.run_if(any_with_component::<CreateProjectTask>),
                ui::tick_notifications,
            ),
        )
        .add_systems(EguiPrimaryContextPass, ui::render_launcher_ui)
        .run();
}
