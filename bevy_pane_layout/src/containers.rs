//! Meta-module containing all feathers containers (passive widgets that hold other widgets).
use bevy::{
    ecs::system::ResMut,
    feathers::{
        palette,
        theme::{ThemeToken, UiTheme},
    },
    ui::Val,
};

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

pub fn setup(mut theme: ResMut<UiTheme>) {
    theme.set_color("feathers.pane.header.bg", palette::GRAY_0);
    theme.set_color("feathers.pane.header.border", palette::WARM_GRAY_1);
    theme.set_color("feathers.pane.header.text", palette::LIGHT_GRAY_1);
    theme.set_color("feathers.pane.header.divider", palette::WARM_GRAY_1);

    theme.set_color("feathers.subpane.header.bg", palette::GRAY_2);
    theme.set_color("feathers.subpane.header.border", palette::GRAY_3);
    theme.set_color("feathers.subpane.header.text", palette::LIGHT_GRAY_1);
    theme.set_color("feathers.subpane.body.bg", palette::GRAY_1);
    theme.set_color("feathers.subpane.body.border", palette::GRAY_2);
}
