// The RW member bodies several classes repeat.

macro_rules! rw_block_0008 {
    ($a_control_points_list:ident, $a_name:ident, $a_self_intersect:ident, $a_surface_form:ident, $a_u_closed:ident, $a_u_degree:ident, $a_v_closed:ident, $a_v_degree:ident, $ach:ident, $anent4:ident, $data:ident, $ent:ident, $i4:ident, $j4:ident, $nbi4:ident, $nbj4:ident, $nsi4:ident, $nsub4:ident, $num:ident, $text:ident) => {
        rw_block_0010!(
            $a_control_points_list,
            $a_name,
            $a_self_intersect,
            $a_surface_form,
            $a_u_closed,
            $a_u_degree,
            $a_v_closed,
            $a_v_degree,
            $ach,
            $anent4,
            $data,
            $i4,
            $j4,
            $nbi4,
            $nbj4,
            $nsi4,
            $nsub4,
            $num,
            $text
        );
        $ent.init(
            &$a_name,
            $a_u_degree,
            $a_v_degree,
            &$a_control_points_list,
            $a_surface_form,
            $a_u_closed,
            $a_v_closed,
            $a_self_intersect,
        );
    };
}
pub(crate) use rw_block_0008;

macro_rules! rw_block_0009 {
    ($ent:ident, $i3:ident, $i8:ident, $j3:ident, $j8:ident, $sw:ident) => {
        rw_block_0021!($ent, $i3, $j3, $sw);
        rw_block_0070!($ent, $i8, $j8, $sw);
    };
}
pub(crate) use rw_block_0009;

macro_rules! rw_block_0010 {
    ($a_control_points_list:ident, $a_name:ident, $a_self_intersect:ident, $a_surface_form:ident, $a_u_closed:ident, $a_u_degree:ident, $a_v_closed:ident, $a_v_degree:ident, $ach:ident, $anent4:ident, $data:ident, $i4:ident, $j4:ident, $nbi4:ident, $nbj4:ident, $nsi4:ident, $nsub4:ident, $num:ident, $text:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_u_degree: i32 = Default::default();
        $data.read_integer($num, 2, "u_degree", $ach, &mut $a_u_degree)?;
        let mut $a_v_degree: i32 = Default::default();
        $data.read_integer($num, 3, "v_degree", $ach, &mut $a_v_degree)?;
        let mut $a_control_points_list: Handle<HArray2<Handle<StepGeomCartesianPoint>>> =
            Default::default();
        let mut $anent4: Handle<StepGeomCartesianPoint> = Default::default();
        let mut $nsub4: i32 = Default::default();
        if $data.read_sub_list(
            $num,
            4,
            "control_points_list",
            $ach,
            &mut $nsub4,
            false,
            0,
            0,
        )? {
            let $nbi4: i32 = $data.nb_params($nsub4)?;
            let $nbj4: i32 = $data.nb_params($data.param_number($nsub4, 1)?)?;
            $a_control_points_list =
                step_new_array2::<Handle<StepGeomCartesianPoint>>(1, $nbi4, 1, $nbj4)?;
            for $i4 in 1..=$nbi4 {
                let mut $nsi4: i32 = Default::default();
                if $data.read_sub_list(
                    $nsub4,
                    $i4,
                    "sub-part(control_points_list)",
                    $ach,
                    &mut $nsi4,
                    false,
                    0,
                    0,
                )? {
                    for $j4 in 1..=$nbj4 {
                        if $data.read_entity(
                            $nsi4,
                            $j4,
                            "cartesian_point",
                            $ach,
                            StepGeomCartesianPoint::static_type(),
                            &mut $anent4,
                        )? {
                            step_array2_set(
                                &$a_control_points_list,
                                $i4,
                                $j4,
                                $anent4.clone(),
                                "ReadStep",
                            )?;
                        }
                    }
                }
            }
        }
        let mut $a_surface_form: StepGeomBSplineSurfaceForm =
            StepGeomBSplineSurfaceForm::BssfPlaneSurf;
        if $data.param_type($num, 5)? == InterfaceParamType::ParamEnum {
            let $text: Vec<u8> = $data.param_c_value($num, 5)?;
            if !RwStepGeomRwBSplineSurfaceForm::convert_to_enum(&$text, &mut $a_surface_form) {
                $ach.add_fail_text(
                    "Enumeration b_spline_surface_form has not an allowed value",
                    "",
                );
            }
        } else {
            $ach.add_fail_text("Parameter #5 (surface_form) is not an enumeration", "");
        }
        let mut $a_u_closed: StepDataLogical = Default::default();
        $data.read_logical($num, 6, "u_closed", $ach, &mut $a_u_closed)?;
        let mut $a_v_closed: StepDataLogical = Default::default();
        $data.read_logical($num, 7, "v_closed", $ach, &mut $a_v_closed)?;
        let mut $a_self_intersect: StepDataLogical = Default::default();
        $data.read_logical($num, 8, "self_intersect", $ach, &mut $a_self_intersect)?;
    };
}
pub(crate) use rw_block_0010;

