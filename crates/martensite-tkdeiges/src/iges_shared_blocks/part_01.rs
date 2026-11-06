//! Statement runs repeated by several tools.
macro_rules! iges_block_0000 {
    ($box_heights:ident, $box_widths:ident, $char_angles:ident, $char_displays:ident, $char_heights:ident, $char_set_codes:ident, $char_set_entities:ident, $char_widths:ident, $control_code_strings:ident, $font_styles:ident, $inter_char_spaces:ident, $interline_spaces:ident, $mirror_flags:ident, $nb_chars:ident, $rotate_flags:ident, $rotation_angles:ident, $slant_angles:ident, $start_points:ident, $texts:ident) => {
        let mut $char_displays: Handle<HArray1<i32>> = Default::default();
        let mut $char_widths: Handle<HArray1<f64>> = Default::default();
        let mut $char_heights: Handle<HArray1<f64>> = Default::default();
        let mut $inter_char_spaces: Handle<HArray1<f64>> = Default::default();
        let mut $interline_spaces: Handle<HArray1<f64>> = Default::default();
        let mut $font_styles: Handle<HArray1<i32>> = Default::default();
        let mut $char_angles: Handle<HArray1<f64>> = Default::default();
        let mut $control_code_strings: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
        crate::iges_shared_blocks::iges_block_0057!($box_heights, $box_widths, $nb_chars);
        let mut $char_set_codes: Handle<HArray1<i32>> = Default::default();
        let mut $char_set_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            Default::default();
        crate::iges_shared_blocks::iges_block_0021!(
            $mirror_flags,
            $rotate_flags,
            $rotation_angles,
            $slant_angles,
            $start_points,
            $texts
        );
    };
}
pub(crate) use iges_block_0000;

macro_rules! iges_block_0001 {
    ($box_heights:ident, $box_widths:ident, $font_codes:ident, $font_entities:ident, $mirror_flags:ident, $nb_chars:ident, $rotate_flags:ident, $rotation_angles:ident, $slant_angles:ident, $start_points:ident, $texts:ident) => {
        crate::iges_shared_blocks::iges_block_0057!($box_heights, $box_widths, $nb_chars);
        let mut $font_codes: Handle<HArray1<i32>> = Default::default();
        let mut $font_entities: Handle<HArray1<Handle<IgesGraphTextFontDef>>> = Default::default();
        crate::iges_shared_blocks::iges_block_0021!(
            $mirror_flags,
            $rotate_flags,
            $rotation_angles,
            $slant_angles,
            $start_points,
            $texts
        );
    };
}
pub(crate) use iges_block_0001;

macro_rules! iges_block_0002 {
    ($anew:ident, $another:ident, $i:ident, $newdisp:ident, $tc:ident, $temp_display_entities:ident, $up:ident) => {
        let mut $newdisp: InterfaceEntityIterator = Default::default();
        let mut $i: i32 = Default::default();
        let mut $up: i32 = Default::default();
        $up = $another.nb_displayed_entities()?;
        if $up == 0 {
            return Ok(());
        }
        let mut $temp_display_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            Default::default();
        let mut $anew: Handle<StandardTransient> = Default::default();
        {
            $i = 1;
            while $i <= $up {
                if $tc.search(&$another.displayed_entity($i)?.cast(), &mut $anew)? {
                    $newdisp.get_one_item(&$anew.cast());
                }
                $i += 1;
            }
        }
        $up = $newdisp.nb_entities();
        $i = 0;
        if $up > 0 {
            $temp_display_entities = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, $up)?;
        }
    };
}
pub(crate) use iges_block_0002;

macro_rules! iges_block_0003 {
    ($ent:ident, $i:ident, $iw:ident, $num:ident) => {
        let mut $i: i32 = Default::default();
        let mut $num: i32 = Default::default();
        $iw.send($ent.nb_context_flags())?;
        $iw.send($ent.nb_flow_associativities()?)?;
        $iw.send($ent.nb_connect_points()?)?;
        $iw.send($ent.nb_joins()?)?;
        $iw.send($ent.nb_flow_names()?)?;
        $iw.send($ent.nb_text_display_templates()?)?;
        $iw.send($ent.nb_cont_flow_associativities()?)?;
        $iw.send($ent.type_of_flow())?;
    };
}
pub(crate) use iges_block_0003;

