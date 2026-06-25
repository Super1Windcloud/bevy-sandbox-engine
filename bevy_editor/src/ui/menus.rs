use bevy::prelude::*;
use bevy_editor_styles::Theme;

use super::{EditorStrings, ShellAction};

#[derive(Resource, Default)]
pub(super) struct EditorMenuState {
    pub(super) file_menu_open: bool,
    pub(super) window_menu_open: bool,
    pub(super) game_object_menu_open: bool,
    pub(super) help_menu_open: bool,
    pub(super) about_dialog_open: bool,
}

impl EditorMenuState {
    pub(super) fn close_menus(&mut self) {
        self.file_menu_open = false;
        self.window_menu_open = false;
        self.game_object_menu_open = false;
        self.help_menu_open = false;
    }
}

#[derive(Component)]
pub(super) struct MenuOverlayElement;

pub(super) fn render_menu_overlays(
    mut commands: Commands,
    menu_state: Res<EditorMenuState>,
    theme: Res<Theme>,
    i18n: Res<super::EditorUiState>,
    overlays: Query<Entity, With<MenuOverlayElement>>,
) {
    if !menu_state.is_changed() {
        return;
    }

    for entity in &overlays {
        commands.entity(entity).despawn();
    }

    let i18n = super::strings(i18n.locale);

    if menu_state.file_menu_open
        || menu_state.window_menu_open
        || menu_state.game_object_menu_open
        || menu_state.help_menu_open
    {
        commands.spawn((
            Button,
            super::ShellButton(ShellAction::CloseMenus),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
            ZIndex(99),
            MenuOverlayElement,
        ));
    }

    if menu_state.file_menu_open {
        spawn_file_menu(&mut commands, &theme, &i18n);
    }

    if menu_state.window_menu_open {
        spawn_window_menu(&mut commands, &theme, &i18n);
    }

    if menu_state.game_object_menu_open {
        spawn_game_object_menu(&mut commands, &theme, &i18n);
    }

    if menu_state.help_menu_open {
        spawn_help_menu(&mut commands, &theme, &i18n);
    }

    if menu_state.about_dialog_open {
        spawn_about_dialog(&mut commands, &theme, &i18n);
    }
}

fn spawn_file_menu(commands: &mut Commands, theme: &Theme, i18n: &EditorStrings) {
    spawn_menu(commands, 8.0, 216.0, |menu| {
        spawn_menu_item(
            menu,
            theme,
            i18n.file_new_scene,
            Some("Ctrl+N"),
            false,
            ShellAction::FileMenuItem,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.file_open_scene,
            Some("Ctrl+O"),
            false,
            ShellAction::FileMenuItem,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.file_open_recent_scene,
            None,
            true,
            ShellAction::FileMenuItem,
        );
        spawn_menu_separator(menu);
        spawn_menu_item(
            menu,
            theme,
            i18n.file_save,
            Some("Ctrl+S"),
            false,
            ShellAction::FileMenuItem,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.file_save_all,
            Some("Ctrl+Shift+S"),
            false,
            ShellAction::FileMenuItem,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.file_save_as,
            Some("Ctrl+Alt+S"),
            false,
            ShellAction::FileMenuItem,
        );
        spawn_menu_separator(menu);
        spawn_menu_item(
            menu,
            theme,
            i18n.file_auto_backup_settings,
            None,
            false,
            ShellAction::FileMenuItem,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.file_restore_backup,
            None,
            false,
            ShellAction::FileMenuItem,
        );
        spawn_menu_separator(menu);
        spawn_menu_item(
            menu,
            theme,
            i18n.file_publish_game,
            None,
            false,
            ShellAction::FileMenuItem,
        );
        spawn_menu_separator(menu);
        spawn_menu_item(
            menu,
            theme,
            i18n.file_new_project,
            None,
            false,
            ShellAction::FileMenuItem,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.file_open_project,
            None,
            false,
            ShellAction::FileMenuItem,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.file_exit,
            None,
            false,
            ShellAction::FileMenuItem,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.file_start_server,
            None,
            false,
            ShellAction::FileMenuItem,
        );
    });
}

