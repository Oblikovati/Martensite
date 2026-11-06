#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepReprRwConfigurationItem,
    "RwStepReprRwConfigurationItem"
);
impl RwStepReprRwConfigurationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprConfigurationItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "configuration_item")? {
            return Ok(());
        }
        rw_block_0068!(a_description, a_id, a_name, ach, data, has_description, num);
        let mut a_item_concept: Handle<StepReprProductConcept> = Default::default();
        data.read_entity(
            num,
            4,
            "item_concept",
            ach,
            StepReprProductConcept::static_type(),
            &mut a_item_concept,
        )?;
        let mut a_purpose: Handle<HAsciiString> = Default::default();
        let mut has_purpose: bool = true;
        if data.is_param_defined(num, 5)? {
            data.read_string(num, 5, "purpose", ach, &mut a_purpose)?;
        } else {
            has_purpose = false;
        }
        ent.init(
            &a_id,
            &a_name,
            has_description,
            &a_description,
            &a_item_concept,
            has_purpose,
            &a_purpose,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprConfigurationItem>,
    ) -> Result<(), Failure> {
        rw_block_0178!(ent, sw);
        sw.send_entity(&ent.item_concept())?;
        if ent.has_purpose() {
            sw.send_entity(&ent.purpose())?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprConfigurationItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.item_concept().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwConstructiveGeometryRepresentation,
    "RwStepReprRwConstructiveGeometryRepresentation"
);
impl RwStepReprRwConstructiveGeometryRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprConstructiveGeometryRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "constructive_geometry_representation")? {
            return Ok(());
        }
        rw_block_0179!(a_items, a_name, ach, data, num);
        let mut an_ent: Handle<StepReprRepresentationItem> = Default::default();
        let mut nsub: i32 = Default::default();
        if data.read_sub_list(num, 2, "items", ach, &mut nsub, false, 0, 0)? {
            let nb: i32 = data.nb_params(nsub)?;
            a_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, nb)?;
            for i in 1..=nb {
                if data.read_entity(
                    nsub,
                    i,
                    "representation_item",
                    ach,
                    StepReprRepresentationItem::static_type(),
                    &mut an_ent,
                )? {
                    step_array1_set(&a_items, i, an_ent.clone(), "ReadStep")?;
                }
            }
        }
        let mut a_context_of_items: Handle<StepReprRepresentationContext> = Default::default();
        data.read_entity(
            num,
            3,
            "context_of_items",
            ach,
            StepReprRepresentationContext::static_type(),
            &mut a_context_of_items,
        )?;
        ent.init(&a_name, &a_items, &a_context_of_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprConstructiveGeometryRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        rw_block_0163!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprConstructiveGeometryRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0189!(ent, i, iter, nb_elem);
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwConstructiveGeometryRepresentationRelationship,
    "RwStepReprRwConstructiveGeometryRepresentationRelationship"
);
impl RwStepReprRwConstructiveGeometryRepresentationRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprConstructiveGeometryRepresentationRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            4,
            ach,
            "constructive_geometry_representation_relationship",
        )? {
            return Ok(());
        }
        rw_block_0055!(a_description, a_name, a_rep1, a_rep2, ach, data, num);
        ent.init(&a_name, &a_description, &a_rep1, &a_rep2);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprConstructiveGeometryRepresentationRelationship>,
    ) -> Result<(), Failure> {
        rw_block_0165!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprConstructiveGeometryRepresentationRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.rep1().cast());
        iter.get_one_item(&ent.rep2().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwContinuosShapeAspect,
    "RwStepReprRwContinuosShapeAspect"
);
impl RwStepReprRwContinuosShapeAspect {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprContinuosShapeAspect>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "continuos_shape_aspect")? {
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
        ent: &Handle<StepReprContinuosShapeAspect>,
    ) -> Result<(), Failure> {
        rw_block_0166!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprContinuosShapeAspect>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        Ok(())
    }
}