macro_rules! iges_block_0004 {
    ($breakpoint_parameters:ident, $color_definitions:ident, $color_values:ident, $display_flags:ident, $line_font_definitions:ident, $line_font_values:ident, $line_weights:ident, $nbval:ident, $views:ident) => {
        let mut $nbval: i32 = Default::default();
        let mut $views: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> = Default::default();
        let mut $breakpoint_parameters: Handle<HArray1<f64>> = Default::default();
        let mut $display_flags: Handle<HArray1<i32>> = Default::default();
        let mut $color_values: Handle<HArray1<i32>> = Default::default();
        let mut $color_definitions: Handle<HArray1<Handle<IgesGraphColor>>> = Default::default();
        let mut $line_font_values: Handle<HArray1<i32>> = Default::default();
        let mut $line_font_definitions: Handle<HArray1<Handle<dyn IgesDataLineFontEntity>>> =
            Default::default();
        let mut $line_weights: Handle<HArray1<i32>> = Default::default();
    };
}
pub(crate) use iges_block_0004;

macro_rules! iges_block_0005 {
    ($a_scii_codes:ident, $move_pen_x:ident, $move_pen_y:ident, $next_char_x:ident, $next_char_y:ident, $pen_flags:ident, $pen_motions:ident, $scale:ident, $superseded_entity:ident) => {
        let mut $superseded_entity: Handle<IgesGraphTextFontDef> = Default::default();
        let mut $scale: i32 = Default::default();
        let mut $a_scii_codes: Handle<HArray1<i32>> = Default::default();
        let mut $next_char_x: Handle<HArray1<i32>> = Default::default();
        let mut $next_char_y: Handle<HArray1<i32>> = Default::default();
        let mut $pen_motions: Handle<HArray1<i32>> = Default::default();
        let mut $pen_flags: Handle<IgesBasicHArray1OfHArray1OfInteger> = Default::default();
        let mut $move_pen_x: Handle<IgesBasicHArray1OfHArray1OfInteger> = Default::default();
        let mut $move_pen_y: Handle<IgesBasicHArray1OfHArray1OfInteger> = Default::default();
    };
}
pub(crate) use iges_block_0005;

