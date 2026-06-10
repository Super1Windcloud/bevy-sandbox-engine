use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use bevy_egui::egui::{self, TextureHandle};
use bevy_sandbox_engine::project::{ProjectInfo, run_project};

use crate::ProjectInfoList;

use super::*;

fn project_thumbnail(ui: &mut egui::Ui, texture: Option<&TextureHandle>, offset: egui::Vec2) {
    let size = egui::vec2(72.0, 72.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let draw_rect = rect.translate(offset);
    let painter = ui.painter();
    painter.rect_filled(draw_rect, 2.0, egui::Color32::from_rgb(64, 64, 64));

    if let Some(texture) = texture {
        painter.image(
            texture.id(),
            draw_rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    } else {
        painter.text(
            draw_rect.center(),
            egui::Align2::CENTER_CENTER,
            "BS",
            egui::FontId::proportional(18.0),
            egui::Color32::from_rgb(220, 220, 220),
        );
    }
}

fn project_more_button(ui: &mut egui::Ui, offset: egui::Vec2) -> egui::Response {
    let response = ui.add(
        egui::Button::new("")
            .min_size(egui::vec2(24.0, 24.0))
            .frame(false),
    );
    let painter = ui.painter_at(response.rect.translate(offset));
    let color = if response.hovered() {
        egui::Color32::from_rgb(220, 220, 220)
    } else {
        TEXT_MUTED
    };
    let center_x = response.rect.center().x + offset.x;
    let center_y = response.rect.center().y + offset.y;
    for dot_offset in [-5.0_f32, 0.0, 5.0] {
        painter.circle_filled(egui::pos2(center_x, center_y + dot_offset), 1.8, color);
    }
    response
}

fn render_project_card_menu(
    ctx: &egui::Context,
    ui_state: &mut LauncherUiState,
    project_list: &ProjectInfoList,
    i18n: &Strings,
) -> Option<PathBuf> {
    let Some((project_path, anchor, just_opened)) = ui_state.project_card_menu.as_ref().map(
        |menu_state| {
            (
                menu_state.project_path.clone(),
                menu_state.anchor,
                menu_state.just_opened,
            )
        },
    ) else {
        return None;
    };

    let mut close_menu = false;
    let mut remove_path = None;
    let project_name = project_list
        .0
        .iter()
        .find(|project| project.path == project_path)
        .and_then(|project| project.name())
        .unwrap_or_else(|| "Unknown".to_string());

    let area_response = egui::Area::new(egui::Id::new(("project-card-menu", &project_path)))
        .order(egui::Order::Foreground)
        .fixed_pos(anchor)
        .show(ctx, |ui| {
            egui::Frame::popup(&ctx.style())
                .fill(SURFACE_BG)
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(78, 78, 78)))
                .corner_radius(6)
                .show(ui, |ui| {
                    ui.set_min_width(168.0);

                    if ui.button(i18n.rename_project).clicked() {
                        ui_state.rename_dialog = Some(RenameProjectDialogState {
                            project_path: project_path.clone(),
                            project_name: project_name.clone(),
                        });
                        close_menu = true;
                    }

                    if ui.button(i18n.open_project_folder).clicked() {
                        if let Err(error) = open_project_folder(&project_path) {
                            push_notification(
                                ui_state,
                                format!("{}: {error}", i18n.open_folder_failed),
                            );
                        }
                        close_menu = true;
                    }

                    if ui.button(i18n.remove_project).clicked() {
                        remove_path = Some(project_path.clone());
                        close_menu = true;
                    }
                });
        });

    let menu_rect = area_response.response.rect;
    let clicked_outside = if just_opened {
        false
    } else {
        ctx.input(|input| {
            input.pointer.any_click()
                && input
                    .pointer
                    .interact_pos()
                    .is_some_and(|pointer_pos| !menu_rect.contains(pointer_pos))
        })
    };

    if close_menu || clicked_outside {
        ui_state.project_card_menu = None;
    } else if let Some(menu_state) = ui_state.project_card_menu.as_mut() {
        menu_state.just_opened = false;
    }

    remove_path
}

