//

//! `TopologyHideData`: the visible and hidden segments of a shape in one view, from the
//! polyhedral hidden-line removal.
use martensite_raster_core::{PaletteColor, RasterDisplay};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkhlr::hlr_algo::{EdgeIterator, EdgeStatus, Projector};
use martensite_tkhlr::hlr_brep::{BiPntFlags, BiPoint, PolyAlgo, PolyAlgoSegment};
use martensite_tkmath::gp::{Transform3, Xyz};

/// `TopologyHideData`: the hidden-line result of one view (its projection, focal
/// distance and discretisation angle) and the shape the last pick touched.
#[derive(Debug, Clone)]
pub struct HiddenLineRecords {
    view: i32,
    trsf: Transform3,
    focal: f64,
    visible: Vec<BiPoint>,
    hidden: Vec<BiPoint>,
    pick_shape: Shape,
    angle: f64,
}

impl Default for HiddenLineRecords {
    /// `TopologyHideData`: no view (-1), focal and angle 0.
    fn default() -> Self {
        Self {
            view: -1,
            trsf: Transform3::default(),
            focal: 0.0,
            visible: Vec::new(),
            hidden: Vec::new(),
            pick_shape: Shape::default(),
            angle: 0.0,
        }
    }
}

impl HiddenLineRecords {
    /// `Set(ViewId, TProj, Focal, S, ang)`: runs `HLRBRepPolyAlgo` on `shape` with the
    /// projector of the view (perspective when `focal > 0`) and keeps each visible
    /// and hidden part of each segment in model coordinates.
    /// Example: `records.set(1, (Transform3::default, 0.), &shape, 0.5)?`.
    pub fn set(
        &mut self,
        view: i32,
        (trsf, focal): (Transform3, f64),
        shape: &Shape,
        angle: f64,
    ) -> Result<(), Failure> {
        (self.view, self.trsf, self.focal, self.angle) = (view, trsf, focal, angle);
        let hider = PolyAlgo::with_shape(shape)?;
        hider.set_projector(&Projector::with_transformation(trsf, focal > 0., focal)?);
        hider.update()?;
        self.visible.clear();
        self.hidden.clear();
        self.load_hidden_parts(&hider)
    }

    fn load_hidden_parts(&mut self, hider: &PolyAlgo) -> Result<(), Failure> {
        hider.init_hide()?;
        while hider.more_hide() {
            let mut status = EdgeStatus::default();
            let segment = hider.hide(&mut status)?;
            self.add_parts(&segment, &status)?;
            hider.next_hide()?;
        }
        Ok(())
    }

    /// The visible then hidden parts of one segment, as `BiPoint`s between the
    /// parameters `sta` and `end` of the segment.
    fn add_parts(&mut self, segment: &PolyAlgoSegment, status: &EdgeStatus) -> Result<(), Failure> {
        let partition = SegmentPartition::new(segment);
        let mut parts = EdgeIterator::new();
        parts.init_visible(status);
        while parts.more_visible() {
            let (start, _, end, _) = parts.visible()?;
            self.visible.push(partition.part(start, end));
            parts.next_visible();
        }
        parts.init_hidden(status)?;
        while parts.more_hidden() {
            let (start, _, end, _) = parts.hidden();
            self.hidden.push(partition.part(start, end));
            parts.next_hidden()?;
        }
        Ok(())
    }

    /// `ViewId`; `records.view_id()` identifies the cached view.
    pub fn view_id(&self) -> i32 {
        self.view
    }

    /// `Angle`; `records.angle()` returns the recorded discretisation angle.
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// `IsSame(TProj, Focal)`: the same rotation (and, in perspective, the same focal
    /// distance and translation), compared exactly; `records.is_same(&projection, 0.)`.
    pub fn is_same(&self, trsf: &Transform3, focal: f64) -> bool {
        if focal > 0. {
            if self.focal <= 0. || self.focal != focal {
                return false;
            }
            if trsf.translation_part() != self.trsf.translation_part() {
                return false;
            }
        }
        // `HVectorialPart` is the unscaled matrix.
        trsf.matrix() == self.trsf.matrix()
    }

