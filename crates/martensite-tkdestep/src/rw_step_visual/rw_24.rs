#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwTextStyleForDefinedFont,
    "RwStepVisualRwTextStyleForDefinedFont"
);
impl RwStepVisualRwTextStyleForDefinedFont {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualTextStyleForDefinedFont>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            1,
            ach,
            "text_style_for_defined_font has not 1 parameter(s)",
        )? {
            return Ok(());
        }
        let mut a_text_colour: Handle<StepVisualColour> = Default::default();
        data.read_entity(
            num,
            1,
            "text_colour",
            ach,
            StepVisualColour::static_type(),
            &mut a_text_colour,
        )?;
        ent.init(&a_text_colour);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualTextStyleForDefinedFont>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.text_colour())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualTextStyleForDefinedFont>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.text_colour().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTextStyleWithBoxCharacteristics,
    "RwStepVisualRwTextStyleWithBoxCharacteristics"
);
impl RwStepVisualRwTextStyleWithBoxCharacteristics {
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualTextStyleWithBoxCharacteristics>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.character_appearance())?;
        sw.send_entity(&ent.characteristics())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualTextStyleWithBoxCharacteristics>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.character_appearance().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTriangulatedFace,
    "RwStepVisualRwTriangulatedFace"
);
impl RwStepVisualRwTriangulatedFace {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTriangulatedFace>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 7, the_check, "triangulated_face")? {
            return Ok(());
        }
        rw_block_b_0015!(
            a_pnindex,
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
            num2,
            num4,
            sub4,
            sub6,
            subj4,
            the_check,
            the_data,
            the_num
        );
        let mut a_triangles: Handle<HArray2<i32>> = Default::default();
        let mut sub7: i32 = 0;
        if the_data.read_sub_list(the_num, 7, "triangles", the_check, &mut sub7, false, 0, 0)? {
            let nb0: i32 = the_data.nb_params(sub7)?;
            let nbj0: i32 = the_data.nb_params(the_data.param_number(sub7, 1)?)?;
            a_triangles = step_new_array2::<i32>(1, nb0, 1, nbj0)?;
            for i0 in 1..=nb0 {
                let mut subj7: i32 = 0;
                if the_data.read_sub_list(
                    sub7,
                    i0,
                    "sub-part(triangles)",
                    the_check,
                    &mut subj7,
                    false,
                    0,
                    0,
                )? {
                    let num4: i32 = subj7;
                    for j0 in 1..=nbj0 {
                        let mut an_it0: i32 = Default::default();
                        the_data.read_integer(num4, j0, "integer", the_check, &mut an_it0)?;
                        step_array2_set(&a_triangles, i0, j0, an_it0, "ReadStep")?;
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
            &a_pnindex,
            &a_triangles,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTriangulatedFace>,
    ) -> Result<(), Failure> {
        rw_block_b_0031!(i3, i5, j3, the_ent, the_sw, var0);
        for i6 in 1..=step_live(&the_ent.triangles(), "WriteStep")?
            .array2()
            .col_length()
        {
            the_sw.new_line(false)?;
            the_sw.open_sub()?;
            for j6 in 1..=step_live(&the_ent.triangles(), "WriteStep")?
                .array2()
                .row_length()
            {
                let var0: i32 = step_array2_item(&the_ent.triangles(), i6, j6, "WriteStep")?;
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
        the_ent: &Handle<StepVisualTriangulatedFace>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0215!(the_ent, the_iter);
        Ok(())
    }
}
