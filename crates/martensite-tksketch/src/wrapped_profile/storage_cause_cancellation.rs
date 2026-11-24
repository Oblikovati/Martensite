//! Preserve cancellation only when it is retained in the storage failure cause.
use super::*;
use crate::surface_sketch::wrapped_profile::{
    CancellationCause, math_cancellation::MathCancellation,
};
impl StorageFailure {
    pub(in crate::surface_sketch::wrapped_profile) fn is_cancelled(&self) -> bool {
        self.cancelled()
    }
}
impl CancellationCause for StorageFailure {
    fn cancelled(&self) -> bool {
        match &self.cause {
            StorageCause::Certificate(cause) => cause.cause.is_cancelled(),
            StorageCause::Profile(cause) => cause.cause.is_cancelled(),
            StorageCause::Lift(cause) => cause.cause.is_cancelled(),
            StorageCause::CapResidual(cause) => cause.cause.is_cancelled(),
            StorageCause::Flux(cause) => cause.cause.is_cancelled(),
            StorageCause::Comparison(cause) => cause.cause.is_cancelled(),
            StorageCause::Depths(cause) => cause.is_cancelled(),
            StorageCause::Trim { cause, .. } => cause.cancelled(),
            StorageCause::Native(..)
            | StorageCause::Tolerance { .. }
            | StorageCause::Mismatch { .. } => false,
        }
    }
}
