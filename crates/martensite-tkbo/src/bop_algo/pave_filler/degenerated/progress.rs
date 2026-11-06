//! Degenerate processing polls after each source edge, except the native empty-list continuation.
use super::super::progress_observation::native_user_break;
use super::*;
use crate::bop_algo::ProgressCancellation;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
impl BOPAlgoPaveFiller {
    /// Preserve post-work cancellation and diagnostic modifications at native source-edge positions.
    /// Example: `filler.process_de_with_progress(range, observed)?` after pcurve creation.
    pub(in crate::bop_algo::pave_filler) fn process_de_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::unnamed(range, 1., false);
        let count = self.source_shape_count()?;
        for edge in 0..count {
            if !self.is_source_edge(edge)? || !self.process_de_source(edge)? {
                continue;
            }
            if native_user_break(&self.options, &scope, observed) {
                return Ok(());
            }
        }
        Ok(())
    }
    fn source_shape_count(&self) -> Result<i32, Failure> {
        Ok(self
            .data_structure
            .as_ref()
            .ok_or_else(missing_structure)?
            .nb_source_shapes())
    }
    fn is_source_edge(&self, index: i32) -> Result<bool, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        Ok(structure.shape_info(index)?.shape_type() == ShapeEnum::Edge)
    }
    fn process_de_source(&mut self, edge: i32) -> Result<bool, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let info = structure.shape_info(edge)?;
        let mut owner = 0;
        if !info.has_flag_into(&mut owner) {
            return Ok(true);
        }
        let vertex = structure.same_domain_index(info.sub_shapes()[0]);
        match structure.shape_info(owner)?.shape_type() {
            ShapeEnum::Face => self.process_face_de(edge, owner, vertex),
            ShapeEnum::Edge => {
                self.process_edge_de(edge, vertex)?;
                Ok(true)
            }
            _ => Ok(true),
        }
    }
}
#[cfg(test)]
#[path = "contract.rs"]
mod tests;

#[cfg(test)]
#[path = "census.rs"]
mod census;
#[cfg(test)]
#[path = "fixture.rs"]
mod fixture;
