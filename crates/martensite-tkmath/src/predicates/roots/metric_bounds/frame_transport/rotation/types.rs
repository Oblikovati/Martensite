// SPDX-License-Identifier: GPL-2.0-only
//! Original rotation inputs and effective work limits travel with every success or refusal.
use super::*;
/// Bounded proof of sine/cosine for the exact supplied binary64 angle.
#[derive(Clone, Debug, PartialEq)]
pub struct FrameRotationReport {
    /// Requested angle, in radians, before any argument reduction.
    pub angle: f64,
    /// Effective fixed-grid precision.
    pub fractional_bits: u16,
    /// Finite arithmetic operation allowance.
    pub operation_limit: u32,
    /// Actually consumed bounded arithmetic operations.
    pub operations: u32,
    /// Certified cosine enclosure.
    pub cosine_bounds: [f64; 2],
    /// Certified sine enclosure.
    pub sine_bounds: [f64; 2],
}
/// Typed reason a rotation-aware frame certificate was not returned.
#[derive(Clone, Debug, PartialEq)]
pub enum FrameRotationFailure {
    /// The requested angle was not finite.
    InvalidAngle,
    /// Reference normalization or stored-map qualification failed.
    Frame(FrameTransportError),
    /// Bounded trigonometric argument reduction or arithmetic did not complete.
    Trigonometric(Box<CylindricalProfileFailure>),
}
/// Original angle and effective arithmetic settings accompany a refusal.
#[derive(Clone, Debug, PartialEq)]
pub struct FrameRotationError {
    /// Original supplied angle.
    pub angle: f64,
    /// Effective fixed-grid precision.
    pub fractional_bits: u16,
    /// Finite arithmetic operation allowance.
    pub operation_limit: u32,
    /// Original typed cause.
    pub cause: FrameRotationFailure,
}
impl std::fmt::Display for FrameRotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "frame rotation {self:?}; expected a finite angle with completed bounded trigonometry and a qualified positive frame"
        )
    }
}
impl std::error::Error for FrameRotationError {}
/// Stored-map qualification and error against the independently rotated ideal reference.
#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedRotatedFrameTransport {
    pub(super) frame: CertifiedFrameTransport,
    pub(super) matrix_error: f64,
    pub(super) rotation: FrameRotationReport,
}
impl CertifiedRotatedFrameTransport {
    /// Original normalized reference without rotation; `proof.unrotated_frame().input()`.
    pub fn unrotated_frame(&self) -> &CertifiedFrameTransport {
        &self.frame
    }
    /// Exact requested reference and stored coefficients; `proof.input().stored`.
    pub fn input(&self) -> FrameTransportInput {
        self.frame.input()
    }
    /// Certified operator-error upper bound against the rotated ideal frame; `proof.matrix_error_bound()`.
    pub fn matrix_error_bound(&self) -> f64 {
        self.matrix_error
    }
    /// Stored map's positive stretch bounds; `proof.stretch_bounds()[0] > 0.`.
    pub fn stretch_bounds(&self) -> [f64; 2] {
        self.frame.stretch_bounds()
    }
    /// Actual stored map's strictly positive determinant enclosure; `proof.determinant_bounds()`.
    pub fn determinant_bounds(&self) -> [f64; 2] {
        self.frame.determinant_bounds()
    }
    /// Original angle, effective arithmetic settings and retained trigonometric bounds.
    /// Example: `proof.rotation().angle`.
    pub fn rotation(&self) -> &FrameRotationReport {
        &self.rotation
    }
}
