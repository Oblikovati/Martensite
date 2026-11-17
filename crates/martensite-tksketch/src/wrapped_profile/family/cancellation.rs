//! Typed cancellation survives family error-first propagation without losing its cause (#526).
use super::*;
mod sources;
use crate::surface_sketch::wrapped_profile::math_cancellation::MathCancellation;

impl CancellationCause for FamilyFailure {
    fn cancelled(&self) -> bool {
        match self {
            Self::Selection(cause) => cause.cancelled(),
            Self::Cancelled(_) => true,
            Self::Member(cause) => cause.cancelled(),
            Self::Binding(cause) => cause.cancelled(),
            Self::History(cause) => cause.cancelled(),
            Self::Native { .. } | Self::Encoding(_) | Self::Coverage(_) | Self::Budget(_) => false,
        }
    }
}
impl CancellationCause for Refusal {
    fn cancelled(&self) -> bool {
        self.cause.cancelled()
    }
}
impl CancellationCause for RefusalCause {
    fn cancelled(&self) -> bool {
        match self {
            Self::Native(_) | Self::Coverage(_) => false,
            Self::NormalMeasure(cause) => cause.cancelled(),
            Self::Storage(cause) => cause.is_cancelled(),
            Self::Cones(cause) => cause.cancelled(),
            #[cfg(test)]
            Self::Planning(cause) => cause.source_cause().is_cancelled(),
            #[cfg(test)]
            Self::Planar(cause) => cause.cancelled(),
            #[cfg(test)]
            Self::Clearance(cause) => cause.cancelled(),
        }
    }
}
impl CancellationCause for normal_measure::NormalMeasureFailure {
    fn cancelled(&self) -> bool {
        match self {
            Self::Sources(cause) => cause.cancelled(),
            Self::Storage(cause) => cause.is_cancelled(),
            Self::Certificate(cause) => cause.is_cancelled(),
            Self::SpanSequence { .. } => false,
        }
    }
}

#[cfg(test)]
mod tests;
