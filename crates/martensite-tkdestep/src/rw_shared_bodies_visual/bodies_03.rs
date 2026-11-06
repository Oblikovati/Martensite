// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0017 {
    ($a_datum_system:ident, $a_description:ident, $a_magnitude:ident, $a_name:ident, $a_toleranced_shape_aspect:ident, $ach:ident, $an_it0:ident, $data:ident, $i0:ident, $nb0:ident, $num:ident, $num0:ident, $num2:ident, $sub5:ident) => {
        rw_block_b_0033!(
            $a_description,
            $a_magnitude,
            $a_name,
            $a_toleranced_shape_aspect,
            $ach,
            $data,
            $num,
            $num0
        );
        $data.named_for_complex_short(
            "GEOMETRIC_TOLERANCE_WITH_DATUM_REFERENCE",
            "GTWDR",
            $num0,
            &mut $num,
            $ach,
        );
        let mut $a_datum_system: Handle<HArray1<StepDimTolDatumSystemOrReference>> =
            Default::default();
        let mut $sub5: i32 = 0;
        if $data.read_sub_list($num, 1, "datum_system", $ach, &mut $sub5, false, 0, 0)? {
            let $nb0: i32 = $data.nb_params($sub5)?;
            $a_datum_system = step_new_array1::<StepDimTolDatumSystemOrReference>(1, $nb0)?;
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
                step_array1_set(&$a_datum_system, $i0, $an_it0.clone(), "ReadStep")?;
            }
        }
    };
}
pub(crate) use rw_block_b_0017;

macro_rules! rw_block_b_0018 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0029!($the_ent, $the_sw);
        if $the_ent.has_lower_limit_actual_translation() {
            $the_sw.send_real($the_ent.lower_limit_actual_translation())?;
        } else {
            $the_sw.send_undef()?;
        }
        if $the_ent.has_upper_limit_actual_translation() {
            $the_sw.send_real($the_ent.upper_limit_actual_translation())?;
        } else {
            $the_sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_b_0018;

macro_rules! rw_block_b_0019 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0029!($the_ent, $the_sw);
        rw_block_b_0208!($the_ent, $the_sw);
    };
}
pub(crate) use rw_block_b_0019;

macro_rules! rw_block_b_0020 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0029!($the_ent, $the_sw);
        rw_block_b_0210!($the_ent, $the_sw);
    };
}
pub(crate) use rw_block_b_0020;

macro_rules! rw_block_b_0021 {
    ($a_first:ident, $a_gtwm:ident, $a_last:ident, $a_max_tol:ident, $a_modifiers:ident, $a_type:ident, $ach:ident, $an_it0:ident, $data:ident, $i0:ident, $nb0:ident, $num:ident, $num0:ident, $num2:ident, $sub:ident, $text:ident) => {
        $data.named_for_complex(
            "GEOMETRIC_TOLERANCE_WITH_MAXIMUM_TOLERANCE",
            $num0,
            &mut $num,
            $ach,
        );
        let mut $a_max_tol: Handle<StepBasicLengthMeasureWithUnit> = Default::default();
        $data.read_entity(
            $num,
            1,
            "maximum_upper_tolerance",
            $ach,
            StepBasicLengthMeasureWithUnit::static_type(),
            &mut $a_max_tol,
        )?;
        rw_block_b_0034!(
            $a_first,
            $a_gtwm,
            $a_last,
            $a_modifiers,
            $a_type,
            $ach,
            $an_it0,
            $data,
            $i0,
            $nb0,
            $num,
            $num0,
            $num2,
            $sub,
            $text
        );
    };
}
pub(crate) use rw_block_b_0021;

