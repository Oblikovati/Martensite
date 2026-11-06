// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0124 {
    ($a_invisible_items:ident, $a_invisible_items_item:ident, $ach:ident, $data:ident, $i1:ident, $nb1:ident, $nsub1:ident, $num:ident) => {
        let mut $a_invisible_items: Handle<HArray1<StepVisualInvisibleItem>> = Default::default();
        let mut $a_invisible_items_item: StepVisualInvisibleItem = Default::default();
        let mut $nsub1: i32 = Default::default();
        if $data.read_sub_list($num, 1, "invisible_items", $ach, &mut $nsub1, false, 0, 0)? {
            let $nb1: i32 = $data.nb_params($nsub1)?;
            $a_invisible_items = step_new_array1::<StepVisualInvisibleItem>(1, $nb1)?;
            for $i1 in 1..=$nb1 {
                if $data.read_entity_select(
                    $nsub1,
                    $i1,
                    "invisible_items",
                    $ach,
                    &mut $a_invisible_items_item,
                )? {
                    step_array1_set(
                        &$a_invisible_items,
                        $i1,
                        $a_invisible_items_item.clone(),
                        "ReadStep",
                    )?;
                }
            }
        }
    };
}
pub(crate) use rw_block_b_0124;

macro_rules! rw_block_b_0125 {
    ($the_ent:ident, $the_sw:ident) => {
        $the_sw.send_entity(
            &step_live(&$the_ent.item_defined_transformation(), "WriteStep")?.name(),
        )?;
        if step_live(&$the_ent.item_defined_transformation(), "WriteStep")?.has_description() {
            $the_sw.send_entity(
                &step_live(&$the_ent.item_defined_transformation(), "WriteStep")?.description(),
            )?;
        } else {
            $the_sw.send_undef()?;
        }
        $the_sw.send_entity(
            &step_live(&$the_ent.item_defined_transformation(), "WriteStep")?.transform_item1(),
        )?;
        $the_sw.send_entity(
            &step_live(&$the_ent.item_defined_transformation(), "WriteStep")?.transform_item2(),
        )?;
    };
}
pub(crate) use rw_block_b_0125;

