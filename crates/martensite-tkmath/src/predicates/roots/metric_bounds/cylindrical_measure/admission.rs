// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn admit(query: CylindricalMeasureQuery<'_>) -> Checked<()> {
    admit_policy(query.options)?;
    admit_limit(query.limit)?;
    if query.sources.is_empty() || query.sources.len() > usize::from(query.options.max_sources) {
        return Err(CylindricalMeasureFailure::SourceCount);
    }
    if !query.depths.iter().all(|v| v.is_finite()) || query.depths[0] >= query.depths[1] {
        return Err(CylindricalMeasureFailure::Depths);
    }
    let expected = query.sources[0];
    for (index, source) in query.sources.iter().enumerate() {
        admit_source(index, source, &expected)?;
    }
    Ok(())
}
fn admit_policy(p: CylindricalMeasureOptions) -> Checked<()> {
    if p.behavior_version != 1
        || !(8..=4096).contains(&p.fractional_bits)
        || p.max_operations > 1_000_000
        || !(1..=4096).contains(&p.max_sources)
    {
        return Err(CylindricalMeasureFailure::Policy);
    }
    Ok(())
}

fn admit_limit(limit: CylindricalMeasureLimit) -> Checked<()> {
    if let CylindricalMeasureLimit::Refuse(cap) = limit
        && (!cap.is_finite() || cap < 0.)
    {
        return Err(CylindricalMeasureFailure::Policy);
    }
    Ok(())
}
fn admit_source(
    index: usize,
    source: &CylindricalProfileSource<'_>,
    expected: &CylindricalProfileSource<'_>,
) -> Checked<()> {
    if !source.matches()
        || source.support_radius.to_bits() != expected.support_radius.to_bits()
        || source.angular_origin.to_bits() != expected.angular_origin.to_bits()
    {
        return Err(CylindricalMeasureFailure::Source { index });
    }
    Ok(())
}
