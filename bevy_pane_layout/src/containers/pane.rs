use bevy::{
    feathers::{
        cursor::EntityCursor,
        theme::{ThemeBackgroundColor, ThemeBorderColor, ThemeFontColor},
    },
    prelude::*,
    window::SystemCursorIcon,
};
use bevy_editor_styles::Theme;

use super::PANE_HEADER_DIVIDER;
use super::{HEADER_HEIGHT, PANE_HEADER_BG, PANE_HEADER_BORDER, PANE_HEADER_TEXT};

pub fn root_node() -> Node {
    Node {
        padding: UiRect::all(Val::Px(1.5)),
        ..default()
    }
}

pub fn area_node(theme: &Theme) -> Node {
    Node {
        overflow: Overflow::clip(),
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        border_radius: theme.general.border_radius,
        ..default()
    }
}

pub fn header_node(theme: &Theme) -> Node {
    Node {
        padding: UiRect::axes(Val::Px(5.), Val::Px(3.)),
        width: Val::Percent(100.),
        min_height: HEADER_HEIGHT,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        flex_shrink: 0.,
        border: UiRect::all(Val::Px(1.0)),
        border_radius: theme.pane.header_border_radius,
        column_gap: Val::Px(6.0),
        ..default()
    }
}

pub fn header_title_row_node() -> Node {
    Node {
        align_items: AlignItems::Center,
        flex_shrink: 0.0,
        column_gap: Val::Px(2.0),
        ..default()
    }
}

pub fn header_theme() -> (ThemeBackgroundColor, ThemeBorderColor, ThemeFontColor) {
    (
        ThemeBackgroundColor(PANE_HEADER_BG),
        ThemeBorderColor(PANE_HEADER_BORDER),
        ThemeFontColor(PANE_HEADER_TEXT),
    )
}

pub fn header_cursor() -> EntityCursor {
    EntityCursor::System(SystemCursorIcon::Pointer)
}

pub fn header_divider_node() -> Node {
    Node {
        width: Val::Px(1.0),
        height: Val::Px(14.0),
        margin: UiRect::horizontal(Val::Px(4.0)),
        ..default()
    }
}

pub fn header_divider_theme() -> ThemeBackgroundColor {
    ThemeBackgroundColor(PANE_HEADER_DIVIDER)
}

pub fn title_font(theme: &Theme) -> TextFont {
    TextFont {
        font: theme.text.font.clone(),
        font_size: 14.0,
        ..default()
    }
}
