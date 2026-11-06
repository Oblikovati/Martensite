#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepGeomRwUniformSurface, "RwStepGeomRwUniformSurface");
impl RwStepGeomRwUniformSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomUniformSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 8, ach, "uniform_surface")? {
            return Ok(());
        }
        rw_block_0008!(
            a_control_points_list,
            a_name,
            a_self_intersect,
            a_surface_form,
            a_u_closed,
            a_u_degree,
            a_v_closed,
            a_v_degree,
            ach,
            anent4,
            data,
            ent,
            i4,
            j4,
            nbi4,
            nbj4,
            nsi4,
            nsub4,
            num,
            text
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomUniformSurface>,
    ) -> Result<(), Failure> {
        rw_block_0035!(ent, i4, j4, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomUniformSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0177!(ent, is1, iter, js1, nbi_elem1, nbj_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwUniformSurfaceAndRationalBSplineSurface,
    "RwStepGeomRwUniformSurfaceAndRationalBSplineSurface"
);
impl RwStepGeomRwUniformSurfaceAndRationalBSplineSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomUniformSurfaceAndRationalBSplineSurface>,
    ) -> Result<(), Failure> {
        let mut num: i32 = num0;
        rw_block_0002!(
            a_control_points_list,
            a_self_intersect,
            a_surface_form,
            a_u_closed,
            a_u_degree,
            a_v_closed,
            a_v_degree,
            a_weights_data,
            a_weights_data_item,
            ach,
            anent3,
            data,
            i3,
            j3,
            nbi3,
            nbj3,
            nsi3,
            nsub3,
            nsub8,
            num,
            text
        );
        rw_block_0062!(
            a_name,
            a_weights_data,
            a_weights_data_item,
            ach,
            data,
            i8,
            j8,
            nbi8,
            nbj8,
            nsi8,
            nsub8,
            num
        );
        num = data.next_for_complex(num);
        if !data.check_nb_params(num, 0, ach, "uniform_surface")? {
            return Ok(());
        }
        ent.init_fields(
            &a_name,
            a_u_degree,
            a_v_degree,
            &a_control_points_list,
            a_surface_form,
            a_u_closed,
            a_v_closed,
            a_self_intersect,
            &a_weights_data,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomUniformSurfaceAndRationalBSplineSurface>,
    ) -> Result<(), Failure> {
        rw_block_0009!(ent, i3, i8, j3, j8, sw);
        step_start_entity(sw, "UNIFORM_SURFACE")?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomUniformSurfaceAndRationalBSplineSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0177!(ent, is1, iter, js1, nbi_elem1, nbj_elem1);
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwVector, "RwStepGeomRwVector");
impl RwStepGeomRwVector {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomVector>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "vector")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_orientation: Handle<StepGeomDirection> = Default::default();
        data.read_entity(
            num,
            2,
            "orientation",
            ach,
            StepGeomDirection::static_type(),
            &mut a_orientation,
        )?;
        let mut a_magnitude: f64 = Default::default();
        data.read_real(num, 3, "magnitude", ach, &mut a_magnitude)?;
        ent.init(&a_name, &a_orientation, a_magnitude);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomVector>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.orientation())?;
        sw.send_real(ent.magnitude())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomVector>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.orientation().cast());
        Ok(())
    }
    /// `Check`.
    pub fn check(
        &self,
        ent: &Handle<StepGeomVector>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.magnitude().abs() < f64::EPSILON {
            ach.add_fail_text("ERROR: Magnitude of Vector = 0.0", "");
        }
        Ok(())
    }
}
