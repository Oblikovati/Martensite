//! Statement runs the classes of the geometric packages repeat.
macro_rules! iges_shared_025 {
    ($ent:ident, $pr:ident, $self:ident, $temp_corner:ident, $temp_x_axis:ident, $temp_z_axis:ident, $tempreal:ident) => {
        if $pr.defined_else_skip()? {
            if $pr.read_real_text(&$pr.current(), "Corner Point (X)", &mut $tempreal)? {
                $temp_corner.set_x($tempreal);
            }
        } else {
            $temp_corner.set_x(0.0);
        }
        $crate::iges_basic::shared_runs::iges_shared_001!(
            $ent,
            $pr,
            $self,
            $temp_corner,
            $temp_x_axis,
            $temp_z_axis,
            $tempreal
        );
    };
}
pub(crate) use iges_shared_025;

macro_rules! iges_shared_026 {
    ($ent_live:ident, $iw:ident) => {
        $iw.send($ent_live.start_point().x())?;
        $iw.send($ent_live.start_point().y())?;
        $iw.send($ent_live.end_point().x())?;
        $iw.send($ent_live.end_point().y())?;
    };
}
pub(crate) use iges_shared_026;

macro_rules! iges_shared_027 {
    ($ent_live:ident, $iter:ident) => {
        $iter.get_one_item(&$ent_live.center().cast::<StandardTransient>());
        $iter.get_one_item(&$ent_live.axis().cast::<StandardTransient>());
        $iter.get_one_item(&$ent_live.reference_dir().cast::<StandardTransient>());
    };
}
pub(crate) use iges_shared_027;

macro_rules! iges_shared_028 {
    ($ent_live:ident, $iter:ident) => {
        $iter.get_one_item(&$ent_live.location_point().cast::<StandardTransient>());
        $iter.get_one_item(&$ent_live.axis().cast::<StandardTransient>());
        $iter.get_one_item(&$ent_live.reference_dir().cast::<StandardTransient>());
    };
}
pub(crate) use iges_shared_028;

macro_rules! iges_shared_029 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.graphics_ignored(-1);
        $dc.line_font(IgesDataDefType::DefVoid);
        $dc.line_weight(IgesDataDefType::DefVoid);
        $dc.color(IgesDataDefType::DefVoid);
        $dc.blank_status_required(0);
        $dc.hierarchy_status_required(0);
    };
}
pub(crate) use iges_shared_029;

macro_rules! iges_shared_030 {
    ($ent_array:ident, $ianul:ident, $nb:ident) => {
        if $ianul == 0 {
            return Ok(false);
        }
        let mut $ent_array: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        if $ianul < $nb {
            $ent_array = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, $nb - $ianul)?;
        }
    };
}
pub(crate) use iges_shared_030;

macro_rules! iges_shared_031 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.graphics_ignored(-1);
        $dc.line_font(IgesDataDefType::DefVoid);
        $dc.line_weight(IgesDataDefType::DefVoid);
        $dc.color(IgesDataDefType::DefVoid);
        $dc.blank_status_ignored();
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_shared_031;

macro_rules! iges_shared_032 {
    ($ent:ident, $ent_live:ident, $ir:ident, $pr:ident, $self:ident, $temp_refdir:ident) => {
        if $ent_live.form_number() == 1 {
            $pr.read_entity_typed(
                $ir,
                &$pr.current(),
                "Reference direction",
                IgesGeomDirection::static_type(),
                &mut $temp_refdir,
                false,
            )?;
        }
        $self
            .dir_checker($ent)
            .check_type_and_form($pr.check(), $ent);
    };
}
pub(crate) use iges_shared_032;

macro_rules! iges_shared_033 {
    ($all_knots:ident, $all_poles:ident, $all_weights:ident) => {
        let mut $all_knots: Handle<HArray1<f64>> = Handle::default();
        let mut $all_weights: Handle<HArray1<f64>> = Handle::default();
        let mut $all_poles: Handle<HArray1<Xyz>> = Handle::default();
    };
}
pub(crate) use iges_shared_033;

