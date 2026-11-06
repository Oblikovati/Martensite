#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwPointOnSurfacePairWithRange,
    "RwStepKinematicsRwPointOnSurfacePairWithRange"
);
impl RwStepKinematicsRwPointOnSurfacePairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPointOnSurfacePairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 14, the_arch, "point_on_surface_pair_with_range")? {
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
        let mut a_point_on_surface_pair_pair_surface: Handle<StepGeomSurface> = Default::default();
        the_data.read_entity(
            the_num,
            7,
            "point_on_surface_pair.pair_surface",
            the_arch,
            StepGeomSurface::static_type(),
            &mut a_point_on_surface_pair_pair_surface,
        )?;
        let mut a_range_on_pair_surface: Handle<StepGeomRectangularTrimmedSurface> =
            Default::default();
        the_data.read_entity(
            the_num,
            8,
            "range_on_pair_surface",
            the_arch,
            StepGeomRectangularTrimmedSurface::static_type(),
            &mut a_range_on_pair_surface,
        )?;
        let (a_lower_limit_yaw, has_lower_limit_yaw) =
            step_read_optional_real(the_data, the_num, 9, "lower_limit_yaw", the_arch)?;
        let (a_upper_limit_yaw, has_upper_limit_yaw) =
            step_read_optional_real(the_data, the_num, 10, "upper_limit_yaw", the_arch)?;
        let (a_lower_limit_pitch, has_lower_limit_pitch) =
            step_read_optional_real(the_data, the_num, 11, "lower_limit_pitch", the_arch)?;
        let (a_upper_limit_pitch, has_upper_limit_pitch) =
            step_read_optional_real(the_data, the_num, 12, "upper_limit_pitch", the_arch)?;
        let (a_lower_limit_roll, has_lower_limit_roll) =
            step_read_optional_real(the_data, the_num, 13, "lower_limit_roll", the_arch)?;
        let (a_upper_limit_roll, has_upper_limit_roll) =
            step_read_optional_real(the_data, the_num, 14, "upper_limit_roll", the_arch)?;
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            &a_point_on_surface_pair_pair_surface,
            &a_range_on_pair_surface,
            has_lower_limit_yaw,
            a_lower_limit_yaw,
            has_upper_limit_yaw,
            a_upper_limit_yaw,
            has_lower_limit_pitch,
            a_lower_limit_pitch,
            has_upper_limit_pitch,
            a_upper_limit_pitch,
            has_lower_limit_roll,
            a_lower_limit_roll,
            has_upper_limit_roll,
            a_upper_limit_roll,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsPointOnSurfacePairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0070!(the_ent, the_sw);
        the_sw.send_entity(&the_ent.range_on_pair_surface())?;
        rw_block_b_0083!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPointOnSurfacePairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0175!(iter, the_ent);
        iter.add_item(&the_ent.parent.pair_surface().cast());
        iter.add_item(&the_ent.range_on_pair_surface().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwPrismaticPair,
    "RwStepKinematicsRwPrismaticPair"
);
impl RwStepKinematicsRwPrismaticPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPrismaticPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 12, the_arch, "prismatic_pair")? {
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
        the_ent: &Handle<StepKinematicsPrismaticPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0029!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPrismaticPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwPrismaticPairValue,
    "RwStepKinematicsRwPrismaticPairValue"
);
impl RwStepKinematicsRwPrismaticPairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPrismaticPairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_arch, "prismatic_pair_value")? {
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
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            a_actual_translation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsPrismaticPairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_real(the_ent.actual_translation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPrismaticPairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwPrismaticPairWithRange,
    "RwStepKinematicsRwPrismaticPairWithRange"
);
impl RwStepKinematicsRwPrismaticPairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPrismaticPairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 14, the_arch, "prismatic_pair_with_range")? {
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
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsPrismaticPairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0018!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPrismaticPairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0175!(iter, the_ent);
        Ok(())
    }
}
