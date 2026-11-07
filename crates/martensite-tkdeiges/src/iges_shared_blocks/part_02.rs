//! Statement runs repeated by several tools.
macro_rules! iges_block_0028 {
    ($ent:ident, $first_leader:ident, $first_witness:ident, $ir:ident, $pr:ident, $second_leader:ident, $second_witness:ident, $self:ident) => {
        crate::iges_shared_blocks::iges_block_0068!($first_leader, $ir, $pr, $second_leader);
        $pr.read_entity_typed(
            $ir,
            &$pr.current(),
            "First Witness Entity",
            <IgesDimenWitnessLine>::static_type(),
            &mut $first_witness,
            true,
        )?;
        $pr.read_entity_typed(
            $ir,
            &$pr.current(),
            "Second Witness Entity",
            <IgesDimenWitnessLine>::static_type(),
            &mut $second_witness,
            true,
        )?;
        $self
            .dir_checker($ent)
            .check_type_and_form($pr.c_check(), $ent);
    };
}
pub(crate) use iges_block_0028;

macro_rules! iges_block_0029 {
    ($first_leader:ident, $first_witness:ident, $ir:ident, $note:ident, $pr:ident, $second_leader:ident, $second_witness:ident) => {
        let mut $first_leader: Handle<IgesDimenLeaderArrow> = Default::default();
        let mut $second_leader: Handle<IgesDimenLeaderArrow> = Default::default();
        let mut $first_witness: Handle<IgesDimenWitnessLine> = Default::default();
        let mut $second_witness: Handle<IgesDimenWitnessLine> = Default::default();
        $pr.read_entity_typed(
            $ir,
            &$pr.current(),
            "General Note Entity",
            <IgesDimenGeneralNote>::static_type(),
            &mut $note,
            false,
        )?;
    };
}
pub(crate) use iges_block_0029;

macro_rules! iges_block_0030 {
    ($annotations:ident, $nbval:ident, $orientation_angles:ident, $view_origins:ident, $views:ident) => {
        let mut $nbval: i32 = Default::default();
        crate::iges_shared_blocks::iges_block_0055!($orientation_angles, $view_origins, $views);
        let mut $annotations: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
    };
}
pub(crate) use iges_block_0030;

macro_rules! iges_block_0031 {
    ($attr_types:ident, $attr_value_counts:ident, $attr_value_data_types:ident, $attr_value_pointers:ident, $attr_values:ident) => {
        let mut $attr_types: Handle<HArray1<i32>> = Default::default();
        let mut $attr_value_data_types: Handle<HArray1<i32>> = Default::default();
        let mut $attr_value_counts: Handle<HArray1<i32>> = Default::default();
        let mut $attr_values: Handle<HArray1<Handle<StandardTransient>>> = Default::default();
        let mut $attr_value_pointers: Handle<IgesDefsHArray1OfHArray1OfTextDisplayTemplate> =
            Default::default();
    };
}
pub(crate) use iges_block_0031;

macro_rules! iges_block_0032 {
    ($ent:ident, $i:ident, $iw:ident, $upper:ident) => {
        let mut $upper: i32 = $ent.nb_points()?;
        $iw.send($ent.datatype())?;
        $iw.send($upper)?;
        $iw.send($ent.z_displacement())?;
        for $i in 1..=$upper {
            $iw.send($ent.point($i)?.x())?;
            $iw.send($ent.point($i)?.y())?;
        }
    };
}
pub(crate) use iges_block_0032;

macro_rules! iges_block_0033 {
    ($ent:ident, $i:ident, $iter:ident, $up:ident) => {
        let mut $up: i32 = $ent.nb_views()?;
        let mut $i: i32 = Default::default();
        for $i in 1..=$up {
            $iter.get_one_item(&$ent.view_item($i)?.cast());
        }
        $up = $ent.nb_annotations()?;
        for $i in 1..=$up {
            $iter.get_one_item(&$ent.annotation($i)?.cast());
        }
    };
}
pub(crate) use iges_block_0033;

macro_rules! iges_block_0034 {
    ($ent:ident, $i:ident, $iw:ident, $up:ident) => {
        $iw.send($ent.list_count()?)?;
        $iw.send_boolean($ent.do_dont_flag())?;
        let mut $i: i32 = Default::default();
        let mut $up: i32 = $ent.list_count()?;
        for $i in 1..=$up {
            $iw.send($ent.list_position($i)?)?;
        }
    };
}
pub(crate) use iges_block_0034;

