//! Owned admission metadata is not a certificate for a later-mutated entity.
use martensite_tkmath::predicates::roots::closed_extrusion::{
    ClosedExtrusionInputError, ClosedExtrusionReport,
};

#[derive(Clone, Debug, PartialEq)]
/// Snapshot metadata read while the extrusion and basis guards overlap.
pub struct ClosedSplineExtrusionLayout {
    /// Stored polynomial degree.
    pub degree: i32,
    /// Stored periodicity flag.
    pub periodic: bool,
    /// Whether the actual evaluator uses stored weights.
    pub rational: bool,
    /// Whether a delegated curve evaluator replaces the stored representation.
    pub basis_override: bool,
    /// Lower bounds of poles, weights, knots, multiplicities and flat knots.
    pub lower_bounds: [i32; 5],
    /// Actual stored flat knot values, not a later reconstruction.
    pub flat_knots: Vec<f64>,
    /// Actual stored multiplicities.
    pub multiplicities: Vec<i32>,
    /// Copied extrusion direction, without normalization or transformation.
    pub direction: [f64; 3],
    /// Requested finite extrusion parameter interval, not inferred face trims.
    pub extrusion_interval: [f64; 2],
}

#[derive(Clone, Debug, PartialEq)]
/// A represented map for which no support proof was established.
pub enum ClosedSplineExtrusionInconclusive {
    /// The surface evaluator is external to the stored extrusion formula.
    SurfaceEvaluationOverride {
        /// Copied stored direction; not a description of the delegated evaluator.
        direction: [f64; 3],
        /// Unchanged requested interval.
        interval: [f64; 2],
    },
    /// Only an actual stored spline basis is supported.
    UnsupportedBasis {
        /// Observed dynamic type information.
        actual: String,
        /// Copied stored direction.
        direction: [f64; 3],
        /// Unchanged requested interval.
        interval: [f64; 2],
    },
    /// A delegated curve evaluator does not follow from the stored poles.
    BasisEvaluationOverride(ClosedSplineExtrusionLayout),
    /// The represented endpoints are not the exact declared closing seam.
    NotExactlyClosed {
        /// Coherent admission metadata.
        layout: ClosedSplineExtrusionLayout,
        /// Actual first represented endpoint.
        first: [f64; 3],
        /// Actual last represented endpoint.
        last: [f64; 3],
    },
    /// Degree, periodicity or array indexing is outside the admitted class.
    UnsupportedLayout(ClosedSplineExtrusionLayout),
}

#[derive(Clone, Debug)]
/// An owned numeric report and the coherent geometry admission that produced it.
pub struct ClosedSplineExtrusionSnapshot {
    pub(in crate::geom) numeric: Box<ClosedExtrusionReport>,
    pub(in crate::geom) layout: ClosedSplineExtrusionLayout,
}
impl ClosedSplineExtrusionSnapshot {
    /// Read the support-only numerical outcome; `snapshot.numeric_report().certificate()`.
    pub fn numeric_report(&self) -> &ClosedExtrusionReport {
        &self.numeric
    }
    /// Read immutable admission metadata; `snapshot.layout().basis_override` is false.
    pub fn layout(&self) -> &ClosedSplineExtrusionLayout {
        &self.layout
    }
}

#[derive(Clone, Debug)]
/// Captured means numeric evaluation ran, not that face validity was established.
pub enum ClosedSplineExtrusionExclusion {
    /// Actual owned inputs and a mathematical outcome, which can itself be incomplete.
    Captured(ClosedSplineExtrusionSnapshot),
    /// No supported represented map was captured.
    Inconclusive(ClosedSplineExtrusionInconclusive),
}

#[derive(Clone, Debug, PartialEq)]
/// Invalid numeric or inconsistent evaluator storage, distinct from unproved absence.
pub enum ClosedSplineExtrusionError {
    /// Exact input admission failed with its offending values.
    Input(ClosedExtrusionInputError),
    /// Stored multiplicity cannot be represented as a positive count.
    Multiplicity {
        /// Stored element index.
        index: usize,
        /// Offending signed value.
        value: i32,
    },
    /// Stored flat knots disagree with the same snapshot's distinct knots and counts.
    FlatSequence {
        /// Actual stored flat sequence.
        actual: Vec<f64>,
        /// Required sequence derived exactly from the captured knots and counts.
        expected: Vec<f64>,
    },
}
impl std::fmt::Display for ClosedSplineExtrusionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "closed spline extrusion capture={self:?}; expected coherent finite stored spline values and matching flat knots"
        )
    }
}
impl std::error::Error for ClosedSplineExtrusionError {}
