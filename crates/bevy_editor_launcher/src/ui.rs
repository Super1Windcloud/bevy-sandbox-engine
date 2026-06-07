use std::io::ErrorKind;
use std::path::Path;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use bevy_sandbox_engine::project::{run_project, set_project_list, templates::Templates};

use crate::{ProjectInfoList, spawn_create_new_project_task};

#[derive(Resource, Default)]
pub struct LauncherUiState {
    pub notifications: Vec<Notification>,
}

pub struct Notification {
    pub text: String,
    pub ttl: Timer,
}

const LAUNCHER_BG: Color = Color::srgb(0.039, 0.047, 0.063);

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(LAUNCHER_BG),
            ..default()
        },
    ));
}

pub fn tick_notifications(time: Res<Time>, mut ui_state: ResMut<LauncherUiState>) {
    for notification in &mut ui_state.notifications {
        notification.ttl.tick(time.delta());
    }
    ui_state
        .notifications
        .retain(|notification| !notification.ttl.is_finished());
}

fn push_notification(ui_state: &mut LauncherUiState, message: impl Into<String>) {
    ui_state.notifications.push(Notification {
        text: message.into(),
        ttl: Timer::from_seconds(3.0, TimerMode::Once),
    });
}

pub fn render_launcher_ui(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut project_list: ResMut<ProjectInfoList>,
    mut ui_state: ResMut<LauncherUiState>,
    mut exit: MessageWriter<AppExit>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    ctx.set_visuals(egui::Visuals::dark());

    egui::TopBottomPanel::top("top_bar")
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(18, 20, 24))
                .inner_margin(egui::Margin::symmetric(16, 12)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Bevy Sandbox Engine");
                ui.separator();
                ui.label("Sandbox game engine editor");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("New Project").clicked() {
                        let new_project_path = rfd::FileDialog::new().pick_folder();
                        if let Some(path) = new_project_path {
                            spawn_create_new_project_task(&mut commands, Templates::Blank, path);
                        }
                    }
                });
            });
        });

    egui::TopBottomPanel::bottom("status_bar")
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(14, 16, 18))
                .inner_margin(egui::Margin::symmetric(16, 10)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Projects: {}", project_list.0.len()));
                ui.separator();
                ui.label("Launcher: egui");
            });
        });

    egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(10, 12, 16))
                .inner_margin(18),
        )
        .show(ctx, |ui| {
            ui.add_space(8.0);
            ui.heading("Project Workspace");
            ui.label("Open an existing sandbox project or create a new one from the launcher.");
            ui.add_space(16.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                if project_list.0.is_empty() {
                    ui.group(|ui| {
                        ui.label("No projects yet.");
                        ui.label("Use \"New Project\" to create your first sandbox workspace.");
                    });
                    return;
                }

                let mut remove_path = None;

                for project in &project_list.0 {
                    ui.add_space(6.0);
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(24, 27, 34))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(52, 58, 72)))
                        .corner_radius(12)
                        .inner_margin(16)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.heading(project.name().unwrap_or_else(|| "Unknown".to_string()));
                                    ui.label(project.path.display().to_string());
                                });

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui.button("Open").clicked() {
                                            if !Path::new(&project.path).exists() {
                                                let project_name = project
                                                    .name()
                                                    .unwrap_or_else(|| "Unknown".to_string());
                                                push_notification(
                                                    &mut ui_state,
                                                    format!("Project not found: '{project_name}'"),
                                                );
                                                remove_path = Some(project.path.clone());
                                                return;
                                            }

                                            match run_project(project) {
                                                Ok(_) => {
                                                    exit.write(AppExit::Success);
                                                }
                                                Err(error) => match error.kind() {
                                                    ErrorKind::NotFound | ErrorKind::InvalidData => {
                                                        let project_name = project
                                                            .name()
                                                            .unwrap_or_else(|| "Unknown".to_string());
                                                        push_notification(
                                                            &mut ui_state,
                                                            format!(
                                                                "Failed to run project: '{project_name}'"
                                                            ),
                                                        );
                                                        remove_path = Some(project.path.clone());
                                                    }
                                                    _ => {
                                                        push_notification(
                                                            &mut ui_state,
                                                            format!("Error running project: '{error}'"),
                                                        );
                                                    }
                                                },
                                            }
                                        }

                                        if ui.button("Reveal").clicked() {
                                            push_notification(
                                                &mut ui_state,
                                                format!("Path: {}", project.path.display()),
                                            );
                                        }
                                    },
                                );
                            });
                        });
                }

                if let Some(path) = remove_path {
                    project_list.0.retain(|project| project.path != path);
                    set_project_list(project_list.0.clone());
                }
            });
        });

    if !ui_state.notifications.is_empty() {
        egui::Area::new("notifications".into())
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-20.0, 20.0))
            .show(ctx, |ui| {
                ui.set_width(320.0);
                for notification in &ui_state.notifications {
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(32, 36, 44))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(83, 92, 112)))
                        .corner_radius(10)
                        .inner_margin(12)
                        .show(ui, |ui| {
                            ui.label(&notification.text);
                        });
                    ui.add_space(8.0);
                }
            });
    }

    Ok(())
}
