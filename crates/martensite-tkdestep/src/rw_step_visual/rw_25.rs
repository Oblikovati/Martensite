#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwTriangulatedSurfaceSet,
    "RwStepVisualRwTriangulatedSurfaceSet"
);
impl RwStepVisualRwTriangulatedSurfaceSet {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTriangulatedSurfaceSet>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 6, the_check, "triangulated_surface_set")? {
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
        let mut a_norm_sub4: i32 = 0;
        if the_data.read_sub_list(
            the_num,
            4,
            "tessellated_surface_set.normals",
            the_check,
            &mut a_norm_sub4,
            false,
            0,
            0,
        )? {
            let a_norm_nb: i32 = the_data.nb_params(a_norm_sub4)?;
            let an_axes_nb: i32 = the_data.nb_params(the_data.param_number(a_norm_sub4, 1)?)?;
            a_tessellated_surface_set_normals =
                step_new_array2::<f64>(1, a_norm_nb, 1, an_axes_nb)?;
            for a_norm_index in 1..=a_norm_nb {
                let mut an_axes_sub4: i32 = 0;
                if the_data.read_sub_list(
                    a_norm_sub4,
                    a_norm_index,
                    "sub-part(tessellated_surface_set.normals)",
                    the_check,
                    &mut an_axes_sub4,
                    false,
                    0,
                    0,
                )? {
                    let an_axis_num4: i32 = an_axes_sub4;
                    for an_axis_index in 1..=an_axes_nb {
                        let mut a_value: f64 = Default::default();
                        the_data.read_real(
                            an_axis_num4,
                            an_axis_index,
                            "real",
                            the_check,
                            &mut a_value,
                        )?;
                        step_array2_set(
                            &a_tessellated_surface_set_normals,
                            a_norm_index,
                            an_axis_index,
                            a_value,
                            "ReadStep",
                        )?;
                    }
                }
            }
        }
        let mut a_pnindex: Handle<HArray1<i32>> = Default::default();
        let mut a_pn_sub5: i32 = 0;
        if the_data.read_sub_list(
            the_num,
            5,
            "pnindex",
            the_check,
            &mut a_pn_sub5,
            false,
            0,
            0,
        )? {
            let a_pn_nb: i32 = the_data.nb_params(a_pn_sub5)?;
            a_pnindex = step_new_array1::<i32>(1, a_pn_nb)?;
            let a_pn_num2: i32 = a_pn_sub5;
            for a_pn_index in 1..=a_pn_nb {
                let mut a_value: i32 = Default::default();
                the_data.read_integer(a_pn_num2, a_pn_index, "integer", the_check, &mut a_value)?;
                step_array1_set(&a_pnindex, a_pn_index, a_value, "ReadStep")?;
            }
        }
        let mut a_triangles: Handle<HArray2<i32>> = Default::default();
        let mut a_tr_sub6: i32 = 0;
        if the_data.read_sub_list(
            the_num,
            6,
            "triangles",
            the_check,
            &mut a_tr_sub6,
            false,
            0,
            0,
        )? {
            let a_tria_nb: i32 = the_data.nb_params(a_tr_sub6)?;
            let an_axes_nb: i32 = the_data.nb_params(the_data.param_number(a_tr_sub6, 1)?)?;
            a_triangles = step_new_array2::<i32>(1, a_tria_nb, 1, an_axes_nb)?;
            for a_tria_index in 1..=a_tria_nb {
                let mut an_axes_subj6: i32 = 0;
                if the_data.read_sub_list(
                    a_tr_sub6,
                    a_tria_index,
                    "sub-part(triangles)",
                    the_check,
                    &mut an_axes_subj6,
                    false,
                    0,
                    0,
                )? {
                    let an_axis_num4: i32 = an_axes_subj6;
                    for an_axis_index in 1..=an_axes_nb {
                        let mut a_value: i32 = Default::default();
                        the_data.read_integer(
                            an_axis_num4,
                            an_axis_index,
                            "integer",
                            the_check,
                            &mut a_value,
                        )?;
                        step_array2_set(
                            &a_triangles,
                            a_tria_index,
                            an_axis_index,
                            a_value,
                            "ReadStep",
                        )?;
                    }
                }
            }
        }
        the_ent.init(
            &a_representation_item_name,
            &a_tessellated_surface_set_coordinates,
            a_tessellated_surface_set_pnmax,
            &a_tessellated_surface_set_normals,
            &a_pnindex,
            &a_triangles,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTriangulatedSurfaceSet>,
    ) -> Result<(), Failure> {
        rw_block_b_0166!(the_ent, the_sw);
        for a_norm_index in 1..=step_live(&the_ent.normals(), "WriteStep")?
            .array2()
            .col_length()
        {
            the_sw.new_line(false)?;
            the_sw.open_sub()?;
            for an_axis_index in 1..=step_live(&the_ent.normals(), "WriteStep")?
                .array2()
                .row_length()
            {
                let a_value: f64 =
                    step_array2_item(&the_ent.normals(), a_norm_index, an_axis_index, "WriteStep")?;
                the_sw.send_real(a_value)?;
            }
            the_sw.close_sub()?;
        }
        the_sw.close_sub()?;
        the_sw.open_sub()?;
        for i in step_live(&the_ent.pnindex(), "WriteStep")?.array1().lower()
            ..=step_live(&the_ent.pnindex(), "WriteStep")?.array1().upper()
        {
            the_sw.send_integer(step_array1_item(&the_ent.pnindex(), i, "WriteStep")?)?;
        }
        the_sw.close_sub()?;
        the_sw.open_sub()?;
        for a_tria_index in 1..=step_live(&the_ent.triangles(), "WriteStep")?
            .array2()
            .col_length()
        {
            the_sw.new_line(false)?;
            the_sw.open_sub()?;
            for an_axis_index in 1..=step_live(&the_ent.triangles(), "WriteStep")?
                .array2()
                .row_length()
            {
                let a_value: i32 = step_array2_item(
                    &the_ent.triangles(),
                    a_tria_index,
                    an_axis_index,
                    "WriteStep",
                )?;
                the_sw.send_integer(a_value)?;
            }
            the_sw.close_sub()?;
        }
        the_sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualTriangulatedSurfaceSet>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        the_iter.add_item(&the_ent.parent.coordinates().cast());
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwViewVolume, "RwStepVisualRwViewVolume");
impl RwStepVisualRwViewVolume {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualViewVolume>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 9, ach, "view_volume")? {
            return Ok(());
        }
        let mut a_projection_type: StepVisualCentralOrParallel =
            StepVisualCentralOrParallel::CopCentral;
        if data.param_type(num, 1)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 1)?;
            if !RwStepVisualRwCentralOrParallel::convert_to_enum(&text, &mut a_projection_type) {
                ach.add_fail_text(
                    "Enumeration central_or_parallel has not an allowed value",
                    "",
                );
            }
        } else {
            ach.add_fail_text("Parameter #1 (projection_type) is not an enumeration", "");
        }
        let mut a_projection_point: Handle<StepGeomCartesianPoint> = Default::default();
        data.read_entity(
            num,
            2,
            "projection_point",
            ach,
            StepGeomCartesianPoint::static_type(),
            &mut a_projection_point,
        )?;
        let mut a_view_plane_distance: f64 = Default::default();
        data.read_real(
            num,
            3,
            "view_plane_distance",
            ach,
            &mut a_view_plane_distance,
        )?;
        let mut a_front_plane_distance: f64 = Default::default();
        data.read_real(
            num,
            4,
            "front_plane_distance",
            ach,
            &mut a_front_plane_distance,
        )?;
        let mut a_front_plane_clipping: bool = Default::default();
        data.read_boolean(
            num,
            5,
            "front_plane_clipping",
            ach,
            &mut a_front_plane_clipping,
        )?;
        let mut a_back_plane_distance: f64 = Default::default();
        data.read_real(
            num,
            6,
            "back_plane_distance",
            ach,
            &mut a_back_plane_distance,
        )?;
        let mut a_back_plane_clipping: bool = Default::default();
        data.read_boolean(
            num,
            7,
            "back_plane_clipping",
            ach,
            &mut a_back_plane_clipping,
        )?;
        let mut a_view_volume_sides_clipping: bool = Default::default();
        data.read_boolean(
            num,
            8,
            "view_volume_sides_clipping",
            ach,
            &mut a_view_volume_sides_clipping,
        )?;
        let mut a_view_window: Handle<StepVisualPlanarBox> = Default::default();
        data.read_entity(
            num,
            9,
            "view_window",
            ach,
            StepVisualPlanarBox::static_type(),
            &mut a_view_window,
        )?;
        ent.init(
            a_projection_type,
            &a_projection_point,
            a_view_plane_distance,
            a_front_plane_distance,
            a_front_plane_clipping,
            a_back_plane_distance,
            a_back_plane_clipping,
            a_view_volume_sides_clipping,
            &a_view_window,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualViewVolume>,
    ) -> Result<(), Failure> {
        sw.send_enum(RwStepVisualRwCentralOrParallel::convert_to_string(
            ent.projection_type(),
        ))?;
        sw.send_entity(&ent.projection_point())?;
        sw.send_real(ent.view_plane_distance())?;
        sw.send_real(ent.front_plane_distance())?;
        sw.send_boolean(ent.front_plane_clipping())?;
        sw.send_real(ent.back_plane_distance())?;
        sw.send_boolean(ent.back_plane_clipping())?;
        sw.send_boolean(ent.view_volume_sides_clipping())?;
        sw.send_entity(&ent.view_window())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualViewVolume>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.projection_point().cast());
        iter.get_one_item(&ent.view_window().cast());
        Ok(())
    }
}
