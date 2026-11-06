// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0026 {
    ($a_representation_item_name:ident, $a_tessellated_face_coordinates:ident, $a_tessellated_face_geometric_link:ident, $a_tessellated_face_normals:ident, $a_tessellated_face_pnmax:ident, $an_it0:ident, $has_tessellated_face_geometric_link:ident, $i0:ident, $j0:ident, $nb0:ident, $nbj0:ident, $num4:ident, $sub4:ident, $subj4:ident, $the_check:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            1,
            "representation_item.name",
            $the_check,
            &mut $a_representation_item_name,
        )?;
        let mut $a_tessellated_face_coordinates: Handle<StepVisualCoordinatesList> =
            Default::default();
        $the_data.read_entity(
            $the_num,
            2,
            "tessellated_face.coordinates",
            $the_check,
            StepVisualCoordinatesList::static_type(),
            &mut $a_tessellated_face_coordinates,
        )?;
        let mut $a_tessellated_face_pnmax: i32 = Default::default();
        $the_data.read_integer(
            $the_num,
            3,
            "tessellated_face.pnmax",
            $the_check,
            &mut $a_tessellated_face_pnmax,
        )?;
        let mut $a_tessellated_face_normals: Handle<HArray2<f64>> = Default::default();
        let mut $sub4: i32 = 0;
        if $the_data.read_sub_list(
            $the_num,
            4,
            "tessellated_face.normals",
            $the_check,
            &mut $sub4,
            false,
            0,
            0,
        )? {
            let $nb0: i32 = $the_data.nb_params($sub4)?;
            let $nbj0: i32 = $the_data.nb_params($the_data.param_number($sub4, 1)?)?;
            $a_tessellated_face_normals = step_new_array2::<f64>(1, $nb0, 1, $nbj0)?;
            for $i0 in 1..=$nb0 {
                let mut $subj4: i32 = 0;
                if $the_data.read_sub_list(
                    $sub4,
                    $i0,
                    "sub-part(tessellated_face.normals)",
                    $the_check,
                    &mut $subj4,
                    false,
                    0,
                    0,
                )? {
                    let $num4: i32 = $subj4;
                    for $j0 in 1..=$nbj0 {
                        let mut $an_it0: f64 = Default::default();
                        $the_data.read_real($num4, $j0, "real", $the_check, &mut $an_it0)?;
                        step_array2_set(
                            &$a_tessellated_face_normals,
                            $i0,
                            $j0,
                            $an_it0,
                            "ReadStep",
                        )?;
                    }
                }
            }
        }
        let mut $a_tessellated_face_geometric_link: StepVisualFaceOrSurface = Default::default();
        let mut $has_tessellated_face_geometric_link: bool = true;
        if $the_data.is_param_defined($the_num, 5)? {
            $the_data.read_entity_select(
                $the_num,
                5,
                "tessellated_face.geometric_link",
                $the_check,
                &mut $a_tessellated_face_geometric_link,
            )?;
        } else {
            $has_tessellated_face_geometric_link = false;
            $a_tessellated_face_geometric_link = StepVisualFaceOrSurface::default();
        }
    };
}
pub(crate) use rw_block_b_0026;

