#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepDimTolRwTotalRunoutTolerance,
    "RwStepDimTolRwTotalRunoutTolerance"
);
impl RwStepDimTolRwTotalRunoutTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolTotalRunoutTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "total_runout_tolerance")? {
            return Ok(());
        }
        rw_block_b_0027!(
            a_geometric_tolerance_description,
            a_geometric_tolerance_magnitude,
            a_geometric_tolerance_name,
            a_geometric_tolerance_toleranced_shape_aspect,
            a_geometric_tolerance_with_datum_reference_datum_system,
            ach,
            an_it0,
            data,
            ent,
            i0,
            nb0,
            num,
            num2,
            sub5
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolTotalRunoutTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0073!(ent, i4, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolTotalRunoutTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0173!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwUnequallyDisposedGeometricTolerance,
    "RwStepDimTolRwUnequallyDisposedGeometricTolerance"
);
impl RwStepDimTolRwUnequallyDisposedGeometricTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolUnequallyDisposedGeometricTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "unequally_disposed_geometric_tolerance")? {
            return Ok(());
        }
        rw_block_b_0061!(
            a_description,
            a_magnitude,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            data,
            num
        );
        let mut a_displacement: Handle<StepBasicLengthMeasureWithUnit> = Default::default();
        data.read_entity(
            num,
            5,
            "displacement",
            ach,
            StepBasicLengthMeasureWithUnit::static_type(),
            &mut a_displacement,
        )?;
        ent.init(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
            &a_displacement,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolUnequallyDisposedGeometricTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0161!(ent, sw);
        sw.send_entity(&ent.displacement())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolUnequallyDisposedGeometricTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.magnitude());
        iter.add_item(&ent.toleranced_shape_aspect().value());
        Ok(())
    }
}
