//! Clip a zero-width line to a rectangle without changing which pixels it covers.
//!
//! The clipped endpoints are solved from the Bresenham error equation instead of the
//! geometric intersection, so the visible part of a clipped line is exactly the visible
//! part of the same line drawn without clipping. All arithmetic is unsigned 32-bit and
//! wrapping, the width the X sample server uses, so far off-window lines clip identically.

/// Point lies left of the rectangle.
pub(super) const OUT_LEFT: u32 = 0x08;
/// Point lies right of the rectangle.
pub(super) const OUT_RIGHT: u32 = 0x04;
/// Point lies above the rectangle.
pub(super) const OUT_ABOVE: u32 = 0x02;
/// Point lies below the rectangle.
pub(super) const OUT_BELOW: u32 = 0x01;
/// Octant bit: x decreases from start to end.
pub(super) const X_DECREASING: u32 = 4;
/// Octant bit: y decreases from start to end.
pub(super) const Y_DECREASING: u32 = 2;
/// Octant bit: the line steps along y every pixel.
pub(super) const Y_MAJOR: u32 = 1;

// Terms of the sixteen endpoint equations (see `solve_steps`).
const T_2NDX: u32 = 1 << 0;
const T_DXNOTY: u32 = 1 << 1;
const T_SUBDXORY: u32 = 1 << 2;
const T_BIASSUBONE: u32 = 1 << 3;
const T_DIV2DX: u32 = 1 << 4;
const T_ADDONE: u32 = 1 << 5;
const T_ADDDX: u32 = T_DXNOTY;
const T_SUBDX: u32 = T_DXNOTY | T_SUBDXORY;
const T_ADDDY: u32 = 0;
const T_SUBDY: u32 = T_SUBDXORY;

// Moving an x coordinate solves y steps; moving a y coordinate solves x steps.
const EQN1: u32 = T_ADDDX | T_DIV2DX;
const EQN2: u32 = T_ADDDX | T_BIASSUBONE | T_DIV2DX;
const EQN3: u32 = T_SUBDY | T_BIASSUBONE | T_DIV2DX | T_ADDONE;
const EQN3B: u32 = T_ADDDY | T_BIASSUBONE | T_DIV2DX;
const EQN4: u32 = T_SUBDY | T_DIV2DX | T_ADDONE;
const EQN4B: u32 = T_ADDDY | T_DIV2DX;
const EQN5: u32 = T_2NDX | T_SUBDX | T_BIASSUBONE | T_ADDONE;
const EQN5B: u32 = T_2NDX | T_ADDDX | T_BIASSUBONE;
const EQN6: u32 = T_2NDX | T_SUBDX | T_ADDONE;
const EQN6B: u32 = T_2NDX | T_ADDDX;
const EQN7: u32 = T_2NDX | T_ADDDY;
const EQN8: u32 = T_2NDX | T_ADDDY | T_BIASSUBONE;

/// Clipping only needs a few passes per endpoint; the cap guards against a malformed input.
const MAX_CLIP_PASSES: usize = 16;

/// An inclusive pixel rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ClipRect {
    /// Smallest visible `[x, y]`.
    pub(super) min: [i32; 2],
    /// Largest visible `[x, y]`.
    pub(super) max: [i32; 2],
}

impl ClipRect {
    /// The sides a point lies beyond; `rect.outcode([-1, 3]) == OUT_LEFT` for `min = [0, 0]`.
    pub(super) fn outcode(&self, point: [i32; 2]) -> u32 {
        let mut code = 0;
        if point[0] < self.min[0] {
            code |= OUT_LEFT;
        }
        if point[0] > self.max[0] {
            code |= OUT_RIGHT;
        }
        if point[1] < self.min[1] {
            code |= OUT_ABOVE;
        }
        if point[1] > self.max[1] {
            code |= OUT_BELOW;
        }
        code
    }
}

