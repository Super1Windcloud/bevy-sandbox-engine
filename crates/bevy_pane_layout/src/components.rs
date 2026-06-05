//! Pane UI component helpers.

use bevy::prelude::*;

/// Align this node's position and size with its parent.
pub fn fit_to_parent() -> Node {
    Node {
        position_type: PositionType::Absolute,
        top: Val::ZERO,
        bottom: Val::ZERO,
        left: Val::ZERO,
        right: Val::ZERO,
        ..default()
    }
}
