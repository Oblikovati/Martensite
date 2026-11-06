#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepGeomRwDegenerateToroidalSurface,
    "RwStepGeomRwDegenerateToroidalSurface"
);
impl RwStepGeomRwDegenerateToroidalSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomDegenerateToroidalSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "degenerate_toroidal_surface")? {
            return Ok(());
        }
        rw_block_0059!(
            a_major_radius,
            a_minor_radius,
            a_name,
            a_position,
            ach,
            data,
            num
        );
        let mut a_select_outer: bool = Default::default();
        data.read_boolean(num, 5, "select_outer", ach, &mut a_select_outer)?;
        ent.init(
            &a_name,
            &a_position,
            a_major_radius,
            a_minor_radius,
            a_select_outer,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomDegenerateToroidalSurface>,
    ) -> Result<(), Failure> {
        rw_block_0167!(ent, sw);
        sw.send_boolean(ent.select_outer())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomDegenerateToroidalSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwDirection, "RwStepGeomRwDirection");
impl RwStepGeomRwDirection {
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomDirection>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_direction_ratios() {
            sw.send_real(ent.direction_ratios_value(i2)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `Check`.
    pub fn check(
        &self,
        ent: &Handle<StepGeomDirection>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let nb_val: i32 = ent.nb_direction_ratios();
        let i: i32 = Default::default();
        for i in 1..=nb_val {
            if ent.direction_ratios_value(i)?.abs() >= f64::EPSILON {
                break;
            }
        }
        if i > nb_val {
            ach.add_fail_text("ERROR: DirectionRatios all 0.0", "");
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwElementarySurface,
    "RwStepGeomRwElementarySurface"
);
impl RwStepGeomRwElementarySurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomElementarySurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "elementary_surface")? {
            return Ok(());
        }
        rw_block_0107!(a_name, a_position, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomElementarySurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.position())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomElementarySurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwEllipse, "RwStepGeomRwEllipse");
impl RwStepGeomRwEllipse {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomEllipse>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "ellipse")? {
            return Ok(());
        }
        rw_block_0153!(a_name, a_position, ach, data, num);
        let mut a_semi_axis1: f64 = Default::default();
        data.read_real(num, 3, "semi_axis_1", ach, &mut a_semi_axis1)?;
        let mut a_semi_axis2: f64 = Default::default();
        data.read_real(num, 4, "semi_axis_2", ach, &mut a_semi_axis2)?;
        ent.init(&a_name, &a_position, a_semi_axis1, a_semi_axis2);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomEllipse>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.position().value())?;
        sw.send_real(ent.semi_axis1())?;
        sw.send_real(ent.semi_axis2())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomEllipse>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().value());
        Ok(())
    }
    /// `Check`.
    pub fn check(
        &self,
        ent: &Handle<StepGeomEllipse>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.semi_axis1() < ent.semi_axis2() {
            ach.add_warning_text("ERROR: Ellipse: SemiMajor smaller than SemiMinor", "");
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwEvaluatedDegeneratePcurve,
    "RwStepGeomRwEvaluatedDegeneratePcurve"
);
impl RwStepGeomRwEvaluatedDegeneratePcurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomEvaluatedDegeneratePcurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "evaluated_degenerate_pcurve")? {
            return Ok(());
        }
        rw_block_0081!(
            a_basis_surface,
            a_name,
            a_reference_to_curve,
            ach,
            data,
            num
        );
        let mut a_equivalent_point: Handle<StepGeomCartesianPoint> = Default::default();
        data.read_entity(
            num,
            4,
            "equivalent_point",
            ach,
            StepGeomCartesianPoint::static_type(),
            &mut a_equivalent_point,
        )?;
        ent.init(
            &a_name,
            &a_basis_surface,
            &a_reference_to_curve,
            &a_equivalent_point,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomEvaluatedDegeneratePcurve>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.basis_surface())?;
        sw.send_entity(&ent.reference_to_curve())?;
        sw.send_entity(&ent.equivalent_point())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomEvaluatedDegeneratePcurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.basis_surface().cast());
        iter.get_one_item(&ent.reference_to_curve().cast());
        iter.get_one_item(&ent.equivalent_point().cast());
        Ok(())
    }
}
