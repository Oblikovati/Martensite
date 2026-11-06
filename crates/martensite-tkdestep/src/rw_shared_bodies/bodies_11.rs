// The RW member bodies several classes repeat.

macro_rules! rw_block_0092 {
    ($ent:ident, $i3:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.curve3d())?;
        $sw.open_sub()?;
        for $i3 in 1..=$ent.nb_associated_geometry()? {
            $sw.send_entity(&$ent.associated_geometry_value($i3)?.value())?;
        }
        $sw.close_sub()?;
        $sw.send_enum(
            RwStepGeomRwPreferredSurfaceCurveRepresentation::convert_to_string(
                $ent.master_representation(),
            ),
        )?;
    };
}
pub(crate) use rw_block_0092;

macro_rules! rw_block_0093 {
    ($ent:ident, $has_aprefix:ident, $sw:ident) => {
        step_start_entity($sw, "NAMED_UNIT")?;
        $sw.send_derived()?;
        rw_block_0145!($ent, $has_aprefix, $sw);
    };
}
pub(crate) use rw_block_0093;

macro_rules! rw_block_0094 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        rw_block_0162!($ent, $i2, $sw);
        $sw.send_entity(&$ent.face_geometry())?;
        $sw.send_boolean($ent.same_sense())?;
    };
}
pub(crate) use rw_block_0094;

macro_rules! rw_block_0095 {
    ($ent:ident, $i6:ident, $sw:ident) => {
        step_start_entity($sw, "RATIONAL_B_SPLINE_CURVE")?;
        $sw.open_sub()?;
        for $i6 in 1..=$ent.nb_weights_data()? {
            $sw.send_real($ent.weights_data_value($i6)?)?;
        }
        $sw.close_sub()?;
        step_start_entity($sw, "REPRESENTATION_ITEM")?;
        $sw.send_entity(&$ent.name())?;
    };
}
pub(crate) use rw_block_0095;

macro_rules! rw_block_0096 {
    ($a_name:ident, $a_prefix:ident, $ach:ident, $data:ident, $has_aprefix:ident, $num:ident, $text:ident) => {
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
                $ach.add_fail_text("Parameter #2 (prefix) is not an enumeration", "");
                return Ok(());
            }
        }
        let mut $a_name: StepBasicSiUnitName = Default::default();
    };
}
pub(crate) use rw_block_0096;

macro_rules! rw_block_0097 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.parent.parent.id())?;
        rw_block_0208!($ent, $sw);
        $sw.send_entity(
            &$ent
                .parent
                .parent
                .relating_product_definition_ap242()
                .value(),
        )?;
        $sw.send_entity(
            &$ent
                .parent
                .parent
                .related_product_definition_ap242()
                .value(),
        )?;
    };
}
pub(crate) use rw_block_0097;

macro_rules! rw_block_0098 {
    ($a_description:ident, $a_name:ident, $ach:ident, $data:ident, $has_adescription:ident, $num:ident) => {
        rw_block_0199!($a_description, $a_name, $ach, $data, $num);
        let mut $has_adescription: bool = true;
        if $data.is_param_defined($num, 2)? {
            $data.read_string($num, 2, "description", $ach, &mut $a_description)?;
        } else {
            $has_adescription = false;
            $a_description = Default::default();
        }
    };
}
pub(crate) use rw_block_0098;

macro_rules! rw_block_0099 {
    ($ent:ident, $i1:ident, $sw:ident, $var0:ident) => {
        $sw.send_entity(&$ent.parent.parent.name())?;
        $sw.open_sub()?;
        for $i1 in 1..=$ent.parent.parent.nb_items() {
            let $var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&$ent.parent.parent.items(), $i1, "WriteStep")?;
            $sw.send_entity(&$var0)?;
        }
        $sw.close_sub()?;
        $sw.send_entity(&$ent.parent.parent.context_of_items())?;
    };
}
pub(crate) use rw_block_0099;

macro_rules! rw_block_0100 {
    ($a_conversion_factor:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        if !$data.check_nb_params($num, 2, $ach, "conversion_based_unit")? {
            return Ok(());
        }
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_conversion_factor: Handle<StandardTransient> = Default::default();
        $data.read_entity(
            $num,
            2,
            "conversion_factor",
            $ach,
            StandardTransient::static_type(),
            &mut $a_conversion_factor,
        )?;
    };
}
pub(crate) use rw_block_0100;

macro_rules! rw_block_0101 {
    ($ent:ident, $i3:ident, $sw:ident) => {
        $sw.send_entity(&$ent.curve3d())?;
        $sw.open_sub()?;
        for $i3 in 1..=$ent.nb_associated_geometry()? {
            if !$ent.associated_geometry_value($i3)?.value().is_null() {
                $sw.send_entity(&$ent.associated_geometry_value($i3)?.value())?;
            }
        }
        $sw.close_sub()?;
        $sw.send_enum(
            RwStepGeomRwPreferredSurfaceCurveRepresentation::convert_to_string(
                $ent.master_representation(),
            ),
        )?;
    };
}
pub(crate) use rw_block_0101;

macro_rules! rw_block_0102 {
    ($a_description:ident, $a_name:ident, $ach:ident, $data:ident, $has_description:ident, $num:ident) => {
        rw_block_0199!($a_description, $a_name, $ach, $data, $num);
        let mut $has_description: bool = true;
        if $data.is_param_defined($num, 2)? {
            $data.read_string($num, 2, "description", $ach, &mut $a_description)?;
        } else {
            $has_description = false;
        }
    };
}
pub(crate) use rw_block_0102;

macro_rules! rw_block_0103 {
    ($a_unit_component:ident, $ach:ident, $data:ident, $mvc:ident, $num:ident) => {
        if !$data.check_nb_params($num, 2, $ach, "measure_with_unit")? {
            return Ok(());
        }
        rw_block_0143!($a_unit_component, $ach, $data, $mvc, $num);
    };
}
pub(crate) use rw_block_0103;
