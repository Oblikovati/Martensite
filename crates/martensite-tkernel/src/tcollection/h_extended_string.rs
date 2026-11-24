//! A transient, handle-shared ExtendedString.
//!
//! The string is edited through any handle that shares it. The `Handle` lends
//! `&T`, so the string sits behind one reader-writer lock and every member takes `&self`.
//! A member that reads another object copies what it needs before it locks its own
//! string, so passing the same object twice (`text.assign_cat(&text)`) cannot deadlock.
use super::{ExtendedString, HAsciiString};
#[cfg(test)]
use crate::standard::Handle;
use crate::standard::StandardTransient;
#[cfg(test)]
use std::sync::RwLockWriteGuard;
use std::sync::{PoisonError, RwLock, RwLockReadGuard};
mod editing;
mod hasher;
pub use hasher::HExtendedStringHasher;

/// A shared, lockable ExtendedString; `Handle::new(HExtendedString::from_c_string(b"abc"))`.
#[repr(C)]
#[derive(Default)]
pub struct HExtendedString {
    string: RwLock<ExtendedString>,
}
crate::impl_transient!(
    HExtendedString,
    "TCollection_HExtendedString",
    StandardTransient,
    handle_key_policy: HExtendedStringHasher
);
impl HExtendedString {
    /// Wraps a C string, each byte widened (no multibyte decoding);
    /// `HExtendedString::from_c_string(b"abc")`.
    pub fn from_c_string(bytes: &[u8]) -> Self {
        Self::from(ExtendedString::from_c_string(bytes, false))
    }
    /// Wraps UTF-16 read through the first zero unit; `HExtendedString::from_utf16(&[65, 0])`.
    pub fn from_utf16(units: &[u16]) -> Self {
        Self::from(ExtendedString::from_utf16(units))
    }
    /// Wraps one code unit (empty for zero); `HExtendedString::from_char(0x20AC)`.
    #[cfg(test)]
    pub(crate) fn from_char(unit: u16) -> Self {
        Self::from(ExtendedString::from_char(unit))
    }
    /// Wraps `length` copies of `unit`; Standard_OutOfRange for a negative length;
    /// `HExtendedString::filled(3, 45)?`.
    #[cfg(test)]
    pub(crate) fn filled(length: i32, unit: u16) -> Result<Self, crate::standard::Failure> {
        ExtendedString::filled(length, unit).map(Self::from)
    }
    /// Converts an HAsciiString's string as UTF-8 when it decodes (the handle constructor);
    /// `HExtendedString::from_h_ascii_string(&ascii)`.
    pub fn from_h_ascii_string(source: &HAsciiString) -> Self {
        Self::from(ExtendedString::from_ascii(&source.string()))
    }
    /// Copies another object's string into a new object (the handle constructor);
    /// `HExtendedString::from_h_extended_string(&other)`.
    pub fn from_h_extended_string(source: &Self) -> Self {
        Self::from(source.string().clone())
    }
    /// Borrows the string for reading (`String`); `text.string().length()`.
    pub fn string(&self) -> RwLockReadGuard<'_, ExtendedString> {
        self.string.read().unwrap_or_else(PoisonError::into_inner)
    }
    /// Locks the string for an edit; no code panics while holding it.
    #[cfg(test)]
    fn string_mut(&self) -> RwLockWriteGuard<'_, ExtendedString> {
        self.string.write().unwrap_or_else(PoisonError::into_inner)
    }
    /// Wraps a result string into a fresh handle.
    #[cfg(test)]
    fn share(string: ExtendedString) -> Handle<Self> {
        Handle::new(Self::from(string))
    }
}
impl From<ExtendedString> for HExtendedString {
    fn from(string: ExtendedString) -> Self {
        Self {
            string: RwLock::new(string),
        }
    }
}
impl From<&ExtendedString> for HExtendedString {
    fn from(string: &ExtendedString) -> Self {
        Self::from(string.clone())
    }
}
#[cfg(test)]
mod method_contracts;
