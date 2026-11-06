#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepShapeRwEdgeBasedWireframeModel,
    "RwStepShapeRwEdgeBasedWireframeModel"
);
impl RwStepShapeRwEdgeBasedWireframeModel {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeEdgeBasedWireframeModel>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "edge_based_wireframe_model")? {
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
        let mut a_ebwm_boundary: Handle<HArray1<Handle<StepShapeConnectedEdgeSet>>> =
            Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "ebwm_boundary", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            a_ebwm_boundary = step_new_array1::<Handle<StepShapeConnectedEdgeSet>>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepShapeConnectedEdgeSet> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "ebwm_boundary",
                    ach,
                    StepShapeConnectedEdgeSet::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_ebwm_boundary, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(&a_representation_item_name, &a_ebwm_boundary);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeEdgeBasedWireframeModel>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.ebwm_boundary(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: Handle<StepShapeConnectedEdgeSet> =
                step_array1_item(&ent.ebwm_boundary(), i1, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeEdgeBasedWireframeModel>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.ebwm_boundary(), "Share")?.array1().length() {
            let var0: Handle<StepShapeConnectedEdgeSet> =
                step_array1_item(&ent.ebwm_boundary(), i1, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwEdgeBasedWireframeShapeRepresentation,
    "RwStepShapeRwEdgeBasedWireframeShapeRepresentation"
);
impl RwStepShapeRwEdgeBasedWireframeShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeEdgeBasedWireframeShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "edge_based_wireframe_shape_representation")? {
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
        ent: &Handle<StepShapeEdgeBasedWireframeShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0099!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeEdgeBasedWireframeShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0207!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwEdgeCurve, "RwStepShapeRwEdgeCurve");
impl RwStepShapeRwEdgeCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_step_data: &Handle<StepDataStepReaderData>,
        the_record_id: i32,
        the_message_tool: &Handle<InterfaceCheck>,
        the_edge_curve: &Handle<StepShapeEdgeCurve>,
    ) -> Result<(), Failure> {
        if !the_step_data.check_nb_params(the_record_id, 5, the_message_tool, "edge_curve")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        the_step_data.read_string(the_record_id, 1, "name", the_message_tool, &mut a_name)?;
        let mut an_edge_start: Handle<StepShapeVertex> = Default::default();
        the_step_data.read_entity(
            the_record_id,
            2,
            "edge_start",
            the_message_tool,
            StepShapeVertex::static_type(),
            &mut an_edge_start,
        )?;
        let mut an_edge_end: Handle<StepShapeVertex> = Default::default();
        the_step_data.read_entity(
            the_record_id,
            3,
            "edge_end",
            the_message_tool,
            StepShapeVertex::static_type(),
            &mut an_edge_end,
        )?;
        let mut an_edge_geometry: Handle<StepGeomCurve> = Default::default();
        the_step_data.read_entity(
            the_record_id,
            4,
            "edge_geometry",
            the_message_tool,
            StepGeomCurve::static_type(),
            &mut an_edge_geometry,
        )?;
        let mut a_same_sense: bool = Default::default();
        the_step_data.read_boolean(
            the_record_id,
            5,
            "same_sense",
            the_message_tool,
            &mut a_same_sense,
        )?;
        the_edge_curve.init(
            &a_name,
            &an_edge_start,
            &an_edge_end,
            &an_edge_geometry,
            a_same_sense,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_step_writer: &mut StepDataStepWriter,
        the_edge_curve: &Handle<StepShapeEdgeCurve>,
    ) -> Result<(), Failure> {
        the_step_writer.send_entity(&the_edge_curve.name())?;
        the_step_writer.send_entity(&the_edge_curve.edge_start())?;
        the_step_writer.send_entity(&the_edge_curve.edge_end())?;
        the_step_writer.send_entity(&the_edge_curve.edge_geometry())?;
        the_step_writer.send_boolean(the_edge_curve.same_sense())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_edge_curve: &Handle<StepShapeEdgeCurve>,
        the_shared_entities_it: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        the_shared_entities_it.get_one_item(&the_edge_curve.edge_start().cast());
        the_shared_entities_it.get_one_item(&the_edge_curve.edge_end().cast());
        the_shared_entities_it.get_one_item(&the_edge_curve.edge_geometry().cast());
        Ok(())
    }
}
