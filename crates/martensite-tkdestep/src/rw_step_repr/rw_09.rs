#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepReprRwPerpendicularTo, "RwStepReprRwPerpendicularTo");
impl RwStepReprRwPerpendicularTo {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprPerpendicularTo>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "perpendicular_to")? {
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
        ent: &Handle<StepReprPerpendicularTo>,
    ) -> Result<(), Failure> {
        rw_block_0166!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprPerpendicularTo>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(RwStepReprRwProductConcept, "RwStepReprRwProductConcept");
impl RwStepReprRwProductConcept {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprProductConcept>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "product_concept")? {
            return Ok(());
        }
        rw_block_0068!(a_description, a_id, a_name, ach, data, has_description, num);
        let mut a_market_context: Handle<StepBasicProductConceptContext> = Default::default();
        data.read_entity(
            num,
            4,
            "market_context",
            ach,
            StepBasicProductConceptContext::static_type(),
            &mut a_market_context,
        )?;
        ent.init(
            &a_id,
            &a_name,
            has_description,
            &a_description,
            &a_market_context,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprProductConcept>,
    ) -> Result<(), Failure> {
        rw_block_0178!(ent, sw);
        sw.send_entity(&ent.market_context())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprProductConcept>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.market_context().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwProductDefinitionShape,
    "RwStepReprRwProductDefinitionShape"
);
impl RwStepReprRwProductDefinitionShape {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprProductDefinitionShape>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "product_definition_shape")? {
            return Ok(());
        }
        rw_block_0057!(
            a_property_definition_definition,
            a_property_definition_description,
            a_property_definition_name,
            ach,
            data,
            ent,
            has_property_definition_description,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprProductDefinitionShape>,
    ) -> Result<(), Failure> {
        rw_block_0171!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprProductDefinitionShape>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.definition().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwPropertyDefinition,
    "RwStepReprRwPropertyDefinition"
);
impl RwStepReprRwPropertyDefinition {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprPropertyDefinition>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "property_definition")? {
            return Ok(());
        }
        rw_block_0102!(a_description, a_name, ach, data, has_description, num);
        let mut a_definition: StepReprCharacterizedDefinition = Default::default();
        data.read_entity_select(num, 3, "definition", ach, &mut a_definition)?;
        ent.init(&a_name, has_description, &a_description, &a_definition);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprPropertyDefinition>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        if ent.has_description() {
            sw.send_entity(&ent.description())?;
        } else {
            sw.send_undef()?;
        }
        sw.send_entity(&ent.definition().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprPropertyDefinition>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.definition().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwPropertyDefinitionRelationship,
    "RwStepReprRwPropertyDefinitionRelationship"
);
impl RwStepReprRwPropertyDefinitionRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprPropertyDefinitionRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "property_definition_relationship")? {
            return Ok(());
        }
        rw_block_0149!(a_description, a_name, ach, data, num);
        let mut a_relating_property_definition: Handle<StepReprPropertyDefinition> =
            Default::default();
        data.read_entity(
            num,
            3,
            "relating_property_definition",
            ach,
            StepReprPropertyDefinition::static_type(),
            &mut a_relating_property_definition,
        )?;
        let mut a_related_property_definition: Handle<StepReprPropertyDefinition> =
            Default::default();
        data.read_entity(
            num,
            4,
            "related_property_definition",
            ach,
            StepReprPropertyDefinition::static_type(),
            &mut a_related_property_definition,
        )?;
        ent.init(
            &a_name,
            &a_description,
            &a_relating_property_definition,
            &a_related_property_definition,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprPropertyDefinitionRelationship>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        sw.send_entity(&ent.relating_property_definition())?;
        sw.send_entity(&ent.related_property_definition())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprPropertyDefinitionRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.relating_property_definition().cast());
        iter.add_item(&ent.related_property_definition().cast());
        Ok(())
    }
}
