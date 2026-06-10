use std::path::PathBuf;

use bevy::prelude::*;
use bevy_egui::egui::{self, TextureHandle};

use super::*;

fn template_tab(ui: &mut egui::Ui, active: bool, label: &str) -> egui::Response {
    let text =
        egui::RichText::new(label)
            .size(20.0)
            .color(if active { TAB_ACTIVE } else { TAB_INACTIVE });
    ui.add(
        egui::Button::new(text)
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::NONE)
            .frame(false),
    )
}

fn ensure_template_preview_texture(
    ctx: &egui::Context,
    ui_state: &mut LauncherUiState,
    card: &TemplateCard,
) -> Option<TextureHandle> {
    if let Some(texture) = ui_state.template_preview_textures.get(&card.template_id) {
        return Some(texture.clone());
    }

    let path = card.preview_image.as_ref()?;
    let texture_name = format!("template-preview-{}", card.template_id);
    let texture = load_png_texture(ctx, &texture_name, path)?;
    ui_state
        .template_preview_textures
        .insert(card.template_id.clone(), texture.clone());
    Some(texture)
}

fn paint_template_preview(ui: &mut egui::Ui, card: &TemplateCard, texture: Option<&TextureHandle>) {
    let width = 182.0;
    let height = 182.0;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    if let Some(texture) = texture {
        painter.rect_filled(rect, 6.0, egui::Color32::from_rgb(28, 28, 28));
        painter.image(
            texture.id(),
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
        return;
    }

    painter.rect_filled(rect, 6.0, card.bottom_color);
    let top_rect = egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.center().y));
    painter.rect_filled(top_rect, 6.0, card.top_color);
}

fn template_preview(ui: &mut egui::Ui, card: &TemplateCard, ui_state: &mut LauncherUiState) {
    let texture = ensure_template_preview_texture(ui.ctx(), ui_state, card);
    paint_template_preview(ui, card, texture.as_ref());
}

fn template_card(ui: &mut egui::Ui, card: &TemplateCard, ui_state: &mut LauncherUiState) {
    let locale = ui_state.locale;
    ui.vertical(|ui| {
        let frame_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(72, 72, 72));
        let hovered_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(244, 178, 47));
        let card_frame = egui::Frame::new()
            .fill(SURFACE_CARD)
            .stroke(frame_stroke)
            .corner_radius(6)
            .inner_margin(0);

        let card_response = card_frame
            .show(ui, |ui| {
                template_preview(ui, card, ui_state);
            })
            .response;
        let response = ui
            .interact(
                card_response.rect,
                ui.make_persistent_id(("template_card", card.template_id.as_str())),
                egui::Sense::click(),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand);

        if response.hovered() {
            ui.painter().rect_stroke(
                card_response.rect,
                6.0,
                hovered_stroke,
                egui::StrokeKind::Inside,
            );
        }

        if response.clicked() {
            ui_state.create_dialog = Some(CreateProjectDialogState {
                template: card.clone(),
                project_name: default_project_name(),
                storage_location: default_project_location(),
            });
        }

        ui.add_space(6.0);
        ui.label(egui::RichText::new(card.title(locale)).size(18.0));
    });
}

pub(super) fn render_create_page(
    ui: &mut egui::Ui,
    ui_state: &mut LauncherUiState,
    i18n: &Strings,
) {
    ensure_project_templates(ui_state);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 20.0;
        if template_tab(
            ui,
            ui_state.template_tab == TemplateTab::Project,
            i18n.project_templates,
        )
        .clicked()
        {
            ui_state.template_tab = TemplateTab::Project;
        }
        if template_tab(
            ui,
            ui_state.template_tab == TemplateTab::Mod,
            i18n.mod_templates,
        )
        .clicked()
        {
            ui_state.template_tab = TemplateTab::Mod;
        }
    });

    ui.add_space(18.0);

    let cards = match ui_state.template_tab {
        TemplateTab::Project => Some(ui_state.project_templates.clone()),
        TemplateTab::Mod => Some(ui_state.mod_templates.clone()),
    };

    let Some(cards) = cards else {
        return;
    };

    if cards.is_empty() {
        egui::Frame::new()
            .fill(SURFACE_BG)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(76, 76, 76)))
            .corner_radius(6)
            .inner_margin(egui::Margin::symmetric(18, 18))
            .show(ui, |ui| {
                let is_mod_tab = ui_state.template_tab == TemplateTab::Mod;
                ui.label(
                    egui::RichText::new(if is_mod_tab {
                        i18n.no_mod_templates
                    } else {
                        "No templates found."
                    })
                    .size(18.0),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(if is_mod_tab {
                        i18n.no_mod_templates_desc
                    } else {
                        "Add a template folder under project_templates/ to make it appear here."
                    })
                    .color(TEXT_MUTED),
                );
            });
        return;
    }

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
        for card in &cards {
            template_card(ui, card, ui_state);
        }
    });
}

