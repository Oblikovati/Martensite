//! `STEPSelectionsSelectForTransfer`: the roots of the transfer process.
use crate::step_entity::step_live;
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkxsbase::if_select::{
    IFSelectSelectDeduct, IFSelectSelectDeductBase, IFSelectSelectExtract,
    IFSelectSelectExtractBase, IFSelectSelection, IFSelectSelectionIterator, deduct_fill_iterator,
    extract_label,
};
use martensite_tkxsbase::interface::{
    InterfaceEntityIterator, InterfaceGraph, InterfaceInterfaceModel,
};
use martensite_tkxsbase::xs_control::{XSControlSelectForTransfer, XSControlTransferReader};

/// Select for transfer.
#[derive(Default)]
pub struct STEPSelectionsSelectForTransfer {
    base: XSControlSelectForTransfer,
}
impl_transient!(STEPSelectionsSelectForTransfer, "STEPSelectionsSelectForTransfer", XSControlSelectForTransfer, base, interfaces: [dyn IFSelectSelection, dyn IFSelectSelectDeduct, dyn IFSelectSelectExtract]);

impl STEPSelectionsSelectForTransfer {
    /// `STEPSelectionsSelectForTransfer()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `STEPSelectionsSelectForTransfer(TR)`.
    pub fn with_reader(reader: &Handle<XSControlTransferReader>) -> Self {
        Self {
            base: XSControlSelectForTransfer::with_reader(reader),
        }
    }
    /// `set_reader(TR)`.
    pub fn set_reader(&self, reader: &Handle<XSControlTransferReader>) {
        self.base.set_reader(reader);
    }
}

impl IFSelectSelectExtract for STEPSelectionsSelectForTransfer {
    fn extract_base(&self) -> &IFSelectSelectExtractBase {
        self.base.extract_base()
    }
    fn sort(
        &self,
        rank: i32,
        ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        self.base.sort(rank, ent, model)
    }
    fn extract_label(&self) -> AsciiString {
        self.base.extract_label()
    }
}

impl IFSelectSelectDeduct for STEPSelectionsSelectForTransfer {
    fn deduct_base(&self) -> &IFSelectSelectDeductBase {
        self.base.deduct_base()
    }
}

impl IFSelectSelection for STEPSelectionsSelectForTransfer {
    /// `root_result(G)`: the roots the reader recorded in the transient process.
    fn root_result(&self, _g: &InterfaceGraph) -> Result<InterfaceEntityIterator, Failure> {
        let reader = self.base.reader();
        let process = step_live(&reader, "STEPSelectionsSelectForTransfer")?.transient_process();
        let live = step_live(&process, "STEPSelectionsSelectForTransfer")?;
        let mut iterator = InterfaceEntityIterator::new();
        for root in live.roots_for_transfer().read().iter() {
            iterator.get_one_item(root);
        }
        Ok(iterator)
    }
    fn fill_iterator(&self, iter: &mut IFSelectSelectionIterator) {
        deduct_fill_iterator(self, iter)
    }
    fn label(&self) -> AsciiString {
        extract_label(self)
    }
}
