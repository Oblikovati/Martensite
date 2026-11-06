#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwPreDefinedColour,
    "RwStepVisualRwPreDefinedColour"
);
impl RwStepVisualRwPreDefinedColour {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPreDefinedColour>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "pre_defined_colour")? {
            return Ok(());
        }
        rw_block_b_0187!(a_name, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualPreDefinedColour>,
    ) -> Result<(), Failure> {
        sw.send_entity(&step_live(&ent.get_pre_defined_item(), "WriteStep")?.name())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwPreDefinedCurveFont,
    "RwStepVisualRwPreDefinedCurveFont"
);
impl RwStepVisualRwPreDefinedCurveFont {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPreDefinedCurveFont>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "pre_defined_curve_font")? {
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
        ent: &Handle<StepVisualPreDefinedCurveFont>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwPreDefinedItem, "RwStepVisualRwPreDefinedItem");
impl RwStepVisualRwPreDefinedItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPreDefinedItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "pre_defined_item")? {
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
        ent: &Handle<StepVisualPreDefinedItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwPresentationArea,
    "RwStepVisualRwPresentationArea"
);
impl RwStepVisualRwPresentationArea {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPresentationArea>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "presentation_area")? {
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
        ent: &Handle<StepVisualPresentationArea>,
    ) -> Result<(), Failure> {
        rw_block_b_0120!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualPresentationArea>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0188!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwPresentationLayerAssignment,
    "RwStepVisualRwPresentationLayerAssignment"
);
impl RwStepVisualRwPresentationLayerAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPresentationLayerAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "presentation_layer_assignment")? {
            return Ok(());
        }
        rw_block_b_0147!(a_description, a_name, ach, data, num);
        let mut a_assigned_items: Handle<HArray1<StepVisualLayeredItem>> = Default::default();
        let mut a_assigned_items_item: StepVisualLayeredItem = Default::default();
        let mut nsub3: i32 = Default::default();
        if data.read_sub_list(num, 3, "assigned_items", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            if nb3 > 0 {
                a_assigned_items = step_new_array1::<StepVisualLayeredItem>(1, nb3)?;
                for i3 in 1..=nb3 {
                    if data.read_entity_select(
                        nsub3,
                        i3,
                        "assigned_items",
                        ach,
                        &mut a_assigned_items_item,
                    )? {
                        step_array1_set(
                            &a_assigned_items,
                            i3,
                            a_assigned_items_item.clone(),
                            "ReadStep",
                        )?;
                    }
                }
            }
        }
        ent.init(&a_name, &a_description, &a_assigned_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualPresentationLayerAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        sw.open_sub()?;
        for i3 in 1..=ent.nb_assigned_items() {
            sw.send_entity(&ent.assigned_items_value(i3)?.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualPresentationLayerAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_assigned_items();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.assigned_items_value(is1)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwPresentationLayerUsage,
    "RwStepVisualRwPresentationLayerUsage"
);
impl RwStepVisualRwPresentationLayerUsage {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPresentationLayerUsage>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "presentation_layer_usage")? {
            return Ok(());
        }
        let mut pla: Handle<StepVisualPresentationLayerAssignment> = Default::default();
        let mut pr: Handle<StepVisualPresentationRepresentation> = Default::default();
        data.read_entity(
            num,
            1,
            "assignment",
            ach,
            StepVisualPresentationLayerAssignment::static_type(),
            &mut pla,
        )?;
        data.read_entity(
            num,
            2,
            "presentation",
            ach,
            StepVisualPresentationRepresentation::static_type(),
            &mut pr,
        )?;
        ent.init(&pla, &pr);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualPresentationLayerUsage>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assignment())?;
        sw.send_entity(&ent.presentation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualPresentationLayerUsage>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.assignment().cast());
        iter.add_item(&ent.presentation().cast());
        Ok(())
    }
}
