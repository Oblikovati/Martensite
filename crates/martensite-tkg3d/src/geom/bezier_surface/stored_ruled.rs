//! Own curve state before checking and assembling its ruled surface (#464).
use super::ruled_rows::{RuledControlGrid, ruled_control_grid};
use super::stored_coefficients::{check_finite_weight, exact_weight_variation, finite_poles};
use super::{BezierSurfaceCoefficientError, GeomBezierSurface, SurfaceState};
use crate::geom::{GeomBezierCurve, errors, spline_kind::BezierState};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkmath::gp::Point3;

/// Consumed coefficient input; numerical admission occurs inside the patch factory.
/// Native curve construction retains its existing weight canonicalization rules.
pub struct StoredBezierBoundary {
    /// Pole buffer, captured into independent native ownership before proof reuse.
    pub poles: Array1<'static, Point3>,
    /// Original weight buffer, including values requiring native admission.
    pub weights: Array1<'static, f64>,
}
/// Operation-local native geometry; validate and freeze it before immutable publication.
pub struct StoredRuledGeometry {
    /// First and last depth boundaries in the supplied authored order.
    pub boundaries: [Handle<GeomBezierCurve>; 2],
    /// Independently owned surface whose boundary rows match those curves at construction.
    pub surface: Handle<GeomBezierSurface>,
}
impl GeomBezierSurface {
    /// Admits coefficient rows once before exposing mutable native handles (#464).
    /// `GeomBezierSurface::ruled_with_stored_boundaries([first, last])?`.
    pub fn ruled_with_stored_boundaries(
        rows: [StoredBezierBoundary; 2],
    ) -> Result<StoredRuledGeometry, BezierSurfaceCoefficientError> {
        let first = BezierState::checked(&rows[0].poles, Some(&rows[0].weights))?;
        let last = BezierState::checked(&rows[1].poles, Some(&rows[1].weights))?;
        let states = [first, last];
        let (poles, weights) = checked_grids(&rows, &states)?;
        let flags = weights
            .as_ref()
            .map_or((false, false), exact_weight_variation);
        let surface = Self::stored(SurfaceState::retained_owned(poles, weights, flags)?);
        Ok(StoredRuledGeometry {
            boundaries: states.map(|state| Handle::new(GeomBezierCurve::from_state(state))),
            surface: Handle::new(surface),
        })
    }
}
fn checked_grids(
    rows: &[StoredBezierBoundary; 2],
    states: &[BezierState; 2],
) -> Result<RuledControlGrid, BezierSurfaceCoefficientError> {
    check_degree(states)?;
    let weights = (states[0].rational || states[1].rational)
        .then_some([&states[0].weights, &states[1].weights]);
    let grids = ruled_control_grid(
        [&states[0].poles, &states[1].poles],
        weights,
        states[0].poles.len() as i32,
    )?;
    finite_poles(&grids.0)?;
    check_finite_weight(first_nonfinite_weight(rows))?;
    Ok(grids)
}
fn check_degree(states: &[BezierState; 2]) -> Result<(), martensite_tkernel::standard::Failure> {
    if states[0].degree() != states[1].degree() {
        return Err(errors::construction(&format!(
            "Bezier boundary degrees {} and {}; expected equal degrees",
            states[0].degree(),
            states[1].degree()
        )));
    }
    Ok(())
}
fn first_nonfinite_weight(rows: &[StoredBezierBoundary; 2]) -> Option<f64> {
    for (&first, &last) in rows[0]
        .weights
        .as_slice()
        .iter()
        .zip(rows[1].weights.as_slice())
    {
        if !first.is_finite() {
            return Some(first);
        }
        if !last.is_finite() {
            return Some(last);
        }
    }
    None
}
