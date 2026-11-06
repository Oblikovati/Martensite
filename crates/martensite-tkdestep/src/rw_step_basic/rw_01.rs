#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_enum_text! {
    RwStepBasicRwAheadOrBehind, StepBasicAheadOrBehind, "StepBasicAheadOrBehind", {
        AobAhead => ".AHEAD.",
        AobExact => ".EXACT.",
        AobBehind => ".BEHIND.",
    }
}

step_enum_text! {
    RwStepBasicRwSiPrefix, StepBasicSiPrefix, "StepBasicSiPrefix", {
        SpExa => ".EXA.",
        SpPico => ".PICO.",
        SpMega => ".MEGA.",
        SpFemto => ".FEMTO.",
        SpAtto => ".ATTO.",
        SpCenti => ".CENTI.",
        SpNano => ".NANO.",
        SpHecto => ".HECTO.",
        SpMicro => ".MICRO.",
        SpTera => ".TERA.",
        SpGiga => ".GIGA.",
        SpMilli => ".MILLI.",
        SpPeta => ".PETA.",
        SpDeci => ".DECI.",
        SpKilo => ".KILO.",
        SpDeca => ".DECA.",
    }
}

step_enum_text! {
    RwStepBasicRwSiUnitName, StepBasicSiUnitName, "StepBasicSiUnitName", {
        SunHertz => ".HERTZ.",
        SunDegreeCelsius => ".DEGREE_CELSIUS.",
        SunSiemens => ".SIEMENS.",
        SunSievert => ".SIEVERT.",
        SunLux => ".LUX.",
        SunWatt => ".WATT.",
        SunOhm => ".OHM.",
        SunSecond => ".SECOND.",
        SunBecquerel => ".BECQUEREL.",
        SunPascal => ".PASCAL.",
        SunHenry => ".HENRY.",
        SunTesla => ".TESLA.",
        SunVolt => ".VOLT.",
        SunJoule => ".JOULE.",
        SunKelvin => ".KELVIN.",
        SunAmpere => ".AMPERE.",
        SunGram => ".GRAM.",
        SunSteradian => ".STERADIAN.",
        SunMole => ".MOLE.",
        SunLumen => ".LUMEN.",
        SunGray => ".GRAY.",
        SunCandela => ".CANDELA.",
        SunFarad => ".FARAD.",
        SunRadian => ".RADIAN.",
        SunNewton => ".NEWTON.",
        SunMetre => ".METRE.",
        SunWeber => ".WEBER.",
        SunCoulomb => ".COULOMB.",
    }
}

step_enum_text! {
    RwStepBasicRwSource, StepBasicSource, "StepBasicSource", {
        SBought => ".BOUGHT.",
        SNotKnown => ".NOT_KNOWN.",
        SMade => ".MADE.",
    }
}

step_rw_class!(RwStepBasicRwAction, "RwStepBasicRwAction");
impl RwStepBasicRwAction {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicAction>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "action")? {
            return Ok(());
        }
        rw_block_0102!(a_description, a_name, ach, data, has_description, num);
        let mut a_chosen_method: Handle<StepBasicActionMethod> = Default::default();
        data.read_entity(
            num,
            3,
            "chosen_method",
            ach,
            StepBasicActionMethod::static_type(),
            &mut a_chosen_method,
        )?;
        ent.init(&a_name, has_description, &a_description, &a_chosen_method);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicAction>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        if ent.has_description() {
            sw.send_entity(&ent.description())?;
        } else {
            sw.send_undef()?;
        }
        sw.send_entity(&ent.chosen_method())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicAction>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.chosen_method().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwActionAssignment,
    "RwStepBasicRwActionAssignment"
);
impl RwStepBasicRwActionAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicActionAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "action_assignment")? {
            return Ok(());
        }
        let mut a_assigned_action: Handle<StepBasicAction> = Default::default();
        data.read_entity(
            num,
            1,
            "assigned_action",
            ach,
            StepBasicAction::static_type(),
            &mut a_assigned_action,
        )?;
        ent.init(&a_assigned_action);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicActionAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assigned_action())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicActionAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.assigned_action().cast());
        Ok(())
    }
}
