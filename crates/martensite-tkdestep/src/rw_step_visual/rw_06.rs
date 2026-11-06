#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwComplexTriangulatedFace,
    "RwStepVisualRwComplexTriangulatedFace"
);
impl RwStepVisualRwComplexTriangulatedFace {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualComplexTriangulatedFace>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 8, the_check, "complex_triangulated_face")? {
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
        let mut a_triangle_strips: Handle<HArray1<Handle<StandardTransient>>> = Default::default();
        let mut sub7: i32 = 0;
        if the_data.read_sub_list(
            the_num,
            7,
            "triangle_strips",
            the_check,
            &mut sub7,
            false,
            0,
            0,
        )? {
            let nb0: i32 = the_data.nb_params(sub7)?;
            a_triangle_strips = step_new_array1::<Handle<StandardTransient>>(1, nb0)?;
            for i0 in 1..=nb0 {
                let nbj0: i32 = the_data.nb_params(the_data.param_number(sub7, i0)?)?;
                let a_single_triangle_strip: Handle<HArray1<i32>> =
                    step_new_array1::<i32>(1, nbj0)?;
                let mut subj7: i32 = 0;
                if the_data.read_sub_list(
                    sub7,
                    i0,
                    "sub-part(triangle_strips)",
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
                        step_array1_set(&a_single_triangle_strip, j0, an_it0, "ReadStep")?;
                    }
                    step_array1_set(
                        &a_triangle_strips,
                        i0,
                        a_single_triangle_strip.cast(),
                        "ReadStep",
                    )?;
                }
            }
        }
        let mut a_triangle_fans: Handle<HArray1<Handle<StandardTransient>>> = Default::default();
        let mut sub8: i32 = 0;
        if the_data.read_sub_list(
            the_num,
            8,
            "triangle_fans",
            the_check,
            &mut sub8,
            false,
            0,
            0,
        )? {
            let nb0: i32 = the_data.nb_params(sub8)?;
            a_triangle_fans = step_new_array1::<Handle<StandardTransient>>(1, nb0)?;
            for i0 in 1..=nb0 {
                let nbj0: i32 = the_data.nb_params(the_data.param_number(sub8, i0)?)?;
                let a_single_triangle_fan: Handle<HArray1<i32>> = step_new_array1::<i32>(1, nbj0)?;
                let mut subj8: i32 = 0;
                if the_data.read_sub_list(
                    sub8,
                    i0,
                    "sub-part(triangle_fans)",
                    the_check,
                    &mut subj8,
                    false,
                    0,
                    0,
                )? {
                    let num4: i32 = subj8;
                    for j0 in 1..=nbj0 {
                        let mut an_it0: i32 = Default::default();
                        the_data.read_integer(num4, j0, "integer", the_check, &mut an_it0)?;
                        step_array1_set(&a_single_triangle_fan, j0, an_it0, "ReadStep")?;
                    }
                    step_array1_set(
                        &a_triangle_fans,
                        i0,
                        a_single_triangle_fan.cast(),
                        "ReadStep",
                    )?;
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
            &a_triangle_strips,
            &a_triangle_fans,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualComplexTriangulatedFace>,
    ) -> Result<(), Failure> {
        rw_block_b_0031!(i3, i5, j3, the_ent, the_sw, var0);
        for i6 in 1..=the_ent.nb_triangle_strips() {
            the_sw.new_line(false)?;
            the_sw.open_sub()?;
            let a_triangle_strip: Handle<HArray1<i32>> =
                step_array1_item(&the_ent.triangle_strips(), i6, "WriteStep")?
                    .cast::<HArray1<i32>>();
            for j6 in 1..=step_live(&a_triangle_strip, "WriteStep")?.array1().length() {
                let var0: i32 = step_array1_item(&a_triangle_strip, j6, "WriteStep")?;
                the_sw.send_integer(var0)?;
            }
            the_sw.close_sub()?;
        }
        the_sw.close_sub()?;
        the_sw.open_sub()?;
        for i7 in 1..=the_ent.nb_triangle_fans() {
            the_sw.new_line(false)?;
            the_sw.open_sub()?;
            let a_triangle_fan: Handle<HArray1<i32>> =
                step_array1_item(&the_ent.triangle_fans(), i7, "WriteStep")?.cast::<HArray1<i32>>();
            for j7 in 1..=step_live(&a_triangle_fan, "WriteStep")?.array1().length() {
                let var0: i32 = step_array1_item(&a_triangle_fan, j7, "WriteStep")?;
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
        the_ent: &Handle<StepVisualComplexTriangulatedFace>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0215!(the_ent, the_iter);
        Ok(())
    }
}
