// SPDX-License-Identifier: GPL-2.0-only
//! Reuse componentwise retained residuals in the unwrapped angle/slant plane.
use super::super::developed::CombinedFailure;
use super::super::spatial::SpatialFailure;
use super::*;

impl JoinOperands<'_> {
    pub(in super::super) fn motion_upper(
        &self,
        reference: f64,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Result<num_bigint::BigInt, CombinedFailure> {
        let original = self
            .capture
            .comparison
            .as_ref()
            .ok_or(CombinedFailure::MissingOriginal)?;
        if original.displacement.anchor.turn_adjustment != Some(0) {
            return Err(CombinedFailure::UnalignedAnchor);
        }
        let angular = arithmetic.divide(&original.angle, &self.capture.sine)?;
        let angular = arithmetic.add(&angular, &self.capture.angular)?;
        let angular = arithmetic.add(&angular, &self.trace.angular)?;
        let slant = self.slant_upper(reference, &original.displacement.delta, arithmetic)?;
        // This L1 sum bounds Euclidean motion in the numerical cap coordinates.
        Ok(arithmetic.add(&angular, &slant)?.upper)
    }

    fn slant_upper(
        &self,
        reference: f64,
        original: &Interval,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Result<Interval, CombinedFailure> {
        let (trim, _) =
            material::slant_difference(reference, self.trace, &self.capture.weights, arithmetic)
                .map_err(slant_failure)?;
        let fit = arithmetic.add(original, &self.capture.slant)?;
        Ok(arithmetic.add(&fit, &trim)?)
    }
}

fn slant_failure(cause: CylindricalProfileFailure) -> CombinedFailure {
    match cause {
        CylindricalProfileFailure::Arithmetic(cause) => (*cause).into(),
        other => CombinedFailure::Spatial(SpatialFailure::Original(
            ConicalOriginalFailure::Polynomial(other),
        )),
    }
}
