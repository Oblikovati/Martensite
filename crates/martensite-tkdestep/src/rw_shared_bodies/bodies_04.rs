// The RW member bodies several classes repeat.

macro_rules! rw_block_0017 {
    ($a_name:ident, $a_prefix:ident, $ach:ident, $data:ident, $ent:ident, $has_aprefix:ident, $num:ident, $num0:ident, $sorted:ident, $text:ident) => {
        if !$sorted {
            $num = 0;
        }
        $sorted &= $data.named_for_complex_short("NAMED_UNIT", "NMDUNT", $num0, &mut $num, $ach);
        if !$data.check_nb_params($num, 1, $ach, "named_unit")? {
            return Ok(());
        }
        $data.check_derived($num, 1, "dimensions", $ach, false)?;
        if !$sorted {
            $num = 0;
        }
        $data.named_for_complex_short("SI_UNIT", "SUNT", $num0, &mut $num, $ach);
        rw_block_0063!(
            $a_name,
            $a_prefix,
            $ach,
            $data,
            $ent,
            $has_aprefix,
            $num,
            $text
        );
    };
}
pub(crate) use rw_block_0017;

macro_rules! rw_block_0018 {
    ($a_axis1:ident, $a_axis2:ident, $a_local_origin:ident, $a_name:ident, $a_scale:ident, $ach:ident, $data:ident, $has_aaxis1:ident, $has_aaxis2:ident, $has_ascale:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 3, "name", $ach, &mut $a_name)?;
        let mut $a_axis1: Handle<StepGeomDirection> = Default::default();
        let mut $has_aaxis1: bool = true;
        if $data.is_param_defined($num, 4)? {
            $data.read_entity(
                $num,
                4,
                "axis1",
                $ach,
                StepGeomDirection::static_type(),
                &mut $a_axis1,
            )?;
        } else {
            $has_aaxis1 = false;
            $a_axis1 = Default::default();
        }
        let mut $a_axis2: Handle<StepGeomDirection> = Default::default();
        let mut $has_aaxis2: bool = true;
        if $data.is_param_defined($num, 5)? {
            $data.read_entity(
                $num,
                5,
                "axis2",
                $ach,
                StepGeomDirection::static_type(),
                &mut $a_axis2,
            )?;
        } else {
            $has_aaxis2 = false;
            $a_axis2 = Default::default();
        }
        let mut $a_local_origin: Handle<StepGeomCartesianPoint> = Default::default();
        $data.read_entity(
            $num,
            6,
            "local_origin",
            $ach,
            StepGeomCartesianPoint::static_type(),
            &mut $a_local_origin,
        )?;
        let mut $a_scale: f64 = Default::default();
        let mut $has_ascale: bool = true;
        if $data.is_param_defined($num, 7)? {
            $data.read_real($num, 7, "scale", $ach, &mut $a_scale)?;
        } else {
            $has_ascale = false;
            $a_scale = 0.0;
        }
    };
}
pub(crate) use rw_block_0018;