fn render_project_card(
    ui: &mut egui::Ui,
    project: &ProjectInfo,
    ui_state: &mut LauncherUiState,
    i18n: &Strings,
) -> (egui::Response, Option<egui::Rect>) {
    let card_rect = ui
        .allocate_exact_size(egui::vec2(ui.available_width(), 100.0), egui::Sense::hover())
        .0;

    let pointer_pos = ui.ctx().pointer_hover_pos().filter(|pos| card_rect.contains(*pos));
    let normalized = pointer_pos.map(|pointer| {
        let local = pointer - card_rect.min;
        let nx = ((local.x / card_rect.width()) - 0.5).clamp(-0.5, 0.5);
        let ny = ((local.y / card_rect.height()) - 0.5).clamp(-0.5, 0.5);
        egui::vec2(nx, ny)
    });
    let content_offset = normalized
        .map(|n| egui::vec2(n.x * 6.0, n.y * 4.0))
        .unwrap_or(egui::Vec2::ZERO);
    let thumb_offset = content_offset * 0.9;
    let text_offset = content_offset * 0.35;
    let glow_center = pointer_pos.unwrap_or(card_rect.center());

    let painter = ui.painter_at(card_rect);
    painter.rect_filled(card_rect, 6.0, SURFACE_CARD);
    painter.rect_stroke(
        card_rect,
        6.0,
        egui::Stroke::new(
            1.0,
            if pointer_pos.is_some() {
                egui::Color32::from_rgb(118, 118, 118)
            } else {
                egui::Color32::from_rgb(70, 70, 70)
            },
        ),
        egui::StrokeKind::Inside,
    );
    if pointer_pos.is_some() {
        painter.circle_filled(
            glow_center,
            68.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 10),
        );
    }

    let inner_rect = card_rect.shrink2(egui::vec2(14.0, 14.0));
    let mut menu_rect = None;
    ui.allocate_ui_at_rect(inner_rect.translate(text_offset), |ui| {
        ui.set_clip_rect(card_rect.shrink(2.0));
        ui.horizontal_top(|ui| {
            project_thumbnail(ui, ui_state.brand_texture.as_ref(), thumb_offset - text_offset);
            ui.add_space(14.0);

            let info_width = (ui.available_width() - 34.0).max(160.0);
            ui.vertical(|ui| {
                ui.set_width(info_width);
                ui.set_max_width(info_width);
                ui.spacing_mut().item_spacing.y = 4.0;

                let project_name = project.name().unwrap_or_else(|| "Unknown".to_string());
                let project_id = project_stable_id(project);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!("{project_name} (ID: {project_id})"))
                            .size(18.0)
                            .strong(),
                    )
                    .truncate(),
                );
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!(
                            "{}: {}",
                            i18n.modified_at,
                            project_modified_at(project)
                        ))
                        .size(14.0)
                        .color(TEXT_MUTED),
                    )
                    .truncate(),
                );
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!(
                            "{}: {}",
                            i18n.path_prefix,
                            project.path.display()
                        ))
                        .size(14.0)
                        .color(TEXT_MUTED),
                    )
                    .truncate(),
                );
            });

            ui.scope(|ui| {
                ui.style_mut().spacing.button_padding = egui::vec2(6.0, 2.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                    let menu_response = project_more_button(ui, content_offset * 0.2);
                    menu_rect = Some(menu_response.rect.translate(text_offset));
                    if menu_response.clicked() {
                        ui_state.project_card_menu = Some(ProjectCardMenuState {
                            project_path: project.path.clone(),
                            anchor: egui::pos2(
                                menu_response.rect.right() - 168.0 + text_offset.x,
                                menu_response.rect.bottom() + 4.0 + text_offset.y,
                            ),
                            just_opened: true,
                        });
                    }
                });
            });
        });
    });

    let clickable_rect = if let Some(menu_rect) = menu_rect {
        egui::Rect::from_min_max(
            card_rect.min,
            egui::pos2((menu_rect.min.x - 8.0).max(card_rect.min.x), card_rect.max.y),
        )
    } else {
        card_rect
    };

    let response = ui
        .interact(
            clickable_rect,
            ui.make_persistent_id(("project_card", &project.path)),
            egui::Sense::click(),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand);

    (response, menu_rect)
}

