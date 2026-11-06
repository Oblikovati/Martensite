// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0040 {
    ($a_item:ident, $a_name:ident, $a_styles:ident, $ach:ident, $anent2:ident, $data:ident, $ent:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        if !$data.check_nb_params($num, 3, $ach, "styled_item")? {
            return Ok(());
        }
        rw_block_b_0043!(
            $a_item, $a_name, $a_styles, $ach, $anent2, $data, $ent, $i2, $nb2, $nsub2, $num
        );
    };
}
pub(crate) use rw_block_b_0040;

macro_rules! rw_block_b_0041 {
    ($ent:ident, $i1:ident, $i3:ident, $sw:ident, $var0:ident) => {
        rw_block_b_0104!($ent, $i1, $sw, $var0);
        $sw.open_sub()?;
        for $i3 in 1..=step_live(&$ent.parent.node_list(), "WriteStep")?
            .array1()
            .length()
        {
            let $var0: Handle<StepFEANodeRepresentation> =
                step_array1_item(&$ent.parent.node_list(), $i3, "WriteStep")?;
            $sw.send_entity(&$var0)?;
        }
        $sw.close_sub()?;
        $sw.send_entity(&$ent.model_ref())?;
        $sw.send_entity(&$ent.element_descriptor())?;
    };
}
pub(crate) use rw_block_b_0041;

macro_rules! rw_block_b_0042 {
    ($a_description:ident, $a_magnitude:ident, $a_modifiers:ident, $a_name:ident, $a_toleranced_shape_aspect:ident, $ach:ident, $data:ident, $num:ident, $sub5:ident) => {
        rw_block_b_0061!(
            $a_description,
            $a_magnitude,
            $a_name,
            $a_toleranced_shape_aspect,
            $ach,
            $data,
            $num
        );
        let mut $a_modifiers: Handle<HArray1<StepDimTolGeometricToleranceModifier>> =
            Default::default();
        let mut $sub5: i32 = 0;
    };
}
pub(crate) use rw_block_b_0042;

macro_rules! rw_block_b_0043 {
    ($a_item:ident, $a_name:ident, $a_styles:ident, $ach:ident, $anent2:ident, $data:ident, $ent:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        rw_block_b_0055!(
            $a_item, $a_name, $a_styles, $ach, $anent2, $data, $i2, $nb2, $nsub2, $num
        );
        $ent.init(&$a_name, &$a_styles, &$a_item);
    };
}
pub(crate) use rw_block_b_0043;

macro_rules! rw_block_b_0044 {
    ($a_context_of_items:ident, $a_items:ident, $a_name:ident, $ach:ident, $anent2:ident, $data:ident, $ent:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        rw_block_b_0109!($a_items, $a_name, $ach, $anent2, $data, $nsub2, $num);
        if $data.read_sub_list($num, 2, "items", $ach, &mut $nsub2, false, 0, 0)? {
            let $nb2: i32 = $data.nb_params($nsub2)?;
            $a_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, $nb2)?;
            for $i2 in 1..=$nb2 {
                if $data.read_entity(
                    $nsub2,
                    $i2,
                    "representation_item",
                    $ach,
                    StepReprRepresentationItem::static_type(),
                    &mut $anent2,
                )? {
                    step_array1_set(&$a_items, $i2, $anent2.clone(), "ReadStep")?;
                }
            }
        }
        rw_block_b_0183!(
            $a_context_of_items,
            $a_items,
            $a_name,
            $ach,
            $data,
            $ent,
            $num
        );
    };
}
pub(crate) use rw_block_b_0044;

macro_rules! rw_block_b_0045 {
    ($a_assigned_date_and_time:ident, $a_items:ident, $a_items_item:ident, $a_role:ident, $ach:ident, $data:ident, $ent:ident, $i3:ident, $nb3:ident, $nsub3:ident, $num:ident, $stat3:ident) => {
        rw_block_b_0133!($a_assigned_date_and_time, $a_role, $ach, $data, $num);
        let mut $a_items: Handle<HArray1<StepAP214AutoDesignDateAndTimeItem>> = Default::default();
        let mut $a_items_item: StepAP214AutoDesignDateAndTimeItem = Default::default();
        let mut $nsub3: i32 = Default::default();
        if $data.read_sub_list($num, 3, "items", $ach, &mut $nsub3, false, 0, 0)? {
            let $nb3: i32 = $data.nb_params($nsub3)?;
            $a_items = step_new_array1::<StepAP214AutoDesignDateAndTimeItem>(1, $nb3)?;
            for $i3 in 1..=$nb3 {
                let $stat3: bool =
                    $data.read_entity_select($nsub3, $i3, "items", $ach, &mut $a_items_item)?;
                if $stat3 {
                    step_array1_set(&$a_items, $i3, $a_items_item.clone(), "ReadStep")?;
                }
            }
        }
        $ent.init(&$a_assigned_date_and_time, &$a_role, &$a_items);
    };
}
pub(crate) use rw_block_b_0045;

