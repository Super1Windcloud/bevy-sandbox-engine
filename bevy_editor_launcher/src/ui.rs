use std::io::ErrorKind;
use std::path::Path;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use bevy_sandbox_engine::project::{run_project, set_project_list, templates::Templates};

use crate::{ProjectInfoList, spawn_create_new_project_task};

#[derive(Resource, Default)]
pub struct LauncherUiState {
    pub notifications: Vec<Notification>,
    pub page: LauncherPage,
    pub template_tab: TemplateTab,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum LauncherPage {
    #[default]
    Create,
    Projects,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum TemplateTab {
    #[default]
    Project,
    Mod,
}

pub struct Notification {
    pub text: String,
    pub ttl: Timer,
}

const LAUNCHER_BG: Color = Color::srgb(0.153, 0.153, 0.153);
const SIDEBAR_BG: egui::Color32 = egui::Color32::from_rgb(50, 50, 50);
const SIDEBAR_BRAND_BG: egui::Color32 = egui::Color32::from_rgb(64, 55, 45);
const SURFACE_BG: egui::Color32 = egui::Color32::from_rgb(42, 42, 42);
const SURFACE_SOFT: egui::Color32 = egui::Color32::from_rgb(70, 70, 70);
const SURFACE_CARD: egui::Color32 = egui::Color32::from_rgb(56, 56, 56);
const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(165, 165, 165);
const ACCENT: egui::Color32 = egui::Color32::from_rgb(231, 126, 27);
const TAB_ACTIVE: egui::Color32 = egui::Color32::from_rgb(232, 232, 232);
const TAB_INACTIVE: egui::Color32 = egui::Color32::from_rgb(130, 130, 130);

struct TemplateCard {
    template: Templates,
    title: &'static str,
    subtitle: &'static str,
    top_color: egui::Color32,
    bottom_color: egui::Color32,
}

const PROJECT_TEMPLATES: &[TemplateCard] = &[
    TemplateCard {
        template: Templates::GettingStarted,
        title: "Shooter Template",
        subtitle: "Top-down combat starter.",
        top_color: egui::Color32::from_rgb(118, 82, 43),
        bottom_color: egui::Color32::from_rgb(214, 157, 86),
    },
    TemplateCard {
        template: Templates::Blank,
        title: "Base Template",
        subtitle: "Minimal sandbox workspace.",
        top_color: egui::Color32::from_rgb(103, 140, 201),
        bottom_color: egui::Color32::from_rgb(208, 214, 220),
    },
];

const MOD_TEMPLATES: &[TemplateCard] = &[];

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

fn configure_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(egui::Color32::from_rgb(234, 234, 234));
    visuals.widgets.noninteractive.bg_fill = SURFACE_BG;
    visuals.widgets.inactive.bg_fill = SURFACE_BG;
    visuals.widgets.hovered.bg_fill = SURFACE_SOFT;
    visuals.widgets.active.bg_fill = SURFACE_SOFT;
    visuals.panel_fill = SURFACE_BG;
    visuals.window_fill = SURFACE_BG;
    visuals.selection.bg_fill = ACCENT;
    ctx.set_visuals(visuals);
}

fn nav_button(ui: &mut egui::Ui, selected: bool, icon: &str, label: &str) -> egui::Response {
    let fill = if selected {
        SURFACE_SOFT
    } else {
        egui::Color32::TRANSPARENT
    };
    let text_color = if selected {
        egui::Color32::WHITE
    } else {
        egui::Color32::from_rgb(220, 220, 220)
    };

    egui::Frame::new()
        .fill(fill)
        .corner_radius(6)
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add(
                egui::Button::new(
                    egui::RichText::new(format!("{icon}  {label}"))
                        .size(22.0)
                        .color(text_color),
                )
                .fill(egui::Color32::TRANSPARENT)
                .stroke(egui::Stroke::NONE)
                .frame(false),
            )
        })
        .inner
}

fn template_tab(ui: &mut egui::Ui, active: bool, label: &str) -> egui::Response {
    let text = egui::RichText::new(label)
        .size(20.0)
        .color(if active { TAB_ACTIVE } else { TAB_INACTIVE });
    ui.add(
        egui::Button::new(text)
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::NONE)
            .frame(false),
    )
}

