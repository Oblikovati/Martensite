//! Classify each real loop once; cancellation retains the actual partial internal state.
use super::FaceSupport;
use crate::bop_algo::{ProgressCancellation, builder_area::progress::observe_user_break};
use crate::{bop_algo::Options, int_tools::Context};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp,
    topods::{Face, Shape, ShapeIterator},
    toptools::ShapeMapHasher,
};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    ncollection::{IndexedMap, List},
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;

struct LoopClassification {
    growths: Vec<Face>,
    holes: IndexedMap<Shape, ShapeMapHasher>,
    hole_edges: IndexedMap<Shape, ShapeMapHasher>,
}

impl LoopClassification {
    fn new() -> Self {
        Self {
            growths: Vec::new(),
            holes: IndexedMap::with_hasher(ShapeMapHasher, 1),
            hole_edges: IndexedMap::with_hasher(ShapeMapHasher, 1),
        }
    }

    fn classify(
        &mut self,
        wire: &Shape,
        support: &FaceSupport,
        context: &Context,
    ) -> Result<(), Failure> {
        let face = draft_face(wire, support)?;
        if self.is_growth(wire, &face, context)? {
            self.growths.push(face);
        } else {
            self.holes.add(face.into_shape());
            topexp::map_shapes_of_type(wire, ShapeEnum::Edge, &mut self.hole_edges)?;
        }
        Ok(())
    }
    fn is_growth(&self, wire: &Shape, face: &Face, context: &Context) -> Result<bool, Failure> {
        if is_growth_wire(wire, &self.hole_edges) {
            return Ok(true);
        }
        // The fast check gave no result: run the classification.
        Ok(!context.fclass2d(face)?.read().is_hole())
    }
}

fn draft_face(wire: &Shape, support: &FaceSupport) -> Result<Face, Failure> {
    let mut face = support.make_face()?.into_shape();
    BRepBuilder::default().add(&mut face, wire)?;
    Ok(Face::unchecked(face))
}

/// Classify each actual loop; `classify_loops(loops, support, context, range, options, observed)?`.
pub(super) fn classify_loops(
    loops: &List<Shape>,
    support: &FaceSupport,
    context: &Context,
    range: &ProgressRange<'_>,
    options: &mut Options,
    observed: &ProgressCancellation,
) -> Result<(Vec<Face>, IndexedMap<Shape, ShapeMapHasher>), Failure> {
    let mut classified = LoopClassification::new();
    let scope = ProgressScope::with_literal(range, "Making faces", loops.len() as f64, false);
    for wire in loops.iter() {
        if observe_user_break(options, &scope, observed) {
            break;
        }
        classified.classify(wire, support, context)?;
        drop(scope.next(1.));
    }
    Ok((classified.growths, classified.holes))
}

/// Whether the wire shares an edge with a hole face (`IsGrowthWire`).
fn is_growth_wire(wire: &Shape, hole_edges: &IndexedMap<Shape, ShapeMapHasher>) -> bool {
    hole_edges.extent() > 0
        && ShapeIterator::new(wire, true, true).any(|edge| hole_edges.contains(&edge))
}
