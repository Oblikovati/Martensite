//! Sum nonnegative upper bounds outward, in authored region order, without cancelling errors.
use super::*;
pub(super) fn admit(limit: CylindricalMeasureLimit) -> Checked<()> {
    match limit {
        CylindricalMeasureLimit::Report => Ok(()),
        CylindricalMeasureLimit::Refuse(value) => admit_allowance(value),
    }
}
fn admit_allowance(value: f64) -> Checked<()> {
    if !value.is_finite() || value < 0. {
        return Err(policy(
            "finite nonnegative retained/ideal volume allowance",
            value,
        ));
    }
    Ok(())
}

impl FamilyVolumeBounds {
    pub(super) fn certify(
        family: &family::UnqualifiedFamily,
        limit: CylindricalMeasureLimit,
    ) -> Checked<Self> {
        let mut volume = Self::default();
        for member in family.members.values() {
            volume.add(member)?;
        }
        volume.enforce(limit)?;
        Ok(volume)
    }
    pub(super) fn add(&mut self, member: &pipeline::RetainedMember) -> Checked<()> {
        self.comparison(member)?;
        self.auxiliary(member)
    }
    fn comparison(&mut self, member: &pipeline::RetainedMember) -> Checked<()> {
        let proof = &member.measures.comparison;
        self.discrepancy = add_upper(
            self.discrepancy,
            proof.discrepancy_bound(),
            Quantity::RetainedIdeal,
        )?;
        self.representative_error = add_upper(
            self.representative_error,
            proof.representative_error_bound(),
            Quantity::RepresentativeIdeal,
        )?;
        Ok(())
    }
    fn auxiliary(&mut self, member: &pipeline::RetainedMember) -> Checked<()> {
        let proof = &member.measures.comparison;
        self.integration_error = add_upper(
            self.integration_error,
            proof.flux_certificate().absolute_error_bound(),
            Quantity::Integration,
        )?;
        self.profile_error = add_upper(
            self.profile_error,
            proof.normal_certificate().volume_discrepancy_bound(),
            Quantity::AuthoredIdeal,
        )?;
        Ok(())
    }
    pub(super) fn enforce(&self, limit: CylindricalMeasureLimit) -> Checked<()> {
        let CylindricalMeasureLimit::Refuse(limit) = limit else {
            return Ok(());
        };
        for (quantity, actual) in [
            (Quantity::RetainedIdeal, self.discrepancy),
            (Quantity::RepresentativeIdeal, self.representative_error),
        ] {
            if actual > limit {
                return Err(exceeded(quantity, actual, limit, None, None));
            }
        }
        Ok(())
    }
}
pub(super) fn add_upper(accumulated: f64, contribution: f64, quantity: Quantity) -> Checked<f64> {
    valid_bound(accumulated, quantity)?;
    valid_bound(contribution, quantity)?;
    if contribution == 0. {
        return Ok(accumulated);
    }
    if accumulated == 0. {
        return Ok(contribution);
    }
    let upper = (accumulated + contribution).next_up();
    if !upper.is_finite() {
        return Err(BudgetFailure::Overflow {
            quantity,
            accumulated,
            contribution,
        });
    }
    Ok(upper)
}

fn valid_bound(value: f64, quantity: Quantity) -> Checked<()> {
    if !value.is_finite() || value < 0. {
        return Err(BudgetFailure::Invalid { quantity, value });
    }
    Ok(())
}
