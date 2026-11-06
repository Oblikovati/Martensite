#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwSurfaceStyleRenderingWithProperties,
    "RwStepVisualRwSurfaceStyleRenderingWithProperties"
);
impl RwStepVisualRwSurfaceStyleRenderingWithProperties {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceStyleRenderingWithProperties>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "surface_style_rendering_with_properties")? {
            return Ok(());
        }
        let mut a_surface_style_rendering_rendering_method: StepVisualShadingSurfaceMethod =
            StepVisualShadingSurfaceMethod::SsmNormalShading;
        if data.param_type(num, 1)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 1)?;
            if text == b".CONSTANT_SHADING." {
                a_surface_style_rendering_rendering_method =
                    StepVisualShadingSurfaceMethod::SsmConstantShading;
            } else {
                if text == b".COLOUR_SHADING." {
                    a_surface_style_rendering_rendering_method =
                        StepVisualShadingSurfaceMethod::SsmColourShading;
                } else {
                    if text == b".DOT_SHADING." {
                        a_surface_style_rendering_rendering_method =
                            StepVisualShadingSurfaceMethod::SsmDotShading;
                    } else {
                        if text == b".NORMAL_SHADING." {
                            a_surface_style_rendering_rendering_method =
                                StepVisualShadingSurfaceMethod::SsmNormalShading;
                        } else {
                            ach.add_fail_text("Parameter #1 (surface_style_rendering.rendering_method) has not allowed value", "");
                        }
                    }
                }
            }
        } else {
            ach.add_fail_text(
                "Parameter #1 (surface_style_rendering.rendering_method) is not enumeration",
                "",
            );
        }
        let mut a_surface_style_rendering_surface_colour: Handle<StepVisualColour> =
            Default::default();
        data.read_entity(
            num,
            2,
            "surface_style_rendering.surface_colour",
            ach,
            StepVisualColour::static_type(),
            &mut a_surface_style_rendering_surface_colour,
        )?;
        let mut a_properties: Handle<HArray1<StepVisualRenderingPropertiesSelect>> =
            Default::default();
        let mut sub3: i32 = 0;
        if data.read_sub_list(num, 3, "properties", ach, &mut sub3, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub3)?;
            a_properties = step_new_array1::<StepVisualRenderingPropertiesSelect>(1, nb0)?;
            let num2: i32 = sub3;
            for i0 in 1..=nb0 {
                let mut an_it0: StepVisualRenderingPropertiesSelect = Default::default();
                data.read_entity_select(num2, i0, "rendering_properties_select", ach, &mut an_it0)?;
                step_array1_set(&a_properties, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            a_surface_style_rendering_rendering_method,
            &a_surface_style_rendering_surface_colour,
            &a_properties,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceStyleRenderingWithProperties>,
    ) -> Result<(), Failure> {
        rw_block_b_0203!(ent, sw);
        sw.open_sub()?;
        for i2 in 1..=step_live(&ent.properties(), "WriteStep")?.array1().length() {
            let var0: StepVisualRenderingPropertiesSelect =
                step_array1_item(&ent.properties(), i2, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualSurfaceStyleRenderingWithProperties>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.surface_colour().cast());
        for i2 in 1..=step_live(&ent.properties(), "Share")?.array1().length() {
            let var0: StepVisualRenderingPropertiesSelect =
                step_array1_item(&ent.properties(), i2, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwSurfaceStyleSegmentationCurve,
    "RwStepVisualRwSurfaceStyleSegmentationCurve"
);
impl RwStepVisualRwSurfaceStyleSegmentationCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceStyleSegmentationCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "surface_style_segmentation_curve")? {
            return Ok(());
        }
        let mut a_style_of_segmentation_curve: Handle<StepVisualCurveStyle> = Default::default();
        data.read_entity(
            num,
            1,
            "style_of_segmentation_curve",
            ach,
            StepVisualCurveStyle::static_type(),
            &mut a_style_of_segmentation_curve,
        )?;
        ent.init(&a_style_of_segmentation_curve);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceStyleSegmentationCurve>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.style_of_segmentation_curve())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualSurfaceStyleSegmentationCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.style_of_segmentation_curve().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwSurfaceStyleSilhouette,
    "RwStepVisualRwSurfaceStyleSilhouette"
);
impl RwStepVisualRwSurfaceStyleSilhouette {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceStyleSilhouette>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "surface_style_silhouette")? {
            return Ok(());
        }
        let mut a_style_of_silhouette: Handle<StepVisualCurveStyle> = Default::default();
        data.read_entity(
            num,
            1,
            "style_of_silhouette",
            ach,
            StepVisualCurveStyle::static_type(),
            &mut a_style_of_silhouette,
        )?;
        ent.init(&a_style_of_silhouette);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceStyleSilhouette>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.style_of_silhouette())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualSurfaceStyleSilhouette>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.style_of_silhouette().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwSurfaceStyleTransparent,
    "RwStepVisualRwSurfaceStyleTransparent"
);
impl RwStepVisualRwSurfaceStyleTransparent {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceStyleTransparent>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "surface_style_transparent")? {
            return Ok(());
        }
        let mut a_transparency: f64 = Default::default();
        data.read_real(num, 1, "transparency", ach, &mut a_transparency)?;
        ent.init(a_transparency);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceStyleTransparent>,
    ) -> Result<(), Failure> {
        sw.send_real(ent.transparency())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepVisualSurfaceStyleTransparent>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
