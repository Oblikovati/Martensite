#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwCubicBezierTessellatedEdge,
    "RwStepVisualRwCubicBezierTessellatedEdge"
);
impl RwStepVisualRwCubicBezierTessellatedEdge {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualCubicBezierTessellatedEdge>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_check, "cubic_bezier_tessellated_edge")? {
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
        the_ent.init(
            &a_representation_item_name,
            &a_tessellated_edge_coordinates,
            has_tessellated_edge_geometric_link,
            &a_tessellated_edge_geometric_link,
            &a_tessellated_edge_line_strip,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualCubicBezierTessellatedEdge>,
    ) -> Result<(), Failure> {
        rw_block_b_0085!(i3, the_ent, the_sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualCubicBezierTessellatedEdge>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0215!(the_ent, the_iter);
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwCubicBezierTriangulatedFace,
    "RwStepVisualRwCubicBezierTriangulatedFace"
);
impl RwStepVisualRwCubicBezierTriangulatedFace {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualCubicBezierTriangulatedFace>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 6, the_check, "cubic_bezier_triangulated_face")? {
            return Ok(());
        }
        rw_block_b_0026!(
            a_representation_item_name,
            a_tessellated_face_coordinates,
            a_tessellated_face_geometric_link,
            a_tessellated_face_normals,
            a_tessellated_face_pnmax,
            an_it0,
            has_tessellated_face_geometric_link,
            i0,
            j0,
            nb0,
            nbj0,
            num4,
            sub4,
            subj4,
            the_check,
            the_data,
            the_num
        );
        let mut a_ctriangles: Handle<HArray2<i32>> = Default::default();
        let mut sub6: i32 = 0;
        if the_data.read_sub_list(the_num, 6, "ctriangles", the_check, &mut sub6, false, 0, 0)? {
            let nb0: i32 = the_data.nb_params(sub6)?;
            let nbj0: i32 = the_data.nb_params(the_data.param_number(sub6, 1)?)?;
            a_ctriangles = step_new_array2::<i32>(1, nb0, 1, nbj0)?;
            for i0 in 1..=nb0 {
                let mut subj6: i32 = 0;
                if the_data.read_sub_list(
                    sub6,
                    i0,
                    "sub-part(ctriangles)",
                    the_check,
                    &mut subj6,
                    false,
                    0,
                    0,
                )? {
                    let num4: i32 = subj6;
                    for j0 in 1..=nbj0 {
                        let mut an_it0: i32 = Default::default();
                        the_data.read_integer(num4, j0, "integer", the_check, &mut an_it0)?;
                        step_array2_set(&a_ctriangles, i0, j0, an_it0, "ReadStep")?;
                    }
                }
            }
        }
        the_ent.init(
            &a_representation_item_name,
            &a_tessellated_face_coordinates,
            a_tessellated_face_pnmax,
            &a_tessellated_face_normals,
            has_tessellated_face_geometric_link,
            &a_tessellated_face_geometric_link,
            &a_ctriangles,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualCubicBezierTriangulatedFace>,
    ) -> Result<(), Failure> {
        rw_block_b_0056!(i3, j3, the_ent, the_sw, var0);
        for i5 in 1..=step_live(&the_ent.ctriangles(), "WriteStep")?
            .array2()
            .col_length()
        {
            the_sw.new_line(false)?;
            the_sw.open_sub()?;
            for j5 in 1..=step_live(&the_ent.ctriangles(), "WriteStep")?
                .array2()
                .row_length()
            {
                let var0: i32 = step_array2_item(&the_ent.ctriangles(), i5, j5, "WriteStep")?;
                the_sw.send_integer(var0)?;
            }
            the_sw.close_sub()?;
        }
        the_sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualCubicBezierTriangulatedFace>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0215!(the_ent, the_iter);
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwCurveStyle, "RwStepVisualRwCurveStyle");
impl RwStepVisualRwCurveStyle {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCurveStyle>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "curve_style")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_curve_font: StepVisualCurveStyleFontSelect = Default::default();
        data.read_entity_select(num, 2, "curve_font", ach, &mut a_curve_font)?;
        let mut a_curve_width: StepBasicSizeSelect = Default::default();
        data.read_entity_select(num, 3, "curve_width", ach, &mut a_curve_width)?;
        let mut a_curve_colour: Handle<StepVisualColour> = Default::default();
        data.read_entity(
            num,
            4,
            "curve_colour",
            ach,
            StepVisualColour::static_type(),
            &mut a_curve_colour,
        )?;
        ent.init(&a_name, &a_curve_font, &a_curve_width, &a_curve_colour);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCurveStyle>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.curve_font().value())?;
        sw.send_entity(&ent.curve_width().value())?;
        sw.send_entity(&ent.curve_colour())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCurveStyle>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.curve_font().value());
        iter.get_one_item(&ent.curve_colour().cast());
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwCurveStyleFont, "RwStepVisualRwCurveStyleFont");
impl RwStepVisualRwCurveStyleFont {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCurveStyleFont>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "curve_style_font")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_pattern_list: Handle<HArray1<Handle<StepVisualCurveStyleFontPattern>>> =
            Default::default();
        let mut anent2: Handle<StepVisualCurveStyleFontPattern> = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "pattern_list", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_pattern_list = step_new_array1::<Handle<StepVisualCurveStyleFontPattern>>(1, nb2)?;
            for i2 in 1..=nb2 {
                if data.read_entity(
                    nsub2,
                    i2,
                    "curve_style_font_pattern",
                    ach,
                    StepVisualCurveStyleFontPattern::static_type(),
                    &mut anent2,
                )? {
                    step_array1_set(&a_pattern_list, i2, anent2.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_pattern_list);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCurveStyleFont>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_pattern_list() {
            sw.send_entity(&ent.pattern_list_value(i2)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCurveStyleFont>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_pattern_list();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.pattern_list_value(is1)?.cast());
        }
        Ok(())
    }
}
