#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwSiUnitAndPlaneAngleUnit,
    "RwStepBasicRwSiUnitAndPlaneAngleUnit"
);
impl RwStepBasicRwSiUnitAndPlaneAngleUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSiUnitAndPlaneAngleUnit>,
    ) -> Result<(), Failure> {
        rw_block_0160!(ach, data, num, num0);
        if !data.check_nb_params(num, 0, ach, "plane_angle_unit")? {
            return Ok(());
        }
        num = data.next_for_complex(num);
        rw_block_0063!(a_name, a_prefix, ach, data, ent, has_aprefix, num, text);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSiUnitAndPlaneAngleUnit>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "NAMED_UNIT")?;
        sw.send_derived()?;
        step_start_entity(sw, "PLANE_ANGLE_UNIT")?;
        rw_block_0145!(ent, has_aprefix, sw);
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwSiUnitAndRatioUnit,
    "RwStepBasicRwSiUnitAndRatioUnit"
);
impl RwStepBasicRwSiUnitAndRatioUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSiUnitAndRatioUnit>,
    ) -> Result<(), Failure> {
        rw_block_0160!(ach, data, num, num0);
        if !data.check_nb_params(num, 0, ach, "ratio_unit")? {
            return Ok(());
        }
        rw_block_0064!(a_name, a_prefix, ach, data, has_aprefix, num, text);
        ent.init(has_aprefix, a_prefix, a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSiUnitAndRatioUnit>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "RATIO_UNIT")?;
        rw_block_0093!(ent, has_aprefix, sw);
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwSiUnitAndSolidAngleUnit,
    "RwStepBasicRwSiUnitAndSolidAngleUnit"
);
impl RwStepBasicRwSiUnitAndSolidAngleUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSiUnitAndSolidAngleUnit>,
    ) -> Result<(), Failure> {
        rw_block_0024!(a_name, a_prefix, ach, data, has_aprefix, num, num0, text);
        if !data.check_nb_params(num, 0, ach, "solid_angle_unit")? {
            return Ok(());
        }
        ent.init(has_aprefix, a_prefix, a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSiUnitAndSolidAngleUnit>,
    ) -> Result<(), Failure> {
        rw_block_0093!(ent, has_aprefix, sw);
        step_start_entity(sw, "SOLID_ANGLE_UNIT")?;
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwSiUnitAndThermodynamicTemperatureUnit,
    "RwStepBasicRwSiUnitAndThermodynamicTemperatureUnit"
);
impl RwStepBasicRwSiUnitAndThermodynamicTemperatureUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSiUnitAndThermodynamicTemperatureUnit>,
    ) -> Result<(), Failure> {
        rw_block_0024!(a_name, a_prefix, ach, data, has_aprefix, num, num0, text);
        if !data.check_nb_params(num, 0, ach, "thermodynamic_temperature_unit")? {
            return Ok(());
        }
        ent.init(has_aprefix, a_prefix, a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSiUnitAndThermodynamicTemperatureUnit>,
    ) -> Result<(), Failure> {
        rw_block_0093!(ent, has_aprefix, sw);
        step_start_entity(sw, "THERMODYNAMIC_TEMPERATURE_UNIT")?;
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwSiUnitAndTimeUnit,
    "RwStepBasicRwSiUnitAndTimeUnit"
);
impl RwStepBasicRwSiUnitAndTimeUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSiUnitAndTimeUnit>,
    ) -> Result<(), Failure> {
        rw_block_0024!(a_name, a_prefix, ach, data, has_aprefix, num, num0, text);
        if !data.check_nb_params(num, 0, ach, "time_unit")? {
            return Ok(());
        }
        ent.init(has_aprefix, a_prefix, a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSiUnitAndTimeUnit>,
    ) -> Result<(), Failure> {
        rw_block_0093!(ent, has_aprefix, sw);
        step_start_entity(sw, "TIME_UNIT")?;
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwSiUnitAndVolumeUnit,
    "RwStepBasicRwSiUnitAndVolumeUnit"
);
impl RwStepBasicRwSiUnitAndVolumeUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSiUnitAndVolumeUnit>,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        rw_block_0022!(
            a_dimensions,
            a_name,
            a_prefix,
            ach,
            data,
            has_aprefix,
            num,
            num0,
            text
        );
        data.named_for_complex_short("VOLUME_UNIT", "VLMUNT", num0, &mut num, ach);
        if !data.check_nb_params(num, 0, ach, "volume_unit")? {
            return Ok(());
        }
        ent.init(has_aprefix, a_prefix, a_name);
        ent.set_dimensions(&a_dimensions);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSiUnitAndVolumeUnit>,
    ) -> Result<(), Failure> {
        rw_block_0091!(ent, has_aprefix, sw);
        step_start_entity(sw, "VOLUME_UNIT")?;
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwSolidAngleMeasureWithUnit,
    "RwStepBasicRwSolidAngleMeasureWithUnit"
);
impl RwStepBasicRwSolidAngleMeasureWithUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSolidAngleMeasureWithUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "solid_angle_measure_with_unit")? {
            return Ok(());
        }
        rw_block_0111!(a_unit_component, ach, data, ent, mvc, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSolidAngleMeasureWithUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.value_component_member())?;
        sw.send_entity(&ent.unit_component().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicSolidAngleMeasureWithUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.unit_component().value());
        Ok(())
    }
}
