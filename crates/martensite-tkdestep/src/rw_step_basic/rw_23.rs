#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwSecurityClassification,
    "RwStepBasicRwSecurityClassification"
);
impl RwStepBasicRwSecurityClassification {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSecurityClassification>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "security_classification")? {
            return Ok(());
        }
        rw_block_0147!(a_name, a_purpose, ach, data, num);
        let mut a_security_level: Handle<StepBasicSecurityClassificationLevel> = Default::default();
        data.read_entity(
            num,
            3,
            "security_level",
            ach,
            StepBasicSecurityClassificationLevel::static_type(),
            &mut a_security_level,
        )?;
        ent.init(&a_name, &a_purpose, &a_security_level);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSecurityClassification>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.purpose())?;
        sw.send_entity(&ent.security_level())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicSecurityClassification>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.security_level().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwSecurityClassificationLevel,
    "RwStepBasicRwSecurityClassificationLevel"
);
impl RwStepBasicRwSecurityClassificationLevel {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSecurityClassificationLevel>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "security_classification_level")? {
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
        ent: &Handle<StepBasicSecurityClassificationLevel>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwSiUnit, "RwStepBasicRwSiUnit");
impl RwStepBasicRwSiUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSiUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "si_unit")? {
            return Ok(());
        }
        data.check_derived(num, 1, "dimensions", ach, false)?;
        let mut a_prefix: StepBasicSiPrefix = StepBasicSiPrefix::SpExa;
        let mut has_aprefix: bool = false;
        if data.is_param_defined(num, 2)? {
            if data.param_type(num, 2)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = data.param_c_value(num, 2)?;
                has_aprefix = RwStepBasicRwSiPrefix::convert_to_enum(&text, &mut a_prefix);
                if !has_aprefix {
                    ach.add_fail_text("Enumeration si_prefix has not an allowed value", "");
                }
            } else {
                ach.add_fail_text("Parameter #2 (prefix) is not an enumeration", "");
            }
        }
        let mut a_name: StepBasicSiUnitName = StepBasicSiUnitName::SunMetre;
        if data.param_type(num, 3)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 3)?;
            if !RwStepBasicRwSiUnitName::convert_to_enum(&text, &mut a_name) {
                ach.add_fail_text("Enumeration si_unit_name has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #3 (name) is not an enumeration", "");
        }
        ent.init(has_aprefix, a_prefix, a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSiUnit>,
    ) -> Result<(), Failure> {
        sw.send_derived()?;
        rw_block_0174!(ent, has_aprefix, sw);
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwSiUnitAndAreaUnit,
    "RwStepBasicRwSiUnitAndAreaUnit"
);
impl RwStepBasicRwSiUnitAndAreaUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSiUnitAndAreaUnit>,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        data.named_for_complex_short("AREA_UNIT", "ARUNT", num0, &mut num, ach);
        if !data.check_nb_params(num, 0, ach, "area_unit")? {
            return Ok(());
        }
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
        ent.init(has_aprefix, a_prefix, a_name);
        ent.set_dimensions(&a_dimensions);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSiUnitAndAreaUnit>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "AREA_UNIT")?;
        rw_block_0091!(ent, has_aprefix, sw);
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwSiUnitAndLengthUnit,
    "RwStepBasicRwSiUnitAndLengthUnit"
);
impl RwStepBasicRwSiUnitAndLengthUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSiUnitAndLengthUnit>,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        let mut sorted: bool =
            data.named_for_complex_short("LENGTH_UNIT", "LNGUNT", num0, &mut num, ach);
        if !data.check_nb_params(num, 0, ach, "length_unit")? {
            return Ok(());
        }
        rw_block_0017!(
            a_name,
            a_prefix,
            ach,
            data,
            ent,
            has_aprefix,
            num,
            num0,
            sorted,
            text
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSiUnitAndLengthUnit>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "LENGTH_UNIT")?;
        rw_block_0093!(ent, has_aprefix, sw);
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwSiUnitAndMassUnit,
    "RwStepBasicRwSiUnitAndMassUnit"
);
impl RwStepBasicRwSiUnitAndMassUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicSiUnitAndMassUnit>,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        let mut sorted: bool =
            data.named_for_complex_short("MASS_UNIT", "MSSUNT", num0, &mut num, ach);
        if !data.check_nb_params(num, 0, ach, "mass_unit")? {
            return Ok(());
        }
        rw_block_0017!(
            a_name,
            a_prefix,
            ach,
            data,
            ent,
            has_aprefix,
            num,
            num0,
            sorted,
            text
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicSiUnitAndMassUnit>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "MASS_UNIT")?;
        rw_block_0093!(ent, has_aprefix, sw);
        Ok(())
    }
}
