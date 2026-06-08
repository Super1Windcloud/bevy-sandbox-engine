//! Meta-module containing all feathers containers (passive widgets that hold other widgets).
use bevy::{
    ecs::system::ResMut,
    feathers::{
        theme::{ThemeToken, UiTheme},
    },
    ui::Val,
};
use bevy_editor_styles::{Theme, colors::EditorColors};

pub mod pane;
pub mod subpane;

pub const HEADER_HEIGHT: Val = Val::Px(30.0);

pub const PANE_HEADER_BG: ThemeToken = ThemeToken::new_static("feathers.pane.header.bg");
pub const PANE_HEADER_BORDER: ThemeToken = ThemeToken::new_static("feathers.pane.header.border");
pub const PANE_HEADER_TEXT: ThemeToken = ThemeToken::new_static("feathers.pane.header.text");
pub const PANE_HEADER_DIVIDER: ThemeToken =
    ThemeToken::new_static("feathers.pane.header.divider");

pub const SUBPANE_HEADER_BG: ThemeToken =
    ThemeToken::new_static("feathers.subpane.header.bg");
pub const SUBPANE_HEADER_BORDER: ThemeToken =
    ThemeToken::new_static("feathers.subpane.header.border");
pub const SUBPANE_HEADER_TEXT: ThemeToken =
    ThemeToken::new_static("feathers.subpane.header.text");
pub const SUBPANE_BODY_BG: ThemeToken = ThemeToken::new_static("feathers.subpane.body.bg");
pub const SUBPANE_BODY_BORDER: ThemeToken =
    ThemeToken::new_static("feathers.subpane.body.border");

pub fn setup(mut ui_theme: ResMut<UiTheme>, theme: &Theme) {
    ui_theme.set_color("feathers.pane.header.bg", theme.pane.header_background_color.0);
    ui_theme.set_color("feathers.pane.header.border", EditorColors::BORDER);
    ui_theme.set_color("feathers.pane.header.text", theme.text.text_color);
    ui_theme.set_color("feathers.pane.header.divider", EditorColors::BORDER);

    ui_theme.set_color("feathers.subpane.header.bg", theme.pane.header_background_color.0);
    ui_theme.set_color("feathers.subpane.header.border", EditorColors::BORDER);
    ui_theme.set_color("feathers.subpane.header.text", theme.text.text_color);
    ui_theme.set_color("feathers.subpane.body.bg", theme.pane.area_background_color.0);
    ui_theme.set_color("feathers.subpane.body.border", EditorColors::BORDER);
}
