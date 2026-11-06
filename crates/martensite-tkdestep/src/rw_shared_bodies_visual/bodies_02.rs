// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0010 {
    ($a_base:ident, $a_datum:ident, $a_drmwv:ident, $a_member:ident, $a_modifiers:ident, $a_new_nb_sub:ident, $a_sdrm:ident, $a_shape_aspect_description:ident, $a_shape_aspect_name:ident, $a_shape_aspect_of_shape:ident, $a_shape_aspect_product_definitional:ident, $a_type:ident, $ach:ident, $an_ent:ident, $an_enum_text:ident, $an_items:ident, $data:ident, $ent:ident, $has_modifiers:ident, $i:ident, $nb_elements:ident, $nb_sub:ident, $num:ident) => {
        if !$data.check_nb_params($num, 6, $ach, "datum_reference_element")? {
            return Ok(());
        }
        rw_block_b_0012!(
            $a_base,
            $a_datum,
            $a_drmwv,
            $a_member,
            $a_modifiers,
            $a_new_nb_sub,
            $a_sdrm,
            $a_shape_aspect_description,
            $a_shape_aspect_name,
            $a_shape_aspect_of_shape,
            $a_shape_aspect_product_definitional,
            $a_type,
            $ach,
            $an_ent,
            $an_enum_text,
            $an_items,
            $data,
            $ent,
            $has_modifiers,
            $i,
            $nb_elements,
            $nb_sub,
            $num
        );
    };
}
pub(crate) use rw_block_b_0010;

macro_rules! rw_block_b_0011 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_orientation:ident, $a_pair_curve:ident, $a_representation_item_name:ident, $has_item_defined_transformation_description:ident, $the_arch:ident, $the_data:ident, $the_ent:ident, $the_num:ident) => {
        rw_block_b_0022!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_representation_item_name,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_pair_curve: Handle<StepGeomCurve> = Default::default();
        $the_data.read_entity(
            $the_num,
            7,
            "pair_curve",
            $the_arch,
            StepGeomCurve::static_type(),
            &mut $a_pair_curve,
        )?;
        let mut $a_orientation: bool = Default::default();
        $the_data.read_boolean($the_num, 8, "orientation", $the_arch, &mut $a_orientation)?;
        $the_ent.init(
            &$a_representation_item_name,
            &$a_item_defined_transformation_name,
            $has_item_defined_transformation_description,
            &$a_item_defined_transformation_description,
            &$a_item_defined_transformation_transform_item1,
            &$a_item_defined_transformation_transform_item2,
            &$a_kinematic_pair_joint,
            &$a_pair_curve,
            $a_orientation,
        );
    };
}
pub(crate) use rw_block_b_0011;

