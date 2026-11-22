//! Members that change the shared string, or build a new shared string from it.
//!
//! Byte-string parameters are C strings: the bytes through the first NUL.
use super::HAsciiString;
use crate::standard::{Failure, Handle};
use crate::tcollection::ascii_string::c_string_prefix;
#[cfg(test)]
use crate::tcollection::ascii_string::negative_width;

impl HAsciiString {
    /// Example: `object.assign_cat("tail")`.
    pub fn assign_cat(&self, other: impl AsRef<[u8]>) {
        self.edit().assign_cat_c_bytes(Some(other.as_ref()));
    }
    /// Example: `object.assign_cat_string(&other)`.
    #[cfg(test)]
    pub(crate) fn assign_cat_string(&self, other: &HAsciiString) {
        let other = other.snapshot();
        self.edit().append(&other);
    }
    /// A new object; `object.cat("tail").length()`.
    #[cfg(test)]
    pub(crate) fn cat(&self, other: impl AsRef<[u8]>) -> Handle<Self> {
        Self::shared(self.string().cat(c_string_prefix(other.as_ref())))
    }
    /// Example: `object.cat_string(&other)`.
    #[cfg(test)]
    pub(crate) fn cat_string(&self, other: &HAsciiString) -> Handle<Self> {
        let other = other.snapshot();
        Self::shared(self.string().cat(&other))
    }
    /// Example: `object.capitalize()`.
    #[cfg(test)]
    pub(crate) fn capitalize(&self) {
        self.edit().capitalize();
    }
    /// Example: `object.upper_case()`.
    #[cfg(test)]
    pub(crate) fn upper_case(&self) {
        self.edit().upper_case();
    }
    /// Example: `object.lower_case()`.
    pub fn lower_case(&self) {
        self.edit().lower_case();
    }
    /// `Standard_NegativeValue` for a negative width; `object.center(9, b'*')?`.
    #[cfg(test)]
    pub(crate) fn center(&self, width: i32, filler: u8) -> Result<(), Failure> {
        check_width("Center", width)?;
        self.edit().center(width, filler)
    }
    /// Example: `object.clear()`.
    pub fn clear(&self) {
        self.edit().clear();
    }
    /// Example: `object.left_adjust()`.
    #[cfg(test)]
    pub(crate) fn left_adjust(&self) {
        self.edit().left_adjust();
    }
    /// Example: `object.right_adjust()`.
    #[cfg(test)]
    pub(crate) fn right_adjust(&self) {
        self.edit().right_adjust();
    }
}

impl HAsciiString {
    /// Example: `object.insert(1, "head")?`.
    pub fn insert(&self, position: i32, what: impl AsRef<[u8]>) -> Result<(), Failure> {
        self.edit().insert(position, c_string_prefix(what.as_ref()))
    }
    /// Example: `object.insert_string(1, &other)?`.
    #[cfg(test)]
    pub(crate) fn insert_string(&self, position: i32, what: &HAsciiString) -> Result<(), Failure> {
        let what = what.snapshot();
        self.edit().insert(position, &what)
    }
    /// Example: `object.prepend(&other)`.
    #[cfg(test)]
    pub(crate) fn prepend(&self, other: &HAsciiString) {
        let other = other.snapshot();
        self.edit().prepend(&other);
    }
    /// `Remove`; `object.remove(2, 1)?`.
    #[cfg(test)]
    pub(crate) fn remove(&self, position: i32, count: i32) -> Result<(), Failure> {
        self.edit().remove(position, count)
    }
    /// Example: `object.set_value_char(1, b'x')?`.
    pub fn set_value_char(&self, position: i32, byte: u8) -> Result<(), Failure> {
        self.edit().set_value_char(position, byte)
    }
    /// Example: `object.set_value(2, "yz")?`.
    #[cfg(test)]
    pub(crate) fn set_value(&self, position: i32, what: impl AsRef<[u8]>) -> Result<(), Failure> {
        self.edit()
            .set_value(position, c_string_prefix(what.as_ref()))
    }
    /// Keeps the head and returns the tail as a new object; `object.split(3)?`.
    #[cfg(test)]
    pub(crate) fn split(&self, length: i32) -> Result<Handle<Self>, Failure> {
        let tail = self.edit().split(length)?;
        Ok(Self::shared(tail))
    }
    /// A new object; `object.sub_string(2, 4)?`.
    pub fn sub_string(&self, from: i32, to: i32) -> Result<Handle<Self>, Failure> {
        Ok(Self::shared(self.string().sub_string(from, to)?))
    }
    /// `Token`: a new object; `object.token(" \t", 2)`.
    #[cfg(test)]
    pub(crate) fn token(&self, separators: impl AsRef<[u8]>, which: i32) -> Handle<Self> {
        Self::shared(self.string().token(separators, which))
    }
    /// Example: `object.truncate(3)?`.
    #[cfg(test)]
    pub(crate) fn truncate(&self, length: i32) -> Result<(), Failure> {
        self.edit().truncate(length)
    }
}

/// The width check made before delegating.
#[cfg(test)]
fn check_width(member: &str, width: i32) -> Result<(), Failure> {
    if width < 0 {
        return Err(negative_width(member, width));
    }
    Ok(())
}
