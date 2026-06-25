use std::{collections::HashSet, ffi::c_void, path::PathBuf, sync::OnceLock};

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowCreated};
#[cfg(target_os = "windows")]
use bevy::winit::WINIT_WINDOWS;
use image::ImageReader;
#[cfg(target_os = "windows")]
use raw_window_handle::RawWindowHandle;
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GCLP_HICON, GCLP_HICONSM, GetSystemMetrics, ICON_BIG, ICON_SMALL, ICON_SMALL2, IMAGE_ICON,
    LR_LOADFROMFILE, LoadImageW, SM_CXICON, SM_CXSMICON, SM_CYICON, SM_CYSMICON, SendMessageW,
    SetClassLongPtrW, WM_SETICON,
};
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;
use winit::window::Icon;

static APP_ICON: OnceLock<Option<Icon>> = OnceLock::new();
#[cfg(target_os = "windows")]
static NATIVE_APP_ICONS: OnceLock<Option<NativeAppIcons>> = OnceLock::new();

#[cfg(target_os = "windows")]
#[derive(Clone, Copy)]
struct NativeAppIcons {
    small: usize,
    big: usize,
}

pub struct WindowIconPlugin;

impl Plugin for WindowIconPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WindowIconState>()
            .add_systems(Update, (ensure_primary_window_icon, set_app_icon));
    }
}

#[derive(Resource, Default)]
struct WindowIconState {
    configured_windows: HashSet<Entity>,
    primary_retry_frames: u8,
    logged_missing_png: bool,
    logged_missing_ico: bool,
}

fn app_icon_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(current_dir) = std::env::current_dir() {
        candidates.push(current_dir.join("assets").join("logo.png"));
        candidates.push(current_dir.join("assets").join("logo-100px.png"));
    }

    if let Ok(exe_path) = std::env::current_exe()
        && let Some(exe_dir) = exe_path.parent()
    {
        candidates.push(exe_dir.join("assets").join("logo.png"));
        candidates.push(exe_dir.join("assets").join("logo-100px.png"));
        candidates.push(exe_dir.join("..").join("assets").join("logo.png"));
        candidates.push(exe_dir.join("..").join("assets").join("logo-100px.png"));
    }

    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("assets")
            .join("logo.png"),
    );
    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("assets")
            .join("logo-100px.png"),
    );

    candidates
}

fn load_app_icon() -> Option<Icon> {
    app_icon_candidates().into_iter().find_map(|icon_path| {
        let image = ImageReader::open(icon_path)
            .ok()?
            .decode()
            .ok()?
            .into_rgba8();
        let (width, height) = image.dimensions();

        Icon::from_rgba(image.into_raw(), width, height).ok()
    })
}

fn set_app_icon_for_window(window_entity: Entity, icon_state: &mut WindowIconState) -> bool {
    if icon_state.configured_windows.contains(&window_entity) {
        return true;
    }

    let Some(icon) = APP_ICON.get_or_init(load_app_icon).clone() else {
        if !icon_state.logged_missing_png {
            warn!("Failed to load app icon from assets/logo.png");
            icon_state.logged_missing_png = true;
        }
        return false;
    };

    #[cfg(target_os = "windows")]
    {
        let configured = WINIT_WINDOWS.with_borrow(|winit_windows| {
            let Some(window_id) = winit_windows.entity_to_winit.get(&window_entity) else {
                return false;
            };

            if let Some(window) = winit_windows.windows.get(window_id) {
                window.set_window_icon(Some(icon.clone()));
                window.set_taskbar_icon(Some(icon.clone()));
                set_native_window_icons(window, icon_state);
                true
            } else {
                false
            }
        });

        if configured {
            icon_state.configured_windows.insert(window_entity);
        }

        configured
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = window_entity;
        let _ = icon;
        icon_state.configured_windows.insert(window_entity);
        true
    }
}

