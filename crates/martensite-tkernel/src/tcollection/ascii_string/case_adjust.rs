//! Case mapping, byte replacement, whitespace trimming and justification.
//!
//! Case and whitespace follow the C locale's `toupper`/`tolower`/`isspace`, which touch
//! ASCII bytes only; high-bit bytes are never changed or trimmed.
use super::AsciiString;
#[cfg(test)]
use super::raise::negative_width;
#[cfg(test)]
use crate::standard::Failure;
use crate::standard::character;

impl AsciiString {
    /// Maps ASCII lowercase bytes in place; `text.upper_case()`.
    pub fn upper_case(&mut self) {
        self.terminated.make_ascii_uppercase();
    }
    /// Maps ASCII uppercase bytes in place; `text.lower_case()`.
    pub fn lower_case(&mut self) {
        self.terminated.make_ascii_lowercase();
    }
    /// Uppercases the first byte and lowercases the rest; `text.capitalize()`.
    pub fn capitalize(&mut self) {
        self.lower_case();
        let bytes = &mut self.terminated;
        if let Some(first) = bytes.first_mut() {
            first.make_ascii_uppercase();
        }
    }
    /// Replaces every `from` byte by `to`, optionally ignoring ASCII case;
    /// `text.change_all(b'a', b'o', true)`.
    #[cfg(test)]
    pub(crate) fn change_all(&mut self, from: u8, to: u8, case_sensitive: bool) {
        let length = self.len();
        let wanted = fold(from, case_sensitive);
        let bytes = &mut self.terminated;
        for byte in &mut bytes[..length] {
            if fold(*byte, case_sensitive) == wanted {
                *byte = to;
            }
        }
    }
    /// Erases every occurrence of `byte`; `text.remove_all(b'e')`.
    #[cfg(test)]
    pub(crate) fn remove_all(&mut self, byte: u8) {
        self.remove_all_with_case(byte, true);
    }
    /// Erases every occurrence, optionally ignoring ASCII case;
    /// `text.remove_all_with_case(b'E', false)`.
    #[cfg(test)]
    pub(crate) fn remove_all_with_case(&mut self, byte: u8, case_sensitive: bool) {
        let wanted = fold(byte, case_sensitive);
        let length = self.len();
        let mut retained = 0;
        let bytes = &mut self.terminated;
        for offset in 0..length {
            let value = bytes[offset];
            if fold(value, case_sensitive) != wanted {
                bytes[retained] = value;
                retained += 1;
            }
        }
        self.set_length(retained);
    }
    /// Removes leading C-locale whitespace; `text.left_adjust()`.
    pub fn left_adjust(&mut self) {
        let leading = self
            .as_bytes()
            .iter()
            .take_while(|&&byte| character::is_space(byte))
            .count();
        self.terminated.drain(..leading);
    }
    /// Removes trailing C-locale whitespace; `text.right_adjust()`.
    pub fn right_adjust(&mut self) {
        let trailing = self
            .as_bytes()
            .iter()
            .rev()
            .take_while(|&&byte| character::is_space(byte))
            .count();
        self.set_length(self.len() - trailing);
    }
    /// Pads on the right with `filler` up to `width`; `text.left_justify(8, b'.')?`.
    #[cfg(test)]
    pub(crate) fn left_justify(&mut self, width: i32, filler: u8) -> Result<(), Failure> {
        if width < 0 {
            return Err(negative_width("LeftJustify", width));
        }
        let padding = (width as usize).saturating_sub(self.len());
        if padding == 0 {
            return Ok(());
        }
        self.ensure_owned_terminator();
        let length = self.len();
        self.terminated
            .insert_many(length, std::iter::repeat_n(filler, padding));
        Ok(())
    }
    /// Pads on the left with `filler` up to `width`; `text.right_justify(8, b' ')?`.
    #[cfg(test)]
    pub(crate) fn right_justify(&mut self, width: i32, filler: u8) -> Result<(), Failure> {
        if width < 0 {
            return Err(negative_width("RightJustify", width));
        }
        let padding = (width as usize).saturating_sub(self.len());
        if padding == 0 {
            return Ok(());
        }
        self.ensure_owned_terminator();
        self.terminated
            .insert_many(0, std::iter::repeat_n(filler, padding));
        Ok(())
    }
    /// Pads both sides up to `width`, the odd filler going left;
    /// `AsciiString::new("ab").center(5, b'*')` gives `"**ab*"`.
    #[cfg(test)]
    pub(crate) fn center(&mut self, width: i32, filler: u8) -> Result<(), Failure> {
        if width < 0 {
            return Err(negative_width("Center", width));
        }
        if width > self.length() {
            let left_width = self.length() + (width - self.length()) / 2;
            self.left_justify(left_width, filler)?;
            self.right_justify(width, filler)?;
        }
        Ok(())
    }
}

/// `::UpperCase` when the comparison ignores case, the byte itself otherwise.
#[cfg(test)]
fn fold(byte: u8, case_sensitive: bool) -> u8 {
    if case_sensitive {
        byte
    } else {
        character::upper_case(byte)
    }
}
