#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwSurfaceStyleUsage,
    "RwStepVisualRwSurfaceStyleUsage"
);
impl RwStepVisualRwSurfaceStyleUsage {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceStyleUsage>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "surface_style_usage")? {
            return Ok(());
        }
        let mut a_side: StepVisualSurfaceSide = StepVisualSurfaceSide::SsNegative;
        if data.param_type(num, 1)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 1)?;
            if !RwStepVisualRwSurfaceSide::convert_to_enum(&text, &mut a_side) {
                ach.add_fail_text("Enumeration surface_side has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #1 (side) is not an enumeration", "");
        }
        let mut a_style: Handle<StepVisualSurfaceSideStyle> = Default::default();
        data.read_entity(
            num,
            2,
            "style",
            ach,
            StepVisualSurfaceSideStyle::static_type(),
            &mut a_style,
        )?;
        ent.init(a_side, &a_style);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualSurfaceStyleUsage>,
    ) -> Result<(), Failure> {
        sw.send_enum(RwStepVisualRwSurfaceSide::convert_to_string(ent.side()))?;
        sw.send_entity(&ent.style())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualSurfaceStyleUsage>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.style().cast());
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwTemplate, "RwStepVisualRwTemplate");
impl RwStepVisualRwTemplate {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualTemplate>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "template")? {
            return Ok(());
        }
        rw_block_b_0044!(
            a_context_of_items,
            a_items,
            a_name,
            ach,
            anent2,
            data,
            ent,
            i2,
            nb2,
            nsub2,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualTemplate>,
    ) -> Result<(), Failure> {
        rw_block_b_0120!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualTemplate>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0188!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTemplateInstance,
    "RwStepVisualRwTemplateInstance"
);
impl RwStepVisualRwTemplateInstance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualTemplateInstance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "template_instance")? {
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
        ent: &Handle<StepVisualTemplateInstance>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.mapping_source())?;
        sw.send_entity(&ent.mapping_target())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualTemplateInstance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.mapping_source().cast());
        iter.get_one_item(&ent.mapping_target().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTessellatedAnnotationOccurrence,
    "RwStepVisualRwTessellatedAnnotationOccurrence"
);
impl RwStepVisualRwTessellatedAnnotationOccurrence {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualTessellatedAnnotationOccurrence>,
    ) -> Result<(), Failure> {
        rw_block_b_0040!(
            a_item, a_name, a_styles, ach, anent2, data, ent, i2, nb2, nsub2, num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualTessellatedAnnotationOccurrence>,
    ) -> Result<(), Failure> {
        rw_block_b_0119!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualTessellatedAnnotationOccurrence>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0186!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}