#[cfg(target_os = "windows")]
fn native_icon_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(current_dir) = std::env::current_dir() {
        candidates.push(
            current_dir
                .join("bevy_editor_launcher")
                .join("assets")
                .join("app.ico"),
        );
        candidates.push(
            current_dir
                .join("bevy_editor")
                .join("assets")
                .join("app.ico"),
        );
        candidates.push(current_dir.join("assets").join("app.ico"));
    }

    if let Ok(exe_path) = std::env::current_exe()
        && let Some(exe_dir) = exe_path.parent()
    {
        candidates.push(exe_dir.join("assets").join("app.ico"));
        candidates.push(exe_dir.join("..").join("assets").join("app.ico"));
    }

    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("bevy_editor_launcher")
            .join("assets")
            .join("app.ico"),
    );

    candidates
}

#[cfg(target_os = "windows")]
fn load_native_app_icons() -> Option<NativeAppIcons> {
    native_icon_candidates()
        .into_iter()
        .find_map(load_native_app_icons_from_path)
}

#[cfg(target_os = "windows")]
fn load_native_app_icons_from_path(path: PathBuf) -> Option<NativeAppIcons> {
    if !path.exists() {
        return None;
    }

    let small_width = unsafe { GetSystemMetrics(SM_CXSMICON) };
    let small_height = unsafe { GetSystemMetrics(SM_CYSMICON) };
    let big_width = unsafe { GetSystemMetrics(SM_CXICON) };
    let big_height = unsafe { GetSystemMetrics(SM_CYICON) };

    let small = load_native_app_icon_from_path(&path, small_width, small_height)?;
    let big = load_native_app_icon_from_path(&path, big_width, big_height)
        .or_else(|| load_native_app_icon_from_path(&path, 0, 0))?;

    Some(NativeAppIcons { small, big })
}

#[cfg(target_os = "windows")]
fn load_native_app_icon_from_path(
    path: &std::path::Path,
    width: i32,
    height: i32,
) -> Option<usize> {
    let mut wide_path = path
        .as_os_str()
        .to_string_lossy()
        .encode_utf16()
        .collect::<Vec<u16>>();
    wide_path.push(0);

    let handle = unsafe {
        LoadImageW(
            std::ptr::null_mut(),
            wide_path.as_ptr(),
            IMAGE_ICON,
            width,
            height,
            LR_LOADFROMFILE,
        )
    };

    (!handle.is_null()).then_some(handle as usize)
}

#[cfg(target_os = "windows")]
fn set_native_window_icons(window: &winit::window::Window, icon_state: &mut WindowIconState) {
    let Some(icons) = *NATIVE_APP_ICONS.get_or_init(load_native_app_icons) else {
        if !icon_state.logged_missing_ico {
            warn!("Failed to load native app icon from app.ico");
            icon_state.logged_missing_ico = true;
        }
        return;
    };

    let Ok(window_handle) = (unsafe { window.window_handle_any_thread() }) else {
        return;
    };

    let RawWindowHandle::Win32(handle) = window_handle.as_raw() else {
        return;
    };

    let hwnd = handle.hwnd.get();
    if hwnd == 0 {
        return;
    }

    let hwnd = hwnd as *mut c_void;
    let small_icon = icons.small as isize;
    let big_icon = icons.big as isize;

    unsafe {
        SendMessageW(hwnd, WM_SETICON, ICON_SMALL as usize, small_icon);
        SendMessageW(hwnd, WM_SETICON, ICON_SMALL2 as usize, small_icon);
        SendMessageW(hwnd, WM_SETICON, ICON_BIG as usize, big_icon);
        SetClassLongPtrW(hwnd, GCLP_HICONSM, small_icon);
        SetClassLongPtrW(hwnd, GCLP_HICON, big_icon);
    }
}

fn set_app_icon(
    mut window_created_events: MessageReader<WindowCreated>,
    mut icon_state: ResMut<WindowIconState>,
) {
    for event in window_created_events.read() {
        let _ = set_app_icon_for_window(event.window, &mut icon_state);
    }
}

fn ensure_primary_window_icon(
    primary_window_entity: Single<Entity, With<PrimaryWindow>>,
    mut icon_state: ResMut<WindowIconState>,
) {
    if icon_state
        .configured_windows
        .contains(&*primary_window_entity)
    {
        return;
    }

    if set_app_icon_for_window(*primary_window_entity, &mut icon_state) {
        return;
    }

    icon_state.primary_retry_frames = icon_state.primary_retry_frames.saturating_add(1);
    if icon_state.primary_retry_frames > 120 {
        icon_state.configured_windows.insert(*primary_window_entity);
    }
}
