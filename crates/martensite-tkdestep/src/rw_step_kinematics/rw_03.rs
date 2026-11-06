#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwContextDependentKinematicLinkRepresentation,
    "RwStepKinematicsRwContextDependentKinematicLinkRepresentation"
);
impl RwStepKinematicsRwContextDependentKinematicLinkRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsContextDependentKinematicLinkRepresentation>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            2,
            the_arch,
            "context_dependent_kinematic_link_representation",
        )? {
            return Ok(());
        }
        let mut a_representation_relation: Handle<
            StepKinematicsKinematicLinkRepresentationAssociation,
        > = Default::default();
        the_data.read_entity(
            the_num,
            1,
            "representation_relation",
            the_arch,
            StepKinematicsKinematicLinkRepresentationAssociation::static_type(),
            &mut a_representation_relation,
        )?;
        let mut a_represented_product_relation: Handle<
            StepKinematicsProductDefinitionRelationshipKinematics,
        > = Default::default();
        the_data.read_entity(
            the_num,
            2,
            "represented_product_relation",
            the_arch,
            StepKinematicsProductDefinitionRelationshipKinematics::static_type(),
            &mut a_represented_product_relation,
        )?;
        the_ent.init(&a_representation_relation, &a_represented_product_relation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsContextDependentKinematicLinkRepresentation>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.representation_relation())?;
        the_sw.send_entity(&the_ent.represented_product_relation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsContextDependentKinematicLinkRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.representation_relation().cast());
        iter.add_item(&the_ent.represented_product_relation().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwCylindricalPair,
    "RwStepKinematicsRwCylindricalPair"
);
impl RwStepKinematicsRwCylindricalPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsCylindricalPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 12, the_arch, "cylindrical_pair")? {
            return Ok(());
        }
        rw_block_b_0004!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_low_order_kinematic_pair_rx,
            a_low_order_kinematic_pair_ry,
            a_low_order_kinematic_pair_rz,
            a_low_order_kinematic_pair_tx,
            a_low_order_kinematic_pair_ty,
            a_low_order_kinematic_pair_tz,
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
        the_ent: &Handle<StepKinematicsCylindricalPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0029!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsCylindricalPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwCylindricalPairValue,
    "RwStepKinematicsRwCylindricalPairValue"
);
impl RwStepKinematicsRwCylindricalPairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsCylindricalPairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_arch, "cylindrical_pair_value")? {
            return Ok(());
        }
        rw_block_b_0095!(
            a_actual_translation,
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        let mut a_actual_rotation: f64 = Default::default();
        the_data.read_real(
            the_num,
            4,
            "actual_rotation",
            the_arch,
            &mut a_actual_rotation,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            a_actual_translation,
            a_actual_rotation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsCylindricalPairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_real(the_ent.actual_translation())?;
        the_sw.send_real(the_ent.actual_rotation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsCylindricalPairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwCylindricalPairWithRange,
    "RwStepKinematicsRwCylindricalPairWithRange"
);
impl RwStepKinematicsRwCylindricalPairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsCylindricalPairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 16, the_arch, "cylindrical_pair_with_range")? {
            return Ok(());
        }
        rw_block_b_0000!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_low_order_kinematic_pair_rx,
            a_low_order_kinematic_pair_ry,
            a_low_order_kinematic_pair_rz,
            a_low_order_kinematic_pair_tx,
            a_low_order_kinematic_pair_ty,
            a_low_order_kinematic_pair_tz,
            a_lower_limit_actual_translation,
            a_representation_item_name,
            a_upper_limit_actual_translation,
            has_item_defined_transformation_description,
            has_lower_limit_actual_translation,
            has_upper_limit_actual_translation,
            the_arch,
            the_data,
            the_num
        );
        let (a_lower_limit_actual_rotation, has_lower_limit_actual_rotation) =
            step_read_optional_real(
                the_data,
                the_num,
                15,
                "lower_limit_actual_rotation",
                the_arch,
            )?;
        let (a_upper_limit_actual_rotation, has_upper_limit_actual_rotation) =
            step_read_optional_real(
                the_data,
                the_num,
                16,
                "upper_limit_actual_rotation",
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
            a_low_order_kinematic_pair_tx,
            a_low_order_kinematic_pair_ty,
            a_low_order_kinematic_pair_tz,
            a_low_order_kinematic_pair_rx,
            a_low_order_kinematic_pair_ry,
            a_low_order_kinematic_pair_rz,
            has_lower_limit_actual_translation,
            a_lower_limit_actual_translation,
            has_upper_limit_actual_translation,
            a_upper_limit_actual_translation,
            has_lower_limit_actual_rotation,
            a_lower_limit_actual_rotation,
            has_upper_limit_actual_rotation,
            a_upper_limit_actual_rotation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsCylindricalPairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0018!(the_ent, the_sw);
        rw_block_b_0208!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsCylindricalPairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0175!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwFullyConstrainedPair,
    "RwStepKinematicsRwFullyConstrainedPair"
);
impl RwStepKinematicsRwFullyConstrainedPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsFullyConstrainedPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 12, the_arch, "fully_constrained_pair")? {
            return Ok(());
        }
        rw_block_b_0004!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_low_order_kinematic_pair_rx,
            a_low_order_kinematic_pair_ry,
            a_low_order_kinematic_pair_rz,
            a_low_order_kinematic_pair_tx,
            a_low_order_kinematic_pair_ty,
            a_low_order_kinematic_pair_tz,
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
        the_ent: &Handle<StepKinematicsFullyConstrainedPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0029!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsFullyConstrainedPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        Ok(())
    }
}
