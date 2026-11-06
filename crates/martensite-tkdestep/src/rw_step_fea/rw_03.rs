#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepFEARwCurveElementEndOffset,
    "RwStepFEARwCurveElementEndOffset"
);
impl RwStepFEARwCurveElementEndOffset {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEACurveElementEndOffset>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "curve_element_end_offset")? {
            return Ok(());
        }
        let mut a_coordinate_system: StepFEACurveElementEndCoordinateSystem = Default::default();
        data.read_entity_select(num, 1, "coordinate_system", ach, &mut a_coordinate_system)?;
        let mut a_offset_vector: Handle<HArray1<f64>> = Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "offset_vector", ach, &mut sub2, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub2)?;
            a_offset_vector = step_new_array1::<f64>(1, nb0)?;
            let num2: i32 = sub2;
            for i0 in 1..=nb0 {
                let mut an_it0: f64 = Default::default();
                data.read_real(num2, i0, "real", ach, &mut an_it0)?;
                step_array1_set(&a_offset_vector, i0, an_it0, "ReadStep")?;
            }
        }
        ent.init(&a_coordinate_system, &a_offset_vector);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEACurveElementEndOffset>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.coordinate_system().value())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.offset_vector(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: f64 = step_array1_item(&ent.offset_vector(), i1, "WriteStep")?;
            sw.send_real(var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEACurveElementEndOffset>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.coordinate_system().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwCurveElementEndRelease,
    "RwStepFEARwCurveElementEndRelease"
);
impl RwStepFEARwCurveElementEndRelease {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEACurveElementEndRelease>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "curve_element_end_release")? {
            return Ok(());
        }
        let mut a_coordinate_system: StepFEACurveElementEndCoordinateSystem = Default::default();
        data.read_entity_select(num, 1, "coordinate_system", ach, &mut a_coordinate_system)?;
        let mut a_releases: Handle<HArray1<Handle<StepElementCurveElementEndReleasePacket>>> =
            Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "releases", ach, &mut sub2, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub2)?;
            a_releases =
                step_new_array1::<Handle<StepElementCurveElementEndReleasePacket>>(1, nb0)?;
            let num2: i32 = sub2;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepElementCurveElementEndReleasePacket> =
                    Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "curve_element_end_release_packet",
                    ach,
                    StepElementCurveElementEndReleasePacket::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_releases, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(&a_coordinate_system, &a_releases);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEACurveElementEndRelease>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.coordinate_system().value())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.releases(), "WriteStep")?.array1().length() {
            let var0: Handle<StepElementCurveElementEndReleasePacket> =
                step_array1_item(&ent.releases(), i1, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEACurveElementEndRelease>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.coordinate_system().value());
        for i2 in 1..=step_live(&ent.releases(), "Share")?.array1().length() {
            let var0: Handle<StepElementCurveElementEndReleasePacket> =
                step_array1_item(&ent.releases(), i2, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwCurveElementInterval,
    "RwStepFEARwCurveElementInterval"
);
impl RwStepFEARwCurveElementInterval {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEACurveElementInterval>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "curve_element_interval")? {
            return Ok(());
        }
        let mut a_finish_position: Handle<StepFEACurveElementLocation> = Default::default();
        data.read_entity(
            num,
            1,
            "finish_position",
            ach,
            StepFEACurveElementLocation::static_type(),
            &mut a_finish_position,
        )?;
        let mut a_eu_angles: Handle<StepBasicEulerAngles> = Default::default();
        data.read_entity(
            num,
            2,
            "eu_angles",
            ach,
            StepBasicEulerAngles::static_type(),
            &mut a_eu_angles,
        )?;
        ent.init(&a_finish_position, &a_eu_angles);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEACurveElementInterval>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.finish_position())?;
        sw.send_entity(&ent.eu_angles())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEACurveElementInterval>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.finish_position().cast());
        iter.add_item(&ent.eu_angles().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwCurveElementIntervalConstant,
    "RwStepFEARwCurveElementIntervalConstant"
);
impl RwStepFEARwCurveElementIntervalConstant {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEACurveElementIntervalConstant>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "curve_element_interval_constant")? {
            return Ok(());
        }
        rw_block_b_0132!(
            a_curve_element_interval_eu_angles,
            a_curve_element_interval_finish_position,
            ach,
            data,
            num
        );
        let mut a_section: Handle<StepElementCurveElementSectionDefinition> = Default::default();
        data.read_entity(
            num,
            3,
            "section",
            ach,
            StepElementCurveElementSectionDefinition::static_type(),
            &mut a_section,
        )?;
        ent.init(
            &a_curve_element_interval_finish_position,
            &a_curve_element_interval_eu_angles,
            &a_section,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEACurveElementIntervalConstant>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.finish_position())?;
        sw.send_entity(&ent.parent.eu_angles())?;
        sw.send_entity(&ent.section())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEACurveElementIntervalConstant>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.finish_position().cast());
        iter.add_item(&ent.parent.eu_angles().cast());
        iter.add_item(&ent.section().cast());
        Ok(())
    }
}
