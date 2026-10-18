//! Retains the original decimal-token NaN classification policy and exact text.
fn word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn nan_token(text: &str, start: usize, end: usize) -> bool {
    let digits = end - start;
    let bytes = text.as_bytes();
    if !(16..=20).contains(&digits)
        || start > 0 && word_byte(bytes[start - 1])
        || end < bytes.len() && word_byte(bytes[end])
    {
        return false;
    }
    let Ok(bits) = text[start..end].parse::<u64>() else {
        return false;
    };
    bits & 0x7ff0_0000_0000_0000 == 0x7ff0_0000_0000_0000 && bits & 0x000f_ffff_ffff_ffff != 0
}

fn decimal_ranges(text: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut index = 0;
    while index < text.len() {
        if !text.as_bytes()[index].is_ascii_digit() {
            index += 1;
            continue;
        }
        let start = index;
        while index < text.len() && text.as_bytes()[index].is_ascii_digit() {
            index += 1;
        }
        ranges.push((start, index));
    }
    ranges
}

pub fn canonical_nan_tokens(text: &str) -> String {
    assert!(
        text.is_ascii(),
        "non-ASCII numeric receipt; expected ASCII tokens"
    );
    let mut copied = 0;
    let mut result = String::new();
    for (start, end) in decimal_ranges(text) {
        if !nan_token(text, start, end) {
            continue;
        }
        result.push_str(&text[copied..start]);
        result.push_str("NaN");
        copied = end;
    }
    result.push_str(&text[copied..]);
    result
}

pub fn assert_receipt(request: &str, expected: &str, actual: &str) {
    if expected == actual {
        return;
    }
    assert!(
        canonical_nan_tokens(expected) == canonical_nan_tokens(actual),
        "request={request:?}; expected exact token text and finite/infinite/signed-zero bits (only NaN payload/sign may differ)"
    );
}

#[cfg(test)]
mod tests;
