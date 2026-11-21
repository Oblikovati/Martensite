//! A shared translation cancels; separately rounded evaluations still contribute.
use super::*;

pub(in super::super) fn transport_trace(
    residual: f64,
    stretch: f64,
    evaluation: f64,
) -> Checked<f64> {
    let scaled = scale_upper(residual, stretch, Quantity::Trace)?;
    let evaluations = scale_upper(evaluation, 2., Quantity::Trace)?;
    add_upper(scaled, evaluations, Quantity::Trace)
}
