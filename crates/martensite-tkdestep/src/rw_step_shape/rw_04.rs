#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepShapeRwConnectedEdgeSet,
    "RwStepShapeRwConnectedEdgeSet"
);
impl RwStepShapeRwConnectedEdgeSet {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeConnectedEdgeSet>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "connected_edge_set")? {
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
        let mut a_ces_edges: Handle<HArray1<Handle<StepShapeEdge>>> = Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "ces_edges", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            a_ces_edges = step_new_array1::<Handle<StepShapeEdge>>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepShapeEdge> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "ces_edges",
                    ach,
                    StepShapeEdge::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_ces_edges, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(&a_representation_item_name, &a_ces_edges);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeConnectedEdgeSet>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.ces_edges(), "WriteStep")?.array1().length() {
            let var0: Handle<StepShapeEdge> = step_array1_item(&ent.ces_edges(), i1, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeConnectedEdgeSet>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.ces_edges(), "Share")?.array1().length() {
            let var0: Handle<StepShapeEdge> = step_array1_item(&ent.ces_edges(), i1, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwConnectedFaceSet,
    "RwStepShapeRwConnectedFaceSet"
);
impl RwStepShapeRwConnectedFaceSet {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeConnectedFaceSet>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "connected_face_set")? {
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
        ent: &Handle<StepShapeConnectedFaceSet>,
    ) -> Result<(), Failure> {
        rw_block_0161!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeConnectedFaceSet>,
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
    RwStepShapeRwConnectedFaceShapeRepresentation,
    "RwStepShapeRwConnectedFaceShapeRepresentation"
);
impl RwStepShapeRwConnectedFaceShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeConnectedFaceShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "connected_face_shape_representation")? {
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
        ent: &Handle<StepShapeConnectedFaceShapeRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.name())?;
        sw.open_sub()?;
        for i1 in 1..=ent.parent.nb_items() {
            let var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&ent.parent.items(), i1, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        sw.send_entity(&ent.parent.context_of_items())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeConnectedFaceShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=ent.parent.nb_items() {
            let var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&ent.parent.items(), i1, "Share")?;
            iter.add_item(&var0.cast());
        }
        iter.add_item(&ent.parent.context_of_items().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwConnectedFaceSubSet,
    "RwStepShapeRwConnectedFaceSubSet"
);
impl RwStepShapeRwConnectedFaceSubSet {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeConnectedFaceSubSet>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "connected_face_sub_set")? {
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
        let mut a_connected_face_set_cfs_faces: Handle<HArray1<Handle<StepShapeFace>>> =
            Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(
            num,
            2,
            "connected_face_set.cfs_faces",
            ach,
            &mut sub2,
            false,
            0,
            0,
        )? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            a_connected_face_set_cfs_faces = step_new_array1::<Handle<StepShapeFace>>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepShapeFace> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "connected_face_set.cfs_faces",
                    ach,
                    StepShapeFace::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(
                    &a_connected_face_set_cfs_faces,
                    i0,
                    an_it0.clone(),
                    "ReadStep",
                )?;
            }
        }
        let mut a_parent_face_set: Handle<StepShapeConnectedFaceSet> = Default::default();
        data.read_entity(
            num,
            3,
            "parent_face_set",
            ach,
            StepShapeConnectedFaceSet::static_type(),
            &mut a_parent_face_set,
        )?;
        ent.init(
            &a_representation_item_name,
            &a_connected_face_set_cfs_faces,
            &a_parent_face_set,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeConnectedFaceSubSet>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.parent.name())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.parent.cfs_faces()?, "WriteStep")?
            .array1()
            .length()
        {
            let var0: Handle<StepShapeFace> =
                step_array1_item(&ent.parent.cfs_faces()?, i1, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        sw.send_entity(&ent.parent_face_set())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeConnectedFaceSubSet>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.parent.cfs_faces()?, "Share")?
            .array1()
            .length()
        {
            let var0: Handle<StepShapeFace> =
                step_array1_item(&ent.parent.cfs_faces()?, i1, "Share")?;
            iter.add_item(&var0.cast());
        }
        iter.add_item(&ent.parent_face_set().cast());
        Ok(())
    }
}
