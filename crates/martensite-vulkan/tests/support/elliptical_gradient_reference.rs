//! Independent analytic reference for the elliptical gradient background (decision D2, 2026-10-27).
//!
//! Native OpenGL and Vulkan drivers both rasterize the same 64-triangle fan and differ by one
//! 8-bit level at some pixels. Neither is a reference for the other, so both are graded against
//! the exact image: piecewise-linear interpolation of linear RGB over the native fan geometry,
//! evaluated at pixel centers in f64 and encoded with the exact sRGB transfer function.
//!
//! Bound: every 8-bit channel must be a faithful rounding of the exact value, that is
//! `|byte - 255 * srgb(exact)| < 1`. Both adjacent representable levels are accepted because
//! Vulkan and OpenGL do not require round-to-nearest for sRGB attachment writes. Native software
//! and AMD captures reach at most 0.632 of a level (the recorded controls).

/// Exclusive per-channel bound, in 8-bit levels, between a capture and the exact image.
pub(crate) const FAITHFUL_ROUNDING_LEVELS: f64 = 1.0;

/// Native fan geometry with authored center and rim colors.
/// Example: `EllipticalGradientReference::from_native_records(records, [white, black])`.
pub(crate) struct EllipticalGradientReference {
    center: [f64; 2],
    rim: Vec<[f64; 2]>,
    colors: [[f64; 3]; 2],
}

/// Largest deviation found and the number of graded channels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ReferenceGrade {
    pub(crate) max_levels: f64,
    pub(crate) channels: usize,
}

impl EllipticalGradientReference {
    /// Read the 66 native `x y r g b` f32-bit records; only positions are used.
    /// Example: `from_native_records(include_str!("64x64.txt"), colors)`.
    pub(crate) fn from_native_records(records: &str, colors: [[f32; 4]; 2]) -> Self {
        let points: Vec<[f64; 2]> = records.lines().map(record_position).collect();
        assert_eq!(
            points.len(),
            66,
            "native records={}; expected center + 65 rim",
            points.len()
        );
        Self {
            center: points[0],
            rim: points[1..].to_vec(),
            colors: colors.map(|color| [0, 1, 2].map(|k| f64::from(color[k]))),
        }
    }

    /// Exact 8-bit sRGB levels at a top-left capture pixel; `reference.exact_levels(column, row, height)`.
    pub(crate) fn exact_levels(&self, column: u32, row: u32, height: u32) -> [f64; 3] {
        // The fan is authored in bottom-up window coordinates; captures are top-left rows.
        let point = [f64::from(column) + 0.5, f64::from(height - row) - 0.5];
        let weight = self.rim_weight(point);
        let [center, rim] = self.colors;
        [0, 1, 2].map(|k| 255. * srgb_encode((1. - weight) * center[k] + weight * rim[k]))
    }

    fn rim_weight(&self, point: [f64; 2]) -> f64 {
        // The triangle cones around the center partition the plane; weight > 1 lies past the rim.
        let weight = self
            .rim
            .windows(2)
            .find_map(|edge| fan_rim_weight(self.center, edge[0], edge[1], point));
        match weight {
            Some(weight) if weight <= 1. + 1e-12 => weight,
            _ => panic!("pixel center {point:?} weight={weight:?}; expected inside the fan"),
        }
    }

    /// Grade background pixels (depth 1) of a top-left RGBA capture.
    /// Example: `reference.grade(&capture.rgba, &capture.depth, [width, height])`.
    pub(crate) fn grade(&self, rgba: &[u8], depth: &[f32], extent: [u32; 2]) -> ReferenceGrade {
        let [width, height] = extent;
        require_capture_length(rgba, depth, extent);
        let mut grade = ReferenceGrade {
            max_levels: 0.,
            channels: 0,
        };
        let (pixels, _) = rgba.as_chunks::<4>();
        for (index, pixel) in pixels.iter().enumerate() {
            if depth[index] != 1. {
                continue;
            }
            let (column, row) = (index as u32 % width, index as u32 / width);
            grade_pixel(pixel, self.exact_levels(column, row, height), &mut grade);
        }
        grade
    }
}

fn require_capture_length(rgba: &[u8], depth: &[f32], [width, height]: [u32; 2]) {
    let pixels = (width * height) as usize;
    assert_eq!(
        rgba.len(),
        pixels * 4,
        "rgba bytes; expected width*height*4"
    );
    assert_eq!(depth.len(), pixels, "depth values; expected width*height");
}

fn grade_pixel(pixel: &[u8; 4], exact: [f64; 3], grade: &mut ReferenceGrade) {
    for (byte, exact) in pixel.iter().zip(exact) {
        let deviation = (f64::from(*byte) - exact).abs();
        grade.max_levels = grade.max_levels.max(deviation);
        grade.channels += 1;
    }
}

/// Fail unless every graded channel is a faithful rounding; `assert_faithful(grade, "case")`.
pub(crate) fn assert_faithful(grade: ReferenceGrade, label: &str) {
    println!(
        "{{\"elliptical_reference\":\"{label}\",\"max_levels\":{},\"channels\":{},\"bound\":{FAITHFUL_ROUNDING_LEVELS}}}",
        grade.max_levels, grade.channels
    );
    assert!(grade.channels > 0, "{label}: no background channel graded");
    assert!(
        grade.max_levels < FAITHFUL_ROUNDING_LEVELS,
        "{label}: deviation={} levels; expected < {FAITHFUL_ROUNDING_LEVELS} from the exact fan image",
        grade.max_levels
    );
}

/// Exact sRGB transfer function on [0, 1]; `srgb_encode(0.5)` is about 0.7354.
pub(crate) fn srgb_encode(linear: f64) -> f64 {
    if linear <= 0.003_130_8 {
        return 12.92 * linear;
    }
    1.055 * martensite_cmath::pow(linear, 1. / 2.4) - 0.055
}

fn record_position(line: &str) -> [f64; 2] {
    let bits: Vec<u32> = line
        .split_whitespace()
        .map(|word| word.parse().unwrap())
        .collect();
    assert_eq!(
        bits.len(),
        5,
        "record={line:?}; expected five f32 bit words"
    );
    [bits[0], bits[1]].map(|word| f64::from(f32::from_bits(word)))
}

fn fan_rim_weight(
    center: [f64; 2],
    first: [f64; 2],
    second: [f64; 2],
    point: [f64; 2],
) -> Option<f64> {
    // Barycentric weights of the two rim vertices; their sum is the rim color weight.
    let cross = |u: [f64; 2], v: [f64; 2]| u[0] * v[1] - u[1] * v[0];
    let offset = |p: [f64; 2]| [p[0] - center[0], p[1] - center[1]];
    let (a, b, p) = (offset(first), offset(second), offset(point));
    let area = cross(a, b);
    let (wa, wb) = (cross(p, b) / area, cross(a, p) / area);
    let inside = wa >= -1e-12 && wb >= -1e-12;
    inside.then_some(wa + wb)
}
