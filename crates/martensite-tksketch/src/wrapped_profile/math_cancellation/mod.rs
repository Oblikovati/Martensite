//! Recover cancellation from retained mathematical causes without changing numerical behavior.
use martensite_tkmath::predicates::roots::RootIsolationError;
use martensite_tkmath::predicates::roots::bezier_line::{
    BezierLineError, BezierRayError, BezierWireError,
};
use martensite_tkmath::predicates::roots::metric_bounds::*;
use martensite_tkmath::predicates::roots::signs::RootSignError;
mod integrals;
mod profiles;
mod roots;
pub(super) trait MathCancellation {
    fn is_cancelled(&self) -> bool;
}

#[cfg(test)]
mod tests;
