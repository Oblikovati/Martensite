//! Cross-driver shadow-sample flips (D2 cross-driver bound, the design notes;
//! coordinator decision 2026-10-28, stated here for review).
//!
//! LightShadow.glsl decides each of its 16 Poisson taps by `(aCurrentDepth - aBias) >
//! aClosestDepth`. The map renders into a GL_DEPTH24_STENCIL8 attachment
//! (GlView `myFboDepthFormat`), so the stored value is the light-space window depth rounded
//! to a unit of 1 / (2^24 - 1); the Vulkan map keeps the same depth unrounded (D32_SFLOAT), and
//! each driver interpolates the fragment's light-space position with its own rounding. Where the
//! fragment's light-space depth after bias lies within one such unit of the depth stored at a
//! tap's texel, both drivers legitimately decide that tap either way.
//!
//! Bound: a differing pixel is admitted only when, at the surface point under its centre, some
//! casting light has a tap whose margin `(depth - bias) - stored` is at most one unit in
//! magnitude. Surface points, light-space depths and stored depths are computed in f64 from the
//! submitted planar faces, the view camera and the light matrices the fragments sampled with,
//! never from the images; the bound is not fitted to the measured pixels.

/// One unit of the native 24-bit shadow-map depth.
pub const DEPTH_UNIT: f64 = 1. / 16_777_215.;

/// `occPoissonDisk16` of LightShadow.glsl.
pub const POISSON_DISK: [[f64; 2]; 16] = [
    [-0.94201624, -0.39906216],
    [0.94558609, -0.76890725],
    [-0.09418410, -0.92938870],
    [0.34495938, 0.29387760],
    [-0.91588581, 0.45771432],
    [-0.81544232, -0.87912464],
    [-0.38277543, 0.27676845],
    [0.97484398, 0.75648379],
    [0.44323325, -0.97511554],
    [0.53742981, -0.47373420],
    [-0.26496911, -0.41893023],
    [0.79197514, 0.19090188],
    [-0.24188840, 0.99706507],
    [-0.81409955, 0.91437590],
    [0.19984126, 0.78641367],
    [0.14383161, -0.14100790],
];

/// A submitted planar convex face: corners in order and the normal its vertices carry.
#[derive(Clone, Copy, Debug)]
pub struct FlatFace {
    pub corners: [[f64; 3]; 4],
    pub normal: [f64; 3],
}

/// The six outward faces of the axis-aligned box `min..max` (BRepPrimAPIMakeBox).
pub fn box_faces(min: [f64; 3], max: [f64; 3]) -> Vec<FlatFace> {
    let corner = |bits: [usize; 3]| [0, 1, 2].map(|axis| [min, max][bits[axis]][axis]);
    (0..3)
        .flat_map(|axis| {
            [0, 1].map(|side| {
                let (u, v) = ((axis + 1) % 3, (axis + 2) % 3);
                let at = |a: usize, b: usize| {
                    let mut bits = [0; 3];
                    (bits[axis], bits[u], bits[v]) = (side, a, b);
                    corner(bits)
                };
                let mut normal = [0.; 3];
                normal[axis] = if side == 1 { 1. } else { -1. };
                let corners = [at(0, 0), at(1, 0), at(1, 1), at(0, 1)];
                FlatFace { corners, normal }
            })
        })
        .collect()
}

/// A casting light as the fragments see it: its column-major world-to-clip matrix and
/// `occLight_Position` (direction towards a directional light, position of a spot light).
#[derive(Clone, Copy, Debug)]
pub struct FlipLight {
    pub matrix: [f64; 16],
    pub position: [f64; 3],
}

/// A light's map as the renderer stored it (bottom row first), to check the geometric depths.
#[derive(Clone, Debug, Default)]
pub struct RenderedMap {
    pub depth: Vec<f32>,
}

/// The scene of one dump: faces, the view's NDC-to-world matrix and depth range, the window
/// size, and the shadow maps' lights, resolution and bias.
#[derive(Clone, Debug)]
pub struct ShadowFlipScene {
    pub faces: Vec<FlatFace>,
    pub view_inverse: [f64; 16],
    pub view_depth_range: [f64; 2],
    pub size: (usize, usize),
    pub lights: Vec<FlipLight>,
    pub rendered: Vec<RenderedMap>,
    pub resolution: usize,
    pub bias: f64,
    /// The polygon offset factor of the shaded faces (Graphic3dPolygonOffset::Factor).
    pub slope_factor: f64,
    /// The view pass's submitted triangles, window pixels with rows top down (the recorded
    /// triangulation of the view-snap rule, support/shadow_view_snap.rs).
    pub view_triangles: Vec<[[f64; 2]; 3]>,
}

