#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepAP214RwAppliedPersonAndOrganizationAssignment,
    "RwStepAP214RwAppliedPersonAndOrganizationAssignment"
);
impl RwStepAP214RwAppliedPersonAndOrganizationAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AppliedPersonAndOrganizationAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "applied_person_and_organization_assignment")? {
            return Ok(());
        }
        rw_block_b_0136!(a_assigned_person_and_organization, a_role, ach, data, num);
        let mut a_items: Handle<HArray1<StepAP214PersonAndOrganizationItem>> = Default::default();
        let mut a_items_item: StepAP214PersonAndOrganizationItem = Default::default();
        let mut nsub3: i32 = Default::default();
        if data.read_sub_list(num, 3, "items", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            a_items = step_new_array1::<StepAP214PersonAndOrganizationItem>(1, nb3)?;
            for i3 in 1..=nb3 {
                let stat3: bool =
                    data.read_entity_select(nsub3, i3, "items", ach, &mut a_items_item)?;
                if stat3 {
                    step_array1_set(&a_items, i3, a_items_item.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_assigned_person_and_organization, &a_role, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AppliedPersonAndOrganizationAssignment>,
    ) -> Result<(), Failure> {
        rw_block_b_0116!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AppliedPersonAndOrganizationAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0145!(ent, is3, iter, nb_elem3);
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAppliedPresentedItem,
    "RwStepAP214RwAppliedPresentedItem"
);
impl RwStepAP214RwAppliedPresentedItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AppliedPresentedItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "applied_presented_item")? {
            return Ok(());
        }
        let mut a_items: Handle<HArray1<StepAP214PresentedItemSelect>> = Default::default();
        let mut anent1: StepAP214PresentedItemSelect = Default::default();
        let mut nsub1: i32 = Default::default();
        if data.read_sub_list(num, 1, "items", ach, &mut nsub1, false, 0, 0)? {
            let nb1: i32 = data.nb_params(nsub1)?;
            a_items = step_new_array1::<StepAP214PresentedItemSelect>(1, nb1)?;
            for i1 in 1..=nb1 {
                let stat1: bool = data.read_entity_select(nsub1, i1, "items", ach, &mut anent1)?;
                if stat1 {
                    step_array1_set(&a_items, i1, anent1.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AppliedPresentedItem>,
    ) -> Result<(), Failure> {
        rw_block_b_0199!(ent, i1, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AppliedPresentedItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_items();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.items_value(is1)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAppliedSecurityClassificationAssignment,
    "RwStepAP214RwAppliedSecurityClassificationAssignment"
);
impl RwStepAP214RwAppliedSecurityClassificationAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AppliedSecurityClassificationAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "applied_security_classification_assignment")? {
            return Ok(());
        }
        let mut a_assigned_security_classification: Handle<StepBasicSecurityClassification> =
            Default::default();
        data.read_entity(
            num,
            1,
            "assigned_security_classification",
            ach,
            StepBasicSecurityClassification::static_type(),
            &mut a_assigned_security_classification,
        )?;
        let mut a_items: Handle<HArray1<StepAP214SecurityClassificationItem>> = Default::default();
        let mut anent2: StepAP214SecurityClassificationItem = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "items", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_items = step_new_array1::<StepAP214SecurityClassificationItem>(1, nb2)?;
            for i2 in 1..=nb2 {
                let stat2: bool = data.read_entity_select(nsub2, i2, "items", ach, &mut anent2)?;
                if stat2 {
                    step_array1_set(&a_items, i2, anent2.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_assigned_security_classification, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AppliedSecurityClassificationAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assigned_security_classification())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_items() {
            sw.send_entity(&ent.items_value(i2)?.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AppliedSecurityClassificationAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.assigned_security_classification().cast());
        let nb_elem2: i32 = ent.nb_items();
        for is2 in 1..=nb_elem2 {
            iter.get_one_item(&ent.items_value(is2)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAutoDesignActualDateAndTimeAssignment,
    "RwStepAP214RwAutoDesignActualDateAndTimeAssignment"
);
impl RwStepAP214RwAutoDesignActualDateAndTimeAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignActualDateAndTimeAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "auto_design_actual_date_and_time_assignment")? {
            return Ok(());
        }
        rw_block_b_0045!(
            a_assigned_date_and_time,
            a_items,
            a_items_item,
            a_role,
            ach,
            data,
            ent,
            i3,
            nb3,
            nsub3,
            num,
            stat3
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AutoDesignActualDateAndTimeAssignment>,
    ) -> Result<(), Failure> {
        rw_block_b_0113!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignActualDateAndTimeAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0142!(ent, is3, iter, nb_elem3);
        Ok(())
    }
}
