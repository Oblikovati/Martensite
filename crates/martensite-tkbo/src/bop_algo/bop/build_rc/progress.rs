//! Polls surround the actual source collection and degenerate-edge work, without publishing partial RC.
use super::*;
use crate::bop_algo::{ProgressCancellation, builder_area::progress::observe_user_break};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
impl BOPAlgoBOP {
    /// Select result splits with native poll sites; `bop.build_rc_progress_raw(ds, range, observed)?`.
    pub(in crate::bop_algo::bop) fn build_rc_progress_raw(
        &mut self,
        structure: &DataStructure,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::unnamed(range, 1., false);
        if self.operation == Operation::Fuse {
            return self.build_rc(structure);
        }
        if observe_user_break(self.builder().options(), &scope, observed) {
            return Ok(());
        }
        let elements = [
            group_elements(&self.provider.builder.arguments)?,
            group_elements(self.provider.tools())?,
        ];
        if observe_user_break(self.builder().options(), &scope, observed) {
            return Ok(());
        }
        self.select_rc_progress(structure, &elements, &scope, observed)
    }
    fn select_rc_progress(
        &mut self,
        structure: &DataStructure,
        elements: &[IndexedMapOfShape; 2],
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let mut check_edges = false;
        let groups = [
            self.element_splits(&elements[0], &mut check_edges)?,
            self.element_splits(&elements[1], &mut check_edges)?,
        ];
        let mut compound = Shape::default();
        make_container(ShapeEnum::Compound, &mut compound);
        self.select_splits(&groups, &mut compound)?;
        if check_edges {
            if observe_user_break(self.builder().options(), scope, observed) {
                return Ok(());
            }
            add_squat_degenerated_edges(structure, &mut compound)?;
        }
        self.rc = compound;
        Ok(())
    }
}
