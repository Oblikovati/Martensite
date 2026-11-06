#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepShapeRwBoxedHalfSpace, "RwStepShapeRwBoxedHalfSpace");
impl RwStepShapeRwBoxedHalfSpace {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeBoxedHalfSpace>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "boxed_half_space")? {
            return Ok(());
        }
        rw_block_0086!(a_agreement_flag, a_base_surface, a_name, ach, data, num);
        let mut a_enclosure: Handle<StepShapeBoxDomain> = Default::default();
        data.read_entity(
            num,
            4,
            "enclosure",
            ach,
            StepShapeBoxDomain::static_type(),
            &mut a_enclosure,
        )?;
        ent.init(&a_name, &a_base_surface, a_agreement_flag, &a_enclosure);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeBoxedHalfSpace>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.base_surface())?;
        sw.send_boolean(ent.agreement_flag())?;
        sw.send_entity(&ent.enclosure())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeBoxedHalfSpace>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.base_surface().cast());
        iter.get_one_item(&ent.enclosure().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwBrepWithVoids, "RwStepShapeRwBrepWithVoids");
impl RwStepShapeRwBrepWithVoids {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeBrepWithVoids>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "brep_with_voids")? {
            return Ok(());
        }
        rw_block_0136!(a_name, a_outer, ach, data, num);
        let mut a_voids: Handle<HArray1<Handle<StepShapeOrientedClosedShell>>> = Default::default();
        let mut anent3: Handle<StepShapeOrientedClosedShell> = Default::default();
        let mut nsub3: i32 = Default::default();
        if data.read_sub_list(num, 3, "voids", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            a_voids = step_new_array1::<Handle<StepShapeOrientedClosedShell>>(1, nb3)?;
            for i3 in 1..=nb3 {
                if data.read_entity(
                    nsub3,
                    i3,
                    "oriented_closed_shell",
                    ach,
                    StepShapeOrientedClosedShell::static_type(),
                    &mut anent3,
                )? {
                    step_array1_set(&a_voids, i3, anent3.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_outer, &a_voids);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeBrepWithVoids>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.outer())?;
        sw.open_sub()?;
        for i3 in 1..=ent.nb_voids() {
            sw.send_entity(&ent.voids_value(i3)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeBrepWithVoids>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.outer().cast());
        let nb_elem2: i32 = ent.nb_voids();
        for is2 in 1..=nb_elem2 {
            iter.get_one_item(&ent.voids_value(is2)?.cast());
        }
        Ok(())
    }
    /// `Check`.
    pub fn check(
        &self,
        ent: &Handle<StepShapeBrepWithVoids>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        for i in 1..=ent.nb_voids() {
            if step_live(&ent.voids_value(i)?, "Check")?.orientation() {
                ach.add_warning_text(
                    "Void has orientation .T. while .F. is required by API 214",
                    "",
                );
                break;
            }
        }
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwClosedShell, "RwStepShapeRwClosedShell");
impl RwStepShapeRwClosedShell {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeClosedShell>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "closed_shell")? {
            return Ok(());
        }
        rw_block_0065!(
            a_cfs_faces,
            a_name,
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
        ent: &Handle<StepShapeClosedShell>,
    ) -> Result<(), Failure> {
        rw_block_0161!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeClosedShell>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_cfs_faces()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.cfs_faces_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwCompoundShapeRepresentation,
    "RwStepShapeRwCompoundShapeRepresentation"
);
impl RwStepShapeRwCompoundShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeCompoundShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "compound_shape_representation")? {
            return Ok(());
        }
        rw_block_0051!(
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            ach,
            an_it0,
            data,
            ent,
            i0,
            nb0,
            num,
            num2,
            sub2
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeCompoundShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0099!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeCompoundShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0207!(ent, i1, iter, var0);
        Ok(())
    }
}
