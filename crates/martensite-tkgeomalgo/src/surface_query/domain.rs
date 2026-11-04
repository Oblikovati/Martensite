//! Domain inclusion and declared aliases are exact comparisons, not modelling tolerances.
use super::types::{FaceQueryIncomplete as Cause, SurfaceSelfQuery, require};
use martensite_tkg3d::geom::StoredSplineSurfaceInput;
pub(super) fn validate(
    input: &StoredSplineSurfaceInput,
    request: SurfaceSelfQuery,
    owner: usize,
) -> Result<(), Cause> {
    let periodic = [input.axes()[0].periodic(), input.axes()[1].periodic()];
    require(
        request.periodic == periodic,
        owner,
        "actual support/adaptor periodicity",
        (periodic, request.periodic),
    )?;
    for (axis, bounds) in request.bounds.iter().enumerate() {
        finite_interval(*bounds, owner)?;
        axis_bounds(input.axes()[axis].domain(), *bounds, periodic[axis], owner)?;
    }
    tolerances(request.tolerances, owner)
}
fn tolerances(values: [f64; 2], owner: usize) -> Result<(), Cause> {
    require(
        values.iter().all(|v| v.is_finite() && *v >= 0.),
        owner,
        "finite nonnegative unchanged query tolerances",
        values,
    )
}
fn axis_bounds(
    canonical: [f64; 2],
    bounds: [f64; 2],
    periodic: bool,
    owner: usize,
) -> Result<(), Cause> {
    if periodic {
        return periodic_bounds(canonical, bounds, owner);
    }
    require(
        bounds[0] >= canonical[0] && bounds[1] <= canonical[1],
        owner,
        "query inside canonical nonperiodic support",
        (canonical, bounds),
    )
}
fn finite_interval(bounds: [f64; 2], owner: usize) -> Result<(), Cause> {
    require(
        bounds[0].is_finite() && bounds[1].is_finite() && bounds[0] < bounds[1],
        owner,
        "finite strictly increasing query interval",
        bounds,
    )
}
fn periodic_bounds(canonical: [f64; 2], bounds: [f64; 2], owner: usize) -> Result<(), Cause> {
    let period = canonical[1];
    let double = 2. * period;
    // Zero origin and one adjacent positive cycle make normalization subtraction exact by Sterbenz (#213).
    let supported = canonical[0] == 0.
        && double.is_finite()
        && period > martensite_tkernel::standard::real::epsilon(bounds[1]).abs()
        && bounds[0] >= 0.
        && bounds[1] <= double;
    require(
        supported,
        owner,
        "zero-origin canonical or one adjacent positive U cycle",
        (canonical, bounds),
    )
}