macro_rules! iges_shared_034 {
    ($rmax:ident, $rmin:ident, $self:ident, $xax:ident, $xcen:ident, $yax:ident, $ycen:ident) => {
        let mut $xcen: f64 = 0.0;
        let mut $ycen: f64 = 0.0;
        let mut $xax: f64 = 0.0;
        let mut $yax: f64 = 0.0;
        $self.computed_definition(&mut $xcen, &mut $ycen, &mut $xax, &mut $yax, $rmin, $rmax);
    };
}
pub(crate) use iges_shared_034;

macro_rules! iges_shared_035 {
    ($dumper:ident, $ent_live:ident, $s:ident, $sublevel:ident) => {
        if $ent_live.is_parametrised() {
            write!($s, "Surface is Parametrised  -  Reference direction : ")?;
            $dumper.dump(&$ent_live.reference_dir(), $s, $sublevel, -1)?;
            writeln!($s)?;
        } else {
            writeln!($s, "Surface is UnParametrised")?;
        }
    };
}
pub(crate) use iges_shared_035;

macro_rules! iges_shared_036 {
    ($a_function_coord:ident, $a_tapered_offset_type:ident, $an_offset_type:ident, $arc_length1:ident, $arc_length2:ident, $off_distance1:ident, $off_distance2:ident) => {
        let mut $an_offset_type: i32 = 0;
        let mut $a_function_coord: i32 = 0;
        let mut $a_tapered_offset_type: i32 = 0;
        let mut $off_distance1: f64 = 0.0;
        let mut $off_distance2: f64 = 0.0;
        let mut $arc_length1: f64 = 0.0;
        let mut $arc_length2: f64 = 0.0;
    };
}
pub(crate) use iges_shared_036;

macro_rules! iges_shared_037 {
    ($temp_axis:ident, $temp_location:ident, $temp_radius:ident, $temp_refdir:ident) => {
        let mut $temp_location: Handle<IgesGeomPoint> = Handle::default();
        let mut $temp_axis: Handle<IgesGeomDirection> = Handle::default();
        let mut $temp_refdir: Handle<IgesGeomDirection> = Handle::default();
        let mut $temp_radius: f64 = 0.0;
    };
}
pub(crate) use iges_shared_037;

macro_rules! iges_shared_038 {
    ($a:ident, $b:ident, $c:ident, $d:ident, $ent_live:ident) => {
        let mut $a: f64 = 0.0;
        let mut $b: f64 = 0.0;
        let mut $c: f64 = 0.0;
        let mut $d: f64 = 0.0;
        $ent_live.equation(&mut $a, &mut $b, &mut $c, &mut $d);
    };
}
pub(crate) use iges_shared_038;

macro_rules! iges_shared_039 {
    ($pr:ident, $temp_axis:ident, $temp_center:ident, $temp_height:ident, $tempreal:ident) => {
        let mut $tempreal: f64 = 0.0;
        let mut $temp_center: Xyz = Xyz::default();
        let mut $temp_axis: Xyz = Xyz::default();
        $pr.read_real_text(&$pr.current(), "Height", &mut $temp_height)?;
    };
}
pub(crate) use iges_shared_039;

macro_rules! iges_shared_040 {
    ($temp_corner:ident, $temp_size:ident, $temp_x_axis:ident, $temp_z_axis:ident) => {
        let mut $temp_size: Xyz = Xyz::default();
        let mut $temp_corner: Xyz = Xyz::default();
        let mut $temp_x_axis: Xyz = Xyz::default();
        let mut $temp_z_axis: Xyz = Xyz::default();
    };
}
pub(crate) use iges_shared_040;

macro_rules! iges_shared_041 {
    ($self:ident, $the_a:ident, $the_b:ident, $the_c:ident, $the_d:ident) => {
        let $the_a = $self.state.read().a;
        let $the_b = $self.state.read().b;
        let $the_c = $self.state.read().c;
        let $the_d = $self.state.read().d;
    };
}
pub(crate) use iges_shared_041;

macro_rules! iges_shared_042 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.line_font(IgesDataDefType::DefAny);
        $dc.color(IgesDataDefType::DefAny);
        $dc.blank_status_ignored();
        $dc.subordinate_status_required(1);
        $dc.hierarchy_status_ignored();
    };
}
pub(crate) use iges_shared_042;

