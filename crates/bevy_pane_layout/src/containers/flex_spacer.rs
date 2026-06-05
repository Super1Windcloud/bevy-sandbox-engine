use bevy_proto_bsn::{ConstructPatchExt, Scene, pbsn};
use bevy::ui::Node;

/// An invisible UI node that takes up space, and which has a positive `flex_grow` setting.
pub fn flex_spacer() -> impl Scene {
    pbsn! {
        Node {
            flex_grow: 1.0,
        }
    }
}
