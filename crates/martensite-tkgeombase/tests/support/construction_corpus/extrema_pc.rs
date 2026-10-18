//! The classic point-curve Extrema requests; the counterpart of
//! the implementation (same encoding and token order).
#![allow(clippy::unit_arg, clippy::type_complexity)]
use super::common::{Reader, attempt, end, flag, int, pnt, pnt2d, real};
use super::extrema_el::{poncurv, poncurv2d};
use super::fixture_input::GeometryFixtureInput;
use super::pc2d::curve2d;
use super::pc3d::curve3d;
use super::print::Output;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeombase::extrema::{
    Curve2dTool, CurveTool, ExtremaCurveTool, GCurveLocator, GFuncExtPC, GGExtPC, GGenExtPC,
    GLocateExtPC, GenLocateExtPC, POnCurve,
};
use martensite_tkmath::gp::{Point2, Point3};

/// The 3D or 2D side of a request family (GbSpace3 / GbSpace2).
pub trait Space {
    type Tool: ExtremaCurveTool<Curve: RuntimeTyped + Send + Sync>;
    fn adaptor(
        r: &mut impl GeometryFixtureInput,
    ) -> Result<Handle<<Self::Tool as ExtremaCurveTool>::Curve>, Failure>;
    fn point(r: &mut impl GeometryFixtureInput) -> <Self::Tool as ExtremaCurveTool>::Point;
    fn pnt(output: &Output, p: <Self::Tool as ExtremaCurveTool>::Point);
    fn pon(output: &Output, p: POnCurve<<Self::Tool as ExtremaCurveTool>::Point>);
}
pub struct Space3;
pub struct Space2;
impl Space for Space3 {
    type Tool = CurveTool;
    fn adaptor(r: &mut impl GeometryFixtureInput) -> Result<Handle<dyn Adaptor3dCurve>, Failure> {
        let c = curve3d(r)?;
        if r.flag() {
            let (u1, u2) = (r.real(), r.real());
            return Ok(Handle::new(GeomAdaptorCurve::with_range(&c, u1, u2)?).cast());
        }
        Ok(Handle::new(GeomAdaptorCurve::with_curve(&c)?).cast())
    }
    fn point(r: &mut impl GeometryFixtureInput) -> Point3 {
        r.pnt()
    }
    fn pnt(output: &Output, p: Point3) {
        pnt(output, p);
    }
    fn pon(output: &Output, p: POnCurve<Point3>) {
        poncurv(output, p);
    }
}
impl Space for Space2 {
    type Tool = Curve2dTool;
    fn adaptor(r: &mut impl GeometryFixtureInput) -> Result<Handle<dyn Adaptor2dCurve2d>, Failure> {
        let c = curve2d(r)?;
        if r.flag() {
            let (u1, u2) = (r.real(), r.real());
            return Ok(Handle::new(Geom2dAdaptorCurve::with_range(&c, u1, u2)?).cast());
        }
        Ok(Handle::new(Geom2dAdaptorCurve::with_curve(&c)?).cast())
    }
    fn point(r: &mut impl GeometryFixtureInput) -> Point2 {
        r.pnt2d()
    }
    fn pnt(output: &Output, p: Point2) {
        pnt2d(output, p);
    }
    fn pon(output: &Output, p: POnCurve<Point2>) {
        poncurv2d(output, p);
    }
}

type P<S> = <<S as Space>::Tool as ExtremaCurveTool>::Point;

fn extpc_result<S: Space>(output: &Output, e: &GGExtPC<'_, S::Tool>) {
    flag(output, e.is_done());
    attempt(output, || {
        let n = e.nb_ext()?;
        int(output, n);
        for i in 1..=n {
            real(output, e.square_distance(i)?);
            flag(output, e.is_min(i)?);
            S::pon(output, e.point(i)?);
        }
        Ok(())
    });
    let (d1, d2, p1, p2) = e.trimmed_square_distances();
    real(output, d1);
    real(output, d2);
    S::pnt(output, p1);
    S::pnt(output, p2);
    attempt(output, || Ok(real(output, e.square_distance(0)?)));
}

fn extpc_body<S: Space>(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = S::adaptor(r)?;
    let (tolf, s1, s2) = (r.real(), r.real(), r.real());
    let n = r.integer();
    let mut reused = GGExtPC::<S::Tool>::new();
    attempt(output, || reused.initialize(&*a, s1, s2, tolf));
    for _ in 0..n {
        let p = S::point(r);
        attempt(output, || {
            Ok(extpc_result::<S>(
                output,
                &GGExtPC::<S::Tool>::with_curve(p, &*a, tolf)?,
            ))
        });
        attempt(output, || {
            reused.perform(p)?;
            extpc_result::<S>(output, &reused);
            Ok(())
        });
    }
    Ok(())
}

