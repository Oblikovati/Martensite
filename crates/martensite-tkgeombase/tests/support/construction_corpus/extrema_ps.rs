//! The Extrema point-surface requests; the counterpart of
//! the implementation (same encoding and token order).
#![allow(clippy::unit_arg, clippy::type_complexity)]
use super::bndlib;
use super::common::{Reader, attempt, end, flag, int, pnt, real};
use super::extrema_el::ponsurf;
use super::fixture_input::GeometryFixtureInput;
use super::print::Output;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::geom_adaptor::GeomAdaptorSurface;
use martensite_tkgeombase::extrema::{
    ExtAlgo, ExtFlag, ExtPExtS, ExtPRevS, ExtPS, FuncPSDist, FuncPSNorm, GenExtPS, GenLocateExtPS,
    POnSurf, Sampling,
};
use martensite_tkmath::gp::Vector3;
use martensite_tkmath::math::{Matrix, Vector};

/// gb_ex_surface: a surface spec, a ranged flag and the adaptor bounds.
pub fn surface(r: &mut impl GeometryFixtureInput) -> Result<GeomAdaptorSurface, Failure> {
    let s = bndlib::surface(r)?;
    if r.flag() {
        let bounds = (r.real(), r.real(), r.real(), r.real());
        return GeomAdaptorSurface::with_bounds(&s, bounds);
    }
    GeomAdaptorSurface::with_surface(&s)
}

/// The accessors of a result with NbExt, SquareDistance(N) and Point(N).
pub(super) trait PsView {
    fn done(&self) -> bool;
    fn count(&self) -> Result<i32, Failure>;
    fn sq(&self, n: i32) -> Result<f64, Failure>;
    fn pon(&self, n: i32) -> Result<POnSurf, Failure>;
}

macro_rules! ps_view {
    ($t:ty) => {
        impl PsView for $t {
            fn done(&self) -> bool {
                self.is_done()
            }
            fn count(&self) -> Result<i32, Failure> {
                self.nb_ext()
            }
            fn sq(&self, n: i32) -> Result<f64, Failure> {
                self.square_distance(n)
            }
            fn pon(&self, n: i32) -> Result<POnSurf, Failure> {
                self.point(n)
            }
        }
    };
}
ps_view!(GenExtPS<'_>);
ps_view!(ExtPS<'_>);
ps_view!(ExtPExtS);
ps_view!(ExtPRevS);

pub(super) fn ps_result(output: &Output, e: &impl PsView) {
    flag(output, e.done());
    attempt(output, || {
        let n = e.count()?;
        int(output, n);
        for i in 1..=n {
            real(output, e.sq(i)?);
            ponsurf(output, e.pon(i)?);
        }
        Ok(())
    });
    attempt(output, || Ok(real(output, e.sq(0)?)));
}

fn vec2(a: f64, b: f64) -> Vector<'static> {
    let mut v = Vector::with_bounds(1, 2).unwrap();
    v[1] = a;
    v[2] = b;
    v
}

fn matrix(output: &Output, m: &Matrix<'_>) {
    for i in 1..=2 {
        for j in 1..=2 {
            real(output, m[(i, j)]);
        }
    }
}

fn fpsn_step(output: &Output, f: &mut FuncPSNorm<'_>, uv: &Vector<'_>) {
    let mut m = Matrix::filled(1, 2, 1, 2, 0.0).unwrap();
    attempt(output, || {
        let mut v = vec2(0.0, 0.0);
        flag(output, f.value(uv, &mut v)?);
        real(output, v[1]);
        real(output, v[2]);
        Ok(())
    });
    attempt(output, || {
        flag(output, f.derivatives(uv, &mut m)?);
        matrix(output, &m);
        Ok(())
    });
    attempt(output, || {
        let mut w = vec2(0.0, 0.0);
        flag(output, f.values(uv, &mut w, &mut m)?);
        real(output, w[1]);
        real(output, w[2]);
        matrix(output, &m);
        Ok(())
    });
    attempt(output, || Ok(int(output, f.get_state_number()?)));
}

fn fpsn_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let s = surface(r)?;
    let p = r.pnt();
    let n = r.integer();
    let mut d = FuncPSNorm::new();
    attempt(output, || {
        let mut f0 = vec2(0.0, 0.0);
        Ok(flag(output, d.value(&vec2(0.5, 0.5), &mut f0)?))
    });
    attempt(output, || Ok(int(output, d.get_state_number()?)));
    int(output, d.nb_ext());
    attempt(output, || Ok(real(output, d.square_distance(1)?)));
    attempt(output, || Ok(ponsurf(output, d.point(1)?)));
    let mut f = FuncPSNorm::with_point_surface(p, &s);
    int(output, f.nb_variables());
    int(output, f.nb_equations());
    for _ in 0..n {
        let uv = vec2(r.real(), r.real());
        fpsn_step(output, &mut f, &uv);
    }
    int(output, f.nb_ext());
    for i in 0..=f.nb_ext() + 1 {
        attempt(output, || {
            real(output, f.square_distance(i)?);
            ponsurf(output, f.point(i)?);
            Ok(())
        });
    }
    f.set_point(p.translated(Vector3::new(1.0, 0.0, 0.0)));
    int(output, f.nb_ext());
    attempt(output, || {
        let mut v = vec2(0.0, 0.0);
        flag(output, f.value(&vec2(0.25, 0.25), &mut v)?);
        int(output, f.get_state_number()?);
        int(output, f.nb_ext());
        Ok(())
    });
    f.initialize(&s);
    int(output, f.nb_ext());
    Ok(())
}