macro_rules! rw_block_b_0027 {
    ($a_geometric_tolerance_description:ident, $a_geometric_tolerance_magnitude:ident, $a_geometric_tolerance_name:ident, $a_geometric_tolerance_toleranced_shape_aspect:ident, $a_geometric_tolerance_with_datum_reference_datum_system:ident, $ach:ident, $an_it0:ident, $data:ident, $ent:ident, $i0:ident, $nb0:ident, $num:ident, $num2:ident, $sub5:ident) => {
        rw_block_b_0060!(
            $a_geometric_tolerance_description,
            $a_geometric_tolerance_magnitude,
            $a_geometric_tolerance_name,
            $a_geometric_tolerance_toleranced_shape_aspect,
            $ach,
            $data,
            $num
        );
        let mut $a_geometric_tolerance_with_datum_reference_datum_system: Handle<
            HArray1<StepDimTolDatumSystemOrReference>,
        > = Default::default();
        let mut $sub5: i32 = 0;
        if $data.read_sub_list(
            $num,
            5,
            "geometric_tolerance_with_datum_reference.datum_system",
            $ach,
            &mut $sub5,
            false,
            0,
            0,
        )? {
            let $nb0: i32 = $data.nb_params($sub5)?;
            $a_geometric_tolerance_with_datum_reference_datum_system =
                step_new_array1::<StepDimTolDatumSystemOrReference>(1, $nb0)?;
            let $num2: i32 = $sub5;
            for $i0 in 1..=$nb0 {
                let mut $an_it0: StepDimTolDatumSystemOrReference = Default::default();
                $data.read_entity_select(
                    $num2,
                    $i0,
                    "datum_system_or_reference",
                    $ach,
                    &mut $an_it0,
                )?;
                step_array1_set(
                    &$a_geometric_tolerance_with_datum_reference_datum_system,
                    $i0,
                    $an_it0.clone(),
                    "ReadStep",
                )?;
            }
        }
        $ent.init_geometric_tolerance_target(
            &$a_geometric_tolerance_name,
            &$a_geometric_tolerance_description,
            &$a_geometric_tolerance_magnitude,
            &$a_geometric_tolerance_toleranced_shape_aspect,
            &$a_geometric_tolerance_with_datum_reference_datum_system,
        )?;
    };
}
pub(crate) use rw_block_b_0027;

macro_rules! rw_block_b_0028 {
    ($ent:ident, $i1:ident, $i3:ident, $sw:ident, $var0:ident) => {
        rw_block_b_0041!($ent, $i1, $i3, $sw, $var0);
        $sw.send_entity(&$ent.property())?;
        $sw.send_entity(&$ent.material())?;
    };
}
pub(crate) use rw_block_b_0028;

macro_rules! rw_block_b_0029 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0084!($the_ent, $the_sw);
        $the_sw.send_boolean($the_ent.tx())?;
        $the_sw.send_boolean($the_ent.ty())?;
        $the_sw.send_boolean($the_ent.tz())?;
        $the_sw.send_boolean($the_ent.rx())?;
        $the_sw.send_boolean($the_ent.ry())?;
        $the_sw.send_boolean($the_ent.rz())?;
    };
}
pub(crate) use rw_block_b_0029;

macro_rules! rw_block_b_0030 {
    ($a_type:ident, $ent:ident, $i4:ident, $sw:ident, $var0:ident) => {
        rw_block_b_0202!($a_type, $ent, $sw);
        rw_block_b_0050!($ent, $i4, $sw, $var0);
    };
}
pub(crate) use rw_block_b_0030;

macro_rules! rw_block_b_0031 {
    ($i3:ident, $i5:ident, $j3:ident, $the_ent:ident, $the_sw:ident, $var0:ident) => {
        rw_block_b_0056!($i3, $j3, $the_ent, $the_sw, $var0);
        for $i5 in 1..=step_live(&$the_ent.pnindex(), "WriteStep")?
            .array1()
            .length()
        {
            let $var0: i32 = step_array1_item(&$the_ent.pnindex(), $i5, "WriteStep")?;
            $the_sw.send_integer($var0)?;
        }
        $the_sw.close_sub()?;
        $the_sw.open_sub()?;
    };
}
pub(crate) use rw_block_b_0031;

macro_rules! rw_block_b_0032 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0084!($the_ent, $the_sw);
        $the_sw.send_real($the_ent.radius_first_link())?;
        $the_sw.send_real($the_ent.radius_second_link())?;
        $the_sw.send_real($the_ent.bevel())?;
        $the_sw.send_real($the_ent.helical_angle())?;
        $the_sw.send_real($the_ent.gear_ratio())?;
    };
}
pub(crate) use rw_block_b_0032;

macro_rules! rw_block_b_0033 {
    ($a_description:ident, $a_magnitude:ident, $a_name:ident, $a_toleranced_shape_aspect:ident, $ach:ident, $data:ident, $num:ident, $num0:ident) => {
        let mut $num: i32 = 0;
        $data.named_for_complex_short("GEOMETRIC_TOLERANCE", "GMTTLR", $num0, &mut $num, $ach);
        rw_block_b_0051!(
            $a_description,
            $a_magnitude,
            $a_name,
            $a_toleranced_shape_aspect,
            $ach,
            $data,
            $num
        );
    };
}
pub(crate) use rw_block_b_0033;
