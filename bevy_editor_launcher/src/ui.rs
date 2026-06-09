use std::io::ErrorKind;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{
        self, ColorImage, FontData, FontDefinitions, FontFamily, TextureHandle, TextureOptions,
    },
};
use bevy_sandbox_engine::project::{
    ProjectInfo, get_local_projects, run_project, set_project_list,
    templates::{TemplateDefinition, TemplateKind, TemplatePreviewStyle, list_templates},
};
use chrono::{DateTime, Local};
use sys_locale::get_locale;

use crate::{ProjectInfoList, spawn_create_new_project_task};

#[derive(Resource)]
pub struct LauncherUiState {
    pub notifications: Vec<Notification>,
    pub page: LauncherPage,
    pub template_tab: TemplateTab,
    pub locale: LauncherLocale,
    pub brand_texture: Option<TextureHandle>,
    pub nav_create_texture: Option<TextureHandle>,
    pub nav_projects_texture: Option<TextureHandle>,
    pub fonts_configured: bool,
    project_templates: Vec<TemplateCard>,
    mod_templates: Vec<TemplateCard>,
    templates_loaded: bool,
    create_dialog: Option<CreateProjectDialogState>,
    rename_dialog: Option<RenameProjectDialogState>,
}

impl Default for LauncherUiState {
    fn default() -> Self {
        Self {
            notifications: Vec::new(),
            page: LauncherPage::Create,
            template_tab: TemplateTab::Project,
            locale: LauncherLocale::detect(),
            brand_texture: None,
            nav_create_texture: None,
            nav_projects_texture: None,
            fonts_configured: false,
            project_templates: Vec::new(),
            mod_templates: Vec::new(),
            templates_loaded: false,
            create_dialog: None,
            rename_dialog: None,
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
const NAV_CREATE_TEXTURE_NAME: &str = "launcher-nav-create";
const NAV_PROJECTS_TEXTURE_NAME: &str = "launcher-nav-projects";
const NAV_HOVER_FILL: egui::Color32 = egui::Color32::from_rgb(58, 58, 58);
const NAV_HOVER_STROKE: egui::Color32 = egui::Color32::from_rgb(96, 96, 96);

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
    pub refresh: &'static str,
    pub import_project: &'static str,
    pub new_project: &'static str,
    pub modified_at: &'static str,
    pub rename_project: &'static str,
    pub open_project_folder: &'static str,
    pub remove_project: &'static str,
    pub save: &'static str,
    pub creating_from: &'static str,
    pub project_not_found: &'static str,
    pub failed_to_run_project: &'static str,
    pub error_running_project: &'static str,
    pub path_prefix: &'static str,
    pub failed_to_create_project: &'static str,
    pub imported_project: &'static str,
    pub invalid_project_folder: &'static str,
    pub project_already_imported: &'static str,
    pub open_folder_failed: &'static str,
    pub rename_project_failed: &'static str,
    pub project_removed: &'static str,
    pub engine_name: &'static str,
    pub project_name: &'static str,
    pub storage_location: &'static str,
    pub browse: &'static str,
    pub cancel: &'static str,
    pub create: &'static str,
    pub invalid_project_name: &'static str,
    pub invalid_storage_location: &'static str,
    pub project_already_exists: &'static str,
}

pub fn strings(locale: LauncherLocale) -> Strings {
    match locale {
        LauncherLocale::ZhCn => Strings {
            nav_create: "创建",
            nav_projects: "项目",
            project_templates: "项目模板",
            mod_templates: "Mod模板",
            no_mod_templates: "暂时没有 Mod 模板。",
            no_mod_templates_desc: "",
            projects_title: "我的项目",
            no_recent_projects: "暂无最近项目。",
            no_recent_projects_desc: "请先创建或导入项目。",
            refresh: "刷新",
            import_project: "导入项目",
            new_project: "新建项目",
            modified_at: "最近修改",
            rename_project: "更改名称",
            open_project_folder: "打开项目文件夹",
            remove_project: "删除项目",
            save: "保存",
            creating_from: "正在从模板创建",
            project_not_found: "项目不存在",
            failed_to_run_project: "启动项目失败",
            error_running_project: "运行项目时出错",
            path_prefix: "路径",
            failed_to_create_project: "创建项目失败",
            imported_project: "已导入项目",
            invalid_project_folder: "所选目录不是有效项目",
            project_already_imported: "项目已在列表中",
            open_folder_failed: "打开项目文件夹失败",
            rename_project_failed: "项目名称不能为空",
            project_removed: "已删除项目",
            engine_name: "SandBox Engine",
            project_name: "项目名称",
            storage_location: "存储位置",
            browse: "浏览",
            cancel: "取消",
            create: "创建",
            invalid_project_name: "项目名称不能为空",
            invalid_storage_location: "存储位置无效",
            project_already_exists: "目标项目目录已存在",
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
            no_recent_projects_desc: "Create or import a project first.",
            refresh: "Refresh",
            import_project: "Import Project",
            new_project: "New Project",
            modified_at: "Modified",
            rename_project: "Rename",
            open_project_folder: "Open Project Folder",
            remove_project: "Remove Project",
            save: "Save",
            creating_from: "Creating from",
            project_not_found: "Project not found",
            failed_to_run_project: "Failed to run project",
            error_running_project: "Error running project",
            path_prefix: "Path",
            failed_to_create_project: "Failed to create project",
            imported_project: "Imported project",
            invalid_project_folder: "Selected folder is not a valid project",
            project_already_imported: "Project is already in the list",
            open_folder_failed: "Failed to open project folder",
            rename_project_failed: "Project name is required",
            project_removed: "Removed project",
            engine_name: "SandBox Engine",
            project_name: "Project Name",
            storage_location: "Location",
            browse: "Browse",
            cancel: "Cancel",
            create: "Create",
            invalid_project_name: "Project name is required",
            invalid_storage_location: "Storage location is invalid",
            project_already_exists: "Target project directory already exists",
        },
    }
}

#[derive(Clone)]
struct TemplateCard {
    template_id: String,
    title_zh: String,
    title_en: String,
    subtitle_zh: String,
    subtitle_en: String,
    top_color: egui::Color32,
    bottom_color: egui::Color32,
    preview_style: TemplatePreviewStyle,
}

struct CreateProjectDialogState {
    template: TemplateCard,
    project_name: String,
    storage_location: String,
}

struct RenameProjectDialogState {
    project_path: PathBuf,
    project_name: String,
}

impl TemplateCard {
    fn title(&self, locale: LauncherLocale) -> &str {
        match locale {
            LauncherLocale::ZhCn => &self.title_zh,
            LauncherLocale::EnUs => &self.title_en,
        }
    }

    fn subtitle(&self, locale: LauncherLocale) -> &str {
        match locale {
            LauncherLocale::ZhCn => &self.subtitle_zh,
            LauncherLocale::EnUs => &self.subtitle_en,
        }
    }
}

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

fn default_project_location() -> String {
    if let Some(user_profile) = std::env::var_os("USERPROFILE") {
        let downloads = Path::new(&user_profile).join("Downloads");
        if downloads.exists() {
            return downloads.display().to_string();
        }
        return Path::new(&user_profile).display().to_string();
    }

    std::env::current_dir()
        .ok()
        .unwrap_or_else(|| Path::new(".").to_path_buf())
        .display()
        .to_string()
}

fn default_project_name() -> String {
    "Project".to_string()
}

fn default_card_palette(style: TemplatePreviewStyle) -> (egui::Color32, egui::Color32) {
    match style {
        TemplatePreviewStyle::Grid => (
            egui::Color32::from_rgb(103, 140, 201),
            egui::Color32::from_rgb(208, 214, 220),
        ),
        TemplatePreviewStyle::Shooter => (
            egui::Color32::from_rgb(118, 82, 43),
            egui::Color32::from_rgb(214, 157, 86),
        ),
        TemplatePreviewStyle::Generic => (
            egui::Color32::from_rgb(86, 102, 124),
            egui::Color32::from_rgb(170, 182, 198),
        ),
    }
}

fn parse_color_hex(value: &str) -> Option<egui::Color32> {
    let value = value.trim().trim_start_matches('#');
    if value.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&value[0..2], 16).ok()?;
    let g = u8::from_str_radix(&value[2..4], 16).ok()?;
    let b = u8::from_str_radix(&value[4..6], 16).ok()?;
    Some(egui::Color32::from_rgb(r, g, b))
}

fn build_template_card(template: TemplateDefinition) -> TemplateCard {
    let (default_top_color, default_bottom_color) = default_card_palette(template.preview_style);
    let top_color = template
        .preview_top_color
        .as_deref()
        .and_then(parse_color_hex)
        .unwrap_or(default_top_color);
    let bottom_color = template
        .preview_bottom_color
        .as_deref()
        .and_then(parse_color_hex)
        .unwrap_or(default_bottom_color);

    TemplateCard {
        template_id: template.id,
        title_zh: template.title_zh,
        title_en: template.title_en,
        subtitle_zh: template.subtitle_zh,
        subtitle_en: template.subtitle_en,
        top_color,
        bottom_color,
        preview_style: template.preview_style,
    }
}

fn ensure_project_templates(ui_state: &mut LauncherUiState) {
    if ui_state.templates_loaded {
        return;
    }

    let templates = list_templates().unwrap_or_default();
    ui_state.project_templates = templates
        .iter()
        .filter(|template| template.kind == TemplateKind::Project)
        .cloned()
        .map(build_template_card)
        .collect();
    ui_state.mod_templates = templates
        .into_iter()
        .filter(|template| template.kind == TemplateKind::Mod)
        .map(build_template_card)
        .collect();
    ui_state.templates_loaded = true;
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

    let mut fonts = FontDefinitions::default();
    if let Some(font_bytes) = load_cjk_font_bytes() {
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
    }

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

fn load_png_texture(ctx: &egui::Context, texture_name: &str, path: &Path) -> Option<TextureHandle> {
    let image = image::open(path).ok()?.into_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let color_image = ColorImage::from_rgba_unmultiplied(size, image.as_raw());
    Some(ctx.load_texture(texture_name, color_image, TextureOptions::LINEAR))
}

fn ensure_nav_textures(ctx: &egui::Context, ui_state: &mut LauncherUiState) {
    if ui_state.nav_create_texture.is_some() && ui_state.nav_projects_texture.is_some() {
        return;
    }

    let assets_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    if ui_state.nav_create_texture.is_none() {
        ui_state.nav_create_texture =
            load_png_texture(ctx, NAV_CREATE_TEXTURE_NAME, &assets_dir.join("plus.png"));
    }
    if ui_state.nav_projects_texture.is_none() {
        ui_state.nav_projects_texture = load_png_texture(
            ctx,
            NAV_PROJECTS_TEXTURE_NAME,
            &assets_dir.join("image-off.png"),
        );
    }
}

fn nav_button(
    ui: &mut egui::Ui,
    selected: bool,
    icon: Option<&TextureHandle>,
    label: &str,
) -> egui::Response {
    let desired_size = egui::vec2(ui.available_width(), 46.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);

    let hovered = response.hovered();
    let fill = if selected {
        SURFACE_SOFT
    } else if hovered {
        NAV_HOVER_FILL
    } else {
        egui::Color32::TRANSPARENT
    };
    let stroke = if selected || hovered {
        egui::Stroke::new(1.0, NAV_HOVER_STROKE)
    } else {
        egui::Stroke::NONE
    };
    let text_color = if selected {
        egui::Color32::WHITE
    } else if hovered {
        egui::Color32::from_rgb(236, 236, 236)
    } else {
        egui::Color32::from_rgb(212, 212, 212)
    };

    ui.painter()
        .rect(rect, 6.0, fill, stroke, egui::StrokeKind::Inside);

    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 24.0, rect.center().y),
        egui::vec2(18.0, 18.0),
    );
    let label_pos = egui::pos2(rect.left() + 44.0, rect.center().y);

