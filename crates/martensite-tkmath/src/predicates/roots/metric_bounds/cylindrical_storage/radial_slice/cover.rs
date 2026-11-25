// SPDX-License-Identifier: GPL-2.0-only
//! Prove a common radius interval exists along every extended span parameter.
use super::*;

impl CertifiedCylindricalSlice {
    /// Prove every parameter has its unique fixed-radius sample over the common radial interval.
    ///
    /// The interval's exact endpoints are `R_lower - allowance_lower` and
    /// `R_upper + allowance_upper`. Supply the same radii and allowances to each
    /// side participating in a pair proof. The authoritative storage proof must
    /// match both rails in increasing-radius order.
    /// Example: `slice.require_common_radial_interval(query, &storage, allowance, || false)?`.
    pub fn require_common_radial_interval(
        &self,
        query: CylindricalSliceQuery<'_>,
        storage: &CertifiedCylindricalStorage,
        allowance: [f64; 2],
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Checked<()> {
        let radii = storage.stored_radii();
        self.admit_cover(query, storage, radii, allowance)?;
        let mut arithmetic = Arithmetic::new(arithmetic_policy(query.options), &mut cancelled);
        let extension = extension(query, &mut arithmetic)?;
        let slope = arithmetic.scalar(self.signed_squared_radius_derivative_lower())?;
        for index in 0..2 {
            require_extension(
                index,
                [radii[index], allowance[index]],
                &extension[index],
                &slope,
                &mut arithmetic,
            )?;
        }
        Ok(())
    }

    fn admit_cover(
        &self,
        query: CylindricalSliceQuery<'_>,
        storage: &CertifiedCylindricalStorage,
        radii: [f64; 2],
        allowance: [f64; 2],
    ) -> Checked<()> {
        admission::admit_policy(query.options)?;
        let rails = depth_order(query.rails, query.sense);
        let valid_allowance = storage
            .radial_distance_bounds()
            .into_iter()
            .zip(radii)
            .zip(allowance)
            .all(|((bound, radius), allowance)| {
                allowance.is_finite() && bound <= allowance && allowance < radius
            });
        if !valid_allowance
            || radii[0] >= radii[1]
            || query.depth[0] >= 0.
            || query.depth[1] <= 1.
            || !self.matches_source(query)
            || !storage.matches_source(rails, radii)
        {
            return Err(CylindricalSliceRefusal::RadialCoverSource {
                radii,
                allowance,
                depth: query.depth,
            });
        }
        Ok(())
    }
}

fn depth_order<T>(values: [T; 2], sense: CylindricalRadialSense) -> [T; 2] {
    match sense {
        CylindricalRadialSense::Outward => values,
        CylindricalRadialSense::Inward => {
            let [first, last] = values;
            [last, first]
        }
    }
}

fn extension(
    query: CylindricalSliceQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Interval; 2]> {
    let first = arithmetic.scalar(-query.depth[0])?;
    let last = arithmetic.scalar(query.depth[1])?;
    let one = arithmetic.scalar(1.)?;
    let last = arithmetic.subtract(&last, &one)?;
    Ok(depth_order([first, last], query.sense))
}

fn require_extension(
    radius_index: usize,
    [radius, allowance]: [f64; 2],
    extension: &Interval,
    slope: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<()> {
    let radius = arithmetic.scalar(radius)?;
    let allowance = arithmetic.scalar(allowance)?;
    let product = arithmetic.multiply(&radius, &allowance)?;
    // (R + allowance)^2 - (R - allowance)^2 is the worst endpoint excursion.
    let required = arithmetic.fraction(&product, 4, 1)?;
    let achieved = arithmetic.multiply(extension, slope)?;
    if achieved.lower <= required.upper {
        return Err(CylindricalSliceRefusal::RadialCoverMargin {
            radius_index,
            required: arithmetic.upper_f64(required.upper)?,
            achieved: -arithmetic.upper_f64(-achieved.lower)?,
        });
    }
    Ok(())
}
