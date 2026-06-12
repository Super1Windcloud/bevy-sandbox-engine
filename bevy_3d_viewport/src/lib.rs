//! Minimal stable 3D viewport for the editor shell.
use bevy::{
    camera::{RenderTarget, visibility::RenderLayers},
    image::Image,
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat, TextureUsages},
    ui::ui_layout_system,
    window::PrimaryWindow,
};
use bevy_editor_cam::prelude::{DefaultEditorCamPlugins, EditorCam};
use bevy_editor_styles::Theme;
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings};
use bevy_pane_layout::{components::fit_to_parent, prelude::*};

/// The identifier for the 3D Viewport.
/// This is present on any pane that is a 3D Viewport.
#[derive(Component)]
pub struct Bevy3dViewport {
    camera_id: Entity,
}

impl Default for Bevy3dViewport {
    fn default() -> Self {
        Self {
            camera_id: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Component)]
struct Viewport3dImage {
    camera_id: Entity,
}

#[derive(Component, Default)]
struct RenderTargetResizeState {
    last_size: UVec2,
    stable_frames: u8,
}

/// Plugin for the 3D Viewport pane.
pub struct Viewport3dPanePlugin;

impl Plugin for Viewport3dPanePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<InfiniteGridPlugin>() {
            app.add_plugins(InfiniteGridPlugin);
        }

        app.add_plugins(DefaultEditorCamPlugins)
            .add_systems(Startup, setup)
            .add_systems(
                PostUpdate,
                update_render_target_size.after(ui_layout_system),
            )
            .add_observer(
                |trigger: On<Remove, Bevy3dViewport>,
                 mut commands: Commands,
                 query: Query<&Bevy3dViewport>| {
                    commands
                        .entity(query.get(trigger.event().event_target()).unwrap().camera_id)
                        .despawn();
                },
            );

        app.register_pane("Scene", on_pane_creation);
    }
}

fn setup(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn((
        InfiniteGrid,
        InfiniteGridSettings {
            x_axis_color: theme.viewport.x_axis_color,
            z_axis_color: theme.viewport.z_axis_color,
            major_line_color: theme.viewport.grid_major_line_color,
            minor_line_color: theme.viewport.grid_minor_line_color,
            ..default()
        },
        RenderLayers::layer(1),
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
    let mut image =
        Image::new_target_texture(width, height, TextureFormat::Bgra8UnormSrgb, None);
    image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
    let image_handle = images.add(image);

    let camera_id = commands
        .spawn((
            Camera3d::default(),
            Camera {
                clear_color: ClearColorConfig::Custom(theme.viewport.background_color),
                is_active: false,
                ..default()
            },
            RenderTarget::Image(image_handle.clone().into()),
            EditorCam {
                enabled: false,
                ..default()
            },
            RenderTargetResizeState::default(),
            Transform::from_translation(Vec3::new(5.0, 5.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
            RenderLayers::from_layers(&[0, 1]),
        ))
        .id();

    commands.entity(structure.content).insert(Node {
        flex_grow: 1.0,
        min_width: Val::Px(0.0),
        min_height: Val::Px(0.0),
        ..default()
    });

    commands.entity(structure.content).with_children(|parent| {
        parent.spawn((
            ImageNode::new(image_handle.clone()),
            fit_to_parent(),
            Viewport3dImage { camera_id },
        ));
    });

    commands
        .entity(structure.root)
        .insert(Bevy3dViewport { camera_id });
}

fn update_render_target_size(
    query: Query<(Entity, &Bevy3dViewport)>,
    mut camera_query: Query<(
        &RenderTarget,
        &mut EditorCam,
        &mut Camera,
        &mut RenderTargetResizeState,
    )>,
    content: Query<&PaneContentNode>,
    children_query: Query<&Children>,
    pos_query: Query<&ComputedNode>,
    image_query: Query<&Viewport3dImage>,
    mut images: ResMut<Assets<Image>>,
) {
    for (pane_root, viewport) in &query {
        let Some(content_node_id) = children_query
            .iter_descendants(pane_root)
            .find(|e| content.contains(*e))
        else {
            continue;
        };

        let Ok(computed_node) = pos_query.get(content_node_id) else {
            continue;
        };

        let (target, mut editor_camera, mut camera, mut resize_state) =
            camera_query.get_mut(viewport.camera_id).unwrap();

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
