//! Module for all the UI components of the Asset Browser

use bevy::prelude::*;
use bevy_editor_styles::{EditorLocale, Theme};
use bevy_pane_layout::prelude::*;

use crate::{AssetBrowserLocation, DirectoryContent, io};

pub mod directory_content;
mod nodes;
pub mod top_bar;

/// The root node for the asset browser.
#[derive(Component)]
pub struct AssetBrowserNode;

#[derive(Component)]
struct AssetBrowserSidebar;

/// Spawn [`AssetBrowserNode`] once the pane is created
#[allow(clippy::too_many_arguments)]
pub fn on_pane_creation(
    structure: In<PaneStructure>,
    mut commands: Commands,
    theme: Res<Theme>,
    location: Res<AssetBrowserLocation>,
    asset_server: Res<AssetServer>,
    directory_content: Res<DirectoryContent>,
) {
    let asset_browser = commands
        .entity(structure.content)
        .insert((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.13, 0.13, 0.14)),
        ))
        .id();

    spawn_project_toolbar(&mut commands, &theme).insert(ChildOf(asset_browser));

    let body = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            min_height: Val::Px(0.0),
            ..default()
        })
        .insert(ChildOf(asset_browser))
        .id();

    spawn_project_sidebar(&mut commands, &theme).insert(ChildOf(body));

    let main_panel = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                min_width: Val::Px(0.0),
                min_height: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.16)),
            ChildOf(body),
        ))
        .id();

    top_bar::spawn_top_bar(&mut commands, &theme, &location).insert(ChildOf(main_panel));
    directory_content::spawn_directory_content(
        &mut commands,
        &directory_content,
        &theme,
        &asset_server,
        &location,
    )
    .insert(ChildOf(main_panel));

    commands.entity(structure.root).insert(AssetBrowserNode);
}

pub(crate) const DEFAULT_SOURCE_ID_NAME: &str = "Default";

pub(crate) fn source_id_to_string(source_id: &crate::AssetSourceId) -> String {
    match source_id {
        crate::AssetSourceId::Default => DEFAULT_SOURCE_ID_NAME.to_string(),
        crate::AssetSourceId::Name(name) => name.to_string(),
    }
}

fn spawn_project_toolbar<'a>(commands: &'a mut Commands, theme: &Res<Theme>) -> EntityCommands<'a> {
    let locale = EditorLocale::detect();
    let (search, name, all_a, all_b) = match locale {
        EditorLocale::ZhCn => ("搜索", "名称", "显示所有", "显示所有"),
        EditorLocale::EnUs => ("Search", "Name", "Show All", "Show All"),
    };

    let toolbar = commands
        .spawn((
            Node {
                height: Val::Px(34.0),
                width: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.16, 0.16, 0.17)),
            BorderColor {
                bottom: Color::srgb(0.24, 0.24, 0.25),
                ..default()
            },
        ))
        .id();

    for label in [search, name, all_a, all_b] {
        commands
            .spawn((
                Node {
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.19, 0.19, 0.20)),
                ChildOf(toolbar),
            ))
            .with_children(|row| {
                row.spawn((
                    Text::new(label),
                    TextFont {
                        font: theme.text.font.clone().into(),
                        font_size: FontSize::Px(11.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.74, 0.75, 0.77)),
                ));
            });
    }

    commands.entity(toolbar)
}

fn spawn_project_sidebar<'a>(commands: &'a mut Commands, theme: &Res<Theme>) -> EntityCommands<'a> {
    let locale = EditorLocale::detect();
    let assets_label = match locale {
        EditorLocale::ZhCn => "资源",
        EditorLocale::EnUs => "Assets",
    };
    let folders = match locale {
        EditorLocale::ZhCn => vec!["GUI", "Prefabs", "Resources", "Scenes", "Scripts"],
        EditorLocale::EnUs => vec!["GUI", "Prefabs", "Resources", "Scenes", "Scripts"],
    };

    let sidebar = commands
        .spawn((
            AssetBrowserSidebar,
            Node {
                width: Val::Px(208.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(6.0),
                border: UiRect::right(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.14, 0.14, 0.15)),
            BorderColor {
                right: Color::srgb(0.24, 0.24, 0.25),
                ..default()
            },
        ))
        .id();

    commands.entity(sidebar).with_children(|parent| {
        parent.spawn((
            Text::new(assets_label),
            TextFont {
                font: theme.text.font.clone().into(),
                font_size: FontSize::Px(12.0),
                ..default()
            },
            TextColor(Color::srgb(0.90, 0.90, 0.92)),
        ));

        for folder in folders {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.17, 0.17, 0.18)),
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new(folder),
                        TextFont {
                            font: theme.text.font.clone().into(),
                            font_size: FontSize::Px(11.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.74, 0.75, 0.77)),
                    ));
                })
                .observe(
                    move |trigger: On<Pointer<Release>>,
                          mut commands: Commands,
                          mut location: ResMut<AssetBrowserLocation>| {
                        if trigger.event().button != PointerButton::Primary {
                            return;
                        }
                        location.source_id = Some(crate::AssetSourceId::Default);
                        location.path = std::path::PathBuf::from(folder);
                        commands.run_system_cached(io::task::fetch_directory_content);
                    },
                );
        }
    });

    commands.entity(sidebar)
}
