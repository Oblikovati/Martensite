#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwPresentationRepresentation,
    "RwStepVisualRwPresentationRepresentation"
);
impl RwStepVisualRwPresentationRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPresentationRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "presentation_representation")? {
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
        ent: &Handle<StepVisualPresentationRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_b_0120!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualPresentationRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0188!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwPresentationSet,
    "RwStepVisualRwPresentationSet"
);
impl RwStepVisualRwPresentationSet {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        _unused_3: &Handle<StepVisualPresentationSet>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 0, ach, "presentation_set")? {
            return Ok(());
        }
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        _unused_0: &mut StepDataStepWriter,
        _unused_1: &Handle<StepVisualPresentationSet>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwPresentationSize,
    "RwStepVisualRwPresentationSize"
);
impl RwStepVisualRwPresentationSize {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPresentationSize>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "presentation_size")? {
            return Ok(());
        }
        let mut a_unit: StepVisualPresentationSizeAssignmentSelect = Default::default();
        data.read_entity_select(num, 1, "unit", ach, &mut a_unit)?;
        let mut a_size: Handle<StepVisualPlanarBox> = Default::default();
        data.read_entity(
            num,
            2,
            "size",
            ach,
            StepVisualPlanarBox::static_type(),
            &mut a_size,
        )?;
        ent.init(&a_unit, &a_size);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualPresentationSize>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.unit().value())?;
        sw.send_entity(&ent.size())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualPresentationSize>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.unit().value());
        iter.get_one_item(&ent.size().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwPresentationStyleAssignment,
    "RwStepVisualRwPresentationStyleAssignment"
);
impl RwStepVisualRwPresentationStyleAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPresentationStyleAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "presentation_style_assignment")? {
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
                    let a_null_style: Handle<StepVisualNullStyleMember> =
                        Handle::new(StepVisualNullStyleMember::new());
                    if !a_member.is_null() {
                        let an_enum_text: Vec<u8> = step_live(&a_member, "ReadStep")?
                            .enum_text()
                            .as_bytes()
                            .to_vec();
                        a_null_style.set_enum_text(0, &an_enum_text);
                    }
                    a_styles_item.set_value(&a_null_style.cast())?;
                }
                step_array1_set(&a_styles, i1, a_styles_item.clone(), "ReadStep")?;
            }
        }
        ent.init(&a_styles);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualPresentationStyleAssignment>,
    ) -> Result<(), Failure> {
        rw_block_b_0172!(a_style, ent, i1, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualPresentationStyleAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_styles()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.styles_value(is1)?.value());
        }
        Ok(())
    }
}
