#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwTessellatedConnectingEdge,
    "RwStepVisualRwTessellatedConnectingEdge"
);
impl RwStepVisualRwTessellatedConnectingEdge {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTessellatedConnectingEdge>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 9, the_check, "tessellated_connecting_edge")? {
            return Ok(());
        }
        rw_block_b_0038!(
            a_representation_item_name,
            a_tessellated_edge_coordinates,
            a_tessellated_edge_geometric_link,
            a_tessellated_edge_line_strip,
            an_it0,
            has_tessellated_edge_geometric_link,
            i0,
            nb0,
            num2,
            sub4,
            the_check,
            the_data,
            the_num
        );
        let mut a_smooth: StepDataLogical = Default::default();
        the_data.read_logical(the_num, 5, "smooth", the_check, &mut a_smooth)?;
        let mut a_face1: Handle<StepVisualTessellatedFace> = Default::default();
        the_data.read_entity(
            the_num,
            6,
            "face1",
            the_check,
            StepVisualTessellatedFace::static_type(),
            &mut a_face1,
        )?;
        let mut a_face2: Handle<StepVisualTessellatedFace> = Default::default();
        the_data.read_entity(
            the_num,
            7,
            "face2",
            the_check,
            StepVisualTessellatedFace::static_type(),
            &mut a_face2,
        )?;
        let mut a_line_strip_face1: Handle<HArray1<i32>> = Default::default();
        let mut sub8: i32 = 0;
        if the_data.read_sub_list(
            the_num,
            8,
            "line_strip_face1",
            the_check,
            &mut sub8,
            false,
            0,
            0,
        )? {
            let nb0: i32 = the_data.nb_params(sub8)?;
            a_line_strip_face1 = step_new_array1::<i32>(1, nb0)?;
            let num2: i32 = sub8;
            for i0 in 1..=nb0 {
                let mut an_it0: i32 = Default::default();
                the_data.read_integer(num2, i0, "integer", the_check, &mut an_it0)?;
                step_array1_set(&a_line_strip_face1, i0, an_it0, "ReadStep")?;
            }
        }
        let mut a_line_strip_face2: Handle<HArray1<i32>> = Default::default();
        let mut sub9: i32 = 0;
        if the_data.read_sub_list(
            the_num,
            9,
            "line_strip_face2",
            the_check,
            &mut sub9,
            false,
            0,
            0,
        )? {
            let nb0: i32 = the_data.nb_params(sub9)?;
            a_line_strip_face2 = step_new_array1::<i32>(1, nb0)?;
            let num2: i32 = sub9;
            for i0 in 1..=nb0 {
                let mut an_it0: i32 = Default::default();
                the_data.read_integer(num2, i0, "integer", the_check, &mut an_it0)?;
                step_array1_set(&a_line_strip_face2, i0, an_it0, "ReadStep")?;
            }
        }
        the_ent.init(
            &a_representation_item_name,
            &a_tessellated_edge_coordinates,
            has_tessellated_edge_geometric_link,
            &a_tessellated_edge_geometric_link,
            &a_tessellated_edge_line_strip,
            a_smooth,
            &a_face1,
            &a_face2,
            &a_line_strip_face1,
            &a_line_strip_face2,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTessellatedConnectingEdge>,
    ) -> Result<(), Failure> {
        rw_block_b_0085!(i3, the_ent, the_sw, var0);
        the_sw.send_logical(the_ent.smooth())?;
        the_sw.send_entity(&the_ent.face1())?;
        the_sw.send_entity(&the_ent.face2())?;
        the_sw.open_sub()?;
        for i7 in 1..=step_live(&the_ent.line_strip_face1(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: i32 = step_array1_item(&the_ent.line_strip_face1(), i7, "WriteStep")?;
            the_sw.send_integer(var0)?;
        }
        the_sw.close_sub()?;
        the_sw.open_sub()?;
        for i8 in 1..=step_live(&the_ent.line_strip_face2(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: i32 = step_array1_item(&the_ent.line_strip_face2(), i8, "WriteStep")?;
            the_sw.send_integer(var0)?;
        }
        the_sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualTessellatedConnectingEdge>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0215!(the_ent, the_iter);
        the_iter.add_item(&the_ent.face1().cast());
        the_iter.add_item(&the_ent.face2().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTessellatedCurveSet,
    "RwStepVisualRwTessellatedCurveSet"
);
impl RwStepVisualRwTessellatedCurveSet {
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualTessellatedCurveSet>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.coord_list().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTessellatedEdge,
    "RwStepVisualRwTessellatedEdge"
);
impl RwStepVisualRwTessellatedEdge {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTessellatedEdge>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_check, "tessellated_edge")? {
            return Ok(());
        }
        rw_block_b_0138!(
            a_coordinates,
            a_representation_item_name,
            the_check,
            the_data,
            the_num
        );
        let mut a_geometric_link: StepVisualEdgeOrCurve = Default::default();
        let mut has_geometric_link: bool = true;
        if the_data.is_param_defined(the_num, 3)? {
            the_data.read_entity_select(
                the_num,
                3,
                "geometric_link",
                the_check,
                &mut a_geometric_link,
            )?;
        } else {
            has_geometric_link = false;
            a_geometric_link = StepVisualEdgeOrCurve::default();
        }
        let mut a_line_strip: Handle<HArray1<i32>> = Default::default();
        let mut sub4: i32 = 0;
        if the_data.read_sub_list(the_num, 4, "line_strip", the_check, &mut sub4, false, 0, 0)? {
            let nb0: i32 = the_data.nb_params(sub4)?;
            a_line_strip = step_new_array1::<i32>(1, nb0)?;
            let num2: i32 = sub4;
            for i0 in 1..=nb0 {
                let mut an_it0: i32 = Default::default();
                the_data.read_integer(num2, i0, "integer", the_check, &mut an_it0)?;
                step_array1_set(&a_line_strip, i0, an_it0, "ReadStep")?;
            }
        }
        the_ent.init(
            &a_representation_item_name,
            &a_coordinates,
            has_geometric_link,
            &a_geometric_link,
            &a_line_strip,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTessellatedEdge>,
    ) -> Result<(), Failure> {
        rw_block_b_0085!(i3, the_ent, the_sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualTessellatedEdge>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        the_iter.add_item(&the_ent.coordinates().cast());
        if the_ent.has_geometric_link() {
            the_iter.add_item(&the_ent.geometric_link().value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTessellatedGeometricSet,
    "RwStepVisualRwTessellatedGeometricSet"
);

step_rw_class!(
    RwStepVisualRwTessellatedItem,
    "RwStepVisualRwTessellatedItem"
);
impl RwStepVisualRwTessellatedItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualTessellatedItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "tessellated_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        ent.init(&a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualTessellatedItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}
