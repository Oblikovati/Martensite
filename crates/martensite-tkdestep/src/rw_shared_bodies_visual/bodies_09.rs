// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0066 {
    ($a_representation_context_of_items:ident, $a_representation_items:ident, $a_representation_name:ident, $ach:ident, $an_it0:ident, $data:ident, $i0:ident, $nb0:ident, $num:ident, $num2:ident, $sub2:ident) => {
        rw_block_b_0154!(
            $a_representation_items,
            $a_representation_name,
            $ach,
            $data,
            $num,
            $sub2
        );
        if $data.read_sub_list(
            $num,
            2,
            "representation.items",
            $ach,
            &mut $sub2,
            false,
            0,
            0,
        )? {
            let $nb0: i32 = $data.nb_params($sub2)?;
            $a_representation_items =
                step_new_array1::<Handle<StepReprRepresentationItem>>(1, $nb0)?;
            let $num2: i32 = $sub2;
            for $i0 in 1..=$nb0 {
                let mut $an_it0: Handle<StepReprRepresentationItem> = Default::default();
                $data.read_entity(
                    $num2,
                    $i0,
                    "representation_item",
                    $ach,
                    StepReprRepresentationItem::static_type(),
                    &mut $an_it0,
                )?;
                step_array1_set(&$a_representation_items, $i0, $an_it0.clone(), "ReadStep")?;
            }
        }
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
    };
}
pub(crate) use rw_block_b_0066;

macro_rules! rw_block_b_0067 {
    ($ent:ident, $i1:ident, $i3:ident, $iter:ident, $var0:ident) => {
        rw_block_b_0100!($ent, $i1, $i3, $iter, $var0);
        $iter.add_item(&$ent.property().cast());
        $iter.add_item(&$ent.material().cast());
    };
}
pub(crate) use rw_block_b_0067;

macro_rules! rw_block_b_0068 {
    ($a_type:ident, $ent:ident, $sw:ident) => {
        rw_block_b_0202!($a_type, $ent, $sw);
        rw_block_b_0121!($ent, $sw);
    };
}
pub(crate) use rw_block_b_0068;

macro_rules! rw_block_b_0069 {
    ($a_collected_text:ident, $a_collected_text_item:ident, $a_name:ident, $ach:ident, $data:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_collected_text: Handle<HArray1<StepVisualTextOrCharacter>> = Default::default();
        let mut $a_collected_text_item: StepVisualTextOrCharacter = Default::default();
        let mut $nsub2: i32 = Default::default();
        $nsub2 = $data.sub_list_number($num, 2, false)?;
        if $nsub2 != 0 {
            let $nb2: i32 = $data.nb_params($nsub2)?;
            $a_collected_text = step_new_array1::<StepVisualTextOrCharacter>(1, $nb2)?;
            for $i2 in 1..=$nb2 {
                if $data.read_entity_select(
                    $nsub2,
                    $i2,
                    "collected_text",
                    $ach,
                    &mut $a_collected_text_item,
                )? {
                    step_array1_set(
                        &$a_collected_text,
                        $i2,
                        $a_collected_text_item.clone(),
                        "ReadStep",
                    )?;
                }
            }
        } else {
            $ach.add_fail_text("Parameter #2 (collected_text) is not a LIST", "");
        }
    };
}
pub(crate) use rw_block_b_0069;

macro_rules! rw_block_b_0070 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0084!($the_ent, $the_sw);
        $the_sw.send_entity(&$the_ent.pair_surface())?;
    };
}
pub(crate) use rw_block_b_0070;

macro_rules! rw_block_b_0071 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0084!($the_ent, $the_sw);
        $the_sw.send_real($the_ent.pinion_radius())?;
    };
}
pub(crate) use rw_block_b_0071;

macro_rules! rw_block_b_0072 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0084!($the_ent, $the_sw);
        $the_sw.send_real($the_ent.pitch())?;
    };
}
pub(crate) use rw_block_b_0072;

