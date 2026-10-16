//! JSON dump keys and their text.
use crate::tcollection::AsciiString;

/// Kind of key in dump JSON; `JsonKey::Quote as i32 == 5`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum JsonKey {
    /// No key.
    None = 0,
    /// `{`
    OpenChild = 1,
    /// `}`
    CloseChild = 2,
    /// `[`
    OpenContainer = 3,
    /// `]`
    CloseContainer = 4,
    /// `"`
    Quote = 5,
    /// `: `
    SeparatorKeyToValue = 6,
    /// `, `
    SeparatorValueToValue = 7,
}

impl JsonKey {
    /// Every key the parser tries, in order (OpenChild first).
    pub(super) const PARSED: [JsonKey; 7] = [
        JsonKey::OpenChild,
        JsonKey::CloseChild,
        JsonKey::OpenContainer,
        JsonKey::CloseContainer,
        JsonKey::Quote,
        JsonKey::SeparatorKeyToValue,
        JsonKey::SeparatorValueToValue,
    ];
}

/// The key's text, empty for `None`; `json_key_to_string(JsonKey::Quote) == "\""`.
pub fn json_key_to_string(key: JsonKey) -> &'static str {
    match key {
        JsonKey::None => "",
        JsonKey::OpenChild => "{",
        JsonKey::CloseChild => "}",
        JsonKey::OpenContainer => "[",
        JsonKey::CloseContainer => "]",
        JsonKey::Quote => "\"",
        JsonKey::SeparatorKeyToValue => ": ",
        JsonKey::SeparatorValueToValue => ", ",
    }
}

/// Bytes of the key's text; `json_key_length(JsonKey::SeparatorKeyToValue) == 2`.
pub fn json_key_length(key: JsonKey) -> i32 {
    json_key_to_string(key).len() as i32
}

/// The prefix of a short pointer text; `get_pointer_prefix() == "0x"`.
pub fn get_pointer_prefix() -> AsciiString {
    AsciiString::new("0x")
}