pub fn pc_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || extpc_body::<Space3>(output, &mut r));
    end(output);
}
pub fn pc2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || extpc_body::<Space2>(output, &mut r));
    end(output);
}

/// The accessors of a locate result (LocateExtPC or GenLocateExtPC).
trait LocView<S: Space> {
    fn done(&self) -> bool;
    fn sq(&self) -> Result<f64, Failure>;
    fn min(&self) -> Result<bool, Failure>;
    fn pon(&self) -> Result<POnCurve<P<S>>, Failure>;
}
impl<S: Space> LocView<S> for GLocateExtPC<'_, S::Tool> {
    fn done(&self) -> bool {
        self.is_done()
    }
    fn sq(&self) -> Result<f64, Failure> {
        self.square_distance()
    }
    fn min(&self) -> Result<bool, Failure> {
        self.is_min()
    }
    fn pon(&self) -> Result<POnCurve<P<S>>, Failure> {
        self.point()
    }
}
impl<S: Space> LocView<S> for GenLocateExtPC<'_, S::Tool> {
    fn done(&self) -> bool {
        self.is_done()
    }
    fn sq(&self) -> Result<f64, Failure> {
        self.square_distance()
    }
    fn min(&self) -> Result<bool, Failure> {
        self.is_min()
    }
    fn pon(&self) -> Result<POnCurve<P<S>>, Failure> {
        self.point()
    }
}
fn loc_result<S: Space>(output: &Output, e: &impl LocView<S>) {
    flag(output, e.done());
    attempt(output, || Ok(real(output, e.sq()?)));
    attempt(output, || Ok(flag(output, e.min()?)));
    attempt(output, || Ok(S::pon(output, e.pon()?)));
}

fn locpc_body<S: Space>(output: &Output, r: &mut Reader, owned: bool) -> Result<(), Failure> {
    let a = S::adaptor(r)?;
    let (tolf, s1, s2) = (r.real(), r.real(), r.real());
    let n = r.integer();
    let mut reused = GLocateExtPC::<S::Tool>::new();
    if owned {
        attempt(output, || {
            reused.initialize_owned(a.clone(), (s1, s2), tolf)
        });
    } else {
        attempt(output, || reused.initialize(&*a, (s1, s2), tolf));
    }
    for _ in 0..n {
        let p = S::point(r);
        let u0 = r.real();
        attempt(output, || {
            Ok(loc_result::<S>(
                output,
                &GLocateExtPC::<S::Tool>::with_curve(p, &*a, u0, tolf)?,
            ))
        });
        attempt(output, || {
            reused.perform(p, u0)?;
            loc_result::<S>(output, &reused);
            Ok(())
        });
    }
    Ok(())
}

pub fn locpc_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || locpc_body::<Space3>(output, &mut r, false));
    end(output);
}
pub fn locpc2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || locpc_body::<Space2>(output, &mut r, false));
    end(output);
}

fn epc_result<S: Space>(output: &Output, e: &GGenExtPC<'_, S::Tool>) {
    flag(output, e.is_done());
    attempt(output, || {
        let n = e.nb_ext()?;
        int(output, n);
        for i in 1..=n {
            real(output, e.square_distance(i)?);
            flag(output, e.is_min(i)?);
            S::pon(output, e.point(i)?);
        }
        Ok(())
    });
    attempt(output, || Ok(real(output, e.square_distance(0)?)));
}

fn genpc_body<S: Space>(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = S::adaptor(r)?;
    let nb = r.integer();
    let (umin, usup, tolu, tolf) = (r.real(), r.real(), r.real(), r.real());
    let n = r.integer();
    for _ in 0..n {
        let p = S::point(r);
        attempt(output, || {
            Ok(epc_result::<S>(
                output,
                &GGenExtPC::<S::Tool>::with_curve(p, &*a, nb, tolu, tolf)?,
            ))
        });
        attempt(output, || {
            let e = GGenExtPC::<S::Tool>::with_range(p, &*a, nb, (umin, usup), tolu, tolf)?;
            epc_result::<S>(output, &e);
            Ok(())
        });
        attempt(output, || {
            let l = GenLocateExtPC::<S::Tool>::with_range(
                p,
                &*a,
                0.5 * (umin + usup),
                (umin, usup),
                tolu,
            )?;
            loc_result::<S>(output, &l);
            Ok(())
        });
        attempt(output, || {
            Ok(loc_result::<S>(
                output,
                &GenLocateExtPC::<S::Tool>::with_curve(p, &*a, umin, tolu)?,
            ))
        });
    }
    Ok(())
}

