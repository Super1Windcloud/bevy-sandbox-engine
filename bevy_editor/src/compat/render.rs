use std::{
    fs,
    path::{Path, PathBuf},
};

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::tailwind,
    log::{info, warn},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use serde_json::Value;
use walkdir::WalkDir;

use super::{
    CompatNode, CompatNodeMarker, CompatNodeSource, CompatProjectManifest, CompatProjectResource,
    CompatSceneRoot, CompatScriptList, find_bytes, find_field, find_field_from,
    normalize_lookup_key, read_f32_le, read_u32_le,
};

pub(super) fn migrate_default_scene(
    mut commands: Commands,
    compat_project: Option<Res<CompatProjectResource>>,
    existing_roots: Query<Entity, With<CompatSceneRoot>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let Some(compat_project) = compat_project else {
        return;
    };
    if !existing_roots.is_empty() {
        return;
    }

    let Some(scene) = compat_project
        .scenes
        .iter()
        .find(|scene| scene.name.eq_ignore_ascii_case("DefaultScene"))
        .or_else(|| compat_project.scenes.first())
    else {
        return;
    };

    let root = commands
        .spawn((
            Name::new(format!("CompatScene: {}", scene.name)),
            CompatSceneRoot {
                path: scene.path.clone(),
            },
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    for node in &scene.nodes {
        let source_label = match &node.source {
            CompatNodeSource::NativeSceneObject => "scene-object".to_string(),
            CompatNodeSource::PrefabInstance {
                prefab_path,
                prefab_name,
                prefab_uuid,
            } => format!(
                "prefab:{}:{}:{}",
                prefab_name.clone().unwrap_or_else(|| "unknown".to_string()),
                prefab_path
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "unresolved".to_string()),
                prefab_uuid.clone().unwrap_or_else(|| "no-uuid".to_string())
            ),
        };

        let entity = commands
            .spawn((
                Name::new(node.name.clone()),
                CompatNodeMarker { source_label },
                CompatScriptList(node.script_components.clone()),
                node.transform,
                Visibility::default(),
            ))
            .id();

        commands.entity(root).add_child(entity);

        let spawned_render_assets = spawn_prefab_render_assets(
            &mut commands,
            &compat_project.manifest,
            node,
            entity,
            &mut meshes,
            &mut materials,
            &asset_server,
        );

        if !spawned_render_assets {
            let color = if node.script_components.is_empty() {
                tailwind::SLATE_500
            } else {
                tailwind::EMERALD_500
            };
            commands.entity(entity).insert((
                Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(0.35)))),
                MeshMaterial3d(materials.add(Color::from(color))),
            ));
        }
    }

    info!(
        "Migrated compatibility scene '{}' into {} Bevy placeholder entities",
        scene.path.display(),
        scene.nodes.len()
    );
}

fn spawn_prefab_render_assets(
    commands: &mut Commands,
    manifest: &CompatProjectManifest,
    node: &CompatNode,
    parent: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
) -> bool {
    let CompatNodeSource::PrefabInstance {
        prefab_path: Some(prefab_path),
        ..
    } = &node.source
    else {
        return false;
    };

    let Some(prefab) = manifest
        .prefabs
        .iter()
        .find(|prefab| &prefab.path == prefab_path)
    else {
        return false;
    };

    let prefab_asset_dir = prefab_path.parent().unwrap_or_else(|| Path::new(""));
    let mesh_dir = prefab_asset_dir.join("Mesh");
    let material_dir = prefab_asset_dir.join("Material");
    let mut spawned_any = false;

    for render_node in &prefab.render_nodes {
        let Some(mesh_path) =
            find_named_asset(&manifest.assets_root, &mesh_dir, &render_node.name, "mesh")
        else {
            continue;
        };

        let absolute_mesh_path = manifest.assets_root.join(&mesh_path);
        let Some(mesh) = load_compat_mesh(&absolute_mesh_path) else {
            warn!(
                "Failed to decode compatibility mesh '{}'",
                absolute_mesh_path.display()
            );
            continue;
        };

        let material = find_named_asset(
            &manifest.assets_root,
            &material_dir,
            &render_node.name,
            "mat",
        )
        .and_then(|path| load_compat_material(manifest, &path, materials, asset_server))
        .unwrap_or_else(|| materials.add(default_preview_material(&render_node.name)));

        commands.entity(parent).with_children(|parent| {
            parent.spawn((
                Name::new(format!("{} mesh", render_node.name)),
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material),
                render_node.transform,
                Visibility::default(),
            ));
        });
        spawned_any = true;
    }

    spawned_any
}

