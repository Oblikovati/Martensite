// SPDX-License-Identifier: GPL-2.0-only
use super::super::BoundedArithmeticFailure;
use super::*;
use std::collections::BTreeSet;

pub(super) fn admit(
    query: CylindricalClearanceQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<BTreeSet<[usize; 2]>> {
    poll(cancelled)?;
    policy(query.options)?;
    count("sources", query.sources.len(), 16_384, true)?;
    value("requested clearance", 0, query.requested_clearance, true)?;
    for (index, scalar) in query.domain.offset().into_iter().enumerate() {
        value("relative translation", index, scalar, false)?;
    }
    value("support radius", 0, query.support_radius, true)?;
    value("angular origin", 0, query.angular_origin, false)?;
    for (index, source) in query.sources.iter().enumerate() {
        poll(cancelled)?;
        source_shape(index, source)?;
    }
    exclusions(query, cancelled)
}

fn policy(options: CylindricalClearanceOptions) -> Checked<()> {
    if !(8..=4096).contains(&options.fractional_bits)
        || options.max_domains > 1_000_000
        || options.max_curve_nodes > 65_536
        || options.max_depth > 32
        || options.max_operations > 1_000_000
    {
        return Err(CylindricalClearanceFailure::Policy(options));
    }
    Ok(())
}

fn count(field: &'static str, actual: usize, maximum: usize, nonempty: bool) -> Checked<()> {
    if actual > maximum || (nonempty && actual == 0) {
        return Err(CylindricalClearanceFailure::InputCount {
            field,
            actual,
            maximum,
        });
    }
    Ok(())
}

fn value(field: &'static str, index: usize, value: f64, positive: bool) -> Checked<()> {
    if !value.is_finite() || (positive && value <= 0.) {
        return Err(CylindricalClearanceFailure::Value {
            field,
            index,
            value,
        });
    }
    Ok(())
}

fn source_shape(index: usize, source: &CylindricalClearanceSource<'_>) -> Checked<()> {
    if !(2..=4).contains(&source.authored.len()) {
        return Err(CylindricalClearanceFailure::SourceShape {
            index,
            poles: source.authored.len(),
        });
    }
    for (pole, point) in source.authored.iter().enumerate() {
        finite_point(index, pole, point)?;
    }
    Ok(())
}

fn finite_point(source: usize, pole: usize, point: &[f64; 2]) -> Checked<()> {
    for (axis, value) in point.iter().enumerate() {
        finite_coordinate(source, pole, axis, *value)?;
    }
    Ok(())
}

fn finite_coordinate(source: usize, pole: usize, axis: usize, value: f64) -> Checked<()> {
    if !value.is_finite() {
        return Err(CylindricalClearanceFailure::SourceValue {
            source,
            pole,
            axis,
            value,
        });
    }
    Ok(())
}

fn exclusions(
    query: CylindricalClearanceQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<BTreeSet<[usize; 2]>> {
    let excluded = query.domain.exclusions();
    count("excluded pairs", excluded.len(), 65_536, false)?;
    let mut result = BTreeSet::new();
    for pair in excluded {
        poll(cancelled)?;
        if pair[0] >= pair[1] || pair[1] >= query.sources.len() || !result.insert(*pair) {
            return Err(CylindricalClearanceFailure::ExcludedPair {
                pair: *pair,
                source_count: query.sources.len(),
            });
        }
    }
    Ok(result)
}

fn poll(cancelled: &mut (dyn FnMut() -> bool + Send)) -> Checked<()> {
    if cancelled() {
        return Err(BoundedArithmeticFailure::Cancelled.into());
    }
    Ok(())
}
