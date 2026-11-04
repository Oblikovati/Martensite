//! Analytic expected frame for the oblique fixed-function cap lit by a positional source.
//!
//! Defined behaviour (crates/martensite-vulkan/shaders/lighting.glsl, GL 2.1 §2.14.1 P1P2 rule): a cap
//! vertex with eye-space w = 0 is a point at infinity in direction V. A finite light is seen from
//! it along -V̂ at infinite distance, so the attenuation 1/(k0 + k1·d) is 0 when k1 > 0 and
//! 1/k0 otherwise. Ambient, emissive and directional terms do not depend on V.
//!
//! The cap is a fan of triangles: centre C (w = 1) and two cardinal directions D1, D2 (w = 0).
//! An orthographic clip w equals the object w, so the perspective-correct weights of the cap
//! point P = C + s·D1 + t·D2 are (1, s, t)/(1 + s + t). The centre weight is
//! b0 = 1/(1 + |u| + |v|), with (u, v) the plane-local offset of P from C, and the fixed Gouraud
//! colour is texel ⊙ (b0·clamp(Cc) + (1 − b0)·clamp(C∞)).
//!
//! Bound: each RGB channel lies within one 8-bit level of the exact value (any faithful
//! rounding), alpha and background are exact, and depth is within the canonical 1e-6 gate of
//! the analytic plane depth. Pixel centres within 1e-4 texel of a texel edge, or within 1e-4 px
//! of a cap edge, have no single analytic value; the reference refuses such a fixture.
use martensite_vulkan::FrameCapture;

/// view_setup::configure_camera: 64 px square orthographic view of height 20 from z = 20.
const PIXELS: usize = 64;
const VIEW_HEIGHT: f64 = 20.;
const EYE_Z: f64 = 20.;
const Z_RANGE: (f64, f64) = (1., 100.);
/// capping_scene::cube half extent.
const CUBE_HALF: f64 = 6.;
const BACKGROUND: [u8; 4] = [0, 0, 255, 255];
const DEPTH_GATE: f64 = 1e-6;
const EDGE_MARGIN: f64 = 1e-4;

/// Lighting and texture inputs read from the live fixture through the public API.
#[derive(Clone, Debug)]
pub struct CapInfinityInputs {
    /// Unit-normal plane equation [a, b, c, d] of the section.
    pub plane: [f64; 4],
    pub light_position: [f64; 3],
    /// The (constant, linear) attenuation.
    pub attenuation: (f64, f64),
    /// Linear light colour, as uploaded.
    pub light_color: [f64; 3],
    /// Sum of the linear ambient light colours.
    pub scene_ambient: [f64; 3],
    /// sRGB-encoded material⊙interior terms (FFP disables sRGB rendering).
    pub emission: [f64; 3],
    pub ambient: [f64; 3],
    pub diffuse: [f64; 3],
    /// Texture coordinate scale and the four texels of the 2x2 top-down image, row-major.
    pub texture_scale: f64,
    pub texels: [[f64; 3]; 4],
}

/// The first pixel outside the stated bound, if any; `cap_infinity_violation(&capture, &inputs)`.
pub fn cap_infinity_violation(
    capture: &FrameCapture,
    inputs: &CapInfinityInputs,
) -> Option<String> {
    assert_eq!(
        (capture.width, capture.height),
        (PIXELS as u32, PIXELS as u32)
    );
    let frame = CapFrame::new(inputs);
    (0..PIXELS * PIXELS).find_map(|index| pixel_violation(capture, &frame, index))
}

fn pixel_violation(capture: &FrameCapture, frame: &CapFrame, index: usize) -> Option<String> {
    let actual = &capture.rgba[index * 4..index * 4 + 4];
    let depth = f64::from(capture.depth[index]);
    let Some(point) = frame.cap_point(index % PIXELS, index / PIXELS) else {
        return (actual != BACKGROUND || depth != 1.)
            .then(|| format!("pixel {index}: {actual:?} depth {depth}; expected background"));
    };
    let expected = frame.color(point);
    let color_ok =
        (0..3).all(|k| (f64::from(actual[k]) - expected[k]).abs() < 1.) && actual[3] == 255;
    let depth_expected = (EYE_Z - point[2] - Z_RANGE.0) / (Z_RANGE.1 - Z_RANGE.0);
    let depth_ok = (depth - depth_expected).abs() <= DEPTH_GATE;
    (!color_ok || !depth_ok).then(|| {
        format!(
            "pixel {index}: {actual:?} depth {depth}; expected {expected:?} depth {depth_expected}"
        )
    })
}

