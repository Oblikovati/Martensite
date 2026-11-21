//! The error status and status extension pair that every storage data section keeps.
use super::StorageError;
use crate::tcollection::AsciiString;

/// Last error of a section and the name of the step that raised it.
#[derive(Clone, Debug, Default)]
pub(super) struct SectionStatus {
    /// The status.
    pub(super) error: StorageError,
    /// The status extension: the failing step's context.
    pub(super) extension: AsciiString,
}

impl SectionStatus {
    /// Runs a step and records its status:
    /// the status is always stored, the context only on failure. Returns whether reading goes on.
    /// Example: `status.record(driver.begin_read_type_section()?, "BeginReadTypeSection")`.
    pub(super) fn record(&mut self, error: StorageError, context: &str) -> bool {
        self.error = error;
        if error == StorageError::Ok {
            return true;
        }
        self.extension = AsciiString::new(context);
        false
    }

    /// Back to `Ok` with an empty extension; `status.clear()`.
    pub(super) fn clear(&mut self) {
        self.error = StorageError::Ok;
        self.extension = AsciiString::default();
    }
}
