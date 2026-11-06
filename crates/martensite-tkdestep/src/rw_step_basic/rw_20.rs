#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwProductDefinition,
    "RwStepBasicRwProductDefinition"
);
impl RwStepBasicRwProductDefinition {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductDefinition>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "product_definition")? {
            return Ok(());
        }
        rw_block_0127!(a_description, a_id, ach, data, num);
        rw_block_0124!(a_formation, a_frame_of_reference, ach, data, num);
        ent.init(&a_id, &a_description, &a_formation, &a_frame_of_reference);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductDefinition>,
    ) -> Result<(), Failure> {
        rw_block_0212!(ent, sw);
        sw.send_entity(&ent.formation())?;
        sw.send_entity(&ent.frame_of_reference())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductDefinition>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.formation().cast());
        iter.get_one_item(&ent.frame_of_reference().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwProductDefinitionContext,
    "RwStepBasicRwProductDefinitionContext"
);
impl RwStepBasicRwProductDefinitionContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductDefinitionContext>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "product_definition_context")? {
            return Ok(());
        }
        rw_block_0129!(a_frame_of_reference, a_name, ach, data, num);
        let mut a_life_cycle_stage: Handle<HAsciiString> = Default::default();
        data.read_string(num, 3, "life_cycle_stage", ach, &mut a_life_cycle_stage)?;
        ent.init(&a_name, &a_frame_of_reference, &a_life_cycle_stage);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductDefinitionContext>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.frame_of_reference())?;
        sw.send_entity(&ent.life_cycle_stage())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductDefinitionContext>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.frame_of_reference().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwProductDefinitionEffectivity,
    "RwStepBasicRwProductDefinitionEffectivity"
);
impl RwStepBasicRwProductDefinitionEffectivity {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductDefinitionEffectivity>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "product_definition_effectivity")? {
            return Ok(());
        }
        let mut a_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "id", ach, &mut a_id)?;
        let mut a_usage: Handle<StepBasicProductDefinitionRelationship> = Default::default();
        data.read_entity(
            num,
            2,
            "kind",
            ach,
            StepBasicProductDefinitionRelationship::static_type(),
            &mut a_usage,
        )?;
        ent.init(&a_id, &a_usage);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductDefinitionEffectivity>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.id())?;
        sw.send_entity(&ent.usage())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductDefinitionEffectivity>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.usage().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwProductDefinitionFormation,
    "RwStepBasicRwProductDefinitionFormation"
);
impl RwStepBasicRwProductDefinitionFormation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductDefinitionFormation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "product_definition_formation")? {
            return Ok(());
        }
        rw_block_0127!(a_description, a_id, ach, data, num);
        let mut a_of_product: Handle<StepBasicProduct> = Default::default();
        data.read_entity(
            num,
            3,
            "of_product",
            ach,
            StepBasicProduct::static_type(),
            &mut a_of_product,
        )?;
        ent.init(&a_id, &a_description, &a_of_product);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductDefinitionFormation>,
    ) -> Result<(), Failure> {
        rw_block_0212!(ent, sw);
        sw.send_entity(&ent.of_product())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductDefinitionFormation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.of_product().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwProductDefinitionFormationRelationship,
    "RwStepBasicRwProductDefinitionFormationRelationship"
);
impl RwStepBasicRwProductDefinitionFormationRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductDefinitionFormationRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "product_definition_formation_relationship")? {
            return Ok(());
        }
        rw_block_0110!(a_description, a_id, a_name, ach, data, num);
        data.read_string(num, 3, "description", ach, &mut a_description)?;
        let mut a_relating_product_definition_formation: Handle<
            StepBasicProductDefinitionFormation,
        > = Default::default();
        data.read_entity(
            num,
            4,
            "relating_product_definition_formation",
            ach,
            StepBasicProductDefinitionFormation::static_type(),
            &mut a_relating_product_definition_formation,
        )?;
        let mut a_related_product_definition_formation: Handle<
            StepBasicProductDefinitionFormation,
        > = Default::default();
        data.read_entity(
            num,
            5,
            "related_product_definition_formation",
            ach,
            StepBasicProductDefinitionFormation::static_type(),
            &mut a_related_product_definition_formation,
        )?;
        ent.init(
            &a_id,
            &a_name,
            &a_description,
            &a_relating_product_definition_formation,
            &a_related_product_definition_formation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductDefinitionFormationRelationship>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.id())?;
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        sw.send_entity(&ent.relating_product_definition_formation())?;
        sw.send_entity(&ent.related_product_definition_formation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductDefinitionFormationRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.relating_product_definition_formation().cast());
        iter.add_item(&ent.related_product_definition_formation().cast());
        Ok(())
    }
}
