// SPDX-License-Identifier: GPL-2.0-only
//! Owned coefficient bindings; no native handles or mutable algorithm state.

/// Requested axis/radial reference and actual stored linear matrix, in row order.
/// Example: `FrameTransportInput { axis: [0.,0.,1.], radial: [1.,0.,0.], stored: [[1.,0.,0.],[0.,1.,0.],[0.,0.,1.]] }`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameTransportInput {
    /// Nonzero reference axis; normalization is mathematical, not rounded first.
    pub axis: [f64; 3],
    /// Reference radial vector, not parallel to the axis.
    pub radial: [f64; 3],
    /// Actual stored coefficients to compare with the ideal right-handed frame.
    pub stored: [[f64; 3]; 3],
}
/// Certified operator-error, stretch and determinant bounds for these exact inputs.
/// This does not admit native transforms or certify a caller's point-domain bounds.
#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedFrameTransport {
    pub(super) input: FrameTransportInput,
    pub(super) matrix_error: f64,
    pub(super) stretch: [f64; 2],
    pub(super) determinant: [f64; 2],
    pub(super) gram_residual: f64,
}
impl CertifiedFrameTransport {
    /// Exact binary64 inputs bound to this evidence; `proof.input().stored`.
    pub fn input(&self) -> FrameTransportInput {
        self.input
    }
    /// Upper bound on the spectral norm of stored minus ideal frame; `proof.matrix_error_bound()`.
    pub fn matrix_error_bound(&self) -> f64 {
        self.matrix_error
    }
    /// Positive lower and finite upper stretch bounds; `proof.stretch_bounds()`.
    pub fn stretch_bounds(&self) -> [f64; 2] {
        self.stretch
    }
    /// Outward enclosure of the strictly positive stored determinant; `proof.determinant_bounds()`.
    pub fn determinant_bounds(&self) -> [f64; 2] {
        self.determinant
    }
    /// Upper bound on maximum absolute Gram-residual row sum; `proof.gram_residual_upper()`.
    pub fn gram_residual_upper(&self) -> f64 {
        self.gram_residual
    }
}
