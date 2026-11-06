#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepAP203RwCcDesignApproval,
    "RwStepAP203RwCcDesignApproval"
);
impl RwStepAP203RwCcDesignApproval {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP203CcDesignApproval>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "cc_design_approval")? {
            return Ok(());
        }
        let mut a_approval_assignment_assigned_approval: Handle<StepBasicApproval> =
            Default::default();
        data.read_entity(
            num,
            1,
            "approval_assignment.assigned_approval",
            ach,
            StepBasicApproval::static_type(),
            &mut a_approval_assignment_assigned_approval,
        )?;
        let mut a_items: Handle<HArray1<StepAP203ApprovedItem>> = Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "items", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            a_items = step_new_array1::<StepAP203ApprovedItem>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: StepAP203ApprovedItem = Default::default();
                data.read_entity_select(num2, i0, "items", ach, &mut an_it0)?;
                step_array1_set(&a_items, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(&a_approval_assignment_assigned_approval, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP203CcDesignApproval>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.assigned_approval())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.items(), "WriteStep")?.array1().length() {
            let var0: StepAP203ApprovedItem = step_array1_item(&ent.items(), i1, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP203CcDesignApproval>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.assigned_approval().cast());
        for i2 in 1..=step_live(&ent.items(), "Share")?.array1().length() {
            let var0: StepAP203ApprovedItem = step_array1_item(&ent.items(), i2, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepAP203RwCcDesignCertification,
    "RwStepAP203RwCcDesignCertification"
);
impl RwStepAP203RwCcDesignCertification {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP203CcDesignCertification>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "cc_design_certification")? {
            return Ok(());
        }
        let mut a_certification_assignment_assigned_certification: Handle<StepBasicCertification> =
            Default::default();
        data.read_entity(
            num,
            1,
            "certification_assignment.assigned_certification",
            ach,
            StepBasicCertification::static_type(),
            &mut a_certification_assignment_assigned_certification,
        )?;
        let mut a_items: Handle<HArray1<StepAP203CertifiedItem>> = Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "items", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            a_items = step_new_array1::<StepAP203CertifiedItem>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: StepAP203CertifiedItem = Default::default();
                data.read_entity_select(num2, i0, "items", ach, &mut an_it0)?;
                step_array1_set(&a_items, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(&a_certification_assignment_assigned_certification, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP203CcDesignCertification>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.assigned_certification())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.items(), "WriteStep")?.array1().length() {
            let var0: StepAP203CertifiedItem = step_array1_item(&ent.items(), i1, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP203CcDesignCertification>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.assigned_certification().cast());
        for i2 in 1..=step_live(&ent.items(), "Share")?.array1().length() {
            let var0: StepAP203CertifiedItem = step_array1_item(&ent.items(), i2, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepAP203RwCcDesignContract,
    "RwStepAP203RwCcDesignContract"
);
impl RwStepAP203RwCcDesignContract {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP203CcDesignContract>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "cc_design_contract")? {
            return Ok(());
        }
        let mut a_contract_assignment_assigned_contract: Handle<StepBasicContract> =
            Default::default();
        data.read_entity(
            num,
            1,
            "contract_assignment.assigned_contract",
            ach,
            StepBasicContract::static_type(),
            &mut a_contract_assignment_assigned_contract,
        )?;
        let mut a_items: Handle<HArray1<StepAP203ContractedItem>> = Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "items", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            a_items = step_new_array1::<StepAP203ContractedItem>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: StepAP203ContractedItem = Default::default();
                data.read_entity_select(num2, i0, "items", ach, &mut an_it0)?;
                step_array1_set(&a_items, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(&a_contract_assignment_assigned_contract, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP203CcDesignContract>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.assigned_contract())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.items(), "WriteStep")?.array1().length() {
            let var0: StepAP203ContractedItem = step_array1_item(&ent.items(), i1, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP203CcDesignContract>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.assigned_contract().cast());
        for i2 in 1..=step_live(&ent.items(), "Share")?.array1().length() {
            let var0: StepAP203ContractedItem = step_array1_item(&ent.items(), i2, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}
