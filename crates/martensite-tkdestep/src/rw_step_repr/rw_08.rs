#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepReprRwMaterialPropertyRepresentation,
    "RwStepReprRwMaterialPropertyRepresentation"
);
impl RwStepReprRwMaterialPropertyRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprMaterialPropertyRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "material_property_representation")? {
            return Ok(());
        }
        rw_block_0142!(
            a_property_definition_representation_definition,
            a_property_definition_representation_used_representation,
            ach,
            data,
            num
        );
        let mut a_dependent_environment: Handle<StepReprDataEnvironment> = Default::default();
        data.read_entity(
            num,
            3,
            "dependent_environment",
            ach,
            StepReprDataEnvironment::static_type(),
            &mut a_dependent_environment,
        )?;
        ent.init(
            &a_property_definition_representation_definition,
            &a_property_definition_representation_used_representation,
            &a_dependent_environment,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprMaterialPropertyRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.definition().value())?;
        sw.send_entity(&ent.parent.used_representation())?;
        sw.send_entity(&ent.dependent_environment())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprMaterialPropertyRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.definition().value());
        iter.add_item(&ent.parent.used_representation().cast());
        iter.add_item(&ent.dependent_environment().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwMeasureRepresentationItem,
    "RwStepReprRwMeasureRepresentationItem"
);
impl RwStepReprRwMeasureRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprMeasureRepresentationItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "measure_representation_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut mvc: Handle<StepBasicMeasureValueMember> =
            Handle::new(StepBasicMeasureValueMember::new());
        data.read_member_typed(num, 2, "value_component", ach, &mut mvc)?;
        let mut a_unit_component: StepBasicUnit = Default::default();
        data.read_entity_select(num, 3, "unit_component", ach, &mut a_unit_component)?;
        ent.init(&a_name, &mvc, &a_unit_component)?;
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprMeasureRepresentationItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        rw_block_0209!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprMeasureRepresentationItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&step_live(&ent.measure(), "Share")?.unit_component().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwMechanicalDesignAndDraughtingRelationship,
    "RwStepReprRwMechanicalDesignAndDraughtingRelationship"
);
impl RwStepReprRwMechanicalDesignAndDraughtingRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_ach: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepReprMechanicalDesignAndDraughtingRelationship>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            4,
            the_ach,
            "mechanical_design_and_draughting_relationship",
        )? {
            return Ok(());
        }
        rw_block_0195!(a_description, a_name, the_ach, the_data, the_num);
        if the_data.is_param_defined(the_num, 2)? {
            the_data.read_string(the_num, 2, "description", the_ach, &mut a_description)?;
        }
        let mut a_rep1: Handle<StepReprRepresentation> = Default::default();
        the_data.read_entity(
            the_num,
            3,
            "rep_1",
            the_ach,
            StepReprRepresentation::static_type(),
            &mut a_rep1,
        )?;
        let mut a_rep2: Handle<StepReprRepresentation> = Default::default();
        the_data.read_entity(
            the_num,
            4,
            "rep_2",
            the_ach,
            StepReprRepresentation::static_type(),
            &mut a_rep2,
        )?;
        the_ent.init(&a_name, &a_description, &a_rep1, &a_rep2);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepReprMechanicalDesignAndDraughtingRelationship>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.description())?;
        the_sw.send_entity(&the_ent.rep1())?;
        the_sw.send_entity(&the_ent.rep2())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepReprMechanicalDesignAndDraughtingRelationship>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        the_iter.get_one_item(&the_ent.rep1().cast());
        the_iter.get_one_item(&the_ent.rep2().cast());
        Ok(())
    }
}

step_rw_class!(RwStepReprRwParallelOffset, "RwStepReprRwParallelOffset");
impl RwStepReprRwParallelOffset {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprParallelOffset>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "parallel_offset")? {
            return Ok(());
        }
        rw_block_0056!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            ach,
            data,
            num
        );
        let mut an_offset: Handle<StandardTransient> = Default::default();
        data.read_entity(
            num,
            5,
            "offset",
            ach,
            StandardTransient::static_type(),
            &mut an_offset,
        )?;
        ent.init(
            &a_shape_aspect_name,
            &a_shape_aspect_description,
            &a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            &an_offset,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprParallelOffset>,
    ) -> Result<(), Failure> {
        rw_block_0166!(ent, sw);
        sw.send_entity(&ent.offset())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprParallelOffset>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwParametricRepresentationContext,
    "RwStepReprRwParametricRepresentationContext"
);
impl RwStepReprRwParametricRepresentationContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprParametricRepresentationContext>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "parametric_representation_context")? {
            return Ok(());
        }
        rw_block_0150!(a_context_identifier, a_context_type, ach, data, num);
        ent.init(&a_context_identifier, &a_context_type);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprParametricRepresentationContext>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.context_identifier())?;
        sw.send_entity(&ent.context_type())?;
        Ok(())
    }
}
