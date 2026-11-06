#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwPlanarPairValue,
    "RwStepKinematicsRwPlanarPairValue"
);
impl RwStepKinematicsRwPlanarPairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPlanarPairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 5, the_arch, "planar_pair_value")? {
            return Ok(());
        }
        rw_block_b_0096!(
            a_actual_rotation,
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        let mut a_actual_translation_x: f64 = Default::default();
        the_data.read_real(
            the_num,
            4,
            "actual_translation_x",
            the_arch,
            &mut a_actual_translation_x,
        )?;
        let mut a_actual_translation_y: f64 = Default::default();
        the_data.read_real(
            the_num,
            5,
            "actual_translation_y",
            the_arch,
            &mut a_actual_translation_y,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            a_actual_rotation,
            a_actual_translation_x,
            a_actual_translation_y,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsPlanarPairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_real(the_ent.actual_rotation())?;
        the_sw.send_real(the_ent.actual_translation_x())?;
        the_sw.send_real(the_ent.actual_translation_y())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPlanarPairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwPlanarPairWithRange,
    "RwStepKinematicsRwPlanarPairWithRange"
);
impl RwStepKinematicsRwPlanarPairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPlanarPairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 18, the_arch, "planar_pair_with_range")? {
            return Ok(());
        }
        rw_block_b_0001!(
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
            a_lower_limit_actual_rotation,
            a_representation_item_name,
            a_upper_limit_actual_rotation,
            has_item_defined_transformation_description,
            has_lower_limit_actual_rotation,
            has_upper_limit_actual_rotation,
            the_arch,
            the_data,
            the_num
        );
        let (a_lower_limit_actual_translation_x, has_lower_limit_actual_translation_x) =
            step_read_optional_real(
                the_data,
                the_num,
                15,
                "lower_limit_actual_translation_x",
                the_arch,
            )?;
        let (a_upper_limit_actual_translation_x, has_upper_limit_actual_translation_x) =
            step_read_optional_real(
                the_data,
                the_num,
                16,
                "upper_limit_actual_translation_x",
                the_arch,
            )?;
        let (a_lower_limit_actual_translation_y, has_lower_limit_actual_translation_y) =
            step_read_optional_real(
                the_data,
                the_num,
                17,
                "lower_limit_actual_translation_y",
                the_arch,
            )?;
        let (a_upper_limit_actual_translation_y, has_upper_limit_actual_translation_y) =
            step_read_optional_real(
                the_data,
                the_num,
                18,
                "upper_limit_actual_translation_y",
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
            has_lower_limit_actual_rotation,
            a_lower_limit_actual_rotation,
            has_upper_limit_actual_rotation,
            a_upper_limit_actual_rotation,
            has_lower_limit_actual_translation_x,
            a_lower_limit_actual_translation_x,
            has_upper_limit_actual_translation_x,
            a_upper_limit_actual_translation_x,
            has_lower_limit_actual_translation_y,
            a_lower_limit_actual_translation_y,
            has_upper_limit_actual_translation_y,
            a_upper_limit_actual_translation_y,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsPlanarPairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0019!(the_ent, the_sw);
        rw_block_b_0126!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPlanarPairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0175!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwPointOnPlanarCurvePair,
    "RwStepKinematicsRwPointOnPlanarCurvePair"
);
impl RwStepKinematicsRwPointOnPlanarCurvePair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPointOnPlanarCurvePair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 8, the_arch, "point_on_planar_curve_pair")? {
            return Ok(());
        }
        rw_block_b_0011!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_orientation,
            a_pair_curve,
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
        the_ent: &Handle<StepKinematicsPointOnPlanarCurvePair>,
    ) -> Result<(), Failure> {
        rw_block_b_0057!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPointOnPlanarCurvePair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0129!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwPointOnPlanarCurvePairValue,
    "RwStepKinematicsRwPointOnPlanarCurvePairValue"
);
impl RwStepKinematicsRwPointOnPlanarCurvePairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPointOnPlanarCurvePairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_arch, "point_on_planar_curve_pair_value")? {
            return Ok(());
        }
        rw_block_b_0139!(
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        let mut a_actual_point_on_curve: Handle<StepGeomPointOnCurve> = Default::default();
        the_data.read_entity(
            the_num,
            3,
            "actual_point_on_curve",
            the_arch,
            StepGeomPointOnCurve::static_type(),
            &mut a_actual_point_on_curve,
        )?;
        rw_block_b_0201!(
            a_input_orientation,
            a_items,
            an_it0,
            i0,
            nb,
            nsub,
            num2,
            the_arch,
            the_data,
            the_num
        );
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            &a_actual_point_on_curve,
            &a_input_orientation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsPointOnPlanarCurvePairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_entity(&the_ent.actual_point_on_curve())?;
        rw_block_b_0218!(i, the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPointOnPlanarCurvePairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        iter.add_item(&the_ent.actual_point_on_curve().cast());
        if !the_ent
            .input_orientation()
            .rotation_about_direction()
            .is_null()
        {
            iter.add_item(&the_ent.input_orientation().value());
        }
        Ok(())
    }
}
