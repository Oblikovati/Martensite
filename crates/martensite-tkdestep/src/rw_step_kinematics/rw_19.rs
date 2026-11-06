#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwSphericalPairWithRange,
    "RwStepKinematicsRwSphericalPairWithRange"
);
impl RwStepKinematicsRwSphericalPairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsSphericalPairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 18, the_arch, "spherical_pair_with_range")? {
            return Ok(());
        }
        rw_block_b_0002!(
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
            a_lower_limit_yaw,
            a_representation_item_name,
            a_upper_limit_yaw,
            has_item_defined_transformation_description,
            has_lower_limit_yaw,
            has_upper_limit_yaw,
            the_arch,
            the_data,
            the_num
        );
        let (a_lower_limit_pitch, has_lower_limit_pitch) =
            step_read_optional_real(the_data, the_num, 15, "lower_limit_pitch", the_arch)?;
        let (a_upper_limit_pitch, has_upper_limit_pitch) =
            step_read_optional_real(the_data, the_num, 16, "upper_limit_pitch", the_arch)?;
        let (a_lower_limit_roll, has_lower_limit_roll) =
            step_read_optional_real(the_data, the_num, 17, "lower_limit_roll", the_arch)?;
        let (a_upper_limit_roll, has_upper_limit_roll) =
            step_read_optional_real(the_data, the_num, 18, "upper_limit_roll", the_arch)?;
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
        the_ent: &Handle<StepKinematicsSphericalPairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0020!(the_ent, the_sw);
        rw_block_b_0127!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsSphericalPairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0175!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwSurfacePairWithRange,
    "RwStepKinematicsRwSurfacePairWithRange"
);
impl RwStepKinematicsRwSurfacePairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsSurfacePairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 13, the_arch, "surface_pair_with_range")? {
            return Ok(());
        }
        rw_block_b_0009!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_representation_item_name,
            a_surface_pair_orientation,
            a_surface_pair_surface1,
            a_surface_pair_surface2,
            has_item_defined_transformation_description,
            the_arch,
            the_data,
            the_num
        );
        let mut a_range_on_surface1: Handle<StepGeomRectangularTrimmedSurface> = Default::default();
        the_data.read_entity(
            the_num,
            10,
            "range_on_surface1",
            the_arch,
            StepGeomRectangularTrimmedSurface::static_type(),
            &mut a_range_on_surface1,
        )?;
        let mut a_range_on_surface2: Handle<StepGeomRectangularTrimmedSurface> = Default::default();
        the_data.read_entity(
            the_num,
            11,
            "range_on_surface2",
            the_arch,
            StepGeomRectangularTrimmedSurface::static_type(),
            &mut a_range_on_surface2,
        )?;
        let (a_lower_limit_actual_rotation, has_lower_limit_actual_rotation) =
            step_read_optional_real(
                the_data,
                the_num,
                12,
                "lower_limit_actual_rotation",
                the_arch,
            )?;
        let (a_upper_limit_actual_rotation, has_upper_limit_actual_rotation) =
            step_read_optional_real(
                the_data,
                the_num,
                13,
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
            &a_surface_pair_surface1,
            &a_surface_pair_surface2,
            a_surface_pair_orientation,
            &a_range_on_surface1,
            &a_range_on_surface2,
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
        the_ent: &Handle<StepKinematicsSurfacePairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0049!(the_ent, the_sw);
        the_sw.send_entity(&the_ent.range_on_surface1())?;
        the_sw.send_entity(&the_ent.range_on_surface2())?;
        rw_block_b_0208!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsSurfacePairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0103!(iter, the_ent);
        iter.add_item(&the_ent.range_on_surface1().cast());
        iter.add_item(&the_ent.range_on_surface2().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwUnconstrainedPair,
    "RwStepKinematicsRwUnconstrainedPair"
);
impl RwStepKinematicsRwUnconstrainedPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsUnconstrainedPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 12, the_arch, "unconstrained_pair")? {
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
        the_ent: &Handle<StepKinematicsUnconstrainedPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0029!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsUnconstrainedPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwUnconstrainedPairValue,
    "RwStepKinematicsRwUnconstrainedPairValue"
);
impl RwStepKinematicsRwUnconstrainedPairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsUnconstrainedPairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_arch, "unconstrained_pair_value")? {
            return Ok(());
        }
        rw_block_b_0139!(
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        let mut a_actual_placement: Handle<StepGeomAxis2Placement3d> = Default::default();
        the_data.read_entity(
            the_num,
            3,
            "actual_placement",
            the_arch,
            StepGeomAxis2Placement3d::static_type(),
            &mut a_actual_placement,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            &a_actual_placement,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsUnconstrainedPairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_entity(&the_ent.actual_placement())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsUnconstrainedPairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        iter.add_item(&the_ent.actual_placement().cast());
        Ok(())
    }
}
