#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepReprRwItemDefinedTransformation,
    "RwStepReprRwItemDefinedTransformation"
);
impl RwStepReprRwItemDefinedTransformation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprItemDefinedTransformation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "item_defined_transformation")? {
            return Ok(());
        }
        let mut name: Handle<HAsciiString> = Default::default();
        let mut description: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut name)?;
        if data.is_param_defined(num, 2)? {
            data.read_string(num, 2, "description", ach, &mut description)?;
        }
        let mut ti1: Handle<StepReprRepresentationItem> = Default::default();
        let mut ti2: Handle<StepReprRepresentationItem> = Default::default();
        data.read_entity(
            num,
            3,
            "transform_item_1",
            ach,
            StepReprRepresentationItem::static_type(),
            &mut ti1,
        )?;
        data.read_entity(
            num,
            4,
            "transform_item_2",
            ach,
            StepReprRepresentationItem::static_type(),
            &mut ti2,
        )?;
        ent.init(&name, &description, &ti1, &ti2);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprItemDefinedTransformation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        sw.send_entity(&ent.transform_item1())?;
        sw.send_entity(&ent.transform_item2())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprItemDefinedTransformation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.transform_item1().cast());
        iter.add_item(&ent.transform_item2().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwMakeFromUsageOption,
    "RwStepReprRwMakeFromUsageOption"
);
impl RwStepReprRwMakeFromUsageOption {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprMakeFromUsageOption>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 8, ach, "make_from_usage_option")? {
            return Ok(());
        }
        rw_block_0025!(
            a_product_definition_relationship_description,
            a_product_definition_relationship_id,
            a_product_definition_relationship_name,
            a_product_definition_relationship_related_product_definition,
            a_product_definition_relationship_relating_product_definition,
            ach,
            data,
            has_product_definition_relationship_description,
            num
        );
        let mut a_ranking: i32 = Default::default();
        data.read_integer(num, 6, "ranking", ach, &mut a_ranking)?;
        let mut a_ranking_rationale: Handle<HAsciiString> = Default::default();
        data.read_string(num, 7, "ranking_rationale", ach, &mut a_ranking_rationale)?;
        let mut a_quantity: Handle<StandardTransient> = Default::default();
        data.read_entity(
            num,
            8,
            "quantity",
            ach,
            StandardTransient::static_type(),
            &mut a_quantity,
        )?;
        ent.init_product_definition_or_reference(
            &a_product_definition_relationship_id,
            &a_product_definition_relationship_name,
            has_product_definition_relationship_description,
            &a_product_definition_relationship_description,
            &a_product_definition_relationship_relating_product_definition,
            &a_product_definition_relationship_related_product_definition,
            a_ranking,
            &a_ranking_rationale,
            &a_quantity,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprMakeFromUsageOption>,
    ) -> Result<(), Failure> {
        rw_block_0097!(ent, sw);
        sw.send_integer(ent.ranking())?;
        sw.send_entity(&ent.ranking_rationale())?;
        sw.send_entity(&ent.quantity())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprMakeFromUsageOption>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(
            &ent.parent
                .parent
                .relating_product_definition_ap242()
                .value(),
        );
        iter.add_item(&ent.parent.parent.related_product_definition_ap242().value());
        iter.add_item(&ent.quantity());
        Ok(())
    }
}

step_rw_class!(RwStepReprRwMappedItem, "RwStepReprRwMappedItem");
impl RwStepReprRwMappedItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprMappedItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "mapped_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_mapping_source: Handle<StepReprRepresentationMap> = Default::default();
        data.read_entity(
            num,
            2,
            "mapping_source",
            ach,
            StepReprRepresentationMap::static_type(),
            &mut a_mapping_source,
        )?;
        let mut a_mapping_target: Handle<StepReprRepresentationItem> = Default::default();
        data.read_entity(
            num,
            3,
            "mapping_target",
            ach,
            StepReprRepresentationItem::static_type(),
            &mut a_mapping_target,
        )?;
        ent.init(&a_name, &a_mapping_source, &a_mapping_target);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprMappedItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.mapping_source())?;
        sw.send_entity(&ent.mapping_target())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprMappedItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.mapping_source().cast());
        iter.get_one_item(&ent.mapping_target().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwMaterialDesignation,
    "RwStepReprRwMaterialDesignation"
);
impl RwStepReprRwMaterialDesignation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprMaterialDesignation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "material_designation")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_of_definition: StepReprCharacterizedDefinition = Default::default();
        data.read_entity_select(num, 2, "of_definition", ach, &mut a_of_definition)?;
        ent.init(&a_name, &a_of_definition);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprMaterialDesignation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.of_definition().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprMaterialDesignation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_definition().value());
        Ok(())
    }
}

step_rw_class!(RwStepReprRwMaterialProperty, "RwStepReprRwMaterialProperty");
impl RwStepReprRwMaterialProperty {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprMaterialProperty>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "material_property")? {
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
        ent: &Handle<StepReprMaterialProperty>,
    ) -> Result<(), Failure> {
        rw_block_0171!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprMaterialProperty>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.definition().value());
        Ok(())
    }
}
