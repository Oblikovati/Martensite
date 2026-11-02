use super::{BilinearSplineError, BilinearSplineLayout};
use crate::geom::bspline_surface::state::{Direction, SurfaceState};

pub(super) fn layout(state: &SurfaceState) -> BilinearSplineLayout {
    BilinearSplineLayout {
        degrees: [state.u.deg, state.v.deg],
        poles: [state.poles.nb_rows(), state.poles.nb_columns()],
        weights: [state.weights.nb_rows(), state.weights.nb_columns()],
        periodic: [state.u.periodic, state.v.periodic],
        knots: [state.u.knots.len(), state.v.knots.len()],
        multiplicities: [
            state.u.mults.as_slice().to_vec(),
            state.v.mults.as_slice().to_vec(),
        ],
    }
}
pub(super) fn supported(layout: &BilinearSplineLayout) -> bool {
    layout.degrees == [1, 1]
        && layout.poles == [2, 2]
        && layout.weights == [2, 2]
        && layout.periodic == [false, false]
        && layout.knots == [2, 2]
        && layout.multiplicities.iter().all(|m| m.as_slice() == [2, 2])
}
pub(super) fn interval(direction: &Direction) -> Result<[f64; 2], BilinearSplineError> {
    use martensite_tkg2d::geom2d::arrays::at;
    let first = at(&direction.knots, 1).map_err(BilinearSplineError::Storage)?;
    let last = at(&direction.knots, 2).map_err(BilinearSplineError::Storage)?;
    Ok([first, last])
}

pub(super) fn validate_stored_values(state: &SurfaceState) -> Result<(), BilinearSplineError> {
    for (point, pole) in state.poles.as_slice().iter().enumerate() {
        for (axis, value) in [pole.x(), pole.y(), pole.z()].into_iter().enumerate() {
            finite("pole coordinate", point * 3 + axis, value, false)?;
        }
    }
    for (index, &weight) in state.weights.as_slice().iter().enumerate() {
        finite("weight", index, weight, true)?;
    }
    for (component, direction) in [("U knot", &state.u), ("V knot", &state.v)] {
        for (index, &knot) in direction.knots.as_slice().iter().enumerate() {
            finite(component, index, knot, false)?;
        }
    }
    Ok(())
}
fn finite(
    component: &'static str,
    index: usize,
    value: f64,
    positive: bool,
) -> Result<(), BilinearSplineError> {
    if !value.is_finite() || (positive && value <= 0.) {
        return Err(BilinearSplineError::StoredValue {
            component,
            index,
            value,
            positive,
        });
    }
    Ok(())
}
