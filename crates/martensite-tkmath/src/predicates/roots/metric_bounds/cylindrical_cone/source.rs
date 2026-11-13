// SPDX-License-Identifier: GPL-2.0-only
use super::*;

pub(super) fn derivatives(
    source: &CylindricalConeSource<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<[Interval; 2]>> {
    let poles = curve::capture_planar(source.authored, arithmetic)?;
    let mut axes = Vec::new();
    for axis in &poles {
        let restricted = curve::restrict(
            axis,
            source.interval.index(),
            source.interval.level(),
            arithmetic,
        )?;
        axes.push(curve::derivative(&restricted, arithmetic)?);
    }
    let count = axes[0].len().max(axes[1].len());
    // Only an exactly constant coordinate is compressed; its zero derivative can be repeated.
    Ok((0..count)
        .map(|i| [coefficient(&axes[0], i), coefficient(&axes[1], i)])
        .collect())
}

fn coefficient(axis: &[Interval], index: usize) -> Interval {
    axis[if axis.len() == 1 { 0 } else { index }].clone()
}