fn find_named_asset(
    assets_root: &Path,
    preferred_dir: &Path,
    name: &str,
    extension: &str,
) -> Option<PathBuf> {
    let direct = preferred_dir.join(format!("{name}.{extension}"));
    if assets_root.join(&direct).is_file() {
        return Some(direct);
    }

    let normalized_name = normalize_lookup_key(name);
    WalkDir::new(assets_root)
        .into_iter()
        .filter_map(Result::ok)
        .find_map(|entry| {
            if !entry.file_type().is_file() {
                return None;
            }
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some(extension) {
                return None;
            }
            let stem = path.file_stem()?.to_string_lossy();
            if normalize_lookup_key(&stem) == normalized_name {
                path.strip_prefix(assets_root).ok().map(Path::to_path_buf)
            } else {
                None
            }
        })
}

fn default_preview_material(name: &str) -> StandardMaterial {
    let palette = [
        tailwind::SKY_500,
        tailwind::EMERALD_500,
        tailwind::AMBER_500,
        tailwind::ROSE_500,
        tailwind::VIOLET_500,
    ];
    let hash = name.bytes().fold(0usize, |acc, byte| {
        acc.wrapping_mul(31).wrapping_add(byte as usize)
    });
    StandardMaterial {
        base_color: Color::from(palette[hash % palette.len()]),
        perceptual_roughness: 0.8,
        ..default()
    }
}

fn load_compat_material(
    manifest: &CompatProjectManifest,
    relative_path: &Path,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
) -> Option<Handle<StandardMaterial>> {
    let text = fs::read_to_string(manifest.assets_root.join(relative_path)).ok()?;
    let value = serde_json::from_str::<Value>(&text).ok()?;
    let base_color = material_color(&value).unwrap_or(Color::WHITE);
    let texture_path = material_main_texture_path(manifest, &value);
    let base_color_texture = texture_path.map(|path| {
        let asset_path = path.to_string_lossy().replace('\\', "/");
        asset_server.load(asset_path)
    });

    Some(materials.add(StandardMaterial {
        base_color,
        base_color_texture,
        perceptual_roughness: 0.8,
        ..default()
    }))
}

fn material_color(value: &Value) -> Option<Color> {
    let vectors = value.get("vectors")?.as_array()?;
    let color = vectors.iter().find(|entry| {
        entry
            .get("name")
            .and_then(Value::as_str)
            .is_some_and(|name| name == "_Color")
    })?;
    let rgba = color.get("value")?.as_array()?;
    let r = rgba.first()?.as_f64()? as f32;
    let g = rgba.get(1)?.as_f64()? as f32;
    let b = rgba.get(2)?.as_f64()? as f32;
    let a = rgba.get(3).and_then(Value::as_f64).unwrap_or(1.0) as f32;
    Some(Color::srgba(r, g, b, a))
}

fn material_main_texture_path(manifest: &CompatProjectManifest, value: &Value) -> Option<PathBuf> {
    let textures = value.get("textures")?.as_array()?;
    let texture = textures.iter().find(|entry| {
        entry
            .get("name")
            .and_then(Value::as_str)
            .is_some_and(|name| name == "_MainTex")
    })?;
    let uuid = texture
        .get("value")?
        .get("id")?
        .as_str()?
        .to_ascii_uppercase();
    manifest.uuid_index.get(&uuid).cloned()
}

fn load_compat_mesh(path: &Path) -> Option<Mesh> {
    let bytes = fs::read(path).ok()?;
    let indices = read_mesh_indices(&bytes)?;
    let vertex_data_start = find_bytes(&bytes, b"vertex_data", 0)?;
    let data_field = find_field_from(&bytes, b"data", vertex_data_start + b"vertex_data".len())?;
    let vertex_data = read_field_bytes_at(&bytes, data_field, b"data")?;
    let max_index = indices.iter().copied().max()? as usize;
    let vertex_count = max_index + 1;
    let stride = vertex_data.len().checked_div(vertex_count)?;

    if stride >= 12 && stride.checked_mul(vertex_count)? == vertex_data.len() {
        read_positions_with_stride(vertex_data, vertex_count, stride)
            .filter(|positions| score_position_candidate(positions, &indices) > 0.0)
            .map(|positions| mesh_from_positions_and_indices(positions, indices.clone()))
    } else {
        find_positions_in_vertex_blob(vertex_data, vertex_count, &indices)
            .map(|positions| mesh_from_positions_and_indices(positions, indices))
    }
}

fn read_mesh_indices(bytes: &[u8]) -> Option<Vec<u32>> {
    let field_offset = find_field(bytes, b"indices")?;
    let data = read_field_bytes_at(bytes, field_offset, b"indices")?;
    if data.len() % 2 != 0 {
        return None;
    }
    Some(
        data.chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]) as u32)
            .collect(),
    )
}

fn read_field_bytes_at<'a>(bytes: &'a [u8], field_offset: usize, field: &[u8]) -> Option<&'a [u8]> {
    let byte_len_offset = field_offset + 1 + field.len();
    let byte_len = read_u32_le(bytes, byte_len_offset)? as usize;
    let data_start = byte_len_offset + 4;
    bytes.get(data_start..data_start + byte_len)
}