fn transform(matrix: &[f64; 16], point: [f64; 4]) -> [f64; 4] {
    let row = |r: usize| (0..4).map(|c| matrix[4 * c + r] * point[c]).sum::<f64>();
    [row(0), row(1), row(2), row(3)]
}

pub fn unproject(inverse: &[f64; 16], ndc: [f64; 3]) -> [f64; 3] {
    let [x, y, z, w] = transform(inverse, [ndc[0], ndc[1], ndc[2], 1.]);
    [x / w, y / w, z / w]
}

pub fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

impl FlatFace {
    fn plane(&self) -> [f64; 3] {
        cross(
            sub(self.corners[1], self.corners[0]),
            sub(self.corners[2], self.corners[0]),
        )
    }
    /// The line parameter where `from -> to` crosses the face's plane, if it is not parallel.
    fn plane_crossing(&self, from: [f64; 3], to: [f64; 3]) -> Option<f64> {
        let plane = self.plane();
        let along = dot(plane, sub(to, from));
        (along != 0.).then(|| dot(plane, sub(self.corners[0], from)) / along)
    }
    /// The segment parameter in `0..=1` where `from -> to` crosses this face, if it does.
    fn crossing(&self, from: [f64; 3], to: [f64; 3]) -> Option<f64> {
        let t = self.plane_crossing(from, to)?;
        let point = lerp(from, to, t);
        ((0. ..=1.).contains(&t) && self.contains(point, self.plane())).then_some(t)
    }
    /// Whether `point` of the face plane lies inside the convex outline (boundary included).
    fn contains(&self, point: [f64; 3], plane: [f64; 3]) -> bool {
        (0..4).all(|i| {
            let (a, b) = (self.corners[i], self.corners[(i + 1) % 4]);
            dot(cross(sub(b, a), sub(point, a)), plane) >= 0.
        })
    }
}

/// The nearest crossing of `from -> to` with `faces`: parameter and face.
pub fn nearest(faces: &[FlatFace], from: [f64; 3], to: [f64; 3]) -> Option<(f64, FlatFace)> {
    faces
        .iter()
        .filter_map(|face| face.crossing(from, to).map(|t| (t, *face)))
        .min_by(|a, b| a.0.total_cmp(&b.0))
}

fn lerp(from: [f64; 3], to: [f64; 3], t: f64) -> [f64; 3] {
    [0, 1, 2].map(|i| from[i] + t * (to[i] - from[i]))
}

/// The light-space window coordinates `xyz / w * 0.5 + 0.5` of a world point.
pub fn light_window(light: &FlipLight, point: [f64; 3]) -> [f64; 3] {
    let [x, y, z, w] = transform(&light.matrix, [point[0], point[1], point[2], 1.]);
    [x, y, z].map(|value| value / w * 0.5 + 0.5)
}

