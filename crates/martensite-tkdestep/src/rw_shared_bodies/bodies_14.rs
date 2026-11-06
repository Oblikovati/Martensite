// The RW member bodies several classes repeat.

macro_rules! rw_block_0138 {
    ($a_name:ident, $a_swept_face:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_swept_face: Handle<StepShapeFaceSurface> = Default::default();
        $data.read_entity(
            $num,
            2,
            "swept_face",
            $ach,
            StepShapeFaceSurface::static_type(),
            &mut $a_swept_face,
        )?;
    };
}
pub(crate) use rw_block_0138;

macro_rules! rw_block_0139 {
    ($a_name:ident, $a_position:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_position: Handle<StepGeomAxis1Placement> = Default::default();
        $data.read_entity(
            $num,
            2,
            "position",
            $ach,
            StepGeomAxis1Placement::static_type(),
            &mut $a_position,
        )?;
    };
}
pub(crate) use rw_block_0139;

macro_rules! rw_block_0140 {
    ($a_name:ident, $a_position:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_position: Handle<StepGeomAxis2Placement3d> = Default::default();
        $data.read_entity(
            $num,
            2,
            "position",
            $ach,
            StepGeomAxis2Placement3d::static_type(),
            &mut $a_position,
        )?;
    };
}
pub(crate) use rw_block_0140;

macro_rules! rw_block_0141 {
    ($a_description:ident, $a_name:ident, $the_ach:ident, $the_data:ident, $the_num:ident) => {
        if !$the_data.check_nb_params($the_num, 4, $the_ach, "general_property_association")? {
            return Ok(());
        }
        rw_block_0195!($a_description, $a_name, $the_ach, $the_data, $the_num);
    };
}
pub(crate) use rw_block_0141;

macro_rules! rw_block_0142 {
    ($a_property_definition_representation_definition:ident, $a_property_definition_representation_used_representation:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_property_definition_representation_definition: StepReprRepresentedDefinition =
            Default::default();
        $data.read_entity_select(
            $num,
            1,
            "property_definition_representation.definition",
            $ach,
            &mut $a_property_definition_representation_definition,
        )?;
        let mut $a_property_definition_representation_used_representation: Handle<
            StepReprRepresentation,
        > = Default::default();
        $data.read_entity(
            $num,
            2,
            "property_definition_representation.used_representation",
            $ach,
            StepReprRepresentation::static_type(),
            &mut $a_property_definition_representation_used_representation,
        )?;
    };
}
pub(crate) use rw_block_0142;

macro_rules! rw_block_0143 {
    ($a_unit_component:ident, $ach:ident, $data:ident, $mvc:ident, $num:ident) => {
        let mut $mvc: Handle<StepBasicMeasureValueMember> =
            Handle::new(StepBasicMeasureValueMember::new());
        $data.read_member_typed($num, 1, "value_component", $ach, &mut $mvc)?;
        let mut $a_unit_component: StepBasicUnit = Default::default();
        $data.read_entity_select($num, 2, "unit_component", $ach, &mut $a_unit_component)?;
    };
}
pub(crate) use rw_block_0143;

macro_rules! rw_block_0144 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        if $ent.has_description() {
            $sw.send_entity(&$ent.description())?;
        } else {
            $sw.send_undef()?;
        }
        $sw.send_entity(&$ent.relating_shape_aspect())?;
        $sw.send_entity(&$ent.related_shape_aspect())?;
    };
}
pub(crate) use rw_block_0144;

macro_rules! rw_block_0145 {
    ($ent:ident, $has_aprefix:ident, $sw:ident) => {
        step_start_entity($sw, "SI_UNIT")?;
        rw_block_0174!($ent, $has_aprefix, $sw);
    };
}
pub(crate) use rw_block_0145;

macro_rules! rw_block_0146 {
    ($a_weights_data:ident, $a_weights_data_item:ident, $ach:ident, $data:ident, $nsub8:ident, $num:ident) => {
        rw_block_0181!($a_weights_data, $a_weights_data_item, $ach, $data, $num);
        let mut $nsub8: i32 = Default::default();
    };
}
pub(crate) use rw_block_0146;

macro_rules! rw_block_0147 {
    ($a_name:ident, $a_purpose:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_purpose: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 2, "purpose", $ach, &mut $a_purpose)?;
    };
}
pub(crate) use rw_block_0147;

macro_rules! rw_block_0148 {
    ($a_description:ident, $a_id:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0200!($a_description, $a_id, $ach, $data, $num);
        $data.read_string($num, 2, "description", $ach, &mut $a_description)?;
    };
}
pub(crate) use rw_block_0148;

macro_rules! rw_block_0149 {
    ($a_description:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0199!($a_description, $a_name, $ach, $data, $num);
        $data.read_string($num, 2, "description", $ach, &mut $a_description)?;
    };
}
pub(crate) use rw_block_0149;

macro_rules! rw_block_0150 {
    ($a_context_identifier:ident, $a_context_type:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_context_identifier: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "context_identifier",
            $ach,
            &mut $a_context_identifier,
        )?;
        let mut $a_context_type: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 2, "context_type", $ach, &mut $a_context_type)?;
    };
}
pub(crate) use rw_block_0150;

macro_rules! rw_block_0151 {
    ($ent:ident, $sw:ident) => {
        step_start_entity($sw, "MEASURE_REPRESENTATION_ITEM")?;
        step_start_entity($sw, "MEASURE_WITH_UNIT")?;
        $sw.send_entity(
            &step_live(&$ent.get_measure_with_unit(), "WriteStep")?.value_component_member(),
        )?;
        $sw.send_entity(
            &step_live(&$ent.get_measure_with_unit(), "WriteStep")?
                .unit_component()
                .value(),
        )?;
    };
}
pub(crate) use rw_block_0151;

macro_rules! rw_block_0152 {
    ($a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        $num = $data.next_for_complex($num);
        rw_block_0176!($a_name, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0152;

macro_rules! rw_block_0153 {
    ($a_name:ident, $a_position:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_position: StepGeomAxis2Placement = Default::default();
        $data.read_entity_select($num, 2, "position", $ach, &mut $a_position)?;
    };
}
pub(crate) use rw_block_0153;