pub fn fpsn_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || fpsn_body(output, &mut r));
    end(output);
}

fn fpsd_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let s = surface(r)?;
    let p = r.pnt();
    let n = r.integer();
    let f = FuncPSDist::new(&s, p);
    int(output, f.nb_variables());
    for _ in 0..n {
        let x = vec2(r.real(), r.real());
        attempt(output, || {
            let mut v = 0.0;
            flag(output, f.value(&x, &mut v)?);
            real(output, v);
            Ok(())
        });
        let mut g = vec2(0.0, 0.0);
        attempt(output, || {
            flag(output, f.gradient(&x, &mut g)?);
            real(output, g[1]);
            real(output, g[2]);
            Ok(())
        });
        attempt(output, || {
            let mut v = 0.0;
            let mut g = vec2(0.0, 0.0);
            flag(output, f.values(&x, &mut v, &mut g)?);
            real(output, v);
            real(output, g[1]);
            real(output, g[2]);
            Ok(())
        });
    }
    Ok(())
}

pub fn fpsd_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || fpsd_body(output, &mut r));
    end(output);
}

fn glps_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let s = surface(r)?;
    let p = r.pnt();
    let (tu, tv) = (r.real(), r.real());
    let n = r.integer();
    let conf = martensite_tkernel::precision::p_confusion();
    let mut e = GenLocateExtPS::new(&s, tu, tv);
    let mut d = GenLocateExtPS::new(&s, conf, conf);
    flag(output, e.is_done());
    attempt(output, || Ok(real(output, e.square_distance()?)));
    attempt(output, || Ok(ponsurf(output, e.point()?)));
    for _ in 0..n {
        let (u0, v0) = (r.real(), r.real());
        for crit in [false, true] {
            attempt(output, || {
                e.perform(p, u0, v0, crit)?;
                flag(output, e.is_done());
                attempt(output, || {
                    real(output, e.square_distance()?);
                    ponsurf(output, e.point()?);
                    Ok(())
                });
                Ok(())
            });
        }
        attempt(output, || {
            d.perform(p, u0, v0, false)?;
            flag(output, d.is_done());
            attempt(output, || Ok(real(output, d.square_distance()?)));
            Ok(())
        });
        attempt(output, || {
            Ok(flag(output, GenLocateExtPS::is_min_dist(p, &s, u0, v0)?))
        });
    }
    Ok(())
}

pub fn glps_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || glps_body(output, &mut r));
    end(output);
}

