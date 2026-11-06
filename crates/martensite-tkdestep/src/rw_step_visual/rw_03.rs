#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwBackgroundColour,
    "RwStepVisualRwBackgroundColour"
);
impl RwStepVisualRwBackgroundColour {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualBackgroundColour>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "background_colour")? {
            return Ok(());
        }
        let mut a_presentation: StepVisualAreaOrView = Default::default();
        data.read_entity_select(num, 1, "presentation", ach, &mut a_presentation)?;
        ent.init(&a_presentation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualBackgroundColour>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.presentation().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualBackgroundColour>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.presentation().value());
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwCameraImage, "RwStepVisualRwCameraImage");
impl RwStepVisualRwCameraImage {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCameraImage>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "camera_image")? {
            return Ok(());
        }
        rw_block_b_0074!(
            a_mapping_source,
            a_mapping_target,
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
        ent: &Handle<StepVisualCameraImage>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.mapping_source())?;
        sw.send_entity(&ent.mapping_target())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCameraImage>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.mapping_source().cast());
        iter.get_one_item(&ent.mapping_target().cast());
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwCameraModel, "RwStepVisualRwCameraModel");
impl RwStepVisualRwCameraModel {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCameraModel>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "camera_model has not 1 parameter(s)")? {
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
        ent: &Handle<StepVisualCameraModel>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwCameraModelD2, "RwStepVisualRwCameraModelD2");
impl RwStepVisualRwCameraModelD2 {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCameraModelD2>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "camera_model_d2")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_view_window: Handle<StepVisualPlanarBox> = Default::default();
        data.read_entity(
            num,
            2,
            "view_window",
            ach,
            StepVisualPlanarBox::static_type(),
            &mut a_view_window,
        )?;
        let mut a_view_window_clipping: bool = Default::default();
        data.read_boolean(
            num,
            3,
            "view_window_clipping",
            ach,
            &mut a_view_window_clipping,
        )?;
        ent.init(&a_name, &a_view_window, a_view_window_clipping);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCameraModelD2>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.view_window())?;
        sw.send_boolean(ent.view_window_clipping())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCameraModelD2>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.view_window().cast());
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwCameraModelD3, "RwStepVisualRwCameraModelD3");
impl RwStepVisualRwCameraModelD3 {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCameraModelD3>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "camera_model_d3")? {
            return Ok(());
        }
        rw_block_b_0087!(
            a_name,
            a_perspective_of_volume,
            a_view_reference_system,
            ach,
            data,
            num
        );
        ent.init(&a_name, &a_view_reference_system, &a_perspective_of_volume);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCameraModelD3>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.view_reference_system())?;
        sw.send_entity(&ent.perspective_of_volume())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCameraModelD3>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.view_reference_system().cast());
        iter.get_one_item(&ent.perspective_of_volume().cast());
        Ok(())
    }
}