fn template_preview(ui: &mut egui::Ui, card: &TemplateCard) {
    let width = 182.0;
    let height = 182.0;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 6.0, card.bottom_color);

    let top_rect = egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.center().y));
    painter.rect_filled(top_rect, 6.0, card.top_color);

    let horizon = rect.top() + 70.0;
    painter.line_segment(
        [egui::pos2(rect.left(), horizon), egui::pos2(rect.right(), horizon)],
        egui::Stroke::new(2.0, egui::Color32::from_white_alpha(40)),
    );

    if matches!(card.template, Templates::Blank) {
        let ground = egui::Rect::from_min_max(
            egui::pos2(rect.left(), horizon),
            egui::pos2(rect.right(), rect.bottom()),
        );
        for i in 0..8 {
            let x = ground.left() + i as f32 * 24.0;
            painter.line_segment(
                [egui::pos2(x, ground.top()), egui::pos2(x - 36.0, ground.bottom())],
                egui::Stroke::new(1.0, egui::Color32::from_gray(230)),
            );
            painter.line_segment(
                [
                    egui::pos2(ground.left(), ground.top() + i as f32 * 18.0),
                    egui::pos2(ground.right(), ground.top() + i as f32 * 18.0),
                ],
                egui::Stroke::new(1.0, egui::Color32::from_gray(220)),
            );
        }

        let ramp = vec![
            egui::pos2(rect.left() + 18.0, horizon + 24.0),
            egui::pos2(rect.left() + 84.0, horizon + 2.0),
            egui::pos2(rect.left() + 118.0, horizon + 22.0),
            egui::pos2(rect.left() + 40.0, horizon + 44.0),
        ];
        painter.add(egui::Shape::convex_polygon(
            ramp,
            egui::Color32::from_rgb(233, 233, 233),
            egui::Stroke::NONE,
        ));

        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(rect.left() + 108.0, horizon + 18.0),
                egui::vec2(58.0, 34.0),
            ),
            2.0,
            egui::Color32::from_rgb(167, 174, 185),
        );
    } else {
        painter.circle_filled(
            egui::pos2(rect.left() + 42.0, rect.top() + 95.0),
            16.0,
            egui::Color32::from_rgb(210, 94, 39),
        );
        painter.circle_filled(
            egui::pos2(rect.left() + 42.0, rect.top() + 95.0),
            8.0,
            egui::Color32::from_rgb(255, 197, 69),
        );

        for offset in [0.0, 10.0, 22.0, 35.0] {
            painter.line_segment(
                [
                    egui::pos2(rect.left() + 42.0 + offset, rect.top() + 72.0 - offset * 0.7),
                    egui::pos2(rect.left() + 46.0 + offset, rect.top() + 44.0 - offset),
                ],
                egui::Stroke::new(2.0, egui::Color32::from_rgb(253, 224, 84)),
            );
        }

        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(rect.left() + 18.0, rect.bottom() - 46.0),
                egui::vec2(26.0, 18.0),
            ),
            2.0,
            egui::Color32::from_rgb(126, 104, 77),
        );
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(rect.left() + 50.0, rect.bottom() - 54.0),
                egui::vec2(22.0, 22.0),
            ),
            2.0,
            egui::Color32::from_rgb(109, 88, 65),
        );
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(rect.right() - 34.0, rect.top() + 30.0),
                egui::vec2(20.0, 52.0),
            ),
            1.0,
            egui::Color32::from_rgb(103, 52, 39),
        );
    }
}

fn template_card(
    ui: &mut egui::Ui,
    commands: &mut Commands,
    card: &TemplateCard,
    ui_state: &mut LauncherUiState,
) {
    ui.vertical(|ui| {
        let card_frame = egui::Frame::new()
            .fill(SURFACE_CARD)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(72, 72, 72)))
            .corner_radius(6)
            .inner_margin(0);

        let response = card_frame
            .show(ui, |ui| {
                template_preview(ui, card);
            })
            .response;

        if response.clicked() {
            let new_project_path = rfd::FileDialog::new().pick_folder();
            if let Some(path) = new_project_path {
                spawn_create_new_project_task(commands, card.template, path);
                push_notification(ui_state, format!("Creating from {}", card.title));
            }
        }

        ui.add_space(6.0);
        ui.label(egui::RichText::new(card.title).size(18.0));
        ui.label(egui::RichText::new(card.subtitle).size(14.0).color(TEXT_MUTED));
    });
}

fn render_create_page(ui: &mut egui::Ui, commands: &mut Commands, ui_state: &mut LauncherUiState) {
    ui.horizontal(|ui| {
        if template_tab(ui, ui_state.template_tab == TemplateTab::Project, "Project Templates").clicked()
        {
            ui_state.template_tab = TemplateTab::Project;
        }
        if template_tab(ui, ui_state.template_tab == TemplateTab::Mod, "Mod Templates").clicked() {
            ui_state.template_tab = TemplateTab::Mod;
        }
    });

    ui.add_space(18.0);

    let cards = match ui_state.template_tab {
        TemplateTab::Project => PROJECT_TEMPLATES,
        TemplateTab::Mod => MOD_TEMPLATES,
    };

    if cards.is_empty() {
        egui::Frame::new()
            .fill(SURFACE_BG)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(76, 76, 76)))
            .corner_radius(6)
            .inner_margin(egui::Margin::symmetric(18, 18))
            .show(ui, |ui| {
                ui.label(egui::RichText::new("No mod templates yet.").size(18.0));
                ui.add_space(4.0);
                ui.label(egui::RichText::new("The home page still stays usable and never renders as an empty blank screen.").color(TEXT_MUTED));
            });
        return;
    }

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
        for card in cards {
            template_card(ui, commands, card, ui_state);
        }
    });
}

