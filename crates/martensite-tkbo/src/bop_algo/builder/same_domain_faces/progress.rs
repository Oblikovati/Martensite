//! Exact native raw merge order is independent of retained typed cancellation evidence.
use super::{
    BOPAlgoBuilder, DataStructure, ShapeMapHasher, fill_map, make_blocks, preparation, tasks,
};
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, ProgressOperation, ProgressStage,
    builder_area::progress::observe_user_break,
};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};
use preparation::{SameDomainLinks, SameDomainPreparation};

impl BOPAlgoBuilder {
    /// Execute the real SD stage; `builder.fill_same_domain_faces_progress_raw(ds, range, observed)?`.
    /// Raw diagnostic bindings may remain on cancellation; this completion never certifies publication.
    pub(crate) fn fill_same_domain_faces_progress_raw(
        &mut self,
        structure: &DataStructure,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        if structure.interf_ff_records().is_empty() {
            return Ok(same_domain_completion(observed));
        }
        let scope = ProgressScope::unnamed(range, 10.0, false);
        if let Some(prepared) = preparation::prepare(self, structure, &scope, observed)? {
            self.run_same_domain_pairs(structure, prepared, &scope, observed)?;
        }
        Ok(same_domain_completion(observed))
    }

    fn run_same_domain_pairs(
        &mut self,
        structure: &DataStructure,
        mut prepared: SameDomainPreparation,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        drop(scope.next(1.0));
        let checking = tasks::checking_scope(scope, prepared.pairs.len());
        let checked = self.check_prepared_pairs(prepared.pairs, &checking, observed)?;
        if observe_user_break(self.options(), scope, observed) {
            return Ok(());
        }
        for pair in checked {
            if pair.same_domain {
                fill_map(&mut prepared.links, pair.first, pair.second);
            }
        }
        self.bind_same_domain_links(structure, prepared.links, scope, observed)
    }

    fn check_prepared_pairs(
        &self,
        pairs: Vec<(super::Shape, super::Shape)>,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<tasks::CheckedSameDomainPair>, Failure> {
        tasks::check_pairs(
            pairs,
            &self.context,
            self.options().run_parallel(),
            self.options().fuzzy_value(),
            scope,
            observed,
        )
    }

    fn bind_same_domain_links(
        &mut self,
        structure: &DataStructure,
        links: SameDomainLinks,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let groups = make_blocks(&links, ShapeMapHasher)?;
        let scope = same_domain_binding_scope(outer, groups.len());
        for group in groups {
            if observe_user_break(self.options(), &scope, observed) {
                return Ok(());
            }
            self.bind_same_domain_group(structure, &group);
            drop(scope.next(1.0));
        }
        self.update_face_images_with_same_domain(structure)
    }
}

fn same_domain_completion(observed: &ProgressCancellation) -> ProgressCompletion {
    if !observed.is_cancelled() {
        return ProgressCompletion::Completed;
    }
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::Builder,
        stage: ProgressStage::Images,
        context: None,
    }
}

fn same_domain_binding_scope<'a>(outer: &'a ProgressScope<'a>, count: usize) -> ProgressScope<'a> {
    let range = outer.next(3.0);
    ProgressScope::with_literal(&range, "Filling same domain faces map", count as f64, false)
}
