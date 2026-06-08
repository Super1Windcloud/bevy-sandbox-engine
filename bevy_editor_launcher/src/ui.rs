use std::io::ErrorKind;
use std::ops::Mul;
use std::path::Path;

use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{
        self, ColorImage, FontData, FontDefinitions, FontFamily, TextureHandle, TextureOptions,
    },
};
use bevy_sandbox_engine::project::{run_project, set_project_list, templates::Templates};
use sys_locale::get_locale;

use crate::{ProjectInfoList, spawn_create_new_project_task};

#[derive(Resource)]
pub struct LauncherUiState {
    pub notifications: Vec<Notification>,
    pub page: LauncherPage,
    pub template_tab: TemplateTab,
    pub locale: LauncherLocale,
    pub brand_texture: Option<TextureHandle>,
    pub fonts_configured: bool,
}

impl Default for LauncherUiState {
    fn default() -> Self {
        Self {
            notifications: Vec::new(),
            page: LauncherPage::Create,
            template_tab: TemplateTab::Project,
            locale: LauncherLocale::detect(),
            brand_texture: None,
            fonts_configured: false,
        }
    }
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LauncherLocale {
    ZhCn,
    EnUs,
}

impl LauncherLocale {
    fn detect() -> Self {
        let locale = get_locale().unwrap_or_else(|| "en-US".to_string());
        let normalized = locale.to_ascii_lowercase();
        if normalized.starts_with("zh") {
            Self::ZhCn
        } else {
            Self::EnUs
        }
    }
}

pub struct Notification {
    pub text: String,
    pub ttl: Timer,
}

const LAUNCHER_BG: Color = Color::srgb(0.153, 0.153, 0.153);
const SIDEBAR_BG: egui::Color32 = egui::Color32::from_rgb(50, 50, 50);
const SURFACE_BG: egui::Color32 = egui::Color32::from_rgb(42, 42, 42);
const SURFACE_SOFT: egui::Color32 = egui::Color32::from_rgb(70, 70, 70);
const SURFACE_CARD: egui::Color32 = egui::Color32::from_rgb(56, 56, 56);
const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(165, 165, 165);
const TAB_ACTIVE: egui::Color32 = egui::Color32::from_rgb(232, 232, 232);
const TAB_INACTIVE: egui::Color32 = egui::Color32::from_rgb(130, 130, 130);
const BRAND_TEXTURE_NAME: &str = "launcher-brand-logo";
const CJK_FONT_NAME: &str = "launcher-cjk-font";
const BRAND_ICON_SIZE: f32 = 112.0;

pub struct Strings {
    pub nav_create: &'static str,
    pub nav_projects: &'static str,
    pub project_templates: &'static str,
    pub mod_templates: &'static str,
    pub no_mod_templates: &'static str,
    pub no_mod_templates_desc: &'static str,
    pub projects_title: &'static str,
    pub no_recent_projects: &'static str,
    pub no_recent_projects_desc: &'static str,
    pub open: &'static str,
    pub reveal: &'static str,
    pub creating_from: &'static str,
    pub project_not_found: &'static str,
    pub failed_to_run_project: &'static str,
    pub error_running_project: &'static str,
    pub path_prefix: &'static str,
    pub created_project: &'static str,
    pub failed_to_create_project: &'static str,
    pub engine_name: &'static str,
}

pub fn strings(locale: LauncherLocale) -> Strings {
    match locale {
        LauncherLocale::ZhCn => Strings {
            nav_create: "创建",
            nav_projects: "项目",
            project_templates: "项目模板",
            mod_templates: "Mod模板",
            no_mod_templates: "暂时没有 Mod 模板。",
            no_mod_templates_desc: "首页会始终显示有效内容，不会出现空白页。",
            projects_title: "项目",
            no_recent_projects: "暂无最近项目。",
            no_recent_projects_desc: "请先从创建页选择模板生成项目。",
            open: "打开",
            reveal: "定位",
            creating_from: "正在从模板创建",
            project_not_found: "项目不存在",
            failed_to_run_project: "启动项目失败",
            error_running_project: "运行项目时出错",
            path_prefix: "路径",
            created_project: "已创建项目",
            failed_to_create_project: "创建项目失败",
            engine_name: "SandBox Engine",
        },
        LauncherLocale::EnUs => Strings {
            nav_create: "Create",
            nav_projects: "Projects",
            project_templates: "Project Templates",
            mod_templates: "Mod Templates",
            no_mod_templates: "No mod templates yet.",
            no_mod_templates_desc: "The home page always keeps meaningful content instead of rendering blank.",
            projects_title: "Projects",
            no_recent_projects: "No recent projects.",
            no_recent_projects_desc: "Create a project from a template first.",
            open: "Open",
            reveal: "Reveal",
            creating_from: "Creating from",
            project_not_found: "Project not found",
            failed_to_run_project: "Failed to run project",
            error_running_project: "Error running project",
            path_prefix: "Path",
            created_project: "Created project",
            failed_to_create_project: "Failed to create project",
            engine_name: "SandBox Engine",
        },
    }
}

struct TemplateCard {
    template: Templates,
    title_zh: &'static str,
    title_en: &'static str,
    subtitle_zh: &'static str,
    subtitle_en: &'static str,
    top_color: egui::Color32,
    bottom_color: egui::Color32,
}

impl TemplateCard {
    fn title(&self, locale: LauncherLocale) -> &'static str {
        match locale {
            LauncherLocale::ZhCn => self.title_zh,
            LauncherLocale::EnUs => self.title_en,
        }
    }

