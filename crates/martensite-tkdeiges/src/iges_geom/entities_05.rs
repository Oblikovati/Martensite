#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_basic::IgesBasicSubfigureDef;
use crate::iges_basic::entity_shape::iges_placement_methods;
use crate::iges_basic::shared_runs::iges_shared_041;
use crate::iges_data::IgesDataIgesEntity;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines IGESOffsetSurface, Type `140` Form `0`
/// An offset surface is a surface defined in terms of an
/// already existing surface.If S(u, v) is a parametrised
/// regular surface and N(u, v) is a differential field of
/// unit normal vectors defined on the whole surface, and
/// "d" a fixed non zero real number, then offset surface
/// to S is a parametrised surface S(u, v) given by
/// O(u, v) = S(u, v) + d * N(u, v);
/// u1 `= u `= u2; v1 `= v `= v2;
IgesGeomOffsetSurface, IgesGeomOffsetSurfaceState, "IgesGeomOffsetSurface", parent IgesDataIgesEntity, { indicator: Xyz, distance: f64, surface: Handle<IgesDataIgesEntity> }
}
impl IgesGeomOffsetSurface {
    iges_methods! {
    get distance = distance: f64;
    get surface = surface: Handle<IgesDataIgesEntity>;
    }
    iges_placement_methods! {
    vector offset_indicator = indicator;
    located_vector transformed_offset_indicator = indicator;
    }
    /// This method is used to set the fields of the class
    /// OffsetSurface
    /// - anIndicator : Offset indicator
    /// - aDistance   : Offset distance
    /// - aSurface    : Surface that is offset
    pub fn init(&self, an_indicator: Xyz, a_distance: f64, a_surface: &Handle<IgesDataIgesEntity>) {
        {
            let mut state = self.state.write();
            state.indicator = an_indicator;
            state.distance = a_distance;
            state.surface = a_surface.clone();
        }
        self.init_type_and_form(140, 0);
    }
}
iges_entity! {
/// defines IGESPlane, Type `108` Form `-1,0,1`
/// A plane entity can be used to represent unbounded plane,
/// as well as bounded portion of a plane. In either of the
/// above cases the plane is defined within definition space
/// by means of coefficients A, B, C, D where at least one of
/// A, B, C is non-zero and A * XT + B * YT + C * ZT = D
IgesGeomPlane, IgesGeomPlaneState, "IgesGeomPlane", parent IgesDataIgesEntity, { a: f64, b: f64, c: f64, d: f64, curve: Handle<IgesDataIgesEntity>, attach: Xyz, size: f64 }
}
impl IgesGeomPlane {
    iges_methods! {
    get bounding_curve = curve: Handle<IgesDataIgesEntity>;
    get symbol_size = size: f64;
    }
    iges_placement_methods! {
    has has_bounding_curve = curve;
    point symbol_attach = attach;
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        a_curve: &Handle<IgesDataIgesEntity>,
        attach: Xyz,
        a_size: f64,
    ) {
        {
            let mut state = self.state.write();
            state.a = a;
            state.b = b;
            state.c = c;
            state.d = d;
            state.curve = a_curve.clone();
            state.attach = attach;
            state.size = a_size;
        }
        self.init_type_and_form(108, self.form_number());
    }
    /// Changes FormNumber (indicates the Type of Bound :
    /// 0 no Bound, 1 (External) Bound, -1 Hole)
    /// Remark that Init keeps this Value and must be consistent :
    /// aCurve Null if FormNumber = 0, Non-Null else
    /// Error if not in ranges [0-1] or [10-12]
    pub fn set_form_number(&self, form: i32) {
        let mut fn_: i32 = 0;
        if form < 0 {
            fn_ = -1;
        }
        if form > 0 {
            fn_ = 1;
        }
        self.init_type_and_form(108, fn_);
    }
    /// `equation`.
    pub fn equation(&self, a: &mut f64, b: &mut f64, c: &mut f64, d: &mut f64) {
        iges_shared_041!(self, the_a, the_b, the_c, the_d);
        *a = the_a;
        *b = the_b;
        *c = the_c;
        *d = the_d;
    }
    /// `transformed_equation`.
    pub fn transformed_equation(
        &self,
        a: &mut f64,
        b: &mut f64,
        c: &mut f64,
        d: &mut f64,
    ) -> Result<(), Failure> {
        let the_d = self.state.read().d;
        let the_c = self.state.read().c;
        let the_b = self.state.read().b;
        let the_a = self.state.read().a;
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut z1: f64 = the_d / the_c;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = the_d / the_b;
        let mut z2: f64 = 0.0;
        let mut x3: f64 = the_d / the_a;
        let mut y3: f64 = 0.0;
        let mut z3: f64 = 0.0;
        let mut p1: Xyz = Xyz::new(x1, y1, z1);
        let mut p2: Xyz = Xyz::new(x2, y2, z2);
        let mut p3: Xyz = Xyz::new(x3, y3, z3);
        self.location()?.transforms(&mut p1);
        self.location()?.transforms(&mut p2);
        self.location()?.transforms(&mut p3);
        x1 = p1.x();
        y1 = p1.y();
        z1 = p1.z();
        x2 = p2.x();
        y2 = p2.y();
        z2 = p2.z();
        x3 = p3.x();
        y3 = p3.y();
        z3 = p3.z();
        let c1: f64 = y1 * (-z3 + z2) + y2 * (-z1 + z3) + y3 * (z1 - z2);
        let c2: f64 = x1 * (z3 - z2) + x2 * (-z3 + z1) + x3 * (-z1 + z2);
        let c3: f64 = x1 * (-y3 + y2) + x2 * (-y1 + y3) + x3 * (y1 - y2);
        *a = c1;
        *b = c2;
        *c = c3;
        *d = c1 * x2 + c2 * y2 + c3 * z3;
        Ok(())
    }
    /// returns True if bounding curve exists and bounded portion is negative
    pub fn has_bounding_curve_hole(&self) -> bool {
        let the_curve = self.state.read().curve.clone();
        self.form_number() == -1 && !the_curve.is_null()
    }
    /// returns True if SymbolSize() ` 0, False if SymbolSize() = 0
    pub fn has_symbol_attach(&self) -> bool {
        let the_size = self.state.read().size;
        the_size > 0.0
    }
    /// returns (X, Y, Z) if symbol exists after applying Transf. Matrix
    /// else returns (0, 0, 0)
    pub fn transformed_symbol_attach(&self) -> Result<Point3, Failure> {
        let the_size = self.state.read().size;
        let the_attach = self.state.read().attach;
        if the_size > 0.0 && self.has_transf() {
            let mut symbol: Xyz = the_attach;
            self.location()?.transforms(&mut symbol);
            Ok(Point3::from(symbol))
        } else {
            Ok(Point3::new(0.0, 0.0, 0.0))
        }
    }
}
iges_entity! {
/// defines IGESPoint, Type `116` Form `0`
IgesGeomPoint, IgesGeomPointState, "IgesGeomPoint", parent IgesDataIgesEntity, { point: Xyz, symbol: Handle<IgesBasicSubfigureDef> }
}
impl IgesGeomPoint {
    iges_methods! {
    get display_symbol = symbol: Handle<IgesBasicSubfigureDef>;
    }
    iges_placement_methods! {
    point value = point;
    located_point transformed_value = point;
    has has_display_symbol = symbol;
    }
    /// This method is used to set the fields of the class Point
    /// - aPoint  : Coordinates of point
    /// - aSymbol : SubfigureDefinition entity specifying the
    ///   display symbol if there exists one, or zero
    pub fn init(&self, a_point: Xyz, a_symbol: &Handle<IgesBasicSubfigureDef>) {
        {
            let mut state = self.state.write();
            state.point = a_point;
            state.symbol = a_symbol.clone();
        }
        self.init_type_and_form(116, 0);
    }
}
iges_entity! {
/// defines IGESRuledSurface, Type `118` Form `0-1`
/// A ruled surface is formed by moving a line connecting points
/// of equal relative arc length or equal relative parametric
/// value on two parametric curves from a start point to a
/// terminate point on the curves. The parametric curves may be
/// points, lines, circles, conics, rational B-splines,
/// parametric splines or any parametric curve defined in
/// the IGES specification.
IgesGeomRuledSurface, IgesGeomRuledSurfaceState, "IgesGeomRuledSurface", parent IgesDataIgesEntity, { curve1: Handle<IgesDataIgesEntity>, curve2: Handle<IgesDataIgesEntity>, dir_flag: i32, dev_flag: i32 }
}
impl IgesGeomRuledSurface {
    iges_methods! {
    get first_curve = curve1: Handle<IgesDataIgesEntity>;
    get second_curve = curve2: Handle<IgesDataIgesEntity>;
    get direction_flag = dir_flag: i32;
    }
    /// This method is used to set the fields of the class
    /// RuledSurface
    /// - aCurve       : First parametric curve
    /// - anotherCurve : Second parametric curve
    /// - aDirFlag     : Direction Flag
    ///   0 = Join first to first, last to last
    ///   1 = Join first to last, last to first
    /// - aDevFlag     : Developable Surface Flag
    ///   1 = Developable
    ///   0 = Possibly not
    pub fn init(
        &self,
        a_curve: &Handle<IgesDataIgesEntity>,
        another_curve: &Handle<IgesDataIgesEntity>,
        a_dir_flag: i32,
        a_dev_flag: i32,
    ) {
        {
            let mut state = self.state.write();
            state.curve1 = a_curve.clone();
            state.curve2 = another_curve.clone();
            state.dir_flag = a_dir_flag;
            state.dev_flag = a_dev_flag;
        }
        self.init_type_and_form(118, self.form_number());
    }
    /// Sets `me` to be Ruled by Parameter (Form 1) if `mode` is
    /// True, or Ruled by Length (Form 0) else
    pub fn set_ruled_by_parameter(&self, f: bool) {
        self.init_type_and_form(118, if f { 1 } else { 0 });
    }
    /// Returns True if Form is 1
    pub fn is_ruled_by_parameter(&self) -> bool {
        self.form_number() != 0
    }
    /// returns True if developable else False
    pub fn is_developable(&self) -> bool {
        let the_dev_flag = self.state.read().dev_flag;
        the_dev_flag == 1
    }
}
