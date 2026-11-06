#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwPairRepresentationRelationship,
    "RwStepKinematicsRwPairRepresentationRelationship"
);
impl RwStepKinematicsRwPairRepresentationRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPairRepresentationRelationship>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 6, the_arch, "pair_representation_relationship")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(
            the_num,
            1,
            "representation_item.name",
            the_arch,
            &mut a_representation_item_name,
        )?;
        let mut a_representation_relationship_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(
            the_num,
            2,
            "representation_relationship.name",
            the_arch,
            &mut a_representation_relationship_name,
        )?;
        let mut a_representation_relationship_description: Handle<HAsciiString> =
            Default::default();
        let mut has_representation_relationship_description: bool = true;
        if the_data.is_param_defined(the_num, 3)? {
            the_data.read_string(
                the_num,
                3,
                "representation_relationship.description",
                the_arch,
                &mut a_representation_relationship_description,
            )?;
        } else {
            has_representation_relationship_description = false;
            a_representation_relationship_description = Default::default();
        }
        let mut a_representation_relationship_rep1: StepReprRepresentationOrRepresentationReference = Default::default();
        the_data.read_entity_select(
            the_num,
            4,
            "representation_relationship.rep1",
            the_arch,
            &mut a_representation_relationship_rep1,
        )?;
        let mut a_representation_relationship_rep2: StepReprRepresentationOrRepresentationReference = Default::default();
        the_data.read_entity_select(
            the_num,
            5,
            "representation_relationship.rep2",
            the_arch,
            &mut a_representation_relationship_rep2,
        )?;
        let mut a_representation_relationship_with_transformation_transformation_operator: StepReprTransformation = Default::default();
        the_data.read_entity_select(
            the_num,
            6,
            "representation_relationship_with_transformation.transformation_operator",
            the_arch,
            &mut a_representation_relationship_with_transformation_transformation_operator,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_representation_relationship_name,
            has_representation_relationship_description,
            &a_representation_relationship_description,
            &a_representation_relationship_rep1,
            &a_representation_relationship_rep2,
            &a_representation_relationship_with_transformation_transformation_operator,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsPairRepresentationRelationship>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.name())?;
        if step_live(
            &the_ent.representation_relationship_with_transformation(),
            "WriteStep",
        )?
        .has_description()
        {
            the_sw.send_entity(
                &step_live(
                    &the_ent.representation_relationship_with_transformation(),
                    "WriteStep",
                )?
                .description(),
            )?;
        } else {
            the_sw.send_undef()?;
        }
        the_sw.send_entity(
            &step_live(
                &the_ent.representation_relationship_with_transformation(),
                "WriteStep",
            )?
            .rep1(),
        )?;
        the_sw.send_entity(
            &step_live(
                &the_ent.representation_relationship_with_transformation(),
                "WriteStep",
            )?
            .rep2(),
        )?;
        the_sw.send_entity(
            &step_live(
                &the_ent.representation_relationship_with_transformation(),
                "WriteStep",
            )?
            .transformation_operator()
            .value(),
        )?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPairRepresentationRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(
            &step_live(
                &the_ent.representation_relationship_with_transformation(),
                "Share",
            )?
            .rep1()
            .cast(),
        );
        iter.add_item(
            &step_live(
                &the_ent.representation_relationship_with_transformation(),
                "Share",
            )?
            .rep2()
            .cast(),
        );
        iter.add_item(
            &step_live(
                &the_ent.representation_relationship_with_transformation(),
                "Share",
            )?
            .transformation_operator()
            .value(),
        );
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwPlanarCurvePair,
    "RwStepKinematicsRwPlanarCurvePair"
);
impl RwStepKinematicsRwPlanarCurvePair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPlanarCurvePair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 9, the_arch, "planar_curve_pair")? {
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
        let mut a_curve1: Handle<StepGeomCurve> = Default::default();
        the_data.read_entity(
            the_num,
            7,
            "curve1",
            the_arch,
            StepGeomCurve::static_type(),
            &mut a_curve1,
        )?;
        let mut a_curve2: Handle<StepGeomCurve> = Default::default();
        the_data.read_entity(
            the_num,
            8,
            "curve2",
            the_arch,
            StepGeomCurve::static_type(),
            &mut a_curve2,
        )?;
        let mut a_orientation: bool = Default::default();
        the_data.read_boolean(the_num, 9, "orientation", the_arch, &mut a_orientation)?;
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            &a_curve1,
            &a_curve2,
            a_orientation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsPlanarCurvePair>,
    ) -> Result<(), Failure> {
        rw_block_b_0048!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPlanarCurvePair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        iter.add_item(&the_ent.curve1().cast());
        iter.add_item(&the_ent.curve2().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwPlanarCurvePairRange,
    "RwStepKinematicsRwPlanarCurvePairRange"
);
impl RwStepKinematicsRwPlanarCurvePairRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPlanarCurvePairRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 11, the_arch, "planar_curve_pair_range")? {
            return Ok(());
        }
        rw_block_b_0008!(
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
            the_num
        );
        let mut a_range_on_curve1: Handle<StepGeomTrimmedCurve> = Default::default();
        the_data.read_entity(
            the_num,
            10,
            "range_on_curve1",
            the_arch,
            StepGeomTrimmedCurve::static_type(),
            &mut a_range_on_curve1,
        )?;
        let mut a_range_on_curve2: Handle<StepGeomTrimmedCurve> = Default::default();
        the_data.read_entity(
            the_num,
            11,
            "range_on_curve2",
            the_arch,
            StepGeomTrimmedCurve::static_type(),
            &mut a_range_on_curve2,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            &a_planar_curve_pair_curve1,
            &a_planar_curve_pair_curve2,
            a_planar_curve_pair_orientation,
            &a_range_on_curve1,
            &a_range_on_curve2,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsPlanarCurvePairRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0048!(the_ent, the_sw);
        the_sw.send_entity(&the_ent.range_on_curve1())?;
        the_sw.send_entity(&the_ent.range_on_curve2())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPlanarCurvePairRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0102!(iter, the_ent);
        iter.add_item(&the_ent.range_on_curve1().cast());
        iter.add_item(&the_ent.range_on_curve2().cast());
        Ok(())
    }
}

step_rw_class!(RwStepKinematicsRwPlanarPair, "RwStepKinematicsRwPlanarPair");
impl RwStepKinematicsRwPlanarPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsPlanarPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 12, the_arch, "planar_pair")? {
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
        the_ent: &Handle<StepKinematicsPlanarPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0029!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsPlanarPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        Ok(())
    }
}
