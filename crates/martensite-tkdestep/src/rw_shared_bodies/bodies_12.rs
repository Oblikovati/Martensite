// The RW member bodies several classes repeat.

macro_rules! rw_block_0104 {
    ($a_context_identifier:ident, $a_context_type:ident, $ach:ident, $data:ident, $num:ident) => {
        if !$data.check_nb_params($num, 2, $ach, "representation_context")? {
            return Ok(());
        }
        rw_block_0150!($a_context_identifier, $a_context_type, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0104;

macro_rules! rw_block_0105 {
    ($a_axis:ident, $a_location:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0130!($a_location, $a_name, $ach, $data, $num);
        let mut $a_axis: Handle<StepGeomDirection> = Default::default();
    };
}
pub(crate) use rw_block_0105;

macro_rules! rw_block_0106 {
    ($a_conversion_factor:ident, $a_dimensions:ident, $a_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        $num = $data.next_for_complex($num);
        rw_block_0172!($a_dimensions, $ach, $data, $num);
        $ent.init(&$a_dimensions, &$a_name, &$a_conversion_factor);
    };
}
pub(crate) use rw_block_0106;

macro_rules! rw_block_0107 {
    ($a_name:ident, $a_position:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        rw_block_0140!($a_name, $a_position, $ach, $data, $num);
        $ent.init(&$a_name, &$a_position);
    };
}
pub(crate) use rw_block_0107;

macro_rules! rw_block_0108 {
    ($a_items:ident, $a_name:ident, $ach:ident, $anent2:ident, $data:ident, $nsub2:ident, $num:ident) => {
        rw_block_0179!($a_items, $a_name, $ach, $data, $num);
        let mut $anent2: Handle<StepReprRepresentationItem> = Default::default();
        let mut $nsub2: i32 = Default::default();
    };
}
pub(crate) use rw_block_0108;

macro_rules! rw_block_0109 {
    ($a_bounds:ident, $a_name:ident, $ach:ident, $anent2:ident, $data:ident, $nsub2:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_bounds: Handle<HArray1<Handle<StepShapeFaceBound>>> = Default::default();
        let mut $anent2: Handle<StepShapeFaceBound> = Default::default();
        let mut $nsub2: i32 = Default::default();
    };
}
pub(crate) use rw_block_0109;

macro_rules! rw_block_0110 {
    ($a_description:ident, $a_id:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_id: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "id", $ach, &mut $a_id)?;
        rw_block_0194!($a_description, $a_name, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0110;

macro_rules! rw_block_0111 {
    ($a_unit_component:ident, $ach:ident, $data:ident, $ent:ident, $mvc:ident, $num:ident) => {
        rw_block_0143!($a_unit_component, $ach, $data, $mvc, $num);
        $ent.init(&$mvc, &$a_unit_component);
    };
}
pub(crate) use rw_block_0111;

macro_rules! rw_block_0112 {
    ($ent:ident, $sw:ident) => {
        step_start_entity($sw, "LENGTH_MEASURE_WITH_UNIT")?;
        rw_block_0151!($ent, $sw);
    };
}
pub(crate) use rw_block_0112;

macro_rules! rw_block_0113 {
    ($ent:ident, $sw:ident) => {
        rw_block_0151!($ent, $sw);
        step_start_entity($sw, "PLANE_ANGLE_MEASURE_WITH_UNIT")?;
    };
}
pub(crate) use rw_block_0113;

macro_rules! rw_block_0114 {
    ($a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0152!($a_name, $ach, $data, $num);
        $num = $data.next_for_complex($num);
    };
}
pub(crate) use rw_block_0114;

macro_rules! rw_block_0115 {
    ($a_coordinate_space_dimension:ident, $ach:ident, $data:ident, $num:ident, $num0:ident) => {
        rw_block_0156!($a_coordinate_space_dimension, $ach, $data, $num, $num0);
        $num = $data.next_for_complex($num);
    };
}
pub(crate) use rw_block_0115;

macro_rules! rw_block_0116 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.open_sub()?;
        for $i2 in 1..=$ent.nb_items() {
            $sw.send_entity(&$ent.items_value($i2)?)?;
        }
        $sw.close_sub()?;
        $sw.send_entity(&$ent.context_of_items())?;
    };
}
pub(crate) use rw_block_0116;

macro_rules! rw_block_0117 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.open_sub()?;
        for $i2 in 1..=$ent.nb_segments() {
            $sw.send_entity(&$ent.segments_value($i2)?)?;
        }
        $sw.close_sub()?;
        $sw.send_logical($ent.self_intersect())?;
    };
}
pub(crate) use rw_block_0117;

macro_rules! rw_block_0118 {
    ($ent:ident, $i:ident, $nbq:ident, $sw:ident) => {
        let $i: i32 = Default::default();
        let $nbq: i32 = $ent.nb_qualifiers()?;
        $sw.open_sub()?;
        for $i in 1..=$nbq {
            $sw.send_entity(&$ent.qualifiers_value($i)?.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_0118;

macro_rules! rw_block_0119 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.position())?;
        $sw.send_real($ent.x())?;
        $sw.send_real($ent.y())?;
        $sw.send_real($ent.z())?;
    };
}
pub(crate) use rw_block_0119;

macro_rules! rw_block_0120 {
    ($ent:ident, $sw:ident) => {
        $sw.close_sub()?;
        $sw.send_enum(RwStepGeomRwBSplineSurfaceForm::convert_to_string(
            $ent.surface_form(),
        ))?;
        $sw.send_logical($ent.u_closed())?;
        $sw.send_logical($ent.v_closed())?;
        $sw.send_logical($ent.self_intersect())?;
    };
}
pub(crate) use rw_block_0120;
