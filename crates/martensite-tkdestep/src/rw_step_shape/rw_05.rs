#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepShapeRwContextDependentShapeRepresentation,
    "RwStepShapeRwContextDependentShapeRepresentation"
);
impl RwStepShapeRwContextDependentShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeContextDependentShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "context_dependent_shape_representation")? {
            return Ok(());
        }
        let mut a_rep_rel: Handle<StepReprShapeRepresentationRelationship> = Default::default();
        data.read_entity(
            num,
            1,
            "representation_relation",
            ach,
            StepReprShapeRepresentationRelationship::static_type(),
            &mut a_rep_rel,
        )?;
        let mut a_pro_rel: Handle<StepReprProductDefinitionShape> = Default::default();
        data.read_entity(
            num,
            2,
            "represented_product_relation",
            ach,
            StepReprProductDefinitionShape::static_type(),
            &mut a_pro_rel,
        )?;
        ent.init(&a_rep_rel, &a_pro_rel);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeContextDependentShapeRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.representation_relation())?;
        sw.send_entity(&ent.represented_product_relation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeContextDependentShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.representation_relation().cast());
        iter.add_item(&ent.represented_product_relation().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwCsgShapeRepresentation,
    "RwStepShapeRwCsgShapeRepresentation"
);
impl RwStepShapeRwCsgShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeCsgShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "csg_shape_representation")? {
            return Ok(());
        }
        rw_block_0039!(
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
        ent: &Handle<StepShapeCsgShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0116!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeCsgShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0190!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwCsgSolid, "RwStepShapeRwCsgSolid");
impl RwStepShapeRwCsgSolid {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeCsgSolid>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "csg_solid")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_boolean_result: Handle<StepShapeBooleanResult> = Default::default();
        data.read_entity(
            num,
            2,
            "tree_root_expression",
            ach,
            StepShapeBooleanResult::static_type(),
            &mut a_boolean_result,
        )?;
        let mut a_tree_root_expression: StepShapeCsgSelect = Default::default();
        a_tree_root_expression.set_boolean_result(&a_boolean_result);
        ent.init(&a_name, &a_tree_root_expression);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeCsgSolid>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.tree_root_expression().boolean_result())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeCsgSolid>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.tree_root_expression().boolean_result().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwDefinitionalRepresentationAndShapeRepresentation,
    "RwStepShapeRwDefinitionalRepresentationAndShapeRepresentation"
);
impl RwStepShapeRwDefinitionalRepresentationAndShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeDefinitionalRepresentationAndShapeRepresentation>,
    ) -> Result<(), Failure> {
        let mut num: i32 = num0;
        num = data.next_for_complex(num);
        rw_block_0038!(
            a_context_of_items,
            a_items,
            a_name,
            ach,
            anent2,
            data,
            i2,
            nb2,
            nsub2,
            num
        );
        num = data.next_for_complex(num);
        ent.init(&a_name, &a_items, &a_context_of_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeDefinitionalRepresentationAndShapeRepresentation>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "DEFINITIONAL_REPRESENTATION")?;
        step_start_entity(sw, "REPRESENTATION")?;
        rw_block_0116!(ent, i2, sw);
        step_start_entity(sw, "SHAPE_REPRESENTATION")?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeDefinitionalRepresentationAndShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0190!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwDimensionalCharacteristicRepresentation,
    "RwStepShapeRwDimensionalCharacteristicRepresentation"
);
impl RwStepShapeRwDimensionalCharacteristicRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeDimensionalCharacteristicRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "dimensional_characteristic_representation")? {
            return Ok(());
        }
        let mut a_dimension: StepShapeDimensionalCharacteristic = Default::default();
        data.read_entity_select(num, 1, "dimension", ach, &mut a_dimension)?;
        let mut a_representation: Handle<StepShapeShapeDimensionRepresentation> =
            Default::default();
        data.read_entity(
            num,
            2,
            "representation",
            ach,
            StepShapeShapeDimensionRepresentation::static_type(),
            &mut a_representation,
        )?;
        ent.init(&a_dimension, &a_representation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeDimensionalCharacteristicRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.dimension().value())?;
        sw.send_entity(&ent.representation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeDimensionalCharacteristicRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.dimension().value());
        iter.add_item(&ent.representation().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwDimensionalLocation,
    "RwStepShapeRwDimensionalLocation"
);
impl RwStepShapeRwDimensionalLocation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeDimensionalLocation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "dimensional_location")? {
            return Ok(());
        }
        rw_block_0032!(
            a_shape_aspect_relationship_description,
            a_shape_aspect_relationship_name,
            a_shape_aspect_relationship_related_shape_aspect,
            a_shape_aspect_relationship_relating_shape_aspect,
            ach,
            data,
            ent,
            has_shape_aspect_relationship_description,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeDimensionalLocation>,
    ) -> Result<(), Failure> {
        rw_block_0128!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeDimensionalLocation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.relating_shape_aspect().cast());
        iter.add_item(&ent.parent.related_shape_aspect().cast());
        Ok(())
    }
}
