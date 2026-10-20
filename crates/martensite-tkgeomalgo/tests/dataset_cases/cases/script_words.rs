//! The word list and string operations the case scripts apply to command results.

/// The words of a word list of plain words (`llength`, `lindex`).
pub fn words(list: &str) -> Vec<&str> {
    list.split_whitespace().collect()
}

/// `lindex $list end` (an empty string for an empty list).
pub fn last_word(list: &str) -> &str {
    words(list).last().copied().unwrap_or("")
}

/// `lindex $list end-1`.
pub fn second_last_word(list: &str) -> &str {
    let words = words(list);
    if words.len() < 2 {
        return "";
    }
    words[words.len() - 2]
}

/// The first number after `prefix` in `text`, as script `regexp {prefix +([-0-9.+eE]+)}`
/// captures it (`None` when the pattern does not match).
pub fn number_after<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    let start = text.find(prefix)? + prefix.len();
    let rest = text[start..].trim_start_matches(' ');
    let end = rest
        .find(|c: char| !(c.is_ascii_digit() || "-.+eE".contains(c)))
        .unwrap_or(rest.len());
    (end > 0).then(|| &rest[..end])
}

/// script `string range $text first last` (character indices, inclusive).
pub fn string_range(text: &str, first: usize, last: usize) -> String {
    text.chars().skip(first).take(last + 1 - first).collect()
}

#[test]
fn script_word_helpers() {
    assert_eq!(words(" a 3d curve"), ["a", "3d", "curve"]);
    assert_eq!(last_word(""), "");
    assert_eq!(second_last_word(" a 3d curve"), "3d");
    assert_eq!(
        number_after("ext_1  parameter 1 = 0.75\n", "parameter 1 ="),
        Some("0.75")
    );
    assert_eq!(string_range("abcdef", 1, 3), "bcd");
}
