#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepBasicRwDateAndTime, "RwStepBasicRwDateAndTime");
impl RwStepBasicRwDateAndTime {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDateAndTime>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "date_and_time")? {
            return Ok(());
        }
        let mut a_date_component: Handle<StepBasicDate> = Default::default();
        data.read_entity(
            num,
            1,
            "date_component",
            ach,
            StepBasicDate::static_type(),
            &mut a_date_component,
        )?;
        let mut a_time_component: Handle<StepBasicLocalTime> = Default::default();
        data.read_entity(
            num,
            2,
            "time_component",
            ach,
            StepBasicLocalTime::static_type(),
            &mut a_time_component,
        )?;
        ent.init(&a_date_component, &a_time_component);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDateAndTime>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.date_component())?;
        sw.send_entity(&ent.time_component())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicDateAndTime>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.date_component().cast());
        iter.get_one_item(&ent.time_component().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwDateRole, "RwStepBasicRwDateRole");
impl RwStepBasicRwDateRole {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDateRole>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "date_role")? {
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
        ent: &Handle<StepBasicDateRole>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwDateTimeRole, "RwStepBasicRwDateTimeRole");
impl RwStepBasicRwDateTimeRole {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDateTimeRole>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "date_time_role")? {
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
        ent: &Handle<StepBasicDateTimeRole>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwDerivedUnit, "RwStepBasicRwDerivedUnit");
impl RwStepBasicRwDerivedUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDerivedUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "derived_unit")? {
            return Ok(());
        }
        let mut elts: Handle<HArray1<Handle<StepBasicDerivedUnitElement>>> = Default::default();
        let mut anelt: Handle<StepBasicDerivedUnitElement> = Default::default();
        let mut nsub1: i32 = Default::default();
        if data.read_sub_list(num, 1, "elements", ach, &mut nsub1, false, 0, 0)? {
            let nb1: i32 = data.nb_params(nsub1)?;
            elts = step_new_array1::<Handle<StepBasicDerivedUnitElement>>(1, nb1)?;
            for i1 in 1..=nb1 {
                if data.read_entity(
                    nsub1,
                    i1,
                    "element",
                    ach,
                    StepBasicDerivedUnitElement::static_type(),
                    &mut anelt,
                )? {
                    step_array1_set(&elts, i1, anelt.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&elts);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDerivedUnit>,
    ) -> Result<(), Failure> {
        let i: i32 = Default::default();
        let nb: i32 = ent.nb_elements()?;
        sw.open_sub()?;
        for i in 1..=nb {
            sw.send_entity(&ent.elements_value(i)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicDerivedUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let i: i32 = Default::default();
        let nb: i32 = ent.nb_elements()?;
        for i in 1..=nb {
            iter.get_one_item(&ent.elements_value(i)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwDerivedUnitElement,
    "RwStepBasicRwDerivedUnitElement"
);
impl RwStepBasicRwDerivedUnitElement {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDerivedUnitElement>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "derived_unit_element")? {
            return Ok(());
        }
        let mut nu: Handle<StepBasicNamedUnit> = Default::default();
        data.read_entity(
            num,
            1,
            "unit",
            ach,
            StepBasicNamedUnit::static_type(),
            &mut nu,
        )?;
        let mut expo: f64 = Default::default();
        data.read_real(num, 2, "exponent", ach, &mut expo)?;
        ent.init(&nu, expo);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDerivedUnitElement>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.unit())?;
        sw.send_real(ent.exponent())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicDerivedUnitElement>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.unit().cast());
        Ok(())
    }
}
