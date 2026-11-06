#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepAP214RwAppliedExternalIdentificationAssignment,
    "RwStepAP214RwAppliedExternalIdentificationAssignment"
);
impl RwStepAP214RwAppliedExternalIdentificationAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AppliedExternalIdentificationAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "applied_external_identification_assignment")? {
            return Ok(());
        }
        let mut a_identification_assignment_assigned_id: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "identification_assignment.assigned_id",
            ach,
            &mut a_identification_assignment_assigned_id,
        )?;
        let mut a_identification_assignment_role: Handle<StepBasicIdentificationRole> =
            Default::default();
        data.read_entity(
            num,
            2,
            "identification_assignment.role",
            ach,
            StepBasicIdentificationRole::static_type(),
            &mut a_identification_assignment_role,
        )?;
        let mut a_external_identification_assignment_source: Handle<StepBasicExternalSource> =
            Default::default();
        data.read_entity(
            num,
            3,
            "external_identification_assignment.source",
            ach,
            StepBasicExternalSource::static_type(),
            &mut a_external_identification_assignment_source,
        )?;
        let mut a_items: Handle<HArray1<StepAP214ExternalIdentificationItem>> = Default::default();
        let mut sub4: i32 = 0;
        if data.read_sub_list(num, 4, "items", ach, &mut sub4, false, 0, 0)? {
            let num2: i32 = sub4;
            let nb0: i32 = data.nb_params(num2)?;
            a_items = step_new_array1::<StepAP214ExternalIdentificationItem>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: StepAP214ExternalIdentificationItem = Default::default();
                data.read_entity_select(num2, i0, "items", ach, &mut an_it0)?;
                step_array1_set(&a_items, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            &a_identification_assignment_assigned_id,
            &a_identification_assignment_role,
            &a_external_identification_assignment_source,
            &a_items,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AppliedExternalIdentificationAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.assigned_id())?;
        sw.send_entity(&ent.parent.parent.role())?;
        sw.send_entity(&ent.parent.source())?;
        sw.open_sub()?;
        for i3 in 1..=step_live(&ent.items(), "WriteStep")?.array1().length() {
            let var0: StepAP214ExternalIdentificationItem =
                step_array1_item(&ent.items(), i3, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AppliedExternalIdentificationAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.parent.role().cast());
        iter.add_item(&ent.parent.source().cast());
        for i3 in 1..=step_live(&ent.items(), "Share")?.array1().length() {
            let var0: StepAP214ExternalIdentificationItem =
                step_array1_item(&ent.items(), i3, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAppliedGroupAssignment,
    "RwStepAP214RwAppliedGroupAssignment"
);
impl RwStepAP214RwAppliedGroupAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AppliedGroupAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "applied_group_assignment")? {
            return Ok(());
        }
        let mut a_group_assignment_assigned_group: Handle<StepBasicGroup> = Default::default();
        data.read_entity(
            num,
            1,
            "group_assignment.assigned_group",
            ach,
            StepBasicGroup::static_type(),
            &mut a_group_assignment_assigned_group,
        )?;
        let mut an_items: Handle<HArray1<StepAP214GroupItem>> = Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "items", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            if nb0 != 0 {
                an_items = step_new_array1::<StepAP214GroupItem>(1, nb0)?;
                for i0 in 1..=nb0 {
                    let mut an_it0: StepAP214GroupItem = Default::default();
                    data.read_entity_select(num2, i0, "items", ach, &mut an_it0)?;
                    step_array1_set(&an_items, i0, an_it0.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_group_assignment_assigned_group, &an_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AppliedGroupAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.assigned_group())?;
        sw.open_sub()?;
        if !ent.items().is_null() {
            for i1 in 1..=step_live(&ent.items(), "WriteStep")?.array1().length() {
                let var0: StepAP214GroupItem = step_array1_item(&ent.items(), i1, "WriteStep")?;
                sw.send_entity(&var0.value())?;
            }
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AppliedGroupAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.assigned_group().cast());
        if !ent.items().is_null() {
            for i2 in 1..=step_live(&ent.items(), "Share")?.array1().length() {
                let var0: StepAP214GroupItem = step_array1_item(&ent.items(), i2, "Share")?;
                iter.add_item(&var0.value());
            }
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAppliedOrganizationAssignment,
    "RwStepAP214RwAppliedOrganizationAssignment"
);
impl RwStepAP214RwAppliedOrganizationAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AppliedOrganizationAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "applied_organization_assignment")? {
            return Ok(());
        }
        rw_block_b_0135!(a_assigned_organization, a_role, ach, data, num);
        let mut a_items: Handle<HArray1<StepAP214OrganizationItem>> = Default::default();
        let mut a_items_item: StepAP214OrganizationItem = Default::default();
        let mut nsub3: i32 = Default::default();
        if data.read_sub_list(num, 3, "items", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            a_items = step_new_array1::<StepAP214OrganizationItem>(1, nb3)?;
            for i3 in 1..=nb3 {
                let stat3: bool =
                    data.read_entity_select(nsub3, i3, "items", ach, &mut a_items_item)?;
                if stat3 {
                    step_array1_set(&a_items, i3, a_items_item.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_assigned_organization, &a_role, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AppliedOrganizationAssignment>,
    ) -> Result<(), Failure> {
        rw_block_b_0115!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AppliedOrganizationAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0144!(ent, is3, iter, nb_elem3);
        Ok(())
    }
}
