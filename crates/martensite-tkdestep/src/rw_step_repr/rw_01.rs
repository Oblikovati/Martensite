#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepReprRwAllAroundShapeAspect,
    "RwStepReprRwAllAroundShapeAspect"
);
impl RwStepReprRwAllAroundShapeAspect {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprAllAroundShapeAspect>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "all_around_shape_aspect")? {
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
        ent: &Handle<StepReprAllAroundShapeAspect>,
    ) -> Result<(), Failure> {
        rw_block_0166!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprAllAroundShapeAspect>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(RwStepReprRwApex, "RwStepReprRwApex");
impl RwStepReprRwApex {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprApex>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "apex")? {
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
        ent: &Handle<StepReprApex>,
    ) -> Result<(), Failure> {
        rw_block_0166!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprApex>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwAssemblyComponentUsage,
    "RwStepReprRwAssemblyComponentUsage"
);
impl RwStepReprRwAssemblyComponentUsage {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprAssemblyComponentUsage>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "assembly_component_usage")? {
            return Ok(());
        }
        rw_block_0046!(
            a_product_definition_relationship_description,
            a_product_definition_relationship_id,
            a_product_definition_relationship_name,
            a_product_definition_relationship_relating_product_definition,
            ach,
            data,
            has_product_definition_relationship_description,
            num
        );
        if a_product_definition_relationship_relating_product_definition
            .value()
            .is_null()
        {
            let mut a_product_definition_shape: Handle<StepReprProductDefinitionShape> =
                Default::default();
            data.read_entity(
                num,
                4,
                "product_definition_relationship.relating_product_definition_shape",
                ach,
                StepReprProductDefinitionShape::static_type(),
                &mut a_product_definition_shape,
            )?;
            if !a_product_definition_shape.is_null() {
                a_product_definition_relationship_relating_product_definition.set_value(
                    &step_live(&a_product_definition_shape, "ReadStep")?
                        .definition()
                        .product_definition()
                        .cast(),
                )?;
            }
        }
        let mut a_product_definition_relationship_related_product_definition: StepBasicProductDefinitionOrReference = Default::default();
        data.read_entity_select(
            num,
            5,
            "product_definition_relationship.related_product_definition",
            ach,
            &mut a_product_definition_relationship_related_product_definition,
        )?;
        if a_product_definition_relationship_related_product_definition
            .value()
            .is_null()
        {
            let mut a_product_definition_shape: Handle<StepReprProductDefinitionShape> =
                Default::default();
            data.read_entity(
                num,
                5,
                "product_definition_relationship.related_product_definition_shape",
                ach,
                StepReprProductDefinitionShape::static_type(),
                &mut a_product_definition_shape,
            )?;
            if !a_product_definition_shape.is_null() {
                a_product_definition_relationship_related_product_definition.set_value(
                    &step_live(&a_product_definition_shape, "ReadStep")?
                        .definition()
                        .product_definition()
                        .cast(),
                )?;
            }
        }
        let mut a_reference_designator: Handle<HAsciiString> = Default::default();
        let mut has_reference_designator: bool = true;
        if data.is_param_defined(num, 6)? {
            data.read_string(
                num,
                6,
                "reference_designator",
                ach,
                &mut a_reference_designator,
            )?;
        } else {
            has_reference_designator = false;
        }
        ent.init_product_definition_or_reference(
            &a_product_definition_relationship_id,
            &a_product_definition_relationship_name,
            has_product_definition_relationship_description,
            &a_product_definition_relationship_description,
            &a_product_definition_relationship_relating_product_definition,
            &a_product_definition_relationship_related_product_definition,
            has_reference_designator,
            &a_reference_designator,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprAssemblyComponentUsage>,
    ) -> Result<(), Failure> {
        rw_block_0097!(ent, sw);
        if ent.has_reference_designator() {
            sw.send_entity(&ent.reference_designator())?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprAssemblyComponentUsage>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(
            &ent.parent
                .parent
                .relating_product_definition_ap242()
                .value(),
        );
        iter.add_item(&ent.parent.parent.related_product_definition_ap242().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwAssemblyComponentUsageSubstitute,
    "RwStepReprRwAssemblyComponentUsageSubstitute"
);
impl RwStepReprRwAssemblyComponentUsageSubstitute {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprAssemblyComponentUsageSubstitute>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "assembly_component_usage_substitute")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_def: Handle<HAsciiString> = Default::default();
        if data.is_param_defined(num, 2)? {
            data.read_string(num, 2, "definition", ach, &mut a_def)?;
        }
        let mut a_base: Handle<StepReprAssemblyComponentUsage> = Default::default();
        data.read_entity(
            num,
            3,
            "base",
            ach,
            StepReprAssemblyComponentUsage::static_type(),
            &mut a_base,
        )?;
        let mut a_subs: Handle<StepReprAssemblyComponentUsage> = Default::default();
        data.read_entity(
            num,
            4,
            "substitute",
            ach,
            StepReprAssemblyComponentUsage::static_type(),
            &mut a_subs,
        )?;
        ent.init(&a_name, &a_def, &a_base, &a_subs);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprAssemblyComponentUsageSubstitute>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.definition())?;
        sw.send_entity(&ent.base())?;
        sw.send_entity(&ent.substitute())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprAssemblyComponentUsageSubstitute>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.base().cast());
        iter.get_one_item(&ent.substitute().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwBetweenShapeAspect,
    "RwStepReprRwBetweenShapeAspect"
);
impl RwStepReprRwBetweenShapeAspect {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprBetweenShapeAspect>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "between_shape_aspect")? {
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
        ent: &Handle<StepReprBetweenShapeAspect>,
    ) -> Result<(), Failure> {
        rw_block_0166!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprBetweenShapeAspect>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        Ok(())
    }
}
