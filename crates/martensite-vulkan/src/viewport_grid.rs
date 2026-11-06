//! Explicit viewport quantization never infers an exact grid from a device's minimum precision.
use std::fmt;

/// Owned window-coordinate precision with nearest-even ties.
/// Example: `RenderViewportGridPrecision::Fractional12.snap(0.5 / 4096.)?` returns zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RenderViewportGridPrecision {
    /// Eight fractional bits per window coordinate.
    Fractional8,
    /// Ten fractional bits per window coordinate.
    Fractional10,
    /// Twelve fractional bits per window coordinate.
    Fractional12,
}

/// Nonfinite input or arithmetic is incomplete, never a silently clamped grid coordinate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderViewportGridFailure {
    /// Original input does not describe a finite coordinate.
    NonFiniteCoordinate {
        /// Original binary64 coordinate bits, including the nonfinite payload.
        bits: u64,
    },
    /// Scaling the finite coordinate exceeded finite binary64 arithmetic.
    ScaledCoordinateOverflow {
        /// Original finite binary64 coordinate bits.
        bits: u64,
        /// Declared grid whose scaling exceeded the finite coordinate range.
        precision: RenderViewportGridPrecision,
    },
}

impl RenderViewportGridPrecision {
    /// Return the declared bit count; `RenderViewportGridPrecision::Fractional8.fractional_bits()` is `8`.
    pub const fn fractional_bits(self) -> u32 {
        match self {
            Self::Fractional8 => 8,
            Self::Fractional10 => 10,
            Self::Fractional12 => 12,
        }
    }

    /// Round one finite coordinate with nearest-even ties; `RenderViewportGridPrecision::Fractional8.snap(1.)?` is `1`.
    pub fn snap(self, coordinate: f64) -> Result<f64, RenderViewportGridFailure> {
        if !coordinate.is_finite() {
            return Err(RenderViewportGridFailure::NonFiniteCoordinate {
                bits: coordinate.to_bits(),
            });
        }
        let scale = f64::from(1_u32 << self.fractional_bits());
        let scaled = coordinate * scale;
        if !scaled.is_finite() {
            return Err(RenderViewportGridFailure::ScaledCoordinateOverflow {
                bits: coordinate.to_bits(),
                precision: self,
            });
        }
        Ok(scaled.round_ties_even() / scale)
    }
}

impl fmt::Display for RenderViewportGridFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteCoordinate { bits } => write!(
                formatter,
                "viewport coordinate bits={bits:#018x}; expected finite binary64 coordinate"
            ),
            Self::ScaledCoordinateOverflow { bits, precision } => write!(
                formatter,
                "viewport coordinate bits={bits:#018x}, precision={precision:?}; expected finite scaled coordinate"
            ),
        }
    }
}

impl std::error::Error for RenderViewportGridFailure {}

#[cfg(test)]
#[path = "viewport_grid_tests.rs"]
mod tests;
