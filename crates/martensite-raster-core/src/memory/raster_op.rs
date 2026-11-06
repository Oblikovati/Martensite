//! The sixteen bitwise raster operations applied to 24-bit RGB pixels.

/// Bits of a pixel that drawing may change: a 24-bit TrueColor visual.
pub(super) const PIXEL_MASK: u32 = 0x00ff_ffff;
/// Copy the source pixel (raster operation 3).
pub(super) const RASTER_COPY: i32 = 3;

/// A raster operation code in `0..16`; the code is the operation's truth table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RasterOp(u8);

impl RasterOp {
    /// Accept a code in `0..16` and reject anything else; `RasterOp::from_code(6)` is xor.
    pub(super) fn from_code(code: i32) -> Option<Self> {
        u8::try_from(code).ok().filter(|code| *code < 16).map(Self)
    }

    /// Combine source and destination bitwise; `RasterOp::from_code(6)?.apply(0xff, 0x0f) == 0xf0`.
    ///
    /// Bit `3 - (2 * source + destination)` of the code holds the result for that bit pair,
    /// so copy is `0b0011` and xor is `0b0110`.
    pub(super) fn apply(self, source: u32, destination: u32) -> u32 {
        let code = self.0;
        let mut result = 0;
        if code & 8 != 0 {
            result |= !source & !destination;
        }
        if code & 4 != 0 {
            result |= !source & destination;
        }
        if code & 2 != 0 {
            result |= source & !destination;
        }
        if code & 1 != 0 {
            result |= source & destination;
        }
        (result & PIXEL_MASK) | (destination & !PIXEL_MASK)
    }
}

impl Default for RasterOp {
    fn default() -> Self {
        Self(RASTER_COPY as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn op(code: i32) -> RasterOp {
        RasterOp::from_code(code).unwrap()
    }

    #[test]
    fn codes_outside_sixteen_are_rejected() {
        assert_eq!(RasterOp::from_code(-1), None);
        assert_eq!(RasterOp::from_code(16), None);
        assert_eq!(RasterOp::default(), op(3));
    }

    #[test]
    fn copy_xor_and_constant_operations_follow_their_truth_tables() {
        assert_eq!(op(3).apply(0x12_3456, 0xab_cdef), 0x12_3456);
        assert_eq!(op(6).apply(0xff_0000, 0x0f_0f0f), 0xf0_0f0f);
        assert_eq!(op(0).apply(0xff_ffff, 0x12_3456), 0);
        assert_eq!(op(15).apply(0, 0), PIXEL_MASK);
        assert_eq!(op(5).apply(0xff_ffff, 0x12_3456), 0x12_3456);
        assert_eq!(op(10).apply(0, 0x00_00ff), 0xff_ff00);
    }

    #[test]
    fn every_operation_matches_its_single_bit_truth_table() {
        for code in 0..16 {
            for (source, destination) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                let expected = (code >> (3 - (2 * source + destination))) & 1;
                assert_eq!(
                    op(code).apply(source as u32, destination as u32) & 1,
                    expected as u32
                );
            }
        }
    }
}
