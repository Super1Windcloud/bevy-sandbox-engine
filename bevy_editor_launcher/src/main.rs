//! The launcher for Bevy Sandbox Engine.
//!
//! The launcher provide a bunch of functionalities to manage your projects.

use std::{collections::HashMap, path::PathBuf, process::Child, time::Duration};

#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use std::sync::{
    Mutex,
    mpsc::{Receiver, Sender, TryRecvError, channel},
};

use bevy::{
    diagnostic::FrameCount,
    ecs::schedule::common_conditions::any_with_component,
    gilrs::GilrsPlugin,
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
    tasks::{IoTaskPool, Task, block_on, futures_lite::future},
    window::{MonitorSelection, PrimaryWindow, WindowCloseRequested, WindowMode, WindowPosition},
    winit::{EventLoopProxy, EventLoopProxyWrapper, WINIT_WINDOWS, WinitUserEvent},
};
use bevy::window::close_when_requested;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
#[cfg(target_os = "windows")]
use sys_locale::get_locale;
#[cfg(target_os = "windows")]
use tray_item::{IconSource, TrayItem};

use bevy_sandbox_engine::project::{
    ProjectInfo, create_new_project, get_local_projects, run_project, set_project_list,
};
use bevy_sandbox_engine::window_icon::WindowIconPlugin;

#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::WindowsAndMessaging::{
    IMAGE_ICON, LR_DEFAULTSIZE, LR_LOADFROMFILE, LoadImageW,
};

mod ui;

const SHOW_WINDOW_AFTER_FRAMES: u32 = 5;

fn show_primary_window_when_ready(
    mut primary_window: Single<&mut Window, With<PrimaryWindow>>,
    frame_count: Res<FrameCount>,
    mut window_state: ResMut<LauncherWindowState>,
) {
    if window_state.show_on_ready && frame_count.0 >= SHOW_WINDOW_AFTER_FRAMES {
        primary_window.visible = true;
        primary_window.set_minimized(false);
        primary_window.focused = true;
        window_state.show_on_ready = false;
        window_state.focus_requested = true;
    }
}

fn focus_primary_window_on_show(
    primary_window_entity: Single<Entity, With<PrimaryWindow>>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    mut window_state: ResMut<LauncherWindowState>,
) {
    if !window_state.focus_requested || !primary_window.visible {
        return;
    }

    WINIT_WINDOWS.with_borrow(|winit_windows| {
        let Some(window_id) = winit_windows.entity_to_winit.get(&*primary_window_entity) else {
            return;
        };

        let Some(window) = winit_windows.windows.get(window_id) else {
            return;
        };

        if window.has_focus() {
            window_state.focus_requested = false;
        } else {
            window.focus_window();
        }
    });
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
                match run_project(&project_info) {
                    Ok(_) => {
                        ui_state.hide_window_requested = true;
                    }
                    Err(error) => {
                        error!("Failed to run new project after creation: {:?}", error);
                        ui_state.notifications.push(ui::Notification {
                            text: format!("{}: {}", locale_strings.failed_to_run_project, error),
                            ttl: Timer::from_seconds(3.0, TimerMode::Once),
                        });
                    }
                }
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
        commands.entity(task_entity).despawn();
    }
}

/// Spawn a new [`CreateProjectTask`] to create a new project
fn spawn_create_new_project_task(commands: &mut Commands, template_id: String, path: PathBuf) {
    let task = IoTaskPool::get().spawn(async move { create_new_project(template_id, path).await });
    commands.spawn(CreateProjectTask(task));
}

#[derive(Resource)]
struct ProjectInfoList(Vec<ProjectInfo>);

#[derive(Resource)]
struct LauncherWindowState {
    show_on_ready: bool,
    focus_requested: bool,
}

