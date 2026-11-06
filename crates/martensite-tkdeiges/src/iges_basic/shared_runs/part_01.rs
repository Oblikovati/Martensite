//! Statement runs the classes of the geometric packages repeat.
macro_rules! iges_shared_001 {
    ($ent:ident, $pr:ident, $self:ident, $temp_corner:ident, $temp_x_axis:ident, $temp_z_axis:ident, $tempreal:ident) => {
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Corner Point (Y)", &mut $tempreal)? {
                $temp_corner.set_y($tempreal);
            }
        } else {
            $temp_corner.set_y(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Corner Point (Z)", &mut $tempreal)? {
                $temp_corner.set_z($tempreal);
            }
        } else {
            $temp_corner.set_z(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Local X axis (I)", &mut $tempreal)? {
                $temp_x_axis.set_x($tempreal);
            }
        } else {
            $temp_x_axis.set_x(1.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Local X axis (J)", &mut $tempreal)? {
                $temp_x_axis.set_y($tempreal);
            }
        } else {
            $temp_x_axis.set_y(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Local X axis (K)", &mut $tempreal)? {
                $temp_x_axis.set_z($tempreal);
            }
        } else {
            $temp_x_axis.set_z(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Local Z axis (I)", &mut $tempreal)? {
                $temp_z_axis.set_x($tempreal);
            }
        } else {
            $temp_z_axis.set_x(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Local Z axis (J)", &mut $tempreal)? {
                $temp_z_axis.set_y($tempreal);
            }
        } else {
            $temp_z_axis.set_y(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Local Z axis (K)", &mut $tempreal)? {
                $temp_z_axis.set_z($tempreal);
            }
        } else {
            $temp_z_axis.set_z(1.0);
        }
        $self
            .dir_checker($ent)
            .check_type_and_form($pr.check(), $ent);
    };
}
pub(crate) use iges_shared_001;

macro_rules! iges_shared_002 {
    ($ent:ident, $pr:ident, $self:ident, $temp_axis:ident, $temp_center:ident, $tempreal:ident) => {
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Face center (X)", &mut $tempreal)? {
                $temp_center.set_x($tempreal);
            }
        } else {
            $temp_center.set_x(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Face center (Y)", &mut $tempreal)? {
                $temp_center.set_y($tempreal);
            }
        } else {
            $temp_center.set_y(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Face center (Z)", &mut $tempreal)? {
                $temp_center.set_z($tempreal);
            }
        } else {
            $temp_center.set_z(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Axis direction (I)", &mut $tempreal)? {
                $temp_axis.set_x($tempreal);
            }
        } else {
            $temp_axis.set_x(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Axis direction (J)", &mut $tempreal)? {
                $temp_axis.set_y($tempreal);
            }
        } else {
            $temp_axis.set_y(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Axis direction (K)", &mut $tempreal)? {
                $temp_axis.set_z($tempreal);
            }
        } else {
            $temp_axis.set_z(1.0);
        }
        $self
            .dir_checker($ent)
            .check_type_and_form($pr.check(), $ent);
    };
}
pub(crate) use iges_shared_002;

macro_rules! iges_shared_003 {
    ($ent:ident, $pr:ident, $self:ident, $temp_axis:ident, $tempreal:ident) => {
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Axis direction (I)", &mut $tempreal)? {
                $temp_axis.set_x($tempreal);
            }
        } else {
            $temp_axis.set_x(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Axis direction (J)", &mut $tempreal)? {
                $temp_axis.set_y($tempreal);
            }
        } else {
            $temp_axis.set_y(0.0);
        }
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Axis direction (K)", &mut $tempreal)? {
                $temp_axis.set_z($tempreal);
            }
        } else {
            $temp_axis.set_z(1.0);
        }
        $self
            .dir_checker($ent)
            .check_type_and_form($pr.check(), $ent);
    };
}
pub(crate) use iges_shared_003;

