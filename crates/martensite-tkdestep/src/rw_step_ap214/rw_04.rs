#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepAP214RwAutoDesignActualDateAssignment,
    "RwStepAP214RwAutoDesignActualDateAssignment"
);
impl RwStepAP214RwAutoDesignActualDateAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignActualDateAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "auto_design_actual_date_assignment")? {
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
        ent: &Handle<StepAP214AutoDesignActualDateAssignment>,
    ) -> Result<(), Failure> {
        rw_block_b_0114!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignActualDateAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0143!(ent, is3, iter, nb_elem3);
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAutoDesignApprovalAssignment,
    "RwStepAP214RwAutoDesignApprovalAssignment"
);
impl RwStepAP214RwAutoDesignApprovalAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignApprovalAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "auto_design_approval_assignment")? {
            return Ok(());
        }
        let mut a_assigned_approval: Handle<StepBasicApproval> = Default::default();
        data.read_entity(
            num,
            1,
            "assigned_approval",
            ach,
            StepBasicApproval::static_type(),
            &mut a_assigned_approval,
        )?;
        let mut a_items: Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>> = Default::default();
        let mut a_items_item: StepAP214AutoDesignGeneralOrgItem = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "items", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_items = step_new_array1::<StepAP214AutoDesignGeneralOrgItem>(1, nb2)?;
            for i2 in 1..=nb2 {
                let stat2: bool =
                    data.read_entity_select(nsub2, i2, "items", ach, &mut a_items_item)?;
                if stat2 {
                    step_array1_set(&a_items, i2, a_items_item.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_assigned_approval, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AutoDesignApprovalAssignment>,
    ) -> Result<(), Failure> {
        rw_block_b_0159!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignApprovalAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0184!(ent, is2, iter, nb_elem2);
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAutoDesignDateAndPersonAssignment,
    "RwStepAP214RwAutoDesignDateAndPersonAssignment"
);
impl RwStepAP214RwAutoDesignDateAndPersonAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignDateAndPersonAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "auto_design_date_and_person_assignment")? {
            return Ok(());
        }
        rw_block_b_0136!(a_assigned_person_and_organization, a_role, ach, data, num);
        let mut a_items: Handle<HArray1<StepAP214AutoDesignDateAndPersonItem>> = Default::default();
        let mut a_items_item: StepAP214AutoDesignDateAndPersonItem = Default::default();
        let mut nsub3: i32 = Default::default();
        if data.read_sub_list(num, 3, "items", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            a_items = step_new_array1::<StepAP214AutoDesignDateAndPersonItem>(1, nb3)?;
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
        ent: &Handle<StepAP214AutoDesignDateAndPersonAssignment>,
    ) -> Result<(), Failure> {
        rw_block_b_0116!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignDateAndPersonAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0145!(ent, is3, iter, nb_elem3);
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAutoDesignDocumentReference,
    "RwStepAP214RwAutoDesignDocumentReference"
);
impl RwStepAP214RwAutoDesignDocumentReference {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignDocumentReference>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "auto_deesign_document_reference")? {
            return Ok(());
        }
        rw_block_b_0140!(ach, adoc, asource, data, num);
        let mut a_items: Handle<HArray1<StepAP214AutoDesignReferencingItem>> = Default::default();
        let mut an_item: StepAP214AutoDesignReferencingItem = Default::default();
        let mut nsub3: i32 = Default::default();
        if data.read_sub_list(num, 3, "items", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            a_items = step_new_array1::<StepAP214AutoDesignReferencingItem>(1, nb3)?;
            for i3 in 1..=nb3 {
                let stat3: bool = data.read_entity_select(nsub3, i3, "item", ach, &mut an_item)?;
                if stat3 {
                    step_array1_set(&a_items, i3, an_item.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&adoc, &asource, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AutoDesignDocumentReference>,
    ) -> Result<(), Failure> {
        rw_block_b_0117!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignDocumentReference>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0216!(ent, i3, iter);
        Ok(())
    }
}
