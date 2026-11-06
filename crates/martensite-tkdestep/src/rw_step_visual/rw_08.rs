#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepVisualRwCompositeText, "RwStepVisualRwCompositeText");
impl RwStepVisualRwCompositeText {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCompositeText>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "composite_text has not 2 parameter(s)")? {
            return Ok(());
        }
        rw_block_b_0069!(
            a_collected_text,
            a_collected_text_item,
            a_name,
            ach,
            data,
            i2,
            nb2,
            nsub2,
            num
        );
        ent.init(&a_name, &a_collected_text);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCompositeText>,
    ) -> Result<(), Failure> {
        rw_block_b_0157!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCompositeText>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_collected_text()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.collected_text_value(is1)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwCompositeTextWithExtent,
    "RwStepVisualRwCompositeTextWithExtent"
);
impl RwStepVisualRwCompositeTextWithExtent {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCompositeTextWithExtent>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            3,
            ach,
            "composite_text_with_extent has not 3 parameter(s)",
        )? {
            return Ok(());
        }
        rw_block_b_0069!(
            a_collected_text,
            a_collected_text_item,
            a_name,
            ach,
            data,
            i2,
            nb2,
            nsub2,
            num
        );
        let mut a_extent: Handle<StepVisualPlanarExtent> = Default::default();
        data.read_entity(
            num,
            3,
            "extent",
            ach,
            StepVisualPlanarExtent::static_type(),
            &mut a_extent,
        )?;
        ent.init(&a_name, &a_collected_text, &a_extent);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCompositeTextWithExtent>,
    ) -> Result<(), Failure> {
        rw_block_b_0157!(ent, i2, sw);
        sw.send_entity(&ent.extent())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCompositeTextWithExtent>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_collected_text()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.collected_text_value(is1)?.value());
        }
        iter.get_one_item(&ent.extent().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwContextDependentInvisibility,
    "RwStepVisualRwContextDependentInvisibility"
);
impl RwStepVisualRwContextDependentInvisibility {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualContextDependentInvisibility>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "context_dependent_invisibility")? {
            return Ok(());
        }
        rw_block_b_0124!(
            a_invisible_items,
            a_invisible_items_item,
            ach,
            data,
            i1,
            nb1,
            nsub1,
            num
        );
        let mut a_presentation_context: StepVisualInvisibilityContext = Default::default();
        data.read_entity_select(
            num,
            2,
            "presentation_context",
            ach,
            &mut a_presentation_context,
        )?;
        ent.init(&a_invisible_items, &a_presentation_context);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualContextDependentInvisibility>,
    ) -> Result<(), Failure> {
        rw_block_b_0194!(ent, i1, sw);
        sw.send_entity(&ent.presentation_context().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualContextDependentInvisibility>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_invisible_items()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.invisible_items_value(is1)?.value());
        }
        iter.get_one_item(&ent.presentation_context().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwContextDependentOverRidingStyledItem,
    "RwStepVisualRwContextDependentOverRidingStyledItem"
);
impl RwStepVisualRwContextDependentOverRidingStyledItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualContextDependentOverRidingStyledItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "context_dependent_over_riding_styled_item")? {
            return Ok(());
        }
        rw_block_b_0039!(
            a_item,
            a_name,
            a_over_ridden_style,
            a_styles,
            ach,
            anent2,
            data,
            i2,
            nb2,
            nsub2,
            num
        );
        let mut a_style_context: Handle<HArray1<StepVisualStyleContextSelect>> = Default::default();
        let mut a_style_context_item: StepVisualStyleContextSelect = Default::default();
        let mut nsub5: i32 = Default::default();
        if data.read_sub_list(num, 5, "style_context", ach, &mut nsub5, false, 0, 0)? {
            let nb5: i32 = data.nb_params(nsub5)?;
            a_style_context = step_new_array1::<StepVisualStyleContextSelect>(1, nb5)?;
            for i5 in 1..=nb5 {
                if data.read_entity_select(
                    nsub5,
                    i5,
                    "style_context",
                    ach,
                    &mut a_style_context_item,
                )? {
                    step_array1_set(
                        &a_style_context,
                        i5,
                        a_style_context_item.clone(),
                        "ReadStep",
                    )?;
                }
            }
        } else {
            a_style_context = Handle::new(HArray1::<StepVisualStyleContextSelect>::default());
        }
        ent.init(
            &a_name,
            &a_styles,
            &a_item,
            &a_over_ridden_style,
            &a_style_context,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualContextDependentOverRidingStyledItem>,
    ) -> Result<(), Failure> {
        rw_block_b_0098!(ent, i2, sw);
        sw.open_sub()?;
        for i5 in 1..=ent.nb_style_context()? {
            sw.send_entity(&ent.style_context_value(i5)?.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualContextDependentOverRidingStyledItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0146!(ent, is1, iter, nb_elem1);
        let nb_elem4: i32 = ent.nb_style_context()?;
        for is4 in 1..=nb_elem4 {
            iter.get_one_item(&ent.style_context_value(is4)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwCoordinatesList,
    "RwStepVisualRwCoordinatesList"
);