macro_rules! iges_shared_043 {
    ($ach:ident, $ent_live:ident, $eps:ident, $prosca:ident) => {
        let $eps: f64 = 1.0e-04;
        let $prosca: f64 = $ent_live.x_axis()? * $ent_live.z_axis()?;
        if $prosca < -$eps || $prosca > $eps {
            $ach.add_fail_text("Local Z axis : Not orthogonal to X axis", "");
        }
    };
}
pub(crate) use iges_shared_043;

macro_rules! iges_shared_044 {
    ($ent_live:ident, $iw:ident) => {
        $iw.send($ent_live.center().x())?;
        $iw.send($ent_live.center().y())?;
        $iw.send($ent_live.center().z())?;
    };
}
pub(crate) use iges_shared_044;

macro_rules! iges_shared_045 {
    ($ent_live:ident, $iw:ident) => {
        $iw.send($ent_live.end_point().x())?;
        $iw.send($ent_live.end_point().y())?;
        $iw.send($ent_live.end_point().z())?;
    };
}
pub(crate) use iges_shared_045;

macro_rules! iges_shared_046 {
    ($ent_live:ident, $iw:ident) => {
        $iw.send($ent_live.size().x())?;
        $iw.send($ent_live.size().y())?;
        $iw.send($ent_live.size().z())?;
    };
}
pub(crate) use iges_shared_046;

macro_rules! iges_shared_047 {
    ($ent_live:ident, $iw:ident) => {
        $iw.send($ent_live.value().x())?;
        $iw.send($ent_live.value().y())?;
        $iw.send($ent_live.value().z())?;
    };
}
pub(crate) use iges_shared_047;

macro_rules! iges_shared_048 {
    ($another_live:ident, $ent_array:ident, $lower:ident, $upper:ident) => {
        let $lower: i32 = 1;
        let $upper: i32 = $another_live.nb_entities();
        let $ent_array: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>($lower, $upper)?;
    };
}
pub(crate) use iges_shared_048;

macro_rules! iges_shared_049 {
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident) => {
        let mut $a: f64 = 0.0;
        let mut $b: f64 = 0.0;
        let mut $c: f64 = 0.0;
        let mut $d: f64 = 0.0;
        let mut $e: f64 = 0.0;
        let mut $f: f64 = 0.0;
    };
}
pub(crate) use iges_shared_049;

macro_rules! iges_shared_050 {
    ($a_closed:ident, $a_degree:ident, $a_periodic:ident, $a_planar:ident, $a_polynomial:ident, $an_index:ident) => {
        let mut $an_index: i32 = 0;
        let mut $a_degree: i32 = 0;
        let mut $a_planar: bool = false;
        let mut $a_closed: bool = false;
        let mut $a_polynomial: bool = false;
        let mut $a_periodic: bool = false;
    };
}
pub(crate) use iges_shared_050;

macro_rules! iges_shared_051 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.line_font(IgesDataDefType::DefVoid);
        $dc.line_weight(IgesDataDefType::DefVoid);
        $dc.color(IgesDataDefType::DefAny);
        $dc.subordinate_status_required(1);
    };
}
pub(crate) use iges_shared_051;

macro_rules! iges_shared_052 {
    ($dc:ident) => {
        $dc.structure(IgesDataDefType::DefVoid);
        $dc.line_font(IgesDataDefType::DefVoid);
        $dc.line_weight(IgesDataDefType::DefVoid);
        $dc.color(IgesDataDefType::DefVoid);
        $dc.subordinate_status_required(1);
    };
}
pub(crate) use iges_shared_052;

macro_rules! iges_shared_053 {
    ($dumper:ident, $ent_live:ident, $level:ident, $s:ident, $sublevel:ident) => {
        let $sublevel: i32 = if $level <= 4 { 0 } else { 1 };
        write!($s, "Center : ")?;
        $dumper.dump(&$ent_live.center(), $s, $sublevel, -1)?;
    };
}
pub(crate) use iges_shared_053;

macro_rules! iges_shared_054 {
    ($ent_live:ident, $level:ident, $s:ident) => {
        write!($s, "  Normal Axis : ")?;
        iges_dump_xyz_l(
            $s,
            $level,
            &Xyz::from($ent_live.axis()),
            &$ent_live.vector_location()?,
        )?;
        writeln!($s)?;
    };
}
pub(crate) use iges_shared_054;
