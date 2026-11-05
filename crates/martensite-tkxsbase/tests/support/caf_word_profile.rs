//! Captured regexp-word-v1 boundaries use actual native prefix/suffix behavior, not Unicode-category assumptions.
//! Tcl8.6.14, system codec iso8859-1, explicit UTF8 conversion; all1,112,064 Unicode scalars qualified.
//! This fixture does not change the historical BOP predicate, whose Unicode limitation remains separate.
const RANGES: &[u8] = include_bytes!("fixtures/caf-word-boundaries.bin");
const SPACES: &[u8] = include_bytes!("fixtures/caf-space-boundaries.bin");

fn range(ranges: &[u8], index: usize) -> (u32, u32) {
    let bytes = &ranges[index * 8..index * 8 + 8];
    let first = u32::from_le_bytes(bytes[..4].try_into().unwrap());
    let last = u32::from_le_bytes(bytes[4..].try_into().unwrap());
    (first, last)
}

/// Classify using the qualified captured range profile; `word_character('¾') == false`.
fn contains(character: char, ranges: &[u8]) -> bool {
    let value = u32::from(character);
    let (mut first, mut after) = (0, ranges.len() / 8);
    while first < after {
        let middle = first + (after - first) / 2;
        let (low, high) = range(ranges, middle);
        if value < low {
            after = middle;
            continue;
        }
        if value > high {
            first = middle + 1;
            continue;
        }
        return true;
    }
    false
}

/// Use qualified native word boundaries; `word_character('¾') == false`.
pub(crate) fn word_character(character: char) -> bool {
    let profile = RANGES;
    contains(character, profile)
}

/// Use exact native regexp whitespace, including U+200B; `space_character('\u{200b}') == true`.
pub(crate) fn space_character(character: char) -> bool {
    let profile = SPACES;
    contains(character, profile)
}

#[cfg(test)]
#[allow(dead_code)] // Only the dedicated profile host invokes these retained assertions.
pub(crate) mod tests {
    use super::{RANGES, SPACES, contains, range, word_character};

    fn assert_range(ranges: &[u8], index: usize) {
        let (first, last) = range(ranges, index);
        assert!(first <= last);
        for value in [first, last] {
            assert!(contains(char::from_u32(value).unwrap(), ranges));
        }
        for value in [first.saturating_sub(1), last + 1] {
            assert!(!contains(char::from_u32(value).unwrap(), ranges));
        }
        if index > 0 {
            assert!(range(ranges, index - 1).1 + 1 < first);
        }
    }

    fn assert_ascii_words() {
        for value in 0..=127 {
            let character = char::from_u32(value).unwrap();
            assert_eq!(
                word_character(character),
                character.is_ascii_alphanumeric() || character == '_'
            );
        }
    }

    /// Check every captured endpoint and the ASCII definition once; `tests::complete_range_endpoints_and_ascii_word_definition_are_consistent()` .
    pub(crate) fn complete_range_endpoints_and_ascii_word_definition_are_consistent() {
        for ranges in [RANGES, SPACES] {
            assert_eq!(ranges.len() % 8, 0);
            for index in 0..ranges.len() / 8 {
                assert_range(ranges, index);
            }
        }
        assert_ascii_words();
    }
}
