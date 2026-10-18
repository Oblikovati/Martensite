//! Owned MathBase numeric request driver, part 1; the counterpart of
//! the implementation.
use super::print::{Output, Reader, end, int, real};
use martensite_tkgeombase::adv_app2var::math_base as mb;

pub fn reals(r: &mut Reader<'_>, n: i32) -> Vec<f64> {
    read_reals(n, || r.real())
}
pub fn read_reals(n: i32, mut next: impl FnMut() -> f64) -> Vec<f64> {
    (0..n.max(0)).map(|_| next()).collect()
}
pub fn fill(n: i32) -> Vec<f64> {
    vec![7.5; n.max(1) as usize]
}
pub fn print(output: &Output, values: &[f64]) {
    values.iter().for_each(|v| real(output, *v));
}
fn epsilons(output: &Output) {
    let (mut e1, mut e3) = (0.0, 0.0);
    mb::mmeps1_(&mut e1);
    mb::mmveps3_(&mut e3);
    real(output, e1);
    real(output, e3);
}

pub fn norms_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let n = r.integer();
    let (v, w) = (reals(&mut r, n), reals(&mut r, n));
    let (x, k) = (r.real(), r.integer());
    real(output, mb::mzsnorm_(n, &v));
    real(output, mb::msc_(n, &v, &w));
    real(output, mb::pow_di(x, k));
    epsilons(output);
    end(output);
}

pub fn mmwprcs_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let e: Vec<f64> = reals(&mut r, 4);
    let (n1, n2) = (r.integer(), r.integer());
    mb::mmwprcs_(e[0], e[1], e[2], e[3], n1, n2);
    epsilons(output);
    end(output);
}

pub fn mmapcmp_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (ndim, ncofmx, ncoeff) = (r.integer(), r.integer(), r.integer());
    let old = reals(&mut r, ncofmx * ndim);
    let mut out = fill(((ncoeff - 1) / 2 + 1) * 2 * ndim);
    mb::mmapcmp_(ndim, ncofmx, ncoeff, &old, &mut out);
    print(output, &out);
    end(output);
}

pub fn mmdrc11_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (iordre, ndimen, ncoeff) = (r.integer(), r.integer(), r.integer());
    let c = reals(&mut r, ncoeff * ndimen);
    let (mut pts, mut fact) = (fill(2 * (iordre + 1) * ndimen), fill(iordre + 1));
    mb::mmdrc11_(iordre, ndimen, ncoeff, &c, &mut pts, &mut fact);
    print(output, &pts);
    print(output, &fact);
    end(output);
}

pub fn mmfmca8_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (nd, nu, nv, ndx, nux) = (
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
    );
    let inplace = r.flag();
    let ini = reals(&mut r, nd * nu * nv);
    let mut res = fill(ndx * nux * nv);
    if inplace {
        res[..ini.len()].copy_from_slice(&ini);
        let copy = res.clone();
        mb::mmfmca8_(nd, nu, nv, ndx, nux, nv, &copy, &mut res);
    } else {
        mb::mmfmca8_(nd, nu, nv, ndx, nux, nv, &ini, &mut res);
    }
    print(output, &res);
    end(output);
}

pub fn mmfmca9_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (ndx, nux, nd, nu, nv) = (
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
    );
    let inplace = r.flag();
    let mut ini = reals(&mut r, ndx * nux * nv);
    if inplace {
        let copy = ini.clone();
        mb::mmfmca9_(ndx, nux, nv, nd, nu, nv, &copy, &mut ini);
        print(output, &ini);
    } else {
        let mut res = fill(nd * nu * nv);
        mb::mmfmca9_(ndx, nux, nv, nd, nu, nv, &ini, &mut res);
        print(output, &res);
    }
    end(output);
}

pub fn mmfmcb5_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (is, ndx, n1, nc, n2, nd) = (
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
    );
    let (mut c1, mut c2) = (reals(&mut r, ndx * n1), reals(&mut r, n2 * nd));
    let mut ier = 0;
    mb::mmfmcb5_(is, ndx, n1, &mut c1, nc, n2, nd, &mut c2, &mut ier);
    int(output, ier);
    print(output, &c1);
    print(output, &c2);
    end(output);
}

pub fn mmfmtb1_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (m1, i1, j1, m2) = (r.integer(), r.integer(), r.integer(), r.integer());
    let t1 = reals(&mut r, m1 * j1);
    let mut t2 = fill(m2 * i1);
    let (mut i2, mut j2, mut ier) = (-1, -1, -1);
    mb::mmfmtb1_(m1, &t1, i1, j1, m2, &mut t2, &mut i2, &mut j2, &mut ier);
    int(output, i2);
    int(output, j2);
    int(output, ier);
    print(output, &t2);
    end(output);
}

