//! View gizmo support for the 3D viewport.

use bevy::{
    asset::RenderAssetUsages,
    camera::{RenderTarget, visibility::RenderLayers},
    prelude::*,
    render::render_resource::{Extent3d, Face, TextureDimension, TextureFormat, TextureUsages},
};
use bevy_editor_cam::prelude::EditorCam;
use bevy_editor_styles::Theme;
pub const VIEW_GIZMO_TEXTURE_SIZE: u32 = 125;
pub const VIEW_GIZMO_LAYER: usize = 22;
const GIZMO_CAMERA_ZOOM: f32 = 3.5;

pub struct ViewGizmoPlugin;
impl Plugin for ViewGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_view_gizmo)
            .add_systems(Update, (spawn_view_gizmo_camera, update_view_gizmo));
    }
}

#[derive(Component)]
pub struct ViewGizmoCamera;

#[derive(Component)]
pub struct ViewGizmoCameraTarget(pub Handle<Image>);

pub fn spawn_view_gizmo_node(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    parent: Entity,
) -> Entity {
    let size = Extent3d {
        width: VIEW_GIZMO_TEXTURE_SIZE,
        height: VIEW_GIZMO_TEXTURE_SIZE,
        ..default()
    };

    let mut target_texture = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    target_texture.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image = images.add(target_texture);

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::ZERO,
                left: Val::ZERO,
                width: Val::Px(VIEW_GIZMO_TEXTURE_SIZE as f32),
                height: Val::Px(VIEW_GIZMO_TEXTURE_SIZE as f32),
                ..default()
            },
            ViewGizmoCameraTarget(image.clone()),
            ImageNode::new(image),
            ChildOf(parent),
        ))
        .id()
}

fn setup_view_gizmo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    theme: Res<Theme>,
) {
    let view_gizmo_pass_layer = RenderLayers::layer(VIEW_GIZMO_LAYER);
    let sphere = meshes.add(Sphere::new(0.2).mesh().uv(32, 18));

    for (axis, color) in [
        (Vec3::X, theme.viewport.x_axis_color),
        (Vec3::Y, theme.viewport.y_axis_color),
        (Vec3::Z, theme.viewport.z_axis_color),
    ] {
        let mut gizmo = GizmoAsset::new();
        gizmo.line(Vec3::ZERO, axis, color);
        commands.spawn((
            Gizmo {
                handle: gizmo_assets.add(gizmo),
                line_config: GizmoLineConfig {
                    width: 2.5,
                    ..default()
                },
                ..default()
            },
            Transform::from_xyz(0., 0., 0.),
            view_gizmo_pass_layer.clone(),
        ));
        commands.spawn((
            Mesh3d(sphere.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                unlit: true,
                ..Default::default()
            })),
            Transform::from_translation(axis),
            view_gizmo_pass_layer.clone(),
        ));
    }

    let sphere = meshes.add(Sphere::new(1.3).mesh().uv(32, 18));
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: LinearRgba::new(0.0, 0.0, 0.0, 0.5).into(),
            unlit: true,
            cull_mode: Some(Face::Front),
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        view_gizmo_pass_layer.clone(),
    ));
}

fn spawn_view_gizmo_camera(
    mut commands: Commands,
    q: Query<&ViewGizmoCameraTarget, Added<ViewGizmoCameraTarget>>,
) {
    let view_gizmo_pass_layer = RenderLayers::layer(VIEW_GIZMO_LAYER);
    for target in &q {
        commands.spawn((
            Camera3d::default(),
            Camera {
                clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                ..default()
            },
            RenderTarget::Image(target.0.clone().into()),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
            view_gizmo_pass_layer.clone(),
            ViewGizmoCamera,
        ));
    }
}

fn update_view_gizmo(
    mut view_cube_camera: Query<&mut Transform, (With<ViewGizmoCamera>, With<Camera3d>)>,
    viewport_camera: Query<&Transform, (Without<ViewGizmoCamera>, With<Camera3d>, With<EditorCam>)>,
) {
    for mut transform in &mut view_cube_camera {
        if let Ok(viewport_camera_transform) = viewport_camera.single() {
            transform.translation = viewport_camera_transform.back() * GIZMO_CAMERA_ZOOM;
            transform.rotation = viewport_camera_transform.rotation;
        }
    }
}
