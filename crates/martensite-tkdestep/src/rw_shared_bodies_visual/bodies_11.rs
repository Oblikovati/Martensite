// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0082 {
    ($a_action_assignment_assigned_action:ident, $a_items:ident, $ach:ident, $an_it0:ident, $data:ident, $ent:ident, $i0:ident, $nb0:ident, $num:ident, $num2:ident, $sub2:ident) => {
        let mut $a_action_assignment_assigned_action: Handle<StepBasicAction> = Default::default();
        $data.read_entity(
            $num,
            1,
            "action_assignment.assigned_action",
            $ach,
            StepBasicAction::static_type(),
            &mut $a_action_assignment_assigned_action,
        )?;
        let mut $a_items: Handle<HArray1<StepAP203WorkItem>> = Default::default();
        let mut $sub2: i32 = 0;
        if $data.read_sub_list($num, 2, "items", $ach, &mut $sub2, false, 0, 0)? {
            let $num2: i32 = $sub2;
            let $nb0: i32 = $data.nb_params($num2)?;
            $a_items = step_new_array1::<StepAP203WorkItem>(1, $nb0)?;
            for $i0 in 1..=$nb0 {
                let mut $an_it0: StepAP203WorkItem = Default::default();
                $data.read_entity_select($num2, $i0, "items", $ach, &mut $an_it0)?;
                step_array1_set(&$a_items, $i0, $an_it0.clone(), "ReadStep")?;
            }
        }
        $ent.init(&$a_action_assignment_assigned_action, &$a_items);
    };
}
pub(crate) use rw_block_b_0082;

macro_rules! rw_block_b_0083 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0210!($the_ent, $the_sw);
        rw_block_b_0127!($the_ent, $the_sw);
    };
}
pub(crate) use rw_block_b_0083;

macro_rules! rw_block_b_0084 {
    ($the_ent:ident, $the_sw:ident) => {
        $the_sw.send_entity(&$the_ent.name())?;
        rw_block_b_0125!($the_ent, $the_sw);
        $the_sw.send_entity(&$the_ent.joint())?;
    };
}
pub(crate) use rw_block_b_0084;

macro_rules! rw_block_b_0085 {
    ($i3:ident, $the_ent:ident, $the_sw:ident, $var0:ident) => {
        $the_sw.send_entity(&$the_ent.name())?;
        $the_sw.send_entity(&$the_ent.coordinates())?;
        if $the_ent.has_geometric_link() {
            $the_sw.send_entity(&$the_ent.geometric_link().value())?;
        } else {
            $the_sw.send_undef()?;
        }
        $the_sw.open_sub()?;
        for $i3 in 1..=step_live(&$the_ent.line_strip(), "WriteStep")?
            .array1()
            .length()
        {
            let $var0: i32 = step_array1_item(&$the_ent.line_strip(), $i3, "WriteStep")?;
            $the_sw.send_integer($var0)?;
        }
        $the_sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0085;

macro_rules! rw_block_b_0086 {
    ($ent:ident, $i1:ident, $sw:ident, $var0:ident) => {
        $sw.send_entity(&$ent.parent.parent.parent.name())?;
        $sw.open_sub()?;
        for $i1 in 1..=$ent.parent.parent.parent.nb_items() {
            let $var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&$ent.parent.parent.parent.items(), $i1, "WriteStep")?;
            $sw.send_entity(&$var0)?;
        }
        $sw.close_sub()?;
        $sw.send_entity(&$ent.parent.parent.parent.context_of_items())?;
        $sw.send_entity(&$ent.parent.parent.model_ref())?;
    };
}
pub(crate) use rw_block_b_0086;

macro_rules! rw_block_b_0087 {
    ($a_name:ident, $a_perspective_of_volume:ident, $a_view_reference_system:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_view_reference_system: Handle<StepGeomAxis2Placement3d> = Default::default();
        $data.read_entity(
            $num,
            2,
            "view_reference_system",
            $ach,
            StepGeomAxis2Placement3d::static_type(),
            &mut $a_view_reference_system,
        )?;
        let mut $a_perspective_of_volume: Handle<StepVisualViewVolume> = Default::default();
        $data.read_entity(
            $num,
            3,
            "perspective_of_volume",
            $ach,
            StepVisualViewVolume::static_type(),
            &mut $a_perspective_of_volume,
        )?;
    };
}
pub(crate) use rw_block_b_0087;

macro_rules! rw_block_b_0088 {
    ($a_edge_edge_end:ident, $a_edge_edge_start:ident, $a_representation_item_name:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            1,
            "representation_item.name",
            $the_arch,
            &mut $a_representation_item_name,
        )?;
        let mut $a_edge_edge_start: Handle<StepShapeVertex> = Default::default();
        $the_data.read_entity(
            $the_num,
            2,
            "edge.edge_start",
            $the_arch,
            StepShapeVertex::static_type(),
            &mut $a_edge_edge_start,
        )?;
        let mut $a_edge_edge_end: Handle<StepShapeVertex> = Default::default();
        $the_data.read_entity(
            $the_num,
            3,
            "edge.edge_end",
            $the_arch,
            StepShapeVertex::static_type(),
            &mut $a_edge_edge_end,
        )?;
    };
}
pub(crate) use rw_block_b_0088;

