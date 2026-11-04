//! View-snap tap flips (user decision 2026-10-28, rule B; AMD only):
//! the recorded controls
//!
//! The light-space position LightShadow.glsl samples is interpolated with the barycentrics of
//! the view triangle the rasterizer snapped, so the sample at a pixel centre `p` is the surface
//! point under `q = sum(b'_i * v_i)`, where `b'` are the barycentrics of `p` in the snapped
//! triangle and `v_i` the unsnapped window vertices. RADV snaps to 2^-8 px and radeonsi to
//! 2^-12 px (Mesa 26.1.6, viewports up to 1024 px). A differing pixel passes when, on the
//! recorded triangle of its face, the casting lights' shadowed-tap total at RADV's `q` differs
//! from the total at radeonsi's `q`, and RADV's pixel is darker exactly when it has more
//! shadowed taps.
use super::shadow_flip_reference::ShadowFlipScene;

/// RADV's subpixel grid, in fractional bits.
const RADV_GRID_BITS: i32 = 8;
/// radeonsi's subpixel grid for viewports up to 1024 px, in fractional bits.
const RADEONSI_GRID_BITS: i32 = 12;
/// How close, in pixels, a recorded vertex must be to a projected face corner to belong to it:
/// the recorded vertices come through the renderer's f32 clip transform, the corners through
/// the f64 camera, so they agree to about 1e-4 px; distinct corners lie pixels apart.
const CORNER_MATCH: f64 = 1e-3;

/// `vertex` snapped to the 2^-bits grid of OpenGL's y-up window frame of height `height`.
fn snapped(vertex: [f64; 2], bits: i32, height: f64) -> [f64; 2] {
    let scale = martensite_cmath::powi(2f64, bits);
    let x = (vertex[0] * scale).round() / scale;
    [x, height - ((height - vertex[1]) * scale).round() / scale]
}

/// Barycentric weights of `p` in triangle `t`.
fn barycentric(t: [[f64; 2]; 3], p: [f64; 2]) -> [f64; 3] {
    let [[x0, y0], [x1, y1], [x2, y2]] = t;
    let det = (x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0);
    let l1 = ((p[0] - x0) * (y2 - y0) - (x2 - x0) * (p[1] - y0)) / det;
    let l2 = ((x1 - x0) * (p[1] - y0) - (p[0] - x0) * (y1 - y0)) / det;
    [1. - l1 - l2, l1, l2]
}

/// The window position whose surface the snapped recorded triangle samples at the centre of
/// pixel (column, row): the recorded triangle of the face under the centre whose snapped
/// outline holds it; `None` off every face or without such a triangle.
pub fn snapped_sample(
    scene: &ShadowFlipScene,
    (column, row): (usize, usize),
    bits: i32,
) -> Option<[f64; 2]> {
    let (_, face) = scene.surface(column, row)?;
    let corners = face.corners.map(|corner| scene.window(corner));
    let on_face = |v: &[f64; 2]| {
        corners
            .iter()
            .any(|c| martensite_cmath::hypot(c[0] - v[0], c[1] - v[1]) <= CORNER_MATCH)
    };
    let p = [column as f64 + 0.5, row as f64 + 0.5];
    let height = scene.size.1 as f64;
    scene
        .view_triangles
        .iter()
        .filter(|t| t.iter().all(on_face))
        .find_map(|t| {
            let weights = barycentric(t.map(|v| snapped(v, bits, height)), p);
            weights.iter().all(|w| *w >= 0.).then(|| {
                let at = |axis: usize| (0..3).map(|k| weights[k] * t[k][axis]).sum();
                [at(0), at(1)]
            })
        })
}

/// Each casting light's shadowed tap count at window position `at`.
pub fn shadowed_taps(scene: &ShadowFlipScene, at: [f64; 2]) -> Option<Vec<usize>> {
    let (point, face) = scene.surface_at(at)?;
    let count = |light| {
        let margins = scene.tap_margins(light, point, face.normal);
        margins.iter().filter(|margin| **margin > 0.).count()
    };
    Some(scene.lights.iter().map(count).collect())
}

/// The shadowed-tap total on RADV's grid minus the total on radeonsi's grid at a pixel.
pub fn tap_total_change(scene: &ShadowFlipScene, pixel: (usize, usize)) -> Option<i64> {
    let total = |bits: i32| -> Option<i64> {
        let taps = shadowed_taps(scene, snapped_sample(scene, pixel, bits)?)?;
        Some(taps.iter().sum::<usize>() as i64)
    };
    Some(total(RADV_GRID_BITS)? - total(RADEONSI_GRID_BITS)?)
}

