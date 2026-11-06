#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwExternallyDefinedCurveFont,
    "RwStepVisualRwExternallyDefinedCurveFont"
);
impl RwStepVisualRwExternallyDefinedCurveFont {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualExternallyDefinedCurveFont>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "externally_defined_curve_font")? {
            return Ok(());
        }
        let mut a_externally_defined_item_item_id: StepBasicSourceItem = Default::default();
        data.read_entity_select(
            num,
            1,
            "externally_defined_item.item_id",
            ach,
            &mut a_externally_defined_item_item_id,
        )?;
        let mut a_externally_defined_item_source: Handle<StepBasicExternalSource> =
            Default::default();
        data.read_entity(
            num,
            2,
            "externally_defined_item.source",
            ach,
            StepBasicExternalSource::static_type(),
            &mut a_externally_defined_item_source,
        )?;
        ent.init(
            &a_externally_defined_item_item_id,
            &a_externally_defined_item_source,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualExternallyDefinedCurveFont>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.item_id().value())?;
        sw.send_entity(&ent.parent.source())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualExternallyDefinedCurveFont>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.item_id().value());
        iter.add_item(&ent.parent.source().cast());
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwFillAreaStyle, "RwStepVisualRwFillAreaStyle");
impl RwStepVisualRwFillAreaStyle {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualFillAreaStyle>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fill_area_style")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_fill_styles: Handle<HArray1<StepVisualFillStyleSelect>> = Default::default();
        let mut a_fill_styles_item: StepVisualFillStyleSelect = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "fill_styles", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_fill_styles = step_new_array1::<StepVisualFillStyleSelect>(1, nb2)?;
            for i2 in 1..=nb2 {
                if data.read_entity_select(
                    nsub2,
                    i2,
                    "fill_styles",
                    ach,
                    &mut a_fill_styles_item,
                )? {
                    step_array1_set(&a_fill_styles, i2, a_fill_styles_item.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_fill_styles);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualFillAreaStyle>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_fill_styles() {
            sw.send_entity(&ent.fill_styles_value(i2)?.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualFillAreaStyle>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_fill_styles();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.fill_styles_value(is1)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwFillAreaStyleColour,
    "RwStepVisualRwFillAreaStyleColour"
);
impl RwStepVisualRwFillAreaStyleColour {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualFillAreaStyleColour>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fill_area_style_colour")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_fill_colour: Handle<StepVisualColour> = Default::default();
        data.read_entity(
            num,
            2,
            "fill_colour",
            ach,
            StepVisualColour::static_type(),
            &mut a_fill_colour,
        )?;
        ent.init(&a_name, &a_fill_colour);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualFillAreaStyleColour>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.fill_colour())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualFillAreaStyleColour>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.fill_colour().cast());
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwInvisibility, "RwStepVisualRwInvisibility");
impl RwStepVisualRwInvisibility {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualInvisibility>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "invisibility")? {
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
        ent.init(&a_invisible_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualInvisibility>,
    ) -> Result<(), Failure> {
        rw_block_b_0194!(ent, i1, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualInvisibility>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_invisible_items()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.invisible_items_value(is1)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwMechanicalDesignGeometricPresentationArea,
    "RwStepVisualRwMechanicalDesignGeometricPresentationArea"
);
impl RwStepVisualRwMechanicalDesignGeometricPresentationArea {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualMechanicalDesignGeometricPresentationArea>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "mechanical_design_geometric_presentation_area")? {
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
        ent: &Handle<StepVisualMechanicalDesignGeometricPresentationArea>,
    ) -> Result<(), Failure> {
        rw_block_b_0120!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualMechanicalDesignGeometricPresentationArea>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0188!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}