impl Default for LauncherWindowState {
    fn default() -> Self {
        Self {
            show_on_ready: true,
            focus_requested: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProjectLaunchPhase {
    Ready,
    Launching,
    Running,
}

pub struct RunningProject {
    pub child: Child,
    pub phase: ProjectLaunchPhase,
    pub launch_timer: Timer,
}

#[derive(Resource, Default)]
pub struct RunningProjects(pub HashMap<PathBuf, RunningProject>);

#[cfg(target_os = "windows")]
#[derive(Clone, Copy)]
enum TrayCommand {
    ShowMainWindow,
    HideWindow,
    ExitLauncher,
}

#[cfg(target_os = "windows")]
#[derive(Resource)]
struct TrayCommandQueue(Mutex<Receiver<TrayCommand>>);

#[cfg(target_os = "windows")]
#[derive(Resource)]
struct LauncherTray(TrayItem);

#[cfg(target_os = "windows")]
struct TrayStrings {
    tooltip: &'static str,
    show: &'static str,
    hide: &'static str,
    exit: &'static str,
}

#[cfg(target_os = "windows")]
fn tray_strings() -> TrayStrings {
    let locale = get_locale().unwrap_or_else(|| "en-US".to_string());
    if locale.to_ascii_lowercase().starts_with("zh") {
        TrayStrings {
            tooltip: "Bevy Sandbox Engine Launcher",
            show: "打开主界面",
            hide: "隐藏窗口",
            exit: "退出",
        }
    } else {
        TrayStrings {
            tooltip: "Bevy Sandbox Engine Launcher",
            show: "Open Launcher",
            hide: "Hide Window",
            exit: "Exit",
        }
    }
}

#[cfg(target_os = "windows")]
fn send_tray_command(
    command_tx: &Sender<TrayCommand>,
    event_loop_proxy: &EventLoopProxy<WinitUserEvent>,
    command: TrayCommand,
) {
    if command_tx.send(command).is_ok() {
        let _ = event_loop_proxy.send_event(WinitUserEvent::WakeUp);
    }
}

#[cfg(target_os = "windows")]
fn load_tray_icon_handle() -> Option<isize> {
    let icon_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("app.ico");
    let mut wide_path = icon_path
        .as_os_str()
        .to_string_lossy()
        .encode_utf16()
        .collect::<Vec<u16>>();
    wide_path.push(0);

    // SAFETY: The path buffer is null-terminated and remains alive for the duration of the call.
    let handle = unsafe {
        LoadImageW(
            null_mut(),
            wide_path.as_ptr(),
            IMAGE_ICON,
            0,
            0,
            LR_LOADFROMFILE | LR_DEFAULTSIZE,
        )
    };

    (!handle.is_null()).then_some(handle as isize)
}

#[cfg(target_os = "windows")]
fn create_launcher_tray(
    command_tx: Sender<TrayCommand>,
    event_loop_proxy: EventLoopProxy<WinitUserEvent>,
) -> Result<TrayItem, tray_item::TIError> {
    let strings = tray_strings();
    let icon = load_tray_icon_handle()
        .map(IconSource::RawIcon)
        .unwrap_or(IconSource::Resource(""));
    let mut tray = TrayItem::new(strings.tooltip, icon)?;

    let show_tx = command_tx.clone();
    let show_proxy = event_loop_proxy.clone();
    tray.add_menu_item(strings.show, move || {
        send_tray_command(&show_tx, &show_proxy, TrayCommand::ShowMainWindow);
    })?;

    let hide_tx = command_tx.clone();
    let hide_proxy = event_loop_proxy.clone();
    tray.add_menu_item(strings.hide, move || {
        send_tray_command(&hide_tx, &hide_proxy, TrayCommand::HideWindow);
    })?;

    tray.add_menu_item(strings.exit, move || {
        send_tray_command(&command_tx, &event_loop_proxy, TrayCommand::ExitLauncher);
    })?;

    Ok(tray)
}

#[cfg(target_os = "windows")]
fn setup_tray(mut commands: Commands, event_loop_proxy: Res<EventLoopProxyWrapper>) {
    let (command_tx, command_rx) = channel();
    match create_launcher_tray(command_tx, (**event_loop_proxy).clone()) {
        Ok(tray) => {
            commands.insert_resource(TrayCommandQueue(Mutex::new(command_rx)));
            commands.insert_resource(LauncherTray(tray));
        }
        Err(error) => {
            error!("Failed to create launcher tray icon: {error}");
        }
    }
}

#[cfg(target_os = "windows")]
fn poll_tray_commands(
    command_queue: Option<Res<TrayCommandQueue>>,
    tray: Option<Res<LauncherTray>>,
    mut primary_window: Single<&mut Window, With<PrimaryWindow>>,
    mut window_state: ResMut<LauncherWindowState>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(command_queue) = command_queue else {
        return;
    };
    let _tray = tray;

    let receiver = match command_queue.0.lock() {
        Ok(receiver) => receiver,
        Err(error) => {
            error!("Failed to lock tray command queue: {error}");
            return;
        }
    };

    loop {
        match receiver.try_recv() {
            Ok(TrayCommand::ShowMainWindow) => {
                primary_window.visible = true;
                primary_window.set_minimized(false);
                primary_window.focused = true;
                window_state.focus_requested = true;
            }
            Ok(TrayCommand::HideWindow) => {
                primary_window.visible = false;
                window_state.focus_requested = false;
            }
            Ok(TrayCommand::ExitLauncher) => {
                exit.write(AppExit::Success);
            }
            Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => break,
        }
    }
}

fn exit_launcher_on_close_request(
    mut close_requests: MessageReader<WindowCloseRequested>,
    primary_window_entity: Single<Entity, With<PrimaryWindow>>,
    mut exit: MessageWriter<AppExit>,
) {
    for request in close_requests.read() {
        if request.window == *primary_window_entity {
            exit.write(AppExit::Success);
        }
    }
}

fn hide_launcher_on_request(
    mut primary_window: Single<&mut Window, With<PrimaryWindow>>,
    mut window_state: ResMut<LauncherWindowState>,
    mut ui_state: ResMut<ui::LauncherUiState>,
) {
    if !ui_state.hide_window_requested {
        return;
    }

    primary_window.visible = false;
    window_state.focus_requested = false;
    ui_state.hide_window_requested = false;
}

fn tick_running_projects(time: Res<Time>, mut running_projects: ResMut<RunningProjects>) {
    running_projects.0.retain(|_, running_project| {
        if let Ok(Some(_)) = running_project.child.try_wait() {
            return false;
        }

        if running_project.phase == ProjectLaunchPhase::Launching {
            running_project.launch_timer.tick(time.delta());
            if running_project.launch_timer.elapsed() >= Duration::from_secs(2) {
                running_project.phase = ProjectLaunchPhase::Running;
            }
        }

        true
    });
}

fn main() {
    let render_plugin = RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
            backends: Some(default_render_backends()),
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Sandbox Engine Launcher".to_string(),
                        resolution: bevy::window::WindowResolution::new(800, 600),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        mode: WindowMode::Windowed,
                        focused: true,
                        decorations: true,
                        visible: false,
                        ..default()
                    }),

                    close_when_requested: false,
                    ..default()
                })
                .disable::<GilrsPlugin>()
                .set(render_plugin),
            EguiPlugin::default(),
            WindowIconPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.039, 0.047, 0.063)))
        .insert_resource(ProjectInfoList(get_local_projects()))
        .insert_resource(LauncherWindowState::default())
        .insert_resource(RunningProjects::default())
        .insert_resource(ui::LauncherUiState::default())
        .add_systems(Startup, ui::setup)
        .add_systems(Startup, setup_tray)
        .add_systems(Update, show_primary_window_when_ready)
        .add_systems(Update, tick_running_projects)
        .add_systems(
            Update,
            (
                focus_primary_window_on_show,
                hide_launcher_on_request,
                exit_launcher_on_close_request,
                ui::sync_system_locale,
                poll_create_project_task.run_if(any_with_component::<CreateProjectTask>),
                ui::tick_notifications,
            ),
        )
        .add_systems(Update, poll_tray_commands)
        .add_systems(EguiPrimaryContextPass, ui::render_launcher_ui)
        .run();
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
