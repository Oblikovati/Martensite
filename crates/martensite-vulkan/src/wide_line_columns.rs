//! The columns (rows, for y-major segments) a native wide line covers.
//!
//! Native GL drivers (Mesa llvmpipe `lp_setup_line`, matched by radeonsi in
//! the recorded wide_lines capture) decide whether the first and last columns of an
//! aliased wide segment are drawn by the diamond-exit rule on the *original* endpoints, then
//! cover every column in between with a minor-direction run of w pixels centred on the segment.
//! The Vulkan replicas are one-pixel lines shifted by -(w - 1) / 2 + k, whose own diamond tests
//! would see the shifted endpoints; moving each segment's endpoints along it to the centre of its
//! first column and of the column after its last makes every replica cover exactly the native
//! columns.
use crate::{PrimitiveDraw, RenderVertex};

/// A segment's replacement endpoints: unchanged, covering no column, or moved along the segment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SegmentColumns {
    Unchanged,
    Empty,
    Moved([RenderVertex; 2]),
}

/// One endpoint in clip space and in GL (y-up) window coordinates.
#[derive(Clone, Copy)]
struct WindowPoint {
    clip_w: f32,
    window: [f32; 2],
}

/// Relocate the segment `[a, b]` of `draw` on a framebuffer of `extent` for replicas shifted by
/// `offsets` in the minor direction (see the module docs). Segments whose replicas already cover
/// the native columns keep their endpoints, so the snapped line stays the native one; segments
/// reaching behind the eye keep them too. Example:
/// `segment_columns(&draw, [0, 1], [64, 64], (true, &[-0.5, 0.5]))`.
pub(crate) fn segment_columns(
    draw: &PrimitiveDraw,
    [a, b]: [u32; 2],
    extent: [u32; 2],
    (x_major, offsets): (bool, &[f32]),
) -> SegmentColumns {
    let (from, to) = (draw.vertices[a as usize], draw.vertices[b as usize]);
    let (Some(p0), Some(p1)) = (
        window_point(draw, from, extent),
        window_point(draw, to, extent),
    ) else {
        return SegmentColumns::Unchanged;
    };
    let axis = usize::from(!x_major);
    let Some(native) = covered_centres(p0.window, p1.window, axis) else {
        return SegmentColumns::Empty;
    };
    if replicas_cover(p0.window, p1.window, (axis, offsets), native) {
        return SegmentColumns::Unchanged;
    }
    let [start, end] = native;
    let span = p1.window[axis] - p0.window[axis];
    let moved = [start, end].map(|centre| {
        let t = (centre - p0.window[axis]) / span;
        lerp_vertex(from, to, perspective_parameter(t, p0.clip_w, p1.clip_w))
    });
    SegmentColumns::Moved(moved)
}

fn window_point(
    draw: &PrimitiveDraw,
    vertex: RenderVertex,
    extent: [u32; 2],
) -> Option<WindowPoint> {
    let [x, y, z] = vertex.position;
    let source = [x, y, z, vertex.weight];
    let m = &draw.clip_transform;
    let clip: [f32; 4] =
        std::array::from_fn(|row| (0..4).map(|col| m[col * 4 + row] * source[col]).sum());
    // Behind the eye or on the near plane the window position is not the rasterized one.
    if clip[3] <= 0. || clip[2] < -clip[3] {
        return None;
    }
    let window = [0, 1].map(|axis| (clip[axis] / clip[3] + 1.) * extent[axis] as f32 / 2.);
    Some(WindowPoint {
        clip_w: clip[3],
        window,
    })
}

/// Whether every one-pixel replica, shifted by its offset in the minor direction, covers the
/// `native` columns by its own endpoint tests.
fn replicas_cover(
    p0: [f32; 2],
    p1: [f32; 2],
    (axis, offsets): (usize, &[f32]),
    native: [f32; 2],
) -> bool {
    let shift = |point: [f32; 2], offset: f32| {
        let mut shifted = point;
        shifted[1 - axis] += offset;
        shifted
    };
    offsets.iter().all(|&offset| {
        let (start, end) = (shift(p0, offset), shift(p1, offset));
        !near_diamond_edge(start)
            && !near_diamond_edge(end)
            && covered_centres(start, end, axis) == Some(native)
    })
}

/// Whether a replica endpoint lies within a sixty-fourth of a pixel of its diamond's edge, where
/// the device's float rounding, not the geometry, would decide its diamond-exit test.
fn near_diamond_edge(point: [f32; 2]) -> bool {
    let distance = point.map(|value| (fract(value) - 0.5).abs());
    (distance[0] + distance[1] - 0.5).abs() < 1. / 64.
}