pub(super) fn render_rename_project_dialog(
    ctx: &egui::Context,
    project_list: &mut ProjectInfoList,
    ui_state: &mut LauncherUiState,
    i18n: &Strings,
) {
    let Some(dialog) = ui_state.rename_dialog.as_mut() else {
        return;
    };

    let mut close_dialog = false;
    let mut save_name = false;

    egui::Window::new(i18n.rename_project)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .default_width(320.0)
        .frame(
            egui::Frame::window(&ctx.style())
                .fill(SURFACE_BG)
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(78, 78, 78)))
                .corner_radius(6),
        )
        .show(ctx, |ui| {
            ui.label(egui::RichText::new(i18n.project_name).color(TEXT_MUTED));
            ui.add_space(6.0);
            ui.add(
                egui::TextEdit::singleline(&mut dialog.project_name).desired_width(f32::INFINITY),
            );

            ui.add_space(18.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 12.0;
                if ui
                    .add_sized([120.0, 34.0], egui::Button::new(i18n.cancel))
                    .clicked()
                {
                    close_dialog = true;
                }
                if ui
                    .add_sized([120.0, 34.0], egui::Button::new(i18n.save))
                    .clicked()
                {
                    save_name = true;
                }
            });
        });

    if close_dialog {
        ui_state.rename_dialog = None;
        return;
    }

    if !save_name {
        return;
    }

    let Some(dialog) = ui_state.rename_dialog.as_ref() else {
        return;
    };
    let project_name = dialog.project_name.trim();
    if project_name.is_empty() {
        push_notification(ui_state, i18n.rename_project_failed);
        return;
    }

    if let Some(project) = project_list
        .0
        .iter_mut()
        .find(|project| project.path == dialog.project_path)
    {
        project.display_name = Some(project_name.to_string());
        set_project_list(project_list.0.clone());
    }

    ui_state.rename_dialog = None;
}

pub(super) fn render_projects_page(
    ui: &mut egui::Ui,
    project_list: &mut ProjectInfoList,
    ui_state: &mut LauncherUiState,
    exit: &mut MessageWriter<AppExit>,
    i18n: &Strings,
) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(i18n.projects_title).size(24.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 10.0;

            if ui.button(i18n.new_project).clicked() {
                ui_state.page = LauncherPage::Create;
            }

            if ui.button(i18n.import_project).clicked() {
                if let Some(project_path) = rfd::FileDialog::new().pick_folder() {
                    import_project_folder(project_list, ui_state, i18n, project_path);
                }
            }

            if ui.button(i18n.refresh).clicked() {
                project_list.0 = get_local_projects();
            }
        });
    });
    ui.add_space(14.0);

    if project_list.0.is_empty() {
        ui.label(egui::RichText::new(i18n.no_recent_projects).size(18.0));
        ui.add_space(4.0);
        ui.label(egui::RichText::new(i18n.no_recent_projects_desc).color(TEXT_MUTED));
        return;
    }

    let mut remove_path = None;
    let ctx = ui.ctx().clone();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for project in &project_list.0 {
            let (card_response, menu_rect) = render_project_card(ui, project, ui_state, i18n);
            let pointer_pos = card_response.interact_pointer_pos();
            let over_menu = pointer_pos
                .zip(menu_rect)
                .is_some_and(|(pointer_pos, menu_rect)| menu_rect.contains(pointer_pos));
            if card_response.double_clicked() && !over_menu {
                if !Path::new(&project.path).exists() {
                    let project_name = project.name().unwrap_or_else(|| "Unknown".to_string());
                    push_notification(
                        ui_state,
                        format!("{}: '{project_name}'", i18n.project_not_found),
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
                            let project_name =
                                project.name().unwrap_or_else(|| "Unknown".to_string());
                            push_notification(
                                ui_state,
                                format!("{}: '{project_name}'", i18n.failed_to_run_project),
                            );
                            remove_path = Some(project.path.clone());
                        }
                        _ => {
                            push_notification(
                                ui_state,
                                format!("{}: {error}", i18n.error_running_project),
                            );
                        }
                    },
                }
            }
            ui.add_space(8.0);
        }
    });

    if remove_path.is_none() {
        remove_path = render_project_card_menu(&ctx, ui_state, project_list, i18n);
    }

    if let Some(path) = remove_path {
        project_list.0.retain(|project| project.path != path);
        set_project_list(project_list.0.clone());
        push_notification(ui_state, i18n.project_removed);
    }
}