    if let Some(icon) = icon {
        ui.painter().image(
            icon.id(),
            icon_rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            text_color,
        );
    }
    ui.painter().text(
        label_pos,
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(18.0),
        text_color,
    );

    response
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

fn is_valid_project_folder(path: &Path) -> bool {
    path.join("Cargo.toml").exists()
        && path.join("src").is_dir()
        && path.join("src").join("main.rs").exists()
}

fn import_project_folder(
    project_list: &mut ProjectInfoList,
    ui_state: &mut LauncherUiState,
    i18n: &Strings,
    project_path: PathBuf,
) {
    if !is_valid_project_folder(&project_path) {
        push_notification(
            ui_state,
            format!(
                "{}: {}",
                i18n.invalid_project_folder,
                project_path.display()
            ),
        );
        return;
    }

    if project_list
        .0
        .iter()
        .any(|project| project.path == project_path)
    {
        push_notification(
            ui_state,
            format!(
                "{}: {}",
                i18n.project_already_imported,
                project_path.display()
            ),
        );
        return;
    }

    let project_info = ProjectInfo {
        path: project_path,
        display_name: None,
        last_opened: SystemTime::now(),
    };
    let project_name = project_info.name().unwrap_or_else(|| "Unknown".to_string());
    project_list.0.push(project_info);
    set_project_list(project_list.0.clone());
    push_notification(
        ui_state,
        format!("{}: {project_name}", i18n.imported_project),
    );
}

fn project_modified_at(project: &ProjectInfo) -> String {
    let Some(timestamp) =
        std::fs::metadata(&project.path).ok().and_then(|metadata| metadata.modified().ok())
    else {
        return "-".to_string();
    };

    let modified_at: DateTime<Local> = timestamp.into();
    modified_at.format("%Y-%m-%d %H:%M").to_string()
}

fn project_stable_id(project: &ProjectInfo) -> u32 {
    let mut hasher = DefaultHasher::new();
    project.path.hash(&mut hasher);
    (hasher.finish() % 900_000 + 100_000) as u32
}

fn open_project_folder(path: &Path) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        return Command::new("explorer")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|error| std::io::Error::other(format!("Failed to open folder: {error}")));
    }

    #[cfg(target_os = "macos")]
    {
        return Command::new("open")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|error| std::io::Error::other(format!("Failed to open folder: {error}")));
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|error| std::io::Error::other(format!("Failed to open folder: {error}")));
    }
}

