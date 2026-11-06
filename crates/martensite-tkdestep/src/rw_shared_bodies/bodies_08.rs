// The RW member bodies several classes repeat.

macro_rules! rw_block_0057 {
    ($a_property_definition_definition:ident, $a_property_definition_description:ident, $a_property_definition_name:ident, $ach:ident, $data:ident, $ent:ident, $has_property_definition_description:ident, $num:ident) => {
        rw_block_0196!(
            $a_property_definition_description,
            $a_property_definition_name,
            $ach,
            $data,
            $num
        );
        let mut $has_property_definition_description: bool = true;
        if $data.is_param_defined($num, 2)? {
            $data.read_string(
                $num,
                2,
                "property_definition.description",
                $ach,
                &mut $a_property_definition_description,
            )?;
        } else {
            $has_property_definition_description = false;
        }
        let mut $a_property_definition_definition: StepReprCharacterizedDefinition =
            Default::default();
        $data.read_entity_select(
            $num,
            3,
            "property_definition.definition",
            $ach,
            &mut $a_property_definition_definition,
        )?;
        $ent.init(
            &$a_property_definition_name,
            $has_property_definition_description,
            &$a_property_definition_description,
            &$a_property_definition_definition,
        );
    };
}
pub(crate) use rw_block_0057;

macro_rules! rw_block_0058 {
    ($a_control_points_list:ident, $a_u_degree:ident, $a_v_degree:ident, $ach:ident, $anent3:ident, $data:ident, $nsub3:ident, $num:ident) => {
        if !$data.check_nb_params($num, 7, $ach, "b_spline_surface")? {
            return Ok(());
        }
        let mut $a_u_degree: i32 = Default::default();
        $data.read_integer($num, 1, "u_degree", $ach, &mut $a_u_degree)?;
        let mut $a_v_degree: i32 = Default::default();
        $data.read_integer($num, 2, "v_degree", $ach, &mut $a_v_degree)?;
        let mut $a_control_points_list: Handle<HArray2<Handle<StepGeomCartesianPoint>>> =
            Default::default();
        let mut $anent3: Handle<StepGeomCartesianPoint> = Default::default();
        let mut $nsub3: i32 = Default::default();
    };
}
pub(crate) use rw_block_0058;

