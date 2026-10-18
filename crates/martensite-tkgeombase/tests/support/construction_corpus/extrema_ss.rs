//! The Extrema surface-surface requests; the counterpart of
//! the implementation (same encoding and token order).
#![allow(clippy::unit_arg, clippy::type_complexity)]
use super::common::{Reader, attempt, end, flag, int, real};
use super::extrema_el::ponsurf;
use super::extrema_ps::surface;
use super::print::Output;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkgeombase::extrema::{ExtSS, FuncExtSS, GenExtSS};
use martensite_tkmath::math::{Matrix, Vector};

fn vector(values: &[f64]) -> Vector<'static> {
    let mut v = Vector::with_bounds(1, values.len() as i32).unwrap();
    for (i, x) in values.iter().enumerate() {
        v[i as i32 + 1] = *x;
    }
    v
}

fn matrix(output: &Output, m: &Matrix<'_>) {
    for i in 1..=4 {
        for j in 1..=4 {
            real(output, m[(i, j)]);
        }
    }
}

fn fess_step(output: &Output, f: &mut FuncExtSS<'_>, x: &Vector<'_>) {
    let mut m = Matrix::filled(1, 4, 1, 4, 0.0).unwrap();
    attempt(output, || {
        let mut v = vector(&[0.0; 4]);
        flag(output, f.value(x, &mut v)?);
        (1..=4).for_each(|i| real(output, v[i]));
        Ok(())
    });
    attempt(output, || {
        flag(output, f.derivatives(x, &mut m)?);
        matrix(output, &m);
        Ok(())
    });
    attempt(output, || {
        let mut w = vector(&[0.0; 4]);
        flag(output, f.values(x, &mut w, &mut m)?);
        (1..=4).for_each(|i| real(output, w[i]));
        matrix(output, &m);
        Ok(())
    });
    attempt(output, || Ok(int(output, f.get_state_number()?)));
}

fn fess_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = surface(r)?;
    let b = surface(r)?;
    let n = r.integer();
    let mut d = FuncExtSS::new();
    attempt(output, || {
        let mut f0 = vector(&[0.0; 4]);
        Ok(flag(output, d.value(&vector(&[0.5; 4]), &mut f0)?))
    });
    attempt(output, || Ok(int(output, d.get_state_number()?)));
    int(output, d.nb_ext());
    attempt(output, || Ok(real(output, d.square_distance(1)?)));
    attempt(output, || Ok(ponsurf(output, d.point_on_s1(1)?)));
    attempt(output, || Ok(ponsurf(output, d.point_on_s2(1)?)));
    let mut f = FuncExtSS::with_surfaces(&a, &b);
    int(output, f.nb_variables());
    int(output, f.nb_equations());
    attempt(output, || Ok(int(output, f.get_state_number()?)));
    for _ in 0..n {
        let x = vector(&[r.real(), r.real(), r.real(), r.real()]);
        fess_step(output, &mut f, &x);
    }
    int(output, f.nb_ext());
    for i in 0..=f.nb_ext() + 1 {
        attempt(output, || {
            real(output, f.square_distance(i)?);
            ponsurf(output, f.point_on_s1(i)?);
            ponsurf(output, f.point_on_s2(i)?);
            Ok(())
        });
    }
    f.initialize(&b, &a);
    int(output, f.nb_ext());
    Ok(())
}

pub fn fess_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || fess_body(output, &mut r));
    end(output);
}

fn genss_result(output: &Output, e: &GenExtSS<'_>) {
    flag(output, e.is_done());
    attempt(output, || {
        let n = e.nb_ext()?;
        int(output, n);
        for i in 1..=n {
            real(output, e.square_distance(i)?);
            ponsurf(output, e.point_on_s1(i)?);
            ponsurf(output, e.point_on_s2(i)?);
        }
        Ok(())
    });
    attempt(output, || Ok(real(output, e.square_distance(0)?)));
}

fn ranges(r: &mut Reader) -> ([f64; 4], [f64; 4]) {
    let p: Vec<f64> = (0..8).map(|_| r.real()).collect();
    ([p[0], p[1], p[2], p[3]], [p[4], p[5], p[6], p[7]])
}

fn genss_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = surface(r)?;
    let b = surface(r)?;
    let (nu, nv) = (r.integer(), r.integer());
    let (t1, t2) = (r.real(), r.real());
    let (p1, p2) = ranges(r);
    attempt(output, || {
        genss_result(output, &GenExtSS::with_surfaces(&a, &b, nu, nv, t1, t2)?);
        Ok(())
    });
    attempt(output, || {
        genss_result(
            output,
            &GenExtSS::with_ranges(&a, &b, (nu, nv), p1, p2, (t1, t2))?,
        );
        Ok(())
    });
    let mut d = GenExtSS::new();
    flag(output, d.is_done());
    attempt(output, || Ok(int(output, d.nb_ext()?)));
    attempt(output, || {
        d.initialize_range(&b, (nu, nv), (p2[0], p2[1]), (p2[2], p2[3]), t2)?;
        d.perform_range(&a, (p1[0], p1[1]), (p1[2], p1[3]), t1)?;
        genss_result(output, &d);
        d.perform(&a, t1)?;
        genss_result(output, &d);
        Ok(())
    });
    Ok(())
}

pub fn genss_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || genss_body(output, &mut r));
    end(output);
}

fn ss_result(output: &Output, e: &ExtSS<'_>) {
    flag(output, e.is_done());
    attempt(output, || Ok(flag(output, e.is_parallel()?)));
    attempt(output, || {
        let n = e.nb_ext()?;
        int(output, n);
        for i in 1..=n {
            real(output, e.square_distance(i)?);
            attempt(output, || {
                let (p, q) = e.points(i)?;
                ponsurf(output, p);
                ponsurf(output, q);
                Ok(())
            });
        }
        Ok(())
    });
    attempt(output, || Ok(real(output, e.square_distance(0)?)));
}

fn ss_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = surface(r)?;
    let b = surface(r)?;
    let (t1, t2) = (r.real(), r.real());
    let (p1, p2) = ranges(r);
    attempt(output, || {
        ss_result(output, &ExtSS::with_surfaces(&a, &b, t1, t2)?);
        Ok(())
    });
    attempt(output, || {
        ss_result(output, &ExtSS::with_ranges(&a, &b, p1, p2, (t1, t2))?);
        Ok(())
    });
    let mut d = ExtSS::new();
    flag(output, d.is_done());
    attempt(output, || Ok(int(output, d.nb_ext()?)));
    attempt(output, || {
        d.initialize(&b, p2, t2)?;
        d.perform(&a, p1, t1)?;
        ss_result(output, &d);
        let bounds = [
            a.first_u_parameter()?,
            a.last_u_parameter()?,
            a.first_v_parameter()?,
            a.last_v_parameter()?,
        ];
        d.perform(&a, bounds, t1)?;
        ss_result(output, &d);
        Ok(())
    });
    Ok(())
}

pub fn ss_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || ss_body(output, &mut r));
    end(output);
}
