//! The half-step raster shift of a shadow map (coordinator decisions 2026-10-28, cross-driver
//! AMD radeonsi versus RADV only).
//!
//! Both AMD drivers snap the light-space vertices of the depth pass to their own sub-pixel grid,
//! so the map's raster (silhouette coverage and the depth interpolated at texel centres) may lie
//! up to half a snap step (1/512 texel) apart between drivers. A differing pixel is admitted
//! when a same-driver capture with the map raster shifted by at most 1/512 texel along one axis
//! reproduces the native value exactly. Whether a tap reads a caster-silhouette texel is kept
//! as a diagnostic.
use super::shadow_flip_reference::{
    FlipLight, POISSON_DISK, ShadowFlipScene, dot, invert, light_window, nearest, unproject,
};

/// Half of the 2^-8 sub-pixel snap step, in shadow-map texels: the largest admitted shift.
pub const HALF_SNAP_STEP: f64 = 1. / 512.;

/// The raster shifts of the control captures: half and quarter snap steps along each axis.
pub const CONTROL_SHIFTS: [[f64; 2]; 8] = [
    [HALF_SNAP_STEP, 0.],
    [-HALF_SNAP_STEP, 0.],
    [0., HALF_SNAP_STEP],
    [0., -HALF_SNAP_STEP],
    [HALF_SNAP_STEP / 2., 0.],
    [-HALF_SNAP_STEP / 2., 0.],
    [0., HALF_SNAP_STEP / 2.],
    [0., -HALF_SNAP_STEP / 2.],
];

/// Whether the centre ray of texel (column, row) of `light`'s map meets one of the first
/// `casters` faces of the scene.
fn covered(scene: &ShadowFlipScene, light: &FlipLight, casters: usize, texel: [i64; 2]) -> bool {
    let size = scene.resolution as f64;
    let [x, y] = texel.map(|t| (t as f64 + 0.5) / size * 2. - 1.);
    let inverse = invert(&light.matrix);
    let from = unproject(&inverse, [x, y, -1.]);
    let to = unproject(&inverse, [x, y, 1.]);
    nearest(&scene.faces[..casters], from, to).is_some()
}

/// Whether some tap of `light` at the surface point reads a silhouette texel of the casters
/// whose coverage change would change the tap's decision.
fn reads_silhouette(
    scene: &ShadowFlipScene,
    light: &FlipLight,
    (point, normal): ([f64; 3], [f64; 3]),
    casters: usize,
) -> bool {
    let [u, v, depth] = light_window(light, point);
    if !(0. ..=1.).contains(&u) || !(0. ..=1.).contains(&v) || depth > 1. {
        return false;
    }
    let bias = (scene.bias * (1. - dot(normal, light.position))).max(scene.bias * 0.1);
    let size = scene.resolution as f64;
    let last = scene.resolution as i64 - 1;
    let shadowed =
        |[c, r]: [i64; 2]| depth - bias > scene.stored_depth(light, c as usize, r as usize);
    POISSON_DISK.iter().any(|tap| {
        let texel = [u + tap[0] / size, v + tap[1] / size]
            .map(|c| ((c * size).floor() as i64).clamp(0, last));
        let neighbours =
            [[1, 0], [-1, 0], [0, 1], [0, -1]].map(|[dc, dr]| [texel[0] + dc, texel[1] + dr]);
        neighbours
            .into_iter()
            .filter(|n| n.iter().all(|&t| (0..=last).contains(&t)))
            .any(|n| {
                covered(scene, light, casters, texel) != covered(scene, light, casters, n)
                    && shadowed(texel) != shadowed(n)
            })
    })
}

/// Whether the surface under pixel (column, row) has a tap reading a caster-silhouette texel
/// of some light's map; the first `casters` faces of the scene cast.
/// Example: `reads_caster_silhouette(&scene, x, y, 6)`.
pub fn reads_caster_silhouette(
    scene: &ShadowFlipScene,
    (column, row): (usize, usize),
    casters: usize,
) -> bool {
    let Some((point, face)) = scene.surface(column, row) else {
        return false;
    };
    let lights = scene.lights.iter();
    lights
        .into_iter()
        .any(|light| reads_silhouette(scene, light, (point, face.normal), casters))
}

/// Whether pixel (x, y) is a proven shifted-raster flip: one of the shifted control captures
/// equals the native value there. Example: `admits_shifted_raster_flip(at, (&native, &controls), w)`.
pub fn admits_shifted_raster_flip(
    pixel: (usize, usize),
    (native, controls): (&[u8], &[Vec<u8>]),
    width: usize,
) -> bool {
    let at = 3 * (pixel.1 * width + pixel.0);
    controls
        .iter()
        .any(|control| control[at..at + 3] == native[at..at + 3])
}
