#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwCurveStyleFontPattern,
    "RwStepVisualRwCurveStyleFontPattern"
);
impl RwStepVisualRwCurveStyleFontPattern {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCurveStyleFontPattern>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "curve_style_font_pattern")? {
            return Ok(());
        }
        let mut a_visible_segment_length: f64 = Default::default();
        data.read_real(
            num,
            1,
            "visible_segment_length",
            ach,
            &mut a_visible_segment_length,
        )?;
        let mut a_invisible_segment_length: f64 = Default::default();
        data.read_real(
            num,
            2,
            "invisible_segment_length",
            ach,
            &mut a_invisible_segment_length,
        )?;
        ent.init(a_visible_segment_length, a_invisible_segment_length);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCurveStyleFontPattern>,
    ) -> Result<(), Failure> {
        sw.send_real(ent.visible_segment_length())?;
        sw.send_real(ent.invisible_segment_length())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwDraughtingCallout,
    "RwStepVisualRwDraughtingCallout"
);
impl RwStepVisualRwDraughtingCallout {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualDraughtingCallout>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "draughting_callout")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_contents: Handle<HArray1<StepVisualDraughtingCalloutElement>> =
            Default::default();
        let mut an_ent: StepVisualDraughtingCalloutElement = Default::default();
        let mut nb_sub: i32 = Default::default();
        if data.read_sub_list(num, 2, "contents", ach, &mut nb_sub, false, 0, 0)? {
            let nb_elements: i32 = data.nb_params(nb_sub)?;
            a_contents = step_new_array1::<StepVisualDraughtingCalloutElement>(1, nb_elements)?;
            for i in 1..=nb_elements {
                if data.read_entity_select(nb_sub, i, "content", ach, &mut an_ent)? {
                    step_array1_set(&a_contents, i, an_ent.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_contents);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualDraughtingCallout>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i in 1..=ent.nb_contents() {
            sw.send_entity(&ent.contents_value(i)?.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualDraughtingCallout>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let i: i32 = Default::default();
        let nb: i32 = ent.nb_contents();
        for i in 1..=nb {
            iter.add_item(&ent.contents_value(i)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwDraughtingModel,
    "RwStepVisualRwDraughtingModel"
);
impl RwStepVisualRwDraughtingModel {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualDraughtingModel>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "draughting_model")? {
            return Ok(());
        }
        rw_block_b_0154!(
            a_representation_items,
            a_representation_name,
            ach,
            data,
            num,
            sub2
        );
        if data.read_sub_list(num, 2, "representation.items", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            if nb0 > 0 {
                a_representation_items =
                    step_new_array1::<Handle<StepReprRepresentationItem>>(1, nb0)?;
                for i0 in 1..=nb0 {
                    let mut an_it0: Handle<StepReprRepresentationItem> = Default::default();
                    data.read_entity(
                        num2,
                        i0,
                        "representation.items",
                        ach,
                        StepReprRepresentationItem::static_type(),
                        &mut an_it0,
                    )?;
                    step_array1_set(&a_representation_items, i0, an_it0.clone(), "ReadStep")?;
                }
            }
        }
        let mut a_representation_context_of_items: Handle<StepReprRepresentationContext> =
            Default::default();
        data.read_entity(
            num,
            3,
            "representation.context_of_items",
            ach,
            StepReprRepresentationContext::static_type(),
            &mut a_representation_context_of_items,
        )?;
        ent.init(
            &a_representation_name,
            &a_representation_items,
            &a_representation_context_of_items,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualDraughtingModel>,
    ) -> Result<(), Failure> {
        rw_block_b_0107!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualDraughtingModel>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0207!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwDraughtingPreDefinedColour,
    "RwStepVisualRwDraughtingPreDefinedColour"
);
impl RwStepVisualRwDraughtingPreDefinedColour {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualDraughtingPreDefinedColour>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            1,
            ach,
            "draughting_pre_defined_colour has not 1 parameter(s)",
        )? {
            return Ok(());
        }
        rw_block_b_0187!(a_name, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualDraughtingPreDefinedColour>,
    ) -> Result<(), Failure> {
        sw.send_entity(&step_live(&ent.get_pre_defined_item(), "WriteStep")?.name())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwDraughtingPreDefinedCurveFont,
    "RwStepVisualRwDraughtingPreDefinedCurveFont"
);
impl RwStepVisualRwDraughtingPreDefinedCurveFont {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualDraughtingPreDefinedCurveFont>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            1,
            ach,
            "draughting_pre_defined_curve_font has not 1 parameter(s)",
        )? {
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
        ent: &Handle<StepVisualDraughtingPreDefinedCurveFont>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}
