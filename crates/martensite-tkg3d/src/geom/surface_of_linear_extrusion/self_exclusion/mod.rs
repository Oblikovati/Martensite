//! The exact calculation starts only after a surface-to-basis coherent owned capture.
pub(in crate::geom) mod types;
use super::GeomSurfaceOfLinearExtrusion;
use crate::geom::GeomBSplineCurve;
use martensite_tkmath::predicates::roots::closed_extrusion::{
    ClosedExtrusionPolicy, ClosedQuadraticExtrusionInput, certify_closed_quadratic_extrusion,
};
pub use types::{
    ClosedSplineExtrusionError, ClosedSplineExtrusionExclusion, ClosedSplineExtrusionInconclusive,
    ClosedSplineExtrusionLayout, ClosedSplineExtrusionSnapshot,
};

/// Owned spline admission copied under a retained basis lease; it contains no live geometry.
///
/// Match `ClosedSplineExtrusionInput::Captured(input, layout)` before exact execution.
#[derive(Clone, Debug)]
pub enum ClosedSplineExtrusionInput {
    Captured(ClosedQuadraticExtrusionInput, ClosedSplineExtrusionLayout),
    Inconclusive(ClosedSplineExtrusionInconclusive),
}
pub(in crate::geom) type BasisAdmission = ClosedSplineExtrusionInput;

impl GeomSurfaceOfLinearExtrusion {
    /// Analyze an owned support snapshot; `surface.closed_quadratic_self_exclusion([-1.,1.], policy, ||false)?`.
    /// The result does not validate face trims, locations or a later-mutated surface handle.
    pub fn closed_quadratic_self_exclusion(
        &self,
        interval: [f64; 2],
        policy: ClosedExtrusionPolicy,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<ClosedSplineExtrusionExclusion, ClosedSplineExtrusionError> {
        let snapshot = self.closed_basis_capture(interval)?;
        Ok(match snapshot {
            BasisAdmission::Captured(input, layout) => {
                captured_outcome(input, layout, policy, &mut cancelled)
            }
            BasisAdmission::Inconclusive(reason) => {
                ClosedSplineExtrusionExclusion::Inconclusive(reason)
            }
        })
    }
    fn closed_basis_capture(
        &self,
        interval: [f64; 2],
    ) -> Result<BasisAdmission, ClosedSplineExtrusionError> {
        let state = self.state.read();
        capture_stored_basis(&state, interval)
    }
}

fn captured_outcome(
    input: ClosedQuadraticExtrusionInput,
    layout: ClosedSplineExtrusionLayout,
    policy: ClosedExtrusionPolicy,
    cancelled: &mut impl FnMut() -> bool,
) -> ClosedSplineExtrusionExclusion {
    ClosedSplineExtrusionExclusion::Captured(ClosedSplineExtrusionSnapshot {
        numeric: Box::new(certify_closed_quadratic_extrusion(input, policy, cancelled)),
        layout,
    })
}

fn capture_stored_basis(
    state: &super::ExtrusionState,
    interval: [f64; 2],
) -> Result<BasisAdmission, ClosedSplineExtrusionError> {
    let d = state.direction;
    let direction = [d.x(), d.y(), d.z()];
    if !state.eval_rep.is_null() {
        return Ok(surface_override(direction, interval));
    }
    let basis = state.basis.cast::<GeomBSplineCurve>();
    let Some(curve) = basis.get() else {
        return Ok(unsupported_basis(state, direction, interval));
    };
    curve.closed_extrusion_basis_capture(direction, interval)
}

fn surface_override(direction: [f64; 3], interval: [f64; 2]) -> BasisAdmission {
    BasisAdmission::Inconclusive(
        ClosedSplineExtrusionInconclusive::SurfaceEvaluationOverride {
            direction,
            interval,
        },
    )
}

fn unsupported_basis(
    state: &super::ExtrusionState,
    direction: [f64; 3],
    interval: [f64; 2],
) -> BasisAdmission {
    let actual = state
        .basis
        .dynamic_type()
        .map_or_else(|| "null".to_owned(), |kind| kind.system_name().to_owned());
    BasisAdmission::Inconclusive(ClosedSplineExtrusionInconclusive::UnsupportedBasis {
        actual,
        direction,
        interval,
    })
}

#[cfg(test)]
mod tests;
