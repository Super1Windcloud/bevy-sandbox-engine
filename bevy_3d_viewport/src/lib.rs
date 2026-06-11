//! 3D viewport pane skeleton.
use bevy::prelude::*;
use bevy_editor_styles::Theme;
use bevy_pane_layout::prelude::*;

/// The identifier for the 3D Viewport.
/// This is present on any pane that is a 3D Viewport.
#[derive(Component, Default)]
pub struct Bevy3dViewport;

/// Plugin for the 3D Viewport pane.
pub struct Viewport3dPanePlugin;

impl Plugin for Viewport3dPanePlugin {
    fn build(&self, app: &mut App) {
        app.register_pane("Scene", on_pane_creation);
    }
}

fn on_pane_creation(
    structure: In<PaneStructure>,
    mut commands: Commands,
    theme: Res<Theme>,
) {
    commands.entity(structure.root).insert(Bevy3dViewport);
    commands.entity(structure.content).insert((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(theme.viewport.background_color),
    ));

    commands.entity(structure.content).with_children(|parent| {
        parent.spawn((
            Node {
                height: Val::Px(30.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.09, 0.11, 0.95)),
        ))
        .with_children(|bar| {
            bar.spawn((
                Text::new("Scene"),
                TextFont {
                    font: theme.text.font.clone(),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.88, 0.89, 0.91)),
            ));
            bar.spawn((
                Text::new("Viewport skeleton"),
                TextFont {
                    font: theme.text.font.clone(),
                    font_size: 11.0,
                    ..default()
                },
                TextColor(Color::srgb(0.58, 0.61, 0.66)),
            ));
        });

        parent.spawn((
            Node {
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|content| {
            content.spawn((
                Text::new("3D viewport temporarily reduced to a stable placeholder"),
                TextFont {
                    font: theme.text.font.clone(),
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.72, 0.74, 0.77)),
            ));
        });
    });
}
