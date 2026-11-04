//! GL-vs-Vulkan exact ties (user decision, 2026-10-27, the design notes).
//!
//! OpenGL rasterizes in a y-up window and Vulkan in a y-down framebuffer, so a primitive
//! boundary that falls exactly on a pixel centre (a fill or wide-line edge), or a thin line that
//! passes exactly through a diamond corner (half way between two pixel centres along its minor
//! axis), is resolved on opposite sides by the two APIs. Only differences proven, from the
//! submitted geometry, to lie on such a tie may use the `raster_shift` shift of
//! `the recording tool`: each differing pixel's colour, with its depth within the
//! `1e-6` depth gate, occurs within one pixel in the other image. When every differing pixel is
//! such a proven tie there is no percentage cap (revised the same day). Every other pixel stays
//! byte-exact, with depth within `1e-6`.
//!
//! Loci are window-space lines in capture pixels (origin top-left, y down, pixel centres at
//! half-integers), computed by the caller from the scene geometry, never from the images.
//!
//! Two further rules take same-driver control captures (`TieControls`, user decisions
//! 2026-10-28):
//! - **Ties that uncover a surface.** A tie pixel may show the surface under the primitive when
//!   it equals, colour and depth, the same driver's capture without that primitive, and the other
//!   image's primitive sample occurs within one pixel of it.
//! - **Undefined loci.** Inside an `UndefinedLocus` (the dash start of an edge from a vertex at
//!   infinity, exactly coplanar edge and face depths) each image's pixel may equal its own
//!   driver's face-only or edge-only capture.

/// A primitive boundary that may produce an exact rasterization tie.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TieLocus {
    /// Edge of a filled region (polygon edge, wide-line or point side) on the line
    /// `a·x + b·y + c = 0`; a pixel ties when its centre lies exactly on it.
    FillEdge { a: f64, b: f64, c: f64 },
    /// One-pixel line on `a·x + b·y + c = 0`; a pixel ties when its centre lies exactly half a
    /// pixel from the line along the line's minor axis (the diamond-exit corner).
    ThinLine { a: f64, b: f64, c: f64 },
}

/// Exactness of a tie test: loci and centres are dyadic, so a true tie evaluates to zero.
const TIE_EPSILON: f64 = 1e-9;

impl TieLocus {
    /// Horizontal thin line at window `y`; `TieLocus::thin_row(16.)`.
    pub fn thin_row(y: f64) -> Self {
        Self::ThinLine {
            a: 0.,
            b: 1.,
            c: -y,
        }
    }
    /// Horizontal fill edge at window `y`; `TieLocus::fill_row(30.5)`.
    pub fn fill_row(y: f64) -> Self {
        Self::FillEdge {
            a: 0.,
            b: 1.,
            c: -y,
        }
    }
    /// Whether the centre of pixel (column, row) lies on this tie; `locus.ties(3, 15)`.
    pub fn ties(&self, column: usize, row: usize) -> bool {
        let (x, y) = (column as f64 + 0.5, row as f64 + 0.5);
        match *self {
            Self::FillEdge { a, b, c } => (a * x + b * y + c).abs() <= TIE_EPSILON,
            Self::ThinLine { a, b, c } => {
                // Offset along the minor axis: divide by the major-axis coefficient.
                let major = a.abs().max(b.abs());
                ((a * x + b * y + c).abs() / major - 0.5).abs() <= TIE_EPSILON
            }
        }
    }
}

/// Depth gate shared with `native_golden::assert_golden_depth`.
const DEPTH_GATE: f32 = 1e-6;

/// One RGBA8 image and its per-pixel depth, both top-left row order.
#[derive(Clone, Copy, Debug)]
pub struct TieFrame<'a> {
    pub rgba: &'a [u8],
    pub depth: &'a [f32],
}

/// Counts reported for a graded comparison; `lit` is informational (no percentage cap).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TieGrade {
    pub differing: usize,
    pub lit: usize,
}

/// A band the geometry proves undefined: pixel centres within `half_width` of
/// `a·x + b·y + c = 0`; e.g. the 3 px edge through a cap centre, `half_width: 1.5`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UndefinedLocus {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub half_width: f64,
}