fn ext_flag(value: i32) -> ExtFlag {
    match value {
        0 => ExtFlag::Min,
        1 => ExtFlag::Max,
        _ => ExtFlag::MinMax,
    }
}

fn ext_algo(value: i32) -> ExtAlgo {
    if value == 0 {
        ExtAlgo::Grad
    } else {
        ExtAlgo::Tree
    }
}

fn other_algo(algo: ExtAlgo) -> ExtAlgo {
    if algo == ExtAlgo::Grad {
        ExtAlgo::Tree
    } else {
        ExtAlgo::Grad
    }
}

fn genps_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let s = surface(r)?;
    let p = r.pnt();
    let (nu, nv) = (r.integer(), r.integer());
    let (tu, tv) = (r.real(), r.real());
    let (fl, al) = (ext_flag(r.integer()), ext_algo(r.integer()));
    let (u1, u2, v1, v2) = (r.real(), r.real(), r.real(), r.real());
    let p2 = r.pnt();
    let sampling = Sampling {
        nb_u: nu,
        nb_v: nv,
        tol_u: tu,
        tol_v: tv,
    };
    attempt(output, || {
        let mut e = GenExtPS::with_point(p, &s, sampling, fl, al)?;
        ps_result(output, &e);
        attempt(output, || {
            e.perform(p2)?;
            ps_result(output, &e);
            Ok(())
        });
        e.set_flag(ExtFlag::MinMax);
        attempt(output, || {
            e.perform(p)?;
            ps_result(output, &e);
            Ok(())
        });
        Ok(())
    });
    attempt(output, || {
        let mut e = GenExtPS::with_range(p, &s, sampling, (u1, u2), (v1, v2), (fl, al))?;
        ps_result(output, &e);
        e.set_algo(other_algo(al));
        attempt(output, || {
            e.perform(p2)?;
            ps_result(output, &e);
            Ok(())
        });
        e.set_algo(al);
        attempt(output, || {
            e.perform(p)?;
            ps_result(output, &e);
            Ok(())
        });
        Ok(())
    });
    let mut d = GenExtPS::new();
    flag(output, d.is_done());
    attempt(output, || Ok(int(output, d.nb_ext()?)));
    attempt(output, || {
        d.initialize(
            &s,
            Sampling {
                nb_u: 1,
                ..sampling
            },
        )
    });
    Ok(())
}

pub fn genps_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || genps_body(output, &mut r));
    end(output);
}

fn ps_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let s = surface(r)?;
    let p = r.pnt();
    let (tu, tv) = (r.real(), r.real());
    let (fl, al) = (ext_flag(r.integer()), ext_algo(r.integer()));
    let (u1, u2, v1, v2) = (r.real(), r.real(), r.real(), r.real());
    let p2 = r.pnt();
    attempt(output, || {
        let mut e = ExtPS::with_surface(p, &s, tu, tv, fl, al)?;
        ps_result(output, &e);
        attempt(output, || {
            e.perform(p2)?;
            ps_result(output, &e);
            Ok(())
        });
        Ok(())
    });
    attempt(output, || {
        let mut e = ExtPS::with_range(p, &s, (u1, u2), (v1, v2), (tu, tv), (fl, al))?;
        ps_result(output, &e);
        e.set_flag(ExtFlag::MinMax);
        e.set_algo(other_algo(al));
        attempt(output, || {
            e.perform(p2)?;
            ps_result(output, &e);
            Ok(())
        });
        Ok(())
    });
    let mut d = ExtPS::new();
    flag(output, d.is_done());
    attempt(output, || Ok(int(output, d.nb_ext()?)));
    attempt(output, || {
        d.initialize(&s, (u1, u2), (v1, v2), (tu, tv))?;
        d.perform(p)?;
        ps_result(output, &d);
        let (t, q) = d.trimmed_square_distances();
        for i in 0..4 {
            real(output, t[i]);
            pnt(output, q[i]);
        }
        Ok(())
    });
    Ok(())
}

pub fn ps_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || ps_body(output, &mut r));
    end(output);
}
