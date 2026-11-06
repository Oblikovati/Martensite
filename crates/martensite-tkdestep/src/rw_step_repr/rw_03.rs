#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepReprRwCompositeShapeAspect,
    "RwStepReprRwCompositeShapeAspect"
);
impl RwStepReprRwCompositeShapeAspect {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprCompositeShapeAspect>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "composite_shape_aspect")? {
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
        ent: &Handle<StepReprCompositeShapeAspect>,
    ) -> Result<(), Failure> {
        rw_block_0158!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprCompositeShapeAspect>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwCompoundRepresentationItem,
    "RwStepReprRwCompoundRepresentationItem"
);
impl RwStepReprRwCompoundRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprCompoundRepresentationItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "compound_representation_item")? {
            return Ok(());
        }
        rw_block_0108!(a_items, a_name, ach, anent2, data, nsub2, num);
        if data.read_sub_list(num, 2, "item_element", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, nb2)?;
            for i2 in 1..=nb2 {
                if data.read_entity(
                    nsub2,
                    i2,
                    "representation_item",
                    ach,
                    StepReprRepresentationItem::static_type(),
                    &mut anent2,
                )? {
                    step_array1_set(&a_items, i2, anent2.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprCompoundRepresentationItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_item_element() {
            sw.send_entity(&ent.item_element_value(i2)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprCompoundRepresentationItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let i: i32 = Default::default();
        let nb: i32 = ent.nb_item_element();
        for i in 1..=nb {
            iter.add_item(&ent.item_element_value(i)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwConfigurationDesign,
    "RwStepReprRwConfigurationDesign"
);
impl RwStepReprRwConfigurationDesign {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprConfigurationDesign>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "configuration_design")? {
            return Ok(());
        }
        let mut a_configuration: Handle<StepReprConfigurationItem> = Default::default();
        data.read_entity(
            num,
            1,
            "configuration",
            ach,
            StepReprConfigurationItem::static_type(),
            &mut a_configuration,
        )?;
        let mut a_design: StepReprConfigurationDesignItem = Default::default();
        data.read_entity_select(num, 2, "design", ach, &mut a_design)?;
        ent.init(&a_configuration, &a_design);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprConfigurationDesign>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.configuration())?;
        sw.send_entity(&ent.design().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprConfigurationDesign>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.configuration().cast());
        iter.add_item(&ent.design().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwConfigurationEffectivity,
    "RwStepReprRwConfigurationEffectivity"
);
impl RwStepReprRwConfigurationEffectivity {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprConfigurationEffectivity>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "configuration_effectivity")? {
            return Ok(());
        }
        let mut a_effectivity_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "effectivity.id", ach, &mut a_effectivity_id)?;
        let mut a_product_definition_effectivity_usage: Handle<
            StepBasicProductDefinitionRelationship,
        > = Default::default();
        data.read_entity(
            num,
            2,
            "product_definition_effectivity.usage",
            ach,
            StepBasicProductDefinitionRelationship::static_type(),
            &mut a_product_definition_effectivity_usage,
        )?;
        let mut a_configuration: Handle<StepReprConfigurationDesign> = Default::default();
        data.read_entity(
            num,
            3,
            "configuration",
            ach,
            StepReprConfigurationDesign::static_type(),
            &mut a_configuration,
        )?;
        ent.init(
            &a_effectivity_id,
            &a_product_definition_effectivity_usage,
            &a_configuration,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprConfigurationEffectivity>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.id())?;
        sw.send_entity(&ent.parent.usage())?;
        sw.send_entity(&ent.configuration())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprConfigurationEffectivity>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.usage().cast());
        iter.add_item(&ent.configuration().cast());
        Ok(())
    }
}
