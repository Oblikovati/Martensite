//! RADV against radeonsi: a pixel where a one-pixel line and a blended triangle (a text glyph)
//! lie within one 24-bit depth unit of each other (D2 cross-driver bound, coordinator decision
//! 2026-10-28; stated here for review). Native renders into 24-bit depth; RADV offers no 24-bit
//! depth attachment, and the float32 vertex transform alone moves z/w by about one such unit,
//! so either fragment may win the depth test on either driver.
//!
//! Depths come from the submitted geometry: window depth `(z/w + 1) / 2` in units of 2^-24, the
//! triangle's plus its polygon offset (`factor * max slope + units`, one unit of resolution).
//! The pixel must then equal, byte for byte, the same-driver capture without the triangle (the
//! line in front), or the triangle's fragment blended over that capture (the triangle in
//! front) within one level per channel. The fragment's colour times alpha is read from the
//! same-driver capture of the triangle over the black background, its alpha from the submitted
//! (linear) vertex colour. The view renders into an sRGB attachment, like
//! GL_SRGB8_ALPHA8: blending is linear and captures are encoded, so the rule decodes both
//! captures, blends in linear space and encodes the result. Lavapipe never uses this bound.
use martensite_vulkan::{PrimitiveDraw, PrimitiveTopology};

/// 24-bit depth units per window depth.
const UNITS: f64 = 16_777_215.;

/// Window position and depth (24-bit units) of an object point under a column-major clip matrix.
fn window(clip: &[f32; 16], point: [f32; 3], size: f64) -> Option<[f64; 3]> {
    let row = |r: usize| {
        (0..4)
            .map(|k| f64::from(clip[k * 4 + r]) * [point[0], point[1], point[2], 1.][k] as f64)
            .sum::<f64>()
    };
    let w = row(3);
    (w > 0.).then(|| {
        let (x, y, z) = (row(0) / w, row(1) / w, row(2) / w);
        [
            (x * 0.5 + 0.5) * size,
            (0.5 - y * 0.5) * size,
            (z + 1.) * 0.5 * UNITS,
        ]
    })
}

/// Depth of a one-pixel line at the pixel centre when the line passes within half a pixel of
/// it along its minor axis (z/w is affine along the window segment).
fn line_depth(a: [f64; 3], b: [f64; 3], centre: [f64; 2]) -> Option<f64> {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let major = if dx.abs() >= dy.abs() { 0 } else { 1 };
    let span = b[major] - a[major];
    let t = (centre[major] - a[major]) / span;
    if !(0. ..=1.).contains(&t) {
        return None;
    }
    let minor = a[1 - major] + t * (b[1 - major] - a[1 - major]);
    ((minor - centre[1 - major]).abs() < 0.5).then(|| a[2] + t * (b[2] - a[2]))
}

/// Depth plus polygon offset of a triangle covering the pixel centre, if it covers it.
fn triangle_depth(t: [[f64; 3]; 3], centre: [f64; 2], offset: (f32, f32)) -> Option<f64> {
    let area =
        (t[1][0] - t[0][0]) * (t[2][1] - t[0][1]) - (t[2][0] - t[0][0]) * (t[1][1] - t[0][1]);
    if area == 0. {
        return None;
    }
    let edge = |p: [f64; 3], q: [f64; 3]| {
        ((q[0] - p[0]) * (centre[1] - p[1]) - (q[1] - p[1]) * (centre[0] - p[0])) / area
    };
    let (l0, l1, l2) = (edge(t[1], t[2]), edge(t[2], t[0]), edge(t[0], t[1]));
    if l0 < 0. || l1 < 0. || l2 < 0. {
        return None;
    }
    let dzdx = ((t[1][2] - t[0][2]) * (t[2][1] - t[0][1])
        - (t[2][2] - t[0][2]) * (t[1][1] - t[0][1]))
        / area;
    let dzdy = ((t[1][0] - t[0][0]) * (t[2][2] - t[0][2])
        - (t[2][0] - t[0][0]) * (t[1][2] - t[0][2]))
        / area;
    let slope = dzdx.abs().max(dzdy.abs());
    let depth = l0 * t[0][2] + l1 * t[1][2] + l2 * t[2][2];
    Some(depth + f64::from(offset.0) * slope + f64::from(offset.1))
}

/// Window vertices, fill polygon offset `(factor, units)` and first vertex colour of a primitive.
type WindowPrimitive = (Vec<[f64; 3]>, (f32, f32), [f32; 4]);

/// Window primitives of `draws` of one topology: vertex triples or pairs.
fn primitives(
    draws: &[PrimitiveDraw],
    topology: PrimitiveTopology,
    size: f64,
) -> Vec<WindowPrimitive> {
    let arity = if topology == PrimitiveTopology::Lines {
        2
    } else {
        3
    };
    let mut out = Vec::new();
    for draw in draws.iter().filter(|d| d.topology() == topology) {
        let offset = draw.polygon_offset().map_or((0., 0.), |o| {
            let (mode, factor, units) = o.parameters();
            if mode & 1 != 0 {
                (factor, units)
            } else {
                (0., 0.)
            }
        });
        for chunk in draw.indices().chunks(arity) {
            let vertex = |i: &u32| &draw.vertices()[*i as usize];
            let points: Option<Vec<_>> = chunk
                .iter()
                .map(|i| window(draw.clip_transform(), vertex(i).position, size))
                .collect();
            if let Some(points) = points {
                out.push((points, offset, vertex(&chunk[0]).color));
            }
        }
    }
    out
}

