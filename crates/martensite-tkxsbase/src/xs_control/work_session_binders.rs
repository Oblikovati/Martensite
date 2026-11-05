//! Preserve the destructor's binder and finder owners until session cleanup completes.
use super::{XSControlTransferWriter, XSControlWorkSession};
use crate::{
    if_select::IFSelectWorkSessionOps,
    transfer::{TransferBinder, TransferFinder, TransferFinderProcess},
};
use martensite_tkernel::standard::{Failure, Handle};
type RetainedTransferOwners = (
    Vec<Handle<dyn TransferBinder>>,
    Vec<Handle<dyn TransferFinder>>,
);
impl XSControlWorkSession {
    pub(super) fn clear_binders(&self) -> Result<(), Failure> {
        let writer = self.transfer_writer();
        let process = writer
            .get()
            .map(XSControlTransferWriter::finder_process)
            .unwrap_or_default();
        let (heads, finders) = retained_transfer_owners(&process)?;
        if let Some(live) = process.get() {
            live.clear();
        }
        self.clear_data(1)?;
        self.clear_data(5)?;
        release_transfer_owners(heads, finders);
        Ok(())
    }
}
fn retained_transfer_owners(
    process: &Handle<TransferFinderProcess>,
) -> Result<RetainedTransferOwners, Failure> {
    let Some(process) = process.get() else {
        return Ok((Vec::new(), Vec::new()));
    };
    let mut heads = Vec::new();
    let mut finders = Vec::new();
    for index in 1..=process.nb_mapped() {
        let binder = process.map_item(index)?;
        if !binder.is_null() {
            heads.push(binder);
        }
        finders.push(process.mapped(index)?);
    }
    Ok((heads, finders))
}
fn release_transfer_owners(
    heads: Vec<Handle<dyn TransferBinder>>,
    finders: Vec<Handle<dyn TransferFinder>>,
) {
    let mut finders = finders.into_iter();
    for mut head in heads {
        // Source removes one mapped owner before traversing each retained binder chain.
        drop(finders.next());
        while let Some(live) = head.get() {
            let next = live.next_result();
            head.nullify();
            head = next;
        }
    }
}
