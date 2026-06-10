use std::path::Path;

use bevy::prelude::*;
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self, FontData, FontDefinitions, FontFamily},
};
use bevy_pane_layout::{PaneLayoutPlugin, RootPaneLayoutNode, prelude::*};
use bevy_properties_pane::PropertiesPanePlugin;
use bevy_scene_tree::SceneTreePlugin;
use bevy_toolbar::{ActiveTool, EditorTool};
use bevy_transform_gizmos::{GizmoMode, TransformGizmoSettings};
use sys_locale::get_locale;

/// The Bevy Editor UI Plugin.
pub struct EditorUIPlugin;

impl Plugin for EditorUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_setup.in_set(UISet))
            .add_systems(Update, sync_system_locale)
            .add_systems(EguiPrimaryContextPass, render_editor_shell)
            .add_plugins((PaneLayoutPlugin, SceneTreePlugin, PropertiesPanePlugin))
            .register_pane("Console", setup_console_pane)
            .register_pane("Asset Store", setup_asset_store_pane)
            .init_resource::<EditorShellState>()
            .init_resource::<EditorUiState>();
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UISet;

/// The root node for the UI.
#[derive(Component)]
pub struct RootUINode;

#[derive(Resource, Default)]
struct EditorShellState {
    status: String,
    play_state: PlayState,
}

#[derive(Resource)]
struct EditorUiState {
    locale: EditorLocale,
    fonts_configured: bool,
}

