//! The Extrema curve-surface requests; the counterpart of
//! the implementation (same encoding and token order).
#![allow(clippy::unit_arg, clippy::type_complexity)]
use super::common::{Reader, attempt, end, flag, int, real};
use super::extrema_el::{poncurv, ponsurf};
use super::extrema_pc::{Space, Space3};
use super::extrema_ps::surface;
use super::print::Output;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::extrema::{
    ExtCS, FuncExtCS, GenExtCS, GlobOptFuncCQuadric, GlobOptFuncCS, GlobOptFuncConicS,
};
use martensite_tkmath::math::{Matrix, Vector};

fn vector(values: &[f64]) -> Vector<'static> {
    let mut v = Vector::with_bounds(1, values.len() as i32).unwrap();
    for (i, x) in values.iter().enumerate() {
        v[i as i32 + 1] = *x;
    }
    v
}

fn matrix(output: &Output, m: &Matrix<'_>) {
    for i in m.lower_row()..=m.upper_row() {
        for j in m.lower_col()..=m.upper_col() {
            real(output, m[(i, j)]);
        }
    }
}

fn fecs_step(output: &Output, f: &mut FuncExtCS<'_>, x: &Vector<'_>) {
    let mut m = Matrix::filled(1, 3, 1, 3, 0.0).unwrap();
    attempt(output, || {
        let mut v = vector(&[0.0; 3]);
        flag(output, f.value(x, &mut v)?);
        (1..=3).for_each(|i| real(output, v[i]));
        Ok(())
    });
    attempt(output, || {
        flag(output, f.derivatives(x, &mut m)?);
        matrix(output, &m);
        Ok(())
    });
    attempt(output, || {
        let mut w = vector(&[0.0; 3]);
        flag(output, f.values(x, &mut w, &mut m)?);
        (1..=3).for_each(|i| real(output, w[i]));
        matrix(output, &m);
        Ok(())
    });
    attempt(output, || Ok(int(output, f.get_state_number()?)));
}

fn fecs_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = Space3::adaptor(r)?;
    let s = surface(r)?;
    let n = r.integer();
    let mut d = FuncExtCS::new();
    attempt(output, || {
        let mut f0 = vector(&[0.0; 3]);
        Ok(flag(output, d.value(&vector(&[0.5; 3]), &mut f0)?))
    });
    attempt(output, || Ok(int(output, d.get_state_number()?)));
    int(output, d.nb_ext());
    attempt(output, || Ok(real(output, d.square_distance(1)?)));
    attempt(output, || Ok(poncurv(output, d.point_on_curve(1)?)));
    attempt(output, || Ok(ponsurf(output, d.point_on_surface(1)?)));
    let mut f = FuncExtCS::with_curve_surface(&*c, &s);
    int(output, f.nb_variables());
    int(output, f.nb_equations());
    for _ in 0..n {
        let x = vector(&[r.real(), r.real(), r.real()]);
        fecs_step(output, &mut f, &x);
    }
    int(output, f.nb_ext());
    for i in 0..=f.nb_ext() + 1 {
        attempt(output, || {
            real(output, f.square_distance(i)?);
            poncurv(output, f.point_on_curve(i)?);
            ponsurf(output, f.point_on_surface(i)?);
            Ok(())
        });
    }
    f.square_distances().clear();
    int(output, f.nb_ext());
    int(output, f.points_on_curve().len() as i32);
    int(output, f.points_on_surf().len() as i32);
    f.initialize(&*c, &s);
    int(output, f.nb_ext());
    Ok(())
}

pub fn fecs_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || fecs_body(output, &mut r));
    end(output);
}

fn gofcs_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = Space3::adaptor(r)?;
    let s = surface(r)?;
    let n = r.integer();
    let f = GlobOptFuncCS::new(&*c, &s);
    int(output, f.nb_variables());
    for _ in 0..n {
        let x = vector(&[r.real(), r.real(), r.real()]);
        let mut g = vector(&[0.0; 3]);
        let mut h = Matrix::filled(1, 3, 1, 3, 0.0).unwrap();
        attempt(output, || {
            let mut v = 0.0;
            flag(output, f.value(&x, &mut v)?);
            real(output, v);
            Ok(())
        });
        attempt(output, || {
            flag(output, f.gradient(&x, &mut g)?);
            (1..=3).for_each(|i| real(output, g[i]));
            Ok(())
        });
        attempt(output, || {
            let mut v = 0.0;
            flag(output, f.values(&x, &mut v, &mut g)?);
            real(output, v);
            (1..=3).for_each(|i| real(output, g[i]));
            Ok(())
        });
        attempt(output, || {
            let mut v = 0.0;
            flag(output, f.values_hessian(&x, &mut v, &mut g, &mut h)?);
            real(output, v);
            matrix(output, &h);
            Ok(())
        });
    }
    Ok(())
}

pub fn gofcs_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || gofcs_body(output, &mut r));
    end(output);
}

fn ranges(r: &mut Reader) -> [f64; 6] {
    [r.real(), r.real(), r.real(), r.real(), r.real(), r.real()]
}

