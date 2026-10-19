//! The Extrema requests on swept surfaces (ExtPExtS, ExtPRevS); the counterpart of the
//! swept part of the implementation (same encoding and
//! token order).
#![allow(clippy::unit_arg, clippy::type_complexity)]
use super::common::{Reader, attempt, end, flag, int};
use super::extrema_pc::{Space, Space3};
use super::extrema_ps::{PsView, ps_result};
use super::print::Output;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom_adaptor::{
    GeomAdaptorSurfaceOfLinearExtrusion, GeomAdaptorSurfaceOfRevolution,
};
use martensite_tkgeombase::extrema::{ExtPExtS, ExtPRevS};
use martensite_tkmath::gp::Point3;

/// gb_ex_basis_curve: a curve adaptor (gb_ex_adaptor) as a handle.
fn basis_curve(r: &mut Reader) -> Result<Handle<dyn Adaptor3dCurve>, Failure> {
    Space3::adaptor(r)
}

/// The members gb_ex_swept_runs uses.
trait Swept: PsView + Sized {
    type Surface: martensite_tkernel::standard::RuntimeTyped;
    fn with_surface(
        p: Point3,
        s: &Handle<Self::Surface>,
        tu: f64,
        tv: f64,
    ) -> Result<Self, Failure>;
    fn with_range(
        p: Point3,
        s: &Handle<Self::Surface>,
        u: (f64, f64),
        v: (f64, f64),
        t: (f64, f64),
    ) -> Result<Self, Failure>;
    fn fresh() -> Self;
    fn init(
        &mut self,
        s: &Handle<Self::Surface>,
        u: (f64, f64),
        v: (f64, f64),
        t: (f64, f64),
    ) -> Result<(), Failure>;
    fn run(&mut self, p: Point3) -> Result<(), Failure>;
}

macro_rules! swept {
    ($t:ty, $s:ty) => {
        impl Swept for $t {
            type Surface = $s;
            fn with_surface(p: Point3, s: &Handle<$s>, tu: f64, tv: f64) -> Result<Self, Failure> {
                <$t>::with_surface(p, s, tu, tv)
            }
            fn with_range(
                p: Point3,
                s: &Handle<$s>,
                u: (f64, f64),
                v: (f64, f64),
                t: (f64, f64),
            ) -> Result<Self, Failure> {
                <$t>::with_range(p, s, u, v, t)
            }
            fn fresh() -> Self {
                <$t>::new()
            }
            fn init(
                &mut self,
                s: &Handle<$s>,
                u: (f64, f64),
                v: (f64, f64),
                t: (f64, f64),
            ) -> Result<(), Failure> {
                self.initialize(s, u, v, t)
            }
            fn run(&mut self, p: Point3) -> Result<(), Failure> {
                self.perform(p)
            }
        }
    };
}
swept!(ExtPExtS, GeomAdaptorSurfaceOfLinearExtrusion);
swept!(ExtPRevS, GeomAdaptorSurfaceOfRevolution);

fn swept_runs<E: Swept>(
    output: &Output,
    s: &Handle<E::Surface>,
    (p, p2): (Point3, Point3),
    u: (f64, f64),
    v: (f64, f64),
    t: (f64, f64),
) {
    attempt(output, || {
        let mut e = E::with_surface(p, s, t.0, t.1)?;
        ps_result(output, &e);
        attempt(output, || {
            e.run(p2)?;
            ps_result(output, &e);
            Ok(())
        });
        Ok(())
    });
    attempt(output, || {
        let e = E::with_range(p, s, u, v, t)?;
        ps_result(output, &e);
        Ok(())
    });
    let mut d = E::fresh();
    flag(output, d.done());
    attempt(output, || Ok(int(output, d.count()?)));
    attempt(output, || {
        d.init(s, u, v, t)?;
        d.run(p)?;
        ps_result(output, &d);
        d.init(s, u, v, t)?;
        attempt(output, || {
            d.run(p2)?;
            ps_result(output, &d);
            Ok(())
        });
        Ok(())
    });
}

// The common tail of ex_pexts and ex_prevs: P u1 u2 v1 v2 tolu tolv P2.
fn swept_tail(r: &mut Reader) -> ((Point3, Point3), (f64, f64), (f64, f64), (f64, f64)) {
    let p = r.pnt();
    let (u1, u2, v1, v2, tu, tv) = (r.real(), r.real(), r.real(), r.real(), r.real(), r.real());
    let p2 = r.pnt();
    ((p, p2), (u1, u2), (v1, v2), (tu, tv))
}

fn pexts_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = basis_curve(r)?;
    let dir = r.dir();
    let (points, u, v, t) = swept_tail(r);
    let s = Handle::new(GeomAdaptorSurfaceOfLinearExtrusion::with_curve_and_direction(&c, dir));
    swept_runs::<ExtPExtS>(output, &s, points, u, v, t);
    Ok(())
}

pub fn pexts_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || pexts_body(output, &mut r));
    end(output);
}

fn prevs_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = basis_curve(r)?;
    let ax = r.ax1();
    let (points, u, v, t) = swept_tail(r);
    let s = Handle::new(GeomAdaptorSurfaceOfRevolution::with_curve_and_axis(&c, ax)?);
    swept_runs::<ExtPRevS>(output, &s, points, u, v, t);
    Ok(())
}

pub fn prevs_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || prevs_body(output, &mut r));
    end(output);
}
