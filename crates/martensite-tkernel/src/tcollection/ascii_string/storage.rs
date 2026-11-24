//! The static empty buffer differs from an owned buffer truncated to zero.
use super::AsciiString;

// Empty storage marks the static empty string without allocating or mutating it.
pub(super) static SHARED_EMPTY_TERMINATOR: [u8; 1] = [0];

pub(super) fn padded_byte_capacity(length: usize) -> usize {
    // The padded size reserves a terminator and rounds to four bytes.
    (length + 4) & !3
}

impl AsciiString {
    pub(super) fn ensure_owned_terminator(&mut self) {
        if self.terminated.is_empty() {
            self.terminated.push(0);
        }
    }
}

impl Clone for AsciiString {
    fn clone(&self) -> Self {
        Self::from_raw(self.as_bytes())
    }

    fn clone_from(&mut self, source: &Self) {
        self.copy(source);
    }
}

impl PartialEq for AsciiString {
    fn eq(&self, other: &Self) -> bool {
        // Equality compares logical bytes, independent of empty buffer ownership.
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for AsciiString {}

#[cfg(test)]
mod tests {
    use super::{AsciiString, padded_byte_capacity};

    #[test]
    fn padding_includes_a_terminator_at_every_four_byte_boundary() {
        for (length, expected) in [(0, 4), (1, 4), (3, 4), (4, 8), (7, 8), (8, 12)] {
            assert_eq!(padded_byte_capacity(length), expected);
        }
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn short_ascii_uses_inline_storage_within_a_vec_sized_value() {
        assert_eq!(
            std::mem::size_of::<AsciiString>(),
            std::mem::size_of::<Vec<u8>>()
        );
        let inline = AsciiString::filled(15, b'a');
        let allocated = AsciiString::filled(16, b'a');
        assert!(!inline.terminated.spilled());
        assert!(allocated.terminated.spilled());
        assert_eq!(inline.as_bytes_with_nul(), b"aaaaaaaaaaaaaaa\0");
    }

    #[test]
    fn append_reuses_capacity_and_keeps_embedded_nuls_distinct_from_terminator() {
        let mut text = AsciiString::default();
        let mut growths = 0;
        for _ in 0..4096 {
            let capacity = text.terminated.capacity();
            text.append_bytes(b"\0a");
            growths += usize::from(text.terminated.capacity() != capacity);
        }
        assert!(
            growths <= 16,
            "{growths} allocations for 4096 appends; expected amortized growth"
        );
        assert!(
            text.as_bytes()
                .as_chunks::<2>()
                .0
                .iter()
                .all(|bytes| bytes == b"\0a")
        );
        assert_eq!(text.len(), 8192);
        assert_eq!(text.as_bytes_with_nul().last(), Some(&0));
    }
}