impl ShadowFlipScene {
    /// The surface point and face under the centre of window pixel (column, row), rows top down.
    pub fn surface(&self, column: usize, row: usize) -> Option<([f64; 3], FlatFace)> {
        self.surface_at([column as f64 + 0.5, row as f64 + 0.5])
    }
    /// The surface point and face under window position `at` (pixels, rows top down).
    pub fn surface_at(&self, at: [f64; 2]) -> Option<([f64; 3], FlatFace)> {
        let (w, h) = (self.size.0 as f64, self.size.1 as f64);
        let x = at[0] / w * 2. - 1.;
        let y = 1. - at[1] / h * 2.;
        let [near, far] = self.view_depth_range;
        let from = unproject(&self.view_inverse, [x, y, near]);
        let to = unproject(&self.view_inverse, [x, y, far]);
        nearest(&self.faces, from, to).map(|(t, face)| (lerp(from, to, t), face))
    }
    /// The window depth the map stores at texel (column, row), rows bottom up: the nearest face
    /// along the texel centre's light ray raised by the shaded faces' polygon offset
    /// (`slope_factor` times the larger texel step of the face depth, Graphic3dPolygonOffset
    /// with zero units), or the clear depth 1.
    pub fn stored_depth(&self, light: &FlipLight, column: usize, row: usize) -> f64 {
        let size = self.resolution as f64;
        let x = (column as f64 + 0.5) / size * 2. - 1.;
        let y = (row as f64 + 0.5) / size * 2. - 1.;
        let inverse = invert(&light.matrix);
        let ray = |x: f64, y: f64| {
            (
                unproject(&inverse, [x, y, -1.]),
                unproject(&inverse, [x, y, 1.]),
            )
        };
        let (from, to) = ray(x, y);
        let Some((_, face)) = nearest(&self.faces, from, to) else {
            return 1.;
        };
        let depth_at = |(from, to): ([f64; 3], [f64; 3])| {
            let t = face.plane_crossing(from, to).unwrap_or(f64::NAN);
            light_window(light, lerp(from, to, t))[2]
        };
        let depth = depth_at((from, to));
        let step = 2. / size;
        let slope = (depth_at(ray(x + step, y)) - depth)
            .abs()
            .max((depth_at(ray(x, y + step)) - depth).abs());
        (depth + self.slope_factor * slope).min(1.)
    }
    /// The margins `(depth - bias) - stored`, in depth units, of the 16 taps of `light` at a
    /// surface point; empty outside the light frustum, where LightShadow.glsl takes no tap.
    pub fn tap_margins(&self, light: &FlipLight, point: [f64; 3], normal: [f64; 3]) -> Vec<f64> {
        let [u, v, depth] = light_window(light, point);
        if !(0. ..=1.).contains(&u) || !(0. ..=1.).contains(&v) || depth > 1. {
            return Vec::new();
        }
        let bias = (self.bias * (1. - dot(normal, light.position))).max(self.bias * 0.1);
        let texel = 1. / self.resolution as f64;
        let last = self.resolution - 1;
        let index = |coordinate: f64| ((coordinate / texel).floor().max(0.) as usize).min(last);
        POISSON_DISK
            .iter()
            .map(|tap| {
                let (column, row) = (index(u + tap[0] * texel), index(v + tap[1] * texel));
                let stored = self.stored_depth(light, column, row);
                ((depth - bias) - stored) / DEPTH_UNIT
            })
            .collect()
    }
    /// Diagnostic: the smallest distance, in texels, from a tap coordinate to a texel border
    /// across which the tap's decision changes (a nearest-texel selection tie), at the surface
    /// under pixel (column, row); `None` when no tap has such a border.
    pub fn closest_selection_tie(&self, column: usize, row: usize) -> Option<f64> {
        let (point, face) = self.surface(column, row)?;
        self.lights
            .iter()
            .flat_map(|light| self.selection_ties(light, point, face.normal))
            .min_by(f64::total_cmp)
    }
    fn selection_ties(&self, light: &FlipLight, point: [f64; 3], normal: [f64; 3]) -> Vec<f64> {
        let [u, v, depth] = light_window(light, point);
        if !(0. ..=1.).contains(&u) || !(0. ..=1.).contains(&v) || depth > 1. {
            return Vec::new();
        }
        let bias = (self.bias * (1. - dot(normal, light.position))).max(self.bias * 0.1);
        let size = self.resolution as f64;
        let last = self.resolution as i64 - 1;
        let shadowed = |texel: [i64; 2]| {
            let [c, r] = texel.map(|t| t.clamp(0, last) as usize);
            depth - bias > self.stored_depth(light, c, r)
        };
        let mut ties = Vec::new();
        for tap in POISSON_DISK {
            let coord = [(u + tap[0] / size) * size, (v + tap[1] / size) * size];
            let texel = coord.map(|c| c.floor() as i64);
            for axis in 0..2 {
                let border = coord[axis].round();
                let mut other = texel;
                other[axis] = if border > coord[axis] {
                    texel[axis] + 1
                } else {
                    texel[axis] - 1
                };
                if shadowed(texel) != shadowed(other) {
                    ties.push((coord[axis] - border).abs());
                }
            }
        }
        ties
    }
    /// The window position (top-down pixels) of a world point under the view camera.
    pub fn window(&self, point: [f64; 3]) -> [f64; 2] {
        let forward = invert(&self.view_inverse);
        let [x, y, _, w] = transform(&forward, [point[0], point[1], point[2], 1.]);
        let (w_px, h_px) = (self.size.0 as f64, self.size.1 as f64);
        [(x / w * 0.5 + 0.5) * w_px, (0.5 - y / w * 0.5) * h_px]
    }
    /// The smallest offset, in pixels along the edge's minor axis, from the centre of pixel
    /// (column, row) to a projected face edge whose span covers the pixel (the snap bound of
    /// support/snapped_tie_grade.rs applies to fill edges as to lines).
    pub fn edge_offset(&self, column: usize, row: usize) -> f64 {
        let (x, y) = (column as f64 + 0.5, row as f64 + 0.5);
        let edges = self.faces.iter().flat_map(|face| {
            (0..4).map(|i| {
                (
                    self.window(face.corners[i]),
                    self.window(face.corners[(i + 1) % 4]),
                )
            })
        });
        edges
            .filter_map(|(from, to)| {
                let (dx, dy) = (to[0] - from[0], to[1] - from[1]);
                let major = if dx.abs() >= dy.abs() { 0 } else { 1 };
                let along = [x, y][major];
                let (low, high) = (from[major].min(to[major]), from[major].max(to[major]));
                let inside = along >= low - 1. && along <= high + 1.;
                let offset =
                    (dy * (x - from[0]) - dx * (y - from[1])).abs() / dx.abs().max(dy.abs());
                inside.then_some(offset)
            })
            .min_by(f64::total_cmp)
            .unwrap_or(f64::MAX)
    }
    /// The largest difference, in depth units, between the geometric and the rendered stored
    /// depth over the taps at pixel (column, row): the reference's own check of its geometry.
    pub fn rendered_disagreement(&self, column: usize, row: usize) -> Option<f64> {
        let (point, _) = self.surface(column, row)?;
        let size = self.resolution as f64;
        let pairs = self.lights.iter().zip(&self.rendered);
        let differences = pairs.flat_map(|(light, map)| {
            let [u, v, _] = light_window(light, point);
            POISSON_DISK.iter().map(move |tap| {
                let index = |c: f64| ((c * size).floor().max(0.) as usize).min(self.resolution - 1);
                let (c, r) = (index(u + tap[0] / size), index(v + tap[1] / size));
                let stored = f64::from(map.depth[r * self.resolution + c]);
                (stored - self.stored_depth(light, c, r)).abs() / DEPTH_UNIT
            })
        });
        differences.max_by(f64::total_cmp)
    }
    /// The smallest tap margin magnitude, in depth units, over every casting light at the
    /// surface under pixel (column, row); `None` off the surface or outside every frustum.
    pub fn closest_margin(&self, column: usize, row: usize) -> Option<f64> {
        let (point, face) = self.surface(column, row)?;
        self.lights
            .iter()
            .flat_map(|light| self.tap_margins(light, point, face.normal))
            .map(f64::abs)
            .min_by(f64::total_cmp)
    }
    /// Whether pixel (column, row) is a proven texel-selection flip (coordinator decision
    /// 2026-10-28, cross-driver only): some tap coordinate lies within `precision` texels of a
    /// texel border across which its decision changes, where `precision` is the sampler's
    /// sub-texel precision `2^-subTexelPrecisionBits`; either texel may be selected there.
    /// Example: `scene.admits_selection_flip(x, y, 1. / 256.)`.
    pub fn admits_selection_flip(&self, column: usize, row: usize, precision: f64) -> bool {
        self.closest_selection_tie(column, row)
            .is_some_and(|tie| tie <= precision)
    }
    /// Whether pixel (column, row) is a proven tap flip: some margin within one depth unit.
    pub fn admits_flip(&self, column: usize, row: usize) -> bool {
        self.closest_margin(column, row)
            .is_some_and(|margin| margin <= 1.)
    }
}