macro_rules! iges_shared_004 {
    ($ent_live:ident, $iw:ident) => {
        $iw.send($ent_live.corner().x())?;
        $iw.send($ent_live.corner().y())?;
        $iw.send($ent_live.corner().z())?;
        $iw.send($ent_live.x_axis()?.x())?;
        $iw.send($ent_live.x_axis()?.y())?;
        $iw.send($ent_live.x_axis()?.z())?;
        $iw.send($ent_live.z_axis()?.x())?;
        $iw.send($ent_live.z_axis()?.y())?;
        $iw.send($ent_live.z_axis()?.z())?;
    };
}
pub(crate) use iges_shared_004;

macro_rules! iges_shared_005 {
    ($ent:ident, $ent_array:ident, $ent_live:ident, $ir:ident, $nbval:ident, $pr:ident, $self:ident) => {
        let mut $nbval: i32 = 0;
        let mut $ent_array: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        if $pr.read_integer_text(&$pr.current(), "Count of Entities", &mut $nbval)? {
            $pr.read_ents(
                $ir,
                &$pr.current_list($nbval, 1),
                "Entities",
                &mut $ent_array,
                1,
            )?;
        }
        $self
            .dir_checker($ent)
            .check_type_and_form($pr.check(), $ent);
        $ent_live.init(&$ent_array)?;
    };
}
pub(crate) use iges_shared_005;

macro_rules! iges_shared_006 {
    ($ent_live:ident, $iw:ident) => {
        $iw.send($ent_live.axis_point().x())?;
        $iw.send($ent_live.axis_point().y())?;
        $iw.send($ent_live.axis_point().z())?;
        $iw.send($ent_live.axis()?.x())?;
        $iw.send($ent_live.axis()?.y())?;
        $iw.send($ent_live.axis()?.z())?;
    };
}
pub(crate) use iges_shared_006;

macro_rules! iges_shared_007 {
    ($ent_live:ident, $iw:ident) => {
        $iw.send($ent_live.face_center().x())?;
        $iw.send($ent_live.face_center().y())?;
        $iw.send($ent_live.face_center().z())?;
        $iw.send($ent_live.axis()?.x())?;
        $iw.send($ent_live.axis()?.y())?;
        $iw.send($ent_live.axis()?.z())?;
    };
}
pub(crate) use iges_shared_007;

macro_rules! iges_shared_008 {
    ($ax:ident, $ay:ident, $az:ident, $bx:ident, $by:ident, $bz:ident, $cx:ident, $cy:ident, $cz:ident, $dx:ident, $dy:ident, $dz:ident, $iw:ident) => {
        $iw.send($ax)?;
        $iw.send($bx)?;
        $iw.send($cx)?;
        $iw.send($dx)?;
        $iw.send($ay)?;
        $iw.send($by)?;
        $iw.send($cy)?;
        $iw.send($dy)?;
        $iw.send($az)?;
        $iw.send($bz)?;
        $iw.send($cz)?;
        $iw.send($dz)?;
    };
}
pub(crate) use iges_shared_008;

macro_rules! iges_shared_009 {
    ($ax:ident, $ay:ident, $az:ident, $bx:ident, $by:ident, $bz:ident, $cx:ident, $cy:ident, $cz:ident, $dx:ident, $dy:ident, $dz:ident) => {
        let mut $ax: f64 = 0.0;
        let mut $bx: f64 = 0.0;
        let mut $cx: f64 = 0.0;
        let mut $dx: f64 = 0.0;
        let mut $ay: f64 = 0.0;
        let mut $by: f64 = 0.0;
        let mut $cy: f64 = 0.0;
        let mut $dy: f64 = 0.0;
        let mut $az: f64 = 0.0;
        let mut $bz: f64 = 0.0;
        let mut $cz: f64 = 0.0;
        let mut $dz: f64 = 0.0;
    };
}
pub(crate) use iges_shared_009;