macro_rules! rw_block_b_0012 {
    ($a_base:ident, $a_datum:ident, $a_drmwv:ident, $a_member:ident, $a_modifiers:ident, $a_new_nb_sub:ident, $a_sdrm:ident, $a_shape_aspect_description:ident, $a_shape_aspect_name:ident, $a_shape_aspect_of_shape:ident, $a_shape_aspect_product_definitional:ident, $a_type:ident, $ach:ident, $an_ent:ident, $an_enum_text:ident, $an_items:ident, $data:ident, $ent:ident, $has_modifiers:ident, $i:ident, $nb_elements:ident, $nb_sub:ident, $num:ident) => {
        rw_block_b_0058!(
            $a_shape_aspect_description,
            $a_shape_aspect_name,
            $a_shape_aspect_of_shape,
            $a_shape_aspect_product_definitional,
            $ach,
            $data,
            $num
        );
        let mut $a_base: StepDimTolDatumOrCommonDatum = Default::default();
        let mut $a_datum: Handle<StepDimTolDatum> = Default::default();
        let mut $a_type: InterfaceParamType = $data.param_type($num, 5)?;
        if $a_type == InterfaceParamType::ParamIdent {
            $data.read_entity(
                $num,
                5,
                "general_datum_reference.base",
                $ach,
                StepDimTolDatum::static_type(),
                &mut $a_datum,
            )?;
            $a_base.set_value(&$a_datum.cast())?;
        } else {
            let mut $an_items: Handle<HArray1<Handle<StepDimTolDatumReferenceElement>>> =
                Default::default();
            let mut $an_ent: Handle<StepDimTolDatumReferenceElement> = Default::default();
            let mut $nb_sub: i32 = Default::default();
            if $data.read_sub_list(
                $num,
                5,
                "general_datum_reference.base",
                $ach,
                &mut $nb_sub,
                false,
                0,
                0,
            )? {
                $a_type = $data.param_type($nb_sub, 1)?;
                if $a_type == InterfaceParamType::ParamSub {
                    let mut $a_new_nb_sub: i32 = Default::default();
                    if $data.read_sub_list(
                        $nb_sub,
                        1,
                        "general_datum_reference.base",
                        $ach,
                        &mut $a_new_nb_sub,
                        false,
                        0,
                        0,
                    )? {
                        $nb_sub = $a_new_nb_sub;
                    }
                }
                let $nb_elements: i32 = $data.nb_params($nb_sub)?;
                $an_items =
                    step_new_array1::<Handle<StepDimTolDatumReferenceElement>>(1, $nb_elements)?;
                for $i in 1..=$nb_elements {
                    if $data.read_entity(
                        $nb_sub,
                        $i,
                        "datum_reference_element",
                        $ach,
                        StepDimTolDatumReferenceElement::static_type(),
                        &mut $an_ent,
                    )? {
                        step_array1_set(&$an_items, $i, $an_ent.clone(), "ReadStep")?;
                    }
                }
            }
            $a_base.set_value(&$an_items.cast())?;
        }
        let mut $nb_sub: i32 = Default::default();
        let $has_modifiers: bool = $data.read_sub_list(
            $num,
            6,
            "general_datum_reference.modifiers",
            $ach,
            &mut $nb_sub,
            true,
            0,
            0,
        )?;
        let mut $a_modifiers: Handle<HArray1<StepDimTolDatumReferenceModifier>> =
            Default::default();
        if $has_modifiers {
            let mut $an_ent: StepDimTolDatumReferenceModifier = Default::default();
            let $nb_elements: i32 = $data.nb_params($nb_sub)?;
            $a_modifiers = step_new_array1::<StepDimTolDatumReferenceModifier>(1, $nb_elements)?;
            for $i in 1..=$nb_elements {
                $a_type = $data.param_type($nb_sub, $i)?;
                if $a_type == InterfaceParamType::ParamIdent {
                    let mut $a_drmwv: Handle<StepDimTolDatumReferenceModifierWithValue> =
                        Default::default();
                    $data.read_entity(
                        $nb_sub,
                        $i,
                        "datum_reference_modifier_with_value",
                        $ach,
                        StepDimTolDatumReferenceModifierWithValue::static_type(),
                        &mut $a_drmwv,
                    )?;
                    $an_ent.set_value(&$a_drmwv.cast())?;
                } else {
                    let mut $a_member: Handle<dyn StepDataSelectMember> = Default::default();
                    $data.read_member_typed(
                        $nb_sub,
                        $i,
                        "simple_datum_reference_modifier",
                        $ach,
                        &mut $a_member,
                    )?;
                    let $an_enum_text: Vec<u8> = step_live(&$a_member, "ReadStep")?
                        .enum_text()
                        .as_bytes()
                        .to_vec();
                    let $a_sdrm: Handle<StepDimTolSimpleDatumReferenceModifierMember> =
                        Handle::new(StepDimTolSimpleDatumReferenceModifierMember::new());
                    $a_sdrm.set_enum_text(0, &$an_enum_text);
                    $an_ent.set_value(&$a_sdrm.cast())?;
                }
                step_array1_set(&$a_modifiers, $i, $an_ent.clone(), "ReadStep")?;
            }
        }
        $ent.init(
            &$a_shape_aspect_name,
            &$a_shape_aspect_description,
            &$a_shape_aspect_of_shape,
            $a_shape_aspect_product_definitional,
            &$a_base,
            $has_modifiers,
            &$a_modifiers,
        );
    };
}
pub(crate) use rw_block_b_0012;

macro_rules! rw_block_b_0013 {
    ($a_datum_system:ident, $a_description:ident, $a_gtwdr:ident, $a_magnitude:ident, $a_name:ident, $a_toleranced_shape_aspect:ident, $ach:ident, $an_it0:ident, $data:ident, $i0:ident, $nb0:ident, $num:ident, $num0:ident, $num2:ident, $sub5:ident) => {
        rw_block_b_0017!(
            $a_datum_system,
            $a_description,
            $a_magnitude,
            $a_name,
            $a_toleranced_shape_aspect,
            $ach,
            $an_it0,
            $data,
            $i0,
            $nb0,
            $num,
            $num0,
            $num2,
            $sub5
        );
        let $a_gtwdr: Handle<StepDimTolGeometricToleranceWithDatumReference> =
            Handle::new(StepDimTolGeometricToleranceWithDatumReference::new());
        $a_gtwdr.set_datum_system_fields(&$a_datum_system)?;
    };
}
pub(crate) use rw_block_b_0013;