macro_rules! rw_block_0011 {
    ($ent:ident, $i2:ident, $i6:ident, $sw:ident) => {
        rw_block_0029!($ent, $i2, $sw);
        rw_block_0095!($ent, $i6, $sw);
    };
}
pub(crate) use rw_block_0011;

macro_rules! rw_block_0012 {
    ($a_measure_with_unit:ident, $a_name:ident, $a_repr_item:ident, $a_unit_component:ident, $ach:ident, $data:ident, $ent:ident, $mvc:ident, $num:ident, $num0:ident) => {
        rw_block_0047!(
            $a_measure_with_unit,
            $a_unit_component,
            $ach,
            $data,
            $mvc,
            $num,
            $num0
        );
        rw_block_0087!($a_name, $a_repr_item, $ach, $data, $num, $num0);
        $ent.init(&$a_measure_with_unit, &$a_repr_item)?;
    };
}
pub(crate) use rw_block_0012;

macro_rules! rw_block_0013 {
    ($a_closed_curve:ident, $a_control_points_list:ident, $a_curve_form:ident, $a_degree:ident, $a_name:ident, $a_self_intersect:ident, $ach:ident, $anent3:ident, $data:ident, $ent:ident, $i3:ident, $nb3:ident, $nsub3:ident, $num:ident, $text:ident) => {
        rw_block_0014!(
            $a_closed_curve,
            $a_control_points_list,
            $a_curve_form,
            $a_degree,
            $a_name,
            $a_self_intersect,
            $ach,
            $anent3,
            $data,
            $i3,
            $nb3,
            $nsub3,
            $num,
            $text
        );
        $ent.init(
            &$a_name,
            $a_degree,
            &$a_control_points_list,
            $a_curve_form,
            $a_closed_curve,
            $a_self_intersect,
        );
    };
}
pub(crate) use rw_block_0013;

macro_rules! rw_block_0014 {
    ($a_closed_curve:ident, $a_control_points_list:ident, $a_curve_form:ident, $a_degree:ident, $a_name:ident, $a_self_intersect:ident, $ach:ident, $anent3:ident, $data:ident, $i3:ident, $nb3:ident, $nsub3:ident, $num:ident, $text:ident) => {
        rw_block_0074!(
            $a_control_points_list,
            $a_degree,
            $a_name,
            $ach,
            $anent3,
            $data,
            $nsub3,
            $num
        );
        if $data.read_sub_list(
            $num,
            3,
            "control_points_list",
            $ach,
            &mut $nsub3,
            false,
            0,
            0,
        )? {
            let $nb3: i32 = $data.nb_params($nsub3)?;
            $a_control_points_list = step_new_array1::<Handle<StepGeomCartesianPoint>>(1, $nb3)?;
            for $i3 in 1..=$nb3 {
                if $data.read_entity(
                    $nsub3,
                    $i3,
                    "cartesian_point",
                    $ach,
                    StepGeomCartesianPoint::static_type(),
                    &mut $anent3,
                )? {
                    step_array1_set(&$a_control_points_list, $i3, $anent3.clone(), "ReadStep")?;
                }
            }
        }
        rw_block_0080!(
            $a_closed_curve,
            $a_curve_form,
            $a_self_intersect,
            $ach,
            $data,
            $num,
            $text
        );
    };
}
pub(crate) use rw_block_0014;

