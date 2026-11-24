//! Object-space meshlet bounds for conservative raster culling (#594).
use crate::RenderVertex;
use martensite_tkernel::ncollection::Vec3;

#[derive(Clone, Copy, Debug)]
pub(super) struct MeshletRange {
    pub(super) first_vertex: usize,
    pub(super) vertex_count: usize,
    pub(super) first_triangle: usize,
    pub(super) triangle_count: usize,
    pub(super) minimum: [f32; 3],
    pub(super) maximum: [f32; 3],
    /// The cone axis and conservative tangent of its half-angle. A missing cone requires
    /// all triangles to reach the single-polygon raster culling stage.
    pub(super) cone: Option<([f32; 3], f32)>,
}

impl MeshletRange {
    pub(super) fn capture(
        (first_vertex, first_triangle): (usize, usize),
        vertices: &[u32],
        triangles: &[[u8; 3]],
        geometry: &[RenderVertex],
    ) -> Self {
        let (minimum, maximum) = meshlet_bounds(vertices, geometry);
        Self {
            first_vertex,
            vertex_count: vertices.len(),
            first_triangle,
            triangle_count: triangles.len(),
            minimum,
            maximum,
            cone: backface_cone(vertices, triangles, geometry),
        }
    }
}

fn meshlet_bounds(vertices: &[u32], geometry: &[RenderVertex]) -> ([f32; 3], [f32; 3]) {
    let mut minimum = [f32::INFINITY; 3];
    let mut maximum = [f32::NEG_INFINITY; 3];
    for &index in vertices {
        let position = geometry[index as usize].position;
        for axis in 0..3 {
            minimum[axis] = minimum[axis].min(position[axis]);
            maximum[axis] = maximum[axis].max(position[axis]);
        }
    }
    (minimum, maximum)
}

fn backface_cone(
    vertices: &[u32],
    triangles: &[[u8; 3]],
    geometry: &[RenderVertex],
) -> Option<([f32; 3], f32)> {
    let position = |index: u8| {
        let point = geometry[vertices[index as usize] as usize].position;
        let [x, y, z] = point.map(f64::from);
        Vec3::new(x, y, z)
    };
    let normals: Vec<_> = triangles
        .iter()
        .filter_map(|triangle| {
            let [a, b, c] = triangle.map(position);
            normalized(Vec3::cross(&(b - a), &(c - a)))
        })
        .collect();
    let zero = Vec3::new(0., 0., 0.);
    let sum = normals.iter().fold(zero, |total, normal| total + *normal);
    let axis = normalized(sum)?;
    cone_tangent(axis, &normals)
}

fn normalized(vector: Vec3<f64>) -> Option<Vec3<f64>> {
    let length = vector.dot(&vector).sqrt();
    (length > 0. && length.is_finite()).then(|| vector / length)
}

fn cone_tangent(axis: Vec3<f64>, normals: &[Vec3<f64>]) -> Option<([f32; 3], f32)> {
    let cos = normals
        .iter()
        .fold(1_f64, |least, normal| least.min(axis.dot(normal)));
    // Enlarge the cone for binary32 storage and retain the meshlet if it spans a hemisphere.
    let cos = cos - 8. * f64::from(f32::EPSILON);
    if cos <= 0.25 {
        return None;
    }
    let tangent = (1. - cos * cos).sqrt() / cos;
    Some((
        [axis.x(), axis.y(), axis.z()].map(|value| value as f32),
        (tangent + 1e-6) as f32,
    ))
}
