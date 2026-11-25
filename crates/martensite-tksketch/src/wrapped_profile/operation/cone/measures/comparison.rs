//! Reference-body uncertainty and conditional patch-flux comparisons have separate meanings.
use super::*;
#[derive(Clone, Copy, Debug)]
pub(in crate::surface_sketch::wrapped_profile::operation::cone) struct ComparisonBounds {
    pub authored_reference_discrepancy: f64,
    pub reference_integration_error: f64,
    pub proxy_reference_discrepancy: f64,
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn certify(
    geometry: &Geometry,
) -> ConeChecked<ComparisonBounds> {
    let authored = range(geometry.authored.volume())?;
    let reference = &geometry.reference_volume;
    let reference_integration_error = reference.absolute_error_bound()?;
    let authored_reference_discrepancy =
        distance([authored.lower, authored.upper], reference.enclosure())?;
    let proxy_reference_discrepancy = distance(geometry.flux.enclosure(), reference.enclosure())?;
    Ok(ComparisonBounds {
        authored_reference_discrepancy,
        reference_integration_error,
        proxy_reference_discrepancy,
    })
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn member_volume(
    geometry: &Geometry,
) -> ConeChecked<WrappedConeVolumeBounds> {
    let actual = certify(geometry)?;
    Ok(WrappedConeVolumeBounds {
        discrepancy: api::reports::bound(upper(geometry.authored.difference())?),
        integration_error: api::reports::bound(geometry.flux.absolute_error_bound()),
        authored_reference_discrepancy: api::reports::bound(actual.authored_reference_discrepancy),
        proxy_reference_discrepancy: api::reports::bound(actual.proxy_reference_discrepancy),
        reference_integration_error: api::reports::bound(actual.reference_integration_error),
    })
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn distance(
    a: [f64; 2],
    b: [f64; 2],
) -> ConeChecked<f64> {
    let first = difference(a[0], b[1])?;
    let last = difference(a[1], b[0])?;
    Ok(first.max(last))
}
fn difference(a: f64, b: f64) -> ConeChecked<f64> {
    let difference = a - b;
    if !difference.is_finite() {
        return Err(scalar(
            "volume enclosure difference",
            difference,
            "finite outward difference",
        ));
    }
    if difference == 0. {
        return Ok(0.);
    }
    let upper = difference.abs().next_up();
    if !upper.is_finite() {
        return Err(scalar(
            "volume difference bound",
            upper,
            "finite outward upper bound",
        ));
    }
    Ok(upper)
}
pub(super) fn add(accumulated: f64, contribution: f64) -> ConeChecked<f64> {
    valid(accumulated)?;
    valid(contribution)?;
    if contribution == 0. {
        return Ok(accumulated);
    }
    if accumulated == 0. {
        return Ok(contribution);
    }
    let upper = (accumulated + contribution).next_up();
    valid(upper)?;
    Ok(upper)
}
fn valid(value: f64) -> ConeChecked<()> {
    if !value.is_finite() || value < 0. {
        return Err(scalar(
            "volume contribution",
            value,
            "finite nonnegative certified upper bound",
        ));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interval_distance_uses_both_extreme_pairs() {
        assert!(distance([2., 3.], [5., 7.]).unwrap() > 5.);
        assert!(distance([5., 7.], [2., 3.]).unwrap() > 5.);
        assert_eq!(distance([2.; 2], [2.; 2]).unwrap(), 0.);
    }
    #[test]
    fn reference_interval_uncertainty_is_included_once() {
        let error = distance([2., 3.], [3.75, 4.25]).unwrap();
        assert!(error > 2.25);
        assert!(error < 2.26);
        assert!(distance([f64::MAX; 2], [-f64::MAX; 2]).is_err());
    }
}