macro_rules! iges_block_0035 {
    ($dumper:ident, $ent:ident, $s:ident, $sublevel:ident) => {
        write!($s, "\nFirst  Leader  Entity : ")?;
        $dumper.dump(&$ent.first_leader(), $s, $sublevel, -1)?;
        write!($s, "\nSecond Leader  Entity : ")?;
        $dumper.dump(&$ent.second_leader(), $s, $sublevel, -1)?;
        write!($s, "\nFirst  Witness Entity : ")?;
    };
}
pub(crate) use iges_block_0035;

macro_rules! iges_block_0036 {
    ($corner:ident, $mirror_flag:ident, $rotate_flag:ident, $rotation_angle:ident, $slant_angle:ident) => {
        let mut $slant_angle: f64 = Default::default();
        let mut $rotation_angle: f64 = Default::default();
        let mut $mirror_flag: i32 = Default::default();
        let mut $rotate_flag: i32 = Default::default();
        let mut $corner: Xyz = Default::default();
    };
}
pub(crate) use iges_block_0036;

macro_rules! iges_block_0037 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.graphics_ignored(-1);
        $dc.line_font(IgesDataDefType::DefVoid);
        $dc.line_weight(IgesDataDefType::DefVoid);
        $dc.color(IgesDataDefType::DefVoid);
    };
}
pub(crate) use iges_block_0037;

macro_rules! iges_block_0038 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.line_font(IgesDataDefType::DefAny);
        $dc.line_weight(IgesDataDefType::DefValue);
        $dc.color(IgesDataDefType::DefAny);
        $dc.graphics_ignored(1);
    };
}
pub(crate) use iges_block_0038;

macro_rules! iges_block_0039 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.line_font(IgesDataDefType::DefAny);
        $dc.line_weight(IgesDataDefType::DefValue);
        $dc.color(IgesDataDefType::DefAny);
        $dc.use_flag_required(1);
    };
}
pub(crate) use iges_block_0039;

macro_rules! iges_block_0040 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.line_font(IgesDataDefType::DefVoid);
        $dc.line_weight(IgesDataDefType::DefVoid);
        $dc.color(IgesDataDefType::DefVoid);
        $dc.blank_status_ignored();
    };
}
pub(crate) use iges_block_0040;

macro_rules! iges_block_0041 {
    ($annotations:ident, $nbval:ident, $view_origins:ident, $views:ident) => {
        let mut $nbval: i32 = Default::default();
        let mut $views: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> = Default::default();
        let mut $view_origins: Handle<HArray1<Xy>> = Default::default();
        let mut $annotations: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
    };
}
pub(crate) use iges_block_0041;

macro_rules! iges_block_0042 {
    ($ent:ident, $nb:ident, $nulfont:ident, $res:ident) => {
        let mut $res: bool = $ent.rank_line_font() != 1;
        if $res {
            let mut $nulfont: Handle<dyn IgesDataLineFontEntity> = Default::default();
            $ent.init_line_font(&$nulfont, 1);
        }
        if $ent.datatype() == 1 {
            return Ok($res);
        }
        let mut $nb: i32 = $ent.nb_points()?;
    };
}
pub(crate) use iges_block_0042;

macro_rules! iges_block_0043 {
    ($items:ident, $num_items:ident, $orders:ident, $requirements:ident) => {
        let mut $requirements: Handle<HArray1<i32>> = Default::default();
        let mut $orders: Handle<HArray1<i32>> = Default::default();
        let mut $num_items: Handle<HArray1<i32>> = Default::default();
        let mut $items: Handle<IgesBasicHArray1OfHArray1OfInteger> = Default::default();
    };
}
pub(crate) use iges_block_0043;

macro_rules! iges_block_0044 {
    ($another:ident, $data_points:ident, $datatype:ident, $nbval:ident, $z_displacement:ident) => {
        let mut $datatype: i32 = $another.datatype();
        let mut $nbval: i32 = $another.nb_points()?;
        let mut $z_displacement: f64 = $another.z_displacement();
        let mut $data_points: Handle<HArray1<Xy>> = iges_new_array1::<Xy>(1, $nbval)?;
    };
}
pub(crate) use iges_block_0044;

