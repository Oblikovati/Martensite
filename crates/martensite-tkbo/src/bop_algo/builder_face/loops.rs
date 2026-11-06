//! Boundary and internal wires retain the same source edge order and walk.
use super::BuilderFace;
use crate::bop_algo::ProgressCancellation;
use crate::bop_algo::builder_area::progress::observe_user_break;
use crate::bop_algo::{WireEdgeSet, WireSplitter};
#[path = "internal_loop_walk.rs"]
mod internal_loop_walk;
use internal_loop_walk::InternalWireWalk;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    ncollection::Map,
    standard::Failure,
};

impl BuilderFace {
    /// Builds the boundary loops with the wire splitter and the internal loops from the
    /// edges left over (`PerformLoops`).
    /// Example: `receiver.perform_loops(range, observed)` through the actual-H stage hook.
    pub(super) fn perform_loops(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::with_literal(range, "Making wires", 10., false);
        self.area.loops.clear();
        let mut splitter = self.boundary_splitter();
        splitter.perform_observed(&scope.next(9.), observed)?;
        if splitter.options().has_errors() {
            return Ok(());
        }
        self.append_boundary_wires(&splitter);
        self.post_boundary_wires(&scope, observed)
    }

    fn post_boundary_wires(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        self.avoid_unused_edges(scope, observed);
        if self.area.options.has_errors() {
            return Ok(());
        }
        self.make_internal_loops(scope, observed)
    }

    fn boundary_splitter(&self) -> WireSplitter {
        let mut wes = WireEdgeSet::new();
        wes.set_face(&self.face);
        for edge in self.area.shapes.iter() {
            if !self.area.shapes_to_avoid.contains(edge) {
                wes.add_start_element(edge);
            }
        }
        let mut splitter = WireSplitter::new();
        splitter.set_wes(wes);
        splitter
            .options_mut()
            .set_run_parallel(self.area.options.run_parallel());
        splitter.set_context(self.area.context.clone());
        splitter
    }

    fn append_boundary_wires(&mut self, splitter: &WireSplitter) {
        if let Some(wes) = splitter.wes() {
            for wire in wes.shapes().iter() {
                self.area.loops.append(wire.clone());
            }
        }
    }

    /// Adds to the shapes to avoid every edge in neither a loop nor the shapes to avoid.
    fn avoid_unused_edges(&mut self, scope: &ProgressScope<'_>, observed: &ProgressCancellation) {
        let mut used: Map<Shape> = Map::new(1);
        for wire in self.area.loops.iter() {
            for edge in ShapeIterator::new(wire, true, true) {
                used.add(edge);
            }
        }
        if observe_user_break(&self.area.options, scope, observed) {
            return;
        }
        for edge in self.area.shapes_to_avoid.iter() {
            used.add(edge.clone());
        }
        for edge in self.area.shapes.iter() {
            if !used.contains(edge) {
                self.area.shapes_to_avoid.add(edge.clone());
            }
        }
        observe_user_break(&self.area.options, scope, observed);
    }

    /// Groups the edges to avoid into connected internal wires (`myLoopsInternal`).
    fn make_internal_loops(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        self.area.loops_internal.clear();
        let avoid: Vec<Shape> = self.area.shapes_to_avoid.iter().cloned().collect();
        let mut walk = InternalWireWalk::new(&avoid)?;
        let loops = &mut self.area.loops_internal;
        for edge in &avoid {
            if !walk.append_seed(edge, loops, scope, &self.area.options, observed)? {
                break;
            }
        }
        Ok(())
    }
}