macro_rules! rw_block_b_0014 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_pinion_radius:ident, $a_representation_item_name:ident, $has_item_defined_transformation_description:ident, $the_arch:ident, $the_data:ident, $the_ent:ident, $the_num:ident) => {
        rw_block_b_0022!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_representation_item_name,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_pinion_radius: f64 = Default::default();
        $the_data.read_real(
            $the_num,
            7,
            "pinion_radius",
            $the_arch,
            &mut $a_pinion_radius,
        )?;
        $the_ent.init(
            &$a_representation_item_name,
            &$a_item_defined_transformation_name,
            $has_item_defined_transformation_description,
            &$a_item_defined_transformation_description,
            &$a_item_defined_transformation_transform_item1,
            &$a_item_defined_transformation_transform_item2,
            &$a_kinematic_pair_joint,
            $a_pinion_radius,
        );
    };
}
pub(crate) use rw_block_b_0014;

macro_rules! rw_block_b_0015 {
    ($a_pnindex:ident, $a_representation_item_name:ident, $a_tessellated_face_coordinates:ident, $a_tessellated_face_geometric_link:ident, $a_tessellated_face_normals:ident, $a_tessellated_face_pnmax:ident, $an_it0:ident, $has_tessellated_face_geometric_link:ident, $i0:ident, $j0:ident, $nb0:ident, $nbj0:ident, $num2:ident, $num4:ident, $sub4:ident, $sub6:ident, $subj4:ident, $the_check:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0026!(
            $a_representation_item_name,
            $a_tessellated_face_coordinates,
            $a_tessellated_face_geometric_link,
            $a_tessellated_face_normals,
            $a_tessellated_face_pnmax,
            $an_it0,
            $has_tessellated_face_geometric_link,
            $i0,
            $j0,
            $nb0,
            $nbj0,
            $num4,
            $sub4,
            $subj4,
            $the_check,
            $the_data,
            $the_num
        );
        let mut $a_pnindex: Handle<HArray1<i32>> = Default::default();
        let mut $sub6: i32 = 0;
        if $the_data.read_sub_list($the_num, 6, "pnindex", $the_check, &mut $sub6, false, 0, 0)? {
            let $nb0: i32 = $the_data.nb_params($sub6)?;
            $a_pnindex = step_new_array1::<i32>(1, $nb0)?;
            let $num2: i32 = $sub6;
            for $i0 in 1..=$nb0 {
                let mut $an_it0: i32 = Default::default();
                $the_data.read_integer($num2, $i0, "integer", $the_check, &mut $an_it0)?;
                step_array1_set(&$a_pnindex, $i0, $an_it0, "ReadStep")?;
            }
        }
    };
}
pub(crate) use rw_block_b_0015;

macro_rules! rw_block_b_0016 {
    ($a_definition:ident, $a_description:ident, $a_name:ident, $a_representation:ident, $a_type:ident, $ach:ident, $an_ent:ident, $an_items:ident, $data:ident, $ent:ident, $i:ident, $nb_elements:ident, $nb_sub:ident, $num:ident) => {
        if !$data.check_nb_params($num, 5, $ach, "geometric_item_specific_usage")? {
            return Ok(());
        }
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "item_identified_representation_usage.name",
            $ach,
            &mut $a_name,
        )?;
        let mut $a_description: Handle<HAsciiString> = Default::default();
        if $data.is_param_defined($num, 2)? {
            $data.read_string(
                $num,
                2,
                "item_identified_representation_usage.description",
                $ach,
                &mut $a_description,
            )?;
        }
        let mut $a_definition: StepAP242ItemIdentifiedRepresentationUsageDefinition =
            Default::default();
        $data.read_entity_select(
            $num,
            3,
            "item_identified_representation_usage.definition",
            $ach,
            &mut $a_definition,
        )?;
        let mut $a_representation: Handle<StepReprRepresentation> = Default::default();
        $data.read_entity(
            $num,
            4,
            "item_identified_representation_usage.used_representation",
            $ach,
            StepReprRepresentation::static_type(),
            &mut $a_representation,
        )?;
        rw_block_b_0079!(
            $a_definition,
            $a_description,
            $a_name,
            $a_representation,
            $a_type,
            $ach,
            $an_ent,
            $an_items,
            $data,
            $ent,
            $i,
            $nb_elements,
            $nb_sub,
            $num
        );
    };
}
pub(crate) use rw_block_b_0016;
