//! Owned buffer-kind storage for callbacks that reread the caller's field (#323).
use super::Graphic3dBufferType;
use martensite_tkernel::standard::SharedState;
use std::sync::Arc;

/// Shares one buffer-kind field across callbacks; `CaptureBufferKindRef::new(Graphic3dBufferType::Rgb)`.
///
/// Clones retain the same slot after the original owner is dropped. Each access
/// releases its guard before returning, so callers may reenter a capture callback.
#[derive(Clone, Debug)]
pub struct CaptureBufferKindRef {
    slot: Arc<SharedState<Graphic3dBufferType>>,
}
impl CaptureBufferKindRef {
    /// Owns an independent field; `let kind = CaptureBufferKindRef::new(Graphic3dBufferType::Rgb);`.
    pub fn new(kind: Graphic3dBufferType) -> Self {
        Self {
            slot: Arc::new(SharedState::new(kind)),
        }
    }
    /// Copies the current field without retaining a lock; `let value = kind.read();`.
    pub fn read(&self) -> Graphic3dBufferType {
        // Returning the enum keeps callbacks outside the storage lease.
        *self.slot.read()
    }
    /// Updates the shared field before a later read; `kind.set(Graphic3dBufferType::Red);`.
    pub fn set(&self, kind: Graphic3dBufferType) {
        // Only enum assignment occurs under the lease, never external code.
        *self.slot.write() = kind;
    }
    /// Tests slot identity without exposing an address; `assert!(kind.same_slot(&kind.clone()));`.
    pub fn same_slot(&self, other: &Self) -> bool {
        // Equal enum values alone do not identify the caller's field.
        Arc::ptr_eq(&self.slot, &other.slot)
    }
}