    fn subtitle(&self, locale: LauncherLocale) -> &'static str {
        match locale {
            LauncherLocale::ZhCn => self.subtitle_zh,
            LauncherLocale::EnUs => self.subtitle_en,
        }
    }
}

const PROJECT_TEMPLATES: &[TemplateCard] = &[
    TemplateCard {
        template: Templates::GettingStarted,
        title_zh: "俯视射击模板",
        title_en: "Shooter Template",
        subtitle_zh: "适合快速开始俯视角战斗玩法。",
        subtitle_en: "Top-down combat starter.",
        top_color: egui::Color32::from_rgb(118, 82, 43),
        bottom_color: egui::Color32::from_rgb(214, 157, 86),
    },
    TemplateCard {
        template: Templates::Blank,
        title_zh: "基础模板",
        title_en: "Base Template",
        subtitle_zh: "最小化的沙盒项目结构。",
        subtitle_en: "Minimal sandbox workspace.",
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

pub fn sync_system_locale(mut ui_state: ResMut<LauncherUiState>) {
    ui_state.locale = LauncherLocale::detect();
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
    ctx.set_visuals(visuals);
}

fn load_cjk_font_bytes() -> Option<Vec<u8>> {
    let embedded_font = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("assets")
        .join("fonts")
        .join("NotoSansCJKsc-Regular.otf");

    if let Ok(bytes) = std::fs::read(&embedded_font) {
        return Some(bytes);
    }

    let font_candidates = [
        r"C:\Windows\Fonts\msyh.ttc",
        r"C:\Windows\Fonts\simhei.ttf",
        r"C:\Windows\Fonts\Deng.ttf",
        r"C:\Windows\Fonts\simsun.ttc",
    ];

    for path in font_candidates {
        if let Ok(bytes) = std::fs::read(path) {
            return Some(bytes);
        }
    }

    None
}

fn ensure_fonts(ctx: &egui::Context, ui_state: &mut LauncherUiState) {
    if ui_state.fonts_configured {
        return;
    }

    let Some(font_bytes) = load_cjk_font_bytes() else {
        ui_state.fonts_configured = true;
        return;
    };

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        CJK_FONT_NAME.to_string(),
        FontData::from_owned(font_bytes).into(),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, CJK_FONT_NAME.to_string());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, CJK_FONT_NAME.to_string());

    ctx.set_fonts(fonts);
    ui_state.fonts_configured = true;
}

fn ensure_brand_texture(ctx: &egui::Context, ui_state: &mut LauncherUiState) {
    if ui_state.brand_texture.is_some() {
        return;
    }

    let assets_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("assets");
    let preferred_icon_path = assets_dir.join("logo-100px.png");
    let fallback_icon_path = assets_dir.join("logo.png");

    let icon_path = if preferred_icon_path.exists() {
        preferred_icon_path
    } else {
        fallback_icon_path
    };

    let Ok(image) = image::open(icon_path) else {
        return;
    };
    let image = image.into_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.as_raw();
    let color_image = ColorImage::from_rgba_unmultiplied(size, pixels);
    let texture = ctx.load_texture(BRAND_TEXTURE_NAME, color_image, TextureOptions::LINEAR);
    ui_state.brand_texture = Some(texture);
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
        [
            egui::pos2(rect.left(), horizon),
            egui::pos2(rect.right(), horizon),
        ],
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
                [
                    egui::pos2(x, ground.top()),
                    egui::pos2(x - 36.0, ground.bottom()),
                ],
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
                    egui::pos2(
                        rect.left() + 42.0 + offset,
                        rect.top() + 72.0 - offset * 0.7,
                    ),
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
    i18n: &Strings,
) {
    let locale = ui_state.locale;
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
                push_notification(
                    ui_state,
                    format!("{} {}", i18n.creating_from, card.title(locale)),
                );
            }
        }

        ui.add_space(6.0);
        ui.label(egui::RichText::new(card.title(locale)).size(18.0));
        ui.label(
            egui::RichText::new(card.subtitle(locale))
                .size(14.0)
                .color(TEXT_MUTED),
        );
    });
}

