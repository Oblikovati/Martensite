#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepGeomRwOrientedSurface, "RwStepGeomRwOrientedSurface");
impl RwStepGeomRwOrientedSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomOrientedSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "oriented_surface")? {
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
        let mut a_orientation: bool = Default::default();
        data.read_boolean(num, 2, "orientation", ach, &mut a_orientation)?;
        ent.init(&a_representation_item_name, a_orientation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomOrientedSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.parent.name())?;
        sw.send_boolean(ent.orientation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepGeomOrientedSurface>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwOuterBoundaryCurve,
    "RwStepGeomRwOuterBoundaryCurve"
);
impl RwStepGeomRwOuterBoundaryCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomOuterBoundaryCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "outer_boundary_curve")? {
            return Ok(());
        }
        rw_block_0040!(
            a_name,
            a_segments,
            a_self_intersect,
            ach,
            anent2,
            data,
            ent,
            i2,
            nb2,
            nsub2,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomOuterBoundaryCurve>,
    ) -> Result<(), Failure> {
        rw_block_0117!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomOuterBoundaryCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_segments();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.segments_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwParabola, "RwStepGeomRwParabola");
impl RwStepGeomRwParabola {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomParabola>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "parabola")? {
            return Ok(());
        }
        rw_block_0153!(a_name, a_position, ach, data, num);
        let mut a_focal_dist: f64 = Default::default();
        data.read_real(num, 3, "focal_dist", ach, &mut a_focal_dist)?;
        ent.init(&a_name, &a_position, a_focal_dist);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomParabola>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.position().value())?;
        sw.send_real(ent.focal_dist())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomParabola>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().value());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwPcurve, "RwStepGeomRwPcurve");
impl RwStepGeomRwPcurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomPcurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "pcurve")? {
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
        ent: &Handle<StepGeomPcurve>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.basis_surface())?;
        sw.send_entity(&ent.reference_to_curve())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomPcurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.basis_surface().cast());
        iter.get_one_item(&ent.reference_to_curve().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwPlacement, "RwStepGeomRwPlacement");
impl RwStepGeomRwPlacement {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomPlacement>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "placement")? {
            return Ok(());
        }
        rw_block_0130!(a_location, a_name, ach, data, num);
        ent.init(&a_name, &a_location);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomPlacement>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.location())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomPlacement>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.location().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwPlane, "RwStepGeomRwPlane");
impl RwStepGeomRwPlane {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomPlane>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "plane")? {
            return Ok(());
        }
        rw_block_0107!(a_name, a_position, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomPlane>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.position())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomPlane>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
}