struct CapFrame {
    inputs: CapInfinityInputs,
    centre: [f64; 3],
    left: [f64; 3],
    forward: [f64; 3],
    infinite: [f64; 3],
    centre_color: [f64; 3],
}

impl CapFrame {
    fn new(inputs: &CapInfinityInputs) -> Self {
        let [a, b, c, d] = inputs.plane;
        let normal = [a, b, c];
        // capping_geometry::cap_orientation: local X = left, local Y = normal, local Z = n × left.
        let length = (a * a + c * c).sqrt();
        let left = [c / length, 0., -a / length];
        let centre = normal.map(|n| -n * d);
        // The cap normal attribute (0, -1, 0) maps to -n; it must face the +z viewer.
        let front = normal.map(|n| -n);
        assert!(
            front[2] > 0.,
            "cap normal {front:?}; expected a front-facing cap"
        );
        let (infinite, centre_color) = vertex_colors(inputs, centre, front);
        Self {
            inputs: inputs.clone(),
            centre,
            left,
            forward: cross(normal, left),
            infinite,
            centre_color,
        }
    }

    /// The section point under a pixel centre, or None outside the cube.
    fn cap_point(&self, column: usize, row: usize) -> Option<[f64; 3]> {
        let size = VIEW_HEIGHT / PIXELS as f64;
        let x = (column as f64 + 0.5) * size - VIEW_HEIGHT / 2.;
        let y = VIEW_HEIGHT / 2. - (row as f64 + 0.5) * size;
        let [a, b, c, d] = self.inputs.plane;
        let point = [x, y, -(a * x + b * y + d) / c];
        let margin = point
            .iter()
            .map(|v| (CUBE_HALF - v.abs()) / size)
            .fold(f64::MAX, f64::min);
        assert!(
            margin.abs() > EDGE_MARGIN,
            "pixel ({column},{row}) on a cap edge: {point:?}"
        );
        (margin > 0.).then_some(point)
    }

    fn color(&self, point: [f64; 3]) -> [f64; 3] {
        let offset = [0, 1, 2].map(|k| point[k] - self.centre[k]);
        let (u, v) = (dot(offset, self.left), dot(offset, self.forward));
        let weight = 1. / (1. + u.abs() + v.abs());
        let texel = self.texel(u, v);
        [0, 1, 2].map(|k| {
            let lit = weight * self.centre_color[k] + (1. - weight) * self.infinite[k];
            texel[k] * lit * 255.
        })
    }

    /// Nearest texel of the repeated 2x2 image at the cap UV (local x, -local z) · scale.
    fn texel(&self, u: f64, v: f64) -> [f64; 3] {
        let cell = |value: f64| {
            let scaled = 2. * value * self.inputs.texture_scale;
            let distance = (scaled - scaled.round()).abs();
            assert!(distance > EDGE_MARGIN, "uv {value} on a texel edge");
            scaled.floor().rem_euclid(2.) as usize
        };
        // A top-down image stores the t = 1 row first.
        self.inputs.texels[(1 - cell(-v)) * 2 + cell(u)]
    }
}

/// Clamped vertex colours at infinity and at the centre; `vertex_colors(&inputs, c, n)`.
fn vertex_colors(
    inputs: &CapInfinityInputs,
    centre: [f64; 3],
    front: [f64; 3],
) -> ([f64; 3], [f64; 3]) {
    let base = [0, 1, 2].map(|k| inputs.emission[k] + inputs.ambient[k] * inputs.scene_ambient[k]);
    let to_light = [0, 1, 2].map(|k| inputs.light_position[k] - centre[k]);
    let distance = dot(to_light, to_light).sqrt();
    let (constant, linear) = inputs.attenuation;
    let attenuation = 1. / (constant + linear * distance);
    let diffuse = (dot(front, to_light) / distance).max(0.);
    // At infinity the light is seen along -V̂ at infinite distance: k1 > 0 cancels the term.
    assert!(
        linear > 0.,
        "fixture attenuation {:?}; expected k1 > 0",
        inputs.attenuation
    );
    let lit = |scale: f64| {
        [0, 1, 2]
            .map(|k| (base[k] + scale * inputs.light_color[k] * inputs.diffuse[k]).clamp(0., 1.))
    };
    (lit(0.), lit(attenuation * diffuse))
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[cfg(test)]
#[path = "cap_infinity_reference_tests.rs"]
mod tests;
