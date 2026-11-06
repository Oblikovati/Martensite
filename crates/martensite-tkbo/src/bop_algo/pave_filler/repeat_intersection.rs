//! Repeat only increased source vertices, lending the native three child ranges.
use super::{BOPAlgoPaveFiller, missing_structure};
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, ProgressOperation,
    builder_area::progress::classify_area_progress,
};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    ncollection::Map,
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;

impl BOPAlgoPaveFiller {
    /// Repeats vertex intersections after source or same-domain tolerances grew.
    /// Example: `filler.repeat_intersection()?` preserves the original unbound contract.
    pub fn repeat_intersection(&mut self) -> Result<(), Failure> {
        self.repeat_intersection_with_progress(
            &ProgressRange::default(),
            &ProgressCancellation::default(),
        )
        .map(|_| ())
    }

    /// Retain real child completion while forwarding native VV/VE/VF ranges.
    /// Example: `filler.repeat_intersection_with_progress(range, observed)?` before forced intersections.
    pub(super) fn repeat_intersection_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let scope = ProgressScope::with_literal(range, "Repeat intersection", 3., false);
        let extra = self.increased_source_vertices()?;
        if extra.extent() == 0 {
            return Ok(self.repeat_completion(observed));
        }
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        self.iterator
            .as_mut()
            .ok_or_else(missing_structure)?
            .intersect_extended(structure, &extra)?;
        self.repeat_vertex_stages(&scope, observed)
    }

    fn increased_source_vertices(&self) -> Result<Map<i32>, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let mut extra = Map::new(1);
        for index in 0..structure.nb_source_shapes() {
            if structure.shape_info(index)?.shape_type() != ShapeEnum::Vertex {
                continue;
            }
            let mut representative = index;
            let increased = self.increased_ss.contains(&index)
                || (structure.has_shape_sd_into(index, &mut representative)
                    && self.increased_ss.contains(&representative));
            if increased {
                extra.add(index);
            }
        }
        Ok(extra)
    }

    fn repeat_vertex_stages(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        self.perform_vv_with_progress(&scope.next(1.), observed)?;
        if self.options.has_errors() {
            return Ok(self.repeat_completion(observed));
        }
        self.update_pave_blocks_with_sd_vertices()?;
        let outcomes = self.perform_ve_with_progress(&scope.next(1.), observed)?;
        let completion = retain_repeat_outcomes(ProgressCompletion::Completed, outcomes, observed);
        if self.options.has_errors() {
            return Ok(self.finish_repeat(completion, observed));
        }
        self.repeat_face_stage(scope, observed, completion)
    }

    fn repeat_face_stage(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
        completion: ProgressCompletion,
    ) -> Result<ProgressCompletion, Failure> {
        self.update_pave_blocks_with_sd_vertices()?;
        let outcomes = self.perform_vf_with_progress(&scope.next(1.), observed)?;
        let completion = retain_repeat_outcomes(completion, outcomes, observed);
        if self.options.has_errors() {
            return Ok(self.finish_repeat(completion, observed));
        }
        self.update_pave_blocks_with_sd_vertices()?;
        self.update_interfs_with_sd_vertices()?;
        Ok(self.finish_repeat(completion, observed))
    }

    fn finish_repeat(
        &self,
        completion: ProgressCompletion,
        observed: &ProgressCancellation,
    ) -> ProgressCompletion {
        if matches!(completion, ProgressCompletion::Completed) {
            return self.repeat_completion(observed);
        }
        completion
    }

    fn repeat_completion(&self, observed: &ProgressCancellation) -> ProgressCompletion {
        classify_area_progress(
            ProgressOperation::PaveFiller,
            &self.options,
            Ok(()),
            observed,
        )
    }
}

fn retain_repeat_outcomes(
    previous: ProgressCompletion,
    mut outcomes: Vec<ProgressCompletion>,
    observed: &ProgressCancellation,
) -> ProgressCompletion {
    outcomes.insert(0, previous);
    for outcome in &outcomes {
        if matches!(outcome, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
    }
    outcomes
        .iter()
        .find(|outcome| matches!(outcome, ProgressCompletion::Failed { .. }))
        .or_else(|| {
            outcomes
                .iter()
                .find(|outcome| matches!(outcome, ProgressCompletion::Incomplete { .. }))
        })
        .or_else(|| {
            outcomes
                .iter()
                .find(|outcome| matches!(outcome, ProgressCompletion::Cancelled { .. }))
        })
        .cloned()
        .unwrap_or(ProgressCompletion::Completed)
}

#[cfg(test)]
#[path = "repeat_intersection/contract.rs"]
mod tests;

#[cfg(test)]
#[path = "repeat_intersection/census.rs"]
mod census;
