//! `BOPDSIteratorSI`: the broad phase of the self-interference check, which pairs the
//! sub-shapes of one argument with each other.
use super::{BOPDSIterator, DataStructure, PreparationKind, PreparationOptions};
use crate::bop_ds::ShapeInfo;
use crate::int_tools::Context;
use martensite_tkernel::standard::Failure;
use std::ops::{Deref, DerefMut};

/// The candidate pairs of the self-interference check, read through the `BOPDSIterator` API
/// (`Initialize`, `More`, `Next`, `Value`).
///
/// ```
/// use martensite_tkbo::bop_ds::BOPDSIteratorSI;
/// let mut iterator = BOPDSIteratorSI::new();
/// iterator.update_by_level_of_check(4);
/// assert_eq!(iterator.expected_length(), 0);
/// ```
pub struct BOPDSIteratorSI {
    iterator: BOPDSIterator,
}

impl Default for BOPDSIteratorSI {
    fn default() -> Self {
        Self::with_self_kind(BOPDSIterator::default())
    }
}

impl Deref for BOPDSIteratorSI {
    type Target = BOPDSIterator;
    fn deref(&self) -> &BOPDSIterator {
        &self.iterator
    }
}

impl DerefMut for BOPDSIteratorSI {
    fn deref_mut(&mut self) -> &mut BOPDSIterator {
        &mut self.iterator
    }
}

impl BOPDSIteratorSI {
    /// An iterator with no candidate pair (`BOPDSIteratorSI()`); `BOPDSIteratorSI::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Retains the nullable resource for self-interference preparation.
    /// `let iterator = BOPDSIteratorSI::with_allocator(None);`
    pub fn with_allocator(allocator: Option<super::BaseAllocator>) -> Self {
        Self::with_self_kind(BOPDSIterator::with_allocator(allocator))
    }

    fn with_self_kind(mut iterator: BOPDSIterator) -> Self {
        iterator.preparation_kind = PreparationKind::SelfInterference;
        Self { iterator }
    }

    /// Prepares self-interferences, including pairs inside a single source argument.
    /// `iterator.prepare(&structure)?;`
    pub fn prepare(&mut self, structure: &DataStructure) -> Result<(), Failure> {
        // Keep the SI default fuzzy argument while sharing tagged Intersect dispatch.
        self.prepare_with_context(
            structure,
            None,
            false,
            martensite_tkernel::precision::confusion(),
        )
    }

    /// Selects the interfering pairs among all sub-shapes of the argument, the same range
    /// included (`Prepare` with the overridden `Intersect`); a pair of a shape and one of its
    /// own sub-shapes is skipped, and with `check_obb` a pair whose oriented boxes are apart;
    /// `iterator.prepare_with_context(&ds, Some(&context), false, 0.0)?`.
    pub fn prepare_with_context(
        &mut self,
        structure: &DataStructure,
        context: Option<&Context>,
        check_obb: bool,
        fuzzy: f64,
    ) -> Result<(), Failure> {
        self.iterator
            .prepare_with_context(structure, context, check_obb, fuzzy)
    }

    /// Clears the candidate lists of the interference kinds above `level` (0 VV .. 9 ZZ)
    /// (`UpdateByLevelOfCheck`); `iterator.update_by_level_of_check(4)`.
    pub fn update_by_level_of_check(&mut self, level: i32) {
        let first_cleared = (level + 1).max(0) as usize;
        for list in self.iterator.lists.iter_mut().skip(first_cleared) {
            list.clear(false);
        }
    }

    /// Moves the polymorphic iterator used by the pave filler, retaining self preparation.
    /// This corresponds to the checker pointer upcast, not reference base-value slicing.
    /// `filler_iterator = iterator.into_iterator()`.
    pub fn into_iterator(self) -> BOPDSIterator {
        self.iterator
    }
}

// All entry points use the same traversal so dispatch cannot alter ordering or failure phases.
pub(super) fn intersect_self_interferences(
    iterator: &mut BOPDSIterator,
    structure: &DataStructure,
    options: PreparationOptions<'_>,
) -> Result<(), Failure> {
    for pair in iterator.candidate_pairs(structure, ShapeInfo::is_interfering)? {
        append_self_pair(iterator, structure, pair, options)?;
    }
    Ok(())
}

fn append_self_pair(
    iterator: &mut BOPDSIterator,
    structure: &DataStructure,
    pair: super::Pair,
    options: PreparationOptions<'_>,
) -> Result<(), Failure> {
    let (first, second) = pair.indices();
    if iterator.is_contained_pair(structure, first, second)?
        || iterator.is_out_obb(structure, first, second, options)?
    {
        return Ok(());
    }
    iterator.add_typed_pair(structure, pair)
}
