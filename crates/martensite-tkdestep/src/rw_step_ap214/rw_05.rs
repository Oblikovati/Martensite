#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepAP214RwAutoDesignGroupAssignment,
    "RwStepAP214RwAutoDesignGroupAssignment"
);
impl RwStepAP214RwAutoDesignGroupAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignGroupAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "auto_design_group_assignment")? {
            return Ok(());
        }
        let mut a_assigned_group: Handle<StepBasicGroup> = Default::default();
        data.read_entity(
            num,
            1,
            "assigned_group",
            ach,
            StepBasicGroup::static_type(),
            &mut a_assigned_group,
        )?;
        let mut a_items: Handle<HArray1<StepAP214AutoDesignGroupedItem>> = Default::default();
        let mut a_items_item: StepAP214AutoDesignGroupedItem = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "items", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_items = step_new_array1::<StepAP214AutoDesignGroupedItem>(1, nb2)?;
            for i2 in 1..=nb2 {
                let stat2: bool =
                    data.read_entity_select(nsub2, i2, "items", ach, &mut a_items_item)?;
                if stat2 {
                    step_array1_set(&a_items, i2, a_items_item.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_assigned_group, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AutoDesignGroupAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assigned_group())?;
        rw_block_b_0195!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignGroupAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.assigned_group().cast());
        let nb_elem2: i32 = ent.nb_items()?;
        for is2 in 1..=nb_elem2 {
            iter.get_one_item(&ent.items_value(is2)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAutoDesignNominalDateAndTimeAssignment,
    "RwStepAP214RwAutoDesignNominalDateAndTimeAssignment"
);
impl RwStepAP214RwAutoDesignNominalDateAndTimeAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignNominalDateAndTimeAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "auto_design_nominal_date_and_time_assignment")? {
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
        ent: &Handle<StepAP214AutoDesignNominalDateAndTimeAssignment>,
    ) -> Result<(), Failure> {
        rw_block_b_0113!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignNominalDateAndTimeAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0142!(ent, is3, iter, nb_elem3);
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAutoDesignNominalDateAssignment,
    "RwStepAP214RwAutoDesignNominalDateAssignment"
);
impl RwStepAP214RwAutoDesignNominalDateAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignNominalDateAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "auto_design_nominal_date_assignment")? {
            return Ok(());
        }
        rw_block_b_0046!(
            a_assigned_date,
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
        ent: &Handle<StepAP214AutoDesignNominalDateAssignment>,
    ) -> Result<(), Failure> {
        rw_block_b_0114!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignNominalDateAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0143!(ent, is3, iter, nb_elem3);
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAutoDesignOrganizationAssignment,
    "RwStepAP214RwAutoDesignOrganizationAssignment"
);
impl RwStepAP214RwAutoDesignOrganizationAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignOrganizationAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "auto_design_organization_assignment")? {
            return Ok(());
        }
        rw_block_b_0135!(a_assigned_organization, a_role, ach, data, num);
        rw_block_b_0123!(a_items, a_items_item, ach, data, i3, nb3, nsub3, num, stat3);
        ent.init(&a_assigned_organization, &a_role, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AutoDesignOrganizationAssignment>,
    ) -> Result<(), Failure> {
        rw_block_b_0115!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignOrganizationAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0144!(ent, is3, iter, nb_elem3);
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAutoDesignPersonAndOrganizationAssignment,
    "RwStepAP214RwAutoDesignPersonAndOrganizationAssignment"
);
impl RwStepAP214RwAutoDesignPersonAndOrganizationAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignPersonAndOrganizationAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            3,
            ach,
            "auto_design_person_and_organization_assignment",
        )? {
            return Ok(());
        }
        rw_block_b_0136!(a_assigned_person_and_organization, a_role, ach, data, num);
        rw_block_b_0123!(a_items, a_items_item, ach, data, i3, nb3, nsub3, num, stat3);
        ent.init(&a_assigned_person_and_organization, &a_role, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AutoDesignPersonAndOrganizationAssignment>,
    ) -> Result<(), Failure> {
        rw_block_b_0116!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignPersonAndOrganizationAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0145!(ent, is3, iter, nb_elem3);
        Ok(())
    }
}
