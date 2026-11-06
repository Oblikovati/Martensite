#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepReprRwBooleanRepresentationItem,
    "RwStepReprRwBooleanRepresentationItem"
);
impl RwStepReprRwBooleanRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_ach: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepReprBooleanRepresentationItem>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 2, the_ach, "boolean_representation_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(the_num, 1, "name", the_ach, &mut a_name)?;
        let mut a_value: bool = Default::default();
        the_data.read_boolean(the_num, 2, "value", the_ach, &mut a_value)?;
        the_ent.init(&a_name, a_value);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepReprBooleanRepresentationItem>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_integer(i32::from(the_ent.value()))?;
        Ok(())
    }
}

step_rw_class!(RwStepReprRwCentreOfSymmetry, "RwStepReprRwCentreOfSymmetry");
impl RwStepReprRwCentreOfSymmetry {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprCentreOfSymmetry>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "centre_of_symmetry")? {
            return Ok(());
        }
        rw_block_0045!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
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
        ent: &Handle<StepReprCentreOfSymmetry>,
    ) -> Result<(), Failure> {
        rw_block_0166!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprCentreOfSymmetry>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwCharacterizedRepresentation,
    "RwStepReprRwCharacterizedRepresentation"
);
impl RwStepReprRwCharacterizedRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprCharacterizedRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "characterized_representation")? {
            return Ok(());
        }
        rw_block_0149!(a_description, a_name, ach, data, num);
        let mut an_items: Handle<HArray1<Handle<StepReprRepresentationItem>>> = Default::default();
        let mut an_item: Handle<StepReprRepresentationItem> = Default::default();
        let mut nsub: i32 = Default::default();
        if data.read_sub_list(num, 3, "items", ach, &mut nsub, false, 0, 0)? {
            let nb: i32 = data.nb_params(nsub)?;
            an_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, nb)?;
            for i in 1..=nb {
                if data.read_entity(
                    nsub,
                    i,
                    "representation_item",
                    ach,
                    StepReprRepresentationItem::static_type(),
                    &mut an_item,
                )? {
                    step_array1_set(&an_items, i, an_item.clone(), "ReadStep")?;
                }
            }
        }
        let mut a_context_of_items: Handle<StepReprRepresentationContext> = Default::default();
        data.read_entity(
            num,
            4,
            "context_of_items",
            ach,
            StepReprRepresentationContext::static_type(),
            &mut a_context_of_items,
        )?;
        ent.init(&a_name, &a_description, &an_items, &a_context_of_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprCharacterizedRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0211!(ent, sw);
        rw_block_0163!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprCharacterizedRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0189!(ent, i, iter, nb_elem);
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwCompGroupShAspAndCompShAspAndDatumFeatAndShAsp,
    "RwStepReprRwCompGroupShAspAndCompShAspAndDatumFeatAndShAsp"
);
impl RwStepReprRwCompGroupShAspAndCompShAspAndDatumFeatAndShAsp {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp>,
    ) -> Result<(), Failure> {
        rw_block_0020!(
            a_description,
            a_name,
            a_of_shape,
            a_product_definitional,
            ach,
            data,
            ent,
            num,
            num0
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "COMPOSITE_GROUP_SHAPE_ASPECT")?;
        rw_block_0076!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwCompShAspAndDatumFeatAndShAsp,
    "RwStepReprRwCompShAspAndDatumFeatAndShAsp"
);
impl RwStepReprRwCompShAspAndDatumFeatAndShAsp {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprCompShAspAndDatumFeatAndShAsp>,
    ) -> Result<(), Failure> {
        rw_block_0020!(
            a_description,
            a_name,
            a_of_shape,
            a_product_definitional,
            ach,
            data,
            ent,
            num,
            num0
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprCompShAspAndDatumFeatAndShAsp>,
    ) -> Result<(), Failure> {
        rw_block_0076!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprCompShAspAndDatumFeatAndShAsp>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwCompositeGroupShapeAspect,
    "RwStepReprRwCompositeGroupShapeAspect"
);
impl RwStepReprRwCompositeGroupShapeAspect {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprCompositeGroupShapeAspect>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "composite_group_shape_aspect")? {
            return Ok(());
        }
        rw_block_0045!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
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
        ent: &Handle<StepReprCompositeGroupShapeAspect>,
    ) -> Result<(), Failure> {
        rw_block_0166!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprCompositeGroupShapeAspect>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        Ok(())
    }
}
