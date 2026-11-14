//! A refusing family error-budget gate; it does not authorize publication by itself.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::CylindricalMeasureLimit;
use std::collections::{BTreeMap, BTreeSet};
mod error;
mod member;
mod native;
#[cfg(test)]
mod tests;
mod volume;
pub(super) use error::{BudgetFailure, Quantity};
type Checked<T> = Result<T, BudgetFailure>;
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct BudgetLimits {
    spatial: f64,
    profile: f64,
    storage: f64,
    trace: f64,
    clearance: f64,
    retained_vs_ideal: CylindricalMeasureLimit,
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct MemberBounds {
    pub profile: f64,
    pub storage: f64,
    pub trace: f64,
    pub trace_tolerance: f64,
    pub native_modes: [martensite_tkbo::body_check::CheckMode; 4],
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct FamilyVolumeBounds {
    pub discrepancy: f64,
    pub representative_error: f64,
    pub integration_error: f64,
    pub profile_error: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct BudgetEvidence {
    pub limits: BudgetLimits,
    pub members: BTreeMap<SurfaceRegionId, MemberBounds>,
    pub volume: FamilyVolumeBounds,
}
pub(super) fn certify(
    originals: &[Request],
    family: &family::UnqualifiedFamily,
    retained_vs_ideal: CylindricalMeasureLimit,
) -> Checked<BudgetEvidence> {
    let limits = requested_limits(originals, retained_vs_ideal)?;
    certify_with_limits(originals, family, limits)
}
pub(super) fn requested_limits(
    originals: &[Request],
    retained_vs_ideal: CylindricalMeasureLimit,
) -> Checked<BudgetLimits> {
    let first = originals.first().ok_or(BudgetFailure::Empty)?;
    BudgetLimits::new(first.tolerance, retained_vs_ideal)
}
pub(super) fn certify_with_limits(
    originals: &[Request],
    family: &family::UnqualifiedFamily,
    limits: BudgetLimits,
) -> Checked<BudgetEvidence> {
    owners(originals, family)?;
    let members = member_budgets(originals, family, limits)?;
    member::clearance(family, limits.clearance)?;
    let volume = FamilyVolumeBounds::certify(family, limits.retained_vs_ideal)?;
    Ok(BudgetEvidence {
        limits,
        members,
        volume,
    })
}
impl BudgetLimits {
    fn new(spatial: f64, retained_vs_ideal: CylindricalMeasureLimit) -> Checked<Self> {
        admit_spatial(spatial)?;
        volume::admit(retained_vs_ideal)?;
        Ok(Self {
            spatial,
            profile: (spatial / 4.).next_down().max(0.),
            storage: storage_share(spatial),
            trace: spatial / 2.,
            clearance: 4. * spatial,
            retained_vs_ideal,
        })
    }
}
fn owners(originals: &[Request], family: &family::UnqualifiedFamily) -> Checked<()> {
    let expected: BTreeSet<_> = originals.iter().map(|r| r.region).collect();
    let actual: BTreeSet<_> = family.members.keys().copied().collect();
    if expected.len() != originals.len() || actual != expected {
        return Err(BudgetFailure::Owners {
            actual: actual.into_iter().collect(),
            expected: expected.into_iter().collect(),
        });
    }
    family_counts(family)
}
fn family_counts(family: &family::UnqualifiedFamily) -> Checked<()> {
    let coverage = &family.coverage;
    let keys = family.keys.len();
    inventory_count("family members", coverage.members, family.members.len())?;
    inventory_count("frozen bindings", coverage.bindings, keys)?;
    inventory_count("history records", family.history.records.len(), keys)?;
    Ok(())
}
fn upper(
    actual: f64,
    limit: f64,
    quantity: Quantity,
    region: SurfaceRegionId,
    span: Option<CurveSpan>,
) -> Checked<f64> {
    if !actual.is_finite() || actual < 0. {
        return Err(BudgetFailure::Invalid {
            quantity,
            value: actual,
        });
    }
    if actual > limit {
        return Err(exceeded(quantity, actual, limit, Some(region), span));
    }
    Ok(actual)
}

fn inventory_count(field: &'static str, actual: usize, expected: usize) -> Checked<()> {
    if actual != expected {
        return Err(BudgetFailure::Inventory {
            field,
            actual,
            expected,
        });
    }
    Ok(())
}

fn member_budgets(
    originals: &[Request],
    family: &family::UnqualifiedFamily,
    limits: BudgetLimits,
) -> Checked<BTreeMap<SurfaceRegionId, MemberBounds>> {
    originals
        .iter()
        .map(|r| Ok((r.region, member::certify(r, family, limits)?)))
        .collect()
}
fn admit_spatial(value: f64) -> Checked<()> {
    if !value.is_finite() || value <= 0. || !(4. * value).is_finite() {
        return Err(policy(
            "positive finite spatial budget and clearance",
            value,
        ));
    }
    Ok(())
}
fn storage_share(tolerance: f64) -> f64 {
    let nearest = tolerance / 1000.;
    if nearest == 0. {
        return 0.;
    }
    nearest.next_down()
}
fn policy(field: &'static str, value: f64) -> BudgetFailure {
    BudgetFailure::Policy { field, value }
}
fn exceeded(
    quantity: Quantity,
    actual: f64,
    limit: f64,
    region: Option<SurfaceRegionId>,
    span: Option<CurveSpan>,
) -> BudgetFailure {
    BudgetFailure::Limit {
        quantity,
        actual,
        limit,
        region,
        span,
    }
}
