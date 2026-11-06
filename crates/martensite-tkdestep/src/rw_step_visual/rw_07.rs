#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwComplexTriangulatedSurfaceSet,
    "RwStepVisualRwComplexTriangulatedSurfaceSet"
);
impl RwStepVisualRwComplexTriangulatedSurfaceSet {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualComplexTriangulatedSurfaceSet>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 7, the_check, "complex_triangulated_surface_set")? {
            return Ok(());
        }
        rw_block_b_0075!(
            a_representation_item_name,
            a_tessellated_surface_set_coordinates,
            a_tessellated_surface_set_normals,
            a_tessellated_surface_set_pnmax,
            the_check,
            the_data,
            the_num
        );
        let mut sub4: i32 = 0;
        if the_data.read_sub_list(
            the_num,
            4,
            "tessellated_surface_set.normals",
            the_check,
            &mut sub4,
            false,
            0,
            0,
        )? {
            let nb0: i32 = the_data.nb_params(sub4)?;
            let nbj0: i32 = the_data.nb_params(the_data.param_number(sub4, 1)?)?;
            a_tessellated_surface_set_normals = step_new_array2::<f64>(1, nb0, 1, nbj0)?;
            for i0 in 1..=nb0 {
                let mut subj4: i32 = 0;
                if the_data.read_sub_list(
                    sub4,
                    i0,
                    "sub-part(tessellated_surface_set.normals)",
                    the_check,
                    &mut subj4,
                    false,
                    0,
                    0,
                )? {
                    let num4: i32 = subj4;
                    for j0 in 1..=nbj0 {
                        let mut an_it0: f64 = Default::default();
                        the_data.read_real(num4, j0, "real", the_check, &mut an_it0)?;
                        step_array2_set(
                            &a_tessellated_surface_set_normals,
                            i0,
                            j0,
                            an_it0,
                            "ReadStep",
                        )?;
                    }
                }
            }
        }
        let mut a_pnindex: Handle<HArray1<i32>> = Default::default();
        let mut sub5: i32 = 0;
        if the_data.read_sub_list(the_num, 5, "pnindex", the_check, &mut sub5, false, 0, 0)? {
            let nb0: i32 = the_data.nb_params(sub5)?;
            a_pnindex = step_new_array1::<i32>(1, nb0)?;
            let num2: i32 = sub5;
            for i0 in 1..=nb0 {
                let mut an_it0: i32 = Default::default();
                the_data.read_integer(num2, i0, "integer", the_check, &mut an_it0)?;
                step_array1_set(&a_pnindex, i0, an_it0, "ReadStep")?;
            }
        }
        let mut a_triangle_strips: Handle<HArray1<Handle<StandardTransient>>> = Default::default();
        let mut sub6: i32 = 0;
        if the_data.read_sub_list(
            the_num,
            6,
            "triangle_strips",
            the_check,
            &mut sub6,
            false,
            0,
            0,
        )? {
            let nb0: i32 = the_data.nb_params(sub6)?;
            a_triangle_strips = step_new_array1::<Handle<StandardTransient>>(1, nb0)?;
            for i0 in 1..=nb0 {
                let nbj0: i32 = the_data.nb_params(the_data.param_number(sub6, i0)?)?;
                let a_single_triangle_strip: Handle<HArray1<i32>> =
                    step_new_array1::<i32>(1, nbj0)?;
                let mut subj6: i32 = 0;
                if the_data.read_sub_list(
                    sub6,
                    i0,
                    "sub-part(triangle_strips)",
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
        let mut sub7: i32 = 0;
        if the_data.read_sub_list(
            the_num,
            7,
            "triangle_fans",
            the_check,
            &mut sub7,
            false,
            0,
            0,
        )? {
            let nb0: i32 = the_data.nb_params(sub7)?;
            a_triangle_fans = step_new_array1::<Handle<StandardTransient>>(1, nb0)?;
            for i0 in 1..=nb0 {
                let nbj0: i32 = the_data.nb_params(the_data.param_number(sub7, i0)?)?;
                let a_single_triangle_fan: Handle<HArray1<i32>> = step_new_array1::<i32>(1, nbj0)?;
                let mut subj7: i32 = 0;
                if the_data.read_sub_list(
                    sub7,
                    i0,
                    "sub-part(triangle_fans)",
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
            &a_tessellated_surface_set_coordinates,
            a_tessellated_surface_set_pnmax,
            &a_tessellated_surface_set_normals,
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
        the_ent: &Handle<StepVisualComplexTriangulatedSurfaceSet>,
    ) -> Result<(), Failure> {
        rw_block_b_0166!(the_ent, the_sw);
        if the_ent.nb_normals() != 0 {
            for i3 in 1..=step_live(&the_ent.normals(), "WriteStep")?
                .array2()
                .col_length()
            {
                the_sw.new_line(false)?;
                the_sw.open_sub()?;
                for j3 in 1..=step_live(&the_ent.normals(), "WriteStep")?
                    .array2()
                    .row_length()
                {
                    let var0: f64 = step_array2_item(&the_ent.normals(), i3, j3, "WriteStep")?;
                    the_sw.send_real(var0)?;
                }
                the_sw.close_sub()?;
            }
        }
        the_sw.close_sub()?;
        the_sw.open_sub()?;
        for i4 in 1..=step_live(&the_ent.pnindex(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: i32 = step_array1_item(&the_ent.pnindex(), i4, "WriteStep")?;
            the_sw.send_integer(var0)?;
        }
        the_sw.close_sub()?;
        the_sw.open_sub()?;
        for i5 in 1..=the_ent.nb_triangle_strips() {
            the_sw.new_line(false)?;
            the_sw.open_sub()?;
            let a_triangle_strip: Handle<HArray1<i32>> =
                step_array1_item(&the_ent.triangle_strips(), i5, "WriteStep")?
                    .cast::<HArray1<i32>>();
            for j5 in 1..=step_live(&a_triangle_strip, "WriteStep")?.array1().length() {
                let var0: i32 = step_array1_item(&a_triangle_strip, j5, "WriteStep")?;
                the_sw.send_integer(var0)?;
            }
            the_sw.close_sub()?;
        }
        the_sw.close_sub()?;
        the_sw.open_sub()?;
        for i6 in 1..=the_ent.nb_triangle_fans() {
            the_sw.new_line(false)?;
            the_sw.open_sub()?;
            let a_triangle_fan: Handle<HArray1<i32>> =
                step_array1_item(&the_ent.triangle_fans(), i6, "WriteStep")?.cast::<HArray1<i32>>();
            for j6 in 1..=step_live(&a_triangle_fan, "WriteStep")?.array1().length() {
                let var0: i32 = step_array1_item(&a_triangle_fan, j6, "WriteStep")?;
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
        the_ent: &Handle<StepVisualComplexTriangulatedSurfaceSet>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        the_iter.add_item(&the_ent.parent.coordinates().cast());
        Ok(())
    }
}
