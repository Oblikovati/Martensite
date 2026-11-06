// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0136 {
($a_assigned_person_and_organization:ident, $a_role:ident, $ach:ident, $data:ident, $num:ident) => {
let mut $a_assigned_person_and_organization: Handle<StepBasicPersonAndOrganization> = Default::default();
$data.read_entity($num, 1, "assigned_person_and_organization", $ach, StepBasicPersonAndOrganization::static_type(), &mut $a_assigned_person_and_organization)?;
let mut $a_role: Handle<StepBasicPersonAndOrganizationRole> = Default::default();
$data.read_entity($num, 2, "role", $ach, StepBasicPersonAndOrganizationRole::static_type(), &mut $a_role)?;
};
}
pub(crate) use rw_block_b_0136;

macro_rules! rw_block_b_0137 {
    ($ent:ident, $i:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        rw_block_b_0180!($ent, $i, $sw);
    };
}
pub(crate) use rw_block_b_0137;

macro_rules! rw_block_b_0138 {
    ($a_coordinates:ident, $a_representation_item_name:ident, $the_check:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            1,
            "representation_item.name",
            $the_check,
            &mut $a_representation_item_name,
        )?;
        let mut $a_coordinates: Handle<StepVisualCoordinatesList> = Default::default();
        $the_data.read_entity(
            $the_num,
            2,
            "coordinates",
            $the_check,
            StepVisualCoordinatesList::static_type(),
            &mut $a_coordinates,
        )?;
    };
}
pub(crate) use rw_block_b_0138;

macro_rules! rw_block_b_0139 {
    ($a_pair_value_applies_to_pair:ident, $a_representation_item_name:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            1,
            "representation_item.name",
            $the_arch,
            &mut $a_representation_item_name,
        )?;
        let mut $a_pair_value_applies_to_pair: Handle<StepKinematicsKinematicPair> =
            Default::default();
        $the_data.read_entity(
            $the_num,
            2,
            "pair_value.applies_to_pair",
            $the_arch,
            StepKinematicsKinematicPair::static_type(),
            &mut $a_pair_value_applies_to_pair,
        )?;
    };
}
pub(crate) use rw_block_b_0139;

macro_rules! rw_block_b_0140 {
    ($ach:ident, $adoc:ident, $asource:ident, $data:ident, $num:ident) => {
        let mut $adoc: Handle<StepBasicDocument> = Default::default();
        $data.read_entity(
            $num,
            1,
            "assigned_document",
            $ach,
            StepBasicDocument::static_type(),
            &mut $adoc,
        )?;
        let mut $asource: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 2, "source", $ach, &mut $asource)?;
    };
}
pub(crate) use rw_block_b_0140;

macro_rules! rw_block_b_0141 {
    ($a_shape_aspect_of_shape:ident, $a_shape_aspect_product_definitional:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_shape_aspect_of_shape: Handle<StepReprProductDefinitionShape> =
            Default::default();
        $data.read_entity(
            $num,
            3,
            "shape_aspect.of_shape",
            $ach,
            StepReprProductDefinitionShape::static_type(),
            &mut $a_shape_aspect_of_shape,
        )?;
        let mut $a_shape_aspect_product_definitional: StepDataLogical = Default::default();
        $data.read_logical(
            $num,
            4,
            "shape_aspect.product_definitional",
            $ach,
            &mut $a_shape_aspect_product_definitional,
        )?;
    };
}
pub(crate) use rw_block_b_0141;

macro_rules! rw_block_b_0142 {
    ($ent:ident, $is3:ident, $iter:ident, $nb_elem3:ident) => {
        $iter.get_one_item(&$ent.assigned_date_and_time().cast());
        rw_block_b_0185!($ent, $is3, $iter, $nb_elem3);
    };
}
pub(crate) use rw_block_b_0142;

