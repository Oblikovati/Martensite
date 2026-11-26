// SPDX-License-Identifier: GPL-2.0-only
//! Owned font operation results with explicit consumed-input coverage (#341).

/// Explicit coverage accounting for a font/text operation (#341).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FontConsumedCoverage {
    /// Total bytes supplied by the caller.
    pub input_bytes: usize,
    /// Bytes actually consumed by the operation.
    pub consumed_bytes: usize,
    /// Unicode scalar values consumed.
    pub consumed_chars: usize,
    /// Number of glyphs resolved via fallback faces.
    pub fallback_glyphs: usize,
}

impl FontConsumedCoverage {
    /// True when every supplied byte was consumed.
    pub fn is_complete(&self) -> bool {
        self.consumed_bytes == self.input_bytes
    }

    /// Number of unconsumed trailing bytes.
    pub fn remaining_bytes(&self) -> usize {
        self.input_bytes.saturating_sub(self.consumed_bytes)
    }
}

/// Owned font operation output paired with consumed-input coverage (#341).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontOperationResult<T> {
    value: T,
    coverage: FontConsumedCoverage,
}

impl<T> FontOperationResult<T> {
    /// Pairs an owned result value with its consumed-input coverage.
    pub fn new(value: T, coverage: FontConsumedCoverage) -> Self {
        Self { value, coverage }
    }

    /// Borrow the owned operation output.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Consume the wrapper and return the owned output.
    pub fn into_value(self) -> T {
        self.value
    }

    /// Consumed-input coverage metrics.
    pub fn coverage(&self) -> FontConsumedCoverage {
        self.coverage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_font_result_reports_complete_and_partial_coverage() {
        let full = FontOperationResult::new(
            vec![65_u32, 66_u32],
            FontConsumedCoverage {
                input_bytes: 2,
                consumed_bytes: 2,
                consumed_chars: 2,
                fallback_glyphs: 0,
            },
        );
        assert!(full.coverage().is_complete());
        assert_eq!(full.coverage().remaining_bytes(), 0);
        assert_eq!(full.value(), &[65, 66]);

        let partial = FontOperationResult::new(
            vec![65_u32],
            FontConsumedCoverage {
                input_bytes: 4,
                consumed_bytes: 1,
                consumed_chars: 1,
                fallback_glyphs: 1,
            },
        );
        assert!(!partial.coverage().is_complete());
        assert_eq!(partial.coverage().remaining_bytes(), 3);
    }
}
