//! UV-seam texel grading (user decision 2026-10-29): the sphere-map rim rule extended to pixels
//! on a texture-coordinate seam. Which pixels lie on the seam is decided from the submitted
//! geometry: seam vertices share their position with a vertex of different texture coordinates.
use super::grading::UnexplainedPixel;
use super::sphere_map_rim::grade_sphere_map_rim;

/// A projected seam edge in top-down pixel coordinates (pixel centres at `i + 0.5`).
pub type SeamSegment = [[f64; 2]; 2];

/// How far (in pixels) a pixel centre may lie from a seam segment to count as on it: a
/// one-pixel reach covers the pixels whose footprint the segment crosses.
pub const SEAM_REACH: f64 = 1.;

/// Positions closer than this (model units) are one position: the probes compute shared seam
/// positions from angles 0 and 2*pi, which differ in the last bits.
const SHARED_POSITION: f64 = 1e-9;

/// A drawn triangle mesh as submitted: per vertex its projected top-down pixel, model-space
/// position and UV; index triples wound outward (counter-clockwise seen from outside); the eye.
pub struct SeamMesh {
    pub pixels: Vec<[f64; 2]>,
    pub positions: Vec<[f64; 3]>,
    pub uvs: Vec<[f64; 2]>,
    pub triangles: Vec<[usize; 3]>,
    pub eye: [f64; 3],
}

fn difference(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

impl SeamMesh {
    /// Vertices sharing their position with a vertex of different texture coordinates.
    fn seam_vertices(&self) -> Vec<bool> {
        let shares = |i: usize, j: usize| {
            let d = difference(self.positions[i], self.positions[j]);
            d.iter().all(|c| c.abs() <= SHARED_POSITION) && self.uvs[i] != self.uvs[j]
        };
        let count = self.positions.len();
        (0..count)
            .map(|i| (0..count).any(|j| shares(i, j)))
            .collect()
    }

    fn faces_eye(&self, [a, b, c]: [usize; 3]) -> bool {
        let origin = self.positions[a];
        let u = difference(self.positions[b], origin);
        let v = difference(self.positions[c], origin);
        let normal = [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ];
        let to_eye = difference(self.eye, origin);
        normal.iter().zip(to_eye).map(|(n, e)| n * e).sum::<f64>() > 0.
    }

    fn segment(&self, [a, b]: [usize; 2]) -> SeamSegment {
        [self.pixels[a], self.pixels[b]]
    }
}

fn edges([a, b, c]: [usize; 3]) -> [[usize; 2]; 3] {
    [[a, b], [b, c], [c, a]].map(|[p, q]| [p.min(q), p.max(q)])
}

/// The projected seam: edges of front-facing triangles joining two seam vertices, and the
/// silhouette edges (shared with a front-facing triangle) of back-facing triangles that touch a
/// seam vertex, which the rasterizer may show at the outline. The outline clause is the scope
/// the user approved on 2026-10-29 (outline edges of triangles sharing a seam vertex);
/// `seam_segments(&mesh)`.
pub fn seam_segments(mesh: &SeamMesh) -> Vec<SeamSegment> {
    let seam = mesh.seam_vertices();
    let (front, back): (Vec<[usize; 3]>, Vec<[usize; 3]>) = mesh
        .triangles
        .iter()
        .partition(|&&triangle| mesh.faces_eye(triangle));
    let front_edges: Vec<[usize; 2]> = front.iter().flat_map(|t| edges(*t)).collect();
    let on_seam = front_edges.iter().filter(|[a, b]| seam[*a] && seam[*b]);
    let silhouettes = back
        .iter()
        .filter(|t| t.iter().any(|&v| seam[v]))
        .flat_map(|t| edges(*t))
        .filter(|edge| front_edges.contains(edge));
    let chosen: Vec<[usize; 2]> = on_seam.copied().chain(silhouettes).collect();
    chosen.into_iter().map(|edge| mesh.segment(edge)).collect()
}

fn segment_distance(segment: &SeamSegment, point: [f64; 2]) -> f64 {
    let [a, b] = *segment;
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let length2 = dx * dx + dy * dy;
    let t = match length2 > 0. {
        true => (((point[0] - a[0]) * dx + (point[1] - a[1]) * dy) / length2).clamp(0., 1.),
        false => 0.,
    };
    let (cx, cy) = (a[0] + t * dx - point[0], a[1] + t * dy - point[1]);
    (cx * cx + cy * cy).sqrt()
}

/// Whether the centre of pixel `(x, y)` lies within `SEAM_REACH` of a seam segment;
/// `on_seam(&segments, 163, 94)`.
pub fn on_seam(segments: &[SeamSegment], x: usize, y: usize) -> bool {
    let centre = [x as f64 + 0.5, y as f64 + 0.5];
    segments
        .iter()
        .any(|segment| segment_distance(segment, centre) <= SEAM_REACH)
}

/// UV-seam grading (lavapipe): the unexplained pixels beyond one level that lie on the seam pass
/// when the bottom-up control equals native at every one of them, each is covered in both dumps
/// and the coverage masks are identical (the sphere-map rim proof with a seam predicate). Without
/// a control (hardware) nothing is graded. Returns the pixels left and the count graded;
/// `grade_uv_seam(pixels, &rust, native, control, width, &segments)`.
pub fn grade_uv_seam(
    pixels: Vec<UnexplainedPixel>,
    rust: &[u8],
    native: &[u8],
    control: Option<&[u8]>,
    width: usize,
    segments: &[SeamSegment],
) -> (Vec<UnexplainedPixel>, usize) {
    let seam = |x, y| on_seam(segments, x, y);
    grade_sphere_map_rim(pixels, rust, native, control, width, seam)
}

#[cfg(test)]
#[path = "uv_seam_tests.rs"]
mod tests;