/// The window coordinates along `axis` of the first covered column centre and of the centre
/// after the last one, or `None` when no column is covered.
///
/// `lp_setup_line` places the parallelogram edges at the endpoints, then moves an edge by a half
/// pixel where the diamond decision differs from the edge's own coverage: the start edge only
/// ever grows (its column is drawn when either test draws it) and the end edge only ever shrinks
/// (its column is drawn when both do).
pub(crate) fn covered_centres(p0: [f32; 2], p1: [f32; 2], axis: usize) -> Option<[f32; 2]> {
    let step = if p1[axis] >= p0[axis] { 1. } else { -1. };
    let (draw_start, draw_end) = diamond_ends(p0, p1, axis);
    let offset = |value: f32| fract(value) - 0.5;
    // Whether the endpoint's own column centre lies inside the unmoved edge.
    let (edge_start, edge_end) = if step > 0. {
        (offset(p0[axis]) <= 0., offset(p1[axis]) > 0.)
    } else {
        (offset(p0[axis]) > 0., offset(p1[axis]) <= 0.)
    };
    let first = p0[axis].floor() + if draw_start || edge_start { 0. } else { step };
    let last = p1[axis].floor() - if draw_end && edge_end { 0. } else { step };
    if (last - first) * step < 0. {
        return None;
    }
    Some([first + 0.5, last + step + 0.5])
}

/// Mesa `lp_setup_line` diamond-exit decisions for the start and the end of an aliased segment,
/// implemented per major axis (their sign tests differ on exact pixel centres).
fn diamond_ends(p0: [f32; 2], p1: [f32; 2], axis: usize) -> (bool, bool) {
    let ends = LineEnds::new(p0, p1);
    if axis == 0 {
        (ends.x_major_start(), ends.x_major_end())
    } else {
        (ends.y_major_start(), ends.y_major_end())
    }
}

/// `lp_setup_line` inputs: endpoint offsets from their pixel centres and the deltas v1 - v2.
struct LineEnds {
    p0: [f32; 2],
    p1: [f32; 2],
    d0: [f32; 2],
    d1: [f32; 2],
    dx: f32,
    dy: f32,
}

/// `sign(x)` of `lp_setup_line`: whether x >= 0.
fn sign(value: f32) -> bool {
    value >= 0.
}

fn fract(value: f32) -> f32 {
    value - value.floor()
}

impl LineEnds {
    fn new(p0: [f32; 2], p1: [f32; 2]) -> Self {
        let diff = |value: f32| fract(value) - 0.5;
        let (dx, dy) = (p0[0] - p1[0], p0[1] - p1[1]);
        let mut d1 = p1.map(diff);
        // The minor-axis fix-up of the end point.
        if dx.abs() >= dy.abs() && d1[1] == -0.5 && dy < 0. {
            d1[1] = 0.5;
        }
        if dx.abs() < dy.abs() && d1[0] == -0.5 && dx < 0. {
            d1[0] = 0.5;
        }
        Self {
            p0,
            p1,
            d0: p0.map(diff),
            d1,
            dx,
            dy,
        }
    }
    fn x_major_start(&self) -> bool {
        let [x, y] = self.d0;
        if x.abs() + y.abs() < 0.5 {
            return true;
        }
        if sign(x) == sign(-self.dx) {
            return false;
        }
        if sign(-y) != sign(self.dy) {
            return true;
        }
        let crossing = fract(self.p0[1]) + x * (self.dy / self.dx);
        crossing < 1. && crossing > 0.
    }
    fn x_major_end(&self) -> bool {
        let [x, y] = self.d1;
        if x.abs() + y.abs() < 0.5 || sign(x) != sign(-self.dx) {
            return false;
        }
        if sign(-y) == sign(self.dy) {
            return true;
        }
        let crossing = fract(self.p1[1]) + x * (self.dy / self.dx);
        crossing < 1. && crossing > 0.
    }
    fn y_major_start(&self) -> bool {
        let [x, y] = self.d0;
        if x.abs() + y.abs() < 0.5 {
            return true;
        }
        if sign(-y) == sign(self.dy) {
            return false;
        }
        if sign(x) != sign(-self.dx) {
            return true;
        }
        let crossing = fract(self.p0[0]) + y * (self.dx / self.dy);
        crossing < 1. && crossing > 0.
    }
    fn y_major_end(&self) -> bool {
        let [x, y] = self.d1;
        if x.abs() + y.abs() < 0.5 || sign(-y) != sign(self.dy) {
            return false;
        }
        if sign(x) == sign(-self.dx) {
            return true;
        }
        let crossing = fract(self.p1[0]) + y * (self.dx / self.dy);
        (0. ..1.).contains(&crossing)
    }
}

/// The clip-space parameter of the point at window-space parameter `t` (perspective division).
pub(crate) fn perspective_parameter(t: f32, w0: f32, w1: f32) -> f32 {
    t * w0 / (t * w0 + (1. - t) * w1)
}

pub(crate) fn lerp_vertex(from: RenderVertex, to: RenderVertex, t: f32) -> RenderVertex {
    let mix = |a: f32, b: f32| a + (b - a) * t;
    RenderVertex {
        position: std::array::from_fn(|i| mix(from.position[i], to.position[i])),
        weight: mix(from.weight, to.weight),
        color: std::array::from_fn(|i| mix(from.color[i], to.color[i])),
    }
}

#[cfg(test)]
#[path = "wide_line_columns_tests.rs"]
mod tests;
