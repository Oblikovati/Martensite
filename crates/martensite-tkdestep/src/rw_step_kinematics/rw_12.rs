#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwPointOnPlanarCurvePairWithRange,
    "RwStepKinematicsRwPointOnPlanarCurvePairWithRange"
);
impl RwStepKinematicsRwPointOnPlanarCurvePairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPointOnPlanarCurvePairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            15,
            the_arch,
            "point_on_planar_curve_pair_with_range",
        )? {
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
        let mut a_point_on_planar_curve_pair_pair_curve: Handle<StepGeomCurve> = Default::default();
        the_data.read_entity(
            the_num,
            7,
            "point_on_planar_curve_pair.pair_curve",
            the_arch,
            StepGeomCurve::static_type(),
            &mut a_point_on_planar_curve_pair_pair_curve,
        )?;
        let mut a_point_on_planar_curve_pair_orientation: bool = Default::default();
        the_data.read_boolean(
            the_num,
            8,
            "point_on_planar_curve_pair.orientation",
            the_arch,
            &mut a_point_on_planar_curve_pair_orientation,
        )?;
        let mut a_range_on_pair_curve: Handle<StepGeomTrimmedCurve> = Default::default();
        the_data.read_entity(
            the_num,
            9,
            "range_on_pair_curve",
            the_arch,
            StepGeomTrimmedCurve::static_type(),
            &mut a_range_on_pair_curve,
        )?;
        let mut a_lower_limit_yaw: f64 = Default::default();
        let mut has_lower_limit_yaw: bool = true;
        if the_data.is_param_defined(the_num, 10)? {
            the_data.read_real(
                the_num,
                10,
                "lower_limit_yaw",
                the_arch,
                &mut a_lower_limit_yaw,
            )?;
        } else {
            has_lower_limit_yaw = false;
            a_lower_limit_yaw = 0.0;
        }
        let mut a_upper_limit_yaw: f64 = Default::default();
        let mut has_upper_limit_yaw: bool = true;
        if the_data.is_param_defined(the_num, 11)? {
            the_data.read_real(
                the_num,
                11,
                "upper_limit_yaw",
                the_arch,
                &mut a_upper_limit_yaw,
            )?;
        } else {
            has_upper_limit_yaw = false;
            a_upper_limit_yaw = 0.0;
        }
        let mut a_lower_limit_pitch: f64 = Default::default();
        let mut has_lower_limit_pitch: bool = true;
        if the_data.is_param_defined(the_num, 12)? {
            the_data.read_real(
                the_num,
                12,
                "lower_limit_pitch",
                the_arch,
                &mut a_lower_limit_pitch,
            )?;
        } else {
            has_lower_limit_pitch = false;
            a_lower_limit_pitch = 0.0;
        }
        let mut a_upper_limit_pitch: f64 = Default::default();
        let mut has_upper_limit_pitch: bool = true;
        if the_data.is_param_defined(the_num, 13)? {
            the_data.read_real(
                the_num,
                13,
                "upper_limit_pitch",
                the_arch,
                &mut a_upper_limit_pitch,
            )?;
        } else {
            has_upper_limit_pitch = false;
            a_upper_limit_pitch = 0.0;
        }
        let mut a_lower_limit_roll: f64 = Default::default();
        let mut has_lower_limit_roll: bool = true;
        if the_data.is_param_defined(the_num, 14)? {
            the_data.read_real(
                the_num,
                14,
                "lower_limit_roll",
                the_arch,
                &mut a_lower_limit_roll,
            )?;
        } else {
            has_lower_limit_roll = false;
            a_lower_limit_roll = 0.0;
        }
        let mut a_upper_limit_roll: f64 = Default::default();
        let mut has_upper_limit_roll: bool = true;
        if the_data.is_param_defined(the_num, 15)? {
            the_data.read_real(
                the_num,
                15,
                "upper_limit_roll",
                the_arch,
                &mut a_upper_limit_roll,
            )?;
        } else {
            has_upper_limit_roll = false;
            a_upper_limit_roll = 0.0;
        }
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            &a_point_on_planar_curve_pair_pair_curve,
            a_point_on_planar_curve_pair_orientation,
            &a_range_on_pair_curve,
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
        the_ent: &Handle<StepKinematicsPointOnPlanarCurvePairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0057!(the_ent, the_sw);
        the_sw.send_entity(&the_ent.range_on_pair_curve())?;
        rw_block_b_0083!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPointOnPlanarCurvePairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0175!(iter, the_ent);
        iter.add_item(&the_ent.parent.pair_curve().cast());
        iter.add_item(&the_ent.range_on_pair_curve().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwPointOnSurfacePair,
    "RwStepKinematicsRwPointOnSurfacePair"
);
impl RwStepKinematicsRwPointOnSurfacePair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPointOnSurfacePair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 7, the_arch, "point_on_surface_pair")? {
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
        let mut a_pair_surface: Handle<StepGeomSurface> = Default::default();
        the_data.read_entity(
            the_num,
            7,
            "pair_surface",
            the_arch,
            StepGeomSurface::static_type(),
            &mut a_pair_surface,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            &a_pair_surface,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsPointOnSurfacePair>,
    ) -> Result<(), Failure> {
        rw_block_b_0070!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPointOnSurfacePair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        iter.add_item(&the_ent.pair_surface().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwPointOnSurfacePairValue,
    "RwStepKinematicsRwPointOnSurfacePairValue"
);
impl RwStepKinematicsRwPointOnSurfacePairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPointOnSurfacePairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_arch, "point_on_surface_pair_value")? {
            return Ok(());
        }
        rw_block_b_0089!(
            a_actual_point_on_surface,
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
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
            &a_actual_point_on_surface,
            &a_input_orientation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsPointOnSurfacePairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_entity(&the_ent.actual_point_on_surface())?;
        rw_block_b_0218!(i, the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPointOnSurfacePairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        iter.add_item(&the_ent.actual_point_on_surface().cast());
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