impl Default for EditorUiState {
    fn default() -> Self {
        Self {
            locale: EditorLocale::detect(),
            fonts_configured: false,
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum PlayState {
    #[default]
    Editing,
    Playing,
    Paused,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EditorLocale {
    ZhCn,
    EnUs,
}

impl EditorLocale {
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

struct EditorStrings {
    engine_name: &'static str,
    menu_file: &'static str,
    menu_edit: &'static str,
    menu_window: &'static str,
    menu_game_object: &'static str,
    menu_component: &'static str,
    menu_help: &'static str,
    menu_file_message: &'static str,
    menu_edit_message: &'static str,
    menu_window_message: &'static str,
    menu_game_object_message: &'static str,
    menu_component_message: &'static str,
    menu_help_message: &'static str,
    tool_select: &'static str,
    tool_move: &'static str,
    tool_rotate: &'static str,
    tool_scale: &'static str,
    action_new_entity: &'static str,
    action_snap_on: &'static str,
    action_snap_off: &'static str,
    status_new_entity: &'static str,
    status_snap_on: &'static str,
    status_snap_off: &'static str,
    status_playing: &'static str,
    status_paused: &'static str,
    status_stopped: &'static str,
    tool_prefix: &'static str,
    state_editing: &'static str,
    state_playing: &'static str,
    state_paused: &'static str,
    console_title: &'static str,
    asset_store_title: &'static str,
    category_title: &'static str,
    categories: [&'static str; 5],
    console_lines: [&'static str; 4],
    coordinate_text: &'static str,
}

fn strings(locale: EditorLocale) -> EditorStrings {
    match locale {
        EditorLocale::ZhCn => EditorStrings {
            engine_name: "Sandmod Engine",
            menu_file: "文件",
            menu_edit: "编辑",
            menu_window: "窗口",
            menu_game_object: "游戏对象",
            menu_component: "组件",
            menu_help: "帮助",
            menu_file_message: "文件菜单暂未接线",
            menu_edit_message: "编辑菜单暂未接线",
            menu_window_message: "窗口菜单暂未接线",
            menu_game_object_message: "对象菜单暂未接线",
            menu_component_message: "组件菜单暂未接线",
            menu_help_message: "帮助菜单暂未接线",
            tool_select: "选择",
            tool_move: "移动",
            tool_rotate: "旋转",
            tool_scale: "缩放",
            action_new_entity: "新建对象",
            action_snap_on: "捕捉: 开",
            action_snap_off: "捕捉: 关",
            status_new_entity: "已创建新对象",
            status_snap_on: "已启用变换捕捉",
            status_snap_off: "已关闭变换捕捉",
            status_playing: "运行中",
            status_paused: "已暂停",
            status_stopped: "已停止运行",
            tool_prefix: "工具",
            state_editing: "状态: 编辑",
            state_playing: "状态: 运行",
            state_paused: "状态: 暂停",
            console_title: "控制台",
            asset_store_title: "资源商店",
            category_title: "分类",
            categories: ["推荐", "几何体", "生物", "自然", "人造物"],
            console_lines: [
                "[信息] 兼容项目已加载",
                "[信息] DefaultScene 已迁移为占位实体",
                "[警告] 脚本桥接当前仍使用替身引擎 API",
                "[信息] 点击播放后继续逼近运行态一致性",
            ],
            coordinate_text: "坐标 5.00    缩放 1.0    旋转 0.01",
        },
        EditorLocale::EnUs => EditorStrings {
            engine_name: "Sandmod Engine",
            menu_file: "File",
            menu_edit: "Edit",
            menu_window: "Window",
            menu_game_object: "GameObject",
            menu_component: "Component",
            menu_help: "Help",
            menu_file_message: "File menu is not wired yet",
            menu_edit_message: "Edit menu is not wired yet",
            menu_window_message: "Window menu is not wired yet",
            menu_game_object_message: "Game object menu is not wired yet",
            menu_component_message: "Component menu is not wired yet",
            menu_help_message: "Help menu is not wired yet",
            tool_select: "Select",
            tool_move: "Move",
            tool_rotate: "Rotate",
            tool_scale: "Scale",
            action_new_entity: "New Entity",
            action_snap_on: "Snap: On",
            action_snap_off: "Snap: Off",
            status_new_entity: "Spawned a new entity",
            status_snap_on: "Transform snapping enabled",
            status_snap_off: "Transform snapping disabled",
            status_playing: "Playing",
            status_paused: "Paused",
            status_stopped: "Stopped",
            tool_prefix: "Tool",
            state_editing: "State: Editing",
            state_playing: "State: Playing",
            state_paused: "State: Paused",
            console_title: "Console",
            asset_store_title: "Asset Store",
            category_title: "Categories",
            categories: ["Recommended", "Geometry", "Creatures", "Nature", "Props"],
            console_lines: [
                "[Info] Compatibility project loaded",
                "[Info] DefaultScene migrated into placeholder entities",
                "[Warn] Script bridge is still running with stub engine APIs",
                "[Info] Press Play to keep closing the runtime parity gap",
            ],
            coordinate_text: "Pivot 5.00    Scale 1.0    Rotate 0.01",
        },
    }
}

fn ui_setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 10,
            ..default()
        },
    ));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(56.0),
                bottom: Val::Px(26.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            RootUINode,
        ))
        .with_children(|parent| {
            parent.spawn(RootPaneLayoutNode);
        });
}

fn sync_system_locale(mut ui_state: ResMut<EditorUiState>) {
    ui_state.locale = EditorLocale::detect();
}

fn render_editor_shell(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut active_tool: ResMut<ActiveTool>,
    mut gizmo_settings: ResMut<TransformGizmoSettings>,
    mut shell_state: ResMut<EditorShellState>,
    mut ui_state: ResMut<EditorUiState>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    ensure_fonts(ctx, &mut ui_state);
    let i18n = strings(ui_state.locale);

    egui::TopBottomPanel::top("editor_top_bar")
        .exact_height(74.0)
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(43, 43, 43))
                .inner_margin(egui::Margin::symmetric(10, 6)),
        )
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(8.0, 6.0);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 14.0;
                    ui.label(
                        egui::RichText::new(i18n.engine_name)
                            .strong()
                            .color(egui::Color32::from_rgb(238, 238, 239)),
                    );
                    menu_button(
                        ui,
                        i18n.menu_file,
                        &mut shell_state.status,
                        i18n.menu_file_message,
                    );
                    menu_button(
                        ui,
                        i18n.menu_edit,
                        &mut shell_state.status,
                        i18n.menu_edit_message,
                    );
                    menu_button(
                        ui,
                        i18n.menu_window,
                        &mut shell_state.status,
                        i18n.menu_window_message,
                    );
                    menu_button(
                        ui,
                        i18n.menu_game_object,
                        &mut shell_state.status,
                        i18n.menu_game_object_message,
                    );
                    menu_button(
                        ui,
                        i18n.menu_component,
                        &mut shell_state.status,
                        i18n.menu_component_message,
                    );
                    menu_button(
                        ui,
                        i18n.menu_help,
                        &mut shell_state.status,
                        i18n.menu_help_message,
                    );
                });

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;

                    tool_button(
                        ui,
                        &mut active_tool,
                        &mut gizmo_settings,
                        EditorTool::Select,
                        i18n.tool_select,
                    );
                    tool_button(
                        ui,
                        &mut active_tool,
                        &mut gizmo_settings,
                        EditorTool::Move,
                        i18n.tool_move,
                    );
                    tool_button(
                        ui,
                        &mut active_tool,
                        &mut gizmo_settings,
                        EditorTool::Rotate,
                        i18n.tool_rotate,
                    );
                    tool_button(
                        ui,
                        &mut active_tool,
                        &mut gizmo_settings,
                        EditorTool::Scale,
                        i18n.tool_scale,
                    );

                    ui.separator();

                    play_button(
                        ui,
                        &mut shell_state,
                        PlayState::Playing,
                        "▶",
                        i18n.status_playing,
                    );
                    play_button(
                        ui,
                        &mut shell_state,
                        PlayState::Paused,
                        "⏸",
                        i18n.status_paused,
                    );
                    if ui.button("⏹").clicked() {
                        shell_state.play_state = PlayState::Editing;
                        shell_state.status = i18n.status_stopped.to_string();
                    }

                    ui.separator();

                    if ui.button(i18n.action_new_entity).clicked() {
                        spawn_new_entity(&mut commands);
                        shell_state.status = i18n.status_new_entity.to_string();
                    }

                    let snap_label = if gizmo_settings.snap_enabled {
                        i18n.action_snap_on
                    } else {
                        i18n.action_snap_off
                    };
                    if ui.button(snap_label).clicked() {
                        gizmo_settings.snap_enabled = !gizmo_settings.snap_enabled;
                        shell_state.status = if gizmo_settings.snap_enabled {
                            i18n.status_snap_on.to_string()
                        } else {
                            i18n.status_snap_off.to_string()
                        };
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(i18n.coordinate_text)
                                .color(egui::Color32::from_rgb(170, 170, 173)),
                        );
                    });
                });
            });
        });

    egui::TopBottomPanel::bottom("editor_status_bar")
        .exact_height(26.0)
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(16, 18, 20))
                .inner_margin(egui::Margin::symmetric(12, 6)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(&shell_state.status)
                        .color(egui::Color32::from_rgb(188, 192, 198)),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(format!(
                        "{}: {}",
                        i18n.tool_prefix,
                        tool_name(active_tool.0, ui_state.locale)
                    ))
                    .color(egui::Color32::from_rgb(148, 152, 158)),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(play_state_name(shell_state.play_state, &i18n))
                        .color(egui::Color32::from_rgb(148, 152, 158)),
                );
            });
        });

    Ok(())
}