fn read_positions_with_stride(
    blob: &[u8],
    vertex_count: usize,
    stride: usize,
) -> Option<Vec<[f32; 3]>> {
    let mut positions = Vec::with_capacity(vertex_count);
    for vertex in 0..vertex_count {
        let base = vertex.checked_mul(stride)?;
        let x = read_f32_le(blob, base)?;
        let y = read_f32_le(blob, base + 4)?;
        let z = read_f32_le(blob, base + 8)?;
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return None;
        }
        positions.push([x, y, z]);
    }
    Some(positions)
}

fn find_positions_in_vertex_blob(
    blob: &[u8],
    vertex_count: usize,
    indices: &[u32],
) -> Option<Vec<[f32; 3]>> {
    let byte_len = vertex_count.checked_mul(12)?;
    if byte_len > blob.len() {
        return None;
    }

    let mut best: Option<(usize, Vec<[f32; 3]>, f32)> = None;
    let search_len = blob.len().saturating_sub(byte_len).min(4096);
    for offset in (0..search_len).step_by(4) {
        let Some(positions) = read_position_candidate(blob, offset, vertex_count) else {
            continue;
        };
        let score = score_position_candidate(&positions, indices);
        if score <= 0.0 {
            continue;
        }
        if best
            .as_ref()
            .is_none_or(|(_, _, best_score)| score > *best_score)
        {
            best = Some((offset, positions, score));
        }
    }

    best.map(|(_, positions, _)| positions)
}

fn read_position_candidate(
    blob: &[u8],
    offset: usize,
    vertex_count: usize,
) -> Option<Vec<[f32; 3]>> {
    let mut positions = Vec::with_capacity(vertex_count);
    for vertex in 0..vertex_count {
        let base = offset + vertex * 12;
        let x = read_f32_le(blob, base)?;
        let y = read_f32_le(blob, base + 4)?;
        let z = read_f32_le(blob, base + 8)?;
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return None;
        }
        positions.push([x, y, z]);
    }
    Some(positions)
}

fn score_position_candidate(positions: &[[f32; 3]], indices: &[u32]) -> f32 {
    if positions.is_empty() {
        return 0.0;
    }

    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);
    let mut finite_count = 0usize;
    for position in positions {
        let p = Vec3::from_array(*position);
        if p.length_squared() > 10_000.0 {
            return 0.0;
        }
        min = min.min(p);
        max = max.max(p);
        finite_count += 1;
    }

    let extent = max - min;
    if extent.max_element() < 0.0001 || extent.max_element() > 100.0 {
        return 0.0;
    }

    let mut triangle_area = 0.0;
    let mut triangle_count = 0usize;
    for tri in indices.chunks_exact(3).take(128) {
        let a = Vec3::from_array(positions[tri[0] as usize]);
        let b = Vec3::from_array(positions[tri[1] as usize]);
        let c = Vec3::from_array(positions[tri[2] as usize]);
        triangle_area += (b - a).cross(c - a).length();
        triangle_count += 1;
    }

    if triangle_count == 0 || triangle_area <= 0.0001 {
        return 0.0;
    }

    finite_count as f32 + triangle_area.min(1000.0)
}

fn mesh_from_positions_and_indices(positions: Vec<[f32; 3]>, indices: Vec<u32>) -> Mesh {
    let normals = generate_normals(&positions, &indices);
    let uvs = vec![[0.0, 0.0]; positions.len()];
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn generate_normals(positions: &[[f32; 3]], indices: &[u32]) -> Vec<[f32; 3]> {
    let mut normals = vec![Vec3::ZERO; positions.len()];
    for tri in indices.chunks_exact(3) {
        let ia = tri[0] as usize;
        let ib = tri[1] as usize;
        let ic = tri[2] as usize;
        if ia >= positions.len() || ib >= positions.len() || ic >= positions.len() {
            continue;
        }
        let a = Vec3::from_array(positions[ia]);
        let b = Vec3::from_array(positions[ib]);
        let c = Vec3::from_array(positions[ic]);
        let normal = (b - a).cross(c - a);
        normals[ia] += normal;
        normals[ib] += normal;
        normals[ic] += normal;
    }

    normals
        .into_iter()
        .map(|normal| normal.try_normalize().unwrap_or(Vec3::Y).to_array())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_blockman_scene_meshes() {
        for path in [
            r"C:\Users\superuse\Downloads\shooting\Assets\Resources\Scene\Plane\Plane.mesh",
            r"C:\Users\superuse\Downloads\shooting\Assets\Resources\Scene\RadarCar\RadarCar.mesh",
            r"C:\Users\superuse\Downloads\shooting\Assets\Resources\Scene\Car_01\Car_01.mesh",
        ] {
            let path = Path::new(path);
            if path.exists() {
                assert!(load_compat_mesh(path).is_some(), "{}", path.display());
            }
        }
    }
}
