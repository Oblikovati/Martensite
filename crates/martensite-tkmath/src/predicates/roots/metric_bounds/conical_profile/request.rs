// SPDX-License-Identifier: GPL-2.0-only
//! Request admission and source-bound conical clearance publication.
use super::*;
/// Fixed-grid precision and work allowance for a conical clearance proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConeClearanceOptions {
    /// Fractional bits, from 8 through 4096.
    pub fractional_bits: u16,
    /// Maximum charged operations, at most one million.
    pub max_operations: u32,
}
impl Default for ConeClearanceOptions {
    fn default() -> Self {
        Self {
            fractional_bits: 128,
            max_operations: 100_000,
        }
    }
}
/// Explicit polynomial span and normal-offset clearance request.
pub struct ConeRequest<'a> {
    /// Original developed polynomial poles.
    pub poles: &'a [[f64; 2]],
    /// Finite cone semi-angle in radians.
    pub semi_angle: f64,
    /// Increasing normal-depth endpoints.
    pub depths: [f64; 2],
    /// Whether positive depth follows the outward normal.
    pub outward: bool,
    /// Required radial separation from the axis.
    pub clearance: f64,
    /// Maximum dyadic subdivision level, at most 64; no implicit clamp is applied.
    pub max_depth: u8,
    /// Fixed-grid precision and work allowance.
    pub options: ConeClearanceOptions,
}
/// Original request and typed reason no clearance proof was returned.
#[derive(Debug)]
pub struct ConeRequestError {
    pub(super) source_bits: Vec<[u64; 2]>,
    pub(super) semi_angle_bits: u64,
    pub(super) depths_bits: [u64; 2],
    pub(super) outward: bool,
    pub(super) clearance_bits: u64,
    pub(super) max_depth: u8,
    pub(super) fractional_bits: u16,
    pub(super) max_operations: u32,
    pub(super) operations: u32,
    pub(super) cause: Box<ConicalDepthFailure>,
}
/// Completed source-bound proof and requested work policy.
#[derive(Debug)]
pub struct ConeRequestReceipt {
    pub(super) result: ConeReceipt,
    pub(super) fractional_bits: u16,
    pub(super) max_operations: u32,
}
/// Certify that the whole polynomial span remains clear of the cone axis over its normal depths.
/// Example: `let proof = certify_cone_request(request, || false)?;`.
pub fn certify_cone_request(
    request: ConeRequest<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConeRequestReceipt, ConeRequestError> {
    admit_request_policy(&request)?;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: request.options.fractional_bits,
            max_operations: request.options.max_operations,
        },
        &mut cancelled,
    );
    let result = execute_request(&request, &mut arithmetic)?;
    Ok(ConeRequestReceipt {
        result,
        fractional_bits: request.options.fractional_bits,
        max_operations: request.options.max_operations,
    })
}
pub(super) fn execute_request(
    request: &ConeRequest<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<ConeReceipt, ConeRequestError> {
    prove_owned_cone_interval(
        request.poles,
        request.semi_angle,
        request.depths,
        request.outward,
        request.clearance,
        request.max_depth,
        arithmetic,
    )
    .map_err(|cause| cone_request_error(request, arithmetic.operations(), cause))
}
pub(super) fn admit_request_policy(request: &ConeRequest<'_>) -> Result<(), ConeRequestError> {
    if !(8..=4096).contains(&request.options.fractional_bits)
        || request.options.max_operations > 1_000_000
    {
        let cause = ConicalDepthFailure::InvalidPolicy {
            fractional_bits: request.options.fractional_bits,
            max_operations: request.options.max_operations,
        };
        return Err(cone_request_error(request, 0, cause));
    }
    if request.max_depth > 64 {
        return Err(cone_request_error(
            request,
            0,
            ConicalDepthFailure::InvalidDepthLimit {
                requested: request.max_depth,
            },
        ));
    }
    Ok(())
}
pub(super) fn cone_request_error(
    request: &ConeRequest<'_>,
    operations: u32,
    cause: ConicalDepthFailure,
) -> ConeRequestError {
    ConeRequestError {
        source_bits: request
            .poles
            .iter()
            .map(|point| point.map(f64::to_bits))
            .collect(),
        semi_angle_bits: request.semi_angle.to_bits(),
        depths_bits: request.depths.map(f64::to_bits),
        outward: request.outward,
        clearance_bits: request.clearance.to_bits(),
        max_depth: request.max_depth,
        fractional_bits: request.options.fractional_bits,
        max_operations: request.options.max_operations,
        operations,
        cause: Box::new(cause),
    }
}

#[test]
pub(super) fn cone_request_failure_retains_physical_inputs() {
    let request = failure_request(
        &[[1., -0.], [2., 0.]],
        [1., -1.],
        false,
        0.25,
        ConeClearanceOptions::default(),
    );
    let cancelled = || false;
    let error = certify_cone_request(request, cancelled).unwrap_err();
    assert_retained_physical_inputs(&error);
}

#[test]
pub(super) fn cone_request_refuses_invalid_policy_before_arithmetic() {
    let options = ConeClearanceOptions {
        fractional_bits: 7,
        max_operations: 1_000_001,
    };
    let request = failure_request(&[[1., 0.], [2., 0.]], [-1., 1.], true, 0., options);
    let cancelled = || false;
    let error = certify_cone_request(request, cancelled).unwrap_err();
    assert!(matches!(
        *error.cause,
        ConicalDepthFailure::InvalidPolicy {
            fractional_bits: 7,
            max_operations: 1_000_001
        }
    ));
}

#[test]
pub(super) fn successful_cone_request_retains_work_policy_and_proof() {
    let request = ConeRequest {
        poles: &[[3., 0.], [3., 1.]],
        semi_angle: 0.5,
        depths: [-1., 1.],
        outward: true,
        clearance: 0.,
        max_depth: 2,
        options: ConeClearanceOptions::default(),
    };
    let cancelled = || false;
    let receipt = certify_cone_request(request, cancelled).unwrap();
    assert_eq!(
        (receipt.fractional_bits, receipt.max_operations),
        (128, 100_000)
    );
    assert!(receipt.result.proof.operations > 0);
}

impl ConicalDistanceProof {
    /// Proven minimum squared source distance; `proof.minimum_squared()`.
    pub fn minimum_squared(&self) -> &DyadicBound {
        &self.minimum_squared
    }
    /// Fractional grid precision; `proof.fractional_bits()`.
    pub fn fractional_bits(&self) -> u16 {
        self.fractional_bits
    }
    /// Aggregate charged work; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Number of proven dyadic leaves; `proof.leaves()`.
    pub fn leaves(&self) -> usize {
        self.leaves
    }
}
impl ConeRequestReceipt {
    /// Source-bound clearance result; `receipt.result()`.
    pub fn result(&self) -> &ConeReceipt {
        &self.result
    }
    /// Requested arithmetic policy; `receipt.options()`.
    pub fn options(&self) -> ConeClearanceOptions {
        ConeClearanceOptions {
            fractional_bits: self.fractional_bits,
            max_operations: self.max_operations,
        }
    }
}
impl ConeReceipt {
    /// Proven squared source-distance bound; `receipt.proof()`.
    pub fn proof(&self) -> &ConicalDistanceProof {
        &self.proof
    }
    /// Source coordinate bits; `receipt.source_bits()`.
    pub fn source_bits(&self) -> &[[u64; 2]] {
        &self.source_bits
    }
    /// Original semi-angle; `receipt.semi_angle()`.
    pub fn semi_angle(&self) -> f64 {
        f64::from_bits(self.semi_angle_bits)
    }
    /// Original normal-depth interval; `receipt.depths()`.
    pub fn depths(&self) -> [f64; 2] {
        self.depths_bits.map(f64::from_bits)
    }
    /// Requested normal direction; `receipt.outward()`.
    pub fn outward(&self) -> bool {
        self.outward
    }
    /// Required axis clearance; `receipt.clearance()`.
    pub fn clearance(&self) -> f64 {
        f64::from_bits(self.clearance_bits)
    }
    /// Requested subdivision level; `receipt.requested_depth()`.
    pub fn requested_depth(&self) -> u8 {
        self.requested_depth
    }
    /// Certified sine enclosure; `receipt.sine()`.
    pub fn sine(&self) -> &[DyadicBound; 2] {
        &self.sine
    }
    /// Certified cosine enclosure; `receipt.cosine()`.
    pub fn cosine(&self) -> &[DyadicBound; 2] {
        &self.cosine
    }
    /// Certified required squared slant enclosure; `receipt.threshold()`.
    pub fn threshold(&self) -> &[DyadicBound; 2] {
        &self.threshold
    }
}

impl ConeRequestError {
    /// Original source coordinate bits; `error.source_bits()`.
    pub fn source_bits(&self) -> &[[u64; 2]] {
        &self.source_bits
    }
    /// Original semi-angle; `error.semi_angle()`.
    pub fn semi_angle(&self) -> f64 {
        f64::from_bits(self.semi_angle_bits)
    }
    /// Original normal depths; `error.depths()`.
    pub fn depths(&self) -> [f64; 2] {
        self.depths_bits.map(f64::from_bits)
    }
    /// Original normal direction; `error.outward()`.
    pub fn outward(&self) -> bool {
        self.outward
    }
    /// Requested axis clearance; `error.clearance()`.
    pub fn clearance(&self) -> f64 {
        f64::from_bits(self.clearance_bits)
    }
    /// Requested subdivision level; `error.max_depth()`.
    pub fn max_depth(&self) -> u8 {
        self.max_depth
    }
    /// Requested arithmetic settings; `error.options()`.
    pub fn options(&self) -> ConeClearanceOptions {
        ConeClearanceOptions {
            fractional_bits: self.fractional_bits,
            max_operations: self.max_operations,
        }
    }
    /// Completed bounded operations; `error.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Original typed cause; `error.cause()`.
    pub fn cause(&self) -> &ConicalDepthFailure {
        &self.cause
    }
}
impl std::fmt::Display for ConeRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "conical clearance {:?}; expected 2..=4 finite poles, finite semi-angle in (0, pi/2), increasing finite depths, finite nonnegative clearance, subdivision <= 64, precision 8..=4096, operations <= 1000000, and a completed positive axis-separation proof",
            self.cause
        )
    }
}
impl std::error::Error for ConeRequestError {}

