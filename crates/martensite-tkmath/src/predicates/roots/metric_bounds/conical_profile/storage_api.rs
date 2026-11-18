// SPDX-License-Identifier: GPL-2.0-only
//! Public request, evidence, refusal and input binding for the storage proof.
use super::storage::*;
use super::*;
/// Inputs for bounding one ideal cone edge against supplied stored Bézier coefficients.
#[derive(Clone, Copy)]
pub struct ConicalStorageQuery<'a> {
    /// Bernstein half-angle tangent coefficients.
    pub tangent: &'a [f64],
    /// Homogeneous slant numerator coefficients.
    pub numerator: &'a [f64],
    /// Positive homogeneous slant weight coefficients.
    pub weights: &'a [f64],
    /// Finite cone semi-angle in radians, strictly between zero and pi/2.
    pub semi_angle: f64,
    /// Physical angular center in radians.
    pub center: f64,
    /// Signed normal depth.
    pub depth: f64,
    /// Whether positive depth follows the outward cone normal.
    pub outward: bool,
    /// Actual retained local-axis Cartesian Bézier poles.
    pub stored_poles: &'a [[f64; 3]],
    /// Actual retained positive Bézier weights.
    pub stored_weights: &'a [f64],
}
/// Typed refusal or incomplete bounded calculation.
#[derive(Debug)]
pub enum ConicalStorageFailure {
    /// Arithmetic policy exceeds the supported limits.
    Policy {
        /// Requested fractional precision.
        fractional_bits: u16,
        /// Requested work allowance.
        max_operations: u32,
    },
    /// Coefficient counts do not have the supported shape.
    Shape {
        /// Angular tangent count.
        tangent: usize,
        /// Slant numerator count.
        numerator: usize,
        /// Slant weight count.
        weights: usize,
        /// Stored Cartesian pole count.
        poles: usize,
        /// Stored weight count.
        stored_weights: usize,
    },
    /// An input scalar was not finite.
    Nonfinite {
        /// Input role.
        role: &'static str,
        /// Index in the input.
        index: usize,
        /// Coordinate axis, or zero for scalars.
        axis: usize,
        /// Original binary64 bits.
        bits: u64,
    },
    /// Unsupported cone semi-angle.
    Support {
        /// Requested semi-angle.
        semi_angle: f64,
    },
    /// A required weight lower bound was not strictly positive.
    Weight {
        /// Weight role.
        role: &'static str,
        /// Outward lower bound.
        lower: DyadicBound,
    },
    /// Trigonometric or polynomial certificate failed.
    Polynomial(CylindricalProfileFailure),
    /// Bounded arithmetic failed or was cancelled.
    Arithmetic(BoundedArithmeticFailure),
}
/// Completed coefficient comparison and its immutable input binding.
#[derive(Debug)]
pub struct ConicalStorageEvidence {
    pub(super) upper: DyadicBound,
    pub(super) operations: u32,
    pub(super) options: ConeClearanceOptions,
    pub(super) input: StorageSnapshot,
}
/// A failed comparison and the amount of charged work.
#[derive(Debug)]
pub struct ConicalStorageError {
    pub(super) cause: ConicalStorageFailure,
    pub(super) operations: u32,
    pub(super) options: ConeClearanceOptions,
    pub(super) input: Option<Box<StorageSnapshot>>,
}
#[derive(Debug)]
pub(super) struct StorageSnapshot {
    pub(super) tangent: Vec<u64>,
    pub(super) numerator: Vec<u64>,
    pub(super) weights: Vec<u64>,
    pub(super) support: [u64; 3],
    pub(super) outward: bool,
    pub(super) poles: Vec<[u64; 3]>,
    pub(super) stored_weights: Vec<u64>,
}
/// Certify a local-axis ideal-to-stored edge discrepancy on the common Bézier parameter `[0,1]`.
/// The caller must establish native parameter range and location correspondence; topology is not certified.
/// Example: `let evidence = certify_conical_storage(query, options, || false)?;`.
pub fn certify_conical_storage(
    query: ConicalStorageQuery<'_>,
    options: ConeClearanceOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalStorageEvidence, ConicalStorageError> {
    let input = admit_snapshot(&query, options)?;
    execute_storage(query, options, input, &mut cancelled)
}
pub(super) fn admit_snapshot(
    query: &ConicalStorageQuery<'_>,
    options: ConeClearanceOptions,
) -> Result<StorageSnapshot, ConicalStorageError> {
    let admission = admit_counts(query);
    admit_policy(query, options, admission.is_ok())?;
    admission.map_err(|cause| ConicalStorageError {
        cause,
        operations: 0,
        options,
        input: None,
    })?;
    Ok(StorageSnapshot::capture(query))
}
pub(super) fn admit_policy(
    query: &ConicalStorageQuery<'_>,
    options: ConeClearanceOptions,
    valid_shape: bool,
) -> Result<(), ConicalStorageError> {
    if !(8..=4096).contains(&options.fractional_bits) || options.max_operations > 1_000_000 {
        let input = valid_shape.then(|| Box::new(StorageSnapshot::capture(query)));
        let cause = ConicalStorageFailure::Policy {
            fractional_bits: options.fractional_bits,
            max_operations: options.max_operations,
        };
        return Err(ConicalStorageError {
            cause,
            operations: 0,
            options,
            input,
        });
    }
    Ok(())
}
pub(super) fn execute_storage(
    query: ConicalStorageQuery<'_>,
    options: ConeClearanceOptions,
    input: StorageSnapshot,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Result<ConicalStorageEvidence, ConicalStorageError> {
    let mut arithmetic = Arithmetic::new(storage_policy(options), cancelled);
    let bound = match storage_discrepancy(&query, &mut arithmetic) {
        Ok(bound) => bound,
        Err(cause) => {
            return Err(storage_error(
                cause,
                arithmetic.operations(),
                options,
                input,
            ));
        }
    };
    Ok(storage_evidence(
        bound,
        arithmetic.operations(),
        options,
        input,
    ))
}
pub(super) fn storage_policy(options: ConeClearanceOptions) -> ArithmeticPolicy {
    ArithmeticPolicy {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
    }
}
pub(super) fn storage_error(
    cause: ConicalStorageFailure,
    operations: u32,
    options: ConeClearanceOptions,
    input: StorageSnapshot,
) -> ConicalStorageError {
    ConicalStorageError {
        cause,
        operations,
        options,
        input: Some(Box::new(input)),
    }
}
pub(super) fn storage_evidence(
    bound: Interval,
    operations: u32,
    options: ConeClearanceOptions,
    input: StorageSnapshot,
) -> ConicalStorageEvidence {
    ConicalStorageEvidence {
        upper: DyadicBound::new(bound.upper, -i32::from(options.fractional_bits)),
        operations,
        options,
        input,
    }
}

impl ConicalStorageEvidence {
    /// Certified outward upper bound; `evidence.upper()`.
    pub fn upper(&self) -> &DyadicBound {
        &self.upper
    }
    /// Charged arithmetic work; `evidence.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Original arithmetic policy; `evidence.options()`.
    pub fn options(&self) -> ConeClearanceOptions {
        self.options
    }
    /// Check every input bit against the captured request; `evidence.matches(&query)`.
    pub fn matches(&self, query: &ConicalStorageQuery<'_>) -> bool {
        self.input.matches(query)
    }
}
impl ConicalStorageError {
    /// Original typed cause; `error.cause()`.
    pub fn cause(&self) -> &ConicalStorageFailure {
        &self.cause
    }
    /// Charged work before refusal; `error.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Requested arithmetic policy; `error.options()`.
    pub fn options(&self) -> ConeClearanceOptions {
        self.options
    }
    /// Compare retained input bits, or return `None` when the input shape was too large or invalid.
    /// Example: `let same = error.matches(&query);`.
    pub fn matches(&self, query: &ConicalStorageQuery<'_>) -> Option<bool> {
        self.input.as_ref().map(|input| input.matches(query))
    }
}
impl std::fmt::Display for ConicalStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} after {} operations", self.cause, self.operations)
    }
}
impl std::error::Error for ConicalStorageError {}
impl StorageSnapshot {
    pub(super) fn capture(query: &ConicalStorageQuery<'_>) -> Self {
        Self {
            tangent: query.tangent.iter().map(|v| v.to_bits()).collect(),
            numerator: query.numerator.iter().map(|v| v.to_bits()).collect(),
            weights: query.weights.iter().map(|v| v.to_bits()).collect(),
            support: [
                query.semi_angle.to_bits(),
                query.center.to_bits(),
                query.depth.to_bits(),
            ],
            outward: query.outward,
            poles: query
                .stored_poles
                .iter()
                .map(|p| p.map(f64::to_bits))
                .collect(),
            stored_weights: query.stored_weights.iter().map(|v| v.to_bits()).collect(),
        }
    }
    pub(super) fn matches(&self, query: &ConicalStorageQuery<'_>) -> bool {
        self.matches_fit(query) && self.matches_support(query) && self.matches_stored(query)
    }
    pub(super) fn matches_fit(&self, query: &ConicalStorageQuery<'_>) -> bool {
        self.tangent
            .iter()
            .copied()
            .eq(query.tangent.iter().map(|v| v.to_bits()))
            && self
                .numerator
                .iter()
                .copied()
                .eq(query.numerator.iter().map(|v| v.to_bits()))
            && self
                .weights
                .iter()
                .copied()
                .eq(query.weights.iter().map(|v| v.to_bits()))
    }
    pub(super) fn matches_support(&self, query: &ConicalStorageQuery<'_>) -> bool {
        self.support
            == [
                query.semi_angle.to_bits(),
                query.center.to_bits(),
                query.depth.to_bits(),
            ]
            && self.outward == query.outward
    }
    pub(super) fn matches_stored(&self, query: &ConicalStorageQuery<'_>) -> bool {
        self.poles
            .iter()
            .copied()
            .eq(query.stored_poles.iter().map(|p| p.map(f64::to_bits)))
            && self
                .stored_weights
                .iter()
                .copied()
                .eq(query.stored_weights.iter().map(|v| v.to_bits()))
    }
}
