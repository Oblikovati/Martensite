#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepGeomRwCurveBoundedSurface,
    "RwStepGeomRwCurveBoundedSurface"
);
impl RwStepGeomRwCurveBoundedSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomCurveBoundedSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "curve_bounded_surface")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "representation_item.name",
            ach,
            &mut a_representation_item_name,
        )?;
        let mut a_basis_surface: Handle<StepGeomSurface> = Default::default();
        data.read_entity(
            num,
            2,
            "basis_surface",
            ach,
            StepGeomSurface::static_type(),
            &mut a_basis_surface,
        )?;
        let mut a_boundaries: Handle<HArray1<StepGeomSurfaceBoundary>> = Default::default();
        let mut sub3: i32 = 0;
        if data.read_sub_list(num, 3, "boundaries", ach, &mut sub3, false, 0, 0)? {
            let num2: i32 = sub3;
            let nb0: i32 = data.nb_params(num2)?;
            a_boundaries = step_new_array1::<StepGeomSurfaceBoundary>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: StepGeomSurfaceBoundary = Default::default();
                data.read_entity_select(num2, i0, "boundaries", ach, &mut an_it0)?;
                step_array1_set(&a_boundaries, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        let mut a_implicit_outer: bool = Default::default();
        data.read_boolean(num, 4, "implicit_outer", ach, &mut a_implicit_outer)?;
        ent.init(
            &a_representation_item_name,
            &a_basis_surface,
            &a_boundaries,
            a_implicit_outer,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomCurveBoundedSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.parent.parent.name())?;
        sw.send_entity(&ent.basis_surface())?;
        sw.open_sub()?;
        for i2 in 1..=step_live(&ent.boundaries(), "WriteStep")?.array1().length() {
            let var0: StepGeomSurfaceBoundary =
                step_array1_item(&ent.boundaries(), i2, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        sw.send_boolean(ent.implicit_outer())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomCurveBoundedSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.basis_surface().cast());
        for i2 in 1..=step_live(&ent.boundaries(), "Share")?.array1().length() {
            let var0: StepGeomSurfaceBoundary = step_array1_item(&ent.boundaries(), i2, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwCurveReplica, "RwStepGeomRwCurveReplica");
impl RwStepGeomRwCurveReplica {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomCurveReplica>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "curve_replica")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_parent_curve: Handle<StepGeomCurve> = Default::default();
        data.read_entity(
            num,
            2,
            "parent_curve",
            ach,
            StepGeomCurve::static_type(),
            &mut a_parent_curve,
        )?;
        let mut a_transformation: Handle<StepGeomCartesianTransformationOperator> =
            Default::default();
        data.read_entity(
            num,
            3,
            "transformation",
            ach,
            StepGeomCartesianTransformationOperator::static_type(),
            &mut a_transformation,
        )?;
        ent.init(&a_name, &a_parent_curve, &a_transformation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomCurveReplica>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.parent_curve())?;
        sw.send_entity(&ent.transformation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomCurveReplica>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.parent_curve().cast());
        iter.get_one_item(&ent.transformation().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwCylindricalSurface,
    "RwStepGeomRwCylindricalSurface"
);
impl RwStepGeomRwCylindricalSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomCylindricalSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "cylindrical_surface")? {
            return Ok(());
        }
        rw_block_0071!(a_name, a_position, a_radius, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomCylindricalSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.position())?;
        sw.send_real(ent.radius())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomCylindricalSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwDegeneratePcurve, "RwStepGeomRwDegeneratePcurve");
impl RwStepGeomRwDegeneratePcurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomDegeneratePcurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "degenerate_pcurve")? {
            return Ok(());
        }
        rw_block_0067!(
            a_basis_surface,
            a_name,
            a_reference_to_curve,
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
        ent: &Handle<StepGeomDegeneratePcurve>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.basis_surface())?;
        sw.send_entity(&ent.reference_to_curve())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomDegeneratePcurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.basis_surface().cast());
        iter.get_one_item(&ent.reference_to_curve().cast());
        Ok(())
    }
}
