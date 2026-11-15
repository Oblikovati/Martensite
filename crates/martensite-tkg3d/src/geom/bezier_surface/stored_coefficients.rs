//! Checked coefficient retention without the legacy epsilon demotion (#464).
use super::{GeomBezierSurface, SurfaceState, at2, check_pole_counts, check_weights, errors};
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
        check_weights(weights, poles.nb_rows(), poles.nb_columns())?;
        finite_coefficients(poles, weights)?;
        let flags = exact_rationality(weights)?;
        let first = at2(weights, weights.lower_row(), weights.lower_col())?;
        if flags == (false, false) && first != 1. {
            return Err(BezierSurfaceCoefficientError::ConstantNonUnitWeights { weight: first });
        }
        Ok(Self::stored(SurfaceState::retained(poles, weights, flags)?))
    }
}
fn finite_coefficients(
    poles: &Array2<'_, Point3>,
    weights: &Array2<'_, f64>,
) -> Result<(), Failure> {
    finite_poles(poles)?;
    for weight in weights.as_slice() {
        if !weight.is_finite() {
            return Err(errors::construction(&format!(
                "Bezier weight {weight}; expected finite positive weight"
            )));
        }
    }
    Ok(())
}
fn finite_poles(poles: &Array2<'_, Point3>) -> Result<(), Failure> {
    for point in poles.as_slice() {
        let xyz = [point.x(), point.y(), point.z()];
        if !xyz.into_iter().all(f64::is_finite) {
            return Err(errors::construction(&format!(
                "Bezier pole {xyz:?}; expected finite coordinates"
            )));
        }
    }
    Ok(())
}
fn exact_rationality(weights: &Array2<'_, f64>) -> Result<(bool, bool), Failure> {
    let (mut u, mut v) = (false, false);
    for row in weights.lower_row()..=weights.upper_row() {
        for col in weights.lower_col()..=weights.upper_col() {
            let weight = at2(weights, row, col)?;
            u |= row < weights.upper_row() && weight != at2(weights, row + 1, col)?;
            v |= col < weights.upper_col() && weight != at2(weights, row, col + 1)?;
        }
    }
    Ok((u, v))
}
