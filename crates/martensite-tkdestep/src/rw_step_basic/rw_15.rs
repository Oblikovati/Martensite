#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwMechanicalContext,
    "RwStepBasicRwMechanicalContext"
);
impl RwStepBasicRwMechanicalContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicMechanicalContext>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "mechanical_context")? {
            return Ok(());
        }
        rw_block_0069!(
            a_discipline_type,
            a_frame_of_reference,
            a_name,
            ach,
            data,
            ent,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicMechanicalContext>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.frame_of_reference())?;
        sw.send_entity(&ent.discipline_type())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicMechanicalContext>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.frame_of_reference().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwNameAssignment, "RwStepBasicRwNameAssignment");
impl RwStepBasicRwNameAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicNameAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "name_assignment")? {
            return Ok(());
        }
        let mut a_assigned_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "assigned_name", ach, &mut a_assigned_name)?;
        ent.init(&a_assigned_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicNameAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assigned_name())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepBasicNameAssignment>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwNamedUnit, "RwStepBasicRwNamedUnit");
impl RwStepBasicRwNamedUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicNamedUnit>,
    ) -> Result<(), Failure> {
        rw_block_0172!(a_dimensions, ach, data, num);
        ent.init(&a_dimensions);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicNamedUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.dimensions())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicNamedUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwObjectRole, "RwStepBasicRwObjectRole");
impl RwStepBasicRwObjectRole {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicObjectRole>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "object_role")? {
            return Ok(());
        }
        rw_block_0082!(a_description, a_name, ach, data, ent, has_description, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicObjectRole>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        if ent.has_description() {
            sw.send_entity(&ent.description())?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepBasicObjectRole>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwOrdinalDate, "RwStepBasicRwOrdinalDate");
impl RwStepBasicRwOrdinalDate {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicOrdinalDate>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "ordinal_date")? {
            return Ok(());
        }
        rw_block_0157!(a_day_component, a_year_component, ach, data, num);
        ent.init(a_year_component, a_day_component);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicOrdinalDate>,
    ) -> Result<(), Failure> {
        sw.send_integer(ent.year_component())?;
        sw.send_integer(ent.day_component())?;
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwOrganization, "RwStepBasicRwOrganization");
impl RwStepBasicRwOrganization {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicOrganization>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "organization")? {
            return Ok(());
        }
        let mut a_id: Handle<HAsciiString> = Default::default();
        let mut has_aid: bool = true;
        if data.is_param_defined(num, 1)? {
            data.read_string(num, 1, "id", ach, &mut a_id)?;
        } else {
            has_aid = false;
            a_id = Default::default();
        }
        rw_block_0194!(a_description, a_name, ach, data, num);
        if data.is_param_defined(num, 3)? {
            data.read_string(num, 3, "description", ach, &mut a_description)?;
        }
        ent.init(has_aid, &a_id, &a_name, &a_description);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicOrganization>,
    ) -> Result<(), Failure> {
        let has_aid: bool = ent.has_id();
        if has_aid {
            sw.send_entity(&ent.id())?;
        } else {
            sw.send_undef()?;
        }
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        Ok(())
    }
}
