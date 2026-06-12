use bevy::{
    feathers::theme::{ThemeBackgroundColor, ThemeBorderColor, ThemeTextColor},
    prelude::*,
};
use bevy_editor_styles::Theme;

use super::{
    SUBPANE_BODY_BG, SUBPANE_BODY_BORDER, SUBPANE_HEADER_BG, SUBPANE_HEADER_BORDER,
    SUBPANE_HEADER_TEXT,
};

#[allow(dead_code)]
pub fn header_node() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        border: UiRect {
            left: Val::Px(1.0),
            top: Val::Px(1.0),
            right: Val::Px(1.0),
            bottom: Val::Px(0.0),
        },
        padding: UiRect::axes(Val::Px(10.0), Val::Px(0.0)),
        min_height: Val::Px(30.0),
        column_gap: Val::Px(4.0),
        ..default()
    }
}

#[allow(dead_code)]
pub fn header_theme() -> (ThemeBackgroundColor, ThemeBorderColor, ThemeTextColor) {
    (
        ThemeBackgroundColor(SUBPANE_HEADER_BG),
        ThemeBorderColor(SUBPANE_HEADER_BORDER),
        ThemeTextColor(SUBPANE_HEADER_TEXT),
    )
}

pub fn body_node() -> Node {
    Node {
        flex_grow: 1.,
        border: UiRect {
            left: Val::Px(1.0),
            top: Val::Px(0.0),
            right: Val::Px(1.0),
            bottom: Val::Px(1.0),
        },
        padding: UiRect::axes(Val::Px(6.0), Val::Px(6.0)),
        ..default()
    }
}

pub fn body_theme() -> (ThemeBackgroundColor, ThemeBorderColor, ThemeTextColor) {
    (
        ThemeBackgroundColor(SUBPANE_BODY_BG),
        ThemeBorderColor(SUBPANE_BODY_BORDER),
        ThemeTextColor(SUBPANE_HEADER_TEXT),
    )
}

#[allow(dead_code)]
pub fn body_font(theme: &Theme) -> TextFont {
    TextFont {
        font: theme.text.font.clone().into(),
        font_size: FontSize::Px(14.0),
        ..default()
    }
}
