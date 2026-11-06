//! A scoped binding preserves source identity without extending either owner's lifetime.
use super::{BOPDSIterator, Context, DataStructure, Failure, ShapeEnum};

/// Exclusively borrows an iterator and its non-null source for one operation scope.
///
/// ```
/// use martensite_tkbo::bop_ds::{BOPDSIterator, DataStructure};
/// let mut iterator = BOPDSIterator::default();
/// let mut source = DataStructure::default();
/// let binding = iterator.bind_structure(&mut source);
/// assert_eq!(binding.structure().nb_shapes(), 0);
/// ```
/// Direct mutation of the source cannot overlap its binding:
///
/// ```compile_fail,E0499
/// use martensite_tkbo::bop_ds::{BOPDSIterator, DataStructure};
/// let mut iterator = BOPDSIterator::default();
/// let mut source = DataStructure::default();
/// let binding = iterator.bind_structure(&mut source);
/// source.clear();
/// assert_eq!(binding.structure().nb_shapes(), 0);
/// ```
/// Direct mutation of the iterator cannot overlap its binding:
///
/// ```compile_fail,E0499
/// use martensite_tkbo::bop_ds::{BOPDSIterator, DataStructure};
/// let mut iterator = BOPDSIterator::default();
/// let mut source = DataStructure::default();
/// let binding = iterator.bind_structure(&mut source);
/// iterator.set_run_parallel(true);
/// assert_eq!(binding.structure().nb_shapes(), 0);
/// ```
/// Moving the source cannot invalidate an active binding:
///
/// ```compile_fail,E0505
/// use martensite_tkbo::bop_ds::{BOPDSIterator, DataStructure};
/// let mut iterator = BOPDSIterator::default();
/// let mut source = DataStructure::default();
/// let binding = iterator.bind_structure(&mut source);
/// drop(source);
/// assert_eq!(binding.structure().nb_shapes(), 0);
/// ```
/// A binding cannot outlive its source:
///
/// ```compile_fail,E0597
/// use martensite_tkbo::bop_ds::{BOPDSIterator, DataStructure};
/// let mut iterator = BOPDSIterator::default();
/// let binding = {
///     let mut source = DataStructure::default();
///     iterator.bind_structure(&mut source)
/// };
/// assert_eq!(binding.structure().nb_shapes(), 0);
/// ```
pub struct BorrowedIterator<'a> {
    iterator: &'a mut BOPDSIterator,
    structure: &'a mut DataStructure,
}

impl BOPDSIterator {
    /// Binds a non-null source for a scope without preparing or changing pair state.
    /// `let binding = iterator.bind_structure(&mut source);`
    pub fn bind_structure<'a>(
        &'a mut self,
        structure: &'a mut DataStructure,
    ) -> BorrowedIterator<'a> {
        BorrowedIterator {
            iterator: self,
            structure,
        }
    }
}

impl<'a> BorrowedIterator<'a> {
    /// Reads the current pair count without preparing or advancing the iterator.
    /// `assert_eq!(binding.expected_length(), 0);`
    pub fn expected_length(&self) -> i32 {
        self.iterator.expected_length()
    }

    /// Borrows the exact currently bound source; `binding.structure().nb_shapes()`.
    pub fn structure(&self) -> &DataStructure {
        self.structure
    }

    /// Mutates the bound source exclusively; subsequent queries observe these edits.
    /// `binding.structure_mut().add_shape_sd(3, 7);`
    pub fn structure_mut(&mut self) -> &mut DataStructure {
        self.structure
    }

    /// Rebinds without preparation, returning the previous exclusive source borrow.
    /// `let previous = binding.replace_structure(&mut replacement);`
    pub fn replace_structure(&mut self, structure: &'a mut DataStructure) -> &'a mut DataStructure {
        std::mem::replace(&mut self.structure, structure)
    }
}

impl BorrowedIterator<'_> {
    /// Prepares candidates from the currently bound source.
    /// `binding.prepare()?;`
    pub fn prepare(&mut self) -> Result<(), Failure> {
        self.iterator.prepare(self.structure)
    }

    /// Prepares with explicit OBB context and fuzzy settings, forwarding them unchanged.
    /// `binding.prepare_with_context(None, false, 1e-7)?;`
    pub fn prepare_with_context(
        &mut self,
        context: Option<&Context>,
        check_obb: bool,
        fuzzy: f64,
    ) -> Result<(), Failure> {
        self.iterator
            .prepare_with_context(self.structure, context, check_obb, fuzzy)
    }

    /// Selects a prepared interference kind without rebinding its source.
    /// `binding.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);`
    pub fn initialize(&mut self, first: ShapeEnum, second: ShapeEnum) {
        self.iterator.initialize(first, second);
    }

    /// Reports whether the existing cursor addresses another pair; `binding.more()`.
    pub fn more(&self) -> bool {
        self.iterator.more()
    }

    /// Advances the existing cursor without preparing or rebinding; `binding.next()`.
    pub fn next(&mut self) {
        self.iterator.next();
    }

    /// Reads the existing pair using the current bound source's live shape metadata.
    /// `let pair = binding.value()?;`
    pub fn value(&self) -> Result<(i32, i32), Failure> {
        self.iterator.value_with_structure(self.structure)
    }
}