macro_rules! rw_block_b_0046 {
    ($a_assigned_date:ident, $a_items:ident, $a_items_item:ident, $a_role:ident, $ach:ident, $data:ident, $ent:ident, $i3:ident, $nb3:ident, $nsub3:ident, $num:ident, $stat3:ident) => {
        rw_block_b_0134!($a_assigned_date, $a_role, $ach, $data, $num);
        let mut $a_items: Handle<HArray1<StepAP214AutoDesignDatedItem>> = Default::default();
        let mut $a_items_item: StepAP214AutoDesignDatedItem = Default::default();
        let mut $nsub3: i32 = Default::default();
        if $data.read_sub_list($num, 3, "items", $ach, &mut $nsub3, false, 0, 0)? {
            let $nb3: i32 = $data.nb_params($nsub3)?;
            $a_items = step_new_array1::<StepAP214AutoDesignDatedItem>(1, $nb3)?;
            for $i3 in 1..=$nb3 {
                let $stat3: bool =
                    $data.read_entity_select($nsub3, $i3, "items", $ach, &mut $a_items_item)?;
                if $stat3 {
                    step_array1_set(&$a_items, $i3, $a_items_item.clone(), "ReadStep")?;
                }
            }
        }
        $ent.init(&$a_assigned_date, &$a_role, &$a_items);
    };
}
pub(crate) use rw_block_b_0046;

macro_rules! rw_block_b_0047 {
    ($a_type:ident, $ent:ident, $sw:ident) => {
        rw_block_b_0068!($a_type, $ent, $sw);
        step_start_entity($sw, "GEOMETRIC_TOLERANCE_WITH_MAXIMUM_TOLERANCE")?;
        $sw.send_entity(&$ent.get_max_tolerance())?;
    };
}
pub(crate) use rw_block_b_0047;

macro_rules! rw_block_b_0048 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0084!($the_ent, $the_sw);
        $the_sw.send_entity(&$the_ent.curve1())?;
        $the_sw.send_entity(&$the_ent.curve2())?;
        $the_sw.send_boolean($the_ent.orientation())?;
    };
}
pub(crate) use rw_block_b_0048;

macro_rules! rw_block_b_0049 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0084!($the_ent, $the_sw);
        $the_sw.send_entity(&$the_ent.surface1())?;
        $the_sw.send_entity(&$the_ent.surface2())?;
        $the_sw.send_boolean($the_ent.orientation())?;
    };
}
pub(crate) use rw_block_b_0049;

macro_rules! rw_block_b_0050 {
    ($ent:ident, $i4:ident, $sw:ident, $var0:ident) => {
        rw_block_b_0121!($ent, $sw);
        rw_block_b_0128!($ent, $i4, $sw, $var0);
    };
}
pub(crate) use rw_block_b_0050;

macro_rules! rw_block_b_0051 {
    ($a_description:ident, $a_magnitude:ident, $a_name:ident, $a_toleranced_shape_aspect:ident, $ach:ident, $data:ident, $num:ident) => {
        if !$data.check_nb_params($num, 4, $ach, "geometric_tolerance")? {
            return Ok(());
        }
        rw_block_b_0147!($a_description, $a_name, $ach, $data, $num);
        let mut $a_magnitude: Handle<StandardTransient> = Default::default();
        $data.read_entity(
            $num,
            3,
            "magnitude",
            $ach,
            StandardTransient::static_type(),
            &mut $a_magnitude,
        )?;
        let mut $a_toleranced_shape_aspect: StepDimTolGeometricToleranceTarget = Default::default();
        $data.read_entity_select(
            $num,
            4,
            "toleranced_shape_aspect",
            $ach,
            &mut $a_toleranced_shape_aspect,
        )?;
    };
}
pub(crate) use rw_block_b_0051;

macro_rules! rw_block_b_0052 {
    ($a_geometric_tolerance_description:ident, $a_geometric_tolerance_magnitude:ident, $a_geometric_tolerance_name:ident, $a_geometric_tolerance_toleranced_shape_aspect:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        rw_block_b_0060!(
            $a_geometric_tolerance_description,
            $a_geometric_tolerance_magnitude,
            $a_geometric_tolerance_name,
            $a_geometric_tolerance_toleranced_shape_aspect,
            $ach,
            $data,
            $num
        );
        $ent.init_geometric_tolerance_target(
            &$a_geometric_tolerance_name,
            &$a_geometric_tolerance_description,
            &$a_geometric_tolerance_magnitude,
            &$a_geometric_tolerance_toleranced_shape_aspect,
        );
    };
}
pub(crate) use rw_block_b_0052;
