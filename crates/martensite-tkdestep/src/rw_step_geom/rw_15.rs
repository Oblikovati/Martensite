#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepGeomRwPoint, "RwStepGeomRwPoint");
impl RwStepGeomRwPoint {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomPoint>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "point")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        ent.init(&a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomPoint>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwPointOnCurve, "RwStepGeomRwPointOnCurve");
impl RwStepGeomRwPointOnCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomPointOnCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "point_on_curve")? {
            return Ok(());
        }
        rw_block_0132!(a_basis_curve, a_name, ach, data, num);
        let mut a_point_parameter: f64 = Default::default();
        data.read_real(num, 3, "point_parameter", ach, &mut a_point_parameter)?;
        ent.init(&a_name, &a_basis_curve, a_point_parameter);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomPointOnCurve>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.basis_curve())?;
        sw.send_real(ent.point_parameter())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomPointOnCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.basis_curve().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwPointOnSurface, "RwStepGeomRwPointOnSurface");
impl RwStepGeomRwPointOnSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomPointOnSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "point_on_surface")? {
            return Ok(());
        }
        rw_block_0133!(a_basis_surface, a_name, ach, data, num);
        let mut a_point_parameter_u: f64 = Default::default();
        data.read_real(num, 3, "point_parameter_u", ach, &mut a_point_parameter_u)?;
        let mut a_point_parameter_v: f64 = Default::default();
        data.read_real(num, 4, "point_parameter_v", ach, &mut a_point_parameter_v)?;
        ent.init(
            &a_name,
            &a_basis_surface,
            a_point_parameter_u,
            a_point_parameter_v,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomPointOnSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.basis_surface())?;
        sw.send_real(ent.point_parameter_u())?;
        sw.send_real(ent.point_parameter_v())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomPointOnSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.basis_surface().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwPointReplica, "RwStepGeomRwPointReplica");
impl RwStepGeomRwPointReplica {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomPointReplica>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "point_replica")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_parent_pt: Handle<StepGeomPoint> = Default::default();
        data.read_entity(
            num,
            2,
            "parent_pt",
            ach,
            StepGeomPoint::static_type(),
            &mut a_parent_pt,
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
        ent.init(&a_name, &a_parent_pt, &a_transformation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomPointReplica>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.parent_pt())?;
        sw.send_entity(&ent.transformation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomPointReplica>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.parent_pt().cast());
        iter.get_one_item(&ent.transformation().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwPolyline, "RwStepGeomRwPolyline");
impl RwStepGeomRwPolyline {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomPolyline>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "polyline")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_points: Handle<HArray1<Handle<StepGeomCartesianPoint>>> = Default::default();
        let mut anent2: Handle<StepGeomCartesianPoint> = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "points", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_points = step_new_array1::<Handle<StepGeomCartesianPoint>>(1, nb2)?;
            for i2 in 1..=nb2 {
                if data.read_entity(
                    nsub2,
                    i2,
                    "cartesian_point",
                    ach,
                    StepGeomCartesianPoint::static_type(),
                    &mut anent2,
                )? {
                    step_array1_set(&a_points, i2, anent2.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_points);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomPolyline>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_points() {
            sw.send_entity(&ent.points_value(i2)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomPolyline>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_points();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.points_value(is1)?.cast());
        }
        Ok(())
    }
}
