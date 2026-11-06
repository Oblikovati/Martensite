#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwSurfaceStyleReflectanceAmbientDiffuse,
    "RwStepVisualRwSurfaceStyleReflectanceAmbientDiffuse"
);
impl RwStepVisualRwSurfaceStyleReflectanceAmbientDiffuse {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_ach: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualSurfaceStyleReflectanceAmbientDiffuse>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            2,
            the_ach,
            "surface_style_reflectance_ambient_diffuse",
        )? {
            return Ok(());
        }
        rw_block_b_0156!(
            a_ambient_reflectance,
            a_diffuse_reflectance,
            the_ach,
            the_data,
            the_num
        );
        the_ent.init(a_ambient_reflectance, a_diffuse_reflectance);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualSurfaceStyleReflectanceAmbientDiffuse>,
    ) -> Result<(), Failure> {
        the_sw.send_real(the_ent.ambient_reflectance())?;
        the_sw.send_real(the_ent.diffuse_reflectance())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepVisualSurfaceStyleReflectanceAmbientDiffuse>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwSurfaceStyleReflectanceAmbientDiffuseSpecular,
    "RwStepVisualRwSurfaceStyleReflectanceAmbientDiffuseSpecular"
);
impl RwStepVisualRwSurfaceStyleReflectanceAmbientDiffuseSpecular {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_ach: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            5,
            the_ach,
            "surface_style_reflectance_ambient_diffuse_specular",
        )? {
            return Ok(());
        }
        rw_block_b_0156!(
            a_ambient_reflectance,
            a_diffuse_reflectance,
            the_ach,
            the_data,
            the_num
        );
        let mut a_specular_reflectance: f64 = Default::default();
        the_data.read_real(
            the_num,
            3,
            "specular_reflectance",
            the_ach,
            &mut a_specular_reflectance,
        )?;
        let mut a_specular_exponent: f64 = Default::default();
        the_data.read_real(
            the_num,
            4,
            "specular_exponent",
            the_ach,
            &mut a_specular_exponent,
        )?;
        let mut a_specular_colour: Handle<StepVisualColour> = Default::default();
        the_data.read_entity(
            the_num,
            5,
            "specular_colour",
            the_ach,
            StepVisualColour::static_type(),
            &mut a_specular_colour,
        )?;
        the_ent.init(
            a_ambient_reflectance,
            a_diffuse_reflectance,
            a_specular_reflectance,
            a_specular_exponent,
            &a_specular_colour,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular>,
    ) -> Result<(), Failure> {
        the_sw.send_real(the_ent.ambient_reflectance())?;
        the_sw.send_real(the_ent.diffuse_reflectance())?;
        the_sw.send_real(the_ent.specular_reflectance())?;
        the_sw.send_real(the_ent.specular_exponent())?;
        the_sw.send_entity(&the_ent.specular_colour())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        the_iter.add_item(&the_ent.specular_colour().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwSurfaceStyleRendering,
    "RwStepVisualRwSurfaceStyleRendering"
);
impl RwStepVisualRwSurfaceStyleRendering {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceStyleRendering>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "surface_style_rendering")? {
            return Ok(());
        }
        let mut a_rendering_method: StepVisualShadingSurfaceMethod =
            StepVisualShadingSurfaceMethod::SsmNormalShading;
        if data.param_type(num, 1)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 1)?;
            if text == b".CONSTANT_SHADING." {
                a_rendering_method = StepVisualShadingSurfaceMethod::SsmConstantShading;
            } else {
                if text == b".COLOUR_SHADING." {
                    a_rendering_method = StepVisualShadingSurfaceMethod::SsmColourShading;
                } else {
                    if text == b".DOT_SHADING." {
                        a_rendering_method = StepVisualShadingSurfaceMethod::SsmDotShading;
                    } else {
                        if text == b".NORMAL_SHADING." {
                            a_rendering_method = StepVisualShadingSurfaceMethod::SsmNormalShading;
                        } else {
                            ach.add_fail_text(
                                "Parameter #1 (rendering_method) has not allowed value",
                                "",
                            );
                        }
                    }
                }
            }
        } else {
            ach.add_fail_text("Parameter #1 (rendering_method) is not enumeration", "");
        }
        let mut a_surface_colour: Handle<StepVisualColour> = Default::default();
        data.read_entity(
            num,
            2,
            "surface_colour",
            ach,
            StepVisualColour::static_type(),
            &mut a_surface_colour,
        )?;
        ent.init(a_rendering_method, &a_surface_colour);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceStyleRendering>,
    ) -> Result<(), Failure> {
        rw_block_b_0203!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualSurfaceStyleRendering>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.surface_colour().cast());
        Ok(())
    }
}
