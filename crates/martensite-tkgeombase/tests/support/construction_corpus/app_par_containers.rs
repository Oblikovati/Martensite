//! The Rust twin of the implementation.
use super::common::{self as p, Reader, attempt, end};
use super::linear_tokens::{dump, ivector, read_vector, vector};
use super::{
    fixture_input::GeometryFixtureInput,
    matrix_tokens::matrix,
    multi_curve_tokens::{curve, point},
    print::Output,
};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::app_par_curves::{
    self as apc, MultiBSpCurve, MultiCurve, MultiCurveEval, MultiPoint,
};
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tkmath::math::{IntegerVector, Matrix, Vector};

pub fn bernstein_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let nbpoles = r.integer();
    let u = read_vector(&mut r);
    let ddalen = r.integer();
    attempt(output, || {
        let new = || Matrix::filled(u.lower(), u.upper(), 1, nbpoles, -7.).unwrap();
        let (mut a, mut a2, mut da) = (new(), new(), new());
        apc::bernstein_matrix(nbpoles, &u, &mut a)?;
        matrix(output, &a);
        apc::bernstein(nbpoles, &u, &mut a2, &mut da)?;
        matrix(output, &a2);
        matrix(output, &da);
        for i in u.lower()..=u.upper() {
            let mut dda = Vector::filled(1, ddalen, -7.)?;
            apc::second_derivative_bernstein(u[i], &mut dda)?;
            vector(output, &dda);
        }
        Ok(())
    });
    end(output);
}

pub fn spline_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (nbpoles, deg) = (r.integer(), r.integer());
    let (u, flat) = (read_vector(&mut r), read_vector(&mut r));
    attempt(output, || {
        let mut a = Matrix::filled(u.lower(), u.upper(), 1, nbpoles, -7.).unwrap();
        let mut da = a.clone();
        let mut index = IntegerVector::filled(u.lower(), u.upper(), -7)?;
        apc::spline_function(nbpoles, deg, &u, &flat, &mut a, &mut da, &mut index)?;
        matrix(output, &a);
        matrix(output, &da);
        ivector(output, &index);
        Ok(())
    });
    end(output);
}

pub fn read_point(r: &mut impl GeometryFixtureInput) -> Result<MultiPoint, Failure> {
    let (kind, nb3d, nb2d) = (r.integer(), r.integer(), r.integer());
    let p3: Vec<Point3> = (0..nb3d).map(|_| r.pnt()).collect();
    let p2: Vec<Point2> = (0..nb2d).map(|_| r.pnt2d()).collect();
    if kind == 0 {
        let m = MultiPoint::with_counts(nb3d, nb2d)?;
        for (point, i) in p3.iter().zip(1..) {
            m.set_point(i, *point)?;
        }
        for (point, i) in p2.iter().zip(nb3d + 1..) {
            m.set_point2d(i, *point)?;
        }
        return Ok(m);
    }
    let (a3, a2) = (Array1::from_shared(&p3, 1), Array1::from_shared(&p2, 1));
    Ok(match kind {
        1 => MultiPoint::from_points(&a3),
        2 => MultiPoint::from_points2d(&a2),
        _ => MultiPoint::from_points_and_2d(&a3, &a2),
    })
}

fn point_command(output: &Output, r: &mut Reader<'_>, m: &MultiPoint) -> Result<(), Failure> {
    match r.integer() {
        1 => {
            let i = r.integer();
            m.set_point(i, r.pnt())?;
        }
        2 => p::pnt(output, m.point(r.integer())?),
        3 => {
            let i = r.integer();
            m.set_point2d(i, r.pnt2d())?;
        }
        4 => p::pnt2d(output, m.point2d(r.integer())?),
        5 => p::int(output, m.dimension(r.integer())?),
        6 => {
            let i = r.integer();
            let c = [r.real(), r.real(), r.real(), r.real(), r.real(), r.real()];
            m.transform(i, c)?;
        }
        7 => {
            let i = r.integer();
            let c = [r.real(), r.real(), r.real(), r.real()];
            m.transform2d(i, c)?;
        }
        8 => {
            let mut text = String::new();
            m.dump(&mut text).unwrap();
            dump(output, &text);
        }
        _ => {
            let copy = m.clone();
            let i = r.integer();
            copy.set_point(i, r.pnt())?;
            point(output, m)?;
        }
    }
    Ok(())
}

pub fn multipoint_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let m = read_point(&mut r)?;
        point(output, &m)?;
        for _ in 0..r.integer() {
            attempt(output, || point_command(output, &mut r, &m));
        }
        Ok(())
    });
    end(output);
}

/// A curve the probe commands act on: the virtual members through MultiCurveEval.
pub trait ProbeCurve: MultiCurveEval {
    fn base(&self) -> &MultiCurve;
    fn base_mut(&mut self) -> &mut MultiCurve;
}
impl ProbeCurve for MultiCurve {
    fn base(&self) -> &MultiCurve {
        self
    }
    fn base_mut(&mut self) -> &mut MultiCurve {
        self
    }
}
impl ProbeCurve for MultiBSpCurve {
    fn base(&self) -> &MultiCurve {
        self
    }
    fn base_mut(&mut self) -> &mut MultiCurve {
        self
    }
}

