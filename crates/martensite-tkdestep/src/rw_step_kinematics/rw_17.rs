#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwScrewPairWithRange,
    "RwStepKinematicsRwScrewPairWithRange"
);
impl RwStepKinematicsRwScrewPairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsScrewPairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 9, the_arch, "screw_pair_with_range")? {
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
        let mut a_screw_pair_pitch: f64 = Default::default();
        the_data.read_real(
            the_num,
            7,
            "screw_pair.pitch",
            the_arch,
            &mut a_screw_pair_pitch,
        )?;
        let (a_lower_limit_actual_rotation, has_lower_limit_actual_rotation) =
            step_read_optional_real(
                the_data,
                the_num,
                8,
                "lower_limit_actual_rotation",
                the_arch,
            )?;
        let (a_upper_limit_actual_rotation, has_upper_limit_actual_rotation) =
            step_read_optional_real(
                the_data,
                the_num,
                9,
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
            a_screw_pair_pitch,
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
        the_ent: &Handle<StepKinematicsScrewPairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0072!(the_ent, the_sw);
        rw_block_b_0208!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsScrewPairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0175!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwSlidingCurvePair,
    "RwStepKinematicsRwSlidingCurvePair"
);
impl RwStepKinematicsRwSlidingCurvePair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsSlidingCurvePair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 9, the_arch, "sliding_curve_pair")? {
            return Ok(());
        }
        rw_block_b_0006!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_planar_curve_pair_curve1,
            a_planar_curve_pair_curve2,
            a_planar_curve_pair_orientation,
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
        the_ent: &Handle<StepKinematicsSlidingCurvePair>,
    ) -> Result<(), Failure> {
        rw_block_b_0048!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsSlidingCurvePair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0102!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwSlidingCurvePairValue,
    "RwStepKinematicsRwSlidingCurvePairValue"
);
impl RwStepKinematicsRwSlidingCurvePairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsSlidingCurvePairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_arch, "sliding_curve_pair_value")? {
            return Ok(());
        }
        rw_block_b_0090!(
            a_actual_point_on_curve1,
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        let mut a_actual_point_on_curve2: Handle<StepGeomPointOnCurve> = Default::default();
        the_data.read_entity(
            the_num,
            4,
            "actual_point_on_curve2",
            the_arch,
            StepGeomPointOnCurve::static_type(),
            &mut a_actual_point_on_curve2,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            &a_actual_point_on_curve1,
            &a_actual_point_on_curve2,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsSlidingCurvePairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_entity(&the_ent.actual_point_on_curve1())?;
        the_sw.send_entity(&the_ent.actual_point_on_curve2())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsSlidingCurvePairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        iter.add_item(&the_ent.actual_point_on_curve1().cast());
        iter.add_item(&the_ent.actual_point_on_curve2().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwSlidingSurfacePair,
    "RwStepKinematicsRwSlidingSurfacePair"
);
impl RwStepKinematicsRwSlidingSurfacePair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsSlidingSurfacePair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 9, the_arch, "sliding_surface_pair")? {
            return Ok(());
        }
        rw_block_b_0007!(
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
            the_ent,
            the_num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsSlidingSurfacePair>,
    ) -> Result<(), Failure> {
        rw_block_b_0049!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsSlidingSurfacePair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0103!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwSlidingSurfacePairValue,
    "RwStepKinematicsRwSlidingSurfacePairValue"
);
impl RwStepKinematicsRwSlidingSurfacePairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsSlidingSurfacePairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 5, the_arch, "sliding_surface_pair_value")? {
            return Ok(());
        }
        rw_block_b_0139!(
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        let mut a_actual_point_on_surface1: Handle<StepGeomPointOnSurface> = Default::default();
        the_data.read_entity(
            the_num,
            3,
            "actual_point_on_surface1",
            the_arch,
            StepGeomPointOnSurface::static_type(),
            &mut a_actual_point_on_surface1,
        )?;
        let mut a_actual_point_on_surface2: Handle<StepGeomPointOnSurface> = Default::default();
        the_data.read_entity(
            the_num,
            4,
            "actual_point_on_surface2",
            the_arch,
            StepGeomPointOnSurface::static_type(),
            &mut a_actual_point_on_surface2,
        )?;
        let mut a_actual_rotation: f64 = Default::default();
        the_data.read_real(
            the_num,
            5,
            "actual_rotation",
            the_arch,
            &mut a_actual_rotation,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            &a_actual_point_on_surface1,
            &a_actual_point_on_surface2,
            a_actual_rotation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsSlidingSurfacePairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_entity(&the_ent.actual_point_on_surface1())?;
        the_sw.send_entity(&the_ent.actual_point_on_surface2())?;
        the_sw.send_real(the_ent.actual_rotation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsSlidingSurfacePairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        iter.add_item(&the_ent.actual_point_on_surface1().cast());
        iter.add_item(&the_ent.actual_point_on_surface2().cast());
        Ok(())
    }
}
