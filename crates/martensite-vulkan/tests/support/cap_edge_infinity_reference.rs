//! Analytic mesh-edge image of a section cap whose fan reaches vertices at infinity.
//!
//! Defined behaviour (crates/martensite-vulkan/shaders/surface.edges.geom, the design notes):
//! the cap is a fan of four triangles, centre C (w = 1) and two cardinal directions (w = 0). Each
//! edge towards a direction is a ray from C and the edge between two directions lies at infinity.
//! A fragment's edge distance is its viewport distance to the nearest drawn edge line of its
//! triangle, so the cap shows the two lines through C along the plane's local X and Z axes. With
//! skip-first, the triangle (C, D1, D2) hides its first edge C-D1 and keeps only C-D2.
//!
//! The fragment is `mix(edge, source, t)` with `t = smoothstep(width - feather/2, width +
//! feather/2, distance)`, then alpha-tested and blended as `cap_edge_alpha.rs` defines. Three
//! same-driver controls supply its inputs: `face` (edges disabled) must equal the frame outside
//! the analytic cap, `source` (edges disabled, alpha mode Opaque) gives the cap colour and so its
//! texel, and `behind` (capping disabled) gives what a discarded or blended pixel shows.
//!
//! Bound: each RGB channel lies within the composite's range over the half-level quantization of
//! the control bytes, plus one level and 0.01. The output conversion may return either nearest
//! level (Vulkan convertFloatToUint), and the lit source differs from the face pass by a fraction
//! of a level under the edge pipeline's interpolation; 0.01 covers f32 distance and smoothstep.
//! Where the controls are exact texel bytes this is the earlier one-level bound. Alpha is exact. Depth equals `source` where the cap writes depth and
//! `behind` elsewhere. A pixel centre within 1e-4 px of a line separating two skip-first fan
//! triangles, or of the cube outline, has no single analytic value and the reference refuses it.
use martensite_vulkan::FrameCapture;
#[path = "cap_edge_alpha.rs"]
mod alpha;
pub use alpha::CapAlphaPolicy;
use alpha::{EdgeStyle, Outcome, PixelControls};

/// view_setup::configure_camera: 64 px square orthographic view of height 20.
const PIXELS: usize = 64;
const VIEW_HEIGHT: f64 = 20.;
/// capping_scene::cube half extent.
const CUBE_HALF: f64 = 6.;
const ROUNDING: f64 = 1.01;
const BOUNDARY_MARGIN: f64 = 1e-4;

/// The section, edge style and alpha policy read from the live fixture.
#[derive(Clone, Debug)]
pub struct CapEdgeInputs {
    /// Unit-normal plane equation [a, b, c, d] of the section.
    pub plane: [f64; 4],
    /// Linear edge colour and its alpha; a negative alpha draws the hollow edge band.
    pub edge: [f64; 3],
    pub edge_alpha: f64,
    pub width: f64,
    pub feather: f64,
    pub skip_first: bool,
    pub alpha: CapAlphaPolicy,
}

/// Same-driver control frames of the fixture; see the module documentation.
pub struct CapEdgeControls<'a> {
    pub face: &'a FrameCapture,
    pub source: &'a FrameCapture,
    pub behind: &'a FrameCapture,
}

/// The first pixel outside the bound, if any; `cap_edge_violation(&capture, &controls, &inputs)`.
pub fn cap_edge_violation(
    capture: &FrameCapture,
    controls: &CapEdgeControls,
    inputs: &CapEdgeInputs,
) -> Option<String> {
    for frame in [controls.face, controls.source, controls.behind] {
        assert_eq!((capture.width, capture.height), (frame.width, frame.height));
    }
    assert_eq!(
        capture.width as usize, PIXELS,
        "expected the 64 px cap view"
    );
    let fan = CapFan::new(inputs);
    (0..PIXELS * PIXELS).find_map(|index| {
        if !fan.covers(index % PIXELS, index / PIXELS) {
            return exact_violation(capture, controls.face, index);
        }
        fan_violation(capture, controls, &fan, index)
    })
}

fn exact_violation(capture: &FrameCapture, face: &FrameCapture, index: usize) -> Option<String> {
    let range = index * 4..index * 4 + 4;
    let same = capture.rgba[range.clone()] == face.rgba[range.clone()]
        && capture.depth[index] == face.depth[index];
    (!same).then(|| {
        format!(
            "pixel {index}: {:?} depth {}; expected the control {:?} depth {}",
            &capture.rgba[range.clone()],
            capture.depth[index],
            &face.rgba[range],
            face.depth[index]
        )
    })
}