fn menu_button(ui: &mut egui::Ui, label: &str, status: &mut String, message: &str) {
    if ui.button(label).clicked() {
        *status = message.to_string();
    }
}

fn play_button(
    ui: &mut egui::Ui,
    shell_state: &mut EditorShellState,
    target: PlayState,
    label: &str,
    message: &str,
) {
    let selected = shell_state.play_state == target;
    let button = egui::Button::new(label).fill(if selected {
        egui::Color32::from_rgb(88, 132, 55)
    } else {
        egui::Color32::from_rgb(62, 62, 64)
    });

    if ui.add(button).clicked() {
        shell_state.play_state = target;
        shell_state.status = message.to_string();
    }
}

fn tool_button(
    ui: &mut egui::Ui,
    active_tool: &mut ActiveTool,
    gizmo_settings: &mut TransformGizmoSettings,
    tool: EditorTool,
    label: &str,
) {
    let selected = active_tool.0 == tool;
    if ui.selectable_label(selected, label).clicked() {
        active_tool.0 = tool;
        match tool {
            EditorTool::Move => gizmo_settings.mode = GizmoMode::Translate,
            EditorTool::Rotate => gizmo_settings.mode = GizmoMode::Rotate,
            EditorTool::Scale => gizmo_settings.mode = GizmoMode::Scale,
            _ => {}
        }
    }
}

