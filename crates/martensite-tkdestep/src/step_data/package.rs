//! `StepData`: the package class: the recorded header protocols.
use super::{StepDataFileProtocol, StepDataProtocol, StepDataProtocolOps};
use martensite_tkernel::standard::Handle;
use std::sync::{OnceLock, PoisonError, RwLock};

fn header_slot() -> &'static RwLock<Handle<dyn StepDataProtocolOps>> {
    static HEADER: OnceLock<RwLock<Handle<dyn StepDataProtocolOps>>> = OnceLock::new();
    HEADER.get_or_init(|| RwLock::new(Handle::default()))
}

/// `StepData`.
///
/// ```
/// use martensite_tkdestep::step_data::StepData;
/// assert!(StepData::protocol().is_some());
/// ```
pub struct StepData;

impl StepData {
    /// `init`: nothing.
    pub fn init() {}
    /// `Protocol`: the shared `StepDataProtocolOps`.
    pub fn protocol() -> Handle<dyn StepDataProtocolOps> {
        static PROTOCOL: OnceLock<Handle<dyn StepDataProtocolOps>> = OnceLock::new();
        PROTOCOL
            .get_or_init(|| Handle::new(StepDataProtocol::default()).cast())
            .clone()
    }
    /// `add_header_protocol(header)`: the first as is; the following gather in a file protocol.
    pub fn add_header_protocol(header: &Handle<dyn StepDataProtocolOps>) {
        let mut slot = header_slot()
            .write()
            .unwrap_or_else(PoisonError::into_inner);
        if slot.is_null() {
            *slot = header.clone();
            return;
        }
        let mut multiple = slot.cast::<StepDataFileProtocol>();
        if multiple.is_null() {
            multiple = Handle::new(StepDataFileProtocol::default());
            if let Some(live) = multiple.get() {
                live.add(&slot);
            }
        }
        if let Some(live) = multiple.get() {
            live.add(header);
        }
        *slot = multiple.cast();
    }
    /// `header_protocol`: null, one protocol, or a file protocol.
    pub fn header_protocol() -> Handle<dyn StepDataProtocolOps> {
        header_slot()
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}