macro_rules! rw_block_0059 {
    ($a_major_radius:ident, $a_minor_radius:ident, $a_name:ident, $a_position:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0140!($a_name, $a_position, $ach, $data, $num);
        let mut $a_major_radius: f64 = Default::default();
        rw_block_0204!($a_major_radius, $a_minor_radius, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0059;

macro_rules! rw_block_0060 {
    ($a_height:ident, $a_name:ident, $a_position:ident, $a_radius:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0139!($a_name, $a_position, $ach, $data, $num);
        let mut $a_height: f64 = Default::default();
        $data.read_real($num, 3, "height", $ach, &mut $a_height)?;
        let mut $a_radius: f64 = Default::default();
        $data.read_real($num, 4, "radius", $ach, &mut $a_radius)?;
    };
}
pub(crate) use rw_block_0060;

macro_rules! rw_block_0061 {
    ($ent:ident, $i3:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_integer($ent.degree())?;
        $sw.open_sub()?;
        for $i3 in 1..=$ent.nb_control_points_list() {
            $sw.send_entity(&$ent.control_points_list_value($i3)?)?;
        }
        rw_block_0169!($ent, $sw);
    };
}
pub(crate) use rw_block_0061;

macro_rules! rw_block_0062 {
    ($a_name:ident, $a_weights_data:ident, $a_weights_data_item:ident, $ach:ident, $data:ident, $i8:ident, $j8:ident, $nbi8:ident, $nbj8:ident, $nsi8:ident, $nsub8:ident, $num:ident) => {
        if $data.read_sub_list($num, 1, "weights_data", $ach, &mut $nsub8, false, 0, 0)? {
            let $nbi8: i32 = $data.nb_params($nsub8)?;
            let $nbj8: i32 = $data.nb_params($data.param_number($nsub8, 1)?)?;
            $a_weights_data = step_new_array2::<f64>(1, $nbi8, 1, $nbj8)?;
            for $i8 in 1..=$nbi8 {
                let mut $nsi8: i32 = Default::default();
                if $data.read_sub_list(
                    $nsub8,
                    $i8,
                    "sub-part(weights_data)",
                    $ach,
                    &mut $nsi8,
                    false,
                    0,
                    0,
                )? {
                    for $j8 in 1..=$nbj8 {
                        if $data.read_real(
                            $nsi8,
                            $j8,
                            "weights_data",
                            $ach,
                            &mut $a_weights_data_item,
                        )? {
                            step_array2_set(
                                &$a_weights_data,
                                $i8,
                                $j8,
                                $a_weights_data_item,
                                "ReadStep",
                            )?;
                        }
                    }
                }
            }
        }
        rw_block_0089!($a_name, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0062;

macro_rules! rw_block_0063 {
    ($a_name:ident, $a_prefix:ident, $ach:ident, $data:ident, $ent:ident, $has_aprefix:ident, $num:ident, $text:ident) => {
        rw_block_0096!($a_name, $a_prefix, $ach, $data, $has_aprefix, $num, $text);
        rw_block_0214!($a_name, $ach, $data, $num, $text);
        $ent.init($has_aprefix, $a_prefix, $a_name);
    };
}
pub(crate) use rw_block_0063;

macro_rules! rw_block_0064 {
    ($a_name:ident, $a_prefix:ident, $ach:ident, $data:ident, $has_aprefix:ident, $num:ident, $text:ident) => {
        $num = $data.next_for_complex($num);
        if !$data.check_nb_params($num, 2, $ach, "si_unit")? {
            return Ok(());
        }
        let mut $a_prefix: StepBasicSiPrefix = StepBasicSiPrefix::SpExa;
        let mut $has_aprefix: bool = false;
        if $data.is_param_defined($num, 1)? {
            if $data.param_type($num, 1)? == InterfaceParamType::ParamEnum {
                let $text: Vec<u8> = $data.param_c_value($num, 1)?;
                $has_aprefix = RwStepBasicRwSiPrefix::convert_to_enum(&$text, &mut $a_prefix);
                if !$has_aprefix {
                    $ach.add_fail_text("Enumeration si_prefix has not an allowed value", "");
                    return Ok(());
                }
            } else {
                $ach.add_fail_text("Parameter #1 (prefix) is not an enumeration", "");
                return Ok(());
            }
        }
        let mut $a_name: StepBasicSiUnitName = Default::default();
        rw_block_0214!($a_name, $ach, $data, $num, $text);
    };
}
pub(crate) use rw_block_0064;

macro_rules! rw_block_0065 {
    ($a_cfs_faces:ident, $a_name:ident, $ach:ident, $anent2:ident, $data:ident, $ent:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_cfs_faces: Handle<HArray1<Handle<StepShapeFace>>> = Default::default();
        let mut $anent2: Handle<StepShapeFace> = Default::default();
        let mut $nsub2: i32 = Default::default();
        if $data.read_sub_list($num, 2, "cfs_faces", $ach, &mut $nsub2, false, 0, 0)? {
            let $nb2: i32 = $data.nb_params($nsub2)?;
            $a_cfs_faces = step_new_array1::<Handle<StepShapeFace>>(1, $nb2)?;
            for $i2 in 1..=$nb2 {
                if $data.read_entity(
                    $nsub2,
                    $i2,
                    "face",
                    $ach,
                    StepShapeFace::static_type(),
                    &mut $anent2,
                )? {
                    step_array1_set(&$a_cfs_faces, $i2, $anent2.clone(), "ReadStep")?;
                }
            }
        }
        $ent.init(&$a_name, &$a_cfs_faces);
    };
}
pub(crate) use rw_block_0065;