fn project_thumbnail(ui: &mut egui::Ui, texture: Option<&TextureHandle>) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(64, 64, 64))
        .stroke(egui::Stroke::NONE)
        .corner_radius(2)
        .inner_margin(egui::Margin::same(0))
        .show(ui, |ui| {
            ui.set_min_size(egui::vec2(72.0, 72.0));
            ui.centered_and_justified(|ui| {
                if let Some(texture) = texture {
                    ui.add(egui::Image::new(texture).fit_to_exact_size(egui::vec2(72.0, 72.0)));
                } else {
                    ui.label(egui::RichText::new("BS").size(18.0).strong());
                }
            });
        });
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

    match card.preview_style {
        TemplatePreviewStyle::Grid => {
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
        }
        TemplatePreviewStyle::Shooter => {
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
        TemplatePreviewStyle::Generic => {
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(rect.left() + 26.0, rect.top() + 30.0),
                    egui::vec2(128.0, 18.0),
                ),
                4.0,
                egui::Color32::from_white_alpha(44),
            );
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(rect.left() + 26.0, rect.top() + 60.0),
                    egui::vec2(84.0, 84.0),
                ),
                8.0,
                egui::Color32::from_white_alpha(30),
            );
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(rect.left() + 120.0, rect.top() + 78.0),
                    egui::vec2(34.0, 66.0),
                ),
                8.0,
                egui::Color32::from_white_alpha(22),
            );
        }
    }
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
                template_preview(ui, card);
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
        ui.label(
            egui::RichText::new(card.subtitle(locale))
                .size(14.0)
                .color(TEXT_MUTED),
        );
    });
}