fn render_projects_page(
    ui: &mut egui::Ui,
    project_list: &mut ProjectInfoList,
    ui_state: &mut LauncherUiState,
    exit: &mut MessageWriter<AppExit>,
) {
    ui.label(egui::RichText::new("Projects").size(24.0));
    ui.add_space(14.0);

    if project_list.0.is_empty() {
        egui::Frame::new()
            .fill(SURFACE_BG)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(76, 76, 76)))
            .corner_radius(6)
            .inner_margin(egui::Margin::symmetric(18, 18))
            .show(ui, |ui| {
                ui.label(egui::RichText::new("No recent projects.").size(18.0));
                ui.add_space(4.0);
                ui.label(egui::RichText::new("Use the Create page to generate a project from a template.").color(TEXT_MUTED));
            });
        return;
    }

    let mut remove_path = None;

    egui::ScrollArea::vertical().show(ui, |ui| {
        for project in &project_list.0 {
            egui::Frame::new()
                .fill(SURFACE_CARD)
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(74, 74, 74)))
                .corner_radius(6)
                .inner_margin(egui::Margin::symmetric(14, 12))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(
                                    project.name().unwrap_or_else(|| "Unknown".to_string()),
                                )
                                .size(18.0),
                            );
                            ui.label(
                                egui::RichText::new(project.path.display().to_string())
                                    .color(TEXT_MUTED),
                            );
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
                                            ui_state,
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
                                                    ui_state,
                                                    format!(
                                                        "Failed to run project: '{project_name}'"
                                                    ),
                                                );
                                                remove_path = Some(project.path.clone());
                                            }
                                            _ => {
                                                push_notification(
                                                    ui_state,
                                                    format!("Error running project: '{error}'"),
                                                );
                                            }
                                        },
                                    }
                                }

                                if ui.button("Reveal").clicked() {
                                    push_notification(
                                        ui_state,
                                        format!("Path: {}", project.path.display()),
                                    );
                                }
                            },
                        );
                    });
                });
            ui.add_space(10.0);
        }
    });

    if let Some(path) = remove_path {
        project_list.0.retain(|project| project.path != path);
        set_project_list(project_list.0.clone());
    }
}

pub fn render_launcher_ui(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut project_list: ResMut<ProjectInfoList>,
    mut ui_state: ResMut<LauncherUiState>,
    mut exit: MessageWriter<AppExit>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    configure_visuals(ctx);

    egui::SidePanel::left("sidebar")
        .resizable(false)
        .exact_width(208.0)
        .frame(egui::Frame::new().fill(SIDEBAR_BG))
        .show(ctx, |ui| {
            ui.set_width(208.0);

            egui::Frame::new()
                .fill(SIDEBAR_BRAND_BG)
                .inner_margin(egui::Margin::same(22))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("◆").size(38.0).color(ACCENT));
                        ui.label(egui::RichText::new("◆").size(38.0).color(egui::Color32::from_rgb(255, 193, 51)));
                        ui.add_space(8.0);
                    });
                });

            ui.add_space(28.0);

            if nav_button(ui, ui_state.page == LauncherPage::Create, "◈", "Create").clicked() {
                ui_state.page = LauncherPage::Create;
            }
            ui.add_space(8.0);
            if nav_button(ui, ui_state.page == LauncherPage::Projects, "◻", "Projects").clicked() {
                ui_state.page = LauncherPage::Projects;
            }

            ui.add_space(ui.available_height() - 84.0);
            ui.label(egui::RichText::new("SandMod Engine").size(16.0).color(TEXT_MUTED));
            ui.add_space(4.0);
            ui.label(egui::RichText::new("1.3.247 (2188)").size(14.0).color(TEXT_MUTED));
        });

    egui::TopBottomPanel::top("top_bar")
        .exact_height(30.0)
        .frame(egui::Frame::new().fill(SURFACE_BG))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() - 170.0);
                ui.label(egui::RichText::new("◔").size(18.0).color(TEXT_MUTED));
                ui.label(egui::RichText::new("Forgetful_Stml9").size(15.0));
                ui.label(egui::RichText::new("⌄").size(14.0).color(TEXT_MUTED));
            });
        });

    egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(SURFACE_BG)
                .inner_margin(egui::Margin::symmetric(34, 36)),
        )
        .show(ctx, |ui| match ui_state.page {
            LauncherPage::Create => render_create_page(ui, &mut commands, &mut ui_state),
            LauncherPage::Projects => {
                render_projects_page(ui, &mut project_list, &mut ui_state, &mut exit)
            }
        });

    if !ui_state.notifications.is_empty() {
        egui::Area::new("notifications".into())
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-18.0, -18.0))
            .show(ctx, |ui| {
                ui.set_width(280.0);
                for notification in &ui_state.notifications {
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(58, 58, 58))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(86, 86, 86)))
                        .corner_radius(6)
                        .inner_margin(egui::Margin::symmetric(12, 10))
                        .show(ui, |ui| {
                            ui.label(&notification.text);
                        });
                    ui.add_space(8.0);
                }
            });
    }

    Ok(())
}
