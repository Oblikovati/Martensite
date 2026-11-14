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
    Ok(curve::align_planar(&axes[0], &axes[1]))
}
