//! The shadow-map depth grid model (AMD only, user decision 2026-10-28; the design notes
//! and the recorded controls).
//!
//! RADV snaps window vertices to 1/256 px and radeonsi to 1/4096 px, which moves a shadow-map
//! texel's interpolated depth by up to about 1e-4 between the drivers. The model predicts each
//! texel from the recorded shadow-pass triangles alone. It takes the light-clip position in
//! float32 with the shader's column order, then the window position snapped to a driver's grid,
//! then the plane depth at the texel centre over the front faces plus the shadow pass's
//! slope-scaled polygon offset (factor 1), keeping the nearest. A texel beyond the native spread
//! passes when the model predicts RADV on RADV's grid and radeonsi on radeonsi's grid, each within
//! `DEPTH_BOUND`, and the coverage is identical.
use martensite_vulkan::{PrimitiveDraw, PrimitiveTopology};

pub(crate) const RADV_GRID: f64 = 1. / 256.;
pub(crate) const RADEONSI_GRID: f64 = 1. / 4096.;
/// About four float32 steps near depth 1; the model fits both AMD drivers within 1.2e-7.
pub(crate) const DEPTH_BOUND: f64 = 2.5e-7;

/// A shadow-pass triangle in GL window coordinates (texels, rows bottom-up) with window depth.
pub(crate) type LightTriangle = [[f64; 3]; 3];

/// `((c0 * x + c1 * y) + c2 * z) + c3 * w` in float32, as shaders/clip_position.glsl sums it.
fn light_clip(light: &[f32; 16], v: [f32; 4]) -> [f32; 4] {
    std::array::from_fn(|r| {
        let sum = light[r] * v[0] + light[4 + r] * v[1];
        (sum + light[8 + r] * v[2]) + light[12 + r] * v[3]
    })
}

/// The window position and depth of a light-clip point in a `resolution`-texel map.
fn light_window(clip: [f32; 4], resolution: f64) -> [f64; 3] {
    let ndc = |i: usize| f64::from(clip[i]) / f64::from(clip[3]);
    [
        (ndc(0) + 1.) * 0.5 * resolution,
        (ndc(1) + 1.) * 0.5 * resolution,
        ndc(2) * 0.5 + 0.5,
    ]
}

/// The triangles of `draws` in the light window of the map; the scene's shapes have no location,
/// so object coordinates are world coordinates. Example: `light_triangles(&draws, &matrix, 64)`.
pub(crate) fn light_triangles(
    draws: &[PrimitiveDraw],
    light: &[f32; 16],
    resolution: u32,
) -> Vec<LightTriangle> {
    let triangles = draws
        .iter()
        .filter(|d| d.topology() == PrimitiveTopology::Triangles);
    let window = |d: &PrimitiveDraw, i: u32| {
        let v = &d.vertices()[i as usize];
        let point = [v.position[0], v.position[1], v.position[2], v.weight];
        light_window(light_clip(light, point), f64::from(resolution))
    };
    let corners = |d: &PrimitiveDraw| -> Vec<LightTriangle> {
        let indices: Vec<u32> = match d.indices().is_empty() {
            true => (0..d.vertices().len() as u32).collect(),
            false => d.indices().to_vec(),
        };
        let triples = indices.as_chunks::<3>().0.iter();
        triples.map(|t| t.map(|i| window(d, i))).collect()
    };
    triangles.flat_map(corners).collect()
}

/// Round each window position to the grid `step` (texels); depth is not snapped.
fn snapped(triangle: &LightTriangle, step: f64) -> LightTriangle {
    triangle.map(|[x, y, z]| [(x / step).round() * step, (y / step).round() * step, z])
}

/// The predicted depths, bottom row first, 1 where no front face covers the texel centre.
/// Example: `predicted_depths(&triangles, 64, RADV_GRID)`.
pub(crate) fn predicted_depths(
    triangles: &[LightTriangle],
    resolution: u32,
    step: f64,
) -> Vec<f64> {
    let size = resolution as usize;
    let mut depth = vec![1.; size * size];
    for triangle in triangles {
        rasterize_front_face(&snapped(triangle, step), size, &mut depth);
    }
    depth
}

/// Keep the nearer of each covered texel's depth and the triangle's plane depth plus its offset.
fn rasterize_front_face(t: &LightTriangle, size: usize, depth: &mut [f64]) {
    let ([x0, y0, z0], [x1, y1, z1], [x2, y2, z2]) = (t[0], t[1], t[2]);
    let area = (x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0);
    if area <= 0. {
        return; // back faces are culled
    }
    let dzdx = ((z1 - z0) * (y2 - y0) - (z2 - z0) * (y1 - y0)) / area;
    let dzdy = ((x1 - x0) * (z2 - z0) - (x2 - x0) * (z1 - z0)) / area;
    let offset = dzdx.abs().max(dzdy.abs());
    for (k, texel) in depth.iter_mut().enumerate() {
        let (px, py) = ((k % size) as f64 + 0.5, (k / size) as f64 + 0.5);
        let b1 = ((px - x0) * (y2 - y0) - (x2 - x0) * (py - y0)) / area;
        let b2 = ((x1 - x0) * (py - y0) - (px - x0) * (y1 - y0)) / area;
        if b1 >= 0. && b2 >= 0. && b1 + b2 <= 1. {
            let plane = (1. - b1 - b2) * z0 + b1 * z1 + b2 * z2 + offset;
            *texel = texel.min(plane);
        }
    }
}

