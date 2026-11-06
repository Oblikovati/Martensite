#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepShapeRwMeasureRepresentationItemAndQualifiedRepresentationItem,
    "RwStepShapeRwMeasureRepresentationItemAndQualifiedRepresentationItem"
);
impl RwStepShapeRwMeasureRepresentationItemAndQualifiedRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem>,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        data.named_for_complex_short("MEASURE_REPRESENTATION_ITEM", "MSRPIT", num0, &mut num, ach);
        if !data.check_nb_params(num, 2, ach, "measure_representation_item")? {
            return Ok(());
        }
        rw_block_0143!(a_unit_component, ach, data, mvc, num);
        rw_block_0175!(ach, data, num, num0, quals);
        let mut nsub1: i32 = Default::default();
        if data.read_sub_list(num, 1, "qualifiers", ach, &mut nsub1, false, 0, 0)? {
            let nb1: i32 = data.nb_params(nsub1)?;
            quals = step_new_array1::<StepShapeValueQualifier>(1, nb1)?;
            for i1 in 1..=nb1 {
                let mut vq: StepShapeValueQualifier = Default::default();
                if data.read_entity_select(nsub1, i1, "qualifier", ach, &mut vq)? {
                    step_array1_set(&quals, i1, vq.clone(), "ReadStep")?;
                }
            }
        }
        rw_block_0131!(a_name, ach, data, num, num0);
        ent.init(&a_name, &mvc, &a_unit_component, &quals)?;
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "MEASURE_REPRESENTATION_ITEM")?;
        rw_block_0209!(ent, sw);
        step_start_entity(sw, "QUALIFIED_REPRESENTATION_ITEM")?;
        rw_block_0118!(ent, i, nbq, sw);
        step_start_entity(sw, "REPRESENTATION_ITEM")?;
        sw.send_entity(&ent.name())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&step_live(&ent.measure(), "Share")?.unit_component().value());
        rw_block_0197!(ent, i, iter, nbq);
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwNonManifoldSurfaceShapeRepresentation,
    "RwStepShapeRwNonManifoldSurfaceShapeRepresentation"
);
impl RwStepShapeRwNonManifoldSurfaceShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeNonManifoldSurfaceShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "non_manifold_surface_shape_representation")? {
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
        ent: &Handle<StepShapeNonManifoldSurfaceShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0099!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeNonManifoldSurfaceShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0207!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwOpenShell, "RwStepShapeRwOpenShell");
impl RwStepShapeRwOpenShell {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeOpenShell>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "open_shell")? {
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
        ent: &Handle<StepShapeOpenShell>,
    ) -> Result<(), Failure> {
        rw_block_0161!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeOpenShell>,
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
    RwStepShapeRwOrientedClosedShell,
    "RwStepShapeRwOrientedClosedShell"
);
impl RwStepShapeRwOrientedClosedShell {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeOrientedClosedShell>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "oriented_closed_shell")? {
            return Ok(());
        }
        rw_block_0198!(a_name, ach, data, num);
        let mut a_closed_shell_element: Handle<StepShapeClosedShell> = Default::default();
        data.read_entity(
            num,
            3,
            "closed_shell_element",
            ach,
            StepShapeClosedShell::static_type(),
            &mut a_closed_shell_element,
        )?;
        let mut a_orientation: bool = Default::default();
        data.read_boolean(num, 4, "orientation", ach, &mut a_orientation)?;
        ent.init(&a_name, &a_closed_shell_element, a_orientation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeOrientedClosedShell>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_derived()?;
        sw.send_entity(&ent.closed_shell_element())?;
        sw.send_boolean(ent.orientation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeOrientedClosedShell>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.closed_shell_element().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwOrientedEdge, "RwStepShapeRwOrientedEdge");
impl RwStepShapeRwOrientedEdge {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeOrientedEdge>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "oriented_edge")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        data.check_derived(num, 2, "edge_start", ach, false)?;
        data.check_derived(num, 3, "edge_end", ach, false)?;
        let mut a_edge_element: Handle<StepShapeEdge> = Default::default();
        data.read_entity(
            num,
            4,
            "edge_element",
            ach,
            StepShapeEdge::static_type(),
            &mut a_edge_element,
        )?;
        let mut a_orientation: bool = Default::default();
        data.read_boolean(num, 5, "orientation", ach, &mut a_orientation)?;
        ent.init(&a_name, &a_edge_element, a_orientation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeOrientedEdge>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_derived()?;
        sw.send_derived()?;
        sw.send_entity(&ent.edge_element())?;
        sw.send_boolean(ent.orientation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeOrientedEdge>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.edge_element().cast());
        Ok(())
    }
}