/// The inverse of a column-major 4x4 matrix (Gauss-Jordan with partial pivoting).
pub fn invert(matrix: &[f64; 16]) -> [f64; 16] {
    let mut rows: Vec<[f64; 8]> = (0..4)
        .map(|r| {
            let mut row = [0.; 8];
            (0..4).for_each(|c| row[c] = matrix[4 * c + r]);
            row[4 + r] = 1.;
            row
        })
        .collect();
    for column in 0..4 {
        eliminate(&mut rows, column);
    }
    let mut inverse = [0.; 16];
    (0..16).for_each(|i| inverse[i] = rows[i % 4][4 + i / 4]);
    inverse
}

fn eliminate(rows: &mut [[f64; 8]], column: usize) {
    let pivot = (column..4)
        .max_by(|&a, &b| rows[a][column].abs().total_cmp(&rows[b][column].abs()))
        .unwrap();
    rows.swap(column, pivot);
    let lead = rows[column][column];
    assert!(
        lead != 0.,
        "expected an invertible matrix, column {column} is singular"
    );
    rows[column].iter_mut().for_each(|value| *value /= lead);
    let source = rows[column];
    for (index, row) in rows.iter_mut().enumerate() {
        let factor = row[column];
        if index != column && factor != 0. {
            row.iter_mut()
                .zip(source)
                .for_each(|(value, s)| *value -= factor * s);
        }
    }
}
