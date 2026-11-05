//! Borrowed extension access retains the loop's existing field identities.
use super::{TopOpeBRepBuildBlockIterator, TopOpeBRepBuildLoopBase};
use martensite_tkbrep::topods::Shape;

/// A mutable view of an operation-local loop's extension state.
///
/// `*loop_record.extension_state().shape_mode_mut() = false` selects block mode.
/// Obtaining the view requires exclusive access to the loop record. It neither
/// clones the retained shape nor grants mutation through a shared published handle.
pub struct LoopExtensionState<'a> {
    shape_mode: &'a mut bool,
    shape: &'a mut Shape,
    block_cursor: &'a mut TopOpeBRepBuildBlockIterator,
}

/// A shared view of a loop's retained extension state.
///
/// `loop_record.extension_state_ref().shape()` borrows the actual stored shape.
/// Returned references retain the record's lifetime rather than the view's lifetime.
pub struct LoopExtensionStateRef<'a> {
    shape_mode: &'a bool,
    shape: &'a Shape,
    block_cursor: &'a TopOpeBRepBuildBlockIterator,
}

impl TopOpeBRepBuildLoopBase {
    /// Borrows retained fields for const extensions; `let state = loop_record.extension_state_ref();`.
    pub fn extension_state_ref(&self) -> LoopExtensionStateRef<'_> {
        LoopExtensionStateRef {
            shape_mode: &self.my_is_shape,
            shape: &self.my_shape,
            block_cursor: &self.my_block_iterator,
        }
    }

    /// Borrows the state used by existing loop queries; `let state = loop_record.extension_state();`.
    pub fn extension_state(&mut self) -> LoopExtensionState<'_> {
        LoopExtensionState {
            shape_mode: &mut self.my_is_shape,
            shape: &mut self.my_shape,
            block_cursor: &mut self.my_block_iterator,
        }
    }
}

impl LoopExtensionState<'_> {
    /// Selects shape or block mode without resetting either payload; `*state.shape_mode_mut() = true`.
    pub fn shape_mode_mut(&mut self) -> &mut bool {
        self.shape_mode
    }

    /// Edits the retained shape independently of the mode; `*state.shape_mut() = replacement`.
    pub fn shape_mut(&mut self) -> &mut Shape {
        self.shape
    }

    /// Edits the retained cursor, including its current position; `state.block_cursor_mut().next()`.
    pub fn block_cursor_mut(&mut self) -> &mut TopOpeBRepBuildBlockIterator {
        self.block_cursor
    }
}

impl<'a> LoopExtensionStateRef<'a> {
    /// Borrows the actual shape-mode flag; `assert!(*state.shape_mode())`.
    pub fn shape_mode(&self) -> &'a bool {
        self.shape_mode
    }

    /// Borrows the retained shape without cloning it; `let shape = state.shape()`.
    pub fn shape(&self) -> &'a Shape {
        self.shape
    }

    /// Borrows the retained cursor without a snapshot; `let position = state.block_cursor().value()`.
    pub fn block_cursor(&self) -> &'a TopOpeBRepBuildBlockIterator {
        self.block_cursor
    }
}