/// A line and a triangle fragment of `draws` at `pixel` within one depth unit: the triangle's
/// vertex colour and the depth gap; `near_tie(&draws, 1040., (781, 541))`.
pub fn near_tie(
    draws: &[PrimitiveDraw],
    size: f64,
    pixel: (usize, usize),
) -> Option<([f32; 4], f64)> {
    let centre = [pixel.0 as f64 + 0.5, pixel.1 as f64 + 0.5];
    let lines = primitives(draws, PrimitiveTopology::Lines, size);
    let lines: Vec<f64> = lines
        .iter()
        .filter_map(|(p, _, _)| line_depth(p[0], p[1], centre))
        .collect();
    let triangles = primitives(draws, PrimitiveTopology::Triangles, size);
    let fragments = triangles.iter().filter_map(|(p, offset, color)| {
        triangle_depth([p[0], p[1], p[2]], centre, *offset).map(|d| (*color, d))
    });
    fragments
        .flat_map(|(color, depth)| lines.iter().map(move |line| (color, (depth - line).abs())))
        .filter(|(_, gap)| *gap < 1.)
        .min_by(|a, b| a.1.total_cmp(&b.1))
}

/// sRGB transfer function (IEC 61966-2-1): an encoded byte to linear and back to encoded units.
fn srgb_to_linear(byte: u8) -> f64 {
    let c = f64::from(byte) / 255.;
    if c <= 0.04045 {
        c / 12.92
    } else {
        martensite_cmath::pow((c + 0.055) / 1.055, 2.4)
    }
}
fn linear_to_srgb(linear: f64) -> f64 {
    let c = linear.clamp(0., 1.);
    255. * if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * martensite_cmath::pow(c, 1. / 2.4) - 0.055
    }
}

/// The pixel colour rule: `rust` equals `without` (line in front) or the triangle's fragment
/// (`over_black` = colour times alpha, alpha from the linear `color`) blended over `without` in
/// linear space and encoded, within one level.
pub fn depth_tie_colour(
    rust: &[u8],
    without: &[u8],
    over_black: &[u8],
    color: [f32; 4],
) -> Result<(), String> {
    if rust == without {
        return Ok(());
    }
    let channel = (0..3)
        .max_by(|&a, &b| color[a].total_cmp(&color[b]))
        .unwrap();
    let alpha = srgb_to_linear(over_black[channel]) / f64::from(color[channel]);
    let blend = |c: usize| {
        linear_to_srgb(srgb_to_linear(over_black[c]) + srgb_to_linear(without[c]) * (1. - alpha))
    };
    if (0..3).all(|c| (f64::from(rust[c]) - blend(c)).abs() <= 1.) {
        return Ok(());
    }
    Err(format!(
        "colour {rust:?} is neither the capture without the triangle {without:?} nor its fragment {over_black:?} blended over it in linear space (alpha {alpha})"
    ))
}

/// Refuse the bound outside the AMD pair; `require_radv(scene.is_radv())?`.
pub fn require_radv(is_radv: bool) -> Result<(), String> {
    if is_radv {
        return Ok(());
    }
    Err(
        "the depth near-tie bound applies only to RADV against radeonsi; lavapipe stays byte-exact"
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const GLYPH: [f32; 4] = [0., 0., 1., 1.];

    #[test]
    fn a_line_and_a_triangle_within_one_unit_tie_and_two_units_apart_do_not() {
        let line = [[0., 10.5, 100.], [20., 10.5, 100.]];
        let quad = |z: f64| [[5., 5., z], [15., 5., z], [5., 15., z]];
        let at = |z: f64| {
            let depth = triangle_depth(quad(z), [7.5, 10.5], (1., 1.)).unwrap();
            (depth - line_depth(line[0], line[1], [7.5, 10.5]).unwrap()).abs()
        };
        assert!(at(98.6) < 1., "0.4 units apart after the one-unit offset");
        assert!(at(101.) >= 2., "two units apart is no near-tie");
    }

    #[test]
    fn the_pixel_must_be_the_line_or_the_blended_fragment() {
        // The perspective label pixel (781, 541) on RADV: encoded 220 over black is linear
        // alpha 0.716; over the white line, linear 1 - 0.716 encodes to 145.
        let (white, over_black) = ([255, 255, 255], [0, 0, 220]);
        assert!(depth_tie_colour(&white, &white, &over_black, GLYPH).is_ok());
        assert!(depth_tie_colour(&[145, 145, 255], &white, &over_black, GLYPH).is_ok());
        assert!(depth_tie_colour(&[146, 146, 255], &white, &over_black, GLYPH).is_ok());
        // Blending the encoded bytes (255 * (1 - 220/255) = 35) is not the attachment's blend.
        assert!(depth_tie_colour(&[35, 35, 255], &white, &over_black, GLYPH).is_err());
        assert!(depth_tie_colour(&[0, 0, 220], &white, &over_black, GLYPH).is_err());
        assert!(depth_tie_colour(&[150, 150, 255], &white, &over_black, GLYPH).is_err());
    }

    #[test]
    fn the_transfer_function_round_trips_every_byte() {
        for byte in 0..=255u8 {
            assert!((linear_to_srgb(srgb_to_linear(byte)) - f64::from(byte)).abs() < 1e-9);
        }
    }

    #[test]
    fn lavapipe_is_refused() {
        assert!(require_radv(false).is_err());
        assert!(require_radv(true).is_ok());
    }
}
