// SPDX-License-Identifier: GPL-2.0-only
//! Recover the original arithmetic failure through each component boundary.
use super::ConicalStorageFailure;
use super::angle::*;
use super::angular::AngularSpanFailure;
use super::combined::CombinedFailure;
use super::slant::SlantFailure;
use super::spatial::SpatialFailure;
use super::*;
use crate::predicates::roots::metric_bounds::CylindricalProfileFailure;

pub(super) fn combined(failure: &CombinedFailure) -> Option<&BoundedArithmeticFailure> {
    match failure {
        CombinedFailure::Arithmetic(cause) => Some(cause),
        CombinedFailure::Spatial(cause) => spatial(cause),
        CombinedFailure::Storage { cause, .. } => storage(cause),
        _ => None,
    }
}
pub(super) fn spatial(failure: &SpatialFailure) -> Option<&BoundedArithmeticFailure> {
    match failure {
        SpatialFailure::Arithmetic(cause) => Some(cause),
        SpatialFailure::Source(cause) => source(cause),
        SpatialFailure::Original(cause) => original(cause),
        SpatialFailure::Angular(cause) => angular(cause),
        SpatialFailure::Slant(cause) => slant(cause),
        _ => None,
    }
}
pub(super) fn original(failure: &ConicalOriginalFailure) -> Option<&BoundedArithmeticFailure> {
    match failure {
        ConicalOriginalFailure::Arithmetic(cause) => Some(cause),
        ConicalOriginalFailure::Source(cause) => source(cause),
        ConicalOriginalFailure::Polynomial(cause) => polynomial(cause),
        _ => None,
    }
}
pub(super) fn source(failure: &ConicalSourceFailure) -> Option<&BoundedArithmeticFailure> {
    match failure {
        ConicalSourceFailure::Arithmetic(cause) => Some(cause),
        _ => None,
    }
}
pub(super) fn angular(failure: &AngularSpanFailure) -> Option<&BoundedArithmeticFailure> {
    match failure {
        AngularSpanFailure::Arithmetic(cause) => Some(cause),
        AngularSpanFailure::Initial(cause) => initial(cause),
        AngularSpanFailure::Polynomial(cause) => polynomial(cause),
        _ => None,
    }
}
pub(super) fn initial(failure: &InitialAngleFailure) -> Option<&BoundedArithmeticFailure> {
    match failure {
        InitialAngleFailure::Arithmetic(cause) => Some(cause),
        InitialAngleFailure::Trigonometric(cause) => polynomial(cause),
        _ => None,
    }
}
pub(super) fn slant(failure: &SlantFailure) -> Option<&BoundedArithmeticFailure> {
    match failure {
        SlantFailure::Arithmetic(cause) => Some(cause),
        SlantFailure::Polynomial(cause) => polynomial(cause),
        _ => None,
    }
}
pub(super) fn storage(failure: &ConicalStorageFailure) -> Option<&BoundedArithmeticFailure> {
    match failure {
        ConicalStorageFailure::Arithmetic(cause) => Some(cause),
        ConicalStorageFailure::Polynomial(cause) => polynomial(cause),
        _ => None,
    }
}
pub(super) fn polynomial(failure: &CylindricalProfileFailure) -> Option<&BoundedArithmeticFailure> {
    match failure {
        CylindricalProfileFailure::Arithmetic(cause) => Some(cause),
        _ => None,
    }
}