macro_rules! rw_block_b_0073 {
    ($ent:ident, $i4:ident, $sw:ident, $var0:ident) => {
        $sw.send_entity(&$ent.parent.parent.name())?;
        $sw.send_entity(&$ent.parent.parent.description())?;
        $sw.send_entity(&$ent.parent.parent.magnitude())?;
        $sw.send_entity(&$ent.parent.parent.toleranced_shape_aspect().value())?;
        $sw.open_sub()?;
        for $i4 in 1..=step_live(&$ent.parent.datum_system_ap242(), "WriteStep")?
            .array1()
            .length()
        {
            let $var0: StepDimTolDatumSystemOrReference =
                step_array1_item(&$ent.parent.datum_system_ap242(), $i4, "WriteStep")?;
            $sw.send_entity(&$var0.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0073;

macro_rules! rw_block_b_0074 {
    ($a_mapping_source:ident, $a_mapping_target:ident, $a_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_mapping_source: Handle<StepReprRepresentationMap> = Default::default();
        $data.read_entity(
            $num,
            2,
            "mapping_source",
            $ach,
            StepReprRepresentationMap::static_type(),
            &mut $a_mapping_source,
        )?;
        let mut $a_mapping_target: Handle<StepReprRepresentationItem> = Default::default();
        $data.read_entity(
            $num,
            3,
            "mapping_target",
            $ach,
            StepReprRepresentationItem::static_type(),
            &mut $a_mapping_target,
        )?;
        $ent.init(&$a_name, &$a_mapping_source, &$a_mapping_target);
    };
}
pub(crate) use rw_block_b_0074;

macro_rules! rw_block_b_0075 {
    ($a_representation_item_name:ident, $a_tessellated_surface_set_coordinates:ident, $a_tessellated_surface_set_normals:ident, $a_tessellated_surface_set_pnmax:ident, $the_check:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            1,
            "representation_item.name",
            $the_check,
            &mut $a_representation_item_name,
        )?;
        let mut $a_tessellated_surface_set_coordinates: Handle<StepVisualCoordinatesList> =
            Default::default();
        $the_data.read_entity(
            $the_num,
            2,
            "tessellated_surface_set.coordinates",
            $the_check,
            StepVisualCoordinatesList::static_type(),
            &mut $a_tessellated_surface_set_coordinates,
        )?;
        let mut $a_tessellated_surface_set_pnmax: i32 = Default::default();
        $the_data.read_integer(
            $the_num,
            3,
            "tessellated_surface_set.pnmax",
            $the_check,
            &mut $a_tessellated_surface_set_pnmax,
        )?;
        let mut $a_tessellated_surface_set_normals: Handle<HArray2<f64>> = Default::default();
    };
}
pub(crate) use rw_block_b_0075;

macro_rules! rw_block_b_0076 {
    ($a_actual_rotation:ident, $a_pair_value_applies_to_pair:ident, $a_representation_item_name:ident, $the_arch:ident, $the_data:ident, $the_ent:ident, $the_num:ident) => {
        rw_block_b_0096!(
            $a_actual_rotation,
            $a_pair_value_applies_to_pair,
            $a_representation_item_name,
            $the_arch,
            $the_data,
            $the_num
        );
        $the_ent.init(
            &$a_representation_item_name,
            &$a_pair_value_applies_to_pair,
            $a_actual_rotation,
        );
    };
}
pub(crate) use rw_block_b_0076;

macro_rules! rw_block_b_0077 {
    ($a_angle:ident, $a_axis:ident, $a_representation_item_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "representation_item.name",
            $ach,
            &mut $a_representation_item_name,
        )?;
        let mut $a_axis: i32 = Default::default();
        $data.read_integer($num, 2, "axis", $ach, &mut $a_axis)?;
        let mut $a_angle: f64 = Default::default();
        $data.read_real($num, 3, "angle", $ach, &mut $a_angle)?;
        $ent.init(&$a_representation_item_name, $a_axis, $a_angle);
    };
}
pub(crate) use rw_block_b_0077;
