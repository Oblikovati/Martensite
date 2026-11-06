#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol,
    "RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol"
);
impl RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol>,
    ) -> Result<(), Failure> {
        rw_block_b_0013!(
            a_datum_system,
            a_description,
            a_gtwdr,
            a_magnitude,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            an_it0,
            data,
            i0,
            nb0,
            num,
            num0,
            num2,
            sub5
        );
        data.named_for_complex(
            "UNEQUALLY_DISPOSED_GEOMETRIC_TOLERANCE",
            num0,
            &mut num,
            ach,
        );
        let mut a_displacement: Handle<StepBasicLengthMeasureWithUnit> = Default::default();
        data.read_entity(
            num,
            1,
            "displacement",
            ach,
            StepBasicLengthMeasureWithUnit::static_type(),
            &mut a_displacement,
        )?;
        let an_udgt: Handle<StepDimTolUnequallyDisposedGeometricTolerance> =
            Handle::new(StepDimTolUnequallyDisposedGeometricTolerance::new());
        an_udgt.set_displacement(&a_displacement);
        let a_first: Vec<u8> = data.first_complex_type(num0)?;
        let a_last: Vec<u8> = data.third_complex_type(num0)?;
        rw_block_b_0200!(a_first, a_last, a_type, ach);
        ent.init_geometric_tolerance_target(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
            &a_gtwdr,
            a_type,
            &an_udgt,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol>,
    ) -> Result<(), Failure> {
        rw_block_b_0024!(a_type, ent, i4, sw, var0);
        step_start_entity(sw, "UNEQUALLY_DISPOSED_GEOMETRIC_TOLRANCE")?;
        sw.send_entity(
            &step_live(
                &ent.get_unequally_disposed_geometric_tolerance(),
                "WriteStep",
            )?
            .displacement(),
        )?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0168!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwGeoTolAndGeoTolWthMaxTol,
    "RwStepDimTolRwGeoTolAndGeoTolWthMaxTol"
);
impl RwStepDimTolRwGeoTolAndGeoTolWthMaxTol {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthMaxTol>,
    ) -> Result<(), Failure> {
        rw_block_b_0033!(
            a_description,
            a_magnitude,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            data,
            num,
            num0
        );
        rw_block_b_0021!(
            a_first,
            a_gtwm,
            a_last,
            a_max_tol,
            a_modifiers,
            a_type,
            ach,
            an_it0,
            data,
            i0,
            nb0,
            num,
            num0,
            num2,
            sub,
            text
        );
        ent.init_geometric_tolerance_target(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
            &a_gtwm,
            &a_max_tol,
            a_type,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthMaxTol>,
    ) -> Result<(), Failure> {
        rw_block_b_0047!(a_type, ent, sw);
        rw_block_b_0078!(a_gtwm, a_type, ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthMaxTol>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.magnitude());
        iter.add_item(&ent.toleranced_shape_aspect().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwGeoTolAndGeoTolWthMod,
    "RwStepDimTolRwGeoTolAndGeoTolWthMod"
);
impl RwStepDimTolRwGeoTolAndGeoTolWthMod {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthMod>,
    ) -> Result<(), Failure> {
        rw_block_b_0033!(
            a_description,
            a_magnitude,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            data,
            num,
            num0
        );
        rw_block_b_0034!(
            a_first,
            a_gtwm,
            a_last,
            a_modifiers,
            a_type,
            ach,
            an_it0,
            data,
            i0,
            nb0,
            num,
            num0,
            num2,
            sub,
            text
        );
        ent.init_geometric_tolerance_target(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
            &a_gtwm,
            a_type,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthMod>,
    ) -> Result<(), Failure> {
        rw_block_b_0068!(a_type, ent, sw);
        rw_block_b_0078!(a_gtwm, a_type, ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthMod>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.magnitude());
        iter.add_item(&ent.toleranced_shape_aspect().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwGeometricTolerance,
    "RwStepDimTolRwGeometricTolerance"
);
impl RwStepDimTolRwGeometricTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeometricTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0051!(
            a_description,
            a_magnitude,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            data,
            num
        );
        ent.init_geometric_tolerance_target(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeometricTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0161!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeometricTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.magnitude());
        iter.add_item(&ent.toleranced_shape_aspect().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwGeometricToleranceRelationship,
    "RwStepDimTolRwGeometricToleranceRelationship"
);
impl RwStepDimTolRwGeometricToleranceRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeometricToleranceRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "geometric_tolerance_relationship")? {
            return Ok(());
        }
        rw_block_b_0147!(a_description, a_name, ach, data, num);
        let mut a_relating_geometric_tolerance: Handle<StepDimTolGeometricTolerance> =
            Default::default();
        data.read_entity(
            num,
            3,
            "relating_geometric_tolerance",
            ach,
            StepDimTolGeometricTolerance::static_type(),
            &mut a_relating_geometric_tolerance,
        )?;
        let mut a_related_geometric_tolerance: Handle<StepDimTolGeometricTolerance> =
            Default::default();
        data.read_entity(
            num,
            4,
            "related_geometric_tolerance",
            ach,
            StepDimTolGeometricTolerance::static_type(),
            &mut a_related_geometric_tolerance,
        )?;
        ent.init(
            &a_name,
            &a_description,
            &a_relating_geometric_tolerance,
            &a_related_geometric_tolerance,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeometricToleranceRelationship>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        sw.send_entity(&ent.relating_geometric_tolerance())?;
        sw.send_entity(&ent.related_geometric_tolerance())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeometricToleranceRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.relating_geometric_tolerance().cast());
        iter.add_item(&ent.related_geometric_tolerance().cast());
        Ok(())
    }
}