    /// `DrawOn(D, withRg1, withRgN, withHid, VisCol, HidCol)`: the hidden parts (when
    /// asked) in `HidCol`, then the visible ones in `VisCol`; smooth (Rg1) and
    /// seam-like (RgN) lines that are not outlines are left out unless asked. The
    /// first segment the pick touches gives `LastPick`.
    /// Example: `records.draw_on(&mut display, (true, true, false), colors)`.
    pub fn draw_on(
        &mut self,
        display: &mut RasterDisplay<'_>,
        (with_rg1, with_rgn, with_hid): (bool, bool, bool),
        (visible_color, hidden_color): (PaletteColor, PaletteColor),
    ) {
        let mut first_pick = true;
        let flags = (with_rg1, with_rgn);
        if with_hid {
            self.draw_records(display, true, flags, hidden_color, &mut first_pick);
        }
        self.draw_records(display, false, flags, visible_color, &mut first_pick);
    }

    fn draw_records(
        &mut self,
        display: &mut RasterDisplay<'_>,
        hidden: bool,
        flags: (bool, bool),
        color: PaletteColor,
        first_pick: &mut bool,
    ) {
        display.set_color(color);
        let parts = if hidden { &self.hidden } else { &self.visible };
        if let Some(shape) = draw_parts(display, parts, flags, first_pick) {
            self.pick_shape = shape;
        }
    }

    /// `LastPick`; `records.last_pick()` returns the first shape touched by the pick.
    pub fn last_pick(&self) -> &Shape {
        &self.pick_shape
    }
}

/// Draws the kept parts; the shape of the first one picked while `first_pick` holds.
fn draw_parts(
    display: &mut RasterDisplay<'_>,
    parts: &[BiPoint],
    flags: (bool, bool),
    first_pick: &mut bool,
) -> Option<Shape> {
    let mut picked = None;
    for part in parts {
        if let Some(shape) = draw_kept_part(display, part, flags, first_pick) {
            picked = Some(shape);
        }
    }
    picked
}

struct SegmentPartition<'a> {
    segment: &'a PolyAlgoSegment,
    p1: Xyz,
    delta: (f64, f64, f64),
    flags: BiPntFlags,
}

impl<'a> SegmentPartition<'a> {
    fn new(segment: &'a PolyAlgoSegment) -> Self {
        let (p1, p2) = (segment.points.pnt1, segment.points.pnt2);
        let delta = (p2.x() - p1.x(), p2.y() - p1.y(), p2.z() - p1.z());
        let flags = BiPntFlags {
            reg1: segment.reg1,
            regn: segment.regn,
            outl: segment.outl,
            intl: segment.intl,
        };
        Self {
            segment,
            p1,
            delta,
            flags,
        }
    }

    fn part(&self, start: f64, end: f64) -> BiPoint {
        let p1 = self.p1;
        let (dx, dy, dz) = self.delta;
        BiPoint::new(
            [
                p1.x() + start * dx,
                p1.y() + start * dy,
                p1.z() + start * dz,
            ],
            [p1.x() + end * dx, p1.y() + end * dy, p1.z() + end * dz],
            &self.segment.shape,
            self.flags,
        )
    }
}

fn draw_kept_part(
    display: &mut RasterDisplay<'_>,
    part: &BiPoint,
    (with_rg1, with_rgn): (bool, bool),
    first_pick: &mut bool,
) -> Option<Shape> {
    let skipped = (!with_rg1 && part.rg1_line() && !part.out_line())
        || (!with_rgn && part.rgn_line() && !part.out_line());
    if skipped {
        return None;
    }
    display.move_to_3d(part.p1());
    display.draw_to_3d(part.p2());
    if *first_pick && display.has_picked() {
        *first_pick = false;
        return Some(part.shape().clone());
    }
    None
}
