//! Checked coefficient retention without the legacy epsilon demotion (#464).
use super::{
    GeomBezierSurface, SurfaceState, check_pole_counts, check_weight_shape, check_weights, errors,
};
use martensite_tkernel::{ncollection::Array2, standard::Failure};
use martensite_tkmath::gp::Point3;

/// Refusal to retain a checked Bezier control net without changing its coefficients.
#[derive(Debug)]
pub enum BezierSurfaceCoefficientError {
    /// Existing shape, bounds or finite-value validation failed.
    Native(Failure),
    /// The binary representation cannot retain constant non-unit weights truthfully.
    ConstantNonUnitWeights {
        /// Common positive weight that would otherwise be normalized away.
        weight: f64,
    },
}
impl From<Failure> for BezierSurfaceCoefficientError {
    fn from(cause: Failure) -> Self {
        // Rebuilding this from Display would erase its native exception category.
        Self::Native(cause)
    }
}
impl std::fmt::Display for BezierSurfaceCoefficientError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(cause) => write!(out, "{cause}"),
            Self::ConstantNonUnitWeights { weight } => write!(
                out,
                "Bezier weight {weight} (bits {:#018x}); expected nonconstant weights or unit weights",
                weight.to_bits()
            ),
        }
    }
}
impl std::error::Error for BezierSurfaceCoefficientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Native(cause) => Some(cause),
            Self::ConstantNonUnitWeights { .. } => None,
        }
    }
}
impl BezierSurfaceCoefficientError {
    /// Native decoder category for a constant non-unit refusal (#464).
    pub const CONSTANT_NON_UNIT_EXCEPTION: &'static str = "Geom_ConstantNonUnitBezierWeights";
    /// Preserves the native cause or a distinct refusal category; `error.into_failure()`.
    pub fn into_failure(self) -> Failure {
        match self {
            Self::Native(cause) => cause,
            error @ Self::ConstantNonUnitWeights { .. } => {
                Failure::with_message(Self::CONSTANT_NON_UNIT_EXCEPTION, &error.to_string())
                    .with_exception_parent("Standard_ConstructionError")
            }
        }
    }
}

impl GeomBezierSurface {
    /// Retains finite poles and positive weights exactly, refusing constant non-unit weights.
    /// `GeomBezierSurface::from_stored_coefficients(&poles, &weights)?`.
    pub fn from_stored_coefficients(
        poles: &Array2<'_, Point3>,
        weights: &Array2<'_, f64>,
    ) -> Result<Self, BezierSurfaceCoefficientError> {
        check_pole_counts(poles.nb_rows(), poles.nb_columns())?;
        check_weight_shape(weights, poles.nb_rows(), poles.nb_columns())?;
        let scan = checked_uniform_weights(weights)?;
        finite_poles(poles)?;
        check_finite_weight(scan.nonfinite)?;
        let flags = if scan.uniform {
            (false, false)
        } else {
            exact_weight_variation(weights)
        };
        let first = scan.first;
        if flags == (false, false) && first != 1. {
            return Err(BezierSurfaceCoefficientError::ConstantNonUnitWeights { weight: first });
        }
        Ok(Self::stored(SurfaceState::retained(poles, weights, flags)?))
    }
}
pub(super) fn finite_poles(poles: &Array2<'_, Point3>) -> Result<(), Failure> {
    for point in poles.as_slice() {
        let xyz = point.coordinates();
        if !xyz[0].is_finite() || !xyz[1].is_finite() || !xyz[2].is_finite() {
            return Err(errors::construction(&format!(
                "Bezier pole {xyz:?}; expected finite coordinates"
            )));
        }
    }
    Ok(())
}
pub(super) fn exact_weight_variation(weights: &Array2<'_, f64>) -> (bool, bool) {
    let values = weights.as_slice();
    let columns = weights.nb_columns();
    let (mut u, mut v) = (false, false);
    for (index, &weight) in values.iter().enumerate() {
        u = u || index + columns < values.len() && weight != values[index + columns];
        v = v || index % columns + 1 < columns && weight != values[index + 1];
        if u && v {
            break;
        }
    }
    (u, v)
}
struct RetainedWeightScan {
    first: f64,
    uniform: bool,
    nonfinite: Option<f64>,
}
impl RetainedWeightScan {
    #[inline(always)]
    fn record_weight(&mut self, weight: f64) {
        if self.nonfinite.is_none() && !weight.is_finite() {
            self.nonfinite = Some(weight);
        }
        self.uniform &= weight == self.first;
    }
}
fn checked_uniform_weights(weights: &Array2<'_, f64>) -> Result<RetainedWeightScan, Failure> {
    let values = weights.as_slice();
    let mut scan = RetainedWeightScan {
        first: values[0],
        uniform: true,
        nonfinite: None,
    };
    for &weight in values {
        if weight <= martensite_tkmath::gp::resolution() {
            // Only malformed grids need the retained column-major failure search.
            check_weights(weights, weights.nb_rows(), weights.nb_columns())?;
        }
        scan.record_weight(weight);
    }
    Ok(scan)
}
pub(super) fn check_finite_weight(nonfinite: Option<f64>) -> Result<(), Failure> {
    if let Some(weight) = nonfinite {
        return Err(errors::construction(&format!(
            "Bezier weight {weight}; expected finite positive weight"
        )));
    }
    Ok(())
}