macro_rules! iges_shared_010 {
    ($ent_live:ident, $level:ident, $s:ident) => {
        write!($s, "\nXAxis  : ")?;
        iges_dump_xyz_l(
            $s,
            $level,
            &Xyz::from($ent_live.x_axis()?),
            &$ent_live.vector_location()?,
        )?;
        write!($s, "\nZAxis  : ")?;
        iges_dump_xyz_l(
            $s,
            $level,
            &Xyz::from($ent_live.z_axis()?),
            &$ent_live.vector_location()?,
        )?;
        writeln!($s)?;
    };
}
pub(crate) use iges_shared_010;

macro_rules! iges_shared_011 {
    ($ir:ident, $pr:ident, $temp_axis:ident, $temp_location:ident, $temp_radius:ident) => {
        $pr.read_entity_typed(
            $ir,
            &$pr.current(),
            "Point on axis",
            IgesGeomPoint::static_type(),
            &mut $temp_location,
            false,
        )?;
        $pr.read_entity_typed(
            $ir,
            &$pr.current(),
            "Axis direction",
            IgesGeomDirection::static_type(),
            &mut $temp_axis,
            false,
        )?;
        $pr.read_real_text(&$pr.current(), "Radius", &mut $temp_radius)?;
    };
}
pub(crate) use iges_shared_011;

macro_rules! iges_shared_012 {
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $ent_live:ident, $f:ident) => {
        let mut $a: f64 = 0.0;
        let mut $b: f64 = 0.0;
        let mut $c: f64 = 0.0;
        let mut $d: f64 = 0.0;
        let mut $e: f64 = 0.0;
        let mut $f: f64 = 0.0;
        $ent_live.equation(&mut $a, &mut $b, &mut $c, &mut $d, &mut $e, &mut $f);
    };
}
pub(crate) use iges_shared_012;

macro_rules! iges_shared_013 {
    ($dumper:ident, $ent_live:ident, $level:ident, $s:ident, $sublevel:ident) => {
        let $sublevel: i32 = if $level <= 4 { 0 } else { 1 };
        write!($s, "Point on axis  : ")?;
        $dumper.dump(&$ent_live.location_point(), $s, $sublevel, -1)?;
        write!($s, "\nAxis direction : ")?;
        $dumper.dump(&$ent_live.axis(), $s, $sublevel, -1)?;
    };
}
pub(crate) use iges_shared_013;

macro_rules! iges_shared_014 {
    ($another_live:ident, $tc:ident, $temp_axis:ident, $temp_location:ident, $temp_radius:ident) => {
        let $temp_location = $tc
            .transferred(&$another_live.location_point().cast::<StandardTransient>())?
            .cast::<IgesGeomPoint>();
        let $temp_axis = $tc
            .transferred(&$another_live.axis().cast::<StandardTransient>())?
            .cast::<IgesGeomDirection>();
        $temp_radius = $another_live.radius();
    };
}
pub(crate) use iges_shared_014;

macro_rules! iges_shared_015 {
    ($ent_live:ident, $level:ident, $s:ident) => {
        iges_dump_xyz_l(
            $s,
            $level,
            &Xyz::from($ent_live.axis_point()),
            &$ent_live.location()?,
        )?;
        write!($s, "\nAxis direction : ")?;
        iges_dump_xyz_l(
            $s,
            $level,
            &Xyz::from($ent_live.axis()?),
            &$ent_live.vector_location()?,
        )?;
        writeln!($s)?;
    };
}
pub(crate) use iges_shared_015;

macro_rules! iges_shared_016 {
    ($ent_live:ident, $level:ident, $s:ident) => {
        iges_dump_xyz_l(
            $s,
            $level,
            &Xyz::from($ent_live.face_center()),
            &$ent_live.location()?,
        )?;
        write!($s, "\nAxis : ")?;
        iges_dump_xyz_l(
            $s,
            $level,
            &Xyz::from($ent_live.axis()?),
            &$ent_live.vector_location()?,
        )?;
        writeln!($s)?;
    };
}
pub(crate) use iges_shared_016;

