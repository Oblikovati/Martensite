#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepBasicRwActionMethod, "RwStepBasicRwActionMethod");
impl RwStepBasicRwActionMethod {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicActionMethod>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "action_method")? {
            return Ok(());
        }
        rw_block_0102!(a_description, a_name, ach, data, has_description, num);
        let mut a_consequence: Handle<HAsciiString> = Default::default();
        data.read_string(num, 3, "consequence", ach, &mut a_consequence)?;
        let mut a_purpose: Handle<HAsciiString> = Default::default();
        data.read_string(num, 4, "purpose", ach, &mut a_purpose)?;
        ent.init(
            &a_name,
            has_description,
            &a_description,
            &a_consequence,
            &a_purpose,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicActionMethod>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        if ent.has_description() {
            sw.send_entity(&ent.description())?;
        } else {
            sw.send_undef()?;
        }
        sw.send_entity(&ent.consequence())?;
        sw.send_entity(&ent.purpose())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepBasicActionMethod>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwActionRequestAssignment,
    "RwStepBasicRwActionRequestAssignment"
);
impl RwStepBasicRwActionRequestAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicActionRequestAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "action_request_assignment")? {
            return Ok(());
        }
        let mut a_assigned_action_request: Handle<StepBasicVersionedActionRequest> =
            Default::default();
        data.read_entity(
            num,
            1,
            "assigned_action_request",
            ach,
            StepBasicVersionedActionRequest::static_type(),
            &mut a_assigned_action_request,
        )?;
        ent.init(&a_assigned_action_request);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicActionRequestAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assigned_action_request())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicActionRequestAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.assigned_action_request().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwActionRequestSolution,
    "RwStepBasicRwActionRequestSolution"
);
impl RwStepBasicRwActionRequestSolution {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicActionRequestSolution>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "action_request_solution")? {
            return Ok(());
        }
        let mut a_method: Handle<StepBasicActionMethod> = Default::default();
        data.read_entity(
            num,
            1,
            "method",
            ach,
            StepBasicActionMethod::static_type(),
            &mut a_method,
        )?;
        let mut a_request: Handle<StepBasicVersionedActionRequest> = Default::default();
        data.read_entity(
            num,
            2,
            "request",
            ach,
            StepBasicVersionedActionRequest::static_type(),
            &mut a_request,
        )?;
        ent.init(&a_method, &a_request);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicActionRequestSolution>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.method())?;
        sw.send_entity(&ent.request())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicActionRequestSolution>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.method().cast());
        iter.add_item(&ent.request().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwAddress, "RwStepBasicRwAddress");
impl RwStepBasicRwAddress {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicAddress>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 12, ach, "address")? {
            return Ok(());
        }
        rw_block_0000!(
            a_country,
            a_electronic_mail_address,
            a_facsimile_number,
            a_internal_location,
            a_postal_box,
            a_postal_code,
            a_region,
            a_street,
            a_street_number,
            a_telephone_number,
            a_telex_number,
            a_town,
            ach,
            data,
            has_acountry,
            has_aelectronic_mail_address,
            has_afacsimile_number,
            has_ainternal_location,
            has_apostal_box,
            has_apostal_code,
            has_aregion,
            has_astreet,
            has_astreet_number,
            has_atelephone_number,
            has_atelex_number,
            has_atown,
            num
        );
        ent.init(
            has_ainternal_location,
            &a_internal_location,
            has_astreet_number,
            &a_street_number,
            has_astreet,
            &a_street,
            has_apostal_box,
            &a_postal_box,
            has_atown,
            &a_town,
            has_aregion,
            &a_region,
            has_apostal_code,
            &a_postal_code,
            has_acountry,
            &a_country,
            has_afacsimile_number,
            &a_facsimile_number,
            has_atelephone_number,
            &a_telephone_number,
            has_aelectronic_mail_address,
            &a_electronic_mail_address,
            has_atelex_number,
            &a_telex_number,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicAddress>,
    ) -> Result<(), Failure> {
        rw_block_0004!(
            ent,
            has_acountry,
            has_aelectronic_mail_address,
            has_afacsimile_number,
            has_ainternal_location,
            has_apostal_box,
            has_apostal_code,
            has_aregion,
            has_astreet,
            has_astreet_number,
            has_atelephone_number,
            has_atelex_number,
            has_atown,
            sw
        );
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwApplicationContext,
    "RwStepBasicRwApplicationContext"
);
impl RwStepBasicRwApplicationContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicApplicationContext>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "application_context")? {
            return Ok(());
        }
        let mut a_application: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "application", ach, &mut a_application)?;
        ent.init(&a_application);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicApplicationContext>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.application())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwApplicationContextElement,
    "RwStepBasicRwApplicationContextElement"
);
impl RwStepBasicRwApplicationContextElement {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicApplicationContextElement>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "application_context_element")? {
            return Ok(());
        }
        rw_block_0129!(a_frame_of_reference, a_name, ach, data, num);
        ent.init(&a_name, &a_frame_of_reference);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicApplicationContextElement>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.frame_of_reference())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicApplicationContextElement>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.frame_of_reference().cast());
        Ok(())
    }
}
