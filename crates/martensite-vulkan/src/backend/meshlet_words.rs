//! Stock mesh shader storage consists of local topology and unchanged binary32 vertices.
use super::meshlet_geometry;
use crate::PrimitiveDraw;

pub(super) fn encode(draw: &PrimitiveDraw, byte_limit: usize) -> Option<(u32, Vec<u8>)> {
    let normals = &draw.lighting.as_ref()?.normals;
    if normals.len() != draw.vertices.len() {
        return None;
    }
    let lower_bound = draw
        .vertices
        .len()
        .checked_mul(11)?
        .checked_add(draw.indices.len() / 3)?
        .checked_mul(4)?;
    if lower_bound > byte_limit {
        return None;
    }
    let groups = meshlet_geometry::build(&draw.vertices, &draw.indices)?;
    let count = u32::try_from(groups.groups.len()).ok()?;
    let vertices = 4_usize.checked_add(groups.groups.len().checked_mul(16)?)?;
    let unique = vertices.checked_add(draw.vertices.len().checked_mul(11)?)?;
    let triangles = unique.checked_add(groups.vertices.len())?;
    let words = triangles.checked_add(groups.triangles.len())?;
    let size = words.checked_mul(4)?;
    if size > byte_limit {
        return None;
    }
    let mut output = Vec::with_capacity(size);
    append(
        &mut output,
        [
            count,
            u32::try_from(vertices).ok()?,
            u32::try_from(unique).ok()?,
            u32::try_from(triangles).ok()?,
        ],
    );
    for group in &groups.groups {
        append(
            &mut output,
            [
                group.first_vertex,
                group.vertex_count,
                group.first_triangle,
                group.triangle_count,
            ]
            .map(|value| value as u32),
        );
        append(&mut output, box_words(group.minimum));
        append(&mut output, box_words(group.maximum));
        let (axis, tangent) = group.cone.unwrap_or(([0.; 3], 0.));
        append(
            &mut output,
            [axis[0], axis[1], axis[2], tangent].map(f32::to_bits),
        );
    }
    for (vertex, normal) in draw.vertices.iter().zip(normals.iter()) {
        append(
            &mut output,
            [
                vertex.position[0],
                vertex.position[1],
                vertex.position[2],
                vertex.weight,
            ]
            .map(f32::to_bits),
        );
        append(&mut output, vertex.color.map(f32::to_bits));
        append(&mut output, normal.map(f32::to_bits));
    }
    append(&mut output, groups.vertices);
    append(
        &mut output,
        groups
            .triangles
            .into_iter()
            .map(|[a, b, c]| u32::from_le_bytes([a, b, c, 0])),
    );
    Some((count, output))
}

fn box_words(point: [f32; 3]) -> [u32; 4] {
    [point[0], point[1], point[2], 0.].map(f32::to_bits)
}

fn append(output: &mut Vec<u8>, words: impl IntoIterator<Item = u32>) {
    output.extend(words.into_iter().flat_map(u32::to_ne_bytes));
}