/// Rule B at one differing pixel: the grids change the shadowed-tap total, and RADV's pixel is
/// darker (smaller channel sum) exactly when its grid has more shadowed taps.
pub fn admits_view_snap_flip(
    scene: &ShadowFlipScene,
    pixel: (usize, usize),
    radv: &[u8],
    native: &[u8],
) -> bool {
    flip_agrees(tap_total_change(scene, pixel), radv, native)
}

/// Whether a tap-total change agrees with the colour change; `flip_agrees(Some(1), &[1], &[2])`.
fn flip_agrees(change: Option<i64>, radv: &[u8], native: &[u8]) -> bool {
    let sum = |rgb: &[u8]| rgb.iter().map(|&c| i64::from(c)).sum::<i64>();
    let (radv, native) = (sum(radv), sum(native));
    change.is_some_and(|change| change != 0 && radv != native && (change > 0) == (radv < native))
}

/// JSON of the shadowed tap counts on both grids at a pixel (diagnostic).
pub fn view_snap_json(scene: &ShadowFlipScene, pixel: (usize, usize)) -> String {
    let counts =
        |bits: i32| snapped_sample(scene, pixel, bits).and_then(|at| shadowed_taps(scene, at));
    let centre = shadowed_taps(scene, [pixel.0 as f64 + 0.5, pixel.1 as f64 + 0.5]);
    format!(
        "{{\"centre\":{centre:?},\"radv_grid\":{:?},\"radeonsi_grid\":{:?},\"recorded_triangles\":{}}}",
        counts(RADV_GRID_BITS),
        counts(RADEONSI_GRID_BITS),
        scene.view_triangles.len()
    )
}

#[cfg(test)]
mod tests {
    use super::super::shadow_flip_reference_tests::overhead;
    use super::{barycentric, flip_agrees, snapped, snapped_sample, tap_total_change};

    #[test]
    fn vertices_snap_to_the_grid_of_the_y_up_frame() {
        assert_eq!(snapped([1.001, 2.001], 8, 4.), [1., 2.]);
        let fine = snapped([1.001, 2.001], 12, 4.);
        assert!(
            (fine[0] - 4100. / 4096.).abs() < 1e-12,
            "expected 4100/4096, got {fine:?}"
        );
    }

    #[test]
    fn barycentric_weights_sum_to_one_and_locate_corners() {
        let triangle = [[0., 0.], [4., 0.], [0., 4.]];
        assert_eq!(barycentric(triangle, [0., 0.]), [1., 0., 0.]);
        let inside = barycentric(triangle, [1., 1.]);
        assert!(
            (inside.iter().sum::<f64>() - 1.).abs() < 1e-12,
            "{inside:?}"
        );
        assert!(barycentric(triangle, [5., 5.]).iter().any(|w| *w < 0.));
    }

    #[test]
    fn only_a_change_that_matches_the_colour_change_is_a_flip() {
        assert!(flip_agrees(Some(1), &[10, 0, 0], &[12, 0, 0]));
        assert!(flip_agrees(Some(-1), &[12, 0, 0], &[10, 0, 0]));
        assert!(
            !flip_agrees(Some(1), &[12, 0, 0], &[10, 0, 0]),
            "brighter with more shadow"
        );
        assert!(
            !flip_agrees(Some(0), &[10, 0, 0], &[12, 0, 0]),
            "no tap changes"
        );
        assert!(
            !flip_agrees(None, &[10, 0, 0], &[12, 0, 0]),
            "no recorded triangle"
        );
    }

    /// The overhead floor (-5..5 covers window 16..48) as two recorded triangles.
    fn recorded_floor(offset: f64) -> super::ShadowFlipScene {
        let mut scene = overhead(0.5);
        let corners = scene.faces[0].corners.map(|c| scene.window(c));
        let jitter = |mut v: [f64; 2]| {
            v[0] += offset;
            v
        };
        scene.view_triangles = vec![
            [corners[0], corners[1], corners[2]].map(jitter),
            [corners[0], corners[2], corners[3]].map(jitter),
        ];
        scene
    }

    #[test]
    fn the_sample_follows_the_recorded_triangle_and_needs_one() {
        let exact = recorded_floor(0.);
        let sample = snapped_sample(&exact, (30, 30), 8).unwrap();
        assert!(
            (sample[0] - 30.5).abs() < 1e-9 && (sample[1] - 30.5).abs() < 1e-9,
            "{sample:?}"
        );
        // Vertices off the projected corners do not belong to the face: no sample, no flip.
        let foreign = recorded_floor(0.25);
        assert!(snapped_sample(&foreign, (30, 30), 8).is_none());
        assert!(tap_total_change(&foreign, (30, 30)).is_none());
        assert_eq!(tap_total_change(&exact, (30, 30)), Some(0));
    }
}