/// Texels of the RADV dump `ours` that neither stay within `spread` of the radeonsi dump nor
/// follow the grid model (both predictions within `DEPTH_BOUND`, identical coverage).
/// Example: `unexplained_texels(&ours, &native, &triangles, 64, 4.9e-6)`.
pub(crate) fn unexplained_texels(
    ours: &[f32],
    native: &[f32],
    triangles: &[LightTriangle],
    resolution: u32,
    spread: f32,
) -> Vec<usize> {
    let radv = predicted_depths(triangles, resolution, RADV_GRID);
    let radeonsi = predicted_depths(triangles, resolution, RADEONSI_GRID);
    let explained = |k: usize| {
        let (a, b) = (f64::from(ours[k]), f64::from(native[k]));
        if (a < 1.) != (b < 1.) {
            return false;
        }
        if (ours[k] - native[k]).abs() <= spread {
            return true;
        }
        let coverage = (radv[k] < 1.) == (a < 1.) && (radeonsi[k] < 1.) == (b < 1.);
        coverage && (a - radv[k]).abs() <= DEPTH_BOUND && (b - radeonsi[k]).abs() <= DEPTH_BOUND
    };
    (0..ours.len()).filter(|&k| !explained(k)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One front-facing triangle in an 8-texel map with a steep depth slope (0.01 per texel) and
    /// vertices off both grids, so the two grids give depths about 2e-5 apart.
    const SLOPED: LightTriangle = [
        [0.3001, 0.2003, 0.2],
        [7.7007, 0.4009, 0.27],
        [0.5011, 7.6013, 0.21],
    ];

    fn dumps(radv_step: f64, radeonsi_step: f64) -> (Vec<f32>, Vec<f32>) {
        let cast = |d: Vec<f64>| d.into_iter().map(|z| z as f32).collect();
        (
            cast(predicted_depths(&[SLOPED], 8, radv_step)),
            cast(predicted_depths(&[SLOPED], 8, radeonsi_step)),
        )
    }

    #[test]
    fn each_driver_on_its_own_grid_is_explained() {
        let (ours, native) = dumps(RADV_GRID, RADEONSI_GRID);
        let beyond = ours
            .iter()
            .zip(&native)
            .filter(|(a, b)| (*a - *b).abs() > 1e-6)
            .count();
        assert!(
            beyond > 0,
            "the grids must separate the dumps beyond the spread"
        );
        assert_eq!(
            unexplained_texels(&ours, &native, &[SLOPED], 8, 1e-6),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn swapped_grids_are_refused() {
        let (ours, native) = dumps(RADEONSI_GRID, RADV_GRID);
        assert!(!unexplained_texels(&ours, &native, &[SLOPED], 8, 1e-6).is_empty());
    }

    #[test]
    fn a_vertex_moved_by_one_radv_step_is_refused() {
        let mut moved = SLOPED;
        moved[1][0] += RADV_GRID;
        let ours: Vec<f32> = predicted_depths(&[moved], 8, RADV_GRID)
            .into_iter()
            .map(|z| z as f32)
            .collect();
        let (_, native) = dumps(RADV_GRID, RADEONSI_GRID);
        assert!(!unexplained_texels(&ours, &native, &[SLOPED], 8, 1e-6).is_empty());
    }

    #[test]
    fn a_coverage_difference_is_refused_even_within_the_spread() {
        let (mut ours, native) = dumps(RADV_GRID, RADEONSI_GRID);
        let covered = native.iter().position(|z| *z < 1.).unwrap();
        ours[covered] = 1.;
        assert_eq!(
            unexplained_texels(&ours, &native, &[SLOPED], 8, 1.),
            vec![covered]
        );
    }

    #[test]
    fn the_offset_is_the_depth_slope_and_back_faces_are_culled() {
        let flat = [[0., 0., 0.5], [8., 0., 0.5], [0., 8., 0.5]];
        assert_eq!(predicted_depths(&[flat], 8, RADV_GRID)[0], 0.5);
        let sloped = [[0., 0., 0.5], [8., 0., 0.58], [0., 8., 0.5]];
        assert!((predicted_depths(&[sloped], 8, RADV_GRID)[0] - (0.505 + 0.01)).abs() < 1e-12);
        let back = [flat[0], flat[2], flat[1]];
        assert!(
            predicted_depths(&[back], 8, RADV_GRID)
                .iter()
                .all(|z| *z == 1.)
        );
    }
}
