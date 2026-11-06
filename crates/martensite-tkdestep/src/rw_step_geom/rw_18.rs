#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepGeomRwRectangularCompositeSurface,
    "RwStepGeomRwRectangularCompositeSurface"
);
impl RwStepGeomRwRectangularCompositeSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomRectangularCompositeSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "rectangular_composite_surface")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_segments: Handle<HArray2<Handle<StepGeomSurfacePatch>>> = Default::default();
        let mut anent2: Handle<StepGeomSurfacePatch> = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "segments", ach, &mut nsub2, false, 0, 0)? {
            let nbi2: i32 = data.nb_params(nsub2)?;
            let nbj2: i32 = data.nb_params(data.param_number(nsub2, 1)?)?;
            a_segments = step_new_array2::<Handle<StepGeomSurfacePatch>>(1, nbi2, 1, nbj2)?;
            for i2 in 1..=nbi2 {
                let mut nsi2: i32 = Default::default();
                if data.read_sub_list(
                    nsub2,
                    i2,
                    "sub-part(segments)",
                    ach,
                    &mut nsi2,
                    false,
                    0,
                    0,
                )? {
                    for j2 in 1..=nbj2 {
                        if data.read_entity(
                            nsi2,
                            j2,
                            "surface_patch",
                            ach,
                            StepGeomSurfacePatch::static_type(),
                            &mut anent2,
                        )? {
                            step_array2_set(&a_segments, i2, j2, anent2.clone(), "ReadStep")?;
                        }
                    }
                }
            }
        }
        ent.init(&a_name, &a_segments);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomRectangularCompositeSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_segments_i() {
            sw.new_line(false)?;
            sw.open_sub()?;
            for j2 in 1..=ent.nb_segments_j() {
                sw.send_entity(&ent.segments_value(i2, j2)?)?;
                sw.join_last(false);
            }
            sw.close_sub()?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomRectangularCompositeSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nbi_elem1: i32 = ent.nb_segments_i();
        let nbj_elem1: i32 = ent.nb_segments_j();
        for is1 in 1..=nbi_elem1 {
            for js1 in 1..=nbj_elem1 {
                iter.get_one_item(&ent.segments_value(is1, js1)?.cast());
            }
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwRectangularTrimmedSurface,
    "RwStepGeomRwRectangularTrimmedSurface"
);
impl RwStepGeomRwRectangularTrimmedSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomRectangularTrimmedSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 8, ach, "rectangular_trimmed_surface")? {
            return Ok(());
        }
        rw_block_0133!(a_basis_surface, a_name, ach, data, num);
        let mut a_u1: f64 = Default::default();
        data.read_real(num, 3, "u1", ach, &mut a_u1)?;
        let mut a_u2: f64 = Default::default();
        data.read_real(num, 4, "u2", ach, &mut a_u2)?;
        let mut a_v1: f64 = Default::default();
        data.read_real(num, 5, "v1", ach, &mut a_v1)?;
        let mut a_v2: f64 = Default::default();
        data.read_real(num, 6, "v2", ach, &mut a_v2)?;
        let mut a_usense: bool = Default::default();
        data.read_boolean(num, 7, "usense", ach, &mut a_usense)?;
        let mut a_vsense: bool = Default::default();
        data.read_boolean(num, 8, "vsense", ach, &mut a_vsense)?;
        ent.init(
            &a_name,
            &a_basis_surface,
            a_u1,
            a_u2,
            a_v1,
            a_v2,
            a_usense,
            a_vsense,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomRectangularTrimmedSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.basis_surface())?;
        sw.send_real(ent.u1())?;
        sw.send_real(ent.u2())?;
        sw.send_real(ent.v1())?;
        sw.send_real(ent.v2())?;
        sw.send_boolean(ent.usense())?;
        sw.send_boolean(ent.vsense())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomRectangularTrimmedSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.basis_surface().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwReparametrisedCompositeCurveSegment,
    "RwStepGeomRwReparametrisedCompositeCurveSegment"
);
impl RwStepGeomRwReparametrisedCompositeCurveSegment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomReparametrisedCompositeCurveSegment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "reparametrised_composite_curve_segment")? {
            return Ok(());
        }
        rw_block_0079!(
            a_parent_curve,
            a_same_sense,
            a_transition,
            ach,
            data,
            num,
            text
        );
        let mut a_param_length: f64 = Default::default();
        data.read_real(num, 4, "param_length", ach, &mut a_param_length)?;
        ent.init(a_transition, a_same_sense, &a_parent_curve, a_param_length);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomReparametrisedCompositeCurveSegment>,
    ) -> Result<(), Failure> {
        sw.send_enum(RwStepGeomRwTransitionCode::convert_to_string(
            ent.transition(),
        ))?;
        sw.send_boolean(ent.same_sense())?;
        sw.send_entity(&ent.parent_curve())?;
        sw.send_real(ent.param_length())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomReparametrisedCompositeCurveSegment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.parent_curve().cast());
        Ok(())
    }
}
