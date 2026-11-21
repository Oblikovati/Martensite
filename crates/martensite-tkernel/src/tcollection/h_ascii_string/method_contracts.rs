//! Raw buffer and converting constructor expressions.
use super::{AsciiString, HAsciiString};
use crate::standard::{Failure, exceptions};
impl HAsciiString {
    /// Returns the current buffer address for comparison. Moves and edits
    /// through aliases can invalidate the pointer. Use `text.string()` to borrow
    /// the bytes under a read guard.
    pub(crate) fn to_c_ptr(&self) -> *const u8 {
        self.string().as_bytes_with_nul().as_ptr()
    }
    /// Transfers the wrapped string allocation and resets its source; `HAsciiString::from_moved_string(&mut text)`.
    pub(crate) fn from_moved_string(source: &mut AsciiString) -> Self {
        std::mem::take(source).into()
    }
    /// Checks the nullable C-string constructor; `HAsciiString::from_optional_c_string(Some(b"abc"))?`.
    pub(crate) fn from_optional_c_string(source: Option<&[u8]>) -> Result<Self, Failure> {
        source.map(Self::new).ok_or_else(|| exceptions::null_object(
            "TCollection_HAsciiString constructor: null character pointer; expected a terminated byte string"))
    }
}
