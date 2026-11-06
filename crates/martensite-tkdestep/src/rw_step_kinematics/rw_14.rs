#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwProductDefinitionKinematics,
    "RwStepKinematicsRwProductDefinitionKinematics"
);
impl RwStepKinematicsRwProductDefinitionKinematics {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsProductDefinitionKinematics>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_arch, "product_definition_kinematics")? {
            return Ok(());
        }
        rw_block_b_0059!(
            a_property_definition_definition,
            a_property_definition_description,
            a_property_definition_name,
            has_property_definition_description,
            the_arch,
            the_data,
            the_ent,
            the_num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsProductDefinitionKinematics>,
    ) -> Result<(), Failure> {
        rw_block_b_0181!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsProductDefinitionKinematics>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.definition().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwProductDefinitionRelationshipKinematics,
    "RwStepKinematicsRwProductDefinitionRelationshipKinematics"
);
impl RwStepKinematicsRwProductDefinitionRelationshipKinematics {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsProductDefinitionRelationshipKinematics>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            3,
            the_arch,
            "product_definition_relationship_kinematics",
        )? {
            return Ok(());
        }
        rw_block_b_0059!(
            a_property_definition_definition,
            a_property_definition_description,
            a_property_definition_name,
            has_property_definition_description,
            the_arch,
            the_data,
            the_ent,
            the_num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsProductDefinitionRelationshipKinematics>,
    ) -> Result<(), Failure> {
        rw_block_b_0181!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsProductDefinitionRelationshipKinematics>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.definition().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwRackAndPinionPair,
    "RwStepKinematicsRwRackAndPinionPair"
);
impl RwStepKinematicsRwRackAndPinionPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsRackAndPinionPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 7, the_arch, "rack_and_pinion_pair")? {
            return Ok(());
        }
        rw_block_b_0014!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_pinion_radius,
            a_representation_item_name,
            has_item_defined_transformation_description,
            the_arch,
            the_data,
            the_ent,
            the_num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsRackAndPinionPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0071!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsRackAndPinionPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwRackAndPinionPairValue,
    "RwStepKinematicsRwRackAndPinionPairValue"
);
impl RwStepKinematicsRwRackAndPinionPairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsRackAndPinionPairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_arch, "rack_and_pinion_pair_value")? {
            return Ok(());
        }
        rw_block_b_0139!(
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        let mut a_actual_displacement: f64 = Default::default();
        the_data.read_real(
            the_num,
            3,
            "actual_displacement",
            the_arch,
            &mut a_actual_displacement,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            a_actual_displacement,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsRackAndPinionPairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_real(the_ent.actual_displacement())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsRackAndPinionPairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwRackAndPinionPairWithRange,
    "RwStepKinematicsRwRackAndPinionPairWithRange"
);
impl RwStepKinematicsRwRackAndPinionPairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsRackAndPinionPairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 9, the_arch, "rack_and_pinion_pair_with_range")? {
            return Ok(());
        }
        rw_block_b_0022!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_representation_item_name,
            has_item_defined_transformation_description,
            the_arch,
            the_data,
            the_num
        );
        let mut a_rack_and_pinion_pair_pinion_radius: f64 = Default::default();
        the_data.read_real(
            the_num,
            7,
            "rack_and_pinion_pair.pinion_radius",
            the_arch,
            &mut a_rack_and_pinion_pair_pinion_radius,
        )?;
        let (a_lower_limit_rack_displacement, has_lower_limit_rack_displacement) =
            step_read_optional_real(
                the_data,
                the_num,
                8,
                "lower_limit_rack_displacement",
                the_arch,
            )?;
        let (a_upper_limit_rack_displacement, has_upper_limit_rack_displacement) =
            step_read_optional_real(
                the_data,
                the_num,
                9,
                "upper_limit_rack_displacement",
                the_arch,
            )?;
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            a_rack_and_pinion_pair_pinion_radius,
            has_lower_limit_rack_displacement,
            a_lower_limit_rack_displacement,
            has_upper_limit_rack_displacement,
            a_upper_limit_rack_displacement,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsRackAndPinionPairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0071!(the_ent, the_sw);
        step_send_optional_real(
            the_sw,
            the_ent.has_lower_limit_rack_displacement(),
            the_ent.lower_limit_rack_displacement(),
        )?;
        step_send_optional_real(
            the_sw,
            the_ent.has_upper_limit_rack_displacement(),
            the_ent.upper_limit_rack_displacement(),
        )?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsRackAndPinionPairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0175!(iter, the_ent);
        Ok(())
    }
}