fn evaluate(
    output: &Output,
    r: &mut Reader<'_>,
    c: &dyn ProbeCurve,
    code: i32,
) -> Result<(), Failure> {
    let (k, u) = (r.integer(), r.real());
    if c.base().dimension(k)? == 3 {
        let (pt, v1, v2) = match code {
            1 => (c.value_3d(k, u)?, Default::default(), Default::default()),
            2 => {
                let (pt, v1) = c.d1_3d(k, u)?;
                (pt, v1, Default::default())
            }
            _ => c.d2_3d(k, u)?,
        };
        p::pnt(output, pt);
        p::vec(output, v1);
        p::vec(output, v2);
        return Ok(());
    }
    let (pt, v1, v2) = match code {
        1 => (c.value_2d(k, u)?, Default::default(), Default::default()),
        2 => {
            let (pt, v1) = c.d1_2d(k, u)?;
            (pt, v1, Default::default())
        }
        _ => c.d2_2d(k, u)?,
    };
    p::pnt2d(output, pt);
    p::vec2d(output, v1);
    p::vec2d(output, v2);
    Ok(())
}

fn curve_command(
    output: &Output,
    r: &mut Reader<'_>,
    c: &mut dyn ProbeCurve,
) -> Result<(), Failure> {
    match r.integer() {
        code @ 1..=3 => evaluate(output, r, c, code)?,
        4 => p::int(output, c.base().dimension(r.integer())?),
        5 => {
            let (k, dim) = (r.integer(), r.integer());
            let n = c.nb_poles();
            if dim == 3 {
                let mut t: Array1<'static, Point3> = Array1::with_bounds(1, n).unwrap();
                c.base().curve_3d(k, &mut t)?;
                t.iter().for_each(|q| p::pnt(output, *q));
            } else {
                let mut t: Array1<'static, Point2> = Array1::with_bounds(1, n).unwrap();
                c.base().curve_2d(k, &mut t)?;
                t.iter().for_each(|q| p::pnt2d(output, *q));
            }
        }
        6 => {
            let (k, n, dim) = (r.integer(), r.integer(), r.integer());
            if dim == 3 {
                p::pnt(output, c.base().pole(k, n)?);
            } else {
                p::pnt2d(output, c.base().pole2d(k, n)?);
            }
        }
        7 => {
            let k = r.integer();
            let v = [r.real(), r.real(), r.real(), r.real(), r.real(), r.real()];
            c.base().transform(k, v)?;
        }
        8 => {
            let k = r.integer();
            let v = [r.real(), r.real(), r.real(), r.real()];
            c.base().transform2d(k, v)?;
        }
        9 => {
            let mut text = String::new();
            c.dump(&mut text).map_err(|_| Failure::new("dump"))?;
            dump(output, &text);
        }
        10 => p::int(output, c.degree()?),
        11 => {
            let i = r.integer();
            let m = read_point(r)?;
            c.base().set_value(i, &m)?;
        }
        _ => c.base_mut().set_nb_poles(r.integer())?,
    }
    Ok(())
}

fn curve_commands(output: &Output, r: &mut Reader<'_>, c: &mut dyn ProbeCurve) {
    for _ in 0..r.integer() {
        attempt(output, || curve_command(output, r, c));
    }
}

pub fn read_points(
    r: &mut impl GeometryFixtureInput,
) -> Result<Array1<'static, MultiPoint>, Failure> {
    let n = r.integer();
    let points: Vec<MultiPoint> = (0..n).map(|_| read_point(r)).collect::<Result<_, _>>()?;
    let mut array: Array1<'static, MultiPoint> = Array1::with_bounds(1, n).unwrap();
    array.iter_mut().zip(points).for_each(|(slot, m)| *slot = m);
    Ok(array)
}

pub fn multicurve_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let kind = r.integer();
    attempt(output, || {
        if kind == 0 {
            let mut c = MultiCurve::with_nb_poles(r.integer())?;
            p::int(output, c.nb_poles());
            p::int(output, c.nb_curves()?);
            curve_commands(output, &mut r, &mut c);
            return Ok(());
        }
        let poles = read_points(&mut r)?;
        if kind == 1 {
            let mut c = MultiCurve::from_multi_points(&poles);
            curve(output, &c)?;
            curve_commands(output, &mut r, &mut c);
            return Ok(());
        }
        let knots_vector = read_vector(&mut r);
        let knot_values: Vec<f64> = (knots_vector.lower()..=knots_vector.upper())
            .map(|i| knots_vector[i])
            .collect();
        let knots = Array1::from_shared(&knot_values, knots_vector.lower());
        let mults: Vec<i32> = (0..r.integer()).map(|_| r.integer()).collect();
        let mults = Array1::from_shared(&mults, 1);
        let mut c = if kind == 2 {
            MultiBSpCurve::from_multi_points(&poles, &knots, &mults)
        } else {
            MultiBSpCurve::from_multi_curve(&MultiCurve::from_multi_points(&poles), &knots, &mults)
        };
        curve(output, &c)?;
        p::int(output, MultiCurveEval::degree(&c)?);
        let k = c.knots()?;
        p::int(output, k.lower());
        p::int(output, k.upper());
        k.iter().for_each(|v| p::real(output, *v));
        c.multiplicities()?.iter().for_each(|m| p::int(output, *m));
        if kind == 3 {
            c.set_knots(&knots);
            c.set_multiplicities(&mults);
            p::int(output, MultiCurveEval::degree(&c)?);
        }
        curve_commands(output, &mut r, &mut c);
        Ok(())
    });
    end(output);
}