macro_rules! rw_block_b_0022 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_representation_item_name:ident, $has_item_defined_transformation_description:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            1,
            "representation_item.name",
            $the_arch,
            &mut $a_representation_item_name,
        )?;
        let mut $a_item_defined_transformation_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            2,
            "item_defined_transformation.name",
            $the_arch,
            &mut $a_item_defined_transformation_name,
        )?;
        let mut $a_item_defined_transformation_description: Handle<HAsciiString> =
            Default::default();
        let mut $has_item_defined_transformation_description: bool = true;
        if $the_data.is_param_defined($the_num, 3)? {
            $the_data.read_string(
                $the_num,
                3,
                "item_defined_transformation.description",
                $the_arch,
                &mut $a_item_defined_transformation_description,
            )?;
        } else {
            $has_item_defined_transformation_description = false;
            $a_item_defined_transformation_description = Default::default();
        }
        let mut $a_item_defined_transformation_transform_item1: Handle<StepReprRepresentationItem> =
            Default::default();
        $the_data.read_entity(
            $the_num,
            4,
            "item_defined_transformation.transform_item1",
            $the_arch,
            StepReprRepresentationItem::static_type(),
            &mut $a_item_defined_transformation_transform_item1,
        )?;
        let mut $a_item_defined_transformation_transform_item2: Handle<StepReprRepresentationItem> =
            Default::default();
        $the_data.read_entity(
            $the_num,
            5,
            "item_defined_transformation.transform_item2",
            $the_arch,
            StepReprRepresentationItem::static_type(),
            &mut $a_item_defined_transformation_transform_item2,
        )?;
        let mut $a_kinematic_pair_joint: Handle<StepKinematicsKinematicJoint> = Default::default();
        $the_data.read_entity(
            $the_num,
            6,
            "kinematic_pair.joint",
            $the_arch,
            StepKinematicsKinematicJoint::static_type(),
            &mut $a_kinematic_pair_joint,
        )?;
    };
}
pub(crate) use rw_block_b_0022;

macro_rules! rw_block_b_0023 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0029!($the_ent, $the_sw);
        if $the_ent.has_input_skew_angle() {
            $the_sw.send_real($the_ent.input_skew_angle())?;
        } else {
            $the_sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_b_0023;

macro_rules! rw_block_b_0024 {
    ($a_type:ident, $ent:ident, $i4:ident, $sw:ident, $var0:ident) => {
        rw_block_b_0030!($a_type, $ent, $i4, $sw, $var0);
        rw_block_b_0217!($a_type, $sw);
    };
}
pub(crate) use rw_block_b_0024;

macro_rules! rw_block_b_0025 {
    ($a_element_representation_node_list:ident, $a_model_ref:ident, $a_representation_context_of_items:ident, $a_representation_items:ident, $a_representation_name:ident, $ach:ident, $an_it0:ident, $data:ident, $i0:ident, $nb0:ident, $num:ident, $num2:ident, $sub2:ident, $sub4:ident) => {
        rw_block_b_0066!(
            $a_representation_context_of_items,
            $a_representation_items,
            $a_representation_name,
            $ach,
            $an_it0,
            $data,
            $i0,
            $nb0,
            $num,
            $num2,
            $sub2
        );
        let mut $a_element_representation_node_list: Handle<
            HArray1<Handle<StepFEANodeRepresentation>>,
        > = Default::default();
        let mut $sub4: i32 = 0;
        if $data.read_sub_list(
            $num,
            4,
            "element_representation.node_list",
            $ach,
            &mut $sub4,
            false,
            0,
            0,
        )? {
            let $nb0: i32 = $data.nb_params($sub4)?;
            $a_element_representation_node_list =
                step_new_array1::<Handle<StepFEANodeRepresentation>>(1, $nb0)?;
            let $num2: i32 = $sub4;
            for $i0 in 1..=$nb0 {
                let mut $an_it0: Handle<StepFEANodeRepresentation> = Default::default();
                $data.read_entity(
                    $num2,
                    $i0,
                    "node_representation",
                    $ach,
                    StepFEANodeRepresentation::static_type(),
                    &mut $an_it0,
                )?;
                step_array1_set(
                    &$a_element_representation_node_list,
                    $i0,
                    $an_it0.clone(),
                    "ReadStep",
                )?;
            }
        }
        let mut $a_model_ref: Handle<StepFEAFeaModel3d> = Default::default();
        $data.read_entity(
            $num,
            5,
            "model_ref",
            $ach,
            StepFEAFeaModel3d::static_type(),
            &mut $a_model_ref,
        )?;
    };
}
pub(crate) use rw_block_b_0025;
