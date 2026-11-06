#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepAP203RwCcDesignSpecificationReference,
    "RwStepAP203RwCcDesignSpecificationReference"
);
impl RwStepAP203RwCcDesignSpecificationReference {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP203CcDesignSpecificationReference>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "cc_design_specification_reference")? {
            return Ok(());
        }
        let mut a_document_reference_assigned_document: Handle<StepBasicDocument> =
            Default::default();
        data.read_entity(
            num,
            1,
            "document_reference.assigned_document",
            ach,
            StepBasicDocument::static_type(),
            &mut a_document_reference_assigned_document,
        )?;
        let mut a_document_reference_source: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            2,
            "document_reference.source",
            ach,
            &mut a_document_reference_source,
        )?;
        let mut a_items: Handle<HArray1<StepAP203SpecifiedItem>> = Default::default();
        let mut sub3: i32 = 0;
        if data.read_sub_list(num, 3, "items", ach, &mut sub3, false, 0, 0)? {
            let num2: i32 = sub3;
            let nb0: i32 = data.nb_params(num2)?;
            a_items = step_new_array1::<StepAP203SpecifiedItem>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: StepAP203SpecifiedItem = Default::default();
                data.read_entity_select(num2, i0, "items", ach, &mut an_it0)?;
                step_array1_set(&a_items, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            &a_document_reference_assigned_document,
            &a_document_reference_source,
            &a_items,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP203CcDesignSpecificationReference>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.assigned_document())?;
        sw.send_entity(&ent.parent.source())?;
        sw.open_sub()?;
        for i2 in 1..=step_live(&ent.items(), "WriteStep")?.array1().length() {
            let var0: StepAP203SpecifiedItem = step_array1_item(&ent.items(), i2, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP203CcDesignSpecificationReference>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.assigned_document().cast());
        for i2 in 1..=step_live(&ent.items(), "Share")?.array1().length() {
            let var0: StepAP203SpecifiedItem = step_array1_item(&ent.items(), i2, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}

step_rw_class!(RwStepAP203RwChange, "RwStepAP203RwChange");
impl RwStepAP203RwChange {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP203Change>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "change")? {
            return Ok(());
        }
        rw_block_b_0082!(
            a_action_assignment_assigned_action,
            a_items,
            ach,
            an_it0,
            data,
            ent,
            i0,
            nb0,
            num,
            num2,
            sub2
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP203Change>,
    ) -> Result<(), Failure> {
        rw_block_b_0130!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP203Change>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0205!(ent, i2, iter, var0);
        Ok(())
    }
}

step_rw_class!(RwStepAP203RwChangeRequest, "RwStepAP203RwChangeRequest");
impl RwStepAP203RwChangeRequest {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP203ChangeRequest>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "change_request")? {
            return Ok(());
        }
        let mut a_action_request_assignment_assigned_action_request: Handle<
            StepBasicVersionedActionRequest,
        > = Default::default();
        data.read_entity(
            num,
            1,
            "action_request_assignment.assigned_action_request",
            ach,
            StepBasicVersionedActionRequest::static_type(),
            &mut a_action_request_assignment_assigned_action_request,
        )?;
        let mut a_items: Handle<HArray1<StepAP203ChangeRequestItem>> = Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "items", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            a_items = step_new_array1::<StepAP203ChangeRequestItem>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: StepAP203ChangeRequestItem = Default::default();
                data.read_entity_select(num2, i0, "items", ach, &mut an_it0)?;
                step_array1_set(&a_items, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            &a_action_request_assignment_assigned_action_request,
            &a_items,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP203ChangeRequest>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.assigned_action_request())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.items(), "WriteStep")?.array1().length() {
            let var0: StepAP203ChangeRequestItem = step_array1_item(&ent.items(), i1, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP203ChangeRequest>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.assigned_action_request().cast());
        for i2 in 1..=step_live(&ent.items(), "Share")?.array1().length() {
            let var0: StepAP203ChangeRequestItem = step_array1_item(&ent.items(), i2, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}
