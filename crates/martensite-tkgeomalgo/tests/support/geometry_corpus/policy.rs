//! Original numeric comparator: exact text/bits except decimal NaN bit classification.
pub fn equivalent(expected: &str, actual: &str) -> bool {
    canonical_nan_tokens(expected) == canonical_nan_tokens(actual)
}

pub fn difference(expected: &str, actual: &str) -> String {
    let offset = expected
        .bytes()
        .zip(actual.bytes())
        .position(|(left, right)| left != right)
        .unwrap_or(expected.len().min(actual.len()));
    format!(
        "output differs at byte {offset}; expected {} bytes, actual {} bytes",
        expected.len(),
        actual.len()
    )
}

fn canonical_nan_tokens(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut cursor = 0;
    while cursor < text.len() {
        cursor = append_protocol_span(text, cursor, &mut output);
    }
    output
}

fn append_protocol_span(text: &str, start: usize, output: &mut String) -> usize {
    if !text.as_bytes()[start].is_ascii_digit() {
        let character = text[start..].chars().next().unwrap();
        output.push(character);
        return start + character.len_utf8();
    }
    let end = start
        + text.as_bytes()[start..]
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
    output.push_str(if nan_decimal_token(text, start, end) {
        "NaN"
    } else {
        &text[start..end]
    });
    end
}

fn nan_decimal_token(text: &str, start: usize, end: usize) -> bool {
    if !(16..=20).contains(&(end - start)) {
        return false;
    }
    let word = |c: char| c.is_alphanumeric() || c == '_';
    if text[..start].chars().next_back().is_some_and(word)
        || text[end..].chars().next().is_some_and(word)
    {
        return false;
    }
    text[start..end].parse::<u64>().is_ok_and(|bits| {
        bits & 0x7ff0_0000_0000_0000 == 0x7ff0_0000_0000_0000 && bits & 0x000f_ffff_ffff_ffff != 0
    })
}

pub fn fixture_bytes_hash(parts: impl Iterator<Item = &'static str>) -> u64 {
    parts
        .flat_map(str::bytes)
        .fold(0xcbf29ce484222325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
        })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nan_payload_and_sign_are_classification_only() {
        assert!(equivalent(
            "9221120237041090560 \n",
            "18444492273895866369 \n"
        ));
        assert!(!equivalent("9218868437227405312", "9221120237041090560"));
    }
    #[test]
    fn finite_infinity_signed_zero_and_exception_tokens_stay_exact() {
        for (left, right) in [
            ("0", "9223372036854775808"),
            ("4607182418800017408", "4607182418800017409"),
            ("9218868437227405312", "18442240474082181120"),
            ("Standard_OutOfRange", "Standard_DomainError"),
            ("1 \n", "1\n"),
        ] {
            assert!(!equivalent(left, right));
        }
        assert!(equivalent(
            "Standard_OutOfRange \n",
            "Standard_OutOfRange \n"
        ));
    }
    #[test]
    fn encoded_dump_text_and_word_boundaries_are_not_nan_rewritten() {
        for text in [
            "t9221120237041090560",
            "_9221120237041090560",
            "9221120237041090560x",
            "18446744073709551616",
        ] {
            assert_eq!(canonical_nan_tokens(text), text);
        }
        assert_eq!(canonical_nan_tokens("[9221120237041090560]"), "[NaN]");
        assert!(difference("a", "b").contains("byte 0"));
    }
}