pub fn genpc_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || genpc_body::<Space3>(output, &mut r));
    end(output);
}
pub fn genpc2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || genpc_body::<Space2>(output, &mut r));
    end(output);
}

fn pcf_step<S: Space>(output: &Output, f: &mut GFuncExtPC<'_, S::Tool>, u: f64) {
    let (mut v, mut d) = (0.0, 0.0);
    attempt(output, || {
        flag(output, f.value(u, &mut v)?);
        real(output, v);
        Ok(())
    });
    attempt(output, || {
        flag(output, f.derivative(u, &mut d)?);
        real(output, d);
        Ok(())
    });
    attempt(output, || {
        let (mut v, mut d) = (0.0, 0.0);
        flag(output, f.values(u, &mut v, &mut d)?);
        real(output, v);
        real(output, d);
        Ok(())
    });
    attempt(output, || Ok(int(output, f.get_state_number()?)));
}

fn pcf_body<S: Space>(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = S::adaptor(r)?;
    let p = S::point(r);
    let us = r.reals();
    let mut f = GFuncExtPC::<S::Tool>::with_point_curve(p, &*a)?;
    attempt(output, || Ok(real(output, f.search_of_tolerance()?)));
    for u in us {
        pcf_step::<S>(output, &mut f, u);
    }
    int(output, f.nb_ext());
    for i in 0..=f.nb_ext() + 1 {
        attempt(output, || {
            real(output, f.square_distance(i)?);
            flag(output, f.is_min(i)?);
            S::pon(output, f.point(i)?);
            Ok(())
        });
    }
    let mut g = GFuncExtPC::<S::Tool>::new();
    let mut v = 0.0;
    attempt(output, || Ok(flag(output, g.value(0.5, &mut v)?)));
    g.set_point(p);
    attempt(output, || Ok(flag(output, g.value(0.5, &mut v)?)));
    g.initialize(&*a)?;
    g.sub_interval_initialize(0.0, 1.0);
    attempt(output, || {
        flag(output, g.value(0.5, &mut v)?);
        real(output, v);
        Ok(())
    });
    Ok(())
}

pub fn pcf_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || pcf_body::<Space3>(output, &mut r));
    end(output);
}
pub fn pcf2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || pcf_body::<Space2>(output, &mut r));
    end(output);
}

fn cloc_body<S: Space>(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = S::adaptor(r)?;
    let n = r.integer();
    for _ in 0..n {
        let p = S::point(r);
        let nb = r.integer();
        let (umin, usup) = (r.real(), r.real());
        attempt(output, || {
            Ok(S::pon(
                output,
                GCurveLocator::<S::Tool>::locate(p, &*a, nb)?,
            ))
        });
        attempt(output, || {
            Ok(S::pon(
                output,
                GCurveLocator::<S::Tool>::locate_range(p, &*a, nb, umin, usup)?,
            ))
        });
    }
    Ok(())
}

pub fn cloc_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || cloc_body::<Space3>(output, &mut r));
    end(output);
}
pub fn cloc2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || cloc_body::<Space2>(output, &mut r));
    end(output);
}

pub fn ctool_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let a = Space3::adaptor(&mut r)?;
        flag(output, CurveTool::is_periodic(&*a)?);
        attempt(output, || Ok(real(output, CurveTool::period(&*a)?)));
        real(output, CurveTool::resolution(&*a, 1.0e-7)?);
        int(output, CurveTool::get_type(&*a)? as i32);
        int(output, CurveTool::continuity(&*a)? as i32);
        Ok(())
    });
    end(output);
}
pub fn ctool2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let a = Space2::adaptor(&mut r)?;
        flag(output, Curve2dTool::is_periodic(&*a)?);
        flag(output, Curve2dTool::is_closed(&*a)?);
        attempt(output, || Ok(real(output, Curve2dTool::period(&*a)?)));
        real(output, Curve2dTool::resolution(&*a, 1.0e-7)?);
        int(output, Curve2dTool::get_type(&*a)? as i32);
        int(output, Curve2dTool::continuity(&*a)? as i32);
        Ok(())
    });
    end(output);
}

pub fn locpc_owned_case(output: &Output, values: &[f64]) {
    let mut reader = Reader::new(values);
    attempt(output, || locpc_body::<Space3>(output, &mut reader, true));
    end(output);
}
pub fn locpc2d_owned_case(output: &Output, values: &[f64]) {
    let mut reader = Reader::new(values);
    attempt(output, || locpc_body::<Space2>(output, &mut reader, true));
    end(output);
}