macro_rules! rw_block_0015 {
    ($a_assembly_component_usage_reference_designator:ident, $a_product_definition_relationship_description:ident, $a_product_definition_relationship_id:ident, $a_product_definition_relationship_name:ident, $a_product_definition_relationship_related_product_definition:ident, $a_product_definition_relationship_relating_product_definition:ident, $ach:ident, $data:ident, $has_assembly_component_usage_reference_designator:ident, $has_product_definition_relationship_description:ident, $num:ident) => {
        rw_block_0025!(
            $a_product_definition_relationship_description,
            $a_product_definition_relationship_id,
            $a_product_definition_relationship_name,
            $a_product_definition_relationship_related_product_definition,
            $a_product_definition_relationship_relating_product_definition,
            $ach,
            $data,
            $has_product_definition_relationship_description,
            $num
        );
        let mut $a_assembly_component_usage_reference_designator: Handle<HAsciiString> =
            Default::default();
        let mut $has_assembly_component_usage_reference_designator: bool = true;
        if $data.is_param_defined($num, 6)? {
            $data.read_string(
                $num,
                6,
                "assembly_component_usage.reference_designator",
                $ach,
                &mut $a_assembly_component_usage_reference_designator,
            )?;
        } else {
            $has_assembly_component_usage_reference_designator = false;
        }
    };
}
pub(crate) use rw_block_0015;

macro_rules! rw_block_0016 {
    ($a_closed_curve:ident, $a_control_points_list:ident, $a_curve_form:ident, $a_degree:ident, $a_self_intersect:ident, $ach:ident, $anent2:ident, $data:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident, $text:ident) => {
        if !$data.check_nb_params($num, 5, $ach, "b_spline_curve")? {
            return Ok(());
        }
        let mut $a_degree: i32 = Default::default();
        $data.read_integer($num, 1, "degree", $ach, &mut $a_degree)?;
        let mut $a_control_points_list: Handle<HArray1<Handle<StepGeomCartesianPoint>>> =
            Default::default();
        let mut $anent2: Handle<StepGeomCartesianPoint> = Default::default();
        let mut $nsub2: i32 = Default::default();
        if $data.read_sub_list(
            $num,
            2,
            "control_points_list",
            $ach,
            &mut $nsub2,
            false,
            0,
            0,
        )? {
            let $nb2: i32 = $data.nb_params($nsub2)?;
            $a_control_points_list = step_new_array1::<Handle<StepGeomCartesianPoint>>(1, $nb2)?;
            for $i2 in 1..=$nb2 {
                if $data.read_entity(
                    $nsub2,
                    $i2,
                    "cartesian_point",
                    $ach,
                    StepGeomCartesianPoint::static_type(),
                    &mut $anent2,
                )? {
                    step_array1_set(&$a_control_points_list, $i2, $anent2.clone(), "ReadStep")?;
                }
            }
        }
        let mut $a_curve_form: StepGeomBSplineCurveForm =
            StepGeomBSplineCurveForm::BscfPolylineForm;
        if $data.param_type($num, 3)? == InterfaceParamType::ParamEnum {
            let $text: Vec<u8> = $data.param_c_value($num, 3)?;
            if !RwStepGeomRwBSplineCurveForm::convert_to_enum(&$text, &mut $a_curve_form) {
                $ach.add_fail_text(
                    "Enumeration b_spline_curve_form has not an allowed value",
                    "",
                );
            }
        } else {
            $ach.add_fail_text("Parameter #3 (curve_form) is not an enumeration", "");
        }
        let mut $a_closed_curve: StepDataLogical = Default::default();
        $data.read_logical($num, 4, "closed_curve", $ach, &mut $a_closed_curve)?;
        let mut $a_self_intersect: StepDataLogical = Default::default();
        $data.read_logical($num, 5, "self_intersect", $ach, &mut $a_self_intersect)?;
    };
}
pub(crate) use rw_block_0016;
