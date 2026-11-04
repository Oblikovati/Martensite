//! Lower supported OpenGL surface assembly to Vulkan triangle lists.
use crate::{RenderError, error::rejected};
use martensite_tkservice::graphic3d::Graphic3dTypeOfPrimitiveArray;
pub(crate) fn triangle_indices(
    kind: Graphic3dTypeOfPrimitiveArray,
    indices: &[u32],
) -> Result<Vec<u32>, RenderError> {
    use Graphic3dTypeOfPrimitiveArray::{
        Polygons, Quadrangles, Quadranglestrips, TriangleStripAdjacency, Trianglefans, Triangles,
        TrianglesAdjacency, Trianglestrips,
    };
    match kind {
        // OpenGL ignores incomplete primitives at the end of each draw, including each bound.
        Triangles => Ok(indices[..indices.len() / 3 * 3].to_vec()),
        Quadrangles => Ok(quad_triangles(indices)),
        Quadranglestrips => Ok(quad_strip_triangles(indices)),
        Trianglestrips => Ok(strip_triangles(indices)),
        Trianglefans | Polygons => Ok(fan_triangles(indices)),
        TrianglesAdjacency => Ok(adjacency_triangles(indices)),
        TriangleStripAdjacency => Ok(adjacency_strip_triangles(indices)),
        _ => Err(rejected(
            kind,
            "triangles, triangle strips, triangle fans, convex polygons, quadrangles, quadrangle strips or triangle adjacency",
        )),
    }
}
// GlPrimitiveArray draws the adjacency kinds as GL_*_ADJACENCY with the face program and no
// geometry shader, so GL rasterizes only the inner primitive of each group (OpenGL 4.6 section
// 10.1.12-15); the recorded adjacency_arrays capture witnesses it on both drivers.
fn adjacency_triangles(indices: &[u32]) -> Vec<u32> {
    indices
        .as_chunks::<6>()
        .0
        .iter()
        .flat_map(|group| [group[0], group[2], group[4]])
        .collect()
}
/// Triangle `i` of a strip with adjacency joins vertices 2i, 2i+2, 2i+4, the first two swapped
/// for odd `i`; `n` vertices make `(n - 4) / 2` triangles.
fn adjacency_strip_triangles(indices: &[u32]) -> Vec<u32> {
    let count = indices.len().saturating_sub(4) / 2;
    (0..count)
        .flat_map(|i| {
            let [a, b, c] = [indices[2 * i], indices[2 * i + 2], indices[2 * i + 4]];
            if i.is_multiple_of(2) {
                [a, b, c]
            } else {
                [b, a, c]
            }
        })
        .collect()
}
fn strip_triangles(indices: &[u32]) -> Vec<u32> {
    indices
        .windows(3)
        .enumerate()
        .flat_map(|(index, triple)| {
            if index.is_multiple_of(2) {
                [triple[0], triple[1], triple[2]]
            } else {
                [triple[1], triple[0], triple[2]]
            }
        })
        .collect()
}
fn fan_triangles(indices: &[u32]) -> Vec<u32> {
    let Some((&center, rim)) = indices.split_first() else {
        return vec![];
    };
    rim.windows(2)
        .flat_map(|edge| [center, edge[0], edge[1]])
        .collect()
}

pub(crate) fn line_indices(
    kind: Graphic3dTypeOfPrimitiveArray,
    indices: &[u32],
) -> Result<Vec<u32>, RenderError> {
    use Graphic3dTypeOfPrimitiveArray::{LineStripAdjacency, LinesAdjacency, Polylines, Segments};
    match kind {
        Segments => Ok(indices[..indices.len() / 2 * 2].to_vec()),
        Polylines => Ok(strip_segments(indices)),
        LinesAdjacency => Ok(indices
            .as_chunks::<4>()
            .0
            .iter()
            .flat_map(|group| [group[1], group[2]])
            .collect()),
        // The first and last vertices of a strip with adjacency only bound the inner segments.
        LineStripAdjacency => Ok(match indices.len() {
            0..4 => vec![],
            count => strip_segments(&indices[1..count - 1]),
        }),
        _ => Err(rejected(kind, "segments, polylines or line adjacency")),
    }
}
fn strip_segments(indices: &[u32]) -> Vec<u32> {
    indices
        .windows(2)
        .flat_map(|pair| [pair[0], pair[1]])
        .collect()
}

// Paired colored captures expose the native GL_QUADS 0-to-2 diagonal.
fn quad_triangles(indices: &[u32]) -> Vec<u32> {
    indices
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|q| [q[0], q[1], q[2], q[0], q[2], q[3]])
        .collect()
}
fn quad_strip_triangles(indices: &[u32]) -> Vec<u32> {
    indices
        .windows(4)
        .step_by(2)
        .flat_map(|q| [q[0], q[1], q[3], q[2], q[0], q[3]])
        .collect()
}

#[cfg(test)]
#[path = "primitive_topology_tests.rs"]
mod tests;
