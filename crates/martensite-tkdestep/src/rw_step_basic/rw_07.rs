#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwConversionBasedUnitAndSolidAngleUnit,
    "RwStepBasicRwConversionBasedUnitAndSolidAngleUnit"
);
impl RwStepBasicRwConversionBasedUnitAndSolidAngleUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicConversionBasedUnitAndSolidAngleUnit>,
    ) -> Result<(), Failure> {
        rw_block_0026!(
            a_conversion_factor,
            a_dimensions,
            a_name,
            ach,
            data,
            num,
            num0
        );
        if !data.check_nb_params(num, 0, ach, "solid_angle_unit")? {
            return Ok(());
        }
        ent.init(&a_dimensions, &a_name, &a_conversion_factor);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicConversionBasedUnitAndSolidAngleUnit>,
    ) -> Result<(), Failure> {
        rw_block_0121!(ent, sw);
        step_start_entity(sw, "SOLID_ANGLE_UNIT")?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicConversionBasedUnitAndSolidAngleUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        iter.get_one_item(&ent.conversion_factor());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwConversionBasedUnitAndTimeUnit,
    "RwStepBasicRwConversionBasedUnitAndTimeUnit"
);
impl RwStepBasicRwConversionBasedUnitAndTimeUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicConversionBasedUnitAndTimeUnit>,
    ) -> Result<(), Failure> {
        rw_block_0026!(
            a_conversion_factor,
            a_dimensions,
            a_name,
            ach,
            data,
            num,
            num0
        );
        if !data.check_nb_params(num, 0, ach, "time_unit")? {
            return Ok(());
        }
        ent.init(&a_dimensions, &a_name, &a_conversion_factor);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicConversionBasedUnitAndTimeUnit>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "CONVERSION_BASED_UNIT")?;
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.conversion_factor())?;
        step_start_entity(sw, "TIME_UNIT")?;
        step_start_entity(sw, "NAMED_UNIT")?;
        sw.send_entity(&ent.dimensions())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicConversionBasedUnitAndTimeUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        iter.get_one_item(&ent.conversion_factor());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwConversionBasedUnitAndVolumeUnit,
    "RwStepBasicRwConversionBasedUnitAndVolumeUnit"
);
impl RwStepBasicRwConversionBasedUnitAndVolumeUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicConversionBasedUnitAndVolumeUnit>,
    ) -> Result<(), Failure> {
        rw_block_0034!(
            a_conversion_factor,
            a_dimensions,
            a_name,
            ach,
            data,
            num,
            num0
        );
        data.named_for_complex_short("VOLUME_UNIT", "VLMUNT", num0, &mut num, ach);
        if !data.check_nb_params(num, 0, ach, "volume_unit")? {
            return Ok(());
        }
        ent.init(&a_dimensions, &a_name, &a_conversion_factor);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicConversionBasedUnitAndVolumeUnit>,
    ) -> Result<(), Failure> {
        rw_block_0121!(ent, sw);
        step_start_entity(sw, "VOLUME_UNIT")?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicConversionBasedUnitAndVolumeUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        iter.get_one_item(&ent.conversion_factor());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwCoordinatedUniversalTimeOffset,
    "RwStepBasicRwCoordinatedUniversalTimeOffset"
);
impl RwStepBasicRwCoordinatedUniversalTimeOffset {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicCoordinatedUniversalTimeOffset>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "coordinated_universal_time_offset")? {
            return Ok(());
        }
        let mut a_hour_offset: i32 = Default::default();
        data.read_integer(num, 1, "hour_offset", ach, &mut a_hour_offset)?;
        let mut a_minute_offset: i32 = Default::default();
        let mut has_aminute_offset: bool = true;
        if data.is_param_defined(num, 2)? {
            data.read_integer(num, 2, "minute_offset", ach, &mut a_minute_offset)?;
        } else {
            has_aminute_offset = false;
            a_minute_offset = 0;
        }
        let mut a_sense: StepBasicAheadOrBehind = StepBasicAheadOrBehind::AobAhead;
        if data.param_type(num, 3)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 3)?;
            if !RwStepBasicRwAheadOrBehind::convert_to_enum(&text, &mut a_sense) {
                ach.add_fail_text("Enumeration ahead_or_behind has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #3 (sense) is not an enumeration", "");
        }
        ent.init(a_hour_offset, has_aminute_offset, a_minute_offset, a_sense);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicCoordinatedUniversalTimeOffset>,
    ) -> Result<(), Failure> {
        sw.send_integer(ent.hour_offset())?;
        let has_aminute_offset: bool = ent.has_minute_offset();
        if has_aminute_offset {
            sw.send_integer(ent.minute_offset())?;
        } else {
            sw.send_undef()?;
        }
        sw.send_enum(RwStepBasicRwAheadOrBehind::convert_to_string(ent.sense()))?;
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwDate, "RwStepBasicRwDate");
impl RwStepBasicRwDate {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDate>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "date")? {
            return Ok(());
        }
        let mut a_year_component: i32 = Default::default();
        data.read_integer(num, 1, "year_component", ach, &mut a_year_component)?;
        ent.init(a_year_component);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDate>,
    ) -> Result<(), Failure> {
        sw.send_integer(ent.year_component())?;
        Ok(())
    }
}