macro_rules! rw_block_b_0126 {
    ($the_ent:ident, $the_sw:ident) => {
        if $the_ent.has_lower_limit_actual_translation_x() {
            $the_sw.send_real($the_ent.lower_limit_actual_translation_x())?;
        } else {
            $the_sw.send_undef()?;
        }
        if $the_ent.has_upper_limit_actual_translation_x() {
            $the_sw.send_real($the_ent.upper_limit_actual_translation_x())?;
        } else {
            $the_sw.send_undef()?;
        }
        if $the_ent.has_lower_limit_actual_translation_y() {
            $the_sw.send_real($the_ent.lower_limit_actual_translation_y())?;
        } else {
            $the_sw.send_undef()?;
        }
        if $the_ent.has_upper_limit_actual_translation_y() {
            $the_sw.send_real($the_ent.upper_limit_actual_translation_y())?;
        } else {
            $the_sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_b_0126;

macro_rules! rw_block_b_0127 {
    ($the_ent:ident, $the_sw:ident) => {
        if $the_ent.has_lower_limit_pitch() {
            $the_sw.send_real($the_ent.lower_limit_pitch())?;
        } else {
            $the_sw.send_undef()?;
        }
        if $the_ent.has_upper_limit_pitch() {
            $the_sw.send_real($the_ent.upper_limit_pitch())?;
        } else {
            $the_sw.send_undef()?;
        }
        rw_block_b_0209!($the_ent, $the_sw);
    };
}
pub(crate) use rw_block_b_0127;

macro_rules! rw_block_b_0128 {
    ($ent:ident, $i4:ident, $sw:ident, $var0:ident) => {
        step_start_entity($sw, "GEOMETRIC_TOLERANCE_WITH_DATUM_REFERENCE")?;
        $sw.open_sub()?;
        for $i4 in 1..=step_live(
            &step_live(
                &$ent.get_geometric_tolerance_with_datum_reference(),
                "WriteStep",
            )?
            .datum_system_ap242(),
            "WriteStep",
        )?
        .array1()
        .length()
        {
            let $var0: StepDimTolDatumSystemOrReference = step_array1_item(
                &step_live(
                    &$ent.get_geometric_tolerance_with_datum_reference(),
                    "WriteStep",
                )?
                .datum_system_ap242(),
                $i4,
                "WriteStep",
            )?;
            $sw.send_entity(&$var0.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0128;

macro_rules! rw_block_b_0129 {
    ($iter:ident, $the_ent:ident) => {
        rw_block_b_0176!($iter, $the_ent);
        $iter.add_item(&$the_ent.pair_curve().cast());
    };
}
pub(crate) use rw_block_b_0129;

macro_rules! rw_block_b_0130 {
    ($ent:ident, $i1:ident, $sw:ident, $var0:ident) => {
        $sw.send_entity(&$ent.parent.assigned_action())?;
        $sw.open_sub()?;
        for $i1 in 1..=step_live(&$ent.items(), "WriteStep")?.array1().length() {
            let $var0: StepAP203WorkItem = step_array1_item(&$ent.items(), $i1, "WriteStep")?;
            $sw.send_entity(&$var0.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0130;

macro_rules! rw_block_b_0131 {
    ($i1:ident, $the_ent:ident, $the_sw:ident, $var0:ident) => {
        $the_sw.send_entity(&$the_ent.name())?;
        $the_sw.open_sub()?;
        for $i1 in 1..=step_live(&$the_ent.items(), "WriteStep")?.array1().length() {
            let $var0: Handle<StepVisualTessellatedStructuredItem> =
                step_array1_item(&$the_ent.items(), $i1, "WriteStep")?;
            $the_sw.send_entity(&$var0)?;
        }
        $the_sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0131;

macro_rules! rw_block_b_0132 {
    ($a_curve_element_interval_eu_angles:ident, $a_curve_element_interval_finish_position:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_curve_element_interval_finish_position: Handle<StepFEACurveElementLocation> =
            Default::default();
        $data.read_entity(
            $num,
            1,
            "curve_element_interval.finish_position",
            $ach,
            StepFEACurveElementLocation::static_type(),
            &mut $a_curve_element_interval_finish_position,
        )?;
        let mut $a_curve_element_interval_eu_angles: Handle<StepBasicEulerAngles> =
            Default::default();
        $data.read_entity(
            $num,
            2,
            "curve_element_interval.eu_angles",
            $ach,
            StepBasicEulerAngles::static_type(),
            &mut $a_curve_element_interval_eu_angles,
        )?;
    };
}
pub(crate) use rw_block_b_0132;

macro_rules! rw_block_b_0133 {
    ($a_assigned_date_and_time:ident, $a_role:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_assigned_date_and_time: Handle<StepBasicDateAndTime> = Default::default();
        $data.read_entity(
            $num,
            1,
            "assigned_date_and_time",
            $ach,
            StepBasicDateAndTime::static_type(),
            &mut $a_assigned_date_and_time,
        )?;
        let mut $a_role: Handle<StepBasicDateTimeRole> = Default::default();
        $data.read_entity(
            $num,
            2,
            "role",
            $ach,
            StepBasicDateTimeRole::static_type(),
            &mut $a_role,
        )?;
    };
}
pub(crate) use rw_block_b_0133;

macro_rules! rw_block_b_0134 {
    ($a_assigned_date:ident, $a_role:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_assigned_date: Handle<StepBasicDate> = Default::default();
        $data.read_entity(
            $num,
            1,
            "assigned_date",
            $ach,
            StepBasicDate::static_type(),
            &mut $a_assigned_date,
        )?;
        let mut $a_role: Handle<StepBasicDateRole> = Default::default();
        $data.read_entity(
            $num,
            2,
            "role",
            $ach,
            StepBasicDateRole::static_type(),
            &mut $a_role,
        )?;
    };
}
pub(crate) use rw_block_b_0134;

macro_rules! rw_block_b_0135 {
($a_assigned_organization:ident, $a_role:ident, $ach:ident, $data:ident, $num:ident) => {
let mut $a_assigned_organization: Handle<StepBasicOrganization> = Default::default();
$data.read_entity($num, 1, "assigned_organization", $ach, StepBasicOrganization::static_type(), &mut $a_assigned_organization)?;
let mut $a_role: Handle<StepBasicOrganizationRole> = Default::default();
$data.read_entity($num, 2, "role", $ach, StepBasicOrganizationRole::static_type(), &mut $a_role)?;
};
}
pub(crate) use rw_block_b_0135;
