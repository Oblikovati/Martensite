//! Each planar task owns one assigned range; no reusable builder stores or shares it.
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, ProgressOperation, ProgressStage, ProgressWorksite,
    run_context_tasks,
};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topods::{Edge, Face},
};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::{Failure, Handle},
};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tktopalgo::brep_lib::pcurve_for_edge_on_plane;

#[cfg(test)]
#[path = "task_contract.rs"]
mod contract;

pub(super) struct PlanarPcurveInput {
    pub(super) edge: Edge,
    pub(super) face: Face,
}

pub(super) struct PlanarPcurveWork<'p> {
    ordinal: usize,
    input: PlanarPcurveInput,
    curve: Handle<dyn Geom2dCurve>,
    to_update: bool,
    progress: Option<ProgressRange<'p>>,
    completion: Option<ProgressCompletion>,
}

impl<'p> PlanarPcurveWork<'p> {
    fn new(ordinal: usize, input: PlanarPcurveInput, range: ProgressRange<'p>) -> Self {
        Self {
            ordinal,
            input,
            curve: Handle::default(),
            to_update: false,
            progress: Some(range),
            completion: None,
        }
    }

    fn take_range(&mut self) -> Result<ProgressRange<'p>, Failure> {
        self.progress.take().ok_or_else(|| {
            Failure::new(&format!(
                "PlanarPCurve work item {}, expected one unconsumed worker range",
                self.ordinal
            ))
        })
    }

    fn perform(&mut self, observed: &ProgressCancellation) -> Result<(), Failure> {
        let range = self.take_range()?;
        let scope = ProgressScope::unnamed(&range, 1.0, false);
        if !scope.more() {
            observed.record_cancelled();
            self.completion = Some(cancelled_planar_work(self.ordinal));
            return Ok(());
        }
        let (curve, to_update) = pcurve_for_edge_on_plane(&self.input.edge, &self.input.face)?;
        self.curve = curve;
        self.to_update = to_update;
        self.completion = Some(ProgressCompletion::Completed);
        Ok(())
    }

    /// Require a real terminal callback; `work.take_completion()?` precedes parent publication.
    pub(super) fn take_completion(&mut self) -> Result<ProgressCompletion, Failure> {
        self.completion.take().ok_or_else(|| {
            Failure::new(&format!(
                "PlanarPCurve work item {}, expected one actual terminal callback",
                self.ordinal
            ))
        })
    }

    /// Keep native deferred edge mutation; `work.store_curve()?` follows the actual parent poll.
    pub(super) fn store_curve(&self) -> Result<(), Failure> {
        if !self.to_update {
            return Ok(());
        }
        let tolerance = BRepTool::edge_tolerance(&self.input.edge)?;
        BRepBuilder::default().update_edge_pcurve_on_face(
            &self.input.edge,
            &self.curve,
            &self.input.face,
            tolerance,
        )?;
        Ok(())
    }
}

/// Execute the native pure-For policy; `execute_planar_inputs(inputs, scope, parallel, observed)?`.
pub(super) fn execute_planar_inputs<'p>(
    inputs: Vec<PlanarPcurveInput>,
    scope: &'p ProgressScope<'p>,
    parallel: bool,
    observed: &ProgressCancellation,
) -> Result<Vec<PlanarPcurveWork<'p>>, Failure> {
    let mut work = reserved_planar_work(inputs, scope);
    // OSDParallel pure For keeps serial and singleton failures unwrapped.
    let pooled = parallel && work.len() != 1;
    run_context_tasks(
        &mut work,
        &(),
        pooled,
        || (),
        |item, _| item.perform(observed),
    )?;
    Ok(work)
}

fn reserved_planar_work<'p>(
    inputs: Vec<PlanarPcurveInput>,
    scope: &'p ProgressScope<'p>,
) -> Vec<PlanarPcurveWork<'p>> {
    inputs
        .into_iter()
        .enumerate()
        .map(|(ordinal, input)| PlanarPcurveWork::new(ordinal, input, scope.next(1.0)))
        .collect()
}

fn cancelled_planar_work(ordinal: usize) -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::PaveFiller,
        stage: ProgressStage::Preparation,
        context: Some(ProgressWorksite {
            label: "PlanarPCurve",
            input_ordinal: Some(ordinal),
            entity_key: None,
        }),
    }
}