/// The slope description shared by the line walker and the clipper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LineSlope {
    /// Absolute `[dx, dy]`.
    pub(super) delta: [u32; 2],
    /// Octant bits (`X_DECREASING | Y_DECREASING | Y_MAJOR`).
    pub(super) octant: u32,
    /// 1 when this octant prefers an axial step on an exact tie, else 0.
    pub(super) bias: u32,
}

/// The visible part of a line and which endpoints moved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ClippedLine {
    /// First visible pixel.
    pub(super) start: [i32; 2],
    /// Last visible pixel.
    pub(super) end: [i32; 2],
    /// The start was moved onto the rectangle.
    pub(super) start_clipped: bool,
}

/// The rectangle side a pass clips against.
#[derive(Clone, Copy)]
struct ClipEdge {
    axis: usize,
    boundary: i32,
    is_max: bool,
}

/// One solved endpoint equation: which equation, its step distance and anchor.
struct EdgeEquation {
    terms: u32,
    distance: u32,
    anchor: i32,
    negative: bool,
}

struct ClipState {
    points: [[i32; 2]; 2],
    cases: [[i32; 2]; 2],
    outcodes: [u32; 2],
    clipped: [u32; 2],
    swapped: bool,
}

/// Clip `points` (with their `outcodes`) to `rect`; `None` when nothing is visible.
///
/// Example: a horizontal line from `[-5, 2]` to `[5, 2]` clipped to `0..=9` starts at `[0, 2]`.
pub(super) fn zero_clip_line(
    rect: ClipRect,
    slope: LineSlope,
    points: [[i32; 2]; 2],
    outcodes: [u32; 2],
) -> Option<ClippedLine> {
    let mut state = ClipState::new(points, outcodes);
    for _ in 0..MAX_CLIP_PASSES {
        if state.outcodes[0] & state.outcodes[1] != 0 {
            return None;
        }
        if state.outcodes[0] | state.outcodes[1] == 0 {
            return Some(state.finish());
        }
        if state.outcodes[0] == 0 {
            state.swap();
        }
        state.clip_first(rect, slope)?;
    }
    None
}

impl ClipState {
    fn new(points: [[i32; 2]; 2], outcodes: [u32; 2]) -> Self {
        Self {
            points,
            cases: points,
            outcodes,
            clipped: [0, 0],
            swapped: false,
        }
    }

    fn swap(&mut self) {
        self.points.swap(0, 1);
        self.cases.swap(0, 1);
        self.outcodes.swap(0, 1);
        self.clipped.swap(0, 1);
        self.swapped = !self.swapped;
    }

    fn finish(mut self) -> ClippedLine {
        if self.swapped {
            self.points.swap(0, 1);
            self.clipped.swap(0, 1);
        }
        ClippedLine {
            start: self.points[0],
            end: self.points[1],
            start_clipped: self.clipped[0] != 0,
        }
    }

    /// Move the first point onto one violated side; `None` only for a zero divisor.
    fn clip_first(&mut self, rect: ClipRect, slope: LineSlope) -> Option<()> {
        self.clipped[0] |= self.outcodes[0];
        let edge = first_violated_edge(self.outcodes[0], rect);
        let equation = self.edge_equation(edge, slope);
        let steps = solve_steps(&equation, slope)?;
        let steps = if equation.negative {
            steps.wrapping_neg()
        } else {
            steps
        };
        self.points[0][edge.axis] = edge.boundary;
        self.points[0][1 - edge.axis] = (equation.anchor as u32).wrapping_add(steps) as i32;
        self.outcodes[0] = rect.outcode(self.points[0]);
        Some(())
    }

