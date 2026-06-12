use std::{path::PathBuf, sync::OnceLock};

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowCreated};
#[cfg(target_os = "windows")]
use bevy::winit::WINIT_WINDOWS;
use image::ImageReader;
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;
use winit::window::Icon;

static APP_ICON: OnceLock<Option<Icon>> = OnceLock::new();

pub struct WindowIconPlugin;

impl Plugin for WindowIconPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ensure_primary_window_icon, set_app_icon));
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
        warn!("Failed to load app icon from assets/logo.png");
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
