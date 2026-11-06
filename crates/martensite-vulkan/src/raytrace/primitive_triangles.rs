//! GlView::addRaytracePrimitiveArray and addRaytrace*Array: the triangles a
//! surface primitive array contributes to ray tracing, per bound, with the group's material id.
use super::triangle_set::RaytraceTriangleSet;
use crate::RenderError;
use crate::primitive::PrimitiveSubmission;
use martensite_tkmath::bvh::BvhVec3f;
use martensite_tkservice::graphic3d::Graphic3dTypeOfPrimitiveArray as Kind;

/// Whether the array is ray traced (`DrawMode()` from GL_TRIANGLES to GL_POLYGON).
/// Example: `is_raytraced(Graphic3dTypeOfPrimitiveArray::Triangles)`.
pub(crate) fn is_raytraced(kind: Kind) -> bool {
    matches!(
        kind,
        Kind::Triangles
            | Kind::Trianglestrips
            | Kind::Trianglefans
            | Kind::Quadrangles
            | Kind::Quadranglestrips
            | Kind::Polygons
    )
}

/// The triangle set of one array, `None` for arrays that are not ray traced; vertices stay in
/// object space (the structure transform is the set's). Missing normals and texture
/// coordinates are zero. Example: `triangle_set(&primitive, 3)?`.
pub(crate) fn triangle_set(
    primitive: &PrimitiveSubmission,
    material: i32,
) -> Result<Option<RaytraceTriangleSet>, RenderError> {
    if !is_raytraced(primitive.kind) || primitive.attributes.is_null() {
        return Ok(None);
    }
    let positions = primitive.positions()?;
    let count = positions.len();
    let mut set = RaytraceTriangleSet {
        vertices: positions
            .iter()
            .map(|p| BvhVec3f::new(p[0], p[1], p[2]))
            .collect(),
        normals: primitive.normals()?.unwrap_or_else(|| vec![[0.; 3]; count]),
        tex_coords: crate::surface_texture_uv::surface_input_uv(primitive)?
            .iter()
            .map(|uv| [uv[0], uv[1]])
            .collect(),
        ..Default::default()
    };
    let indices = primitive.read_indices(count)?;
    for range in crate::primitive_ranges::primitive_ranges(&primitive.bounds, &indices, [0.; 4])? {
        append_triangles(&mut set.elements, primitive.kind, &range.indices, material);
    }
    Ok(Some(set))
}

/// The triangles of one bound in ray-tracing order.
fn append_triangles(elements: &mut Vec<[i32; 4]>, kind: Kind, indices: &[u32], material: i32) {
    let index = |i: usize| indices[i] as i32;
    let count = indices.len();
    let mut push =
        |a: usize, b: usize, c: usize| elements.push([index(a), index(b), index(c), material]);
    match kind {
        Kind::Triangles => (0..count.saturating_sub(2))
            .step_by(3)
            .for_each(|v| push(v, v + 1, v + 2)),
        Kind::Trianglefans | Kind::Polygons => {
            (0..count.saturating_sub(2)).for_each(|v| push(0, v + 1, v + 2))
        }
        // Odd triangles swap their first two vertices to keep the winding.
        Kind::Trianglestrips => (0..count.saturating_sub(2)).for_each(|v| {
            let odd = v % 2;
            push(v + odd, v + 1 - odd, v + 2)
        }),
        Kind::Quadrangles => (0..count.saturating_sub(3)).step_by(4).for_each(|v| {
            push(v, v + 1, v + 2);
            push(v, v + 2, v + 3);
        }),
        Kind::Quadranglestrips => (0..count.saturating_sub(3)).step_by(2).for_each(|v| {
            push(v, v + 1, v + 2);
            push(v + 1, v + 3, v + 2);
        }),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn lowered(kind: Kind, count: u32) -> Vec<[i32; 4]> {
        let mut elements = Vec::new();
        append_triangles(&mut elements, kind, &(0..count).collect::<Vec<_>>(), 7);
        elements
    }
    #[test]
    fn triangle_lists_drop_incomplete_triangles() {
        assert_eq!(
            lowered(Kind::Triangles, 7),
            vec![[0, 1, 2, 7], [3, 4, 5, 7]]
        );
        assert!(lowered(Kind::Triangles, 2).is_empty());
    }
    #[test]
    fn strips_alternate_winding_and_fans_share_the_first_vertex() {
        assert_eq!(
            lowered(Kind::Trianglestrips, 5),
            vec![[0, 1, 2, 7], [2, 1, 3, 7], [2, 3, 4, 7]]
        );
        assert_eq!(
            lowered(Kind::Polygons, 5),
            vec![[0, 1, 2, 7], [0, 2, 3, 7], [0, 3, 4, 7]]
        );
    }
    #[test]
    fn quadrangle_strips_follow_the_ray_tracing_split() {
        assert_eq!(
            lowered(Kind::Quadrangles, 4),
            vec![[0, 1, 2, 7], [0, 2, 3, 7]]
        );
        assert_eq!(
            lowered(Kind::Quadranglestrips, 6),
            vec![[0, 1, 2, 7], [1, 3, 2, 7], [2, 3, 4, 7], [3, 5, 4, 7]]
        );
    }
    #[test]
    fn segments_and_points_are_not_ray_traced() {
        assert!(!is_raytraced(Kind::Segments) && !is_raytraced(Kind::Points));
        assert!(is_raytraced(Kind::Trianglefans));
    }
}