#[test]
pub(super) fn cone_request_refuses_unrepresentable_dyadic_depth() {
    let request = ConeRequest {
        poles: &[[1., 0.], [2., 0.]],
        semi_angle: 0.5,
        depths: [-1., 1.],
        outward: true,
        clearance: 0.,
        max_depth: 65,
        options: ConeClearanceOptions::default(),
    };
    let error = certify_cone_request(request, || false).unwrap_err();
    assert!(matches!(
        error.cause(),
        ConicalDepthFailure::InvalidDepthLimit { requested: 65 }
    ));
    assert_eq!(error.operations(), 0);
    let diagnostic = error.to_string();
    assert!(diagnostic.contains("65"));
    assert!(diagnostic.contains("subdivision <= 64"));
}

#[cfg(test)]
fn failure_request(
    poles: &[[f64; 2]],
    depths: [f64; 2],
    outward: bool,
    clearance: f64,
    options: ConeClearanceOptions,
) -> ConeRequest<'_> {
    ConeRequest {
        poles,
        semi_angle: 0.5,
        depths,
        outward,
        clearance,
        max_depth: 5,
        options,
    }
}

#[cfg(test)]
fn assert_retained_physical_inputs(error: &ConeRequestError) {
    assert_eq!(error.source_bits[0], [1_f64.to_bits(), (-0_f64).to_bits()]);
    assert_eq!(error.semi_angle_bits, 0.5_f64.to_bits());
    assert_eq!(error.depths_bits, [1_f64.to_bits(), (-1_f64).to_bits()]);
    assert!(!error.outward);
    assert_eq!(error.clearance_bits, 0.25_f64.to_bits());
    assert_eq!(error.max_depth, 5);
    assert_eq!(
        (error.fractional_bits, error.max_operations),
        (128, 100_000)
    );
    assert_eq!(error.operations, 0);
    assert!(matches!(
        *error.cause,
        ConicalDepthFailure::InvalidDepths([1., -1.])
    ));
}
