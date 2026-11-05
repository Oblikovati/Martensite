//! Real Wire stages keep cancellation polls at the native algorithm boundaries.
use super::WireSplitter;
use crate::bop_algo::builder_area::progress::observe_user_break;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use crate::bop_tools::{ConnexityBlock, make_connexity_blocks_from_shapes};
use crate::int_tools::Context;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::ShapeEnum;

#[path = "dispatch.rs"]
mod dispatch;
mod extension_hooks;
mod extension_status;
mod operation_context;
mod prepared_extension;
pub use extension_hooks::WireSplitterHooks;
pub use operation_context::WireOperationContext;
pub use prepared_extension::WirePreparedExtension;

impl WireSplitter {
    /// Execute this concrete wire receiver with progress, returning raw execution completion.
    /// Example: `let status = splitter.perform_with_progress(&indicator.start());`.
    /// `Completed` does not validate topology, geometry, tolerance consistency, or publication.
    /// Cancelled/failed/incomplete raw edge-set state remains diagnostic, not successful output.
    pub fn perform_with_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        // Keep the inherent and extension entries on the same real receiver path.
        dispatch::perform_wire_with_progress(self, range)
    }

    /// Propagate a caller-owned observation without adding task alerts; `splitter.perform_observed(range, observed)`.
    pub(crate) fn perform_observed(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let operation = WireOperationContext::new();
        dispatch::perform_wire_observed(self, range, &operation, observed)
    }

    fn prepare_blocks(&mut self) -> Result<(), Failure> {
        if self.context.is_null() {
            self.context = Handle::new(Context::default());
        }
        let start: Vec<Shape> = match &self.wes {
            Some(wes) => wes.start_elements().iter().cloned().collect(),
            None => return Ok(()),
        };
        self.blocks.clear();
        make_connexity_blocks_from_shapes(
            &start,
            ShapeEnum::Vertex,
            ShapeEnum::Edge,
            &mut self.blocks,
        )?;
        Ok(())
    }

    /// Turns each connexity block into wires (`MakeWires`): a regular block is one wire, the
    /// others go through [`WireSplitter::split_block`].
    fn make_wires(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::unnamed(range, 1., false);
        let mut irregular = self.regular_blocks(&scope, observed)?;
        if self.options.has_errors() {
            return Ok(());
        }
        self.split_collected_blocks(&mut irregular, &scope.next(1.), observed)?;
        self.publish_block_loops(&irregular);
        Ok(())
    }

    fn split_collected_blocks(
        &self,
        blocks: &mut [ConnexityBlock],
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let Some(wes) = self.wes.as_ref() else {
            return Ok(());
        };
        super::tasks::split_irregular(
            wes.face(),
            blocks,
            &self.context,
            self.options.run_parallel(),
            range,
            observed,
        )
    }

    fn publish_block_loops(&mut self, blocks: &[ConnexityBlock]) {
        let Some(wes) = self.wes.as_mut() else {
            return;
        };
        for block in blocks {
            for wire in block.loops().iter() {
                wes.add_shape(wire);
            }
        }
    }

    fn regular_blocks(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ConnexityBlock>, Failure> {
        let Some(wes) = self.wes.as_mut() else {
            return Ok(Vec::new());
        };
        let mut irregular: Vec<ConnexityBlock> = Vec::new();
        for block in self.blocks.iter_mut() {
            if observe_user_break(&self.options, scope, observed) {
                return Ok(Vec::new());
            }
            append_regular_block(wes, block, &mut irregular)?;
        }
        Ok(irregular)
    }
}

fn append_regular_block(
    wes: &mut crate::bop_algo::WireEdgeSet,
    block: &mut ConnexityBlock,
    irregular: &mut Vec<ConnexityBlock>,
) -> Result<(), Failure> {
    if block.is_regular() {
        let wire = WireSplitter::make_wire(block.shapes())?;
        wes.add_shape(&wire.into_shape());
    } else {
        irregular.push(block.clone());
    }
    Ok(())
}

#[cfg(test)]
#[path = "progress_contract.rs"]
mod contract;
#[cfg(test)]
#[path = "progress_fixture.rs"]
mod fixture;
#[cfg(test)]
#[path = "progress_hooks.rs"]
mod hooks;

#[cfg(test)]
#[path = "failure_contract.rs"]
mod failure_contract;

#[cfg(test)]
mod graph_contract;
#[cfg(test)]
mod observations;
#[cfg(test)]
mod scope_contract;