fn render_create_page(ui: &mut egui::Ui, ui_state: &mut LauncherUiState, i18n: &Strings) {
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
                        "Add a template folder under templates/ to make it appear here."
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

fn render_create_project_dialog(
    ctx: &egui::Context,
    commands: &mut Commands,
    ui_state: &mut LauncherUiState,
    i18n: &Strings,
) {
    let Some(dialog) = ui_state.create_dialog.as_mut() else {
        return;
    };

    let mut close_dialog = ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
    let mut create_project = false;
    let locale = ui_state.locale;

    egui::Window::new(dialog.template.title(locale))
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
            template_preview(ui, &dialog.template);
            ui.add_space(12.0);
            ui.label(egui::RichText::new(dialog.template.title(locale)).size(22.0));
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(dialog.template.subtitle(locale))
                    .size(16.0)
                    .color(egui::Color32::WHITE),
            );

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

fn render_rename_project_dialog(
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

fn render_projects_page(
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

    egui::ScrollArea::vertical().show(ui, |ui| {
        for project in &project_list.0 {
            egui::Frame::new()
                .fill(SURFACE_CARD)
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 70)))
                .corner_radius(6)
                .inner_margin(egui::Margin::symmetric(14, 14))
                .show(ui, |ui| {
                    ui.set_min_height(72.0);
                    ui.horizontal_top(|ui| {
                        project_thumbnail(ui, ui_state.brand_texture.as_ref());
                        ui.add_space(14.0);

                        let info_width = (ui.available_width() - 34.0).max(160.0);
                        ui.allocate_ui_with_layout(
                            egui::vec2(info_width, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                let project_name =
                                    project.name().unwrap_or_else(|| "Unknown".to_string());
                                let project_id = project_stable_id(project);
                                ui.horizontal(|ui| {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(project_name).size(18.0).strong(),
                                        )
                                        .truncate(),
                                    );
                                    ui.add_space(4.0);
                                    ui.label(
                                        egui::RichText::new(format!("(ID: {project_id})"))
                                            .size(14.0)
                                            .color(TEXT_MUTED),
                                    );
                                });
                                ui.add_space(4.0);
                                ui.add_sized(
                                    [info_width, 18.0],
                                    egui::Label::new(
                                        egui::RichText::new(format!(
                                            "{}:  {}",
                                            i18n.modified_at,
                                            project_modified_at(project)
                                        ))
                                        .size(14.0)
                                        .color(TEXT_MUTED),
                                    )
                                    .truncate(),
                                );
                                ui.add_space(4.0);
                                ui.add_sized(
                                    [info_width, 18.0],
                                    egui::Label::new(
                                        egui::RichText::new(format!(
                                            "{}:  {}",
                                            i18n.path_prefix,
                                            project.path.display()
                                        ))
                                        .size(14.0)
                                        .color(TEXT_MUTED),
                                    )
                                    .truncate(),
                                );
                            },
                        );

                        ui.scope(|ui| {
                            ui.style_mut().spacing.button_padding = egui::vec2(6.0, 2.0);
                            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                                ui.menu_button(
                                    egui::RichText::new("⋮").size(18.0).color(TEXT_MUTED),
                                    |ui| {
                                        if ui.button(i18n.rename_project).clicked() {
                                            ui_state.rename_dialog = Some(RenameProjectDialogState {
                                                project_path: project.path.clone(),
                                                project_name: project
                                                    .name()
                                                    .unwrap_or_else(|| "Unknown".to_string()),
                                            });
                                            ui.close();
                                        }

                                        if ui.button(i18n.open_project_folder).clicked() {
                                            if let Err(error) = open_project_folder(&project.path) {
                                                push_notification(
                                                    ui_state,
                                                    format!("{}: {error}", i18n.open_folder_failed),
                                                );
                                            }
                                            ui.close();
                                        }

                                        if ui.button(i18n.remove_project).clicked() {
                                            remove_path = Some(project.path.clone());
                                            ui.close();
                                        }
                                    },
                                );
                            });
                        });
                    });

                    let card_response = ui
                        .interact(
                        ui.min_rect(),
                        ui.make_persistent_id(("project_card", &project.path)),
                        egui::Sense::click(),
                    )
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    if card_response.double_clicked() {
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
                });
            ui.add_space(8.0);
        }
    });

    if let Some(path) = remove_path {
        project_list.0.retain(|project| project.path != path);
        set_project_list(project_list.0.clone());
        push_notification(ui_state, i18n.project_removed);
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
    ensure_nav_textures(ctx, &mut ui_state);
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
                                egui::Image::new(texture).fit_to_exact_size(egui::vec2(
                                    BRAND_ICON_SIZE,
                                    BRAND_ICON_SIZE,
                                )),
                            );
                        }
                        ui.add_space(8.0);
                    });
                });

            ui.add_space(28.0);

            if nav_button(
                ui,
                ui_state.page == LauncherPage::Create,
                ui_state.nav_create_texture.as_ref(),
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
                ui_state.nav_projects_texture.as_ref(),
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
            LauncherPage::Create => render_create_page(ui, &mut ui_state, &i18n),
            LauncherPage::Projects => {
                render_projects_page(ui, &mut project_list, &mut ui_state, &mut exit, &i18n);
            }
        });

    render_create_project_dialog(ctx, &mut commands, &mut ui_state, &i18n);
    render_rename_project_dialog(ctx, &mut project_list, &mut ui_state, &i18n);

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
