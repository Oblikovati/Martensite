#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwConversionBasedUnitAndAreaUnit,
    "RwStepBasicRwConversionBasedUnitAndAreaUnit"
);
impl RwStepBasicRwConversionBasedUnitAndAreaUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicConversionBasedUnitAndAreaUnit>,
    ) -> Result<(), Failure> {
        let mut num: i32 = num0;
        if !data.check_nb_params(num, 0, ach, "area_unit")? {
            return Ok(());
        }
        num = data.next_for_complex(num);
        rw_block_0044!(a_conversion_factor, a_dimensions, a_name, ach, data, num);
        ent.init(&a_dimensions, &a_name, &a_conversion_factor);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicConversionBasedUnitAndAreaUnit>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "AREA_UNIT")?;
        rw_block_0121!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicConversionBasedUnitAndAreaUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        iter.get_one_item(&ent.conversion_factor());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwConversionBasedUnitAndLengthUnit,
    "RwStepBasicRwConversionBasedUnitAndLengthUnit"
);
impl RwStepBasicRwConversionBasedUnitAndLengthUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicConversionBasedUnitAndLengthUnit>,
    ) -> Result<(), Failure> {
        rw_block_0073!(a_conversion_factor, a_name, ach, data, num, num0);
        if !data.check_nb_params(num, 0, ach, "length_unit")? {
            return Ok(());
        }
        rw_block_0106!(
            a_conversion_factor,
            a_dimensions,
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
        ent: &Handle<StepBasicConversionBasedUnitAndLengthUnit>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "CONVERSION_BASED_UNIT")?;
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.conversion_factor())?;
        step_start_entity(sw, "LENGTH_UNIT")?;
        step_start_entity(sw, "NAMED_UNIT")?;
        sw.send_entity(&ent.dimensions())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicConversionBasedUnitAndLengthUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        iter.get_one_item(&ent.conversion_factor());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwConversionBasedUnitAndMassUnit,
    "RwStepBasicRwConversionBasedUnitAndMassUnit"
);
impl RwStepBasicRwConversionBasedUnitAndMassUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicConversionBasedUnitAndMassUnit>,
    ) -> Result<(), Failure> {
        rw_block_0073!(a_conversion_factor, a_name, ach, data, num, num0);
        if !data.check_nb_params(num, 0, ach, "Mass_unit")? {
            return Ok(());
        }
        rw_block_0106!(
            a_conversion_factor,
            a_dimensions,
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
        ent: &Handle<StepBasicConversionBasedUnitAndMassUnit>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "CONVERSION_BASED_UNIT")?;
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.conversion_factor())?;
        step_start_entity(sw, "Mass_UNIT")?;
        step_start_entity(sw, "NAMED_UNIT")?;
        sw.send_entity(&ent.dimensions())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicConversionBasedUnitAndMassUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        iter.get_one_item(&ent.conversion_factor());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwConversionBasedUnitAndPlaneAngleUnit,
    "RwStepBasicRwConversionBasedUnitAndPlaneAngleUnit"
);
impl RwStepBasicRwConversionBasedUnitAndPlaneAngleUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicConversionBasedUnitAndPlaneAngleUnit>,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        data.named_for_complex_short("CONVERSION_BASED_UNIT", "CNBSUN", num0, &mut num, ach);
        rw_block_0100!(a_conversion_factor, a_name, ach, data, num);
        rw_block_0123!(a_dimensions, ach, data, num, num0);
        data.named_for_complex_short("PLANE_ANGLE_UNIT", "PLANUN", num0, &mut num, ach);
        if !data.check_nb_params(num, 0, ach, "plane_angle_unit")? {
            return Ok(());
        }
        ent.init(&a_dimensions, &a_name, &a_conversion_factor);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicConversionBasedUnitAndPlaneAngleUnit>,
    ) -> Result<(), Failure> {
        rw_block_0121!(ent, sw);
        step_start_entity(sw, "PLANE_ANGLE_UNIT")?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicConversionBasedUnitAndPlaneAngleUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        iter.get_one_item(&ent.conversion_factor());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwConversionBasedUnitAndRatioUnit,
    "RwStepBasicRwConversionBasedUnitAndRatioUnit"
);
impl RwStepBasicRwConversionBasedUnitAndRatioUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicConversionBasedUnitAndRatioUnit>,
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
        if !data.check_nb_params(num, 0, ach, "ratio_unit")? {
            return Ok(());
        }
        ent.init(&a_dimensions, &a_name, &a_conversion_factor);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicConversionBasedUnitAndRatioUnit>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "CONVERSION_BASED_UNIT")?;
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.conversion_factor())?;
        step_start_entity(sw, "RATIO_UNIT")?;
        step_start_entity(sw, "NAMED_UNIT")?;
        sw.send_entity(&ent.dimensions())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicConversionBasedUnitAndRatioUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        iter.get_one_item(&ent.conversion_factor());
        Ok(())
    }
}
