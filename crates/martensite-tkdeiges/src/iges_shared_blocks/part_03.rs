//! Statement runs repeated by several tools.
macro_rules! iges_block_0056 {
    ($end_list:ident, $notes_list:ident, $start_list:ident) => {
        let mut $end_list: Handle<HArray1<i32>> = Default::default();
        let mut $start_list: Handle<HArray1<i32>> = Default::default();
        let mut $notes_list: Handle<HArray1<i32>> = Default::default();
    };
}
pub(crate) use iges_block_0056;

macro_rules! iges_block_0057 {
    ($box_heights:ident, $box_widths:ident, $nb_chars:ident) => {
        let mut $nb_chars: Handle<HArray1<i32>> = Default::default();
        let mut $box_widths: Handle<HArray1<f64>> = Default::default();
        let mut $box_heights: Handle<HArray1<f64>> = Default::default();
    };
}
pub(crate) use iges_block_0057;

macro_rules! iges_block_0058 {
    ($ent:ident, $pr:ident, $self:ident, $temp_frac_flag:ident, $temp_precision:ident) => {
        $pr.read_integer_text(&$pr.current(), "Fraction Flag", &mut $temp_frac_flag)?;
        $pr.read_integer_text(&$pr.current(), "Precision", &mut $temp_precision)?;
        $self
            .dir_checker($ent)
            .check_type_and_form($pr.c_check(), $ent);
    };
}
pub(crate) use iges_block_0058;

macro_rules! iges_block_0059 {
    ($leaders:ident, $nbval:ident, $note:ident) => {
        let mut $note: Handle<IgesDimenGeneralNote> = Default::default();
        let mut $nbval: i32 = Default::default();
        let mut $leaders: Handle<HArray1<Handle<IgesDimenLeaderArrow>>> = Default::default();
    };
}
pub(crate) use iges_block_0059;

macro_rules! iges_block_0060 {
    ($dumper:ident, $ent:ident, $s:ident, $sublevel:ident) => {
        $dumper.dump(&$ent.first_witness_line(), $s, $sublevel, -1)?;
        write!($s, "\nSecond Witness Entity : ")?;
        $dumper.dump(&$ent.second_witness_line(), $s, $sublevel, -1)?;
    };
}
pub(crate) use iges_block_0060;

macro_rules! iges_block_0061 {
    ($ent:ident, $iter:ident) => {
        $iter.get_one_item(&$ent.note().cast());
        $iter.get_one_item(&$ent.first_leader().cast());
        $iter.get_one_item(&$ent.second_leader().cast());
    };
}
pub(crate) use iges_block_0061;

macro_rules! iges_block_0062 {
    ($ent:ident, $iw:ident) => {
        $iw.send($ent.lower_left_corner().x())?;
        $iw.send($ent.lower_left_corner().y())?;
        $iw.send($ent.lower_left_corner().z())?;
    };
}
pub(crate) use iges_block_0062;

macro_rules! iges_block_0063 {
    ($dumper:ident, $ent:ident, $level:ident, $s:ident, $sublevel:ident) => {
        let mut $sublevel: i32 = if $level <= 4 { 0 } else { 1 };
        write!($s, "General Note : ")?;
        $dumper.dump(&$ent.note(), $s, $sublevel, -1)?;
    };
}
pub(crate) use iges_block_0063;

macro_rules! iges_block_0064 {
    ($ent:ident, $iw:ident) => {
        $iw.send_entity(&$ent.note(), false)?;
        $iw.send_entity(&$ent.first_leader(), false)?;
        $iw.send_entity(&$ent.second_leader(), false)?;
    };
}
pub(crate) use iges_block_0064;

macro_rules! iges_block_0065 {
    ($box_heights:ident, $box_widths:ident, $nb_chars:ident, $nbval:ident) => {
        $nb_chars = iges_new_array1::<i32>(1, $nbval)?;
        $box_widths = iges_new_array1::<f64>(1, $nbval)?;
        $box_heights = iges_new_array1::<f64>(1, $nbval)?;
    };
}
pub(crate) use iges_block_0065;

macro_rules! iges_block_0066 {
    ($annotations:ident, $ent:ident, $ir:ident, $nbval:ident, $pr:ident, $self:ident) => {
        if $pr.read_integer_text(
            &$pr.current(),
            "Count of array of Annotation entities",
            &mut $nbval,
        )? {
            if $nbval > 0 {
                $pr.read_ents(
                    $ir,
                    &$pr.current_list($nbval, 1),
                    "Annotation Entities",
                    &mut $annotations,
                    1,
                )?;
            } else {
                if $nbval < 0 {
                    $pr.add_fail("Count of Annotation entities : Less than zero", "");
                }
            }
        }
        $self
            .dir_checker($ent)
            .check_type_and_form($pr.c_check(), $ent);
    };
}
pub(crate) use iges_block_0066;

macro_rules! iges_block_0067 {
    ($ent:ident, $first:ident, $level:ident, $point:ident, $s:ident) => {
        iges_dump_list_xy_lz(
            $s,
            $level,
            1,
            $ent.nb_points()?,
            |$first| {
                Ok({
                    let $point = $ent.point($first)?;
                    Xy::new($point.x(), $point.y())
                })
            },
            &$ent.location()?,
            $ent.z_displacement(),
        )?;
        writeln!($s)?;
    };
}
pub(crate) use iges_block_0067;

