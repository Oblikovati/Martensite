#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepBasicRwEffectivity, "RwStepBasicRwEffectivity");
impl RwStepBasicRwEffectivity {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicEffectivity>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "effectivity")? {
            return Ok(());
        }
        let mut a_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "id", ach, &mut a_id)?;
        ent.init(&a_id);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicEffectivity>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.id())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepBasicEffectivity>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwEffectivityAssignment,
    "RwStepBasicRwEffectivityAssignment"
);
impl RwStepBasicRwEffectivityAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicEffectivityAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "effectivity_assignment")? {
            return Ok(());
        }
        let mut a_assigned_effectivity: Handle<StepBasicEffectivity> = Default::default();
        data.read_entity(
            num,
            1,
            "assigned_effectivity",
            ach,
            StepBasicEffectivity::static_type(),
            &mut a_assigned_effectivity,
        )?;
        ent.init(&a_assigned_effectivity);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicEffectivityAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assigned_effectivity())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicEffectivityAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.assigned_effectivity().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwEulerAngles, "RwStepBasicRwEulerAngles");
impl RwStepBasicRwEulerAngles {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicEulerAngles>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "euler_angles")? {
            return Ok(());
        }
        let mut a_angles: Handle<HArray1<f64>> = Default::default();
        let mut sub1: i32 = 0;
        if data.read_sub_list(num, 1, "angles", ach, &mut sub1, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub1)?;
            a_angles = step_new_array1::<f64>(1, nb0)?;
            let num2: i32 = sub1;
            for i0 in 1..=nb0 {
                let mut an_it0: f64 = Default::default();
                data.read_real(num2, i0, "real", ach, &mut an_it0)?;
                step_array1_set(&a_angles, i0, an_it0, "ReadStep")?;
            }
        }
        ent.init(&a_angles);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicEulerAngles>,
    ) -> Result<(), Failure> {
        sw.open_sub()?;
        for i0 in 1..=step_live(&ent.angles(), "WriteStep")?.array1().length() {
            let var0: f64 = step_array1_item(&ent.angles(), i0, "WriteStep")?;
            sw.send_real(var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepBasicEulerAngles>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwExternalIdentificationAssignment,
    "RwStepBasicRwExternalIdentificationAssignment"
);
impl RwStepBasicRwExternalIdentificationAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicExternalIdentificationAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "external_identification_assignment")? {
            return Ok(());
        }
        let mut a_identification_assignment_assigned_id: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "identification_assignment.assigned_id",
            ach,
            &mut a_identification_assignment_assigned_id,
        )?;
        let mut a_identification_assignment_role: Handle<StepBasicIdentificationRole> =
            Default::default();
        data.read_entity(
            num,
            2,
            "identification_assignment.role",
            ach,
            StepBasicIdentificationRole::static_type(),
            &mut a_identification_assignment_role,
        )?;
        let mut a_source: Handle<StepBasicExternalSource> = Default::default();
        data.read_entity(
            num,
            3,
            "source",
            ach,
            StepBasicExternalSource::static_type(),
            &mut a_source,
        )?;
        ent.init(
            &a_identification_assignment_assigned_id,
            &a_identification_assignment_role,
            &a_source,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicExternalIdentificationAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.assigned_id())?;
        sw.send_entity(&ent.parent.role())?;
        sw.send_entity(&ent.source())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicExternalIdentificationAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.role().cast());
        iter.add_item(&ent.source().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwExternalSource, "RwStepBasicRwExternalSource");
impl RwStepBasicRwExternalSource {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicExternalSource>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "external_source")? {
            return Ok(());
        }
        let mut a_source_id: StepBasicSourceItem = Default::default();
        data.read_entity_select(num, 1, "source_id", ach, &mut a_source_id)?;
        ent.init(&a_source_id);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicExternalSource>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.source_id().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicExternalSource>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.source_id().value());
        Ok(())
    }
}
