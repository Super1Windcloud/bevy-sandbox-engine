use bevy::prelude::*;
use bevy_editor_styles::{Theme, colors::EditorColors};
use bevy_egui::EguiGlobalSettings;
use bevy_pane_layout::{PaneLayoutPlugin, PaneLayoutSet, RootPaneLayoutNode, prelude::*};
use bevy_properties_pane::PropertiesPanePlugin;
use bevy_scene_tree::SceneTreePlugin;
use bevy_toolbar::{ActiveTool, EditorTool};
use bevy_transform_gizmos::{GizmoMode, TransformGizmoSettings};
use sys_locale::get_locale;

pub struct EditorUIPlugin;

impl Plugin for EditorUIPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, UISet.before(PaneLayoutSet))
            .insert_resource(EguiGlobalSettings {
                auto_create_primary_context: false,
                ..default()
            })
            .add_systems(Startup, ui_setup.in_set(UISet))
            .add_systems(
                Update,
                (sync_system_locale, handle_shell_buttons, sync_shell_labels),
            )
            .add_plugins((PaneLayoutPlugin, SceneTreePlugin, PropertiesPanePlugin))
            .register_pane("Console", setup_console_pane)
            .register_pane("Asset Store", setup_asset_store_pane)
            .init_resource::<EditorShellState>()
            .init_resource::<EditorUiState>();
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UISet;

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
}

