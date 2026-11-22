//! The virtual members of `ChFiDSSpine`, which the fillet and chamfer spines override.
use super::{Spine, SpineData};
use crate::chfi_ds::shared::Shared;
use crate::chfi_ds::{ChamfSpine, ElSpine, FilSpine};
use martensite_tkernel::{
    impl_transient_interface_target,
    standard::{Failure, Handle, RuntimeType, RuntimeTyped, Transient},
};
use std::ops::{Deref, DerefMut};

/// A shared, mutable `occ::handle<ChFiDSSpine>` whose object may be a [`Spine`], a
/// [`FilSpine`] or a [`ChamfSpine`]; a null native handle is `None`.
pub type SpineHandle = Shared<dyn SpineObject>;

/// The polymorphic interface of a guideline (the virtual members of `ChFiDSSpine`).
///
/// Through `Deref`, a `dyn SpineObject` also offers every [`SpineData`] method;
/// the virtual ones are only here, so a call always dispatches to the override.
///
/// ```
/// use martensite_tkfillet::chfi_ds::{FilSpine, SpineHandle, SpineObject, lock, shared};
/// let spine: SpineHandle = shared(FilSpine::new());
/// assert!(lock(&spine).as_fil_spine().is_some());
/// assert_eq!(lock(&spine).nb_edges(), 0);
/// ```
pub trait SpineObject: Transient {
    /// Borrow the shared guideline, e.g. `spine.spine().nb_edges()`.
    fn spine(&self) -> &SpineData;
    /// Edit the shared guideline, e.g. `spine.spine_mut().set_first_status(state)`.
    fn spine_mut(&mut self) -> &mut SpineData;
    /// `Reset(AllData)`: forgets the split (and, with `all_data`, the extensions);
    /// StandardNullObject when `all_data` before Load; `spine.reset(false)?`.
    fn reset(&mut self, all_data: bool) -> Result<(), Failure>;
    /// `AppendElSpine(Els)`; `spine.append_el_spine(&el_spine)?`.
    fn append_el_spine(&mut self, el_spine: &Handle<ElSpine>) -> Result<(), Failure>;
    /// `AppendOffsetElSpine(Els)`; `spine.append_offset_el_spine(&el_spine)`.
    fn append_offset_el_spine(&mut self, el_spine: &Handle<ElSpine>) {
        self.spine_mut().offset_el_spines.push(el_spine.clone());
    }
    /// The object as a fillet spine (`occ::down_cast<ChFiDSFilSpine>`).
    fn as_fil_spine(&self) -> Option<&FilSpine> {
        None
    }
    /// The object as a mutable fillet spine.
    fn as_fil_spine_mut(&mut self) -> Option<&mut FilSpine> {
        None
    }
    /// The object as a chamfer spine (`occ::down_cast<ChFiDSChamfSpine>`).
    fn as_chamf_spine(&self) -> Option<&ChamfSpine> {
        None
    }
    /// The object as a mutable chamfer spine.
    fn as_chamf_spine_mut(&mut self) -> Option<&mut ChamfSpine> {
        None
    }
}

impl RuntimeTyped for dyn SpineObject {
    const TYPE_NAME: &'static str = Spine::TYPE_NAME;

    fn static_type() -> &'static RuntimeType {
        Spine::static_type()
    }
}
impl_transient_interface_target!(dyn SpineObject);

impl SpineObject for Spine {
    fn spine(&self) -> &SpineData {
        self
    }
    fn spine_mut(&mut self) -> &mut SpineData {
        self
    }
    fn reset(&mut self, all_data: bool) -> Result<(), Failure> {
        self.guideline.reset_guideline(all_data)
    }

    fn append_el_spine(&mut self, el_spine: &Handle<ElSpine>) -> Result<(), Failure> {
        self.guideline.push_el_spine(el_spine);
        Ok(())
    }
}

impl SpineData {
    pub(in crate::chfi_ds) fn reset_guideline(&mut self, all_data: bool) -> Result<(), Failure> {
        self.split_done = false;
        self.el_spines.clear();
        if all_data {
            self.ends.first_param = 0.;
            self.ends.last_param = self.total_length()?;
            self.ends.first_prolon = false;
            self.ends.last_prolon = false;
        }
        Ok(())
    }

    pub(in crate::chfi_ds) fn push_el_spine(&mut self, el_spine: &Handle<ElSpine>) {
        self.el_spines.push(el_spine.clone());
    }
}

impl Deref for dyn SpineObject {
    type Target = SpineData;
    fn deref(&self) -> &SpineData {
        self.spine()
    }
}

impl DerefMut for dyn SpineObject {
    fn deref_mut(&mut self) -> &mut SpineData {
        self.spine_mut()
    }
}
