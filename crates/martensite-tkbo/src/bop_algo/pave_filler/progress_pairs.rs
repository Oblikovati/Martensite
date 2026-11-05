//! Ranged stages advance the actual candidate cursor only after the native poll and work.
use super::{BOPAlgoPaveFiller, missing_structure};
use crate::bop_ds::BOPDSIterator;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

impl BOPAlgoPaveFiller {
    /// Borrows the actual iterator so cancellation retains the current native pair.
    /// Example: `let pair = filler.progress_pair_cursor()?.value()?;` after polling.
    pub(super) fn progress_pair_cursor(&mut self) -> Result<&mut BOPDSIterator, Failure> {
        let cursor = self.iterator.as_mut().ok_or_else(missing_structure)?;
        Ok(cursor)
    }

    /// Starts the native pair traversal and returns its genuine candidate count.
    /// Example: `filler.initialize_progress_pairs(ShapeEnum::Face, ShapeEnum::Face)?`.
    pub(super) fn initialize_progress_pairs(
        &mut self,
        first: ShapeEnum,
        second: ShapeEnum,
    ) -> Result<usize, Failure> {
        let cursor = self.progress_pair_cursor()?;
        cursor.initialize(first, second);
        Ok(cursor.expected_length().max(0) as usize)
    }
}