macro_rules! iges_block_0045 {
    ($ent:ident, $i:ident, $iter:ident, $up:ident) => {
        let mut $i: i32 = Default::default();
        let mut $up: i32 = Default::default();
        $up = $ent.nb_displayed_entities()?;
        for $i in 1..=$up {
            $iter.get_one_item(&$ent.displayed_entity($i)?.cast());
        }
    };
}
pub(crate) use iges_block_0045;

macro_rules! iges_block_0046 {
    ($nb_property_values:ident, $nominal_size_name:ident, $nominal_size_value:ident, $standard_name:ident) => {
        let mut $nb_property_values: i32 = Default::default();
        let mut $nominal_size_value: f64 = Default::default();
        let mut $nominal_size_name: Handle<HAsciiString> = Default::default();
        let mut $standard_name: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use iges_block_0046;

macro_rules! iges_block_0047 {
    ($blue:ident, $ent:ident, $green:ident, $red:ident) => {
        let mut $red: f64 = Default::default();
        let mut $green: f64 = Default::default();
        let mut $blue: f64 = Default::default();
        $ent.rgb_intensity(&mut $red, &mut $green, &mut $blue);
    };
}
pub(crate) use iges_block_0047;

macro_rules! iges_block_0048 {
    ($temp_blue:ident, $temp_color_name:ident, $temp_green:ident, $temp_red:ident) => {
        let mut $temp_red: f64 = Default::default();
        let mut $temp_green: f64 = Default::default();
        let mut $temp_blue: f64 = Default::default();
        let mut $temp_color_name: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use iges_block_0048;

macro_rules! iges_block_0049 {
    ($designator:ident, $scale:ident, $translation:ident, $typeflag:ident) => {
        let mut $translation: Xyz = Default::default();
        let mut $scale: Xyz = Default::default();
        let mut $typeflag: i32 = Default::default();
        let mut $designator: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use iges_block_0049;

macro_rules! iges_block_0050 {
    ($ach:ident, $mess:ident, $res:ident) => {
        if !($res != 0) {
            return Ok(());
        }
        let mut $mess = String::new();
        $mess = format!("Mismatch for {} Entities displayed", $res);
        $ach.add_fail_text(&$mess, "Mismatch for %d Entities displayed");
    };
}
pub(crate) use iges_block_0050;

macro_rules! iges_block_0051 {
    ($ent:ident, $nul_display_entities:ident, $res:ident) => {
        if !$res {
            return Ok($res);
        }
        let mut $nul_display_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            Default::default();
        $ent.init_implied(&$nul_display_entities)?;
        return Ok($res);
    };
}
pub(crate) use iges_block_0051;

macro_rules! iges_block_0052 {
    ($another:ident, $first_leader:ident, $note:ident, $second_leader:ident, $tc:ident) => {
        let mut $note: Handle<IgesDimenGeneralNote> =
            $tc.transferred(&$another.note().cast())?.cast();
        crate::iges_shared_blocks::iges_block_0069!($another, $first_leader, $second_leader, $tc);
    };
}
pub(crate) use iges_block_0052;

macro_rules! iges_block_0053 {
    ($ach:ident, $ent:ident) => {
        if $ent.rank_line_font() != 1 {
            $ach.add_fail_text("Line Font Pattern != 1", "");
        }
        if $ent.datatype() != 1 {
            $ach.add_fail_text("Interpretation Flag != 1", "");
        }
        if ($ent.nb_points()? % 2) != 0 {
            $ach.add_fail_text("Number of data points is not even", "");
        }
    };
}
pub(crate) use iges_block_0053;

macro_rules! iges_block_0054 {
    ($unit_scales:ident, $unit_types:ident, $unit_values:ident) => {
        let mut $unit_types: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
        let mut $unit_values: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
        let mut $unit_scales: Handle<HArray1<f64>> = Default::default();
    };
}
pub(crate) use iges_block_0054;

macro_rules! iges_block_0055 {
    ($orientation_angles:ident, $view_origins:ident, $views:ident) => {
        let mut $views: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> = Default::default();
        let mut $view_origins: Handle<HArray1<Xy>> = Default::default();
        let mut $orientation_angles: Handle<HArray1<f64>> = Default::default();
    };
}
pub(crate) use iges_block_0055;
