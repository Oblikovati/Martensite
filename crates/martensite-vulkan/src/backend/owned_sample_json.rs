//! Small typed metadata formatter; artifact hashes and authority remain the Python producer's job.
use std::fmt::Debug;
pub(super) fn object(fields: &[(&str, String)]) -> String {
    let entries: Vec<String> = fields
        .iter()
        .map(|(name, value)| format!("{}:{value}", quote(name)))
        .collect();
    let body = entries.join(",");
    format!("{{{body}}}")
}
pub(super) fn numeric<T: Debug>(value: T) -> String {
    // Call sites use only finite numeric/bool scalars or fixed-size arrays, never debug structs.
    let encoded = format!("{value:?}");
    encoded
}
pub(super) fn quote(value: &str) -> String {
    let mut encoded = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => encoded.push_str("\\\""),
            '\\' => encoded.push_str("\\\\"),
            character if character.is_control() => {
                encoded.push_str(&format!("\\u{:04x}", character as u32))
            }
            character => encoded.push(character),
        }
    }
    encoded.push('"');
    encoded
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn typed_fields_preserve_arrays_and_escape_external_strings() {
        let fields = [("n", numeric([1, 2])), ("s", quote("AMD\n\"GPU\"\\"))];
        assert_eq!(
            object(&fields),
            "{\"n\":[1, 2],\"s\":\"AMD\\u000a\\\"GPU\\\"\\\\\"}"
        );
        assert_eq!(quote("λ"), "\"λ\"");
    }
}
