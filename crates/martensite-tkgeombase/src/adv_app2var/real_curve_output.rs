//! Caller curve regions use the same initialized scalar owner as mapped inputs.
use super::*;
use std::ops::Range;
impl AdvApp2VarIsoRealFrame {
    pub(in crate::adv_app2var) fn configure_curve_region(
        &mut self,
        start: usize,
        length: usize,
    ) -> Result<AdvApp2VarCallerCurveSlots, Failure> {
        self.configure_curve_incidence(start, length, false)
    }
    pub(in crate::adv_app2var) fn configure_scalar_curve_region(
        &mut self,
        start: usize,
        length: usize,
    ) -> Result<AdvApp2VarCallerCurveSlots, Failure> {
        self.configure_curve_incidence(start, length, true)
    }
    fn configure_curve_incidence(
        &mut self,
        start: usize,
        length: usize,
        scalar_incidence: bool,
    ) -> Result<AdvApp2VarCallerCurveSlots, Failure> {
        let capacity = self.values.len();
        let end = start
            .checked_add(length)
            .filter(|end| length > 0 && *end <= capacity)
            .ok_or_else(|| storage::invalid_curve_region(start, length, capacity))?;
        let region = start..end;
        if !scalar_incidence {
            self.validate_curve_scalar_separation(&region)?;
        }
        let incidence = real_incidence(
            capacity,
            self.bindings,
            &self.inputs.slots,
            Some(region.clone()),
        )?;
        self.incidence = incidence;
        self.curve_region = Some(region.clone());
        Ok(self.values.curve_region(region))
    }
    fn validate_curve_scalar_separation(&self, region: &Range<usize>) -> Result<(), Failure> {
        for field in REAL_FIELDS {
            let slot = self.bindings.slot(field);
            if region.contains(&slot) {
                return Err(Failure::with_message(
                    "Standard_DomainError",
                    &format!(
                        "AdvApp2VarCallerCurveSlots: scalar {field:?} slot={slot} intersects region={region:?}; expected curve region disjoint from caller scalar fields; scalar-output incidence remains unsupported"
                    ),
                ));
            }
        }
        Ok(())
    }
    pub(in crate::adv_app2var) fn require_slice_curve(&self) -> Result<(), Failure> {
        if self.curve_region.is_none() {
            return Ok(());
        }
        Err(Failure::with_message(
            "Standard_DomainError",
            "AdvApp2VarCallerCurveSlots: owned region configured; expected owned caller-curve entrypoint instead of separate slice output",
        ))
    }
    pub(in crate::adv_app2var) fn validate_curve_owner(
        &self,
        curve: &AdvApp2VarCallerCurveSlots,
    ) -> Result<(), Failure> {
        let region = self
            .curve_region
            .as_ref()
            .ok_or_else(missing_curve_region)?;
        if self.values.owns_region(curve, region) {
            return Ok(());
        }
        Err(Failure::with_message(
            "Standard_DomainError",
            &format!(
                "AdvApp2VarCallerCurveSlots: owner_matches=false; expected operation-local curve owner and region={region:?}"
            ),
        ))
    }
    pub(in crate::adv_app2var) fn caller_curve_value(&self, index: usize) -> Result<f64, Failure> {
        Ok(self.values.value(self.curve_slot(index)?))
    }
    pub(in crate::adv_app2var) fn set_caller_curve_value(
        &mut self,
        index: usize,
        value: f64,
    ) -> Result<(), Failure> {
        let slot = self.curve_slot(index)?;
        self.values.set_value(slot, value);
        Ok(())
    }
    pub(in crate::adv_app2var) fn caller_curve_value_signed(
        &self,
        index: i32,
    ) -> Result<f64, Failure> {
        Ok(self.values.value(self.curve_signed_slot(index)?))
    }
    pub(in crate::adv_app2var) fn set_caller_curve_value_signed(
        &mut self,
        index: i32,
        value: f64,
    ) -> Result<(), Failure> {
        let slot = self.curve_signed_slot(index)?;
        self.values.set_value(slot, value);
        Ok(())
    }
    fn curve_signed_slot(&self, index: i32) -> Result<usize, Failure> {
        let region = self
            .curve_region
            .as_ref()
            .ok_or_else(missing_curve_region)?;
        checked_real_slot(region.start, index, region.end)
    }
    fn curve_slot(&self, index: usize) -> Result<usize, Failure> {
        let region = self
            .curve_region
            .as_ref()
            .ok_or_else(missing_curve_region)?;
        if index < region.len() {
            return Ok(region.start + index);
        }
        Err(Failure::with_message(
            "Standard_OutOfRange",
            &format!(
                "AdvApp2VarCallerCurveSlots: index={index}; expected curve element in 0..{}",
                region.len()
            ),
        ))
    }
}
fn missing_curve_region() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarCallerCurveSlots: region=None; expected configured operation-local caller curve region",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn frame(initial: f64) -> AdvApp2VarIsoRealFrame {
        let mut values = vec![7.; 7];
        values[5] = initial;
        AdvApp2VarIsoRealFrame::new(
            values,
            AdvApp2VarRealBindings {
                domain: [0, 1, 2, 3],
                constant_parameter: 4,
            },
        )
        .unwrap()
        .with_caller_array_slots(AdvApp2VarCallerRealArray::IsoRoots, vec![5])
        .unwrap()
    }
    fn request(roots: &[f64]) -> IsoApproxRequest<'_> {
        IsoApproxRequest {
            ndimen: 1,
            nbsesp: 1,
            ndimse: &[1],
            uvfonc: [0., 1., 0., 1.],
            tconst: 0.5,
            isofav: 1,
            nbroot: 1,
            rootlg: roots,
            iordre: -1,
            ideriv: 0,
            ndgjac: 1,
            nbcrmx: 1,
            ncflim: 1,
            epsapr: &[1e-6],
        }
    }
    #[test]
    fn explicit_scalar_curve_identity_retains_bit_consistent_seed_contract() {
        let mut owner = frame(0.25);
        let _curve = owner.configure_scalar_curve_region(4, 2).unwrap();
        let failure = owner.seed_request(&request(&[0.25])).unwrap_err();
        assert!(
            failure
                .message()
                .contains("incident CallerCurve[0] value=7.0")
        );
        owner.set_real(AdvApp2VarRealField::ConstantParameter, 0.5);
        owner.seed_request(&request(&[0.25])).unwrap();
        owner.set_caller_curve_value(0, 0.75).unwrap();
        assert_eq!(owner.real(AdvApp2VarRealField::ConstantParameter), 0.75);
        owner.set_real(AdvApp2VarRealField::ConstantParameter, 0.25);
        assert_eq!(owner.caller_curve_value(0).unwrap(), 0.25);
    }
    #[test]
    fn owner_and_region_identity_reject_foreign_or_obsolete_handles() {
        let (mut first, mut second) = (frame(0.25), frame(0.25));
        let old = first.configure_curve_region(5, 2).unwrap();
        let foreign = second.configure_curve_region(5, 2).unwrap();
        assert!(first.validate_curve_owner(&old).is_ok());
        assert!(first.validate_curve_owner(&foreign).is_err());
        let current = first.configure_curve_region(6, 1).unwrap();
        assert!(first.validate_curve_owner(&old).is_err());
        assert!(first.validate_curve_owner(&current).is_ok());
        assert!(first.require_slice_curve().is_err());
    }
    #[test]
    fn root_output_initialization_must_agree_bit_for_bit_before_clear() {
        let mut matching = frame(0.25);
        let _output = matching.configure_curve_region(5, 2).unwrap();
        matching.seed_request(&request(&[0.25])).unwrap();
        matching.set_caller_curve_value(0, 0.75).unwrap();
        assert_eq!(
            matching
                .caller_array_value(AdvApp2VarCallerRealArray::IsoRoots, 0)
                .unwrap(),
            0.75
        );
        let mut mismatch = frame(0.5);
        let _output = mismatch.configure_curve_region(5, 2).unwrap();
        let failure = mismatch.seed_request(&request(&[0.25])).unwrap_err();
        assert!(
            failure
                .message()
                .contains("incident CallerCurve[0] value=0.5")
        );
        assert_eq!(mismatch.caller_curve_value(0).unwrap(), 0.5);
        let mut sign = frame(-0.);
        let _output = sign.configure_curve_region(5, 2).unwrap();
        assert!(sign.seed_request(&request(&[0.])).is_err());
    }
    #[test]
    fn unconfigured_and_out_of_range_curve_writes_fail_without_touching_values() {
        let mut owner = frame(0.25);
        assert!(owner.require_slice_curve().is_ok());
        assert!(owner.set_caller_curve_value(0, 0.).is_err());
        let _output = owner.configure_curve_region(5, 2).unwrap();
        assert_eq!(
            owner
                .set_caller_curve_value(2, 0.)
                .unwrap_err()
                .exception_type(),
            "Standard_OutOfRange"
        );
        assert_eq!(owner.caller_curve_value(0).unwrap(), 0.25);
    }
    #[test]
    fn owned_root_curve_metadata_costs_are_recorded() {
        let mut owner = AdvApp2VarIsoRealFrame::new(
            vec![0.; 69],
            AdvApp2VarRealBindings {
                domain: [0, 1, 2, 3],
                constant_parameter: 4,
            },
        )
        .unwrap()
        .with_caller_array_slots(AdvApp2VarCallerRealArray::IsoRoots, (5..13).collect())
        .unwrap();
        let _curve = owner.configure_curve_region(5, 64).unwrap();
        let identities: usize = owner.incidence.iter().map(|group| group.fields.len()).sum();
        let field_capacity_bytes: usize = owner
            .incidence
            .iter()
            .map(|group| group.fields.capacity() * std::mem::size_of::<CallerRealField>())
            .sum();
        assert_eq!((owner.incidence.len(), identities), (13, 21));
        println!(
            "{{\"metric\":\"caller_root_curve_metadata\",\"groups\":{},\"logical_identities\":{identities},\"group_vec_capacity_bytes\":{},\"field_vec_capacity_bytes\":{field_capacity_bytes},\"curve_region_metadata_bytes\":{},\"real_frame_size\":{}}}",
            owner.incidence.len(),
            owner.incidence.capacity() * std::mem::size_of::<ScalarIncidence<CallerRealField>>(),
            std::mem::size_of::<Option<Range<usize>>>(),
            std::mem::size_of::<AdvApp2VarIsoRealFrame>()
        );
    }
    #[test]
    fn signed_curve_prefix_uses_actual_scalar_slots_and_preserves_positive_bounds() {
        let mut owner = frame(0.25);
        assert!(owner.caller_curve_value_signed(-1).is_err());
        let _curve = owner.configure_curve_region(5, 2).unwrap();
        owner.set_caller_curve_value_signed(-1, 0.5).unwrap();
        assert_eq!(owner.real(AdvApp2VarRealField::ConstantParameter), 0.5);
        owner.set_real(AdvApp2VarRealField::ConstantParameter, 0.75);
        assert_eq!(owner.caller_curve_value_signed(-1).unwrap(), 0.75);
        for index in [-6, 2, i32::MAX] {
            assert!(owner.set_caller_curve_value_signed(index, 1.).is_err());
        }
        assert!(owner.caller_curve_value(2).is_err());
        assert_eq!(owner.caller_curve_value_signed(-1).unwrap(), 0.75);
    }
}
