#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepShapeRwEdgeLoop, "RwStepShapeRwEdgeLoop");
impl RwStepShapeRwEdgeLoop {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeEdgeLoop>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "edge_loop")? {
            return Ok(());
        }
        rw_block_0182!(a_edge_list, a_name, ach, data, num);
        let mut anent: Handle<StepShapeOrientedEdge> = Default::default();
        let mut nsub1: i32 = Default::default();
        if data.read_sub_list(num, 2, "edge_list", ach, &mut nsub1, false, 0, 0)? {
            let nb1: i32 = data.nb_params(nsub1)?;
            a_edge_list = step_new_array1::<Handle<StepShapeOrientedEdge>>(1, nb1)?;
            for i1 in 1..=nb1 {
                if data.read_entity(
                    nsub1,
                    i1,
                    "oriented_edge",
                    ach,
                    StepShapeOrientedEdge::static_type(),
                    &mut anent,
                )? {
                    step_array1_set(&a_edge_list, i1, anent.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_edge_list);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeEdgeLoop>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i1 in 1..=ent.nb_edge_list() {
            sw.send_entity(&ent.edge_list_value(i1)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeEdgeLoop>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_edge_list();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.edge_list_value(is1)?.cast());
        }
        Ok(())
    }
    /// `Check`.
    pub fn check(
        &self,
        ent: &Handle<StepShapeEdgeLoop>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let mut head_to_tail: bool = true;
        let nb_edg: i32 = ent.nb_edge_list();
        if nb_edg == 0 {
            ach.add_fail_text("Edge loop contains empty edge list", "");
            return Ok(());
        }
        let mut the_oe: Handle<StepShapeOrientedEdge> = ent.edge_list_value(1)?;
        let the_vx_frst: Handle<StepShapeVertex> = step_live(&the_oe, "Check")?.edge_start();
        let mut the_vx_lst: Handle<StepShapeVertex> = step_live(&the_oe, "Check")?.edge_end();
        if (nb_edg == 1) && (the_vx_frst != the_vx_lst) {
            ach.add_fail_text("Edge loop composed of single Edge : Start and End Vertex of edge are not identical", "");
        }
        for i in 2..=nb_edg {
            the_oe = ent.edge_list_value(i)?;
            let the_vx_strt: Handle<StepShapeVertex> = step_live(&the_oe, "Check")?.edge_start();
            if the_vx_strt != the_vx_lst {
                head_to_tail = false;
            }
            the_vx_lst = step_live(&the_oe, "Check")?.edge_end();
            if the_vx_strt == the_vx_lst {
                ach.add_warning_text("One edge_curve contains identical vertices", "");
            }
        }
        if the_vx_frst != the_vx_lst {
            head_to_tail = false;
        }
        if !head_to_tail {
            ach.add_fail_text("Error : Path does not head to tail", "");
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwExtrudedAreaSolid,
    "RwStepShapeRwExtrudedAreaSolid"
);
impl RwStepShapeRwExtrudedAreaSolid {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeExtrudedAreaSolid>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "extruded_area_solid")? {
            return Ok(());
        }
        rw_block_0137!(a_name, a_swept_area, ach, data, num);
        let mut a_extruded_direction: Handle<StepGeomDirection> = Default::default();
        rw_block_0183!(a_depth, a_extruded_direction, ach, data, num);
        ent.init(&a_name, &a_swept_area, &a_extruded_direction, a_depth);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeExtrudedAreaSolid>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.swept_area())?;
        sw.send_entity(&ent.extruded_direction())?;
        sw.send_real(ent.depth())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeExtrudedAreaSolid>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.swept_area().cast());
        iter.get_one_item(&ent.extruded_direction().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwExtrudedFaceSolid,
    "RwStepShapeRwExtrudedFaceSolid"
);
impl RwStepShapeRwExtrudedFaceSolid {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeExtrudedFaceSolid>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "extruded_face_solid")? {
            return Ok(());
        }
        rw_block_0138!(a_name, a_swept_face, ach, data, num);
        let mut a_extruded_direction: Handle<StepGeomDirection> = Default::default();
        rw_block_0183!(a_depth, a_extruded_direction, ach, data, num);
        ent.init(&a_name, &a_swept_face, &a_extruded_direction, a_depth);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeExtrudedFaceSolid>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.swept_face())?;
        sw.send_entity(&ent.extruded_direction())?;
        sw.send_real(ent.depth())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeExtrudedFaceSolid>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.swept_face().cast());
        iter.get_one_item(&ent.extruded_direction().cast());
        Ok(())
    }
}