fn fan_violation(
    capture: &FrameCapture,
    controls: &CapEdgeControls,
    fan: &CapFan,
    index: usize,
) -> Option<String> {
    let distance = fan.edge_distance(index % PIXELS, index / PIXELS);
    let pixel = PixelControls {
        source: &controls.source.rgba[index * 4..index * 4 + 3],
        behind: &controls.behind.rgba[index * 4..index * 4 + 3],
    };
    let expected = alpha::outcome(&fan.inputs.alpha, &fan.style(), &pixel, distance);
    let discarded = exact_violation(capture, controls.behind, index);
    let drawn = |range: &[(f64, f64); 3]| drawn_violation(capture, controls, fan, index, range);
    let violation = match &expected {
        Outcome::Discarded => discarded,
        Outcome::Drawn(range) => drawn(range),
        Outcome::Either(range) => discarded.and(drawn(range)),
    };
    violation.map(|failure| format!("{failure} (edge distance {distance} px, {expected:?})"))
}

fn drawn_violation(
    capture: &FrameCapture,
    controls: &CapEdgeControls,
    fan: &CapFan,
    index: usize,
    range: &[(f64, f64); 3],
) -> Option<String> {
    let actual = &capture.rgba[index * 4..index * 4 + 4];
    let color_ok = (0..3).all(|k| {
        let value = f64::from(actual[k]);
        value >= range[k].0 - ROUNDING && value <= range[k].1 + ROUNDING
    });
    let depth_frame = if fan.inputs.alpha.depth_write {
        controls.source
    } else {
        controls.behind
    };
    let rest_ok = actual[3] == controls.face.rgba[index * 4 + 3]
        && capture.depth[index] == depth_frame.depth[index];
    (!color_ok || !rest_ok).then(|| {
        format!(
            "pixel {index}: {actual:?} depth {}; expected depth {}",
            capture.depth[index], depth_frame.depth[index]
        )
    })
}

/// The fan's two edge lines in capture pixels, each as (point, direction): the plane's local X
/// and Z axes through the centre C; `fan_lines(&inputs)[0]`.
pub fn fan_lines(inputs: &CapEdgeInputs) -> [([f64; 2], [f64; 2]); 2] {
    let fan = CapFan::new(inputs);
    [(fan.centre, fan.left), (fan.centre, fan.forward)]
}

/// Viewport-space fan of one section plane.
struct CapFan {
    inputs: CapEdgeInputs,
    centre: [f64; 2],
    /// Viewport directions of the plane's local X (left) and Z (forward) axes.
    left: [f64; 2],
    forward: [f64; 2],
}

impl CapFan {
    fn new(inputs: &CapEdgeInputs) -> Self {
        let [a, b, c, d] = inputs.plane;
        // capping_geometry::cap_orientation: local X = left, local Z = normal x left.
        let length = (a * a + c * c).sqrt();
        let left = [c / length, 0., -a / length];
        let forward = cross([a, b, c], left);
        let centre = [-a * d, -b * d];
        Self {
            inputs: inputs.clone(),
            centre: [pixel_x(centre[0]), pixel_y(centre[1])],
            left: [left[0], -left[1]],
            forward: [forward[0], -forward[1]],
        }
    }

    /// Distance in pixels from the pixel centre to its triangle's drawn edge lines.
    fn edge_distance(&self, column: usize, row: usize) -> f64 {
        let offset = [
            column as f64 + 0.5 - self.centre[0],
            row as f64 + 0.5 - self.centre[1],
        ];
        let to_left = line_distance(offset, self.left);
        let to_forward = line_distance(offset, self.forward);
        assert!(
            to_left.min(to_forward) > BOUNDARY_MARGIN || !self.inputs.skip_first,
            "pixel ({column},{row}) on a fan triangle boundary"
        );
        if !self.inputs.skip_first {
            return to_left.min(to_forward);
        }
        // Local coordinates along (left, forward) select the fan triangle; with skip-first the
        // triangles (C,+Z,+X) and (C,-Z,-X) keep the X ray, (C,-X,+Z) and (C,+X,-Z) the Z ray.
        let along_left = cross2(offset, self.forward) / cross2(self.left, self.forward);
        let along_forward = cross2(self.left, offset) / cross2(self.left, self.forward);
        if along_left * along_forward > 0. {
            to_left
        } else {
            to_forward
        }
    }

    /// Whether the pixel centre lies on the section inside the cube; `fan.covers(20, 31)`.
    fn covers(&self, column: usize, row: usize) -> bool {
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
            margin.abs() > BOUNDARY_MARGIN,
            "pixel ({column},{row}) on the cap outline: {point:?}"
        );
        margin > 0.
    }

    fn style(&self) -> EdgeStyle {
        EdgeStyle {
            color: self.inputs.edge,
            alpha: self.inputs.edge_alpha,
            width: self.inputs.width,
            feather: self.inputs.feather,
        }
    }
}

fn pixel_x(x: f64) -> f64 {
    (x + VIEW_HEIGHT / 2.) * PIXELS as f64 / VIEW_HEIGHT
}
fn pixel_y(y: f64) -> f64 {
    (VIEW_HEIGHT / 2. - y) * PIXELS as f64 / VIEW_HEIGHT
}
fn line_distance(offset: [f64; 2], direction: [f64; 2]) -> f64 {
    cross2(offset, direction).abs() / martensite_cmath::hypot(direction[0], direction[1])
}
fn cross2(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
#[cfg(test)]
#[path = "cap_edge_infinity_reference_tests.rs"]
mod tests;
