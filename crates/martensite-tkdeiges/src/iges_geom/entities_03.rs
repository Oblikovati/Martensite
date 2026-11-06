#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_basic::entity_shape::iges_direction;
use crate::iges_basic::shared_runs::{
    iges_shared_018, iges_shared_019, iges_shared_022, iges_shared_034, iges_shared_041,
    iges_shared_049,
};
use crate::iges_data::IgesDataIgesEntity;
use crate::{iges_entity, iges_methods};
use martensite_cmath as cmath;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{
    Direction3, GeneralTransform3, Point2, Point3, Xy, Xyz, dz, resolution,
};

iges_entity! {
/// defines IGESConicArc, Type `104` Form `0-3` in package IGESGeom
/// A conic arc is a bounded connected portion of a parent
/// conic curve which consists of more than one point. The
/// parent conic curve is either an ellipse, a parabola, or
/// a hyperbola. The definition space coordinate system is
/// always chosen so that the conic arc lies in a plane either
/// coincident with or parallel to XT, YT plane. Within such
/// a plane a conic is defined by the six coefficients in the
/// following equation.
/// A*XT^2 + B*XT*YT + C*YT^2 + D*XT + E*YT + F = 0
IgesGeomConicArc, IgesGeomConicArcState, "IgesGeomConicArc", parent IgesDataIgesEntity, { a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, zt: f64, start: Xy, end: Xy }
}
impl IgesGeomConicArc {
    iges_methods! {
    get z_plane = zt: f64;
    }
    /// This method is used to set the fields of the class
    /// ConicalArc
    /// - A, B, C, D, E, F : Coefficients of the equation
    ///   defining conic arc
    /// - ZT               : Parallel ZT displacement of the arc
    ///   from XT, YT plane.
    /// - aStart           : Starting point of the conic arc
    /// - anEnd            : End point of the conic arc
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
        zt: f64,
        a_start: Xy,
        an_end: Xy,
    ) {
        {
            let mut state = self.state.write();
            state.a = a;
            state.b = b;
            state.c = c;
            state.d = d;
            state.e = e;
            state.f = f;
            state.zt = zt;
            state.start = a_start;
            state.end = an_end;
        }
        let mut fn_: i32 = self.form_number();
        if fn_ == 0 {
            fn_ = self.computed_form_number();
        }
        self.init_type_and_form(104, fn_);
    }
    /// sets the Form Number equal to ComputedFormNumber,
    /// returns True if changed
    pub fn own_correct(&self) -> bool {
        let cfn: i32 = self.computed_form_number();
        if self.form_number() == cfn {
            return false;
        }
        self.init_type_and_form(104, cfn);
        true
    }
    /// Computes the Form Number according to the equation
    /// 1 for Ellipse, 2 for Hyperbola, 3 for Parabola
    pub fn computed_form_number(&self) -> i32 {
        let the_a = self.state.read().a;
        let the_c = self.state.read().c;
        let the_f = self.state.read().f;
        let the_e = self.state.read().e;
        let the_b = self.state.read().b;
        let the_d = self.state.read().d;
        let eps: f64 = 1.0e-08;
        let eps2: f64 = eps * eps;
        let eps4: f64 = eps2 * eps2;
        let q1: f64 = the_a * (the_c * the_f - the_e * the_e / 4.0)
            + the_b / 2.0 * (the_e * the_d / 4.0 - the_b * the_f / 2.0)
            + the_d / 2.0 * (the_b * the_e / 4.0 - the_c * the_d / 2.0);
        let q2: f64 = the_a * the_c - the_b * the_b / 4.0;
        let q3: f64 = the_a + the_c;
        if q2 > eps4 && q1 * q3 < 0.0 {
            return 1;
        }
        if q2 < -eps4 && q1.abs() > eps4 {
            return 2;
        }
        if q2.abs() <= eps4 && q1.abs() > eps4 {
            return 3;
        }
        0
    }
    /// `equation`.
    pub fn equation(
        &self,
        a: &mut f64,
        b: &mut f64,
        c: &mut f64,
        d: &mut f64,
        e: &mut f64,
        f: &mut f64,
    ) {
        iges_shared_041!(self, the_a, the_b, the_c, the_d);
        let the_e = self.state.read().e;
        let the_f = self.state.read().f;
        *a = the_a;
        *b = the_b;
        *c = the_c;
        *d = the_d;
        *e = the_e;
        *f = the_f;
    }
    /// returns the starting point of the arc
    pub fn start_point(&self) -> Point2 {
        let the_start = self.state.read().start;
        Point2::new(the_start.x(), the_start.y())
    }
    /// returns the starting point of the arc after applying
    /// Transf. Matrix
    pub fn transformed_start_point(&self) -> Result<Point3, Failure> {
        iges_shared_019!(self, start, the_start, the_zt);
        Ok(Point3::from(start))
    }
    /// returns the end point of the arc
    pub fn end_point(&self) -> Point2 {
        let the_end = self.state.read().end;
        Point2::new(the_end.x(), the_end.y())
    }
    /// returns the end point of the arc after applying
    /// Transf. Matrix
    pub fn transformed_end_point(&self) -> Result<Point3, Failure> {
        iges_shared_018!(end, self, the_end, the_zt);
        Ok(Point3::from(end))
    }
    /// returns True if parent conic curve is an ellipse
    pub fn is_from_ellipse(&self) -> bool {
        let mut fn_: i32 = self.form_number();
        if fn_ == 0 {
            fn_ = self.computed_form_number();
        }
        fn_ == 1
    }
    /// returns True if parent conic curve is a parabola
    pub fn is_from_parabola(&self) -> bool {
        let mut fn_: i32 = self.form_number();
        if fn_ == 0 {
            fn_ = self.computed_form_number();
        }
        fn_ == 3
    }
    /// returns True if parent conic curve is a hyperbola
    pub fn is_from_hyperbola(&self) -> bool {
        let mut fn_: i32 = self.form_number();
        if fn_ == 0 {
            fn_ = self.computed_form_number();
        }
        fn_ == 2
    }
    /// returns True if StartPoint = EndPoint
    pub fn is_closed(&self) -> bool {
        let the_start = self.state.read().start;
        let the_end = self.state.read().end;
        the_start.x() == the_end.x() && the_start.y() == the_end.y()
    }
    /// Z-Axis of conic (i.e. [0,0,1])
    pub fn axis(&self) -> Direction3 {
        dz()
    }
    /// Z-Axis after applying Trans. Matrix
    pub fn transformed_axis(&self) -> Result<Direction3, Failure> {
        iges_shared_022!(axis, loc, self);
        iges_direction(axis)
    }
    /// Returns a Definition computed from equation, easier to use
    /// `Center` : the center of the conic (meaningless for
    /// a parabola) (defined with Z displacement)
    /// `MainAxis` : the Main Axis of the conic (for a Circle,
    /// arbitrary the X Axis)
    /// `Rmin,Rmax` : Minor and Major Radii of the conic
    /// For a Circle, Rmin = Rmax,
    /// For a Parabola, Rmin = Rmax = the Focal
    /// Warning : the basic definition (by equation) is not very stable,
    /// limit cases may be approximative
    pub fn definition(
        &self,
        center: &mut Point3,
        main_axis: &mut Direction3,
        rmin: &mut f64,
        rmax: &mut f64,
    ) -> Result<(), Failure> {
        let the_zt = self.state.read().zt;
        iges_shared_034!(rmax, rmin, self, xax, xcen, yax, ycen);
        *center = Point3::new(xcen, ycen, the_zt);
        *main_axis = Direction3::new(xax, yax, 0.0)?;
        Ok(())
    }
    /// Same as Definition, but the Location is applied on the
    /// Center and the MainAxis
    pub fn transformed_definition(
        &self,
        center: &mut Point3,
        main_axis: &mut Direction3,
        rmin: &mut f64,
        rmax: &mut f64,
    ) -> Result<(), Failure> {
        let the_zt = self.state.read().zt;
        if !self.has_transf() {
            self.definition(center, main_axis, rmin, rmax)?;
            return Ok(());
        }
        iges_shared_034!(rmax, rmin, self, xax, xcen, yax, ycen);
        let mut loc: GeneralTransform3 = self.location()?;
        let mut cen: Xyz = Xyz::new(xcen, ycen, the_zt);
        let mut axis: Xyz = Xyz::new(xax, yax, 0.0);
        loc.transforms(&mut cen);
        loc.set_translation_part(Xyz::new(0.0, 0.0, 0.0));
        loc.transforms(&mut axis);
        *center = Point3::new(cen.x(), cen.y(), cen.z());
        *main_axis = Direction3::new(axis.x(), axis.y(), axis.z())?;
        Ok(())
    }
    /// Computes and returns the coordinates of the definition of
    /// a comic from its equation. Used by Definition &
    /// TransformedDefinition, or may be called directly if needed
    pub fn computed_definition(
        &self,
        xcen: &mut f64,
        ycen: &mut f64,
        xax: &mut f64,
        yax: &mut f64,
        rmin: &mut f64,
        rmax: &mut f64,
    ) {
        iges_shared_049!(a, b, c, d, e, f);
        self.equation(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f);
        b = b / 2.0;
        d = d / 2.0;
        e = e / 2.0;
        let eps: f64 = 1.0e-08;
        if self.is_from_parabola() {
            *rmax = -1.0;
            *rmin = *rmax;
            if a.abs() <= eps && b.abs() <= eps {
                *xcen = (f * c - e * e) / c / d / 2.0;
                *ycen = e / c;
                let focal: f64 = -d / c;
                *xax = if focal >= 0.0 { 1.0 } else { -1.0 };
                *yax = 0.0;
                *rmax = focal.abs();
                *rmin = *rmax;
            } else {
                let ss: f64 = a + c;
                let cc: f64 = -(a * d + b * e) / ss;
                let dd: f64 = d + (c * d - b * e) / ss;
                let fc: f64 = (a * e - b * d) / ss;
                let ee: f64 = e + fc;
                let dn: f64 = a * ee - dd * b;
                *xcen = (cc * ee + f * b) / dn;
                *ycen = (-cc * dd - f * a) / dn;
                let mut teta: f64 = std::f64::consts::PI / 2.0;
                if b.abs() > eps {
                    teta = cmath::atan(-a / b);
                }
                if fc < 0.0 {
                    teta += std::f64::consts::PI;
                }
                *xax = cmath::cos(teta);
                *yax = cmath::sin(teta);
                *rmax = fc.abs() / f64::sqrt(a * a + b * b) / 2.0;
                *rmin = *rmax;
            }
        } else {
            let gdet: f64 = a * c * f + 2.0 * b * d * e - c * d * d - a * e * e - b * b * f;
            let pdet: f64 = a * c - b * b;
            *xcen = (b * e - c * d) / pdet;
            *ycen = (b * d - a * e) / pdet;
            let mut term1: f64 = a - c;
            let mut term2: f64 = 2.0 * b;
            let mut cos2t: f64 = 0.0;
            let mut auxil: f64 = 0.0;
            if term1.abs() < resolution() {
                cos2t = 1.0;
                auxil = term2;
            } else {
                let t2d: f64 = term2 / term1;
                cos2t = 1.0 / f64::sqrt(1.0 + t2d * t2d);
                auxil = f64::sqrt(term1 * term1 + term2 * term2);
            }
            let cost: f64 = f64::sqrt((1.0 + cos2t) / 2.0);
            let sint: f64 = f64::sqrt((1.0 - cos2t) / 2.0);
            let aprim: f64 = (a + c + auxil) / 2.0;
            let cprim: f64 = (a + c - auxil) / 2.0;
            term1 = -gdet / (aprim * pdet);
            term2 = -gdet / (cprim * pdet);
            if self.is_from_ellipse() {
                *xax = cost;
                *yax = sint;
                *rmin = f64::sqrt(term1);
                *rmax = f64::sqrt(term2);
                if *rmax < *rmin {
                    *rmax = f64::sqrt(term1);
                    *rmin = f64::sqrt(term2);
                }
            } else if term1 <= eps {
                *xax = -sint;
                *yax = cost;
                *rmin = f64::sqrt(-term1);
                *rmax = f64::sqrt(term2);
            } else {
                *xax = cost;
                *yax = sint;
                *rmin = f64::sqrt(-term2);
                *rmax = f64::sqrt(term1);
            }
        }
    }
}
