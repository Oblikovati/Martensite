#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepGeomRwHyperbola, "RwStepGeomRwHyperbola");
impl RwStepGeomRwHyperbola {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomHyperbola>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "hyperbola")? {
            return Ok(());
        }
        rw_block_0153!(a_name, a_position, ach, data, num);
        let mut a_semi_axis: f64 = Default::default();
        data.read_real(num, 3, "semi_axis", ach, &mut a_semi_axis)?;
        let mut a_semi_imag_axis: f64 = Default::default();
        data.read_real(num, 4, "semi_imag_axis", ach, &mut a_semi_imag_axis)?;
        ent.init(&a_name, &a_position, a_semi_axis, a_semi_imag_axis);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomHyperbola>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.position().value())?;
        sw.send_real(ent.semi_axis())?;
        sw.send_real(ent.semi_imag_axis())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomHyperbola>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwIntersectionCurve,
    "RwStepGeomRwIntersectionCurve"
);
impl RwStepGeomRwIntersectionCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomIntersectionCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "intersection_curve")? {
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
        ent: &Handle<StepGeomIntersectionCurve>,
    ) -> Result<(), Failure> {
        rw_block_0092!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomIntersectionCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0187!(ent, is2, iter, nb_elem2);
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwLine, "RwStepGeomRwLine");
impl RwStepGeomRwLine {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomLine>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "line")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_pnt: Handle<StepGeomCartesianPoint> = Default::default();
        data.read_entity(
            num,
            2,
            "pnt",
            ach,
            StepGeomCartesianPoint::static_type(),
            &mut a_pnt,
        )?;
        let mut a_dir: Handle<StepGeomVector> = Default::default();
        data.read_entity(
            num,
            3,
            "dir",
            ach,
            StepGeomVector::static_type(),
            &mut a_dir,
        )?;
        ent.init(&a_name, &a_pnt, &a_dir);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomLine>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.pnt())?;
        sw.send_entity(&ent.dir())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomLine>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.pnt().cast());
        iter.get_one_item(&ent.dir().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwOffsetCurve3d, "RwStepGeomRwOffsetCurve3d");
impl RwStepGeomRwOffsetCurve3d {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomOffsetCurve3d>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "offset_curve_3d")? {
            return Ok(());
        }
        rw_block_0132!(a_basis_curve, a_name, ach, data, num);
        let mut a_distance: f64 = Default::default();
        rw_block_0203!(a_distance, a_self_intersect, ach, data, num);
        let mut a_ref_direction: Handle<StepGeomDirection> = Default::default();
        data.read_entity(
            num,
            5,
            "ref_direction",
            ach,
            StepGeomDirection::static_type(),
            &mut a_ref_direction,
        )?;
        ent.init(
            &a_name,
            &a_basis_curve,
            a_distance,
            a_self_intersect,
            &a_ref_direction,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomOffsetCurve3d>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.basis_curve())?;
        sw.send_real(ent.distance())?;
        sw.send_logical(ent.self_intersect())?;
        sw.send_entity(&ent.ref_direction())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomOffsetCurve3d>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.basis_curve().cast());
        iter.get_one_item(&ent.ref_direction().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwOffsetSurface, "RwStepGeomRwOffsetSurface");
impl RwStepGeomRwOffsetSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomOffsetSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "offset_surface")? {
            return Ok(());
        }
        rw_block_0133!(a_basis_surface, a_name, ach, data, num);
        let mut a_distance: f64 = Default::default();
        rw_block_0203!(a_distance, a_self_intersect, ach, data, num);
        ent.init(&a_name, &a_basis_surface, a_distance, a_self_intersect);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomOffsetSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.basis_surface())?;
        sw.send_real(ent.distance())?;
        sw.send_logical(ent.self_intersect())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomOffsetSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.basis_surface().cast());
        Ok(())
    }
}
