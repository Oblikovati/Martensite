// The RW member bodies several classes repeat.

macro_rules! rw_block_0000 {
    ($a_country:ident, $a_electronic_mail_address:ident, $a_facsimile_number:ident, $a_internal_location:ident, $a_postal_box:ident, $a_postal_code:ident, $a_region:ident, $a_street:ident, $a_street_number:ident, $a_telephone_number:ident, $a_telex_number:ident, $a_town:ident, $ach:ident, $data:ident, $has_acountry:ident, $has_aelectronic_mail_address:ident, $has_afacsimile_number:ident, $has_ainternal_location:ident, $has_apostal_box:ident, $has_apostal_code:ident, $has_aregion:ident, $has_astreet:ident, $has_astreet_number:ident, $has_atelephone_number:ident, $has_atelex_number:ident, $has_atown:ident, $num:ident) => {
        let mut $a_internal_location: Handle<HAsciiString> = Default::default();
        let mut $has_ainternal_location: bool = true;
        if $data.is_param_defined($num, 1)? {
            $data.read_string(
                $num,
                1,
                "internal_location",
                $ach,
                &mut $a_internal_location,
            )?;
        } else {
            $has_ainternal_location = false;
            $a_internal_location = Default::default();
        }
        let mut $a_street_number: Handle<HAsciiString> = Default::default();
        let mut $has_astreet_number: bool = true;
        if $data.is_param_defined($num, 2)? {
            $data.read_string($num, 2, "street_number", $ach, &mut $a_street_number)?;
        } else {
            $has_astreet_number = false;
            $a_street_number = Default::default();
        }
        let mut $a_street: Handle<HAsciiString> = Default::default();
        let mut $has_astreet: bool = true;
        if $data.is_param_defined($num, 3)? {
            $data.read_string($num, 3, "street", $ach, &mut $a_street)?;
        } else {
            $has_astreet = false;
            $a_street = Default::default();
        }
        let mut $a_postal_box: Handle<HAsciiString> = Default::default();
        let mut $has_apostal_box: bool = true;
        if $data.is_param_defined($num, 4)? {
            $data.read_string($num, 4, "postal_box", $ach, &mut $a_postal_box)?;
        } else {
            $has_apostal_box = false;
            $a_postal_box = Default::default();
        }
        let mut $a_town: Handle<HAsciiString> = Default::default();
        let mut $has_atown: bool = true;
        if $data.is_param_defined($num, 5)? {
            $data.read_string($num, 5, "town", $ach, &mut $a_town)?;
        } else {
            $has_atown = false;
            $a_town = Default::default();
        }
        let mut $a_region: Handle<HAsciiString> = Default::default();
        let mut $has_aregion: bool = true;
        if $data.is_param_defined($num, 6)? {
            $data.read_string($num, 6, "region", $ach, &mut $a_region)?;
        } else {
            $has_aregion = false;
            $a_region = Default::default();
        }
        let mut $a_postal_code: Handle<HAsciiString> = Default::default();
        let mut $has_apostal_code: bool = true;
        if $data.is_param_defined($num, 7)? {
            $data.read_string($num, 7, "postal_code", $ach, &mut $a_postal_code)?;
        } else {
            $has_apostal_code = false;
            $a_postal_code = Default::default();
        }
        let mut $a_country: Handle<HAsciiString> = Default::default();
        let mut $has_acountry: bool = true;
        if $data.is_param_defined($num, 8)? {
            $data.read_string($num, 8, "country", $ach, &mut $a_country)?;
        } else {
            $has_acountry = false;
            $a_country = Default::default();
        }
        let mut $a_facsimile_number: Handle<HAsciiString> = Default::default();
        let mut $has_afacsimile_number: bool = true;
        if $data.is_param_defined($num, 9)? {
            $data.read_string($num, 9, "facsimile_number", $ach, &mut $a_facsimile_number)?;
        } else {
            $has_afacsimile_number = false;
            $a_facsimile_number = Default::default();
        }
        let mut $a_telephone_number: Handle<HAsciiString> = Default::default();
        let mut $has_atelephone_number: bool = true;
        if $data.is_param_defined($num, 10)? {
            $data.read_string($num, 10, "telephone_number", $ach, &mut $a_telephone_number)?;
        } else {
            $has_atelephone_number = false;
            $a_telephone_number = Default::default();
        }
        let mut $a_electronic_mail_address: Handle<HAsciiString> = Default::default();
        let mut $has_aelectronic_mail_address: bool = true;
        if $data.is_param_defined($num, 11)? {
            $data.read_string(
                $num,
                11,
                "electronic_mail_address",
                $ach,
                &mut $a_electronic_mail_address,
            )?;
        } else {
            $has_aelectronic_mail_address = false;
            $a_electronic_mail_address = Default::default();
        }
        let mut $a_telex_number: Handle<HAsciiString> = Default::default();
        let mut $has_atelex_number: bool = true;
        if $data.is_param_defined($num, 12)? {
            $data.read_string($num, 12, "telex_number", $ach, &mut $a_telex_number)?;
        } else {
            $has_atelex_number = false;
            $a_telex_number = Default::default();
        }
    };
}
pub(crate) use rw_block_0000;