pub(super) fn render_create_project_dialog(
    ctx: &egui::Context,
    commands: &mut Commands,
    ui_state: &mut LauncherUiState,
    i18n: &Strings,
) {
    let Some(template) = ui_state
        .create_dialog
        .as_ref()
        .map(|dialog| dialog.template.clone())
    else {
        return;
    };
    let preview_texture = ensure_template_preview_texture(ctx, ui_state, &template);

    let Some(dialog) = ui_state.create_dialog.as_mut() else {
        return;
    };

    let mut close_dialog =
        ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
    let mut create_project = false;
    let locale = ui_state.locale;

    egui::Window::new(template.title(locale))
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .default_width(360.0)
        .frame(
            egui::Frame::window(&ctx.style())
                .fill(SURFACE_BG)
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(78, 78, 78)))
                .corner_radius(6),
        )
        .show(ctx, |ui| {
            paint_template_preview(ui, &template, preview_texture.as_ref());
            ui.add_space(12.0);
            ui.label(egui::RichText::new(template.title(locale)).size(22.0));

            ui.add_space(18.0);
            ui.separator();
            ui.add_space(18.0);

            ui.label(egui::RichText::new(i18n.project_name).color(TEXT_MUTED));
            ui.add_space(6.0);
            ui.add(
                egui::TextEdit::singleline(&mut dialog.project_name).desired_width(f32::INFINITY),
            );

            ui.add_space(14.0);
            ui.label(egui::RichText::new(i18n.storage_location).color(TEXT_MUTED));
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut dialog.storage_location).desired_width(250.0),
                );
                if ui.button(i18n.browse).clicked() {
                    let file_dialog = if dialog.storage_location.trim().is_empty() {
                        rfd::FileDialog::new()
                    } else {
                        rfd::FileDialog::new().set_directory(&dialog.storage_location)
                    };

                    if let Some(path) = file_dialog.pick_folder() {
                        dialog.storage_location = path.display().to_string();
                    }
                }
            });

            ui.add_space(18.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 12.0;
                if ui
                    .add_sized([140.0, 34.0], egui::Button::new(i18n.cancel))
                    .clicked()
                {
                    close_dialog = true;
                }
                if ui
                    .add_sized([140.0, 34.0], egui::Button::new(i18n.create))
                    .clicked()
                {
                    create_project = true;
                }
            });
        });

    if close_dialog {
        ui_state.create_dialog = None;
        return;
    }

    if !create_project {
        return;
    }

    let Some(dialog) = ui_state.create_dialog.as_ref() else {
        return;
    };

    let project_name = dialog.project_name.trim();
    if project_name.is_empty() {
        push_notification(ui_state, i18n.invalid_project_name);
        return;
    }

    let storage_root = PathBuf::from(dialog.storage_location.trim());
    if dialog.storage_location.trim().is_empty() || !storage_root.is_dir() {
        push_notification(ui_state, i18n.invalid_storage_location);
        return;
    }

    let target_path = storage_root.join(project_name);
    if target_path.exists() {
        push_notification(ui_state, i18n.project_already_exists);
        return;
    }

    spawn_create_new_project_task(commands, dialog.template.template_id.clone(), target_path);
    push_notification(
        ui_state,
        format!("{} {}", i18n.creating_from, dialog.template.title(locale)),
    );
    ui_state.create_dialog = None;
}
