#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepShapeRwOrientedFace, "RwStepShapeRwOrientedFace");
impl RwStepShapeRwOrientedFace {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeOrientedFace>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "oriented_face")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        data.check_derived(num, 2, "bounds", ach, false)?;
        let mut a_face_element: Handle<StepShapeFace> = Default::default();
        data.read_entity(
            num,
            3,
            "face_element",
            ach,
            StepShapeFace::static_type(),
            &mut a_face_element,
        )?;
        let mut a_orientation: bool = Default::default();
        data.read_boolean(num, 4, "orientation", ach, &mut a_orientation)?;
        ent.init(&a_name, &a_face_element, a_orientation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeOrientedFace>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_derived()?;
        sw.send_entity(&ent.face_element())?;
        sw.send_boolean(ent.orientation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeOrientedFace>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.face_element().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwOrientedOpenShell,
    "RwStepShapeRwOrientedOpenShell"
);
impl RwStepShapeRwOrientedOpenShell {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeOrientedOpenShell>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "oriented_open_shell")? {
            return Ok(());
        }
        rw_block_0198!(a_name, ach, data, num);
        let mut a_open_shell_element: Handle<StepShapeOpenShell> = Default::default();
        data.read_entity(
            num,
            3,
            "open_shell_element",
            ach,
            StepShapeOpenShell::static_type(),
            &mut a_open_shell_element,
        )?;
        let mut a_orientation: bool = Default::default();
        data.read_boolean(num, 4, "orientation", ach, &mut a_orientation)?;
        ent.init(&a_name, &a_open_shell_element, a_orientation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeOrientedOpenShell>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_derived()?;
        sw.send_entity(&ent.open_shell_element())?;
        sw.send_boolean(ent.orientation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeOrientedOpenShell>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.open_shell_element().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwOrientedPath, "RwStepShapeRwOrientedPath");
impl RwStepShapeRwOrientedPath {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeOrientedPath>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "oriented_path")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        data.check_derived(num, 2, "edge_list", ach, false)?;
        let mut a_path_element: Handle<StepShapeEdgeLoop> = Default::default();
        data.read_entity(
            num,
            3,
            "path_element",
            ach,
            StepShapeEdgeLoop::static_type(),
            &mut a_path_element,
        )?;
        let mut a_orientation: bool = Default::default();
        data.read_boolean(num, 4, "orientation", ach, &mut a_orientation)?;
        ent.init(&a_name, &a_path_element, a_orientation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeOrientedPath>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_derived()?;
        sw.send_entity(&ent.path_element())?;
        sw.send_boolean(ent.orientation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeOrientedPath>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.path_element().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwPath, "RwStepShapeRwPath");
impl RwStepShapeRwPath {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapePath>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "path")? {
            return Ok(());
        }
        rw_block_0182!(a_edge_list, a_name, ach, data, num);
        let mut anent2: Handle<StepShapeOrientedEdge> = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "edge_list", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_edge_list = step_new_array1::<Handle<StepShapeOrientedEdge>>(1, nb2)?;
            for i2 in 1..=nb2 {
                if data.read_entity(
                    nsub2,
                    i2,
                    "oriented_edge",
                    ach,
                    StepShapeOrientedEdge::static_type(),
                    &mut anent2,
                )? {
                    step_array1_set(&a_edge_list, i2, anent2.clone(), "ReadStep")?;
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
        ent: &Handle<StepShapePath>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_edge_list()? {
            sw.send_entity(&ent.edge_list_value(i2)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapePath>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_edge_list()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.edge_list_value(is1)?.cast());
        }
        Ok(())
    }
}
