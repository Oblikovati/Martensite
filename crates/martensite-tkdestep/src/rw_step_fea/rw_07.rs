#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepFEARwFeaLinearElasticity,
    "RwStepFEARwFeaLinearElasticity"
);
impl RwStepFEARwFeaLinearElasticity {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaLinearElasticity>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fea_linear_elasticity")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "representation_item.name",
            ach,
            &mut a_representation_item_name,
        )?;
        let mut a_fea_constants: StepFEASymmetricTensor43d = Default::default();
        data.read_entity_select(num, 2, "fea_constants", ach, &mut a_fea_constants)?;
        ent.init(&a_representation_item_name, &a_fea_constants);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaLinearElasticity>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.fea_constants().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaLinearElasticity>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.fea_constants().value());
        Ok(())
    }
}

step_rw_class!(RwStepFEARwFeaMassDensity, "RwStepFEARwFeaMassDensity");
impl RwStepFEARwFeaMassDensity {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaMassDensity>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fea_mass_density")? {
            return Ok(());
        }
        rw_block_b_0112!(
            a_fea_constant,
            a_representation_item_name,
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
        ent: &Handle<StepFEAFeaMassDensity>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_real(ent.fea_constant())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepFEAFeaMassDensity>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaMaterialPropertyRepresentation,
    "RwStepFEARwFeaMaterialPropertyRepresentation"
);
impl RwStepFEARwFeaMaterialPropertyRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaMaterialPropertyRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "fea_material_property_representation")? {
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
        let mut a_definition: StepReprRepresentedDefinition = Default::default();
        a_definition.set_value(&a_property_definition_representation_definition.cast())?;
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
        let mut a_material_property_representation_dependent_environment: Handle<
            StepReprDataEnvironment,
        > = Default::default();
        data.read_entity(
            num,
            3,
            "material_property_representation.dependent_environment",
            ach,
            StepReprDataEnvironment::static_type(),
            &mut a_material_property_representation_dependent_environment,
        )?;
        ent.init(
            &a_definition,
            &a_property_definition_representation_used_representation,
            &a_material_property_representation_dependent_environment,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaMaterialPropertyRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.definition().property_definition())?;
        sw.send_entity(&ent.parent.parent.used_representation())?;
        sw.send_entity(&ent.parent.dependent_environment())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaMaterialPropertyRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.parent.definition().property_definition().cast());
        iter.add_item(&ent.parent.parent.used_representation().cast());
        iter.add_item(&ent.parent.dependent_environment().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaMaterialPropertyRepresentationItem,
    "RwStepFEARwFeaMaterialPropertyRepresentationItem"
);
impl RwStepFEARwFeaMaterialPropertyRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaMaterialPropertyRepresentationItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "fea_material_property_representation_item")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "representation_item.name",
            ach,
            &mut a_representation_item_name,
        )?;
        ent.init(&a_representation_item_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaMaterialPropertyRepresentationItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.name())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepFEAFeaMaterialPropertyRepresentationItem>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(RwStepFEARwFeaModel, "RwStepFEARwFeaModel");
impl RwStepFEARwFeaModel {
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaModel>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0207!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(RwStepFEARwFeaModel3d, "RwStepFEARwFeaModel3d");
impl RwStepFEARwFeaModel3d {
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaModel3d>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0206!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaModelDefinition,
    "RwStepFEARwFeaModelDefinition"
);
impl RwStepFEARwFeaModelDefinition {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaModelDefinition>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "fea_model_definition")? {
            return Ok(());
        }
        rw_block_b_0054!(
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
        ent: &Handle<StepFEAFeaModelDefinition>,
    ) -> Result<(), Failure> {
        rw_block_b_0158!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaModelDefinition>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.of_shape().cast());
        Ok(())
    }
}