fn gofconics_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = Space3::adaptor(r)?;
    let s = surface(r)?;
    let [tf, tl, uf, ul, vf, vl] = ranges(r);
    let n = r.integer();
    let f = GlobOptFuncConicS::with_curve(&*c, &s)?;
    let mut g = GlobOptFuncConicS::with_bounds(&s, (uf, ul), (vf, vl));
    g.load_conic(&*c, tf, tl)?;
    int(output, f.nb_variables());
    for _ in 0..n {
        let x = vector(&[r.real(), r.real()]);
        for e in [&f, &g] {
            attempt(output, || {
                let mut v = 0.0;
                flag(output, e.value(&x, &mut v)?);
                real(output, v);
                Ok(())
            });
            attempt(output, || Ok(real(output, e.conic_parameter(&x)?)));
        }
    }
    Ok(())
}

pub fn gofconics_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || gofconics_body(output, &mut r));
    end(output);
}

fn gofcquad_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = Space3::adaptor(r)?;
    let s = surface(r)?;
    let [tf, tl, uf, ul, vf, vl] = ranges(r);
    let n = r.integer();
    let f = GlobOptFuncCQuadric::with_surface(&*c, &s)?;
    let mut g = GlobOptFuncCQuadric::with_range(&*c, tf, tl);
    g.load_quad(&s, (uf, ul), (vf, vl))?;
    int(output, f.nb_variables());
    for _ in 0..n {
        let x = vector(&[r.real()]);
        for e in [&f, &g] {
            attempt(output, || {
                let mut v = 0.0;
                flag(output, e.value(&x, &mut v)?);
                real(output, v);
                Ok(())
            });
            attempt(output, || {
                let mut uv = vector(&[0.0, 0.0]);
                e.quadric_parameters(&x, &mut uv)?;
                real(output, uv[1]);
                real(output, uv[2]);
                Ok(())
            });
        }
    }
    Ok(())
}

pub fn gofcquad_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || gofcquad_body(output, &mut r));
    end(output);
}

fn gencs_result(output: &Output, e: &GenExtCS<'_>) {
    flag(output, e.is_done());
    attempt(output, || {
        let n = e.nb_ext()?;
        int(output, n);
        for i in 1..=n {
            real(output, e.square_distance(i)?);
            poncurv(output, e.point_on_curve(i)?);
            ponsurf(output, e.point_on_surface(i)?);
        }
        Ok(())
    });
    attempt(output, || Ok(real(output, e.square_distance(0)?)));
}

fn genecs_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = Space3::adaptor(r)?;
    let s = surface(r)?;
    let (nt, nu, nv) = (r.integer(), r.integer(), r.integer());
    let (t1, t2) = (r.real(), r.real());
    let [a, b, u1, u2, v1, v2] = ranges(r);
    attempt(output, || {
        gencs_result(
            output,
            &GenExtCS::with_curve(&*c, &s, (nt, nu, nv), t1, t2)?,
        );
        Ok(())
    });
    attempt(output, || {
        gencs_result(
            output,
            &GenExtCS::with_ranges(&*c, &s, (nt, nu, nv), [a, b, u1, u2, v1, v2], (t1, t2))?,
        );
        Ok(())
    });
    let mut d = GenExtCS::new();
    flag(output, d.is_done());
    attempt(output, || Ok(int(output, d.nb_ext()?)));
    attempt(output, || {
        d.initialize_range(&s, (nu, nv), (u1, u2), (v1, v2), t2)?;
        d.perform_range(&*c, nt, (a, b), t1)?;
        gencs_result(output, &d);
        d.perform(&*c, nt, t1)?;
        gencs_result(output, &d);
        Ok(())
    });
    Ok(())
}

pub fn genecs_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || genecs_body(output, &mut r));
    end(output);
}

fn cs_result(output: &Output, e: &ExtCS<'_>) {
    flag(output, e.is_done());
    attempt(output, || Ok(flag(output, e.is_parallel()?)));
    attempt(output, || {
        let n = e.nb_ext()?;
        int(output, n);
        for i in 1..=n {
            real(output, e.square_distance(i)?);
            attempt(output, || {
                let (p, q) = e.points(i)?;
                poncurv(output, p);
                ponsurf(output, q);
                Ok(())
            });
        }
        Ok(())
    });
    attempt(output, || Ok(real(output, e.square_distance(0)?)));
}

fn cs_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = Space3::adaptor(r)?;
    let s = surface(r)?;
    let (tc, ts) = (r.real(), r.real());
    let [a, b, u1, u2, v1, v2] = ranges(r);
    attempt(output, || {
        cs_result(output, &ExtCS::with_curve(&*c, &s, tc, ts)?);
        Ok(())
    });
    attempt(output, || {
        cs_result(
            output,
            &ExtCS::with_ranges(&*c, &s, (a, b), (u1, u2), (v1, v2), (tc, ts))?,
        );
        Ok(())
    });
    let mut d = ExtCS::new();
    flag(output, d.is_done());
    attempt(output, || Ok(int(output, d.nb_ext()?)));
    attempt(output, || {
        d.initialize(&s, tc, ts)?;
        d.perform(&*c, a, b)?;
        cs_result(output, &d);
        d.perform(&*c, c.first_parameter()?, c.last_parameter()?)?;
        cs_result(output, &d);
        Ok(())
    });
    Ok(())
}

pub fn cs_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || cs_body(output, &mut r));
    end(output);
}