macro_rules! iges_block_0068 {
    ($first_leader:ident, $ir:ident, $pr:ident, $second_leader:ident) => {
        $pr.read_entity_typed(
            $ir,
            &$pr.current(),
            "First Leader Entity",
            <IgesDimenLeaderArrow>::static_type(),
            &mut $first_leader,
            false,
        )?;
        $pr.read_entity_typed(
            $ir,
            &$pr.current(),
            "Second Leader Entity",
            <IgesDimenLeaderArrow>::static_type(),
            &mut $second_leader,
            false,
        )?;
    };
}
pub(crate) use iges_block_0068;

macro_rules! iges_block_0069 {
    ($another:ident, $first_leader:ident, $second_leader:ident, $tc:ident) => {
        let mut $first_leader: Handle<IgesDimenLeaderArrow> =
            $tc.transferred(&$another.first_leader().cast())?.cast();
        let mut $second_leader: Handle<IgesDimenLeaderArrow> =
            $tc.transferred(&$another.second_leader().cast())?.cast();
    };
}
pub(crate) use iges_block_0069;

macro_rules! iges_block_0070 {
    ($ir:ident, $nbval:ident, $note:ident, $pr:ident, $st:ident) => {
        $pr.read_entity_typed(
            $ir,
            &$pr.current(),
            "General Note Entity",
            <IgesDimenGeneralNote>::static_type(),
            &mut $note,
            false,
        )?;
        let mut $st: bool =
            $pr.read_integer_text(&$pr.current(), "Number of Leaders", &mut $nbval)?;
    };
}
pub(crate) use iges_block_0070;

macro_rules! iges_block_0071 {
    ($a_node_identifiers:ident, $a_nodes:ident, $nbnodes:ident) => {
        let mut $a_node_identifiers: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, $nbnodes)?;
        let mut $a_nodes: Handle<HArray1<Handle<IgesAppliNode>>> =
            iges_new_array1::<Handle<IgesAppliNode>>(1, $nbnodes)?;
    };
}
pub(crate) use iges_block_0071;

macro_rules! iges_block_0072 {
    ($area_loc:ident, $area_rotation_angle:ident, $base_line_pos:ident, $box_heights:ident, $box_widths:ident, $char_angles:ident, $char_displays:ident, $char_heights:ident, $char_set_codes:ident, $char_set_entities:ident, $char_widths:ident, $control_code_strings:ident, $ent:ident, $font_styles:ident, $height:ident, $inter_char_spaces:ident, $interline_spaces:ident, $justify_code:ident, $mirror_flags:ident, $nb_chars:ident, $normal_interline_space:ident, $rotate_flags:ident, $rotation_angles:ident, $slant_angles:ident, $start_points:ident, $texts:ident, $width:ident) => {
        $ent.init(
            $width,
            $height,
            $justify_code,
            $area_loc,
            $area_rotation_angle,
            $base_line_pos,
            $normal_interline_space,
            &$char_displays,
            &$char_widths,
            &$char_heights,
            &$inter_char_spaces,
            &$interline_spaces,
            &$font_styles,
            &$char_angles,
            &$control_code_strings,
            &$nb_chars,
            &$box_widths,
            &$box_heights,
            &$char_set_codes,
            &$char_set_entities,
            &$slant_angles,
            &$rotation_angles,
            &$mirror_flags,
            &$rotate_flags,
            &$start_points,
            &$texts,
        )?;
    };
}
pub(crate) use iges_block_0072;

macro_rules! iges_block_0073 {
    ($ach:ident, $ent:ident) => {
        if $ent.subordinate_status() != 0 {
            if ($ent.def_level() != (IgesDataDefList::DefOne))
                && ($ent.def_level() != (IgesDataDefList::DefSeveral))
            {
                $ach.add_fail_text("Level type: Not value/reference", "");
            }
        }
    };
}
pub(crate) use iges_block_0073;

macro_rules! iges_block_0074 {
    ($box_heights:ident, $box_widths:ident, $char_angles:ident, $char_displays:ident, $char_heights:ident, $char_set_codes:ident, $char_set_entities:ident, $char_widths:ident, $control_code_strings:ident, $font_styles:ident, $inter_char_spaces:ident, $interline_spaces:ident, $mirror_flags:ident, $nb_chars:ident, $rotate_flags:ident, $rotation_angles:ident, $slant_angles:ident, $start_points:ident, $texts:ident) => {
        crate::iges_shared_blocks::iges_block_0000!(
            $box_heights,
            $box_widths,
            $char_angles,
            $char_displays,
            $char_heights,
            $char_set_codes,
            $char_set_entities,
            $char_widths,
            $control_code_strings,
            $font_styles,
            $inter_char_spaces,
            $interline_spaces,
            $mirror_flags,
            $nb_chars,
            $rotate_flags,
            $rotation_angles,
            $slant_angles,
            $start_points,
            $texts
        );
    };
}
pub(crate) use iges_block_0074;
