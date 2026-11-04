//! Mesh-edge grid model (user decision 2026-10-28, rule A; AMD only):
//! the recorded controls
//!
//! The mesh-edge geometry stage (`surface.edges.geom`) gives each vertex the altitude of its
//! opposite edge from unsnapped window positions, clamped to at least `10 * width`, and the
//! fragment blends the edge colour by `smoothstep(width - feather / 2, width + feather / 2, d)`
//! with `d` the smallest interpolated altitude. The rasterizer interpolates with barycentrics of
//! the triangle it snapped: RADV to 2^-8 px, radeonsi to 2^-12 px (viewports up to 1024 px,
//! Mesa 26.1.6). A differing pixel passes when this model on RADV's grid over the RADV
//! edge-free capture predicts the RADV pixel within one level, and on radeonsi's grid over the
//! native edge-free capture predicts the native pixel within one level. The blend is linear:
//! the view attachment is sRGB.

/// RADV's subpixel grid, in fractional bits.
pub const RADV_GRID_BITS: i32 = 8;
/// radeonsi's subpixel grid for viewports up to 1024 px, in fractional bits.
pub const RADEONSI_GRID_BITS: i32 = 12;
/// The geometry stage's minimum altitude, in edge widths (`10.0 * occLineWidth`).
const ALTITUDE_CLAMP: f64 = 10.;

/// The edge program's parameters and the submitted triangles (window pixels, rows top down).
pub struct MeshEdgeScene<'a> {
    pub triangles: &'a [[[f64; 2]; 3]],
    pub size: (usize, usize),
    /// Linear RGB edge colour.
    pub edge_colour: [f64; 3],
    pub width: f64,
    pub feather: f64,
}

/// `vertex` on the 2^-bits grid of OpenGL's y-up window frame of height `height`.
fn snapped(vertex: [f64; 2], bits: i32, height: f64) -> [f64; 2] {
    let scale = martensite_cmath::powi(2f64, bits);
    let x = (vertex[0] * scale).round() / scale;
    [x, height - ((height - vertex[1]) * scale).round() / scale]
}

fn barycentric(t: [[f64; 2]; 3], p: [f64; 2]) -> [f64; 3] {
    let [[x0, y0], [x1, y1], [x2, y2]] = t;
    let det = (x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0);
    let l1 = ((p[0] - x0) * (y2 - y0) - (x2 - x0) * (p[1] - y0)) / det;
    let l2 = ((x1 - x0) * (p[1] - y0) - (p[0] - x0) * (y1 - y0)) / det;
    [1. - l1 - l2, l1, l2]
}

/// Counter-clockwise in OpenGL's y-up frame, clockwise in these top-down rows.
fn front_facing(t: &[[f64; 2]; 3]) -> bool {
    let [[x0, y0], [x1, y1], [x2, y2]] = *t;
    (x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0) < 0.
}

/// The clamped altitudes of the geometry stage, opposite each vertex.
fn clamped_altitudes(t: &[[f64; 2]; 3], width: f64) -> [f64; 3] {
    let side = |a: usize, b: usize| martensite_cmath::hypot(t[b][0] - t[a][0], t[b][1] - t[a][1]);
    let [[x0, y0], [x1, y1], [x2, y2]] = *t;
    let area = ((x2 - x0) * (y1 - y0) - (y2 - y0) * (x1 - x0)).abs();
    [side(1, 2), side(0, 2), side(0, 1)].map(|length| (area / length).max(ALTITUDE_CLAMP * width))
}

fn decode(value: u8) -> f64 {
    let c = f64::from(value) / 255.;
    if c <= 0.04045 {
        c / 12.92
    } else {
        martensite_cmath::pow((c + 0.055) / 1.055, 2.4)
    }
}

fn encode(value: f64) -> u8 {
    let c = if value <= 0.0031308 {
        value * 12.92
    } else {
        1.055 * martensite_cmath::pow(value, 1. / 2.4) - 0.055
    };
    (c * 255. + 0.5).floor().clamp(0., 255.) as u8
}

