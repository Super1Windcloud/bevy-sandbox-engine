//! 2d Viewport for Bevy
use bevy::{
    camera::{RenderTarget, visibility::RenderLayers},
    image::Image,
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat, TextureUsages},
    ui::ui_layout_system,
    window::PrimaryWindow,
};
use bevy_editor_camera::{EditorCamera2d, EditorCamera2dPlugin};
use bevy_editor_styles::Theme;
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings};
use bevy_pane_layout::{components::fit_to_parent, prelude::*};

/// The identifier for the 2D Viewport.
#[derive(Component)]
pub struct Bevy2dViewport {
    camera_id: Entity,
}

impl Default for Bevy2dViewport {
    fn default() -> Self {
        Bevy2dViewport {
            camera_id: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Component)]
struct Viewport2dImage {
    camera_id: Entity,
}

#[derive(Component, Default)]
struct RenderTargetResizeState {
    last_size: UVec2,
    stable_frames: u8,
}

/// Plugin for the 2D Viewport pane.
pub struct Viewport2dPanePlugin;

impl Plugin for Viewport2dPanePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<InfiniteGridPlugin>() {
            app.add_plugins(InfiniteGridPlugin);
        }
        app.add_plugins(EditorCamera2dPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                PostUpdate,
                update_render_target_size.after(ui_layout_system),
            )
            .add_observer(
                |trigger: On<Remove, Bevy2dViewport>,
                 mut commands: Commands,
                 query: Query<&Bevy2dViewport>| {
                    commands
                        .entity(query.get(trigger.event().event_target()).unwrap().camera_id)
                        .despawn();
                },
            );

        app.register_pane("Game", on_pane_creation);
    }
}

fn setup(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn((
        InfiniteGrid,
        InfiniteGridSettings {
            scale: 100.,
            dot_fadeout_strength: 0.,
            x_axis_color: theme.viewport.x_axis_color,
            z_axis_color: theme.viewport.y_axis_color,
            major_line_color: theme.viewport.grid_major_line_color,
            minor_line_color: theme.viewport.grid_minor_line_color,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_arc(Vec3::Y, Vec3::Z)),
        RenderLayers::layer(2),
    ));
}

fn on_pane_creation(
    structure: In<PaneStructure>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    theme: Res<Theme>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
) {
    let window_size = primary_window.resolution.physical_size();
    let width = window_size.x.max(1);
    let height = window_size.y.max(1);
    let mut image = Image::new_target_texture(width, height, TextureFormat::Bgra8UnormSrgb, None);
    image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
    let image_handle = images.add(image);

    let camera_id = commands
        .spawn((
            Camera2d,
            EditorCamera2d {
                enabled: false,
                ..default()
            },
            Camera {
                clear_color: ClearColorConfig::Custom(theme.viewport.background_color),
                is_active: false,
                ..default()
            },
            RenderTargetResizeState::default(),
            RenderTarget::Image(image_handle.clone().into()),
            RenderLayers::from_layers(&[0, 2]),
        ))
        .id();

    commands.entity(structure.content).insert(Node {
        flex_grow: 1.0,
        ..default()
    });

    commands.entity(structure.content).with_children(|parent| {
        parent.spawn((
            ImageNode::new(image_handle.clone()),
            fit_to_parent(),
            Viewport2dImage { camera_id },
        ));
    });

    commands
        .entity(structure.root)
        .insert(Bevy2dViewport { camera_id });
}

fn update_render_target_size(
    query: Query<(Entity, &Bevy2dViewport)>,
    mut camera_query: Query<(
        &RenderTarget,
        &mut EditorCamera2d,
        &mut Camera,
        &mut RenderTargetResizeState,
    )>,
    content: Query<&PaneContentNode>,
    children_query: Query<&Children>,
    pos_query: Query<(&ComputedNode, &UiGlobalTransform)>,
    image_query: Query<&Viewport2dImage>,
    mut images: ResMut<Assets<Image>>,
) {
    for (pane_root, viewport) in &query {
        let Some(content_node_id) = children_query
            .iter_descendants(pane_root)
            .find(|e| content.contains(*e))
        else {
            continue;
        };

        let Ok((computed_node, global_transform)) = pos_query.get(content_node_id) else {
            continue;
        };

        let node_position = global_transform.translation;
        let rect = Rect::from_center_size(node_position, computed_node.size());
        let (target, mut editor_camera, mut camera, mut resize_state) =
            camera_query.get_mut(viewport.camera_id).unwrap();
        editor_camera.viewport_override = Some(rect);

        let RenderTarget::Image(image_handle) = target else {
            continue;
        };
        let size = Extent3d {
            width: u32::max(1, computed_node.size().x as u32),
            height: u32::max(1, computed_node.size().y as u32),
            depth_or_array_layers: 1,
        };
        let mut image = images.get_mut(&image_handle.handle).unwrap();
        let current = image.texture_descriptor.size;
        let requested = UVec2::new(size.width, size.height);
        if resize_state.last_size != requested {
            resize_state.last_size = requested;
            resize_state.stable_frames = 0;
        }

        if current.width != size.width || current.height != size.height {
            image.resize(size);
            camera.is_active = false;
            editor_camera.enabled = false;
            continue;
        }

        if resize_state.stable_frames < 1 {
            resize_state.stable_frames += 1;
            camera.is_active = false;
            editor_camera.enabled = false;
            continue;
        }

        camera.is_active = size.width > 1 && size.height > 1;
        editor_camera.enabled = camera.is_active;

        for image_entity in children_query.iter_descendants(content_node_id) {
            if let Ok(image) = image_query.get(image_entity) {
                editor_camera.enabled = image.camera_id == viewport.camera_id && camera.is_active;
            }
        }
    }
}
