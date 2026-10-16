//! A catalog message whose `%` format specifications take arguments.
use super::MsgFile;
use crate::c_format::{CArg, c_sprintf};
use crate::standard::Failure;
use crate::tcollection::{AsciiString, ExtendedString, HAsciiString, HExtendedString};
mod body;
use body::{BodyText, FormatKind};

/// A message text with pending format arguments; `Msg::from_text(&text).arg_integer(5)?`.
///
/// Positions of the parsed specifications live in a flat sequence of
/// `(kind, 0-based start, length)` triplets.
#[derive(Clone, Debug, Default)]
pub struct Msg {
    original: ExtendedString,
    body: ExtendedString,
    formats: Vec<i32>,
}
impl Msg {
    /// Creates an empty message; `Msg::new().value().is_empty()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Loads the text registered under a key in the process catalog; `Msg::from_key(&key)`.
    pub fn from_key(key: &AsciiString) -> Self {
        Self::from_key_in(MsgFile::global(), key)
    }
    /// Loads the text registered under a key in an explicit catalog; `Msg::from_key_in(&catalog, &key)`.
    pub fn from_key_in(catalog: &MsgFile, key: &AsciiString) -> Self {
        let mut message = Self::new();
        message.set(&catalog.msg(key));
        message
    }
    /// Key given as an extended string, converted to UTF-8;
    /// `Msg::from_extended_key(&key)`.
    pub fn from_extended_key(key: &ExtendedString) -> Self {
        Self::from_key(&AsciiString::from(key))
    }
    /// Uses C text as the message, decoded as UTF-8 when valid; `msg.set_text(b"%d items")`.
    pub fn set_text(&mut self, text: &[u8]) {
        self.set(&ExtendedString::from_ascii(&c_string(text)));
    }
    /// Uses a text as the message and records its format specifications; `msg.set(&text)`.
    pub fn set(&mut self, text: &ExtendedString) {
        let parsed = BodyText::parse(text.as_utf16());
        self.body = ExtendedString::from_raw(&parsed.units);
        self.formats = parsed.formats;
        self.original = self.body.clone();
    }
    /// The text as it was set, without arguments; `msg.original()`.
    pub fn original(&self) -> &ExtendedString {
        &self.original
    }
    /// The text with the arguments given so far; `msg.value()`.
    pub fn value(&self) -> &ExtendedString {
        &self.body
    }
    /// True once an argument changed the text; `msg.is_edited()`.
    pub fn is_edited(&self) -> bool {
        !self.original.is_equal(&self.body)
    }
}

/// C string semantics: the text ends at the first NUL.
fn c_string(text: &[u8]) -> AsciiString {
    AsciiString::from_bytes(text.split(|&byte| byte == 0).next().unwrap_or_default())
}

impl Msg {
    /// Fills the next `%s` with C text; `msg.arg_text(b"x")?`.
    ///
    /// A fixed `max(strlen + 1, 1024)`-byte buffer would overflow for a width above that;
    /// the output here is simply the full field.
    pub fn arg_text(&mut self, text: &[u8]) -> Result<&mut Self, Failure> {
        let Some((first, format)) = self.take_format(FormatKind::String)? else {
            return Ok(self);
        };
        let text = c_string(text);
        let formatted = c_sprintf(&format, &[CArg::Text(text.as_bytes())]);
        let decoded = ExtendedString::from_c_string(&formatted, true);
        self.replace_text(first, format.len(), &decoded)?;
        Ok(self)
    }
    /// Fills the next specification with an ASCII string's C string; `msg.arg_ascii(&text)?`.
    pub fn arg_ascii(&mut self, text: &AsciiString) -> Result<&mut Self, Failure> {
        self.arg_text(text.as_bytes())
    }
    /// Fills the next specification with a shared ASCII string; `msg.arg_h_ascii(&text)?`.
    pub fn arg_h_ascii(&mut self, text: &HAsciiString) -> Result<&mut Self, Failure> {
        self.arg_text(&text.as_bytes_with_nul())
    }
    /// Fills the next specification with a shared extended string; `msg.arg_h_extended(&text)?`.
    pub fn arg_h_extended(&mut self, text: &HExtendedString) -> Result<&mut Self, Failure> {
        let value = text.string().clone();
        self.arg_extended(&value)
    }
    /// Fills the next `%s` with an extended string: the text itself, ignoring flags and width; `msg.arg_extended(&text)?`.
    pub fn arg_extended(&mut self, text: &ExtendedString) -> Result<&mut Self, Failure> {
        let Some((first, format)) = self.take_format(FormatKind::String)? else {
            return Ok(self);
        };
        self.replace_text(first, format.len(), text)?;
        Ok(self)
    }
    /// Fills the next integer specification (`%d %i %o %u %x %X`); `msg.arg_integer(5)?`.
    pub fn arg_integer(&mut self, value: i32) -> Result<&mut Self, Failure> {
        self.arg_number(FormatKind::Integer, CArg::Int(value))
    }
    /// Fills the next real specification (`%f %e %E %g %G`); `msg.arg_real(1.5)?`.
    pub fn arg_real(&mut self, value: f64) -> Result<&mut Self, Failure> {
        self.arg_number(FormatKind::Real, CArg::Real(value))
    }
    fn arg_number(&mut self, kind: FormatKind, value: CArg<'_>) -> Result<&mut Self, Failure> {
        let Some((first, format)) = self.take_format(kind)? else {
            return Ok(self);
        };
        let formatted = c_sprintf(&format, &[value]);
        // The formatted bytes widen to UTF-16 units one by one, without multibyte decoding.
        let text = ExtendedString::from_c_string(&formatted, false);
        self.replace_text(first, format.len(), &text)?;
        Ok(self)
    }
}
