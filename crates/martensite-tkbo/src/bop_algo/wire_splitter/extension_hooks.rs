//! Actual mutable receiver substitution, with operation-bound default delegation.
use super::{WireOperationContext, WireSplitter, dispatch};
use crate::bop_algo::ProgressCompletion;
use martensite_tkernel::{message::ProgressRange, standard::Failure};

/// One stored wire receiver and its overridable input check, never a copied base.
/// Example: implement `wire_splitter` with `&mut self.receiver` on a Send wrapper.
/// The accessor must return that same object throughout each operation.
pub trait WireSplitterHooks: Send {
    /// Borrow the same actual receiver; for example, `&mut self.receiver`.
    fn wire_splitter(&mut self) -> &mut WireSplitter;

    /// Check through the bound operation; overrides delegate with `operation.check_data_default(self)`.
    fn check_data(&mut self, operation: &WireOperationContext) -> Result<(), Failure> {
        // Default delegation cannot bypass the binding checked inside an override.
        operation.check_data_default(self)
    }

    /// Run the shared whole program; `WireSplitterHooks::perform_with_progress(&mut wrapper, range)`.
    /// Completion describes raw execution only, never validation or publication.
    fn perform_with_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        // External wrappers and the inherent entry share one actual-receiver program.
        dispatch::perform_wire_with_progress(self, range)
    }
}

impl WireSplitterHooks for WireSplitter {
    fn wire_splitter(&mut self) -> &mut WireSplitter {
        // The default hook borrows the receiver instead of copying its base state.
        self
    }
}