impl UndefinedLocus {
    /// Whether the centre of pixel (column, row) lies in the band; `locus.contains(17, 33)`.
    pub fn contains(&self, column: usize, row: usize) -> bool {
        let (x, y) = (column as f64 + 0.5, row as f64 + 0.5);
        let distance =
            (self.a * x + self.b * y + self.c).abs() / martensite_cmath::hypot(self.a, self.b);
        distance <= self.half_width + TIE_EPSILON
    }
}

/// One driver's control captures, all of the same scene: without the tie's primitive, face
/// only (edges disabled) and edge only (face hidden).
#[derive(Clone, Copy, Debug, Default)]
pub struct DriverControls<'a> {
    pub uncovered: Option<TieFrame<'a>>,
    pub face_only: Option<TieFrame<'a>>,
    pub edge_only: Option<TieFrame<'a>>,
}

/// Same-driver controls for both images plus the undefined loci computed from the geometry.
#[derive(Clone, Copy, Debug, Default)]
pub struct TieControls<'a> {
    pub rust: DriverControls<'a>,
    pub native: DriverControls<'a>,
    pub undefined: &'a [UndefinedLocus],
}

/// Grade `rust` against `native`: every differing pixel must lie on a proven tie and be shifted
/// at most one pixel; `exact_tie_violation(w, h, rust_frame, native_frame, &loci)`.
pub fn exact_tie_violation(
    width: usize,
    height: usize,
    rust: TieFrame,
    native: TieFrame,
    loci: &[TieLocus],
) -> Result<TieGrade, String> {
    let controls = TieControls::default();
    controlled_tie_violation(width, height, rust, native, loci, &controls)
}

/// `exact_tie_violation` plus the uncovered-surface and undefined-locus rules;
/// `controlled_tie_violation(w, h, rust, native, &loci, &controls)`.
pub fn controlled_tie_violation(
    width: usize,
    height: usize,
    rust: TieFrame,
    native: TieFrame,
    loci: &[TieLocus],
    controls: &TieControls,
) -> Result<TieGrade, String> {
    let image = TieImages::new(width, height, rust, native)?;
    for frame in controls.frames() {
        TieImages::new(width, height, frame, native)?;
    }
    let differing = image.differing();
    for &point in &differing {
        pixel_violation(&image, loci, controls, point)?;
    }
    Ok(TieGrade {
        differing: differing.len(),
        lit: image.native_lit(),
    })
}

/// The rule that explains one differing pixel, or the reason none does.
fn pixel_violation(
    image: &TieImages,
    loci: &[TieLocus],
    controls: &TieControls,
    (column, row): (usize, usize),
) -> Result<(), String> {
    let point = (column, row);
    if controls.undefined.iter().any(|u| u.contains(column, row)) {
        return image.either_control(controls, point);
    }
    if !on_tie(loci, point) {
        return Err(format!(
            "pixel ({column},{row}) differs off every proven tie {loci:?}; expected byte-exact"
        ));
    }
    if image.sample_within_one(point) || image.uncovers(controls, point) {
        return Ok(());
    }
    Err(format!(
        "pixel ({column},{row}) colour and depth have no match within 1 px; expected raster_shift"
    ))
}

impl<'a> TieControls<'a> {
    fn frames(&self) -> impl Iterator<Item = TieFrame<'a>> {
        [self.rust, self.native]
            .into_iter()
            .flat_map(|d| [d.uncovered, d.face_only, d.edge_only])
            .flatten()
    }
}

/// Grade a capture against native RGBA and little-endian depth bytes: exact, or every
/// difference a proven tie; `grade_native_capture(&capture, rgba, depth, &loci)`.
pub fn grade_native_capture(
    capture: &martensite_vulkan::FrameCapture,
    native_rgba: &[u8],
    native_depth: &[u8],
    loci: &[TieLocus],
) -> Result<TieGrade, String> {
    let depth: Vec<f32> = native_depth
        .as_chunks::<4>()
        .0
        .iter()
        .map(|bytes| f32::from_le_bytes(*bytes))
        .collect();
    let (width, height) = (capture.width as usize, capture.height as usize);
    let rust = TieFrame {
        rgba: &capture.rgba,
        depth: &capture.depth,
    };
    let native = TieFrame {
        rgba: native_rgba,
        depth: &depth,
    };
    exact_tie_violation(width, height, rust, native, loci)
}

fn on_tie(loci: &[TieLocus], (column, row): (usize, usize)) -> bool {
    loci.iter().any(|locus| locus.ties(column, row))
}

struct TieImages<'a> {
    width: usize,
    height: usize,
    rust: TieFrame<'a>,
    native: TieFrame<'a>,
}

