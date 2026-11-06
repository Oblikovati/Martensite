#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwApplicationProtocolDefinition,
    "RwStepBasicRwApplicationProtocolDefinition"
);
impl RwStepBasicRwApplicationProtocolDefinition {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicApplicationProtocolDefinition>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "application_protocol_definition")? {
            return Ok(());
        }
        let mut a_status: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "status", ach, &mut a_status)?;
        let mut a_application_interpreted_model_schema_name: Handle<HAsciiString> =
            Default::default();
        data.read_string(
            num,
            2,
            "application_interpreted_model_schema_name",
            ach,
            &mut a_application_interpreted_model_schema_name,
        )?;
        let mut a_application_protocol_year: i32 = Default::default();
        data.read_integer(
            num,
            3,
            "application_protocol_year",
            ach,
            &mut a_application_protocol_year,
        )?;
        let mut a_application: Handle<StepBasicApplicationContext> = Default::default();
        data.read_entity(
            num,
            4,
            "application",
            ach,
            StepBasicApplicationContext::static_type(),
            &mut a_application,
        )?;
        ent.init(
            &a_status,
            &a_application_interpreted_model_schema_name,
            a_application_protocol_year,
            &a_application,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicApplicationProtocolDefinition>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.status())?;
        sw.send_entity(&ent.application_interpreted_model_schema_name())?;
        sw.send_integer(ent.application_protocol_year())?;
        sw.send_entity(&ent.application())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicApplicationProtocolDefinition>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.application().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwApproval, "RwStepBasicRwApproval");
impl RwStepBasicRwApproval {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicApproval>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "approval")? {
            return Ok(());
        }
        let mut a_status: Handle<StepBasicApprovalStatus> = Default::default();
        data.read_entity(
            num,
            1,
            "status",
            ach,
            StepBasicApprovalStatus::static_type(),
            &mut a_status,
        )?;
        let mut a_level: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "level", ach, &mut a_level)?;
        ent.init(&a_status, &a_level);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicApproval>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.status())?;
        sw.send_entity(&ent.level())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicApproval>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.status().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwApprovalDateTime,
    "RwStepBasicRwApprovalDateTime"
);
impl RwStepBasicRwApprovalDateTime {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicApprovalDateTime>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "approval_date_time")? {
            return Ok(());
        }
        let mut dts: StepBasicDateTimeSelect = Default::default();
        data.read_entity_select(num, 1, "date_time", ach, &mut dts)?;
        let mut ap: Handle<StepBasicApproval> = Default::default();
        data.read_entity(
            num,
            2,
            "dated_approval",
            ach,
            StepBasicApproval::static_type(),
            &mut ap,
        )?;
        ent.init(&dts, &ap);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicApprovalDateTime>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.date_time().value())?;
        sw.send_entity(&ent.dated_approval())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicApprovalDateTime>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.date_time().value());
        iter.get_one_item(&ent.dated_approval().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwApprovalPersonOrganization,
    "RwStepBasicRwApprovalPersonOrganization"
);
impl RwStepBasicRwApprovalPersonOrganization {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicApprovalPersonOrganization>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "approval_person_organization")? {
            return Ok(());
        }
        let mut a_person_organization: StepBasicPersonOrganizationSelect = Default::default();
        data.read_entity_select(
            num,
            1,
            "person_organization",
            ach,
            &mut a_person_organization,
        )?;
        let mut a_authorized_approval: Handle<StepBasicApproval> = Default::default();
        data.read_entity(
            num,
            2,
            "authorized_approval",
            ach,
            StepBasicApproval::static_type(),
            &mut a_authorized_approval,
        )?;
        let mut a_role: Handle<StepBasicApprovalRole> = Default::default();
        data.read_entity(
            num,
            3,
            "role",
            ach,
            StepBasicApprovalRole::static_type(),
            &mut a_role,
        )?;
        ent.init(&a_person_organization, &a_authorized_approval, &a_role);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicApprovalPersonOrganization>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.person_organization().value())?;
        sw.send_entity(&ent.authorized_approval())?;
        sw.send_entity(&ent.role())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicApprovalPersonOrganization>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.person_organization().value());
        iter.get_one_item(&ent.authorized_approval().cast());
        iter.get_one_item(&ent.role().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwApprovalRelationship,
    "RwStepBasicRwApprovalRelationship"
);
impl RwStepBasicRwApprovalRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicApprovalRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "approval_relationship")? {
            return Ok(());
        }
        rw_block_0126!(a_description, a_name, ach, data, num);
        let mut a_relating_approval: Handle<StepBasicApproval> = Default::default();
        data.read_entity(
            num,
            3,
            "relating_approval",
            ach,
            StepBasicApproval::static_type(),
            &mut a_relating_approval,
        )?;
        let mut a_related_approval: Handle<StepBasicApproval> = Default::default();
        data.read_entity(
            num,
            4,
            "related_approval",
            ach,
            StepBasicApproval::static_type(),
            &mut a_related_approval,
        )?;
        ent.init(
            &a_name,
            &a_description,
            &a_relating_approval,
            &a_related_approval,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicApprovalRelationship>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        sw.send_entity(&ent.relating_approval())?;
        sw.send_entity(&ent.related_approval())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicApprovalRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.relating_approval().cast());
        iter.get_one_item(&ent.related_approval().cast());
        Ok(())
    }
}
