//! Prepared fixed-helper access has no whole-Perform wrapper scope or initial poll.
use super::{WireOperationContext, WireSplitter, extension_status::wire_binding_failure};
use crate::bop_algo::ProgressCompletion;
use martensite_tkernel::{message::ProgressRange, standard::Failure};

/// Borrows one actual receiver/context until its nonvirtual helper is consumed.
/// Example: `splitter.prepare_wires_extension(&mut operation)?.make_wires(range)`.
/// The receiver/adaptor are exclusively mutable and not shared concurrently; the adaptor is Send.
/// Preparation alone produces no successful operation result or published geometry.
#[must_use = "prepared blocks are not execution completion; consume the fixed helper"]
pub struct WirePreparedExtension<'receiver, 'operation> {
    receiver: &'receiver mut WireSplitter,
    operation: &'operation mut WireOperationContext,
}

impl WireSplitter {
    /// Prepare real context/blocks without whole Perform effects; `splitter.prepare_wires_extension(&mut operation)`.
    /// Only classified preparation failure allocates; successful execution keeps borrowed state.
    pub fn prepare_wires_extension<'receiver, 'operation>(
        &'receiver mut self,
        operation: &'operation mut WireOperationContext,
    ) -> Result<WirePreparedExtension<'receiver, 'operation>, Box<ProgressCompletion>> {
        operation
            .bind_direct(self)
            .map_err(wire_binding_failure)
            .map_err(Box::new)?;
        if let Err(cause) = self.prepare_extension_blocks(operation) {
            return Err(Box::new(operation.classify_direct(self, Err(cause))));
        }
        Ok(WirePreparedExtension {
            receiver: self,
            operation,
        })
    }

    fn prepare_extension_blocks(
        &mut self,
        operation: &WireOperationContext,
    ) -> Result<(), Failure> {
        operation.check_direct(self)?;
        if self.wes.is_none() {
            return Err(Failure::with_type(
                "Standard_NullObject",
                "prepared WireSplitter has no edge set; expected actual WES input",
                "",
            ));
        }
        self.prepare_blocks()?;
        operation.mark_prepared()
    }
}

impl WirePreparedExtension<'_, '_> {
    /// Consume the existing nonvirtual helper; `prepared.make_wires(&indicator.start())`.
    /// Raw non-completed loops remain diagnostic, and Completed does not authorize publication.
    pub fn make_wires(self, range: &ProgressRange<'_>) -> ProgressCompletion {
        let result = self.operation.consume_helper(self.receiver).and_then(|()| {
            self.receiver
                .make_wires(range, self.operation.observation())
        });
        self.operation.classify_direct(self.receiver, result)
    }
}
