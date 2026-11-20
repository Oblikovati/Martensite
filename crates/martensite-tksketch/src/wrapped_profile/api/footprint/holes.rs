//! Exclude the entire certified envelope of every bounded inner native trim wire.
use super::*;
use band::BandFailure;
use martensite_tkbo::bop_tools::BoxSelector;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::bvh::{BvhBox, BvhBoxSet, BvhSetTraverse, BvhVec2d};
use martensite_tktopalgo::brep_check::{BRepCheckStatus, BRepCheckWire};
mod curves;
mod periodic;
type Checked<T> = Result<T, BandFailure>;

/// Axis order is angle in radians followed by the meridian in model units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct TrimBox(pub [[f64; 2]; 2]);
impl TrimBox {
    pub const fn empty() -> Self {
        Self([[f64::INFINITY, f64::NEG_INFINITY]; 2])
    }
    pub fn include(&mut self, point: [f64; 2]) {
        for ([lower, upper], value) in self.0.iter_mut().zip(point) {
            *lower = lower.min(value);
            *upper = upper.max(value);
        }
    }
    fn include_box(&mut self, bounds: Self) {
        let [[u0, u1], [v0, v1]] = bounds.0;
        self.include([u0, v0]);
        self.include([u1, v1]);
    }
    fn finite(self) -> bool {
        self.0
            .iter()
            .all(|[a, b]| a.is_finite() && b.is_finite() && a <= b)
    }
    fn strictly_inside(self, limits: [[f64; 2]; 2]) -> bool {
        self.finite()
            && self
                .0
                .into_iter()
                .zip(limits)
                .all(|([low, high], [min, max])| min < low && low <= high && high < max)
    }
}

pub(super) fn capture(
    face: &Face,
    inner: &[Shape],
    limits: [[f64; 2]; 2],
) -> Checked<Vec<TrimBox>> {
    inner
        .iter()
        .map(|wire| {
            let bounds = wire_bounds(face, wire)?;
            if !bounds.strictly_inside(limits) {
                return Err(unsupported(
                    "inner trim pcurve hull strictly inside one outer UV lift",
                ));
            }
            Ok(bounds)
        })
        .collect()
}

fn wire_bounds(face: &Face, wire: &Shape) -> Checked<TrimBox> {
    let wire = native(topods::wire(wire))?;
    let checker = native(BRepCheckWire::new(&wire))?;
    if native(checker.closed2d(face, false))? != BRepCheckStatus::NoError {
        return Err(unsupported("complete closed inner trim wire in UV"));
    }
    let mut bounds = TrimBox::empty();
    let mut edges = native(Explorer::new(&wire, ShapeEnum::Edge, ShapeEnum::Shape))?;
    while edges.more() {
        bounds.include_box(edge_bounds(face, edges.current())?);
        native(edges.next())?;
    }
    Ok(bounds)
}

fn edge_bounds(face: &Face, current: &Shape) -> Checked<TrimBox> {
    let edge = native(topods::edge(current))?;
    if !matches!(
        edge.orientation(),
        Orientation::Forward | Orientation::Reversed
    ) || band::stored_seam(face, &edge)?
    {
        return Err(unsupported(
            "ordinary oriented inner trim edge with one stored pcurve",
        ));
    }
    let stored = native(BRepTool::curve_on_face(&edge, face))?;
    if !stored.is_stored
        || !stored.first.is_finite()
        || !stored.last.is_finite()
        || stored.first >= stored.last
    {
        return Err(unsupported(
            "stored inner trim with a finite increasing interval",
        ));
    }
    curves::bounds(&stored.pcurve, [stored.first, stored.last])
}

pub(super) struct Exclusions(Option<BvhBoxSet<BvhVec2d>>);
impl Exclusions {
    pub fn new(holes: &[TrimBox]) -> Checked<Self> {
        if holes.is_empty() {
            return Ok(Self(None));
        }
        let mut tree = BvhBoxSet::default();
        for (index, bounds) in holes.iter().enumerate() {
            let wire = i32::try_from(index + 1)
                .map_err(|_| unsupported("representable inner wire ordinal"))?;
            for image in periodic::images(*bounds)? {
                tree.add(wire, image);
            }
        }
        native(tree.build())?;
        Ok(Self(Some(tree)))
    }
    pub fn present(&self) -> bool {
        self.0.is_some()
    }
    pub fn overlap(&mut self, bounds: TrimBox) -> Checked<Option<usize>> {
        let Some(tree) = &mut self.0 else {
            return Ok(None);
        };
        let mut selector = BoxSelector::default();
        selector.set_bvh_set(Some(tree));
        let mut first = None;
        for image in periodic::images(bounds)? {
            selector.clear();
            selector.set_box(image);
            native(selector.select())?;
            first = first
                .into_iter()
                .chain(selector.indices().iter().map(|id| *id as usize))
                .min();
        }
        Ok(first)
    }
}

fn native<T>(value: Result<T, Failure>) -> Checked<T> {
    value.map_err(|cause| BandFailure::Native(Box::new(cause)))
}
fn unsupported(expected: &'static str) -> BandFailure {
    BandFailure::Unsupported(expected)
}
