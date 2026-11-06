#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepReprRwSpecifiedHigherUsageOccurrence,
    "RwStepReprRwSpecifiedHigherUsageOccurrence"
);
impl RwStepReprRwSpecifiedHigherUsageOccurrence {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprSpecifiedHigherUsageOccurrence>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 8, ach, "specified_higher_usage_occurrence")? {
            return Ok(());
        }
        rw_block_0015!(
            a_assembly_component_usage_reference_designator,
            a_product_definition_relationship_description,
            a_product_definition_relationship_id,
            a_product_definition_relationship_name,
            a_product_definition_relationship_related_product_definition,
            a_product_definition_relationship_relating_product_definition,
            ach,
            data,
            has_assembly_component_usage_reference_designator,
            has_product_definition_relationship_description,
            num
        );
        let mut a_upper_usage: Handle<StepReprAssemblyComponentUsage> = Default::default();
        data.read_entity(
            num,
            7,
            "upper_usage",
            ach,
            StepReprAssemblyComponentUsage::static_type(),
            &mut a_upper_usage,
        )?;
        let mut a_next_usage: Handle<StepReprNextAssemblyUsageOccurrence> = Default::default();
        data.read_entity(
            num,
            8,
            "next_usage",
            ach,
            StepReprNextAssemblyUsageOccurrence::static_type(),
            &mut a_next_usage,
        )?;
        ent.init_product_definition_or_reference(
            &a_product_definition_relationship_id,
            &a_product_definition_relationship_name,
            has_product_definition_relationship_description,
            &a_product_definition_relationship_description,
            &a_product_definition_relationship_relating_product_definition,
            &a_product_definition_relationship_related_product_definition,
            has_assembly_component_usage_reference_designator,
            &a_assembly_component_usage_reference_designator,
            &a_upper_usage,
            &a_next_usage,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprSpecifiedHigherUsageOccurrence>,
    ) -> Result<(), Failure> {
        rw_block_0078!(ent, sw);
        sw.send_entity(&ent.upper_usage())?;
        sw.send_entity(&ent.next_usage())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprSpecifiedHigherUsageOccurrence>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(
            &ent.parent
                .parent
                .parent
                .relating_product_definition_ap242()
                .value(),
        );
        iter.add_item(
            &ent.parent
                .parent
                .parent
                .related_product_definition_ap242()
                .value(),
        );
        iter.add_item(&ent.upper_usage().cast());
        iter.add_item(&ent.next_usage().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwStructuralResponseProperty,
    "RwStepReprRwStructuralResponseProperty"
);
impl RwStepReprRwStructuralResponseProperty {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprStructuralResponseProperty>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "structural_response_property")? {
            return Ok(());
        }
        rw_block_0196!(
            a_property_definition_description,
            a_property_definition_name,
            ach,
            data,
            num
        );
        data.read_string(
            num,
            2,
            "property_definition.description",
            ach,
            &mut a_property_definition_description,
        )?;
        let mut a_property_definition_definition: StepReprCharacterizedDefinition =
            Default::default();
        data.read_entity_select(
            num,
            3,
            "property_definition.definition",
            ach,
            &mut a_property_definition_definition,
        )?;
        ent.init(
            &a_property_definition_name,
            true,
            &a_property_definition_description,
            &a_property_definition_definition,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprStructuralResponseProperty>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.name())?;
        sw.send_entity(&ent.parent.description())?;
        sw.send_entity(&ent.parent.definition().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprStructuralResponseProperty>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.definition().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwStructuralResponsePropertyDefinitionRepresentation,
    "RwStepReprRwStructuralResponsePropertyDefinitionRepresentation"
);
impl RwStepReprRwStructuralResponsePropertyDefinitionRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprStructuralResponsePropertyDefinitionRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            2,
            ach,
            "structural_response_property_definition_representation",
        )? {
            return Ok(());
        }
        let mut a_property_definition_representation_definition: Handle<
            StepReprPropertyDefinition,
        > = Default::default();
        data.read_entity(
            num,
            1,
            "property_definition_representation.definition",
            ach,
            StepReprPropertyDefinition::static_type(),
            &mut a_property_definition_representation_definition,
        )?;
        let mut a_property_definition_representation_used_representation: Handle<
            StepReprRepresentation,
        > = Default::default();
        data.read_entity(
            num,
            2,
            "property_definition_representation.used_representation",
            ach,
            StepReprRepresentation::static_type(),
            &mut a_property_definition_representation_used_representation,
        )?;
        let mut a_represented_definition: StepReprRepresentedDefinition = Default::default();
        a_represented_definition
            .set_value(&a_property_definition_representation_definition.cast())?;
        ent.init(
            &a_represented_definition,
            &a_property_definition_representation_used_representation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprStructuralResponsePropertyDefinitionRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.definition().value())?;
        sw.send_entity(&ent.parent.used_representation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprStructuralResponsePropertyDefinitionRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.definition().value());
        iter.add_item(&ent.parent.used_representation().cast());
        Ok(())
    }
}

step_rw_class!(RwStepReprRwTangent, "RwStepReprRwTangent");
impl RwStepReprRwTangent {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprTangent>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "tangent")? {
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
        ent: &Handle<StepReprTangent>,
    ) -> Result<(), Failure> {
        rw_block_0166!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprTangent>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwValueRepresentationItem,
    "RwStepReprRwValueRepresentationItem"
);
impl RwStepReprRwValueRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprValueRepresentationItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "value_representation_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_member: Handle<StepBasicMeasureValueMember> =
            Handle::new(StepBasicMeasureValueMember::new());
        data.read_member_typed(num, 2, "value_component", ach, &mut a_member)?;
        ent.init(&a_name, &a_member);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprValueRepresentationItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.value_component_member())?;
        Ok(())
    }
}