impl<'a> TieImages<'a> {
    fn new(
        width: usize,
        height: usize,
        rust: TieFrame<'a>,
        native: TieFrame<'a>,
    ) -> Result<Self, String> {
        let pixels = width * height;
        let lengths = [rust.rgba.len(), native.rgba.len(), rust.depth.len() * 4];
        if lengths.iter().any(|length| *length != pixels * 4) || native.depth.len() != pixels {
            return Err(format!(
                "received RGBA/depth lengths {lengths:?}/{}; expected {} and {pixels} for {width}x{height}",
                native.depth.len(),
                pixels * 4
            ));
        }
        Ok(Self {
            width,
            height,
            rust,
            native,
        })
    }
    fn at(&self, image: &[u8], (column, row): (usize, usize)) -> [u8; 4] {
        let index = (row * self.width + column) * 4;
        [0, 1, 2, 3].map(|k| image[index + k])
    }
    fn same(
        &self,
        mine: TieFrame,
        theirs: TieFrame,
        at: (usize, usize),
        near: (usize, usize),
    ) -> bool {
        let depth =
            |frame: TieFrame, (column, row): (usize, usize)| frame.depth[row * self.width + column];
        self.at(mine.rgba, at) == self.at(theirs.rgba, near)
            && (depth(mine, at) - depth(theirs, near)).abs() <= DEPTH_GATE
    }
    fn background(&self) -> [u8; 4] {
        self.at(self.native.rgba, (0, 0))
    }
    fn differing(&self) -> Vec<(usize, usize)> {
        let points = (0..self.height).flat_map(|row| (0..self.width).map(move |c| (c, row)));
        points
            .filter(|p| !self.same(self.rust, self.native, *p, *p))
            .collect()
    }
    fn native_lit(&self) -> usize {
        let background = self.background();
        let pixels = self.native.rgba.as_chunks::<4>().0.iter();
        pixels.filter(|pixel| **pixel != background).count()
    }
    /// raster_shift's Hausdorff test: the lit sample at `point` occurs 8-adjacent in the other.
    fn sample_within_one(&self, point: (usize, usize)) -> bool {
        let native_lit = self.at(self.native.rgba, point) != self.background();
        let (mine, theirs) = if native_lit {
            (self.native, self.rust)
        } else {
            (self.rust, self.native)
        };
        self.near(point)
            .into_iter()
            .any(|p| self.same(mine, theirs, point, p))
    }
    /// One image shows its own uncovered capture, the other's primitive sample lies 8-adjacent.
    fn uncovers(&self, controls: &TieControls, point: (usize, usize)) -> bool {
        let shows = |mine: TieFrame, control: Option<TieFrame>| {
            control.is_some_and(|c| self.same(mine, c, point, point))
        };
        let shifted = |mine: TieFrame, theirs: TieFrame| {
            self.near(point)
                .into_iter()
                .any(|p| self.same(mine, theirs, point, p))
        };
        (shows(self.rust, controls.rust.uncovered) && shifted(self.native, self.rust))
            || (shows(self.native, controls.native.uncovered) && shifted(self.rust, self.native))
    }
    /// Both images equal their own face-only or edge-only capture at an undefined pixel.
    fn either_control(
        &self,
        controls: &TieControls,
        (column, row): (usize, usize),
    ) -> Result<(), String> {
        let point = (column, row);
        let explained = |mine: TieFrame, own: DriverControls| {
            [own.face_only, own.edge_only]
                .into_iter()
                .flatten()
                .any(|c| self.same(mine, c, point, point))
        };
        if explained(self.rust, controls.rust) && explained(self.native, controls.native) {
            return Ok(());
        }
        Err(format!(
            "pixel ({column},{row}) on an undefined locus equals neither its face-only nor its edge-only capture; expected one per driver"
        ))
    }
    fn near(&self, (column, row): (usize, usize)) -> Vec<(usize, usize)> {
        let (columns, rows) = (
            column.saturating_sub(1)..=column + 1,
            row.saturating_sub(1)..=row + 1,
        );
        rows.flat_map(|r| columns.clone().map(move |c| (c, r)))
            .filter(|(c, r)| *c < self.width && *r < self.height)
            .collect()
    }
}

#[cfg(test)]
#[path = "exact_tie_reference_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "exact_tie_controls_tests.rs"]
mod controls_tests;