macro_rules! rw_block_0001 {
    ($a_closed_curve:ident, $a_control_points_list:ident, $a_curve_form:ident, $a_degree:ident, $a_name:ident, $a_self_intersect:ident, $a_weights_data:ident, $a_weights_data_item:ident, $ach:ident, $anent2:ident, $data:ident, $i2:ident, $i6:ident, $nb2:ident, $nb6:ident, $nsub2:ident, $nsub6:ident, $num:ident, $text:ident) => {
        rw_block_0007!(
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
        rw_block_0041!(
            $a_name,
            $a_weights_data,
            $a_weights_data_item,
            $ach,
            $data,
            $i6,
            $nb6,
            $nsub6,
            $num
        );
    };
}
pub(crate) use rw_block_0001;

macro_rules! rw_block_0002 {
    ($a_control_points_list:ident, $a_self_intersect:ident, $a_surface_form:ident, $a_u_closed:ident, $a_u_degree:ident, $a_v_closed:ident, $a_v_degree:ident, $a_weights_data:ident, $a_weights_data_item:ident, $ach:ident, $anent3:ident, $data:ident, $i3:ident, $j3:ident, $nbi3:ident, $nbj3:ident, $nsi3:ident, $nsub3:ident, $nsub8:ident, $num:ident, $text:ident) => {
        rw_block_0006!(
            $a_control_points_list,
            $a_self_intersect,
            $a_surface_form,
            $a_u_closed,
            $a_u_degree,
            $a_v_closed,
            $a_v_degree,
            $ach,
            $anent3,
            $data,
            $i3,
            $j3,
            $nbi3,
            $nbj3,
            $nsi3,
            $nsub3,
            $num,
            $text
        );
        rw_block_0146!(
            $a_weights_data,
            $a_weights_data_item,
            $ach,
            $data,
            $nsub8,
            $num
        );
    };
}
pub(crate) use rw_block_0002;

macro_rules! rw_block_0003 {
    ($ent:ident, $has_acountry:ident, $has_aelectronic_mail_address:ident, $has_afacsimile_number:ident, $has_ainternal_location:ident, $has_apostal_box:ident, $has_apostal_code:ident, $has_aregion:ident, $has_astreet:ident, $has_astreet_number:ident, $has_atelephone_number:ident, $has_atelex_number:ident, $has_atown:ident, $sw:ident) => {
        rw_block_0004!(
            $ent,
            $has_acountry,
            $has_aelectronic_mail_address,
            $has_afacsimile_number,
            $has_ainternal_location,
            $has_apostal_box,
            $has_apostal_code,
            $has_aregion,
            $has_astreet,
            $has_astreet_number,
            $has_atelephone_number,
            $has_atelex_number,
            $has_atown,
            $sw
        );
        $sw.open_sub()?;
    };
}
pub(crate) use rw_block_0003;
