//! Pointer and returning ownership methods.
use super::ExtendedString;
impl ExtendedString {
    /// Returns the UTF-16 C-string pointer identity, including shared empty storage; `text.to_ext_ptr()`.
    pub(crate) fn to_ext_ptr(&self) -> *const u16 {
        self.as_utf16_with_nul().as_ptr()
    }
    /// Transfers a buffer and leaves the source at the shared terminator; `ExtendedString::move_from(&mut source)`.
    pub(crate) fn move_from(source: &mut Self) -> Self {
        std::mem::take(source)
    }
    /// Returns the move-assignment destination; `text.move_assign_returning(&mut source)`.
    pub(crate) fn move_assign_returning(&mut self, source: &mut Self) -> &mut Self {
        self.move_assign(source);
        self
    }
    /// Preserves self-move assignment and returns its destination; `text.move_assign_self()`.
    pub(crate) fn move_assign_self(&mut self) -> &mut Self {
        self
    }
    /// Copies a UTF-16 string view including embedded zeros and returns the destination; `text.assign_view(&[65, 0, 66])`.
    pub(crate) fn assign_view(&mut self, units: &[u16]) -> &mut Self {
        self.copy(units);
        self
    }
}