pub fn mmfmcar_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (nd, nx, nu, nv) = (r.integer(), r.integer(), r.integer(), r.integer());
    let (u1, u2, v1, v2) = (r.real(), r.real(), r.real(), r.real());
    let old = reals(&mut r, nd * nx * nv);
    let mut new = fill(nd * nx * nv);
    let mut ier = -1;
    mb::mmfmcar_(nd, nx, nu, nv, &old, u1, u2, v1, v2, &mut new, &mut ier);
    int(output, ier);
    print(output, &new);
    end(output);
}

pub fn mmcglc1_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (ndx, nd, nc) = (r.integer(), r.integer(), r.integer());
    let (t0, t1, eps) = (r.real(), r.real(), r.real());
    let c = reals(&mut r, ndx * nc);
    let (mut len, mut err, mut ier) = (7.5, 7.5, -1);
    mb::mmcglc1_(ndx, nd, nc, &c, t0, t1, eps, &mut len, &mut err, &mut ier);
    real(output, len);
    real(output, err);
    int(output, ier);
    end(output);
}

pub fn mmbulld_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (nc, nl, key) = (r.integer(), r.integer(), r.integer());
    let mut t = reals(&mut r, nc * nl);
    mb::mmbulld_(nc, nl, &mut t, key);
    print(output, &t);
    end(output);
}

pub fn mvsheld_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (n, is, key) = (r.integer(), r.integer(), r.integer());
    let mut t = reals(&mut r, n * is);
    mb::mvsheld_(n, is, &mut t, key);
    print(output, &t);
    end(output);
}

pub fn mmcdriv_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (nd, nc, id) = (r.integer(), r.integer(), r.integer());
    let c = reals(&mut r, nd * nc);
    let (mut d, mut ncf) = (fill(nd * nc), -1);
    mb::mmcdriv_(nd, nc, &c, id, &mut ncf, &mut d);
    int(output, ncf);
    print(output, &d);
    end(output);
}

pub fn mmdrvck_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (nc, nd, id, t) = (r.integer(), r.integer(), r.integer(), r.real());
    let c = reals(&mut r, nd * nc);
    let mut p = fill(nd);
    mb::mmdrvck_(nc, nd, &c, id, t, &mut p);
    print(output, &p);
    end(output);
}

pub fn mmcvctx_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (nd, nx, nv) = (r.integer(), r.integer(), r.integer());
    let c = reals(&mut r, nd * 2 * (nv + 1));
    let (mut res, mut aux, mut x) = (
        fill(nx * nd),
        fill((nv + 1 + nd) * (nv + 1)),
        fill((nv + 1) * nd),
    );
    let mut ier = -1;
    mb::mmcvctx_(nd, nx, nv, &c, &mut res, &mut aux, &mut x, &mut ier);
    int(output, ier);
    print(output, &res);
    end(output);
}

pub fn mdsptpt_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let n = r.integer();
    let (p1, p2) = (reals(&mut r, n), reals(&mut r, n));
    let mut d = 7.5;
    mb::mdsptpt_(n, &p1, &p2, &mut d);
    real(output, d);
    end(output);
}

pub fn mmaperx_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (nx, nd, nc, io, nn) = (
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
    );
    let c = reals(&mut r, nx * nd);
    let (mut y, mut e, mut ier) = (fill(nd), 7.5, -1);
    mb::mmaperx_(nx, nd, nc, io, &c, nn, &mut y, &mut e, &mut ier);
    print(output, &y);
    real(output, e);
    int(output, ier);
    end(output);
}

pub fn mmtrpjj_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (nx, nd, nc, eps, io) = (r.integer(), r.integer(), r.integer(), r.real(), r.integer());
    let c = reals(&mut r, nx * nd);
    let (mut y, mut e, mut nn) = (fill(nd), 7.5, -1);
    mb::mmtrpjj_(nx, nd, nc, eps, io, &c, &mut y, &mut e, &mut nn);
    print(output, &y);
    real(output, e);
    int(output, nn);
    end(output);
}

pub fn mmgaus1_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (nf, k, xd, xf, kind) = (r.integer(), r.integer(), r.real(), r.real(), r.integer());
    let mut integrand = |n: i32, t: f64, f: &mut [f64], ier: &mut i32| {
        let v: [f64; 3] = [t * t * t - 2. * t, (3. * t).sin(), (-t).exp()];
        for i in 0..n as usize {
            f[i] = v[i % 3] + i as f64;
        }
        if kind == 0 {
            *ier = 0;
        }
        if kind == 2 {
            *ier = if t > 0.5 { 5 } else { 0 };
        }
    };
    let (mut s1, mut s2, mut s) = (fill(nf), fill(nf), fill(nf));
    let (mut niter, mut ier) = (-1, -7);
    mb::mmgaus1_(
        nf,
        &mut integrand,
        k,
        xd,
        xf,
        &mut s1,
        &mut s2,
        &mut s,
        &mut niter,
        &mut ier,
    );
    print(output, &s);
    int(output, niter);
    int(output, ier);
    end(output);
}