fn tool_name(tool: EditorTool, locale: EditorLocale) -> &'static str {
    match tool {
        EditorTool::Select => match locale {
            EditorLocale::ZhCn => "选择",
            EditorLocale::EnUs => "Select",
        },
        EditorTool::Move => match locale {
            EditorLocale::ZhCn => "移动",
            EditorLocale::EnUs => "Move",
        },
        EditorTool::Rotate => match locale {
            EditorLocale::ZhCn => "旋转",
            EditorLocale::EnUs => "Rotate",
        },
        EditorTool::Scale => match locale {
            EditorLocale::ZhCn => "缩放",
            EditorLocale::EnUs => "Scale",
        },
        EditorTool::NewEntity => match locale {
            EditorLocale::ZhCn => "新建对象",
            EditorLocale::EnUs => "New Entity",
        },
        EditorTool::Save => "Save",
        EditorTool::Load => "Load",
        EditorTool::Undo => "Undo",
        EditorTool::Redo => "Redo",
        EditorTool::Play => "Play",
        EditorTool::Pause => "Pause",
        EditorTool::Stop => "Stop",
    }
}

fn play_state_name(state: PlayState, i18n: &EditorStrings) -> &'static str {
    match state {
        PlayState::Editing => i18n.state_editing,
        PlayState::Playing => i18n.state_playing,
        PlayState::Paused => i18n.state_paused,
    }
}

fn spawn_new_entity(commands: &mut Commands) {
    commands.spawn((
        Name::new("New Entity"),
        Transform::default(),
        Visibility::default(),
    ));
}

fn setup_console_pane(
    pane: In<PaneStructure>,
    mut commands: Commands,
    ui_state: Res<EditorUiState>,
) {
    let i18n = strings(ui_state.locale);
    commands.entity(pane.header).with_children(|parent| {
        parent.spawn((
            Text::new(i18n.console_title),
            TextFont::from_font_size(12.0),
        ));
    });

    commands.entity(pane.content).insert((
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(6.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.12, 0.12, 0.13)),
    ));

    commands.entity(pane.content).with_children(|parent| {
        for line in i18n.console_lines {
            parent.spawn((
                Text::new(line),
                TextFont::from_font_size(11.0),
                TextColor(Color::srgb(0.76, 0.77, 0.79)),
            ));
        }
    });
}

fn setup_asset_store_pane(
    pane: In<PaneStructure>,
    mut commands: Commands,
    ui_state: Res<EditorUiState>,
) {
    let i18n = strings(ui_state.locale);
    commands.entity(pane.header).with_children(|parent| {
        parent.spawn((
            Text::new(i18n.asset_store_title),
            TextFont::from_font_size(12.0),
        ));
    });

    commands.entity(pane.content).insert((
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(8.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.16)),
    ));

    commands.entity(pane.content).with_children(|parent| {
        parent.spawn((
            Text::new(i18n.category_title),
            TextFont::from_font_size(12.0),
            TextColor(Color::srgb(0.86, 0.86, 0.88)),
        ));

        for category in i18n.categories {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.19, 0.19, 0.20)),
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new(category),
                        TextFont::from_font_size(11.0),
                        TextColor(Color::srgb(0.74, 0.75, 0.77)),
                    ));
                });
        }
    });
}

const CJK_FONT_NAME: &str = "editor-cjk-font";

fn ensure_fonts(ctx: &egui::Context, ui_state: &mut EditorUiState) {
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
