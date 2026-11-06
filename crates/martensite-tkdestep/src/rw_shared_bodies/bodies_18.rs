// The RW member bodies several classes repeat.

macro_rules! rw_block_0200 {
    ($a_description:ident, $a_id:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_id: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "id", $ach, &mut $a_id)?;
        let mut $a_description: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use rw_block_0200;

macro_rules! rw_block_0201 {
    ($a_dimensions:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_dimensions: Handle<StepBasicDimensionalExponents> = Default::default();
        $data.read_entity(
            $num,
            1,
            "dimensions",
            $ach,
            StepBasicDimensionalExponents::static_type(),
            &mut $a_dimensions,
        )?;
        $ent.init(&$a_dimensions);
    };
}
pub(crate) use rw_block_0201;

macro_rules! rw_block_0202 {
    ($a_named_unit_dimensions:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_named_unit_dimensions: Handle<StepBasicDimensionalExponents> =
            Default::default();
        $data.read_entity(
            $num,
            1,
            "named_unit.dimensions",
            $ach,
            StepBasicDimensionalExponents::static_type(),
            &mut $a_named_unit_dimensions,
        )?;
        $ent.init(&$a_named_unit_dimensions);
    };
}
pub(crate) use rw_block_0202;

macro_rules! rw_block_0203 {
    ($a_distance:ident, $a_self_intersect:ident, $ach:ident, $data:ident, $num:ident) => {
        $data.read_real($num, 3, "distance", $ach, &mut $a_distance)?;
        let mut $a_self_intersect: StepDataLogical = Default::default();
        $data.read_logical($num, 4, "self_intersect", $ach, &mut $a_self_intersect)?;
    };
}
pub(crate) use rw_block_0203;

macro_rules! rw_block_0204 {
    ($a_major_radius:ident, $a_minor_radius:ident, $ach:ident, $data:ident, $num:ident) => {
        $data.read_real($num, 3, "major_radius", $ach, &mut $a_major_radius)?;
        let mut $a_minor_radius: f64 = Default::default();
        $data.read_real($num, 4, "minor_radius", $ach, &mut $a_minor_radius)?;
    };
}
pub(crate) use rw_block_0204;

macro_rules! rw_block_0205 {
    ($ent:ident, $iter:ident) => {
        $iter.get_one_item(&$ent.rep1().cast());
        $iter.get_one_item(&$ent.rep2().cast());
        $iter.get_one_item(&$ent.transformation_operator().value());
    };
}
pub(crate) use rw_block_0205;

macro_rules! rw_block_0206 {
    ($ach:ident, $data:ident, $num:ident, $num0:ident) => {
        let mut $num: i32 = $num0;
        if !$data.check_nb_params($num, 1, $ach, "named_unit")? {
            return Ok(());
        }
        $data.check_derived($num, 1, "dimensions", $ach, false)?;
    };
}
pub(crate) use rw_block_0206;

macro_rules! rw_block_0207 {
    ($ent:ident, $i1:ident, $iter:ident, $var0:ident) => {
        for $i1 in 1..=$ent.parent.parent.nb_items() {
            let $var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&$ent.parent.parent.items(), $i1, "Share")?;
            $iter.add_item(&$var0.cast());
        }
        $iter.add_item(&$ent.parent.parent.context_of_items().cast());
    };
}
pub(crate) use rw_block_0207;

macro_rules! rw_block_0208 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.parent.parent.name())?;
        if $ent.parent.parent.has_description() {
            $sw.send_entity(&$ent.parent.parent.description())?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_0208;

macro_rules! rw_block_0209 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&step_live(&$ent.measure(), "WriteStep")?.value_component_member())?;
        $sw.send_entity(
            &step_live(&$ent.measure(), "WriteStep")?
                .unit_component()
                .value(),
        )?;
    };
}
pub(crate) use rw_block_0209;

macro_rules! rw_block_0210 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.parent.name())?;
        if $ent.parent.has_description() {
            $sw.send_entity(&$ent.parent.description())?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_0210;

macro_rules! rw_block_0211 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        if !$ent.description().is_null() {
            $sw.send_entity(&$ent.description())?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_0211;

macro_rules! rw_block_0212 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.id())?;
        if !$ent.description().is_null() {
            $sw.send_entity(&$ent.description())?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_0212;

macro_rules! rw_block_0213 {
    ($a_description:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_description: Handle<HAsciiString> = Default::default();
        if $data.is_param_defined($num, 14)? {
            $data.read_string($num, 14, "description", $ach, &mut $a_description)?;
        }
    };
}
pub(crate) use rw_block_0213;

macro_rules! rw_block_0214 {
    ($a_name:ident, $ach:ident, $data:ident, $num:ident, $text:ident) => {
        if $data.param_type($num, 2)? == InterfaceParamType::ParamEnum {
            let $text: Vec<u8> = $data.param_c_value($num, 2)?;
            if !RwStepBasicRwSiUnitName::convert_to_enum(&$text, &mut $a_name) {
                $ach.add_fail_text("Enumeration si_unit_name has not an allowed value", "");
                return Ok(());
            }
        } else {
            $ach.add_fail_text("Parameter #2 (name) is not an enumeration", "");
            return Ok(());
        }
    };
}
pub(crate) use rw_block_0214;
