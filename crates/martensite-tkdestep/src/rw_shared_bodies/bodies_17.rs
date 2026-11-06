// The RW member bodies several classes repeat.

macro_rules! rw_block_0183 {
    ($a_depth:ident, $a_extruded_direction:ident, $ach:ident, $data:ident, $num:ident) => {
        $data.read_entity(
            $num,
            3,
            "extruded_direction",
            $ach,
            StepGeomDirection::static_type(),
            &mut $a_extruded_direction,
        )?;
        let mut $a_depth: f64 = Default::default();
        $data.read_real($num, 4, "depth", $ach, &mut $a_depth)?;
    };
}
pub(crate) use rw_block_0183;

macro_rules! rw_block_0184 {
    ($a_angle:ident, $a_axis:ident, $ach:ident, $data:ident, $num:ident) => {
        $data.read_entity(
            $num,
            3,
            "axis",
            $ach,
            StepGeomAxis1Placement::static_type(),
            &mut $a_axis,
        )?;
        let mut $a_angle: f64 = Default::default();
        $data.read_real($num, 4, "angle", $ach, &mut $a_angle)?;
    };
}
pub(crate) use rw_block_0184;

macro_rules! rw_block_0185 {
    ($a_representation_items:ident, $a_representation_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_representation_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "representation.name",
            $ach,
            &mut $a_representation_name,
        )?;
        let mut $a_representation_items: Handle<HArray1<Handle<StepReprRepresentationItem>>> =
            Default::default();
    };
}
pub(crate) use rw_block_0185;

macro_rules! rw_block_0186 {
    ($a_representation_context_of_items:ident, $a_representation_items:ident, $a_representation_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_representation_context_of_items: Handle<StepReprRepresentationContext> =
            Default::default();
        $data.read_entity(
            $num,
            3,
            "representation.context_of_items",
            $ach,
            StepReprRepresentationContext::static_type(),
            &mut $a_representation_context_of_items,
        )?;
        $ent.init(
            &$a_representation_name,
            &$a_representation_items,
            &$a_representation_context_of_items,
        );
    };
}
pub(crate) use rw_block_0186;

macro_rules! rw_block_0187 {
    ($ent:ident, $is2:ident, $iter:ident, $nb_elem2:ident) => {
        $iter.get_one_item(&$ent.curve3d().cast());
        let $nb_elem2: i32 = $ent.nb_associated_geometry()?;
        for $is2 in 1..=$nb_elem2 {
            $iter.get_one_item(&$ent.associated_geometry_value($is2)?.value());
        }
    };
}
pub(crate) use rw_block_0187;

macro_rules! rw_block_0188 {
    ($ent:ident, $is1:ident, $iter:ident, $nb_elem1:ident) => {
        let $nb_elem1: i32 = $ent.nb_bounds()?;
        for $is1 in 1..=$nb_elem1 {
            $iter.get_one_item(&$ent.bounds_value($is1)?.cast());
        }
        $iter.get_one_item(&$ent.face_geometry().cast());
    };
}
pub(crate) use rw_block_0188;

macro_rules! rw_block_0189 {
    ($ent:ident, $i:ident, $iter:ident, $nb_elem:ident) => {
        let $nb_elem: i32 = $ent.nb_items();
        for $i in 1..=$nb_elem {
            $iter.get_one_item(&$ent.items_value($i)?.cast());
        }
        $iter.get_one_item(&$ent.context_of_items().cast());
    };
}
pub(crate) use rw_block_0189;

macro_rules! rw_block_0190 {
    ($ent:ident, $is1:ident, $iter:ident, $nb_elem1:ident) => {
        let $nb_elem1: i32 = $ent.nb_items();
        for $is1 in 1..=$nb_elem1 {
            $iter.get_one_item(&$ent.items_value($is1)?.cast());
        }
        $iter.get_one_item(&$ent.context_of_items().cast());
    };
}
pub(crate) use rw_block_0190;

macro_rules! rw_block_0191 {
    ($ent:ident, $has_adescription:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        let $has_adescription: bool = $ent.has_description();
        if $has_adescription {
            $sw.send_entity(&$ent.description())?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_0191;

macro_rules! rw_block_0192 {
    ($a_uncertainty:ident, $anent3:ident, $nsub3:ident) => {
        let mut $a_uncertainty: Handle<HArray1<Handle<StepBasicUncertaintyMeasureWithUnit>>> =
            Default::default();
        let mut $anent3: Handle<StepBasicUncertaintyMeasureWithUnit> = Default::default();
        let mut $nsub3: i32 = Default::default();
    };
}
pub(crate) use rw_block_0192;

macro_rules! rw_block_0193 {
    ($a_descr:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_descr: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use rw_block_0193;

macro_rules! rw_block_0194 {
    ($a_description:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 2, "name", $ach, &mut $a_name)?;
        let mut $a_description: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use rw_block_0194;

macro_rules! rw_block_0195 {
    ($a_description:ident, $a_name:ident, $the_ach:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string($the_num, 1, "name", $the_ach, &mut $a_name)?;
        let mut $a_description: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use rw_block_0195;

macro_rules! rw_block_0196 {
    ($a_property_definition_description:ident, $a_property_definition_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_property_definition_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "property_definition.name",
            $ach,
            &mut $a_property_definition_name,
        )?;
        let mut $a_property_definition_description: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use rw_block_0196;

macro_rules! rw_block_0197 {
    ($ent:ident, $i:ident, $iter:ident, $nbq:ident) => {
        let $i: i32 = Default::default();
        let $nbq: i32 = $ent.nb_qualifiers()?;
        for $i in 1..=$nbq {
            $iter.add_item(&$ent.qualifiers_value($i)?.value());
        }
    };
}
pub(crate) use rw_block_0197;

macro_rules! rw_block_0198 {
    ($a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        $data.check_derived($num, 2, "cfs_faces", $ach, false)?;
    };
}
pub(crate) use rw_block_0198;

macro_rules! rw_block_0199 {
    ($a_description:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_description: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use rw_block_0199;