macro_rules! iges_shared_017 {
    ($ent_live:ident, $eps:ident, $pr:ident, $temp_x_axis:ident, $temp_z_axis:ident) => {
        let $eps: f64 = 1.0e-05;
        if !$temp_x_axis.is_equal(&Xyz::from($ent_live.x_axis()?), $eps) {
            $pr.add_warning("XAxis poorly unitary, normalized", "");
        }
        if !$temp_z_axis.is_equal(&Xyz::from($ent_live.z_axis()?), $eps) {
            $pr.add_warning("ZAxis poorly unitary, normalized", "");
        }
    };
}
pub(crate) use iges_shared_017;

macro_rules! iges_shared_018 {
    ($end:ident, $self:ident, $the_end:ident, $the_zt:ident) => {
        let $the_end = $self.state.read().end;
        let $the_zt = $self.state.read().zt;
        let mut $end: Xyz = Xyz::new($the_end.x(), $the_end.y(), $the_zt);
        if $self.has_transf() {
            $self.location()?.transforms(&mut $end);
        }
    };
}
pub(crate) use iges_shared_018;

macro_rules! iges_shared_019 {
    ($self:ident, $start:ident, $the_start:ident, $the_zt:ident) => {
        let $the_start = $self.state.read().start;
        let $the_zt = $self.state.read().zt;
        let mut $start: Xyz = Xyz::new($the_start.x(), $the_start.y(), $the_zt);
        if $self.has_transf() {
            $self.location()?.transforms(&mut $start);
        }
    };
}
pub(crate) use iges_shared_019;

macro_rules! iges_shared_020 {
    ($ax:ident, $ay:ident, $az:ident, $bx:ident, $by:ident, $bz:ident, $cx:ident, $cy:ident, $cz:ident, $dx:ident, $dy:ident, $dz:ident, $ent_live:ident, $i:ident) => {
        $ent_live.x_coord_polynomial($i, &mut $ax, &mut $bx, &mut $cx, &mut $dx)?;
        $ent_live.y_coord_polynomial($i, &mut $ay, &mut $by, &mut $cy, &mut $dy)?;
        $ent_live.z_coord_polynomial($i, &mut $az, &mut $bz, &mut $cz, &mut $dz)?;
    };
}
pub(crate) use iges_shared_020;

macro_rules! iges_shared_021 {
    ($all_xvalues:ident, $all_yvalues:ident, $all_zvalues:ident) => {
        let $all_xvalues: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 4)?;
        let $all_yvalues: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 4)?;
        let $all_zvalues: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 4)?;
    };
}
pub(crate) use iges_shared_021;

macro_rules! iges_shared_022 {
    ($axis:ident, $loc:ident, $self:ident) => {
        let mut $axis: Xyz = Xyz::new(0.0, 0.0, 1.0);
        if !$self.has_transf() {
            return iges_direction($axis);
        }
        let mut $loc: GeneralTransform3 = $self.location()?;
        $loc.set_translation_part(Xyz::new(0.0, 0.0, 0.0));
        $loc.transforms(&mut $axis);
    };
}
pub(crate) use iges_shared_022;

macro_rules! iges_shared_023 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.graphics_ignored(-1);
        $dc.line_font(IgesDataDefType::DefVoid);
        $dc.line_weight(IgesDataDefType::DefVoid);
        $dc.color(IgesDataDefType::DefVoid);
        $dc.blank_status_ignored();
        $dc.use_flag_ignored();
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_shared_023;

macro_rules! iges_shared_024 {
    ($ax:ident, $ay:ident, $az:ident, $bx:ident, $by:ident, $bz:ident, $cx:ident, $cy:ident, $cz:ident, $dx:ident, $dy:ident, $dz:ident, $ent_live:ident) => {
        $ent_live.x_values(&mut $ax, &mut $bx, &mut $cx, &mut $dx)?;
        $ent_live.y_values(&mut $ay, &mut $by, &mut $cy, &mut $dy)?;
        $ent_live.z_values(&mut $az, &mut $bz, &mut $cz, &mut $dz)?;
    };
}
pub(crate) use iges_shared_024;
