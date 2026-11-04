//

//! The adaptive plotting of isos and edges, and the last pick of a drawable shape.
use martensite_raster_core::RasterDisplay;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkmath::{geom_abs::IsoType, gp::Point3};

/// `IsoRatio`: a chord is kept when the midpoint is at most 0.1% off.
const ISO_RATIO: f64 = 1.001;

/// `MaxPlotCount`: the bisection stops after this many calls of one plot (bad
/// curves and surfaces would otherwise recurse very deep).
const MAX_PLOT_COUNT: i32 = 5;

use crate::picks::ShapePickSink;

/// Draws to `point` and records `shape` at `(u, v)` when the pick hits; `halt` is set.
pub(crate) fn draw_and_check(
    display: &mut RasterDisplay<'_>,
    point: Point3,
    (shape, u, v): (&Shape, f64, f64),
    halt: &mut bool,
    pick_sink: &dyn ShapePickSink,
) {
    display.draw_to_3d(point);
    if display.has_picked() {
        pick_sink.record_pick(shape, u, v);
        *halt = true;
    }
}

/// One adaptive plot (`PlotIso` or `PlotEdge` with its `PlotCount`).
pub(crate) struct Plot<'a, 'd> {
    pub(crate) display: &'a mut RasterDisplay<'d>,
    pub(crate) shape: &'a Shape,
    pub(crate) halt: &'a mut bool,
    count: i32,
    pick_sink: &'a dyn ShapePickSink,
}

impl<'a, 'd> Plot<'a, 'd> {
    /// A plot with `PlotCount = 0`.
    pub(crate) fn new(
        display: &'a mut RasterDisplay<'d>,
        shape: &'a Shape,
        halt: &'a mut bool,
        pick_sink: &'a dyn ShapePickSink,
    ) -> Self {
        Self {
            display,
            shape,
            halt,
            count: 0,
            pick_sink,
        }
    }

    /// `PlotIso`: from `(u, v)` one `step` along the iso, bisected while the midpoint is
    /// off the chord (at most [`MAX_PLOT_COUNT`] calls).
    pub(crate) fn iso(
        &mut self,
        surface: &BRepAdaptorSurface,
        kind: IsoType,
        (u, v): (f64, f64),
        step: f64,
    ) -> Result<(), Failure> {
        self.count += 1;
        let along_v = kind == IsoType::IsoU;
        let points = sample_iso_points(surface, along_v, (u, v), step)?;
        let end = iso_end((u, v), along_v, step);
        if self.finish_if_straight(points, || end) {
            return Ok(());
        }
        self.iso(surface, kind, (u, v), step / 2.)?;
        let middle_start = iso_end((u, v), along_v, step / 2.);
        self.iso(surface, kind, middle_start, step / 2.)
    }

    /// `PlotEdge`: from `f` one `step` along the curve, bisected while the midpoint is
    /// off the chord (at most [`MAX_PLOT_COUNT`] calls).
    pub(crate) fn edge(
        &mut self,
        curve: &dyn Adaptor3dCurve,
        f: f64,
        step: f64,
    ) -> Result<(), Failure> {
        self.count += 1;
        let (left, middle, right) = (curve.d0(f)?, curve.d0(f + step / 2.)?, curve.d0(f + step)?);
        if self.finish_if_straight((left, middle, right), || (f + step, 0.0)) {
            return Ok(());
        }
        self.edge(curve, f, step / 2.)?;
        self.edge(curve, f + step / 2., step / 2.)
    }

    fn finish_if_straight(
        &mut self,
        (left, middle, right): (Point3, Point3, Point3),
        at: impl FnOnce() -> (f64, f64),
    ) -> bool {
        if self.count > MAX_PLOT_COUNT || is_straight(left, middle, right) {
            let (u, v) = at();
            draw_and_check(
                self.display,
                right,
                (self.shape, u, v),
                self.halt,
                self.pick_sink,
            );
            return true;
        }
        false
    }
}

/// `Pm.Distance(Pl) + Pm.Distance(Pr) <= IsoRatio * Pl.Distance(Pr)`.
fn is_straight(left: Point3, middle: Point3, right: Point3) -> bool {
    middle.distance(&left) + middle.distance(&right) <= ISO_RATIO * left.distance(&right)
}

fn sample_iso_points(
    surface: &BRepAdaptorSurface,
    along_v: bool,
    (u, v): (f64, f64),
    step: f64,
) -> Result<(Point3, Point3, Point3), Failure> {
    let at = |t: f64| {
        if along_v {
            surface.d0(u, v + t)
        } else {
            surface.d0(u + t, v)
        }
    };
    Ok((surface.d0(u, v)?, at(step / 2.)?, at(step)?))
}

fn iso_end((u, v): (f64, f64), along_v: bool, step: f64) -> (f64, f64) {
    if along_v {
        return (u, v + step);
    }
    (u + step, v)
}
