#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepGeomRwSeamCurve, "RwStepGeomRwSeamCurve");
impl RwStepGeomRwSeamCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomSeamCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "seam_curve")? {
            return Ok(());
        }
        rw_block_0084!(
            a_associated_geometry,
            a_associated_geometry_item,
            a_curve3d,
            a_name,
            ach,
            data,
            num
        );
        let mut assgeomval: Handle<StandardTransient> = Default::default();
        let mut nsub3: i32 = Default::default();
        if data.read_sub_list(num, 3, "associated_geometry", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            a_associated_geometry = step_new_array1::<StepGeomPcurveOrSurface>(1, nb3)?;
            for i3 in 1..=nb3 {
                if data.read_entity_select(
                    nsub3,
                    i3,
                    "associated_geometry",
                    ach,
                    &mut a_associated_geometry_item,
                )? {
                    step_array1_set(
                        &a_associated_geometry,
                        i3,
                        a_associated_geometry_item.clone(),
                        "ReadStep",
                    )?;
                }
                if i3 == 1 {
                    assgeomval = a_associated_geometry_item.value();
                } else {
                    if assgeomval == a_associated_geometry_item.value() {
                        ach.add_fail_text("Seam Curve with twice the same geom", "");
                    }
                }
            }
        }
        rw_block_0170!(
            a_associated_geometry,
            a_curve3d,
            a_master_representation,
            a_name,
            ach,
            data,
            ent,
            num,
            text
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomSeamCurve>,
    ) -> Result<(), Failure> {
        rw_block_0092!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomSeamCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0187!(ent, is2, iter, nb_elem2);
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwSphericalSurface, "RwStepGeomRwSphericalSurface");
impl RwStepGeomRwSphericalSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomSphericalSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "spherical_surface")? {
            return Ok(());
        }
        rw_block_0071!(a_name, a_position, a_radius, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomSphericalSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.position())?;
        sw.send_real(ent.radius())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomSphericalSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwSuParameters, "RwStepGeomRwSuParameters");
impl RwStepGeomRwSuParameters {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_ach: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepGeomSuParameters>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 7, the_ach, "su_parameters")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(
            the_num,
            1,
            "representation_item.name",
            the_ach,
            &mut a_representation_item_name,
        )?;
        let mut a_a: f64 = Default::default();
        the_data.read_real(the_num, 2, "a", the_ach, &mut a_a)?;
        let mut a_alpha: f64 = Default::default();
        the_data.read_real(the_num, 3, "alpha", the_ach, &mut a_alpha)?;
        let mut a_b: f64 = Default::default();
        the_data.read_real(the_num, 4, "b", the_ach, &mut a_b)?;
        let mut a_beta: f64 = Default::default();
        the_data.read_real(the_num, 5, "beta", the_ach, &mut a_beta)?;
        let mut a_c: f64 = Default::default();
        the_data.read_real(the_num, 6, "c", the_ach, &mut a_c)?;
        let mut a_gamma: f64 = Default::default();
        the_data.read_real(the_num, 7, "gamma", the_ach, &mut a_gamma)?;
        the_ent.init(
            &a_representation_item_name,
            a_a,
            a_alpha,
            a_b,
            a_beta,
            a_c,
            a_gamma,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepGeomSuParameters>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_real(the_ent.a())?;
        the_sw.send_real(the_ent.alpha())?;
        the_sw.send_real(the_ent.b())?;
        the_sw.send_real(the_ent.beta())?;
        the_sw.send_real(the_ent.c())?;
        the_sw.send_real(the_ent.gamma())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepGeomSuParameters>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwSurface, "RwStepGeomRwSurface");
impl RwStepGeomRwSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "surface")? {
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
        ent: &Handle<StepGeomSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwSurfaceCurve, "RwStepGeomRwSurfaceCurve");
impl RwStepGeomRwSurfaceCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomSurfaceCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "surface_curve")? {
            return Ok(());
        }
        rw_block_0023!(
            a_associated_geometry,
            a_associated_geometry_item,
            a_curve3d,
            a_master_representation,
            a_name,
            ach,
            data,
            ent,
            i3,
            nb3,
            nsub3,
            num,
            text
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomSurfaceCurve>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        rw_block_0101!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomSurfaceCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0187!(ent, is2, iter, nb_elem2);
        Ok(())
    }
}
