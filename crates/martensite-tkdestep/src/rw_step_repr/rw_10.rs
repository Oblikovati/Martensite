#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepReprRwPropertyDefinitionRepresentation,
    "RwStepReprRwPropertyDefinitionRepresentation"
);
impl RwStepReprRwPropertyDefinitionRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprPropertyDefinitionRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "property_definition_representation")? {
            return Ok(());
        }
        let mut a_definition: StepReprRepresentedDefinition = Default::default();
        data.read_entity_select(num, 1, "definition", ach, &mut a_definition)?;
        let mut a_used_representation: Handle<StepReprRepresentation> = Default::default();
        data.read_entity(
            num,
            2,
            "used_representation",
            ach,
            StepReprRepresentation::static_type(),
            &mut a_used_representation,
        )?;
        ent.init(&a_definition, &a_used_representation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprPropertyDefinitionRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.definition().value())?;
        sw.send_entity(&ent.used_representation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprPropertyDefinitionRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.definition().value());
        iter.add_item(&ent.used_representation().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwQuantifiedAssemblyComponentUsage,
    "RwStepReprRwQuantifiedAssemblyComponentUsage"
);
impl RwStepReprRwQuantifiedAssemblyComponentUsage {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprQuantifiedAssemblyComponentUsage>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 7, ach, "quantified_assembly_component_usage")? {
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
        let mut a_quantity: Handle<StandardTransient> = Default::default();
        data.read_entity(
            num,
            7,
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
            has_assembly_component_usage_reference_designator,
            &a_assembly_component_usage_reference_designator,
            &a_quantity,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprQuantifiedAssemblyComponentUsage>,
    ) -> Result<(), Failure> {
        rw_block_0078!(ent, sw);
        sw.send_entity(&ent.quantity())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprQuantifiedAssemblyComponentUsage>,
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
        iter.add_item(&ent.quantity());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwRealRepresentationItem,
    "RwStepReprRwRealRepresentationItem"
);
impl RwStepReprRwRealRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_ach: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepReprRealRepresentationItem>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 2, the_ach, "real_representation_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(the_num, 1, "name", the_ach, &mut a_name)?;
        let mut a_value: f64 = Default::default();
        the_data.read_real(the_num, 2, "value", the_ach, &mut a_value)?;
        the_ent.init(&a_name, a_value);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepReprRealRepresentationItem>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_real(the_ent.value())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwReprItemAndLengthMeasureWithUnit,
    "RwStepReprRwReprItemAndLengthMeasureWithUnit"
);
impl RwStepReprRwReprItemAndLengthMeasureWithUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprReprItemAndLengthMeasureWithUnit>,
    ) -> Result<(), Failure> {
        rw_block_0012!(
            a_measure_with_unit,
            a_name,
            a_repr_item,
            a_unit_component,
            ach,
            data,
            ent,
            mvc,
            num,
            num0
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprReprItemAndLengthMeasureWithUnit>,
    ) -> Result<(), Failure> {
        rw_block_0112!(ent, sw);
        step_start_entity(sw, "REPRESENTATION_ITEM")?;
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwReprItemAndLengthMeasureWithUnitAndQRI,
    "RwStepReprRwReprItemAndLengthMeasureWithUnitAndQRI"
);
impl RwStepReprRwReprItemAndLengthMeasureWithUnitAndQRI {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprReprItemAndLengthMeasureWithUnitAndQRI>,
    ) -> Result<(), Failure> {
        rw_block_0005!(
            a_measure_with_unit,
            a_name,
            a_qri,
            a_repr_item,
            a_unit_component,
            a_vq,
            ach,
            data,
            ent,
            i,
            mvc,
            nb,
            nsub,
            num,
            num0,
            quals
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprReprItemAndLengthMeasureWithUnitAndQRI>,
    ) -> Result<(), Failure> {
        rw_block_0112!(ent, sw);
        rw_block_0048!(a_qri, ent, i, nbq, sw);
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwReprItemAndPlaneAngleMeasureWithUnit,
    "RwStepReprRwReprItemAndPlaneAngleMeasureWithUnit"
);
impl RwStepReprRwReprItemAndPlaneAngleMeasureWithUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprReprItemAndPlaneAngleMeasureWithUnit>,
    ) -> Result<(), Failure> {
        rw_block_0012!(
            a_measure_with_unit,
            a_name,
            a_repr_item,
            a_unit_component,
            ach,
            data,
            ent,
            mvc,
            num,
            num0
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprReprItemAndPlaneAngleMeasureWithUnit>,
    ) -> Result<(), Failure> {
        rw_block_0113!(ent, sw);
        step_start_entity(sw, "REPRESENTATION_ITEM")?;
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwReprItemAndPlaneAngleMeasureWithUnitAndQRI,
    "RwStepReprRwReprItemAndPlaneAngleMeasureWithUnitAndQRI"
);
impl RwStepReprRwReprItemAndPlaneAngleMeasureWithUnitAndQRI {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI>,
    ) -> Result<(), Failure> {
        rw_block_0005!(
            a_measure_with_unit,
            a_name,
            a_qri,
            a_repr_item,
            a_unit_component,
            a_vq,
            ach,
            data,
            ent,
            i,
            mvc,
            nb,
            nsub,
            num,
            num0,
            quals
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI>,
    ) -> Result<(), Failure> {
        rw_block_0113!(ent, sw);
        rw_block_0048!(a_qri, ent, i, nbq, sw);
        Ok(())
    }
}