macro_rules! iges_block_0006 {
    ($dc:ident) => {
        crate::iges_shared_blocks::iges_block_0024!($dc);
        $dc.subordinate_status_required(1);
        $dc.use_flag_required(2);
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_block_0006;

macro_rules! iges_block_0007 {
    ($dc:ident) => {
        crate::iges_shared_blocks::iges_block_0024!($dc);
        $dc.subordinate_status_required(2);
        $dc.use_flag_required(2);
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_block_0007;

macro_rules! iges_block_0008 {
    ($data_points:ident, $datatype:ident, $nbval:ident, $pr:ident, $st:ident, $z_displacement:ident) => {
        let mut $datatype: i32 = Default::default();
        let mut $z_displacement: f64 = Default::default();
        let mut $nbval: i32 = Default::default();
        let mut $data_points: Handle<HArray1<Xy>> = Default::default();
        $pr.read_integer_text(&$pr.current(), "Interpretation Flag", &mut $datatype)?;
        let mut $st: bool =
            $pr.read_integer_text(&$pr.current(), "Number of data points", &mut $nbval)?;
        if $st && ($nbval > 0) {
            $data_points = iges_new_array1::<Xy>(1, $nbval)?;
        } else {
            $pr.add_fail("Number of data points: Not Positive", "");
        }
        $pr.read_real_text(
            &$pr.current(),
            "Common Z Displacement",
            &mut $z_displacement,
        )?;
    };
}
pub(crate) use iges_block_0008;

macro_rules! iges_block_0009 {
    ($all_data_layer_flag:ident, $all_element_identifiers:ident, $all_element_topology_type:ident, $all_elements:ident, $all_nb_layers:ident, $all_nb_result_data_locs:ident, $all_result_data:ident, $all_result_data_loc:ident) => {
        let mut $all_element_identifiers: Handle<HArray1<i32>> = Default::default();
        let mut $all_elements: Handle<HArray1<Handle<IgesAppliFiniteElement>>> = Default::default();
        let mut $all_element_topology_type: Handle<HArray1<i32>> = Default::default();
        let mut $all_nb_layers: Handle<HArray1<i32>> = Default::default();
        let mut $all_data_layer_flag: Handle<HArray1<i32>> = Default::default();
        let mut $all_nb_result_data_locs: Handle<HArray1<i32>> = Default::default();
        let mut $all_result_data_loc: Handle<IgesBasicHArray1OfHArray1OfInteger> =
            Default::default();
        let mut $all_result_data: Handle<IgesBasicHArray1OfHArray1OfReal> = Default::default();
    };
}
pub(crate) use iges_block_0009;

macro_rules! iges_block_0010 {
    ($dc:ident) => {
        crate::iges_shared_blocks::iges_block_0024!($dc);
        $dc.use_flag_required(3);
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_block_0010;

macro_rules! iges_block_0011 {
    ($dc:ident) => {
        crate::iges_shared_blocks::iges_block_0024!($dc);
        $dc.use_flag_ignored();
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_block_0011;

macro_rules! iges_block_0012 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.line_font(IgesDataDefType::DefValue);
        $dc.line_weight(IgesDataDefType::DefVoid);
        $dc.color(IgesDataDefType::DefVoid);
        $dc.blank_status_ignored();
        $dc.subordinate_status_required(0);
        $dc.use_flag_required(2);
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_block_0012;

macro_rules! iges_block_0013 {
    ($dc:ident) => {
        crate::iges_shared_blocks::iges_block_0026!($dc);
        $dc.use_flag_required(1);
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_block_0013;

macro_rules! iges_block_0014 {
    ($dc:ident) => {
        crate::iges_shared_blocks::iges_block_0026!($dc);
        $dc.use_flag_required(2);
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_block_0014;

macro_rules! iges_block_0015 {
    ($ent:ident, $i:ident, $iter:ident, $num:ident) => {
        let mut $i: i32 = Default::default();
        let mut $num: i32 = Default::default();
        {
            $num = $ent.nb_flow_associativities()?;
            $i = 1;
            while $i <= $num {
                $iter.get_one_item(&$ent.flow_associativity($i)?.cast());
                $i += 1;
            }
        }
        {
            $num = $ent.nb_connect_points()?;
            $i = 1;
            while $i <= $num {
                $iter.get_one_item(&$ent.connect_point($i)?.cast());
                $i += 1;
            }
        }
        {
            $num = $ent.nb_joins()?;
            $i = 1;
            while $i <= $num {
                $iter.get_one_item(&$ent.join($i)?.cast());
                $i += 1;
            }
        }
        {
            $num = $ent.nb_text_display_templates()?;
            $i = 1;
            while $i <= $num {
                $iter.get_one_item(&$ent.text_display_template($i)?.cast());
                $i += 1;
            }
        }
        {
            $num = $ent.nb_cont_flow_associativities()?;
            $i = 1;
            while $i <= $num {
                $iter.get_one_item(&$ent.cont_flow_associativity($i)?.cast());
                $i += 1;
            }
        }
    };
}
pub(crate) use iges_block_0015;

macro_rules! iges_block_0016 {
    ($dc:ident) => {
        crate::iges_shared_blocks::iges_block_0040!($dc);
        $dc.use_flag_required(1);
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_block_0016;

macro_rules! iges_block_0017 {
    ($dc:ident) => {
        crate::iges_shared_blocks::iges_block_0040!($dc);
        $dc.use_flag_ignored();
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_block_0017;

macro_rules! iges_block_0018 {
    ($ent:ident, $i:ident, $iw:ident, $num:ident) => {{
        $num = $ent.nb_flow_associativities()?;
        $i = 1;
        while $i <= $num {
            $iw.send_entity(&$ent.flow_associativity($i)?, false)?;
            $i += 1;
        }
    }
    {
        $num = $ent.nb_connect_points()?;
        $i = 1;
        while $i <= $num {
            $iw.send_entity(&$ent.connect_point($i)?, false)?;
            $i += 1;
        }
    }
    {
        $num = $ent.nb_joins()?;
        $i = 1;
        while $i <= $num {
            $iw.send_entity(&$ent.join($i)?, false)?;
            $i += 1;
        }
    }
    {
        $num = $ent.nb_flow_names()?;
        $i = 1;
        while $i <= $num {
            $iw.send($ent.flow_name($i)?)?;
            $i += 1;
        }
    }
    {
        $num = $ent.nb_text_display_templates()?;
        $i = 1;
        while $i <= $num {
            $iw.send_entity(&$ent.text_display_template($i)?, false)?;
            $i += 1;
        }
    }
    {
        $num = $ent.nb_cont_flow_associativities()?;
        $i = 1;
        while $i <= $num {
            $iw.send_entity(&$ent.cont_flow_associativity($i)?, false)?;
            $i += 1;
        }
    }};
}
pub(crate) use iges_block_0018;

macro_rules! iges_block_0019 {
    ($temp_connect_points:ident, $temp_cont_flow_assocs:ident, $temp_flow_assocs:ident, $temp_flow_names:ident, $temp_joins:ident, $temp_text_display_templates:ident) => {
        let mut $temp_flow_assocs: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
        let mut $temp_connect_points: Handle<HArray1<Handle<IgesDrawConnectPoint>>> =
            Default::default();
        let mut $temp_joins: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
        let mut $temp_flow_names: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
        let mut $temp_text_display_templates: Handle<
            HArray1<Handle<IgesGraphTextDisplayTemplate>>,
        > = Default::default();
        let mut $temp_cont_flow_assocs: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            Default::default();
    };
}
pub(crate) use iges_block_0019;

macro_rules! iges_block_0020 {
    ($displayed_entities:ident, $label_levels:ident, $leader_entities:ident, $nbval:ident, $text_locations:ident, $views:ident) => {
        let mut $nbval: i32 = Default::default();
        let mut $views: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> = Default::default();
        let mut $text_locations: Handle<HArray1<Xyz>> = Default::default();
        let mut $leader_entities: Handle<HArray1<Handle<IgesDimenLeaderArrow>>> =
            Default::default();
        let mut $label_levels: Handle<HArray1<i32>> = Default::default();
        let mut $displayed_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            Default::default();
    };
}
pub(crate) use iges_block_0020;

macro_rules! iges_block_0021 {
    ($mirror_flags:ident, $rotate_flags:ident, $rotation_angles:ident, $slant_angles:ident, $start_points:ident, $texts:ident) => {
        let mut $slant_angles: Handle<HArray1<f64>> = Default::default();
        let mut $rotation_angles: Handle<HArray1<f64>> = Default::default();
        let mut $mirror_flags: Handle<HArray1<i32>> = Default::default();
        let mut $rotate_flags: Handle<HArray1<i32>> = Default::default();
        let mut $start_points: Handle<HArray1<Xyz>> = Default::default();
        let mut $texts: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
    };
}
pub(crate) use iges_block_0021;

macro_rules! iges_block_0022 {
    ($ent:ident, $nb:ident, $nulfont:ident, $pts:ident, $res:ident) => {
        crate::iges_shared_blocks::iges_block_0042!($ent, $nb, $nulfont, $res);
        if $nb == 0 {
            return Ok(false);
        }
        let mut $pts: Handle<HArray1<Xy>> = iges_new_array1::<Xy>(1, $nb)?;
    };
}
pub(crate) use iges_block_0022;

macro_rules! iges_block_0023 {
    ($mirror_flags:ident, $nbval:ident, $rotate_flags:ident, $rotation_angles:ident, $slant_angles:ident, $start_points:ident, $texts:ident) => {
        $slant_angles = iges_new_array1::<f64>(1, $nbval)?;
        $rotation_angles = iges_new_array1::<f64>(1, $nbval)?;
        $mirror_flags = iges_new_array1::<i32>(1, $nbval)?;
        $rotate_flags = iges_new_array1::<i32>(1, $nbval)?;
        $start_points = iges_new_array1::<Xyz>(1, $nbval)?;
        $texts = iges_new_array1::<Handle<HAsciiString>>(1, $nbval)?;
    };
}
pub(crate) use iges_block_0023;

macro_rules! iges_block_0024 {
    ($dc:ident) => {
        crate::iges_shared_blocks::iges_block_0037!($dc);
        $dc.blank_status_ignored();
    };
}
pub(crate) use iges_block_0024;

macro_rules! iges_block_0025 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.line_font(IgesDataDefType::DefValue);
        $dc.line_weight(IgesDataDefType::DefValue);
        $dc.color(IgesDataDefType::DefAny);
        $dc.use_flag_required(1);
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_block_0025;

macro_rules! iges_block_0026 {
    ($dc:ident) => {
        crate::iges_shared_blocks::iges_block_0040!($dc);
        $dc.subordinate_status_required(0);
    };
}
pub(crate) use iges_block_0026;

macro_rules! iges_block_0027 {
    ($dc:ident) => {
        $dc.line_font(IgesDataDefType::DefVoid);
        $dc.line_weight(IgesDataDefType::DefVoid);
        $dc.color(IgesDataDefType::DefAny);
        $dc.blank_status_ignored();
        $dc.use_flag_required(3);
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_block_0027;
