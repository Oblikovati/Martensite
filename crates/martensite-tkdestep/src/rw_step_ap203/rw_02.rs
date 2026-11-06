#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepAP203RwCcDesignDateAndTimeAssignment,
    "RwStepAP203RwCcDesignDateAndTimeAssignment"
);
impl RwStepAP203RwCcDesignDateAndTimeAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP203CcDesignDateAndTimeAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "cc_design_date_and_time_assignment")? {
            return Ok(());
        }
        let mut a_date_and_time_assignment_assigned_date_and_time: Handle<StepBasicDateAndTime> =
            Default::default();
        data.read_entity(
            num,
            1,
            "date_and_time_assignment.assigned_date_and_time",
            ach,
            StepBasicDateAndTime::static_type(),
            &mut a_date_and_time_assignment_assigned_date_and_time,
        )?;
        let mut a_date_and_time_assignment_role: Handle<StepBasicDateTimeRole> = Default::default();
        data.read_entity(
            num,
            2,
            "date_and_time_assignment.role",
            ach,
            StepBasicDateTimeRole::static_type(),
            &mut a_date_and_time_assignment_role,
        )?;
        let mut a_items: Handle<HArray1<StepAP203DateTimeItem>> = Default::default();
        let mut sub3: i32 = 0;
        if data.read_sub_list(num, 3, "items", ach, &mut sub3, false, 0, 0)? {
            let num2: i32 = sub3;
            let nb0: i32 = data.nb_params(num2)?;
            a_items = step_new_array1::<StepAP203DateTimeItem>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: StepAP203DateTimeItem = Default::default();
                data.read_entity_select(num2, i0, "items", ach, &mut an_it0)?;
                step_array1_set(&a_items, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            &a_date_and_time_assignment_assigned_date_and_time,
            &a_date_and_time_assignment_role,
            &a_items,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP203CcDesignDateAndTimeAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.assigned_date_and_time())?;
        sw.send_entity(&ent.parent.role())?;
        sw.open_sub()?;
        for i2 in 1..=step_live(&ent.items(), "WriteStep")?.array1().length() {
            let var0: StepAP203DateTimeItem = step_array1_item(&ent.items(), i2, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP203CcDesignDateAndTimeAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.assigned_date_and_time().cast());
        iter.add_item(&ent.parent.role().cast());
        for i3 in 1..=step_live(&ent.items(), "Share")?.array1().length() {
            let var0: StepAP203DateTimeItem = step_array1_item(&ent.items(), i3, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepAP203RwCcDesignPersonAndOrganizationAssignment,
    "RwStepAP203RwCcDesignPersonAndOrganizationAssignment"
);
impl RwStepAP203RwCcDesignPersonAndOrganizationAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP203CcDesignPersonAndOrganizationAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "cc_design_person_and_organization_assignment")? {
            return Ok(());
        }
        let mut a_person_and_organization_assignment_assigned_person_and_organization: Handle<
            StepBasicPersonAndOrganization,
        > = Default::default();
        data.read_entity(
            num,
            1,
            "person_and_organization_assignment.assigned_person_and_organization",
            ach,
            StepBasicPersonAndOrganization::static_type(),
            &mut a_person_and_organization_assignment_assigned_person_and_organization,
        )?;
        let mut a_person_and_organization_assignment_role: Handle<
            StepBasicPersonAndOrganizationRole,
        > = Default::default();
        data.read_entity(
            num,
            2,
            "person_and_organization_assignment.role",
            ach,
            StepBasicPersonAndOrganizationRole::static_type(),
            &mut a_person_and_organization_assignment_role,
        )?;
        let mut a_items: Handle<HArray1<StepAP203PersonOrganizationItem>> = Default::default();
        let mut sub3: i32 = 0;
        if data.read_sub_list(num, 3, "items", ach, &mut sub3, false, 0, 0)? {
            let num2: i32 = sub3;
            let nb0: i32 = data.nb_params(num2)?;
            a_items = step_new_array1::<StepAP203PersonOrganizationItem>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: StepAP203PersonOrganizationItem = Default::default();
                data.read_entity_select(num2, i0, "items", ach, &mut an_it0)?;
                step_array1_set(&a_items, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            &a_person_and_organization_assignment_assigned_person_and_organization,
            &a_person_and_organization_assignment_role,
            &a_items,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP203CcDesignPersonAndOrganizationAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.assigned_person_and_organization())?;
        sw.send_entity(&ent.parent.role())?;
        sw.open_sub()?;
        for i2 in 1..=step_live(&ent.items(), "WriteStep")?.array1().length() {
            let var0: StepAP203PersonOrganizationItem =
                step_array1_item(&ent.items(), i2, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP203CcDesignPersonAndOrganizationAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.assigned_person_and_organization().cast());
        iter.add_item(&ent.parent.role().cast());
        for i3 in 1..=step_live(&ent.items(), "Share")?.array1().length() {
            let var0: StepAP203PersonOrganizationItem =
                step_array1_item(&ent.items(), i3, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepAP203RwCcDesignSecurityClassification,
    "RwStepAP203RwCcDesignSecurityClassification"
);
impl RwStepAP203RwCcDesignSecurityClassification {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP203CcDesignSecurityClassification>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "cc_design_security_classification")? {
            return Ok(());
        }
        let mut a_security_classification_assignment_assigned_security_classification: Handle<
            StepBasicSecurityClassification,
        > = Default::default();
        data.read_entity(
            num,
            1,
            "security_classification_assignment.assigned_security_classification",
            ach,
            StepBasicSecurityClassification::static_type(),
            &mut a_security_classification_assignment_assigned_security_classification,
        )?;
        let mut a_items: Handle<HArray1<StepAP203ClassifiedItem>> = Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "items", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            a_items = step_new_array1::<StepAP203ClassifiedItem>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: StepAP203ClassifiedItem = Default::default();
                data.read_entity_select(num2, i0, "items", ach, &mut an_it0)?;
                step_array1_set(&a_items, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            &a_security_classification_assignment_assigned_security_classification,
            &a_items,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP203CcDesignSecurityClassification>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.assigned_security_classification())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.items(), "WriteStep")?.array1().length() {
            let var0: StepAP203ClassifiedItem = step_array1_item(&ent.items(), i1, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP203CcDesignSecurityClassification>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.assigned_security_classification().cast());
        for i2 in 1..=step_live(&ent.items(), "Share")?.array1().length() {
            let var0: StepAP203ClassifiedItem = step_array1_item(&ent.items(), i2, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}