fn smoothstep(low: f64, high: f64, x: f64) -> f64 {
    let t = ((x - low) / (high - low)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

impl MeshEdgeScene<'_> {
    /// The smallest interpolated altitude at the centre of pixel (x, y) on the 2^-bits grid:
    /// the first front-facing triangle whose snapped outline holds the centre.
    pub fn edge_distance(&self, (x, y): (usize, usize), bits: i32) -> Option<f64> {
        let centre = [x as f64 + 0.5, y as f64 + 0.5];
        let height = self.size.1 as f64;
        self.triangles
            .iter()
            .filter(|t| front_facing(t))
            .find_map(|t| {
                let weights = barycentric(t.map(|v| snapped(v, bits, height)), centre);
                (weights.iter().all(|w| *w >= 0.)).then(|| {
                    let altitudes = clamped_altitudes(t, self.width);
                    (0..3)
                        .map(|k| altitudes[k] * weights[k])
                        .fold(f64::MAX, f64::min)
                })
            })
    }

    /// The predicted pixel over the edge-free `face` colour, on the 2^-bits grid.
    pub fn predict(&self, pixel: (usize, usize), face: [u8; 3], bits: i32) -> Option<[u8; 3]> {
        let distance = self.edge_distance(pixel, bits)?;
        let half = self.feather * 0.5;
        let s = smoothstep(self.width - half, self.width + half, distance);
        Some([0, 1, 2].map(|k| encode(self.edge_colour[k] * (1. - s) + decode(face[k]) * s)))
    }

    /// Rule A at one differing pixel: each driver's grid predicts its own image within one
    /// level over its own edge-free capture.
    pub fn explains(
        &self,
        pixel: (usize, usize),
        radv: [[u8; 3]; 2],
        native: [[u8; 3]; 2],
    ) -> bool {
        let within = |predicted: Option<[u8; 3]>, actual: [u8; 3]| {
            predicted.is_some_and(|p| p.iter().zip(actual).all(|(a, b)| a.abs_diff(b) <= 1))
        };
        // radv and native are [edged image pixel, edge-free capture pixel].
        within(self.predict(pixel, radv[1], RADV_GRID_BITS), radv[0])
            && within(
                self.predict(pixel, native[1], RADEONSI_GRID_BITS),
                native[0],
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A thin front-facing triangle along row 10 (altitude 0.5 px, clamped to 10) and a fat one.
    const THIN: [[[f64; 2]; 3]; 1] = [[[0., 10.], [10., 10.5], [20., 10.]]];

    fn scene(triangles: &[[[f64; 2]; 3]]) -> MeshEdgeScene<'_> {
        MeshEdgeScene {
            triangles,
            size: (32, 32),
            edge_colour: [1., 0., 0.],
            width: 1.,
            feather: 1.,
        }
    }

    #[test]
    fn grids_round_vertices_in_the_y_up_frame() {
        assert_eq!(snapped([1.001, 2.001], 8, 4.), [1., 2.]);
        assert!(front_facing(&[[0., 0.], [0., 4.], [4., 0.]]));
        assert!(!front_facing(&[[0., 0.], [4., 0.], [0., 4.]]));
    }

    #[test]
    fn the_clamp_scales_the_distance_to_a_thin_triangles_edge() {
        let thin = [[0., 0.], [4., 0.], [0., 0.5]];
        let altitudes = clamped_altitudes(&thin, 1.);
        assert_eq!(altitudes[2], 10.);
        let fat = clamped_altitudes(&[[0., 0.], [40., 0.], [0., 40.]], 1.);
        assert!((fat[2] - 40.).abs() < 1e-12, "{fat:?}");
    }

    #[test]
    fn a_prediction_off_by_two_levels_is_rejected() {
        let edges = scene(&THIN);
        let triangles = [[[0., 0.], [0., 32.], [32., 0.]]];
        let fat = scene(&triangles);
        let face = [100, 100, 100];
        let predicted = fat.predict((4, 4), face, RADV_GRID_BITS).unwrap();
        let off = [predicted[0], predicted[1].saturating_add(2), predicted[2]];
        assert!(fat.explains((4, 4), [predicted, face], [predicted, face]));
        assert!(!fat.explains((4, 4), [off, face], [predicted, face]));
        assert!(!fat.explains((4, 4), [predicted, face], [off, face]));
        assert!(edges.predict((30, 30), face, RADV_GRID_BITS).is_none());
    }

    #[test]
    fn swapped_grids_do_not_explain_a_snap_dependent_edge_pixel() {
        // A triangle 0.5 px high whose base passes 0.0485 px above row 10's centre: the clamp
        // makes the distance 10 / 0.5 times the snapped offset, and RADV's grid moves the base
        // 0.0017 px further than radeonsi's.
        let thin = [[[0.3, 10.4515], [10.1, 10.9515], [20.3, 10.4515]]];
        let edges = scene(&thin);
        let face = [200, 200, 200];
        let coarse = edges.predict((10, 10), face, RADV_GRID_BITS);
        let fine = edges.predict((10, 10), face, RADEONSI_GRID_BITS);
        assert!(coarse.is_some() && fine.is_some());
        let (coarse, fine) = (coarse.unwrap(), fine.unwrap());
        assert!(
            coarse.iter().zip(fine).any(|(a, b)| a.abs_diff(b) > 1),
            "{coarse:?} {fine:?}"
        );
        assert!(edges.explains((10, 10), [coarse, face], [fine, face]));
        assert!(!edges.explains((10, 10), [fine, face], [coarse, face]));
    }
}
