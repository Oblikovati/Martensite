#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepShapeRwLoopAndPath, "RwStepShapeRwLoopAndPath");
impl RwStepShapeRwLoopAndPath {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeLoopAndPath>,
    ) -> Result<(), Failure> {
        let mut num: i32 = num0;
        if !data.check_nb_params(num, 0, ach, "loop")? {
            return Ok(());
        }
        num = data.next_for_complex(num);
        if !data.check_nb_params(num, 1, ach, "path")? {
            return Ok(());
        }
        let mut a_edge_list: Handle<HArray1<Handle<StepShapeOrientedEdge>>> = Default::default();
        let mut anent1: Handle<StepShapeOrientedEdge> = Default::default();
        let mut nsub1: i32 = Default::default();
        if data.read_sub_list(num, 1, "edge_list", ach, &mut nsub1, false, 0, 0)? {
            let nb1: i32 = data.nb_params(nsub1)?;
            a_edge_list = step_new_array1::<Handle<StepShapeOrientedEdge>>(1, nb1)?;
            for i1 in 1..=nb1 {
                if data.read_entity(
                    nsub1,
                    i1,
                    "oriented_edge",
                    ach,
                    StepShapeOrientedEdge::static_type(),
                    &mut anent1,
                )? {
                    step_array1_set(&a_edge_list, i1, anent1.clone(), "ReadStep")?;
                }
            }
        }
        rw_block_0114!(a_name, ach, data, num);
        if !data.check_nb_params(num, 0, ach, "topological_representation_item")? {
            return Ok(());
        }
        ent.init_fields(&a_name, &a_edge_list);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeLoopAndPath>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "LOOP")?;
        step_start_entity(sw, "PATH")?;
        sw.open_sub()?;
        for i1 in 1..=ent.nb_edge_list()? {
            sw.send_entity(&ent.edge_list_value(i1)?)?;
        }
        sw.close_sub()?;
        step_start_entity(sw, "REPRESENTATION_ITEM")?;
        sw.send_entity(&ent.name())?;
        step_start_entity(sw, "TOPOLOGICAL_REPRESENTATION_ITEM")?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeLoopAndPath>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_edge_list()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.edge_list_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwManifoldSolidBrep,
    "RwStepShapeRwManifoldSolidBrep"
);
impl RwStepShapeRwManifoldSolidBrep {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeManifoldSolidBrep>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "manifold_solid_brep")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_outer: Handle<StepShapeConnectedFaceSet> = Default::default();
        data.read_entity(
            num,
            2,
            "outer",
            ach,
            StepShapeConnectedFaceSet::static_type(),
            &mut a_outer,
        )?;
        ent.init_connected_face_set(&a_name, &a_outer);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeManifoldSolidBrep>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.outer())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeManifoldSolidBrep>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.outer().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwManifoldSurfaceShapeRepresentation,
    "RwStepShapeRwManifoldSurfaceShapeRepresentation"
);
impl RwStepShapeRwManifoldSurfaceShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeManifoldSurfaceShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "manifold_surface_shape_representation")? {
            return Ok(());
        }
        rw_block_0039!(
            a_context_of_items,
            a_items,
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
        ent: &Handle<StepShapeManifoldSurfaceShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0116!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeManifoldSurfaceShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0190!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwMeasureQualification,
    "RwStepShapeRwMeasureQualification"
);
impl RwStepShapeRwMeasureQualification {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeMeasureQualification>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "measure_qualification")? {
            return Ok(());
        }
        rw_block_0193!(a_descr, a_name, ach, data, num);
        data.read_string(num, 2, "description", ach, &mut a_descr)?;
        let mut a_qm: Handle<StandardTransient> = Default::default();
        data.read_entity(
            num,
            3,
            "qualified_measure",
            ach,
            StandardTransient::static_type(),
            &mut a_qm,
        )?;
        let mut quals: Handle<HArray1<StepShapeValueQualifier>> = Default::default();
        let mut nsub4: i32 = Default::default();
        if data.read_sub_list(num, 4, "qualifiers", ach, &mut nsub4, false, 0, 0)? {
            let nb4: i32 = data.nb_params(nsub4)?;
            quals = step_new_array1::<StepShapeValueQualifier>(1, nb4)?;
            for i4 in 1..=nb4 {
                let mut vq: StepShapeValueQualifier = Default::default();
                if data.read_entity_select(nsub4, i4, "qualifier", ach, &mut vq)? {
                    step_array1_set(&quals, i4, vq.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_descr, &a_qm, &quals);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeMeasureQualification>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        sw.send_entity(&ent.qualified_measure())?;
        rw_block_0118!(ent, i, nbq, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeMeasureQualification>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0197!(ent, i, iter, nbq);
        Ok(())
    }
}