macro_rules! rw_block_b_0143 {
    ($ent:ident, $is3:ident, $iter:ident, $nb_elem3:ident) => {
        $iter.get_one_item(&$ent.assigned_date().cast());
        rw_block_b_0185!($ent, $is3, $iter, $nb_elem3);
    };
}
pub(crate) use rw_block_b_0143;

macro_rules! rw_block_b_0144 {
    ($ent:ident, $is3:ident, $iter:ident, $nb_elem3:ident) => {
        $iter.get_one_item(&$ent.assigned_organization().cast());
        rw_block_b_0185!($ent, $is3, $iter, $nb_elem3);
    };
}
pub(crate) use rw_block_b_0144;

macro_rules! rw_block_b_0145 {
    ($ent:ident, $is3:ident, $iter:ident, $nb_elem3:ident) => {
        $iter.get_one_item(&$ent.assigned_person_and_organization().cast());
        rw_block_b_0185!($ent, $is3, $iter, $nb_elem3);
    };
}
pub(crate) use rw_block_b_0145;

macro_rules! rw_block_b_0146 {
    ($ent:ident, $is1:ident, $iter:ident, $nb_elem1:ident) => {
        rw_block_b_0186!($ent, $is1, $iter, $nb_elem1);
        $iter.get_one_item(&$ent.over_ridden_style().cast());
    };
}
pub(crate) use rw_block_b_0146;

macro_rules! rw_block_b_0147 {
    ($a_description:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_b_0191!($a_description, $a_name, $ach, $data, $num);
        $data.read_string($num, 2, "description", $ach, &mut $a_description)?;
    };
}
pub(crate) use rw_block_b_0147;

macro_rules! rw_block_b_0148 {
    ($a_description:ident, $a_property_id:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_property_id: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "property_id", $ach, &mut $a_property_id)?;
        let mut $a_description: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 2, "description", $ach, &mut $a_description)?;
    };
}
pub(crate) use rw_block_b_0148;

macro_rules! rw_block_b_0149 {
    ($a_group_description:ident, $a_group_name:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_b_0193!($a_group_description, $a_group_name, $ach, $data, $num);
        $data.read_string(
            $num,
            2,
            "group.description",
            $ach,
            &mut $a_group_description,
        )?;
    };
}
pub(crate) use rw_block_b_0149;

macro_rules! rw_block_b_0150 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.parent.parent.name())?;
        $sw.send_entity(&$ent.parent.parent.description())?;
        $sw.send_entity(&$ent.parent.parent.of_shape())?;
        $sw.send_logical($ent.parent.parent.product_definitional())?;
    };
}
pub(crate) use rw_block_b_0150;

macro_rules! rw_block_b_0151 {
    ($ent:ident, $i:ident, $iter:ident, $nb:ident) => {
        $iter.add_item(&$ent.zone().cast());
        let $i: i32 = Default::default();
        let $nb: i32 = $ent.nb_boundaries();
        for $i in 1..=$nb {
            $iter.add_item(&$ent.boundaries_value($i)?.cast());
        }
    };
}
pub(crate) use rw_block_b_0151;

macro_rules! rw_block_b_0152 {
    ($a_fea_constants:ident, $a_representation_item_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "representation_item.name",
            $ach,
            &mut $a_representation_item_name,
        )?;
        let mut $a_fea_constants: StepFEASymmetricTensor23d = Default::default();
        $data.read_entity_select($num, 2, "fea_constants", $ach, &mut $a_fea_constants)?;
    };
}
pub(crate) use rw_block_b_0152;

macro_rules! rw_block_b_0153 {
    ($ent:ident, $i:ident, $iter:ident, $nb:ident) => {
        $iter.add_item(&$ent.definition().value());
        let $i: i32 = Default::default();
        let $nb: i32 = $ent.nb_identified_item();
        for $i in 1..=$nb {
            $iter.add_item(&$ent.identified_item_value($i)?.cast());
        }
    };
}
pub(crate) use rw_block_b_0153;