fn spawn_window_menu(commands: &mut Commands, theme: &Theme, i18n: &EditorStrings) {
    spawn_menu(commands, 70.0, 144.0, |menu| {
        spawn_menu_item(
            menu,
            theme,
            i18n.window_ai,
            None,
            true,
            ShellAction::WindowCategory,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.window_layout,
            None,
            true,
            ShellAction::WindowCategory,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.window_view,
            None,
            true,
            ShellAction::WindowCategory,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.window_renderer,
            None,
            true,
            ShellAction::WindowCategory,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.window_reduce_tools,
            None,
            false,
            ShellAction::WindowCategory,
        );
    });
}

fn spawn_game_object_menu(commands: &mut Commands, theme: &Theme, i18n: &EditorStrings) {
    spawn_menu(commands, 114.0, 220.0, |menu| {
        spawn_menu_item(
            menu,
            theme,
            i18n.game_object_create_empty,
            Some("Ctrl+Shift+N"),
            false,
            ShellAction::CreateEmptyObject,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.game_object_create_child,
            Some("Alt+Shift+N"),
            false,
            ShellAction::CreateEmptyChild,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.game_object_create_parent,
            Some("Ctrl+Shift+G"),
            false,
            ShellAction::CreateEmptyParent,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.game_object_3d_object,
            None,
            true,
            ShellAction::GameObjectCategory,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.game_object_effects,
            None,
            true,
            ShellAction::GameObjectCategory,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.game_object_light,
            None,
            true,
            ShellAction::GameObjectCategory,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.game_object_audio,
            None,
            true,
            ShellAction::GameObjectCategory,
        );
        spawn_menu_item(
            menu,
            theme,
            i18n.game_object_camera,
            None,
            false,
            ShellAction::GameObjectCategory,
        );
    });
}

fn spawn_help_menu(commands: &mut Commands, theme: &Theme, i18n: &EditorStrings) {
    spawn_menu(commands, 198.0, 180.0, |menu| {
        spawn_menu_item(
            menu,
            theme,
            i18n.about_menu_item,
            None,
            false,
            ShellAction::About,
        );
    });
}

fn spawn_menu(
    commands: &mut Commands,
    left: f32,
    width: f32,
    build: impl FnOnce(&mut ChildSpawnerCommands),
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(28.0),
                left: Val::Px(left),
                width: Val::Px(width),
                padding: UiRect::vertical(Val::Px(4.0)),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.19, 0.19, 0.19)),
            BorderColor::all(Color::srgb(0.34, 0.34, 0.34)),
            ZIndex(100),
            MenuOverlayElement,
        ))
        .with_children(build);
}

fn spawn_menu_item(
    parent: &mut ChildSpawnerCommands,
    theme: &Theme,
    label: &str,
    shortcut: Option<&str>,
    has_submenu: bool,
    action: ShellAction,
) {
    parent
        .spawn((
            Button,
            super::ShellButton(action),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(22.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|item| {
            item.spawn((
                Text::new(label),
                TextFont {
                    font: theme.text.font.clone().into(),
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(theme.text.text_color),
            ));

            let trailing = if has_submenu {
                "›"
            } else {
                shortcut.unwrap_or("")
            };

            item.spawn((
                Text::new(trailing),
                TextFont {
                    font: theme.text.font.clone().into(),
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(theme.text.low_priority),
            ));
        });
}

fn spawn_menu_separator(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            margin: UiRect::vertical(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.34, 0.34, 0.34)),
    ));
}

fn spawn_about_dialog(commands: &mut Commands, theme: &Theme, i18n: &EditorStrings) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.48)),
            ZIndex(101),
            MenuOverlayElement,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: Val::Px(360.0),
                        padding: UiRect::all(Val::Px(18.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.0),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.13, 0.13, 0.15)),
                ))
                .with_children(|dialog| {
                    dialog.spawn((
                        Text::new(i18n.about_title),
                        TextFont {
                            font: theme.text.font.clone().into(),
                            font_size: FontSize::Px(16.0),
                            ..default()
                        },
                        TextColor(theme.text.text_color),
                    ));
                    dialog.spawn((
                        Text::new(i18n.about_body),
                        TextFont {
                            font: theme.text.font.clone().into(),
                            font_size: FontSize::Px(12.0),
                            ..default()
                        },
                        TextColor(theme.text.low_priority),
                    ));
                    spawn_menu_item(
                        dialog,
                        theme,
                        i18n.dialog_close,
                        None,
                        false,
                        ShellAction::CloseAbout,
                    );
                });
        });
}