fn render_create_page(
    ui: &mut egui::Ui,
    commands: &mut Commands,
    ui_state: &mut LauncherUiState,
    i18n: &Strings,
) {
    ui.horizontal(|ui| {
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
                ui.label(egui::RichText::new(i18n.no_mod_templates).size(18.0));
                ui.add_space(4.0);
                ui.label(egui::RichText::new(i18n.no_mod_templates_desc).color(TEXT_MUTED));
            });
        return;
    }

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
        for card in cards {
            template_card(ui, commands, card, ui_state, i18n);
        }
    });
}

fn render_projects_page(
    ui: &mut egui::Ui,
    project_list: &mut ProjectInfoList,
    ui_state: &mut LauncherUiState,
    exit: &mut MessageWriter<AppExit>,
    i18n: &Strings,
) {
    ui.label(egui::RichText::new(i18n.projects_title).size(24.0));
    ui.add_space(14.0);

    if project_list.0.is_empty() {
        egui::Frame::new()
            .fill(SURFACE_BG)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(76, 76, 76)))
            .corner_radius(6)
            .inner_margin(egui::Margin::symmetric(18, 18))
            .show(ui, |ui| {
                ui.label(egui::RichText::new(i18n.no_recent_projects).size(18.0));
                ui.add_space(4.0);
                ui.label(egui::RichText::new(i18n.no_recent_projects_desc).color(TEXT_MUTED));
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

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(i18n.open).clicked() {
                                if !Path::new(&project.path).exists() {
                                    let project_name =
                                        project.name().unwrap_or_else(|| "Unknown".to_string());
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
                                            let project_name = project
                                                .name()
                                                .unwrap_or_else(|| "Unknown".to_string());
                                            push_notification(
                                                ui_state,
                                                format!(
                                                    "{}: '{project_name}'",
                                                    i18n.failed_to_run_project
                                                ),
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

                            if ui.button(i18n.reveal).clicked() {
                                push_notification(
                                    ui_state,
                                    format!("{}: {}", i18n.path_prefix, project.path.display()),
                                );
                            }
                        });
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
    ensure_fonts(ctx, &mut ui_state);
    configure_visuals(ctx);
    ensure_brand_texture(ctx, &mut ui_state);
    let i18n = strings(ui_state.locale);

    egui::SidePanel::left("sidebar")
        .resizable(false)
        .exact_width(208.0)
        .frame(egui::Frame::new().fill(SIDEBAR_BG))
        .show(ctx, |ui| {
            ui.set_width(208.0);

            egui::Frame::new()
                .fill(SIDEBAR_BG)
                .inner_margin(egui::Margin::same(20))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        if let Some(texture) = &ui_state.brand_texture {
                            ui.add(
                                egui::Image::new(texture)
                                    .fit_to_exact_size(egui::vec2(BRAND_ICON_SIZE, BRAND_ICON_SIZE)),
                            );
                        }
                        ui.add_space(8.0);
                    });
                });

            ui.add_space(28.0);

            if nav_button(
                ui,
                ui_state.page == LauncherPage::Create,
                "◈",
                i18n.nav_create,
            )
            .clicked()
            {
                ui_state.page = LauncherPage::Create;
            }
            ui.add_space(8.0);
            if nav_button(
                ui,
                ui_state.page == LauncherPage::Projects,
                "◻",
                i18n.nav_projects,
            )
            .clicked()
            {
                ui_state.page = LauncherPage::Projects;
            }

            let remaining = (ui.available_height() - 56.0).max(0.0);
            ui.add_space(remaining);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(i18n.engine_name)
                        .size(16.0)
                        .color(TEXT_MUTED),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(env!("CARGO_PKG_VERSION"))
                        .size(14.0)
                        .color(TEXT_MUTED),
                );
            });
        });

    egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(SURFACE_BG)
                .inner_margin(egui::Margin::symmetric(34, 36)),
        )
        .show(ctx, |ui| match ui_state.page {
            LauncherPage::Create => render_create_page(ui, &mut commands, &mut ui_state, &i18n),
            LauncherPage::Projects => {
                render_projects_page(ui, &mut project_list, &mut ui_state, &mut exit, &i18n)
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
