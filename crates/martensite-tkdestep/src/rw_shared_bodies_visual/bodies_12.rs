// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0097 {
    ($a_name:ident, $a_size_in_x:ident, $a_size_in_y:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_size_in_x: f64 = Default::default();
        $data.read_real($num, 2, "size_in_x", $ach, &mut $a_size_in_x)?;
        let mut $a_size_in_y: f64 = Default::default();
        $data.read_real($num, 3, "size_in_y", $ach, &mut $a_size_in_y)?;
    };
}
pub(crate) use rw_block_b_0097;

macro_rules! rw_block_b_0098 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        rw_block_b_0119!($ent, $i2, $sw);
        $sw.send_entity(&$ent.over_ridden_style())?;
    };
}
pub(crate) use rw_block_b_0098;

macro_rules! rw_block_b_0099 {
    ($a_items:ident, $a_representation_item_name:ident, $an_it0:ident, $i0:ident, $nb0:ident, $num2:ident, $sub2:ident, $the_check:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            1,
            "representation_item.name",
            $the_check,
            &mut $a_representation_item_name,
        )?;
        let mut $a_items: Handle<HArray1<Handle<StepVisualTessellatedStructuredItem>>> =
            Default::default();
        let mut $sub2: i32 = 0;
        if $the_data.read_sub_list($the_num, 2, "items", $the_check, &mut $sub2, false, 0, 0)? {
            let $nb0: i32 = $the_data.nb_params($sub2)?;
            $a_items = step_new_array1::<Handle<StepVisualTessellatedStructuredItem>>(1, $nb0)?;
            let $num2: i32 = $sub2;
            for $i0 in 1..=$nb0 {
                let mut $an_it0: Handle<StepVisualTessellatedStructuredItem> = Default::default();
                $the_data.read_entity(
                    $num2,
                    $i0,
                    "tessellated_structured_item",
                    $the_check,
                    StepVisualTessellatedStructuredItem::static_type(),
                    &mut $an_it0,
                )?;
                step_array1_set(&$a_items, $i0, $an_it0.clone(), "ReadStep")?;
            }
        }
    };
}
pub(crate) use rw_block_b_0099;

macro_rules! rw_block_b_0100 {
    ($ent:ident, $i1:ident, $i3:ident, $iter:ident, $var0:ident) => {
        rw_block_b_0206!($ent, $i1, $iter, $var0);
        for $i3 in 1..=step_live(&$ent.parent.node_list(), "Share")?
            .array1()
            .length()
        {
            let $var0: Handle<StepFEANodeRepresentation> =
                step_array1_item(&$ent.parent.node_list(), $i3, "Share")?;
            $iter.add_item(&$var0.cast());
        }
        $iter.add_item(&$ent.model_ref().cast());
        $iter.add_item(&$ent.element_descriptor().cast());
    };
}
pub(crate) use rw_block_b_0100;

macro_rules! rw_block_b_0101 {
    ($ent:ident, $i:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.description())?;
        $sw.send_entity(&$ent.definition().value())?;
        $sw.send_entity(&$ent.used_representation())?;
        if $ent.nb_identified_item() == 1 {
            $sw.send_entity(&$ent.identified_item_value(1)?)?;
        } else {
            $sw.open_sub()?;
            for $i in 1..=$ent.nb_identified_item() {
                $sw.send_entity(&$ent.identified_item_value($i)?)?;
            }
            $sw.close_sub()?;
        }
    };
}
pub(crate) use rw_block_b_0101;

macro_rules! rw_block_b_0102 {
    ($iter:ident, $the_ent:ident) => {
        rw_block_b_0175!($iter, $the_ent);
        $iter.add_item(&$the_ent.parent.curve1().cast());
        $iter.add_item(&$the_ent.parent.curve2().cast());
    };
}
pub(crate) use rw_block_b_0102;

macro_rules! rw_block_b_0103 {
    ($iter:ident, $the_ent:ident) => {
        rw_block_b_0175!($iter, $the_ent);
        $iter.add_item(&$the_ent.parent.surface1().cast());
        $iter.add_item(&$the_ent.parent.surface2().cast());
    };
}
pub(crate) use rw_block_b_0103;

macro_rules! rw_block_b_0104 {
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
pub(crate) use rw_block_b_0104;

macro_rules! rw_block_b_0105 {
    ($i1:ident, $the_ent:ident, $the_sw:ident, $var0:ident) => {
        $the_sw.send_entity(&$the_ent.name())?;
        $the_sw.open_sub()?;
        for $i1 in 1..=step_live(&$the_ent.items(), "WriteStep")?.array1().length() {
            let $var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&$the_ent.items(), $i1, "WriteStep")?;
            $the_sw.send_entity(&$var0)?;
        }
        $the_sw.close_sub()?;
        $the_sw.send_entity(&$the_ent.context_of_items())?;
    };
}
pub(crate) use rw_block_b_0105;

macro_rules! rw_block_b_0106 {
    ($a_group_description:ident, $a_group_name:ident, $ach:ident, $data:ident, $has_group_description:ident, $num:ident) => {
        rw_block_b_0193!($a_group_description, $a_group_name, $ach, $data, $num);
        let mut $has_group_description: bool = true;
        if $data.is_param_defined($num, 2)? {
            $data.read_string(
                $num,
                2,
                "group.description",
                $ach,
                &mut $a_group_description,
            )?;
        } else {
            $has_group_description = false;
        }
    };
}
pub(crate) use rw_block_b_0106;

macro_rules! rw_block_b_0107 {
    ($ent:ident, $i1:ident, $sw:ident, $var0:ident) => {
        $sw.send_entity(&$ent.parent.name())?;
        $sw.open_sub()?;
        for $i1 in 1..=$ent.parent.nb_items() {
            let $var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&$ent.parent.items(), $i1, "WriteStep")?;
            $sw.send_entity(&$var0)?;
        }
        $sw.close_sub()?;
        $sw.send_entity(&$ent.parent.context_of_items())?;
    };
}
pub(crate) use rw_block_b_0107;

macro_rules! rw_block_b_0108 {
    ($a_coordinate_system:ident, $a_representation_item_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "representation_item.name",
            $ach,
            &mut $a_representation_item_name,
        )?;
        let mut $a_coordinate_system: Handle<StepFEAFeaAxis2Placement3d> = Default::default();
        $data.read_entity(
            $num,
            2,
            "coordinate_system",
            $ach,
            StepFEAFeaAxis2Placement3d::static_type(),
            &mut $a_coordinate_system,
        )?;
        $ent.init(&$a_representation_item_name, &$a_coordinate_system);
    };
}
pub(crate) use rw_block_b_0108;
