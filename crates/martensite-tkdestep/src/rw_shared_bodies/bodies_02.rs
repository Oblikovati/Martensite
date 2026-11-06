// The RW member bodies several classes repeat.

macro_rules! rw_block_0004 {
    ($ent:ident, $has_acountry:ident, $has_aelectronic_mail_address:ident, $has_afacsimile_number:ident, $has_ainternal_location:ident, $has_apostal_box:ident, $has_apostal_code:ident, $has_aregion:ident, $has_astreet:ident, $has_astreet_number:ident, $has_atelephone_number:ident, $has_atelex_number:ident, $has_atown:ident, $sw:ident) => {
        let $has_ainternal_location: bool = $ent.has_internal_location();
        if $has_ainternal_location {
            $sw.send_entity(&$ent.internal_location())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_astreet_number: bool = $ent.has_street_number();
        if $has_astreet_number {
            $sw.send_entity(&$ent.street_number())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_astreet: bool = $ent.has_street();
        if $has_astreet {
            $sw.send_entity(&$ent.street())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_apostal_box: bool = $ent.has_postal_box();
        if $has_apostal_box {
            $sw.send_entity(&$ent.postal_box())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_atown: bool = $ent.has_town();
        if $has_atown {
            $sw.send_entity(&$ent.town())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_aregion: bool = $ent.has_region();
        if $has_aregion {
            $sw.send_entity(&$ent.region())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_apostal_code: bool = $ent.has_postal_code();
        if $has_apostal_code {
            $sw.send_entity(&$ent.postal_code())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_acountry: bool = $ent.has_country();
        if $has_acountry {
            $sw.send_entity(&$ent.country())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_afacsimile_number: bool = $ent.has_facsimile_number();
        if $has_afacsimile_number {
            $sw.send_entity(&$ent.facsimile_number())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_atelephone_number: bool = $ent.has_telephone_number();
        if $has_atelephone_number {
            $sw.send_entity(&$ent.telephone_number())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_aelectronic_mail_address: bool = $ent.has_electronic_mail_address();
        if $has_aelectronic_mail_address {
            $sw.send_entity(&$ent.electronic_mail_address())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_atelex_number: bool = $ent.has_telex_number();
        if $has_atelex_number {
            $sw.send_entity(&$ent.telex_number())?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_0004;

macro_rules! rw_block_0005 {
    ($a_measure_with_unit:ident, $a_name:ident, $a_qri:ident, $a_repr_item:ident, $a_unit_component:ident, $a_vq:ident, $ach:ident, $data:ident, $ent:ident, $i:ident, $mvc:ident, $nb:ident, $nsub:ident, $num:ident, $num0:ident, $quals:ident) => {
        rw_block_0047!(
            $a_measure_with_unit,
            $a_unit_component,
            $ach,
            $data,
            $mvc,
            $num,
            $num0
        );
        rw_block_0175!($ach, $data, $num, $num0, $quals);
        let mut $nsub: i32 = Default::default();
        if $data.read_sub_list($num, 1, "qualifiers", $ach, &mut $nsub, false, 0, 0)? {
            let $nb: i32 = $data.nb_params($nsub)?;
            $quals = step_new_array1::<StepShapeValueQualifier>(1, $nb)?;
            for $i in 1..=$nb {
                let mut $a_vq: StepShapeValueQualifier = Default::default();
                if $data.read_entity_select($nsub, $i, "qualifier", $ach, &mut $a_vq)? {
                    step_array1_set(&$quals, $i, $a_vq.clone(), "ReadStep")?;
                }
            }
        }
        let $a_qri: Handle<StepShapeQualifiedRepresentationItem> =
            Handle::new(StepShapeQualifiedRepresentationItem::new());
        $a_qri.set_qualifiers(&$quals);
        rw_block_0087!($a_name, $a_repr_item, $ach, $data, $num, $num0);
        $ent.init(&$a_measure_with_unit, &$a_repr_item, &$a_qri)?;
    };
}
pub(crate) use rw_block_0005;

macro_rules! rw_block_0006 {
    ($a_control_points_list:ident, $a_self_intersect:ident, $a_surface_form:ident, $a_u_closed:ident, $a_u_degree:ident, $a_v_closed:ident, $a_v_degree:ident, $ach:ident, $anent3:ident, $data:ident, $i3:ident, $j3:ident, $nbi3:ident, $nbj3:ident, $nsi3:ident, $nsub3:ident, $num:ident, $text:ident) => {
        if !$data.check_nb_params($num, 0, $ach, "bounded_surface")? {
            return Ok(());
        }
        $num = $data.next_for_complex($num);
        rw_block_0058!(
            $a_control_points_list,
            $a_u_degree,
            $a_v_degree,
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
            let $nbi3: i32 = $data.nb_params($nsub3)?;
            let $nbj3: i32 = $data.nb_params($data.param_number($nsub3, 1)?)?;
            $a_control_points_list =
                step_new_array2::<Handle<StepGeomCartesianPoint>>(1, $nbi3, 1, $nbj3)?;
            for $i3 in 1..=$nbi3 {
                let mut $nsi3: i32 = Default::default();
                if $data.read_sub_list(
                    $nsub3,
                    $i3,
                    "sub-part(control_points_list)",
                    $ach,
                    &mut $nsi3,
                    false,
                    0,
                    0,
                )? {
                    for $j3 in 1..=$nbj3 {
                        if $data.read_entity(
                            $nsi3,
                            $j3,
                            "cartesian_point",
                            $ach,
                            StepGeomCartesianPoint::static_type(),
                            &mut $anent3,
                        )? {
                            step_array2_set(
                                &$a_control_points_list,
                                $i3,
                                $j3,
                                $anent3.clone(),
                                "ReadStep",
                            )?;
                        }
                    }
                }
            }
        }
        rw_block_0054!(
            $a_self_intersect,
            $a_surface_form,
            $a_u_closed,
            $a_v_closed,
            $ach,
            $data,
            $num,
            $text
        );
        $num = $data.next_for_complex($num);
        if !$data.check_nb_params($num, 0, $ach, "geometric_representation_item")? {
            return Ok(());
        }
        $num = $data.next_for_complex($num);
    };
}
pub(crate) use rw_block_0006;

macro_rules! rw_block_0007 {
    ($a_closed_curve:ident, $a_control_points_list:ident, $a_curve_form:ident, $a_degree:ident, $a_self_intersect:ident, $ach:ident, $anent2:ident, $data:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident, $text:ident) => {
        if !$data.check_nb_params($num, 0, $ach, "bounded_curve")? {
            return Ok(());
        }
        $num = $data.next_for_complex($num);
        rw_block_0016!(
            $a_closed_curve,
            $a_control_points_list,
            $a_curve_form,
            $a_degree,
            $a_self_intersect,
            $ach,
            $anent2,
            $data,
            $i2,
            $nb2,
            $nsub2,
            $num,
            $text
        );
        $num = $data.next_for_complex($num);
        if !$data.check_nb_params($num, 0, $ach, "curve")? {
            return Ok(());
        }
        $num = $data.next_for_complex($num);
        if !$data.check_nb_params($num, 0, $ach, "geometric_representation_item")? {
            return Ok(());
        }
        $num = $data.next_for_complex($num);
    };
}
pub(crate) use rw_block_0007;
