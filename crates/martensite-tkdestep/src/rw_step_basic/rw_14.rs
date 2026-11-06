#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepBasicRwLengthUnit, "RwStepBasicRwLengthUnit");
impl RwStepBasicRwLengthUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicLengthUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "length_unit")? {
            return Ok(());
        }
        rw_block_0201!(a_dimensions, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicLengthUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.dimensions())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicLengthUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwLocalTime, "RwStepBasicRwLocalTime");
impl RwStepBasicRwLocalTime {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicLocalTime>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "local_time")? {
            return Ok(());
        }
        let mut a_hour_component: i32 = Default::default();
        data.read_integer(num, 1, "hour_component", ach, &mut a_hour_component)?;
        let mut a_minute_component: i32 = Default::default();
        let mut has_aminute_component: bool = true;
        if data.is_param_defined(num, 2)? {
            data.read_integer(num, 2, "minute_component", ach, &mut a_minute_component)?;
        } else {
            has_aminute_component = false;
            a_minute_component = 0;
        }
        let mut a_second_component: f64 = Default::default();
        let mut has_asecond_component: bool = true;
        if data.is_param_defined(num, 3)? {
            data.read_real(num, 3, "second_component", ach, &mut a_second_component)?;
        } else {
            has_asecond_component = false;
            a_second_component = 0.0;
        }
        let mut a_zone: Handle<StepBasicCoordinatedUniversalTimeOffset> = Default::default();
        data.read_entity(
            num,
            4,
            "zone",
            ach,
            StepBasicCoordinatedUniversalTimeOffset::static_type(),
            &mut a_zone,
        )?;
        ent.init(
            a_hour_component,
            has_aminute_component,
            a_minute_component,
            has_asecond_component,
            a_second_component,
            &a_zone,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicLocalTime>,
    ) -> Result<(), Failure> {
        sw.send_integer(ent.hour_component())?;
        let has_aminute_component: bool = ent.has_minute_component();
        if has_aminute_component {
            sw.send_integer(ent.minute_component())?;
        } else {
            sw.send_undef()?;
        }
        let has_asecond_component: bool = ent.has_second_component();
        if has_asecond_component {
            sw.send_real(ent.second_component())?;
        } else {
            sw.send_undef()?;
        }
        sw.send_entity(&ent.zone())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicLocalTime>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.zone().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwMassMeasureWithUnit,
    "RwStepBasicRwMassMeasureWithUnit"
);
impl RwStepBasicRwMassMeasureWithUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicMassMeasureWithUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "Mass_measure_with_unit")? {
            return Ok(());
        }
        rw_block_0111!(a_unit_component, ach, data, ent, mvc, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicMassMeasureWithUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.value_component_member())?;
        sw.send_entity(&ent.unit_component().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicMassMeasureWithUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.unit_component().value());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwMassUnit, "RwStepBasicRwMassUnit");
impl RwStepBasicRwMassUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicMassUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "mass_unit")? {
            return Ok(());
        }
        rw_block_0202!(a_named_unit_dimensions, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicMassUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.dimensions())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicMassUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.dimensions().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwMeasureWithUnit, "RwStepBasicRwMeasureWithUnit");
impl RwStepBasicRwMeasureWithUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicMeasureWithUnit>,
    ) -> Result<(), Failure> {
        rw_block_0103!(a_unit_component, ach, data, mvc, num);
        ent.init(&mvc, &a_unit_component);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicMeasureWithUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.value_component_member())?;
        sw.send_entity(&ent.unit_component().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicMeasureWithUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.unit_component().value());
        Ok(())
    }
}
