#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepBasicRwApprovalRole, "RwStepBasicRwApprovalRole");
impl RwStepBasicRwApprovalRole {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicApprovalRole>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "approval_role")? {
            return Ok(());
        }
        let mut a_role: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "role", ach, &mut a_role)?;
        ent.init(&a_role);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicApprovalRole>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.role())?;
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwApprovalStatus, "RwStepBasicRwApprovalStatus");
impl RwStepBasicRwApprovalStatus {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicApprovalStatus>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "approval_status")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        ent.init(&a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicApprovalStatus>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwCalendarDate, "RwStepBasicRwCalendarDate");
impl RwStepBasicRwCalendarDate {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicCalendarDate>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "calendar_date")? {
            return Ok(());
        }
        rw_block_0157!(a_day_component, a_year_component, ach, data, num);
        let mut a_month_component: i32 = Default::default();
        data.read_integer(num, 3, "month_component", ach, &mut a_month_component)?;
        ent.init(a_year_component, a_day_component, a_month_component);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicCalendarDate>,
    ) -> Result<(), Failure> {
        sw.send_integer(ent.year_component())?;
        sw.send_integer(ent.day_component())?;
        sw.send_integer(ent.month_component())?;
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwCertification, "RwStepBasicRwCertification");
impl RwStepBasicRwCertification {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicCertification>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "certification")? {
            return Ok(());
        }
        rw_block_0147!(a_name, a_purpose, ach, data, num);
        let mut a_kind: Handle<StepBasicCertificationType> = Default::default();
        data.read_entity(
            num,
            3,
            "kind",
            ach,
            StepBasicCertificationType::static_type(),
            &mut a_kind,
        )?;
        ent.init(&a_name, &a_purpose, &a_kind);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicCertification>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.purpose())?;
        sw.send_entity(&ent.kind())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicCertification>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.kind().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwCertificationAssignment,
    "RwStepBasicRwCertificationAssignment"
);
impl RwStepBasicRwCertificationAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicCertificationAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "certification_assignment")? {
            return Ok(());
        }
        let mut a_assigned_certification: Handle<StepBasicCertification> = Default::default();
        data.read_entity(
            num,
            1,
            "assigned_certification",
            ach,
            StepBasicCertification::static_type(),
            &mut a_assigned_certification,
        )?;
        ent.init(&a_assigned_certification);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicCertificationAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assigned_certification())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicCertificationAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.assigned_certification().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwCertificationType,
    "RwStepBasicRwCertificationType"
);
impl RwStepBasicRwCertificationType {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicCertificationType>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "certification_type")? {
            return Ok(());
        }
        let mut a_description: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "description", ach, &mut a_description)?;
        ent.init(&a_description);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicCertificationType>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.description())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepBasicCertificationType>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
