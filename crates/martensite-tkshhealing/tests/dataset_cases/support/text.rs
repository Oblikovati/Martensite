//! Text handling applied to scenario results: captures of
//! numbers after a literal prefix, `lindex` and `llength` of a result list.

/// The `[-0-9.+eE]+` word at the start of `text` (after the spaces the pattern allows).
fn number_word(text: &str) -> Option<(&str, &str)> {
    let text = text.trim_start_matches(' ');
    let end = text
        .find(|c: char| !(c.is_ascii_digit() || "-.+eE".contains(c)))
        .unwrap_or(text.len());
    (end > 0).then(|| (&text[..end], &text[end..]))
}

/// `regexp {<prefix> *([-0-9.+eE]+) +([-0-9.+eE]+) ...} text`: the `count` numbers that
/// follow the first `prefix` in `text`, separated by blanks (or by `sep` and blanks).
pub fn numbers_after(text: &str, prefix: &str, count: usize, sep: &str) -> Option<Vec<f64>> {
    let at = text.find(prefix)?;
    let mut rest = &text[at + prefix.len()..];
    let mut values = Vec::new();
    for index in 0..count {
        if index > 0 {
            rest = rest.trim_start_matches(' ');
            rest = rest.strip_prefix(sep).unwrap_or(rest);
        }
        let (word, tail) = number_word(rest)?;
        values.push(word.parse().ok()?);
        rest = tail;
    }
    Some(values)
}

/// The number of words in a result list (none of the cases count
/// has braces).
pub fn llength(text: &str) -> usize {
    text.split_whitespace().count()
}

/// `lindex text index`.
pub fn lindex(text: &str, index: usize) -> &str {
    text.split_whitespace().nth(index).unwrap_or("")
}

#[test]
fn numbers_follow_the_regexps() {
    let dump = "    - PCurve : 1 on surface 1, range : 0 6.283185307179586\n";
    assert_eq!(
        numbers_after(dump, "PCurve : 1 on surface 1, range : ", 2, ""),
        Some(vec![0.0, std::f64::consts::TAU])
    );
    assert_eq!(
        numbers_after("Origin :1, 2, -3e-2", "Origin :", 3, ","),
        Some(vec![1.0, 2.0, -0.03])
    );
    assert_eq!(
        llength("\"distmini\" command returns:\nd_val d d2 \nOutput is complete.\n"),
        9
    );
}

/// `regexp { *Label *: *(num)[sep *(num)]...}`: the numbers after the first `label`
/// followed by blanks and a colon.
pub fn labeled_numbers(text: &str, label: &str, count: usize, sep: &str) -> Option<Vec<f64>> {
    let mut from = 0;
    while let Some(at) = text[from..].find(label) {
        let rest = text[from + at + label.len()..].trim_start_matches(' ');
        if let Some(rest) = rest.strip_prefix(':') {
            return numbers_after(rest, "", count, sep);
        }
        from += at + label.len();
    }
    None
}

/// `regexp {([-0-9.+eE]+)$} text`: the number that ends the text.
pub fn last_number(text: &str) -> Option<f64> {
    let end = text.trim_end_matches('\n');
    let start = end
        .rfind(|c: char| !(c.is_ascii_digit() || "-.+eE".contains(c)))
        .map_or(0, |at| at + 1);
    end[start..].parse().ok()
}