    /// Choose the near or far endpoint as anchor so the step count stays in 15 bits.
    fn edge_equation(&self, edge: ClipEdge, slope: LineSlope) -> EdgeEquation {
        let other = 1 - edge.axis;
        let decreasing = if edge.axis == 0 {
            Y_DECREASING
        } else {
            X_DECREASING
        };
        let negative = slope.octant & decreasing != 0;
        let x_major = slope.octant & Y_MAJOR == 0;
        let near = edge.distance_from(self.cases[0][edge.axis]);
        let (terms, distance, anchor, negative) = if near <= 32767 {
            let terms = equation_terms(edge.axis == 0, x_major, self.swapped, false);
            (terms, near, self.cases[0][other], negative)
        } else {
            let far = edge.distance_to(self.cases[1][edge.axis]);
            let terms = equation_terms(edge.axis == 0, x_major, self.swapped, true);
            (terms, far, self.cases[1][other], !negative)
        };
        EdgeEquation {
            terms,
            distance,
            anchor,
            negative: negative != self.swapped,
        }
    }
}

impl ClipEdge {
    /// Steps from an outside coordinate in to the boundary.
    fn distance_from(self, coordinate: i32) -> u32 {
        let delta = self.boundary.wrapping_sub(coordinate);
        (if self.is_max {
            delta.wrapping_neg()
        } else {
            delta
        }) as u32
    }

    /// Steps from the boundary on to the far coordinate.
    fn distance_to(self, coordinate: i32) -> u32 {
        let delta = coordinate.wrapping_sub(self.boundary);
        (if self.is_max {
            delta.wrapping_neg()
        } else {
            delta
        }) as u32
    }
}

/// Left, then above, then right, then below: the first side named by `outcode`.
fn first_violated_edge(outcode: u32, rect: ClipRect) -> ClipEdge {
    let (axis, is_max) = if outcode & OUT_LEFT != 0 {
        (0, false)
    } else if outcode & OUT_ABOVE != 0 {
        (1, false)
    } else if outcode & OUT_RIGHT != 0 {
        (0, true)
    } else {
        (1, true)
    };
    let boundary = if is_max {
        rect.max[axis]
    } else {
        rect.min[axis]
    };
    ClipEdge {
        axis,
        boundary,
        is_max,
    }
}

/// Select one of the sixteen endpoint equations; see the module docs for their origin.
fn equation_terms(moves_x: bool, x_major: bool, swapped: bool, far: bool) -> u32 {
    let pick = |plain: u32, when_swapped: u32| if swapped { when_swapped } else { plain };
    match (moves_x, x_major, far) {
        (true, true, false) => pick(EQN1, EQN2),
        (true, false, false) => pick(EQN3, EQN4),
        (true, true, true) => pick(EQN2, EQN1),
        (true, false, true) => pick(EQN4B, EQN3B),
        (false, true, false) => pick(EQN5, EQN6),
        (false, false, false) => pick(EQN7, EQN8),
        (false, true, true) => pick(EQN6B, EQN5B),
        (false, false, true) => pick(EQN8, EQN7),
    }
}

/// Evaluate `floor((2 * steps * d + d' + bias term) / 2d'') (+ 1)` with wrapping `u32`.
fn solve_steps(equation: &EdgeEquation, slope: LineSlope) -> Option<u32> {
    let [dx, dy] = slope.delta;
    let terms = equation.terms;
    let scale = if terms & T_2NDX != 0 { dx } else { dy };
    let mut value = (equation.distance << 1).wrapping_mul(scale);
    let offset = if terms & T_DXNOTY != 0 { dx } else { dy };
    value = if terms & T_SUBDXORY != 0 {
        value.wrapping_sub(offset)
    } else {
        value.wrapping_add(offset)
    };
    value = if terms & T_BIASSUBONE != 0 {
        value.wrapping_add(slope.bias.wrapping_sub(1))
    } else {
        value.wrapping_sub(slope.bias)
    };
    let divisor = (if terms & T_DIV2DX != 0 { dx } else { dy }) << 1;
    let quotient = value.checked_div(divisor)?;
    Some(quotient.wrapping_add(u32::from(terms & T_ADDONE != 0)))
}

#[cfg(test)]
#[path = "zero_clip_tests.rs"]
mod tests;