macro_rules! rw_block_0019 {
    ($a_context_identifier:ident, $a_context_type:ident, $a_units:ident, $ach:ident, $anent2:ident, $data:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        $num = $data.next_for_complex($num);
        if !$data.check_nb_params($num, 1, $ach, "global_unit_assigned_context")? {
            return Ok(());
        }
        let mut $a_units: Handle<HArray1<Handle<StepBasicNamedUnit>>> = Default::default();
        let mut $anent2: Handle<StepBasicNamedUnit> = Default::default();
        let mut $nsub2: i32 = Default::default();
        if $data.read_sub_list($num, 1, "units", $ach, &mut $nsub2, false, 0, 0)? {
            let $nb2: i32 = $data.nb_params($nsub2)?;
            $a_units = step_new_array1::<Handle<StepBasicNamedUnit>>(1, $nb2)?;
            for $i2 in 1..=$nb2 {
                if $data.read_entity(
                    $nsub2,
                    $i2,
                    "unit",
                    $ach,
                    StepBasicNamedUnit::static_type(),
                    &mut $anent2,
                )? {
                    step_array1_set(&$a_units, $i2, $anent2.clone(), "ReadStep")?;
                }
            }
        }
        rw_block_0088!($a_context_identifier, $a_context_type, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0019;

macro_rules! rw_block_0020 {
    ($a_description:ident, $a_name:ident, $a_of_shape:ident, $a_product_definitional:ident, $ach:ident, $data:ident, $ent:ident, $num:ident, $num0:ident) => {
        let mut $num: i32 = 0;
        $data.named_for_complex_short("SHAPE_ASPECT", "SHPASP", $num0, &mut $num, $ach);
        rw_block_0033!(
            $a_description,
            $a_name,
            $a_of_shape,
            $a_product_definitional,
            $ach,
            $data,
            $ent,
            $num
        );
    };
}
pub(crate) use rw_block_0020;

macro_rules! rw_block_0021 {
    ($ent:ident, $i3:ident, $j3:ident, $sw:ident) => {
        rw_block_0028!($ent, $i3, $j3, $sw);
        step_start_entity($sw, "GEOMETRIC_REPRESENTATION_ITEM")?;
    };
}
pub(crate) use rw_block_0021;

macro_rules! rw_block_0022 {
    ($a_dimensions:ident, $a_name:ident, $a_prefix:ident, $ach:ident, $data:ident, $has_aprefix:ident, $num:ident, $num0:ident, $text:ident) => {
        rw_block_0123!($a_dimensions, $ach, $data, $num, $num0);
        $data.named_for_complex_short("SI_UNIT", "SUNT", $num0, &mut $num, $ach);
        rw_block_0096!($a_name, $a_prefix, $ach, $data, $has_aprefix, $num, $text);
        if $data.param_type($num, 2)? == InterfaceParamType::ParamEnum {
            let $text: Vec<u8> = $data.param_c_value($num, 2)?;
            if !RwStepBasicRwSiUnitName::convert_to_enum(&$text, &mut $a_name) {
                $ach.add_fail_text("Enumeration si_unit_name has not an allowed value", "");
                return Ok(());
            }
        } else {
            $ach.add_fail_text("Parameter #3 (name) is not an enumeration", "");
            return Ok(());
        }
    };
}
pub(crate) use rw_block_0022;

macro_rules! rw_block_0023 {
    ($a_associated_geometry:ident, $a_associated_geometry_item:ident, $a_curve3d:ident, $a_master_representation:ident, $a_name:ident, $ach:ident, $data:ident, $ent:ident, $i3:ident, $nb3:ident, $nsub3:ident, $num:ident, $text:ident) => {
        rw_block_0084!(
            $a_associated_geometry,
            $a_associated_geometry_item,
            $a_curve3d,
            $a_name,
            $ach,
            $data,
            $num
        );
        let mut $nsub3: i32 = Default::default();
        if $data.read_sub_list(
            $num,
            3,
            "associated_geometry",
            $ach,
            &mut $nsub3,
            false,
            0,
            0,
        )? {
            let $nb3: i32 = $data.nb_params($nsub3)?;
            $a_associated_geometry = step_new_array1::<StepGeomPcurveOrSurface>(1, $nb3)?;
            for $i3 in 1..=$nb3 {
                if $data.read_entity_select(
                    $nsub3,
                    $i3,
                    "associated_geometry",
                    $ach,
                    &mut $a_associated_geometry_item,
                )? {
                    step_array1_set(
                        &$a_associated_geometry,
                        $i3,
                        $a_associated_geometry_item.clone(),
                        "ReadStep",
                    )?;
                }
            }
        }
        rw_block_0170!(
            $a_associated_geometry,
            $a_curve3d,
            $a_master_representation,
            $a_name,
            $ach,
            $data,
            $ent,
            $num,
            $text
        );
    };
}
pub(crate) use rw_block_0023;

macro_rules! rw_block_0024 {
    ($a_name:ident, $a_prefix:ident, $ach:ident, $data:ident, $has_aprefix:ident, $num:ident, $num0:ident, $text:ident) => {
        rw_block_0206!($ach, $data, $num, $num0);
        rw_block_0064!($a_name, $a_prefix, $ach, $data, $has_aprefix, $num, $text);
        $num = $data.next_for_complex($num);
    };
}
pub(crate) use rw_block_0024;

macro_rules! rw_block_0025 {
($a_product_definition_relationship_description:ident, $a_product_definition_relationship_id:ident, $a_product_definition_relationship_name:ident, $a_product_definition_relationship_related_product_definition:ident, $a_product_definition_relationship_relating_product_definition:ident, $ach:ident, $data:ident, $has_product_definition_relationship_description:ident, $num:ident) => {
rw_block_0046!($a_product_definition_relationship_description, $a_product_definition_relationship_id, $a_product_definition_relationship_name, $a_product_definition_relationship_relating_product_definition, $ach, $data, $has_product_definition_relationship_description, $num);
let mut $a_product_definition_relationship_related_product_definition: StepBasicProductDefinitionOrReference = Default::default();
$data.read_entity_select($num, 5, "product_definition_relationship.related_product_definition", $ach, &mut $a_product_definition_relationship_related_product_definition)?;
};
}
pub(crate) use rw_block_0025;

macro_rules! rw_block_0026 {
    ($a_conversion_factor:ident, $a_dimensions:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident, $num0:ident) => {
        rw_block_0034!(
            $a_conversion_factor,
            $a_dimensions,
            $a_name,
            $ach,
            $data,
            $num,
            $num0
        );
        $num = $data.next_for_complex($num);
    };
}
pub(crate) use rw_block_0026;
