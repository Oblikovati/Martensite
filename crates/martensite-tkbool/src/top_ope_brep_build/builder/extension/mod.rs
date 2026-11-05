//! Composition views preserve the builder's live retained state and ownership.
mod control;
mod results;
mod working;
use super::TopOpeBRepBuildBuilder;

/// Shared access to an operation-local builder's extension state.
///
/// `let state = builder.extension_state();` retains no cloned cache or shape.
/// This view does not make the existing builder algorithms reentrant or promise
/// isolation of shared geometry reachable from their shape values.
pub struct BuilderExtensionState<'a> {
    builder: &'a TopOpeBRepBuildBuilder,
}

/// Exclusive access to an operation-local builder's extension state.
///
/// `*builder.extension_state_mut().classification_value_mut() = false;`
/// requires exclusive access to the owner. It cannot mutate through a shared
/// result handle, and adds no lock, thread-local state, or unsafe implementation.
pub struct BuilderExtensionStateMut<'a> {
    builder: &'a mut TopOpeBRepBuildBuilder,
}

impl TopOpeBRepBuildBuilder {
    /// Borrows retained extension state without mutation; `let state = builder.extension_state();`.
    pub fn extension_state(&self) -> BuilderExtensionState<'_> {
        BuilderExtensionState { builder: self }
    }

    /// Exclusively borrows retained state; `let state = builder.extension_state_mut();`.
    pub fn extension_state_mut(&mut self) -> BuilderExtensionStateMut<'_> {
        BuilderExtensionStateMut { builder: self }
    }
}
