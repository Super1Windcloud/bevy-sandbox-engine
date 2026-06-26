use bevy::prelude::*;
use bevy_editor_styles::{Theme, colors::EditorColors, icons};
use bevy_egui::EguiGlobalSettings;
use bevy_pane_layout::{PaneLayoutPlugin, PaneLayoutSet, RootPaneLayoutNode, prelude::*};
use bevy_properties_pane::PropertiesPanePlugin;
use bevy_scene_tree::SceneTreePlugin;

use crate::locale_env::SupportedLocale;

mod menus;

use menus::{EditorMenuState, render_menu_overlays};

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
                (
                    sync_system_locale,
                    handle_shell_buttons,
                    sync_shell_labels,
                    render_menu_overlays,
                ),
            )
            .add_plugins((PaneLayoutPlugin, SceneTreePlugin, PropertiesPanePlugin))
            .register_pane("Console", setup_console_pane)
            .register_pane("Asset Store", setup_asset_store_pane)
            .init_resource::<EditorShellState>()
            .init_resource::<EditorMenuState>()
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
pub(super) struct EditorUiState {
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
pub(super) enum EditorLocale {
    ZhCn,
    EnUs,
}

impl EditorLocale {
    fn detect() -> Self {
        match SupportedLocale::detect() {
            SupportedLocale::ZhCn => Self::ZhCn,
            SupportedLocale::EnUs => Self::EnUs,
        }
    }
}

pub(super) struct EditorStrings {
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
    file_new_scene: &'static str,
    file_open_scene: &'static str,
    file_open_recent_scene: &'static str,
    file_save: &'static str,
    file_save_all: &'static str,
    file_save_as: &'static str,
    file_auto_backup_settings: &'static str,
    file_restore_backup: &'static str,
    file_publish_game: &'static str,
    file_new_project: &'static str,
    file_open_project: &'static str,
    file_exit: &'static str,
    file_start_server: &'static str,
    window_ai: &'static str,
    window_layout: &'static str,
    window_view: &'static str,
    window_renderer: &'static str,
    window_reduce_tools: &'static str,
    game_object_create_empty: &'static str,
    game_object_create_child: &'static str,
    game_object_create_parent: &'static str,
    game_object_3d_object: &'static str,
    game_object_effects: &'static str,
    game_object_light: &'static str,
    game_object_audio: &'static str,
    game_object_camera: &'static str,
    about_menu_item: &'static str,
    about_title: &'static str,
    about_body: &'static str,
    dialog_close: &'static str,
    status_new_entity: &'static str,
    status_new_child_entity: &'static str,
    status_new_parent_entity: &'static str,
    status_file_menu_item: &'static str,
    status_window_category: &'static str,
    status_game_object_category: &'static str,
    status_playing: &'static str,
    status_paused: &'static str,
    status_step: &'static str,
    status_pivot_center: &'static str,
    status_local_global: &'static str,
    state_editing: &'static str,
    state_playing: &'static str,
    state_paused: &'static str,
    category_title: &'static str,
    categories: [&'static str; 5],
    console_lines: [&'static str; 4],
}

pub(super) fn strings(locale: EditorLocale) -> EditorStrings {
    match locale {
        EditorLocale::ZhCn => EditorStrings {
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
            file_new_scene: "新建场景",
            file_open_scene: "打开场景",
            file_open_recent_scene: "打开最近的场景",
            file_save: "保存",
            file_save_all: "保存全部",
            file_save_as: "另存为",
            file_auto_backup_settings: "自动备份设置",
            file_restore_backup: "恢复备份",
            file_publish_game: "发布游戏",
            file_new_project: "新建项目",
            file_open_project: "打开项目",
            file_exit: "退出",
            file_start_server: "启动服务器...",
            window_ai: "AI",
            window_layout: "布局",
            window_view: "视图",
            window_renderer: "渲染器",
            window_reduce_tools: "减面工具",
            game_object_create_empty: "创建空对象",
            game_object_create_child: "创建空子级",
            game_object_create_parent: "创建空父级",
            game_object_3d_object: "3D 对象",
            game_object_effects: "特效",
            game_object_light: "光源",
            game_object_audio: "音频",
            game_object_camera: "摄像机",
            about_menu_item: "关于 Bevy Sandbox",
            about_title: "关于 Bevy Sandbox",
            about_body: "Bevy Sandbox 是用于构建和调试沙盒项目的编辑器原型。",
            dialog_close: "关闭",
            status_new_entity: "已创建新对象",
            status_new_child_entity: "已创建新子对象",
            status_new_parent_entity: "已创建新父对象",
            status_file_menu_item: "该文件操作暂未接线",
            status_window_category: "该窗口分类暂未接线",
            status_game_object_category: "该游戏对象分类暂未接线",
            status_playing: "运行中",
            status_paused: "已暂停",
            status_step: "单步执行",
            status_pivot_center: "切换枢轴模式",
            status_local_global: "切换本地/世界坐标",
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
        },
        EditorLocale::EnUs => EditorStrings {
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
            file_new_scene: "New Scene",
            file_open_scene: "Open Scene",
            file_open_recent_scene: "Open Recent Scene",
            file_save: "Save",
            file_save_all: "Save All",
            file_save_as: "Save As",
            file_auto_backup_settings: "Auto Backup Settings",
            file_restore_backup: "Restore Backup",
            file_publish_game: "Publish Game",
            file_new_project: "New Project",
            file_open_project: "Open Project",
            file_exit: "Exit",
            file_start_server: "Start Server...",
            window_ai: "AI",
            window_layout: "Layout",
            window_view: "View",
            window_renderer: "Renderer",
            window_reduce_tools: "Reduce Tools",
            game_object_create_empty: "Create Empty",
            game_object_create_child: "Create Empty Child",
            game_object_create_parent: "Create Empty Parent",
            game_object_3d_object: "3D Object",
            game_object_effects: "Effects",
            game_object_light: "Light",
            game_object_audio: "Audio",
            game_object_camera: "Camera",
            about_menu_item: "About Bevy Sandbox",
            about_title: "About Bevy Sandbox",
            about_body: "Bevy Sandbox is an editor prototype for building and debugging sandbox projects.",
            dialog_close: "Close",
            status_new_entity: "Spawned a new entity",
            status_new_child_entity: "Spawned a new child entity",
            status_new_parent_entity: "Spawned a new parent entity",
            status_file_menu_item: "This file action is not wired yet",
            status_window_category: "This window category is not wired yet",
            status_game_object_category: "This game object category is not wired yet",
            status_playing: "Playing",
            status_paused: "Paused",
            status_step: "Step",
            status_pivot_center: "Toggle pivot mode",
            status_local_global: "Toggle local/world space",
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
        },
    }
}

#[derive(Component)]
struct StatusText;

#[derive(Component)]
struct PlayText;

#[derive(Component)]
pub(super) struct ShellButton(ShellAction);

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ShellAction {
    File,
    Edit,
    Window,
    GameObject,
    Component,
    Help,
    About,
    CloseAbout,
    CloseMenus,
    FileMenuItem,
    WindowCategory,
    CreateEmptyObject,
    CreateEmptyChild,
    CreateEmptyParent,
    GameObjectCategory,
    Play,
    Pause,
    Step,
    Pivot,
    LocalGlobal,
}

fn ui_setup(
    mut commands: Commands,
    theme: Res<Theme>,
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
                    column_gap: Val::Px(4.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ))
            .with_children(|menu| {
                for (label, action) in [
                    (i18n.menu_file, ShellAction::File),
                    (i18n.menu_edit, ShellAction::Edit),
                    (i18n.menu_window, ShellAction::Window),
                    (i18n.menu_game_object, ShellAction::GameObject),
                    (i18n.menu_component, ShellAction::Component),
                    (i18n.menu_help, ShellAction::Help),
                ] {
                    spawn_menu_bar_button(menu, &theme, label, action);
                }
            });

            root.spawn((
                Node {
                    height: Val::Px(36.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
            ))
            .with_children(|toolbar| {
                toolbar
                    .spawn((
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(2.0),
                            padding: UiRect::all(Val::Px(4.0)),
                            border_radius: BorderRadius::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.22, 0.22, 0.22)),
                    ))
                    .with_children(|group| {
                        spawn_toolbar_icon_button(
                            group,
                            &theme,
                            icons::PLAY,
                            ShellAction::Play,
                            shell_state.play_state == PlayState::Playing,
                            true,
                        );
                        spawn_toolbar_icon_button(
                            group,
                            &theme,
                            icons::PAUSE,
                            ShellAction::Pause,
                            shell_state.play_state == PlayState::Paused,
                            false,
                        );
                        spawn_toolbar_icon_button(
                            group,
                            &theme,
                            icons::STEP_FORWARD,
                            ShellAction::Step,
                            false,
                            false,
                        );
                        spawn_separator(group);
                        spawn_toolbar_icon_button(
                            group,
                            &theme,
                            icons::BOT,
                            ShellAction::Pivot,
                            false,
                            true,
                        );
                        spawn_toolbar_icon_button(
                            group,
                            &theme,
                            icons::BRUSH_CLEANING,
                            ShellAction::LocalGlobal,
                            false,
                            false,
                        );
                        spawn_toolbar_icon_button(
                            group,
                            &theme,
                            icons::ELLIPSIS_VERTICAL,
                            ShellAction::LocalGlobal,
                            false,
                            false,
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

fn spawn_toolbar_icon_button(
    parent: &mut ChildSpawnerCommands,
    theme: &Theme,
    icon: &str,
    action: ShellAction,
    selected: bool,
    accent: bool,
) {
    parent
        .spawn((
            Button,
            ShellButton(action),
            Node {
                width: Val::Px(28.0),
                height: Val::Px(26.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(if selected {
                Color::srgb(0.43, 0.56, 0.28)
            } else {
                Color::srgb(0.25, 0.25, 0.25)
            }),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(icon),
                TextFont {
                    font: theme.icon.font.clone().into(),
                    font_size: FontSize::Px(17.0),
                    ..default()
                },
                TextColor(if accent {
                    Color::srgb(0.58, 0.84, 0.28)
                } else {
                    Color::srgb(0.78, 0.78, 0.78)
                }),
            ));
        });
}

fn spawn_menu_bar_button(
    parent: &mut ChildSpawnerCommands,
    theme: &Theme,
    label: &str,
    action: ShellAction,
) {
    parent
        .spawn((
            Button,
            ShellButton(action),
            Node {
                height: Val::Px(22.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::horizontal(Val::Px(5.0)),
                border_radius: BorderRadius::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(EditorColors::BUTTON_DEFAULT),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font: theme.text.font.clone().into(),
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(theme.text.text_color),
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
    mut menu_state: ResMut<EditorMenuState>,
    mut commands: Commands,
) {
    let i18n = strings(ui_state.locale);

    for (interaction, button, mut background) in &mut interaction_query {
        let selected = is_selected(button.0, shell_state.play_state);
        match *interaction {
            Interaction::Pressed => {
                match button.0 {
                    ShellAction::File => {
                        menu_state.file_menu_open = !menu_state.file_menu_open;
                        menu_state.window_menu_open = false;
                        menu_state.game_object_menu_open = false;
                        menu_state.help_menu_open = false;
                        shell_state.status = i18n.menu_file_message.to_string();
                    }
                    ShellAction::Edit => shell_state.status = i18n.menu_edit_message.to_string(),
                    ShellAction::Window => {
                        menu_state.window_menu_open = !menu_state.window_menu_open;
                        menu_state.file_menu_open = false;
                        menu_state.game_object_menu_open = false;
                        menu_state.help_menu_open = false;
                        shell_state.status = i18n.menu_window_message.to_string();
                    }
                    ShellAction::GameObject => {
                        menu_state.game_object_menu_open = !menu_state.game_object_menu_open;
                        menu_state.file_menu_open = false;
                        menu_state.window_menu_open = false;
                        menu_state.help_menu_open = false;
                        shell_state.status = i18n.menu_game_object_message.to_string();
                    }
                    ShellAction::Component => {
                        shell_state.status = i18n.menu_component_message.to_string()
                    }
                    ShellAction::Help => {
                        menu_state.help_menu_open = !menu_state.help_menu_open;
                        menu_state.file_menu_open = false;
                        menu_state.window_menu_open = false;
                        menu_state.game_object_menu_open = false;
                    }
                    ShellAction::About => {
                        menu_state.close_menus();
                        menu_state.about_dialog_open = true;
                    }
                    ShellAction::CloseAbout => {
                        menu_state.about_dialog_open = false;
                    }
                    ShellAction::CloseMenus => {
                        menu_state.close_menus();
                    }
                    ShellAction::FileMenuItem => {
                        shell_state.status = i18n.status_file_menu_item.to_string();
                    }
                    ShellAction::WindowCategory => {
                        shell_state.status = i18n.status_window_category.to_string();
                    }
                    ShellAction::CreateEmptyObject => {
                        menu_state.game_object_menu_open = false;
                        spawn_named_entity(&mut commands, "GameObject");
                        shell_state.status = i18n.status_new_entity.to_string();
                    }
                    ShellAction::CreateEmptyChild => {
                        menu_state.game_object_menu_open = false;
                        spawn_named_entity(&mut commands, "Child GameObject");
                        shell_state.status = i18n.status_new_child_entity.to_string();
                    }
                    ShellAction::CreateEmptyParent => {
                        menu_state.game_object_menu_open = false;
                        spawn_named_entity(&mut commands, "Parent GameObject");
                        shell_state.status = i18n.status_new_parent_entity.to_string();
                    }
                    ShellAction::GameObjectCategory => {
                        shell_state.status = i18n.status_game_object_category.to_string();
                    }
                    ShellAction::Play => {
                        shell_state.play_state = PlayState::Playing;
                        shell_state.status = i18n.status_playing.to_string();
                    }
                    ShellAction::Pause => {
                        shell_state.play_state = PlayState::Paused;
                        shell_state.status = i18n.status_paused.to_string();
                    }
                    ShellAction::Step => shell_state.status = i18n.status_step.to_string(),
                    ShellAction::Pivot => shell_state.status = i18n.status_pivot_center.to_string(),
                    ShellAction::LocalGlobal => {
                        shell_state.status = i18n.status_local_global.to_string()
                    }
                }

                *background = BackgroundColor(if is_dismiss_overlay(button.0) {
                    Color::NONE
                } else {
                    button_color(
                        is_selected(button.0, shell_state.play_state),
                        is_ghost(button.0),
                    )
                });
            }
            Interaction::Hovered => {
                *background = BackgroundColor(if is_menu_item(button.0) {
                    Color::srgb(0.32, 0.32, 0.32)
                } else if is_dismiss_overlay(button.0) {
                    Color::NONE
                } else {
                    EditorColors::BUTTON_HOVER
                });
            }
            Interaction::None => {
                *background = BackgroundColor(if is_dismiss_overlay(button.0) {
                    Color::NONE
                } else {
                    button_color(selected, is_ghost(button.0))
                });
            }
        }
    }
}

fn sync_shell_labels(
    shell_state: Res<EditorShellState>,
    ui_state: Res<EditorUiState>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<StatusText>>,
        Query<&mut Text, With<PlayText>>,
    )>,
    mut button_query: Query<(&ShellButton, &mut BackgroundColor), With<Button>>,
) {
    let i18n = strings(ui_state.locale);

    for mut text in &mut text_queries.p0() {
        text.0 = shell_state.status.clone();
    }
    for mut text in &mut text_queries.p1() {
        text.0 = play_state_name(shell_state.play_state, &i18n).to_string();
    }
    if shell_state.is_changed() {
        for (button, mut background) in &mut button_query {
            *background = BackgroundColor(if is_dismiss_overlay(button.0) {
                Color::NONE
            } else {
                button_color(
                    is_selected(button.0, shell_state.play_state),
                    is_ghost(button.0),
                )
            });
        }
    }
}

fn is_ghost(action: ShellAction) -> bool {
    matches!(
        action,
        ShellAction::Play
            | ShellAction::Pause
            | ShellAction::Step
            | ShellAction::Pivot
            | ShellAction::LocalGlobal
            | ShellAction::Help
    )
}

fn is_menu_item(action: ShellAction) -> bool {
    matches!(
        action,
        ShellAction::About
            | ShellAction::FileMenuItem
            | ShellAction::WindowCategory
            | ShellAction::CreateEmptyObject
            | ShellAction::CreateEmptyChild
            | ShellAction::CreateEmptyParent
            | ShellAction::GameObjectCategory
    )
}

fn is_dismiss_overlay(action: ShellAction) -> bool {
    matches!(action, ShellAction::CloseMenus)
}

fn is_selected(action: ShellAction, play_state: PlayState) -> bool {
    match action {
        ShellAction::Play => play_state == PlayState::Playing,
        ShellAction::Pause => play_state == PlayState::Paused,
        _ => false,
    }
}

fn play_state_name(state: PlayState, i18n: &EditorStrings) -> &'static str {
    match state {
        PlayState::Editing => i18n.state_editing,
        PlayState::Playing => i18n.state_playing,
        PlayState::Paused => i18n.state_paused,
    }
}

fn spawn_named_entity(commands: &mut Commands, name: &'static str) {
    commands.spawn((Name::new(name), Transform::default(), Visibility::default()));
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
