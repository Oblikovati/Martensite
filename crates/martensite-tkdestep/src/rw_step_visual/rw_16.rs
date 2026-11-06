#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwSurfaceSideStyle,
    "RwStepVisualRwSurfaceSideStyle"
);
impl RwStepVisualRwSurfaceSideStyle {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceSideStyle>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "surface_side_style")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_styles: Handle<HArray1<StepVisualSurfaceStyleElementSelect>> = Default::default();
        let mut a_styles_item: StepVisualSurfaceStyleElementSelect = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "styles", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_styles = step_new_array1::<StepVisualSurfaceStyleElementSelect>(1, nb2)?;
            for i2 in 1..=nb2 {
                if data.read_entity_select(nsub2, i2, "styles", ach, &mut a_styles_item)? {
                    step_array1_set(&a_styles, i2, a_styles_item.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_styles);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceSideStyle>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_styles()? {
            sw.send_entity(&ent.styles_value(i2)?.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualSurfaceSideStyle>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_styles()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.styles_value(is1)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwSurfaceStyleBoundary,
    "RwStepVisualRwSurfaceStyleBoundary"
);
impl RwStepVisualRwSurfaceStyleBoundary {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceStyleBoundary>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "surface_style_boundary")? {
            return Ok(());
        }
        let mut a_style_of_boundary: Handle<StepVisualCurveStyle> = Default::default();
        data.read_entity(
            num,
            1,
            "style_of_boundary",
            ach,
            StepVisualCurveStyle::static_type(),
            &mut a_style_of_boundary,
        )?;
        ent.init(&a_style_of_boundary);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceStyleBoundary>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.style_of_boundary())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualSurfaceStyleBoundary>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.style_of_boundary().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwSurfaceStyleControlGrid,
    "RwStepVisualRwSurfaceStyleControlGrid"
);
impl RwStepVisualRwSurfaceStyleControlGrid {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceStyleControlGrid>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "surface_style_control_grid")? {
            return Ok(());
        }
        let mut a_style_of_control_grid: Handle<StepVisualCurveStyle> = Default::default();
        data.read_entity(
            num,
            1,
            "style_of_control_grid",
            ach,
            StepVisualCurveStyle::static_type(),
            &mut a_style_of_control_grid,
        )?;
        ent.init(&a_style_of_control_grid);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceStyleControlGrid>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.style_of_control_grid())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualSurfaceStyleControlGrid>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.style_of_control_grid().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwSurfaceStyleFillArea,
    "RwStepVisualRwSurfaceStyleFillArea"
);
impl RwStepVisualRwSurfaceStyleFillArea {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceStyleFillArea>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "surface_style_fill_area")? {
            return Ok(());
        }
        let mut a_fill_area: Handle<StepVisualFillAreaStyle> = Default::default();
        data.read_entity(
            num,
            1,
            "fill_area",
            ach,
            StepVisualFillAreaStyle::static_type(),
            &mut a_fill_area,
        )?;
        ent.init(&a_fill_area);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceStyleFillArea>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.fill_area())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualSurfaceStyleFillArea>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.fill_area().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwSurfaceStyleParameterLine,
    "RwStepVisualRwSurfaceStyleParameterLine"
);
impl RwStepVisualRwSurfaceStyleParameterLine {
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceStyleParameterLine>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.style_of_parameter_lines())?;
        sw.send_entity(&ent.direction_counts())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualSurfaceStyleParameterLine>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.style_of_parameter_lines().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwSurfaceStyleReflectanceAmbient,
    "RwStepVisualRwSurfaceStyleReflectanceAmbient"
);
impl RwStepVisualRwSurfaceStyleReflectanceAmbient {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceStyleReflectanceAmbient>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "surface_style_reflectance_ambient")? {
            return Ok(());
        }
        let mut a_ambient_reflectance: f64 = Default::default();
        data.read_real(
            num,
            1,
            "ambient_reflectance",
            ach,
            &mut a_ambient_reflectance,
        )?;
        ent.init(a_ambient_reflectance);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceStyleReflectanceAmbient>,
    ) -> Result<(), Failure> {
        sw.send_real(ent.ambient_reflectance())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepVisualSurfaceStyleReflectanceAmbient>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
