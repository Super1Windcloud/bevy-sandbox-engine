use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{
        self, ColorImage, FontData, FontDefinitions, FontFamily, TextureHandle, TextureOptions,
    },
};
use bevy_sandbox_engine::project::{
    ProjectInfo, ProjectKind, detect_project_kind, get_local_projects, set_project_list,
    templates::{TemplateDefinition, TemplateKind, TemplatePreviewStyle, list_templates},
};
use sys_locale::get_locale;

use crate::{ProjectInfoList, RunningProjects, spawn_create_new_project_task};

mod projects;
mod templates;

#[derive(Resource)]
pub struct LauncherUiState {
    pub notifications: Vec<Notification>,
    pub page: LauncherPage,
    pub template_tab: TemplateTab,
    pub locale: LauncherLocale,
    pub brand_texture: Option<TextureHandle>,
    pub nav_create_texture: Option<TextureHandle>,
    pub nav_projects_texture: Option<TextureHandle>,
    pub template_preview_textures: HashMap<String, TextureHandle>,
    pub fonts_configured: bool,
    project_templates: Vec<TemplateCard>,
    mod_templates: Vec<TemplateCard>,
    templates_loaded: bool,
    create_dialog: Option<CreateProjectDialogState>,
    rename_dialog: Option<RenameProjectDialogState>,
    project_card_menu: Option<ProjectCardMenuState>,
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
            template_preview_textures: HashMap::new(),
            fonts_configured: false,
            project_templates: Vec::new(),
            mod_templates: Vec::new(),
            templates_loaded: false,
            create_dialog: None,
            rename_dialog: None,
            project_card_menu: None,
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

pub(super) const LAUNCHER_BG: Color = Color::srgb(0.153, 0.153, 0.153);
pub(super) const SIDEBAR_BG: egui::Color32 = egui::Color32::from_rgb(50, 50, 50);
pub(super) const SURFACE_BG: egui::Color32 = egui::Color32::from_rgb(42, 42, 42);
pub(super) const SURFACE_SOFT: egui::Color32 = egui::Color32::from_rgb(70, 70, 70);
pub(super) const SURFACE_CARD: egui::Color32 = egui::Color32::from_rgb(56, 56, 56);
pub(super) const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(165, 165, 165);
pub(super) const TAB_ACTIVE: egui::Color32 = egui::Color32::from_rgb(232, 232, 232);
pub(super) const TAB_INACTIVE: egui::Color32 = egui::Color32::from_rgb(130, 130, 130);
const BRAND_TEXTURE_NAME: &str = "launcher-brand-logo";
const CJK_FONT_NAME: &str = "launcher-cjk-font";
const ICON_FONT_NAME: &str = "launcher-icon-font";
pub(super) const BRAND_ICON_SIZE: f32 = 112.0;
const NAV_CREATE_TEXTURE_NAME: &str = "launcher-nav-create";
const NAV_PROJECTS_TEXTURE_NAME: &str = "launcher-nav-projects";
pub(super) const NAV_HOVER_FILL: egui::Color32 = egui::Color32::from_rgb(58, 58, 58);

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
    pub launch_project: &'static str,
    pub launching_project: &'static str,
    pub terminate_project: &'static str,
    pub project_already_running: &'static str,
    pub failed_to_terminate_project: &'static str,
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
            open_project_folder: "打开项目目录",
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
            launch_project: "启动",
            launching_project: "正在启动",
            terminate_project: "终止",
            project_already_running: "项目已在运行",
            failed_to_terminate_project: "终止项目失败",
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
            launch_project: "Launch",
            launching_project: "Launching",
            terminate_project: "Terminate",
            project_already_running: "Project is already running",
            failed_to_terminate_project: "Failed to terminate project",
        },
    }
}

#[derive(Clone)]
pub(super) struct TemplateCard {
    pub template_id: String,
    pub title_zh: String,
    pub title_en: String,
    pub subtitle_zh: String,
    pub subtitle_en: String,
    pub top_color: egui::Color32,
    pub bottom_color: egui::Color32,
    pub preview_image: Option<PathBuf>,
    pub preview_style: TemplatePreviewStyle,
}

pub(super) struct CreateProjectDialogState {
    pub template: TemplateCard,
    pub project_name: String,
    pub storage_location: String,
}

pub(super) struct RenameProjectDialogState {
    pub project_path: PathBuf,
    pub project_name: String,
}

pub(super) struct ProjectCardMenuState {
    pub project_path: PathBuf,
    pub anchor: egui::Pos2,
    pub just_opened: bool,
}

