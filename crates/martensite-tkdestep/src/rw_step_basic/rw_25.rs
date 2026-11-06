#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepBasicRwSolidAngleUnit, "RwStepBasicRwSolidAngleUnit");
impl RwStepBasicRwSolidAngleUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSolidAngleUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "solid_angle_unit")? {
            return Ok(());
        }
        rw_block_0201!(a_dimensions, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSolidAngleUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.dimensions())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicSolidAngleUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwThermodynamicTemperatureUnit,
    "RwStepBasicRwThermodynamicTemperatureUnit"
);
impl RwStepBasicRwThermodynamicTemperatureUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicThermodynamicTemperatureUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "thermodynamic_temperature_unit")? {
            return Ok(());
        }
        rw_block_0202!(a_named_unit_dimensions, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicThermodynamicTemperatureUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.dimensions())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicThermodynamicTemperatureUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.dimensions().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwUncertaintyMeasureWithUnit,
    "RwStepBasicRwUncertaintyMeasureWithUnit"
);
impl RwStepBasicRwUncertaintyMeasureWithUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicUncertaintyMeasureWithUnit>,
    ) -> Result<(), Failure> {
        if data.is_complex(num) {
            ach.add_warning_text(
                "Complex Type not allowed, only suitable values are read",
                "",
            );
            let mut num1: i32 = num;
            num1 = data.next_for_complex(num1);
            if !data.check_nb_params(num1, 2, ach, "measure_with_unit")? {
                return Ok(());
            }
            let mut mvc: Handle<StepBasicMeasureValueMember> =
                Handle::new(StepBasicMeasureValueMember::new());
            data.read_member_typed(num1, 1, "value_component", ach, &mut mvc)?;
            let mut a_unit_component: StepBasicUnit = Default::default();
            data.read_entity_select(num1, 2, "unit_component", ach, &mut a_unit_component)?;
            num1 = data.next_for_complex(num1);
            if !data.check_nb_params(num1, 2, ach, "uncertainty_measure_with_unit")? {
                return Ok(());
            }
            let mut a_name: Handle<HAsciiString> = Default::default();
            data.read_string(num1, 1, "name", ach, &mut a_name)?;
            let mut a_description: Handle<HAsciiString> = Default::default();
            if data.is_param_defined(num1, 2)? {
                data.read_string(num1, 2, "description", ach, &mut a_description)?;
            }
            ent.init(&mvc, &a_unit_component, &a_name, &a_description);
            return Ok(());
        }
        if !data.check_nb_params(num, 4, ach, "uncertainty_measure_with_unit")? {
            return Ok(());
        }
        rw_block_0143!(a_unit_component, ach, data, mvc, num);
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 3, "name", ach, &mut a_name)?;
        let mut a_description: Handle<HAsciiString> = Default::default();
        data.read_string(num, 4, "description", ach, &mut a_description)?;
        ent.init(&mvc, &a_unit_component, &a_name, &a_description);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicUncertaintyMeasureWithUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.value_component_member())?;
        sw.send_entity(&ent.unit_component().value())?;
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicUncertaintyMeasureWithUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.unit_component().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwVersionedActionRequest,
    "RwStepBasicRwVersionedActionRequest"
);
impl RwStepBasicRwVersionedActionRequest {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicVersionedActionRequest>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "versioned_action_request")? {
            return Ok(());
        }
        let mut a_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "id", ach, &mut a_id)?;
        let mut a_version: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "version", ach, &mut a_version)?;
        let mut a_purpose: Handle<HAsciiString> = Default::default();
        data.read_string(num, 3, "purpose", ach, &mut a_purpose)?;
        let mut a_description: Handle<HAsciiString> = Default::default();
        let mut has_description: bool = true;
        if data.is_param_defined(num, 4)? {
            data.read_string(num, 4, "description", ach, &mut a_description)?;
        } else {
            has_description = false;
        }
        ent.init(
            &a_id,
            &a_version,
            &a_purpose,
            has_description,
            &a_description,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicVersionedActionRequest>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.id())?;
        sw.send_entity(&ent.version())?;
        sw.send_entity(&ent.purpose())?;
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
        _unused_0: &Handle<StepBasicVersionedActionRequest>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
