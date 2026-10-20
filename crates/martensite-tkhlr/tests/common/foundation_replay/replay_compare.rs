//! Exact retained protocol comparison, normalizing only documented NaN bit tokens.

/// Normalize NaN payload/sign while retaining all other protocol bytes.
pub fn canonical_nan_tokens(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len());
    for field in text.split_inclusive(|ch| !is_word_character(ch)) {
        let token = field.trim_end_matches(|ch| !is_word_character(ch));
        if is_nan_bits(token) {
            normalized.push_str("NaN");
            normalized.push_str(&field[token.len()..]);
        } else {
            normalized.push_str(field);
        }
    }
    normalized
}

fn is_word_character(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

fn is_nan_bits(token: &str) -> bool {
    if !(16..=20).contains(&token.len()) || !token.bytes().all(|byte| byte.is_ascii_digit()) {
        return false;
    }
    token
        .parse::<u64>()
        .is_ok_and(|bits| f64::from_bits(bits).is_nan())
}

#[cfg(test)]
mod tests {
    use super::canonical_nan_tokens;

    #[test]
    fn nan_payload_signs_normalize_without_changing_whitespace() {
        let payloads = [
            0x7ff0_0000_0000_0001_u64,
            0x7ff8_0000_0000_0000,
            0xfff8_0000_0000_0042,
        ];
        for bits in payloads {
            assert_eq!(canonical_nan_tokens(&format!("{bits} \n")), "NaN \n");
        }
    }

    #[test]
    fn finite_signed_zero_infinities_integers_and_exceptions_are_exact() {
        for token in [
            "0",
            "9223372036854775808",
            "4607182418800017408",
            "9218868437227405312",
            "18442240474082181120",
            "18446744073709551616",
            "Standard_ConstructionError",
            "7ff8000000000000",
        ] {
            assert_eq!(canonical_nan_tokens(token), token);
        }
    }

    #[test]
    fn punctuation_is_a_word_boundary_but_underscores_are_not() {
        assert_eq!(canonical_nan_tokens("(9221120237041090560),"), "(NaN),");
        assert_eq!(
            canonical_nan_tokens("_9221120237041090560"),
            "_9221120237041090560"
        );
    }

    #[test]
    fn numeric_substrings_of_text_are_not_bit_tokens() {
        let token = "prefix9221120237041090560suffix";
        assert_eq!(canonical_nan_tokens(token), token);
    }
}