impl TemplateCard {
    fn title(&self, locale: LauncherLocale) -> &str {
        match locale {
            LauncherLocale::ZhCn => &self.title_zh,
            LauncherLocale::EnUs => &self.title_en,
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

pub(super) fn push_notification(ui_state: &mut LauncherUiState, message: impl Into<String>) {
    ui_state.notifications.push(Notification {
        text: message.into(),
        ttl: Timer::from_seconds(3.0, TimerMode::Once),
    });
}

pub(super) fn default_project_location() -> String {
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

pub(super) fn default_project_name() -> String {
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
        preview_image: template.preview_image,
        preview_style: template.preview_style,
    }
}

pub(super) fn ensure_project_templates(ui_state: &mut LauncherUiState) {
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
    visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
    visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
    visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
    visuals.widgets.open.bg_stroke = egui::Stroke::NONE;
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

fn load_icon_font_bytes() -> Option<Vec<u8>> {
    let icon_font = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("bevy_editor_styles")
        .join("src")
        .join("assets")
        .join("icons")
        .join("Lucide.ttf");

    std::fs::read(icon_font).ok()
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

    if let Some(icon_font_bytes) = load_icon_font_bytes() {
        fonts.font_data.insert(
            ICON_FONT_NAME.to_string(),
            FontData::from_owned(icon_font_bytes).into(),
        );
        let mut icon_family = vec![ICON_FONT_NAME.to_string()];
        if let Some(proportional_fonts) = fonts.families.get(&FontFamily::Proportional) {
            icon_family.extend(proportional_fonts.iter().cloned());
        }
        fonts
            .families
            .insert(FontFamily::Name(ICON_FONT_NAME.into()), icon_family);
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

pub(super) fn load_png_texture(
    ctx: &egui::Context,
    texture_name: &str,
    path: &Path,
) -> Option<TextureHandle> {
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
    let desired_size = egui::vec2(ui.available_width(), 44.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let hovered = response.hovered();
    let fill = if selected || hovered {
        NAV_HOVER_FILL
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter()
        .rect(rect, 8.0, fill, egui::Stroke::NONE, egui::StrokeKind::Inside);

    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 24.0, rect.center().y),
        egui::vec2(16.0, 16.0),
    );
    let text_color = if selected {
        egui::Color32::from_rgb(242, 242, 242)
    } else if hovered {
        egui::Color32::from_rgb(224, 224, 224)
    } else {
        TEXT_MUTED
    };
    let label_pos = egui::pos2(rect.left() + 42.0, rect.center().y);

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

pub(super) fn is_valid_project_folder(path: &Path) -> bool {
    detect_project_kind(path).is_some()
}

pub(super) fn import_project_folder(
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

    let kind = detect_project_kind(&project_path).unwrap_or(ProjectKind::Rust);
    let project_info = ProjectInfo {
        path: project_path,
        kind,
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

pub(super) fn project_modified_at(project: &ProjectInfo) -> String {
    let Some(timestamp) = std::fs::metadata(&project.path)
        .ok()
        .and_then(|metadata| metadata.modified().ok())
    else {
        return "-".to_string();
    };

    let modified_at: chrono::DateTime<chrono::Local> = timestamp.into();
    modified_at.format("%Y-%m-%d %H:%M").to_string()
}

pub(super) fn project_stable_id(project: &ProjectInfo) -> u32 {
    let mut hasher = DefaultHasher::new();
    project.path.hash(&mut hasher);
    (hasher.finish() % 900_000 + 100_000) as u32
}

pub(super) fn open_project_folder(path: &Path) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        return std::process::Command::new("explorer")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|error| std::io::Error::other(format!("Failed to open folder: {error}")));
    }

    #[cfg(target_os = "macos")]
    {
        return std::process::Command::new("open")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|error| std::io::Error::other(format!("Failed to open folder: {error}")));
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|error| std::io::Error::other(format!("Failed to open folder: {error}")));
    }
}

pub fn render_launcher_ui(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut project_list: ResMut<ProjectInfoList>,
    mut running_projects: ResMut<RunningProjects>,
    mut ui_state: ResMut<LauncherUiState>,
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
            LauncherPage::Create => templates::render_create_page(ui, &mut ui_state, &i18n),
            LauncherPage::Projects => {
                projects::render_projects_page(
                    ui,
                    &mut project_list,
                    &mut running_projects,
                    &mut ui_state,
                    &i18n,
                );
            }
        });

    templates::render_create_project_dialog(ctx, &mut commands, &mut ui_state, &i18n);
    projects::render_rename_project_dialog(ctx, &mut project_list, &mut ui_state, &i18n);

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
