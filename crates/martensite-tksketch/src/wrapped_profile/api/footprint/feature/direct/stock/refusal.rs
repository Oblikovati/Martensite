//! Stock certificate refusals retain their source and measured values.
use martensite_tkernel::standard::Failure;

/// Reason a source frustum or its decoded boundary failed stock certification.
#[derive(Debug)]
#[non_exhaustive]
pub enum ConicalStockFailure {
    /// Original failure from a native stock query.
    Native(Failure),
    /// A stored coordinate, parameter or tolerance was nonfinite.
    NonFinite(f64),
    /// The source lacked the required solid, shell or three-face structure.
    Body,
    /// The source carrier did not meet the analytic frustum or chart contract.
    Carrier,
    /// The cone's semi-angle was outside the proved open interval `(0, 1)`.
    SemiAngle(f64),
    /// A retained disk did not meet the planar cap contract.
    Disk,
    /// The stock band did not have the required closed wire and rim ranges.
    Band,
    /// A native stock curve did not match its expected line or circle.
    Curve,
    /// A stock pcurve did not match its required trace.
    Pcurve,
    /// A retained stock entity did not have the required native incidences.
    Incidence,
    /// The minimum source tolerance and sew allowance was not finite and positive.
    InvalidLimit(f64),
    /// The outward spatial bound must be below the native limit and at most the collar guard.
    SpatialLimit {
        /// Certified upper bound, when finite, on the stock residual.
        upper: f64,
        /// Smallest native tolerance and sew allowance.
        limit: f64,
        /// Upper distance the analytic contact proof can admit.
        collar_guard: f64,
    },
    /// The source did not meet the contact proof's analytic preconditions.
    Contact,
    /// A stock cap contact was unresolved within the native tolerance.
    ContactBound {
        /// Disk index, with the base at zero and lid at one.
        cap: usize,
        /// Bounded signed radius or nonnegative seam residual at that disk.
        gap: [f64; 2],
        /// Smallest native tolerance and sew allowance.
        limit: f64,
    },
    /// The decoded stock words no longer match the certificate's source.
    SourceChanged,
}

impl From<Failure> for ConicalStockFailure {
    fn from(value: Failure) -> Self {
        Self::Native(value)
    }
}

impl std::fmt::Display for ConicalStockFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(cause) => write!(out, "native stock query failed: {cause}"),
            _ => write!(out, "stock certificate refused {self:?}"),
        }
    }
}

const _: fn() = || {
    fn shared<T: Send + Sync>() {}
    shared::<ConicalStockFailure>();
};
