//! Existing exact residual decisions cover entire represented intervals.
mod claims;
mod edges;
mod endpoints;
mod queries;
use super::{
    boundary::{Boundary, Side},
    budget::*,
    decode::*,
    types::*,
};
use claims::append;
use martensite_tkmath::predicates::roots::{RootIsolationOptions, metric_bounds::*};
use queries::{residual, vertex_query};
pub(super) fn certify<F: FnMut() -> bool>(
    strip: &Strip,
    boundary: &Boundary,
    budget: &mut Budget<'_, F>,
    errors: &mut Vec<ClosedFaceErrorBound>,
) -> CheckResult<()> {
    for (index, edge) in strip.edges.iter().enumerate() {
        edges::EdgeMetrics::new(strip, edge, index, boundary).certify(budget, errors)?;
    }
    Ok(())
}
pub(super) fn minimum(values: &[f64], owner: usize) -> CheckResult<f64> {
    if values.iter().any(|v| !v.is_finite() || *v < 0.) {
        return Err(invalid(
            owner,
            "finite nonnegative raw spatial limits",
            format!("{values:?}"),
        ));
    }
    Ok(values.iter().copied().fold(f64::INFINITY, f64::min))
}
