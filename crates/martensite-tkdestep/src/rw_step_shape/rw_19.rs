#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepShapeRwSphere, "RwStepShapeRwSphere");
impl RwStepShapeRwSphere {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeSphere>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "sphere")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_radius: f64 = Default::default();
        data.read_real(num, 2, "radius", ach, &mut a_radius)?;
        let mut a_centre: Handle<StepGeomPoint> = Default::default();
        data.read_entity(
            num,
            3,
            "centre",
            ach,
            StepGeomPoint::static_type(),
            &mut a_centre,
        )?;
        ent.init(&a_name, a_radius, &a_centre);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeSphere>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_real(ent.radius())?;
        sw.send_entity(&ent.centre())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeSphere>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.centre().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwSubedge, "RwStepShapeRwSubedge");
impl RwStepShapeRwSubedge {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeSubedge>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "subedge")? {
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
        let mut a_edge_edge_start: Handle<StepShapeVertex> = Default::default();
        data.read_entity(
            num,
            2,
            "edge.edge_start",
            ach,
            StepShapeVertex::static_type(),
            &mut a_edge_edge_start,
        )?;
        let mut a_edge_edge_end: Handle<StepShapeVertex> = Default::default();
        data.read_entity(
            num,
            3,
            "edge.edge_end",
            ach,
            StepShapeVertex::static_type(),
            &mut a_edge_edge_end,
        )?;
        let mut a_parent_edge: Handle<StepShapeEdge> = Default::default();
        data.read_entity(
            num,
            4,
            "parent_edge",
            ach,
            StepShapeEdge::static_type(),
            &mut a_parent_edge,
        )?;
        ent.init(
            &a_representation_item_name,
            &a_edge_edge_start,
            &a_edge_edge_end,
            &a_parent_edge,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeSubedge>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.parent.name())?;
        sw.send_entity(&ent.parent.edge_start())?;
        sw.send_entity(&ent.parent.edge_end())?;
        sw.send_entity(&ent.parent_edge())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeSubedge>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.edge_start().cast());
        iter.add_item(&ent.parent.edge_end().cast());
        iter.add_item(&ent.parent_edge().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwSubface, "RwStepShapeRwSubface");
impl RwStepShapeRwSubface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeSubface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "subface")? {
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
        let mut a_face_bounds: Handle<HArray1<Handle<StepShapeFaceBound>>> = Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "face.bounds", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            a_face_bounds = step_new_array1::<Handle<StepShapeFaceBound>>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepShapeFaceBound> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "face.bounds",
                    ach,
                    StepShapeFaceBound::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_face_bounds, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        let mut a_parent_face: Handle<StepShapeFace> = Default::default();
        data.read_entity(
            num,
            3,
            "parent_face",
            ach,
            StepShapeFace::static_type(),
            &mut a_parent_face,
        )?;
        ent.init(&a_representation_item_name, &a_face_bounds, &a_parent_face);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeSubface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.parent.name())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.parent.bounds()?, "WriteStep")?
            .array1()
            .length()
        {
            let var0: Handle<StepShapeFaceBound> =
                step_array1_item(&ent.parent.bounds()?, i1, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        sw.send_entity(&ent.parent_face())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeSubface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.parent.bounds()?, "Share")?.array1().length() {
            let var0: Handle<StepShapeFaceBound> =
                step_array1_item(&ent.parent.bounds()?, i1, "Share")?;
            iter.add_item(&var0.cast());
        }
        iter.add_item(&ent.parent_face().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwSweptAreaSolid, "RwStepShapeRwSweptAreaSolid");
impl RwStepShapeRwSweptAreaSolid {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeSweptAreaSolid>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "swept_area_solid")? {
            return Ok(());
        }
        rw_block_0137!(a_name, a_swept_area, ach, data, num);
        ent.init(&a_name, &a_swept_area);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeSweptAreaSolid>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.swept_area())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeSweptAreaSolid>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.swept_area().cast());
        Ok(())
    }
}
