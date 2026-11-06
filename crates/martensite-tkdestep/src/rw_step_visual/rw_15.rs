#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwPresentationStyleByContext,
    "RwStepVisualRwPresentationStyleByContext"
);
impl RwStepVisualRwPresentationStyleByContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPresentationStyleByContext>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "presentation_style_by_context")? {
            return Ok(());
        }
        rw_block_b_0198!(a_styles, a_styles_item, nsub1);
        if data.read_sub_list(num, 1, "styles", ach, &mut nsub1, false, 0, 0)? {
            let nb1: i32 = data.nb_params(nsub1)?;
            a_styles = step_new_array1::<StepVisualPresentationStyleSelect>(1, nb1)?;
            for i1 in 1..=nb1 {
                let a_type: InterfaceParamType = data.param_type(nsub1, i1)?;
                if a_type == InterfaceParamType::ParamIdent {
                    data.read_entity_select(nsub1, i1, "styles", ach, &mut a_styles_item)?;
                } else {
                    let mut a_member: Handle<dyn StepDataSelectMember> = Default::default();
                    data.read_member_typed(nsub1, i1, "null_style", ach, &mut a_member)?;
                    let an_enum_text: Vec<u8> = step_live(&a_member, "ReadStep")?
                        .enum_text()
                        .as_bytes()
                        .to_vec();
                    let a_null_style: Handle<StepVisualNullStyleMember> =
                        Handle::new(StepVisualNullStyleMember::new());
                    a_null_style.set_enum_text(0, &an_enum_text);
                    a_styles_item.set_value(&a_null_style.cast())?;
                }
                step_array1_set(&a_styles, i1, a_styles_item.clone(), "ReadStep")?;
            }
        }
        let mut a_style_context: StepVisualStyleContextSelect = Default::default();
        data.read_entity_select(num, 2, "style_context", ach, &mut a_style_context)?;
        ent.init(&a_styles, &a_style_context);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualPresentationStyleByContext>,
    ) -> Result<(), Failure> {
        rw_block_b_0172!(a_style, ent, i1, sw);
        sw.send_entity(&ent.style_context().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualPresentationStyleByContext>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_styles()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.styles_value(is1)?.value());
        }
        iter.get_one_item(&ent.style_context().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwPresentationView,
    "RwStepVisualRwPresentationView"
);
impl RwStepVisualRwPresentationView {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPresentationView>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "presentation_view has not 3 parameter(s)")? {
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
        ent: &Handle<StepVisualPresentationView>,
    ) -> Result<(), Failure> {
        rw_block_b_0120!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualPresentationView>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0188!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwPresentedItemRepresentation,
    "RwStepVisualRwPresentedItemRepresentation"
);
impl RwStepVisualRwPresentedItemRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPresentedItemRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "presented_item_representation")? {
            return Ok(());
        }
        let mut prs: StepVisualPresentationRepresentationSelect = Default::default();
        data.read_entity_select(num, 1, "date_time", ach, &mut prs)?;
        let mut pi: Handle<StepVisualPresentedItem> = Default::default();
        data.read_entity(
            num,
            2,
            "dated_approval",
            ach,
            StepVisualPresentedItem::static_type(),
            &mut pi,
        )?;
        ent.init(&prs, &pi);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualPresentedItemRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.presentation().value())?;
        sw.send_entity(&ent.item())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualPresentedItemRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.presentation().value());
        iter.get_one_item(&ent.item().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwRepositionedTessellatedGeometricSet,
    "RwStepVisualRwRepositionedTessellatedGeometricSet"
);

step_rw_class!(
    RwStepVisualRwRepositionedTessellatedItem,
    "RwStepVisualRwRepositionedTessellatedItem"
);
impl RwStepVisualRwRepositionedTessellatedItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_ach: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualRepositionedTessellatedItem>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 2, the_ach, "tessellated_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(the_num, 1, "name", the_ach, &mut a_name)?;
        let mut a_location: Handle<StepGeomAxis2Placement3d> = Default::default();
        the_data.read_entity(
            the_num,
            2,
            "location",
            the_ach,
            StepGeomAxis2Placement3d::static_type(),
            &mut a_location,
        )?;
        the_ent.init(&a_name, &a_location);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualRepositionedTessellatedItem>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.location())?;
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwStyledItem, "RwStepVisualRwStyledItem");
impl RwStepVisualRwStyledItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualStyledItem>,
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
        ent: &Handle<StepVisualStyledItem>,
    ) -> Result<(), Failure> {
        rw_block_b_0119!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualStyledItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0186!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}
