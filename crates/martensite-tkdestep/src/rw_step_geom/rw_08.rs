#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepGeomRwBoundedCurve, "RwStepGeomRwBoundedCurve");
impl RwStepGeomRwBoundedCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBoundedCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "bounded_curve")? {
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
        ent: &Handle<StepGeomBoundedCurve>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwBoundedSurface, "RwStepGeomRwBoundedSurface");
impl RwStepGeomRwBoundedSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBoundedSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "bounded_surface")? {
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
        ent: &Handle<StepGeomBoundedSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwCartesianPoint, "RwStepGeomRwCartesianPoint");
impl RwStepGeomRwCartesianPoint {
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomCartesianPoint>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_coordinates() {
            sw.send_real(ent.coordinates_value(i2)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwCartesianTransformationOperator,
    "RwStepGeomRwCartesianTransformationOperator"
);
impl RwStepGeomRwCartesianTransformationOperator {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomCartesianTransformationOperator>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 7, ach, "cartesian_transformation_operator")? {
            return Ok(());
        }
        rw_block_0018!(
            a_axis1,
            a_axis2,
            a_local_origin,
            a_name,
            a_scale,
            ach,
            data,
            has_aaxis1,
            has_aaxis2,
            has_ascale,
            num
        );
        ent.init(
            &a_name,
            has_aaxis1,
            &a_axis1,
            has_aaxis2,
            &a_axis2,
            &a_local_origin,
            has_ascale,
            a_scale,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomCartesianTransformationOperator>,
    ) -> Result<(), Failure> {
        rw_block_0037!(ent, has_aaxis1, has_aaxis2, has_ascale, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomCartesianTransformationOperator>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0173!(ent, iter);
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwCartesianTransformationOperator3d,
    "RwStepGeomRwCartesianTransformationOperator3d"
);
impl RwStepGeomRwCartesianTransformationOperator3d {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomCartesianTransformationOperator3d>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 8, ach, "cartesian_transformation_operator_3d")? {
            return Ok(());
        }
        rw_block_0018!(
            a_axis1,
            a_axis2,
            a_local_origin,
            a_name,
            a_scale,
            ach,
            data,
            has_aaxis1,
            has_aaxis2,
            has_ascale,
            num
        );
        let mut a_axis3: Handle<StepGeomDirection> = Default::default();
        let mut has_aaxis3: bool = true;
        if data.is_param_defined(num, 8)? {
            data.read_entity(
                num,
                8,
                "axis3",
                ach,
                StepGeomDirection::static_type(),
                &mut a_axis3,
            )?;
        } else {
            has_aaxis3 = false;
            a_axis3 = Default::default();
        }
        ent.init(
            &a_name,
            has_aaxis1,
            &a_axis1,
            has_aaxis2,
            &a_axis2,
            &a_local_origin,
            has_ascale,
            a_scale,
            has_aaxis3,
            &a_axis3,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomCartesianTransformationOperator3d>,
    ) -> Result<(), Failure> {
        rw_block_0037!(ent, has_aaxis1, has_aaxis2, has_ascale, sw);
        let has_aaxis3: bool = ent.has_axis3();
        if has_aaxis3 {
            sw.send_entity(&ent.axis3())?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomCartesianTransformationOperator3d>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0173!(ent, iter);
        if ent.has_axis3() {
            iter.get_one_item(&ent.axis3().cast());
        }
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwCircle, "RwStepGeomRwCircle");
impl RwStepGeomRwCircle {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomCircle>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "circle")? {
            return Ok(());
        }
        rw_block_0153!(a_name, a_position, ach, data, num);
        let mut a_radius: f64 = Default::default();
        data.read_real(num, 3, "radius", ach, &mut a_radius)?;
        ent.init(&a_name, &a_position, a_radius);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomCircle>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.position().value())?;
        sw.send_real(ent.radius())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomCircle>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().value());
        Ok(())
    }
}