macro_rules! rw_block_b_0089 {
    ($a_actual_point_on_surface:ident, $a_pair_value_applies_to_pair:ident, $a_representation_item_name:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0139!(
            $a_pair_value_applies_to_pair,
            $a_representation_item_name,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_actual_point_on_surface: Handle<StepGeomPointOnSurface> = Default::default();
        $the_data.read_entity(
            $the_num,
            3,
            "actual_point_on_surface",
            $the_arch,
            StepGeomPointOnSurface::static_type(),
            &mut $a_actual_point_on_surface,
        )?;
    };
}
pub(crate) use rw_block_b_0089;

macro_rules! rw_block_b_0090 {
    ($a_actual_point_on_curve1:ident, $a_pair_value_applies_to_pair:ident, $a_representation_item_name:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0139!(
            $a_pair_value_applies_to_pair,
            $a_representation_item_name,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_actual_point_on_curve1: Handle<StepGeomPointOnCurve> = Default::default();
        $the_data.read_entity(
            $the_num,
            3,
            "actual_point_on_curve1",
            $the_arch,
            StepGeomPointOnCurve::static_type(),
            &mut $a_actual_point_on_curve1,
        )?;
    };
}
pub(crate) use rw_block_b_0090;

macro_rules! rw_block_b_0091 {
    ($ent:ident, $i1:ident, $sw:ident, $var0:ident) => {
        rw_block_b_0104!($ent, $i1, $sw, $var0);
        $sw.send_entity(&$ent.parent.model_ref())?;
    };
}
pub(crate) use rw_block_b_0091;

macro_rules! rw_block_b_0092 {
    ($i1:ident, $the_ent:ident, $the_sw:ident, $var0:ident) => {
        rw_block_b_0105!($i1, $the_ent, $the_sw, $var0);
        $the_sw.send_entity(&$the_ent.parent())?;
    };
}
pub(crate) use rw_block_b_0092;

macro_rules! rw_block_b_0093 {
    ($i1:ident, $the_ent:ident, $the_sw:ident, $var0:ident) => {
        rw_block_b_0105!($i1, $the_ent, $the_sw, $var0);
        $the_sw.send_entity(&$the_ent.represented_link())?;
    };
}
pub(crate) use rw_block_b_0093;

macro_rules! rw_block_b_0094 {
    ($a_fea_group_model_ref:ident, $a_group_description:ident, $a_group_name:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_b_0149!($a_group_description, $a_group_name, $ach, $data, $num);
        let mut $a_fea_group_model_ref: Handle<StepFEAFeaModel> = Default::default();
        $data.read_entity(
            $num,
            3,
            "fea_group.model_ref",
            $ach,
            StepFEAFeaModel::static_type(),
            &mut $a_fea_group_model_ref,
        )?;
    };
}
pub(crate) use rw_block_b_0094;

macro_rules! rw_block_b_0095 {
    ($a_actual_translation:ident, $a_pair_value_applies_to_pair:ident, $a_representation_item_name:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0139!(
            $a_pair_value_applies_to_pair,
            $a_representation_item_name,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_actual_translation: f64 = Default::default();
        $the_data.read_real(
            $the_num,
            3,
            "actual_translation",
            $the_arch,
            &mut $a_actual_translation,
        )?;
    };
}
pub(crate) use rw_block_b_0095;

macro_rules! rw_block_b_0096 {
    ($a_actual_rotation:ident, $a_pair_value_applies_to_pair:ident, $a_representation_item_name:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0139!(
            $a_pair_value_applies_to_pair,
            $a_representation_item_name,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_actual_rotation: f64 = Default::default();
        $the_data.read_real(
            $the_num,
            3,
            "actual_rotation",
            $the_arch,
            &mut $a_actual_rotation,
        )?;
    };
}
pub(crate) use rw_block_b_0096;
