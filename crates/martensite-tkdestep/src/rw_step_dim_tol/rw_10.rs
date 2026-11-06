#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepDimTolRwPositionTolerance,
    "RwStepDimTolRwPositionTolerance"
);
impl RwStepDimTolRwPositionTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolPositionTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "position_tolerance")? {
            return Ok(());
        }
        rw_block_b_0052!(
            a_geometric_tolerance_description,
            a_geometric_tolerance_magnitude,
            a_geometric_tolerance_name,
            a_geometric_tolerance_toleranced_shape_aspect,
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
        ent: &Handle<StepDimTolPositionTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0155!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolPositionTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.magnitude());
        iter.add_item(&ent.parent.toleranced_shape_aspect().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwProjectedZoneDefinition,
    "RwStepDimTolRwProjectedZoneDefinition"
);
impl RwStepDimTolRwProjectedZoneDefinition {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolProjectedZoneDefinition>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "projected_zone_definition")? {
            return Ok(());
        }
        rw_block_b_0081!(
            a_tolerance_zone,
            ach,
            an_ent,
            an_items,
            data,
            i,
            nb_elements,
            nb_sub,
            num
        );
        let mut a_projection_end: Handle<StepReprShapeAspect> = Default::default();
        data.read_entity(
            num,
            3,
            "projection_end",
            ach,
            StepReprShapeAspect::static_type(),
            &mut a_projection_end,
        )?;
        let mut a_projection_length: Handle<StepBasicLengthMeasureWithUnit> = Default::default();
        data.read_entity(
            num,
            4,
            "projection_length",
            ach,
            StepBasicLengthMeasureWithUnit::static_type(),
            &mut a_projection_length,
        )?;
        ent.init(
            &a_tolerance_zone,
            &an_items,
            &a_projection_end,
            &a_projection_length,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolProjectedZoneDefinition>,
    ) -> Result<(), Failure> {
        rw_block_b_0164!(ent, i, sw);
        sw.send_entity(&ent.projection_end())?;
        sw.send_entity(&ent.projection_length())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolProjectedZoneDefinition>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0151!(ent, i, iter, nb);
        iter.add_item(&ent.projection_end().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwRoundnessTolerance,
    "RwStepDimTolRwRoundnessTolerance"
);
impl RwStepDimTolRwRoundnessTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolRoundnessTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "roundness_tolerance")? {
            return Ok(());
        }
        rw_block_b_0052!(
            a_geometric_tolerance_description,
            a_geometric_tolerance_magnitude,
            a_geometric_tolerance_name,
            a_geometric_tolerance_toleranced_shape_aspect,
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
        ent: &Handle<StepDimTolRoundnessTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0155!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolRoundnessTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.magnitude());
        iter.add_item(&ent.parent.toleranced_shape_aspect().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwRunoutZoneDefinition,
    "RwStepDimTolRwRunoutZoneDefinition"
);
impl RwStepDimTolRwRunoutZoneDefinition {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolRunoutZoneDefinition>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "runout_zone_definition")? {
            return Ok(());
        }
        rw_block_b_0081!(
            a_tolerance_zone,
            ach,
            an_ent,
            an_items,
            data,
            i,
            nb_elements,
            nb_sub,
            num
        );
        let mut an_orientation: Handle<StepDimTolRunoutZoneOrientation> = Default::default();
        data.read_entity(
            num,
            3,
            "orientation",
            ach,
            StepDimTolRunoutZoneOrientation::static_type(),
            &mut an_orientation,
        )?;
        ent.init(&a_tolerance_zone, &an_items, &an_orientation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolRunoutZoneDefinition>,
    ) -> Result<(), Failure> {
        rw_block_b_0164!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolRunoutZoneDefinition>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0151!(ent, i, iter, nb);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwRunoutZoneOrientation,
    "RwStepDimTolRwRunoutZoneOrientation"
);
impl RwStepDimTolRwRunoutZoneOrientation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolRunoutZoneOrientation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "runout_zone_orientation")? {
            return Ok(());
        }
        let mut an_angle: Handle<StepBasicPlaneAngleMeasureWithUnit> = Default::default();
        data.read_entity(
            num,
            1,
            "angle",
            ach,
            StepBasicPlaneAngleMeasureWithUnit::static_type(),
            &mut an_angle,
        )?;
        ent.init(&an_angle);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolRunoutZoneOrientation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.angle())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwStraightnessTolerance,
    "RwStepDimTolRwStraightnessTolerance"
);
impl RwStepDimTolRwStraightnessTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolStraightnessTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "straightness_tolerance")? {
            return Ok(());
        }
        rw_block_b_0052!(
            a_geometric_tolerance_description,
            a_geometric_tolerance_magnitude,
            a_geometric_tolerance_name,
            a_geometric_tolerance_toleranced_shape_aspect,
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
        ent: &Handle<StepDimTolStraightnessTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0155!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolStraightnessTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.magnitude());
        iter.add_item(&ent.parent.toleranced_shape_aspect().value());
        Ok(())
    }
}