impl Default for EditorUiState {
    fn default() -> Self {
        Self {
            locale: EditorLocale::detect(),
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
        if locale.to_ascii_lowercase().starts_with("zh") {
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
    status_step: &'static str,
    status_pivot_center: &'static str,
    status_local_global: &'static str,
    tool_prefix: &'static str,
    state_editing: &'static str,
    state_playing: &'static str,
    state_paused: &'static str,
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
            status_step: "单步执行",
            status_pivot_center: "切换枢轴模式",
            status_local_global: "切换本地/世界坐标",
            tool_prefix: "工具",
            state_editing: "状态: 编辑",
            state_playing: "状态: 运行",
            state_paused: "状态: 暂停",
            category_title: "分类",
            categories: ["推荐", "几何体", "生物", "自然", "人造物"],
            console_lines: [
                "[信息] 兼容项目已加载",
                "[信息] DefaultScene 已迁移为占位实体",
                "[警告] 脚本桥接当前仍使用替身引擎 API",
                "[信息] 点击播放后继续逼近运行态一致性",
            ],
            coordinate_text: "坐标 5.00   缩放 1.0   旋转 0.01",
        },
        EditorLocale::EnUs => EditorStrings {
            engine_name: "Bevy Sandbox Engine",
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
            status_step: "Step",
            status_pivot_center: "Toggle pivot mode",
            status_local_global: "Toggle local/world space",
            tool_prefix: "Tool",
            state_editing: "State: Editing",
            state_playing: "State: Playing",
            state_paused: "State: Paused",
            category_title: "Categories",
            categories: ["Recommended", "Geometry", "Creatures", "Nature", "Props"],
            console_lines: [
                "[Info] Compatibility project loaded",
                "[Info] DefaultScene migrated into placeholder entities",
                "[Warn] Script bridge is still running with stub engine APIs",
                "[Info] Press Play to keep closing the runtime parity gap",
            ],
            coordinate_text: "Pivot 5.00   Scale 1.0   Rotate 0.01",
        },
    }
}

#[derive(Component)]
struct StatusText;

#[derive(Component)]
struct ToolText;

#[derive(Component)]
struct PlayText;

#[derive(Component)]
struct SnapButtonText;

#[derive(Component)]
struct ShellButton(ShellAction);

#[derive(Clone, Copy, PartialEq, Eq)]
enum ShellAction {
    File,
    Edit,
    Window,
    GameObject,
    Component,
    Help,
    Play,
    Pause,
    Stop,
    Step,
    Select,
    Move,
    Rotate,
    Scale,
    Pivot,
    LocalGlobal,
    NewEntity,
    ToggleSnap,
}

fn ui_setup(
    mut commands: Commands,
    theme: Res<Theme>,
    active_tool: Res<ActiveTool>,
    shell_state: Res<EditorShellState>,
    ui_state: Res<EditorUiState>,
) {
    let i18n = strings(ui_state.locale);

    let ui_camera = commands
        .spawn((
            Camera2d,
            Camera {
                order: 10,
                ..default()
            },
            IsDefaultUiCamera,
        ))
        .id();

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            UiTargetCamera(ui_camera),
            BackgroundColor(EditorColors::BACKGROUND),
            RootUINode,
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    height: Val::Px(28.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(8.0)),
                    column_gap: Val::Px(12.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ))
            .with_children(|menu| {
                menu.spawn((
                    Text::new(i18n.engine_name),
                    TextFont {
                        font: theme.text.font.clone().into(),
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(theme.text.text_color),
                ));

                for (label, action) in [
                    (i18n.menu_file, ShellAction::File),
                    (i18n.menu_edit, ShellAction::Edit),
                    (i18n.menu_window, ShellAction::Window),
                    (i18n.menu_game_object, ShellAction::GameObject),
                    (i18n.menu_component, ShellAction::Component),
                    (i18n.menu_help, ShellAction::Help),
                ] {
                    spawn_shell_button(menu, &theme, label, action, false, false, false, false);
                }
            });

            root.spawn((
                Node {
                    height: Val::Px(56.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::horizontal(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
            ))
            .with_children(|toolbar| {
                toolbar.spawn(Node {
                    width: Val::Px(180.0),
                    ..default()
                });

                toolbar
                    .spawn((
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(6.0),
                            padding: UiRect::all(Val::Px(8.0)),
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.24, 0.24, 0.24)),
                    ))
                    .with_children(|group| {
                        spawn_shell_button(
                            group,
                            &theme,
                            "▶",
                            ShellAction::Play,
                            true,
                            false,
                            shell_state.play_state == PlayState::Playing,
                            false,
                        );
                        spawn_shell_button(
                            group,
                            &theme,
                            "⏸",
                            ShellAction::Pause,
                            true,
                            false,
                            shell_state.play_state == PlayState::Paused,
                            false,
                        );
                        spawn_shell_button(
                            group,
                            &theme,
                            "⏹",
                            ShellAction::Stop,
                            true,
                            false,
                            shell_state.play_state == PlayState::Editing,
                            false,
                        );
                        spawn_separator(group);
                        spawn_shell_button(
                            group,
                            &theme,
                            i18n.tool_select,
                            ShellAction::Select,
                            false,
                            true,
                            active_tool.0 == EditorTool::Select,
                            false,
                        );
                        spawn_shell_button(
                            group,
                            &theme,
                            i18n.tool_move,
                            ShellAction::Move,
                            false,
                            true,
                            active_tool.0 == EditorTool::Move,
                            false,
                        );
                        spawn_shell_button(
                            group,
                            &theme,
                            i18n.tool_rotate,
                            ShellAction::Rotate,
                            false,
                            true,
                            active_tool.0 == EditorTool::Rotate,
                            false,
                        );
                        spawn_shell_button(
                            group,
                            &theme,
                            i18n.tool_scale,
                            ShellAction::Scale,
                            false,
                            true,
                            active_tool.0 == EditorTool::Scale,
                            false,
                        );
                        spawn_separator(group);
                        spawn_shell_button(
                            group,
                            &theme,
                            "◎",
                            ShellAction::Pivot,
                            true,
                            false,
                            false,
                            false,
                        );
                        spawn_shell_button(
                            group,
                            &theme,
                            "L",
                            ShellAction::LocalGlobal,
                            true,
                            false,
                            false,
                            false,
                        );
                        spawn_shell_button(
                            group,
                            &theme,
                            ">",
                            ShellAction::Step,
                            true,
                            false,
                            false,
                            false,
                        );
                    });

                toolbar
                    .spawn((Node {
                        width: Val::Px(360.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexEnd,
                        column_gap: Val::Px(8.0),
                        ..default()
                    },))
                    .with_children(|right| {
                        right.spawn((
                            Text::new(i18n.coordinate_text),
                            TextFont {
                                font: theme.text.font.clone().into(),
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(theme.text.low_priority),
                        ));
                        spawn_shell_button(
                            right,
                            &theme,
                            i18n.action_new_entity,
                            ShellAction::NewEntity,
                            false,
                            false,
                            false,
                            false,
                        );
                        spawn_shell_button(
                            right,
                            &theme,
                            i18n.action_snap_off,
                            ShellAction::ToggleSnap,
                            false,
                            false,
                            false,
                            true,
                        );
                    });
            });

            root.spawn((Node {
                flex_grow: 1.0,
                min_height: Val::Px(0.0),
                ..default()
            },))
                .with_children(|workspace| {
                    workspace.spawn(RootPaneLayoutNode);
                });

            root.spawn((
                Node {
                    height: Val::Px(24.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(10.0)),
                    column_gap: Val::Px(12.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.06, 0.07, 0.08)),
            ))
            .with_children(|status_bar| {
                status_bar.spawn((
                    Text::new(shell_state.status.clone()),
                    TextFont {
                        font: theme.text.font.clone().into(),
                        font_size: FontSize::Px(11.0),
                        ..default()
                    },
                    TextColor(theme.text.text_color),
                    StatusText,
                ));
                status_bar.spawn((
                    Text::new(format!(
                        "{}: {}",
                        i18n.tool_prefix,
                        tool_name(active_tool.0, ui_state.locale)
                    )),
                    TextFont {
                        font: theme.text.font.clone().into(),
                        font_size: FontSize::Px(11.0),
                        ..default()
                    },
                    TextColor(theme.text.low_priority),
                    ToolText,
                ));
                status_bar.spawn((
                    Text::new(play_state_name(shell_state.play_state, &i18n)),
                    TextFont {
                        font: theme.text.font.clone().into(),
                        font_size: FontSize::Px(11.0),
                        ..default()
                    },
                    TextColor(theme.text.low_priority),
                    PlayText,
                ));
            });
        });
}

fn spawn_shell_button(
    parent: &mut ChildSpawnerCommands,
    theme: &Theme,
    label: &str,
    action: ShellAction,
    compact: bool,
    ghost: bool,
    selected: bool,
    mark_snap_text: bool,
) {
    let width = if compact { 26.0 } else { 66.0 };
    let mut entity = parent.spawn((
        Button,
        ShellButton(action),
        Node {
            min_width: Val::Px(width),
            height: Val::Px(24.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::horizontal(Val::Px(8.0)),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(button_color(selected, ghost)),
    ));

    if mark_snap_text {
        entity.insert(SnapButtonText);
    }

    entity.with_children(|button| {
        button.spawn((
            Text::new(label),
            TextFont {
                font: theme.text.font.clone().into(),
                font_size: FontSize::Px(if compact { 12.0 } else { 11.0 }),
                ..default()
            },
            TextColor(if selected {
                Color::WHITE
            } else {
                theme.text.text_color
            }),
        ));
    });
}

fn spawn_separator(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Px(1.0),
            height: Val::Px(18.0),
            margin: UiRect::horizontal(Val::Px(3.0)),
            ..default()
        },
        BackgroundColor(EditorColors::BORDER),
    ));
}

fn button_color(selected: bool, ghost: bool) -> Color {
    if selected {
        Color::srgb(0.36, 0.48, 0.23)
    } else if ghost {
        Color::srgb(0.21, 0.22, 0.23)
    } else {
        EditorColors::BUTTON_DEFAULT
    }
}

fn sync_system_locale(mut ui_state: ResMut<EditorUiState>) {
    ui_state.locale = EditorLocale::detect();
}

fn handle_shell_buttons(
    mut interaction_query: Query<
        (&Interaction, &ShellButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut shell_state: ResMut<EditorShellState>,
    ui_state: Res<EditorUiState>,
    mut active_tool: ResMut<ActiveTool>,
    mut gizmo_settings: ResMut<TransformGizmoSettings>,
    mut commands: Commands,
) {
    let i18n = strings(ui_state.locale);

    for (interaction, button, mut background) in &mut interaction_query {
        let selected = is_selected(
            button.0,
            active_tool.0,
            shell_state.play_state,
            gizmo_settings.snap_enabled,
        );
        match *interaction {
            Interaction::Pressed => {
                match button.0 {
                    ShellAction::File => shell_state.status = i18n.menu_file_message.to_string(),
                    ShellAction::Edit => shell_state.status = i18n.menu_edit_message.to_string(),
                    ShellAction::Window => {
                        shell_state.status = i18n.menu_window_message.to_string()
                    }
                    ShellAction::GameObject => {
                        shell_state.status = i18n.menu_game_object_message.to_string()
                    }
                    ShellAction::Component => {
                        shell_state.status = i18n.menu_component_message.to_string()
                    }
                    ShellAction::Help => shell_state.status = i18n.menu_help_message.to_string(),
                    ShellAction::Play => {
                        shell_state.play_state = PlayState::Playing;
                        shell_state.status = i18n.status_playing.to_string();
                    }
                    ShellAction::Pause => {
                        shell_state.play_state = PlayState::Paused;
                        shell_state.status = i18n.status_paused.to_string();
                    }
                    ShellAction::Stop => {
                        shell_state.play_state = PlayState::Editing;
                        shell_state.status = i18n.status_stopped.to_string();
                    }
                    ShellAction::Step => shell_state.status = i18n.status_step.to_string(),
                    ShellAction::Select => active_tool.0 = EditorTool::Select,
                    ShellAction::Move => {
                        active_tool.0 = EditorTool::Move;
                        gizmo_settings.mode = GizmoMode::Translate;
                    }
                    ShellAction::Rotate => {
                        active_tool.0 = EditorTool::Rotate;
                        gizmo_settings.mode = GizmoMode::Rotate;
                    }
                    ShellAction::Scale => {
                        active_tool.0 = EditorTool::Scale;
                        gizmo_settings.mode = GizmoMode::Scale;
                    }
                    ShellAction::Pivot => shell_state.status = i18n.status_pivot_center.to_string(),
                    ShellAction::LocalGlobal => {
                        shell_state.status = i18n.status_local_global.to_string()
                    }
                    ShellAction::NewEntity => {
                        spawn_new_entity(&mut commands);
                        shell_state.status = i18n.status_new_entity.to_string();
                    }
                    ShellAction::ToggleSnap => {
                        gizmo_settings.snap_enabled = !gizmo_settings.snap_enabled;
                        shell_state.status = if gizmo_settings.snap_enabled {
                            i18n.status_snap_on.to_string()
                        } else {
                            i18n.status_snap_off.to_string()
                        };
                    }
                }

                *background = BackgroundColor(button_color(
                    is_selected(
                        button.0,
                        active_tool.0,
                        shell_state.play_state,
                        gizmo_settings.snap_enabled,
                    ),
                    is_ghost(button.0),
                ));
            }
            Interaction::Hovered => {
                *background = BackgroundColor(EditorColors::BUTTON_HOVER);
            }
            Interaction::None => {
                *background = BackgroundColor(button_color(selected, is_ghost(button.0)));
            }
        }
    }
}

fn sync_shell_labels(
    shell_state: Res<EditorShellState>,
    ui_state: Res<EditorUiState>,
    active_tool: Res<ActiveTool>,
    gizmo_settings: Res<TransformGizmoSettings>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<StatusText>>,
        Query<&mut Text, With<ToolText>>,
        Query<&mut Text, With<PlayText>>,
        Query<&mut Text, With<SnapButtonText>>,
    )>,
    mut button_query: Query<(&ShellButton, &mut BackgroundColor), With<Button>>,
) {
    let i18n = strings(ui_state.locale);

    for mut text in &mut text_queries.p0() {
        text.0 = shell_state.status.clone();
    }
    for mut text in &mut text_queries.p1() {
        text.0 = format!(
            "{}: {}",
            i18n.tool_prefix,
            tool_name(active_tool.0, ui_state.locale)
        );
    }
    for mut text in &mut text_queries.p2() {
        text.0 = play_state_name(shell_state.play_state, &i18n).to_string();
    }
    for mut text in &mut text_queries.p3() {
        text.0 = if gizmo_settings.snap_enabled {
            i18n.action_snap_on.to_string()
        } else {
            i18n.action_snap_off.to_string()
        };
    }

    if active_tool.is_changed() || shell_state.is_changed() || gizmo_settings.is_changed() {
        for (button, mut background) in &mut button_query {
            *background = BackgroundColor(button_color(
                is_selected(
                    button.0,
                    active_tool.0,
                    shell_state.play_state,
                    gizmo_settings.snap_enabled,
                ),
                is_ghost(button.0),
            ));
        }
    }
}

fn is_ghost(action: ShellAction) -> bool {
    matches!(
        action,
        ShellAction::Play
            | ShellAction::Pause
            | ShellAction::Stop
            | ShellAction::Step
            | ShellAction::Pivot
            | ShellAction::LocalGlobal
    )
}

fn is_selected(
    action: ShellAction,
    active_tool: EditorTool,
    play_state: PlayState,
    snap_enabled: bool,
) -> bool {
    match action {
        ShellAction::Play => play_state == PlayState::Playing,
        ShellAction::Pause => play_state == PlayState::Paused,
        ShellAction::Stop => play_state == PlayState::Editing,
        ShellAction::Select => active_tool == EditorTool::Select,
        ShellAction::Move => active_tool == EditorTool::Move,
        ShellAction::Rotate => active_tool == EditorTool::Rotate,
        ShellAction::Scale => active_tool == EditorTool::Scale,
        ShellAction::ToggleSnap => snap_enabled,
        _ => false,
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
    theme: Res<Theme>,
    ui_state: Res<EditorUiState>,
) {
    let i18n = strings(ui_state.locale);
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
                TextFont {
                    font: theme.text.font.clone().into(),
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(Color::srgb(0.76, 0.77, 0.79)),
            ));
        }
    });
}

fn setup_asset_store_pane(
    pane: In<PaneStructure>,
    mut commands: Commands,
    theme: Res<Theme>,
    ui_state: Res<EditorUiState>,
) {
    let i18n = strings(ui_state.locale);
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
            TextFont {
                font: theme.text.font.clone().into(),
                font_size: FontSize::Px(12.0),
                ..default()
            },
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
                        TextFont {
                            font: theme.text.font.clone().into(),
                            font_size: FontSize::Px(11.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.74, 0.75, 0.77)),
                    ));
                });
        }
    });
}
