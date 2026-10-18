//! The IntAna2d package probe; the counterpart of the implementation.
use super::common::{self as p, Reader, attempt, end, flag, int, real};
use super::print::Output;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::int_ana2d::{
    AnaIntersection, Conic, IntPoint, MyDirectPolynomialRoots, coord_ancien_repere,
    points_confondus, traitement_points_confondus,
};
use martensite_tkmath::gp::{Circle2, Ellipse2, Hyperbola2, Line2, Parabola2};

/// One 2D element of the corpus (GbConic2d).
enum Conic2d {
    Line(Line2),
    Circle(Circle2),
    Ellipse(Ellipse2),
    Parabola(Parabola2),
    Hyperbola(Hyperbola2),
}

fn conic2d(r: &mut Reader) -> Result<Conic2d, Failure> {
    Ok(match r.integer() {
        0 => {
            let point = r.pnt2d();
            Conic2d::Line(Line2::new(point, r.dir2d()))
        }
        1 => {
            let a = r.ax22d();
            Conic2d::Circle(Circle2::new(a, r.real())?)
        }
        2 => {
            let (a, major) = (r.ax22d(), r.real());
            Conic2d::Ellipse(Ellipse2::new(a, major, r.real())?)
        }
        3 => {
            let a = r.ax22d();
            Conic2d::Parabola(Parabola2::new(a, r.real())?)
        }
        _ => {
            let (a, major) = (r.ax22d(), r.real());
            Conic2d::Hyperbola(Hyperbola2::new(a, major, r.real())?)
        }
    })
}

fn implicit(element: &Conic2d) -> Conic {
    match *element {
        Conic2d::Line(l) => Conic::from_line(l),
        Conic2d::Circle(c) => Conic::from_circle(c),
        Conic2d::Ellipse(e) => Conic::from_ellipse(e),
        Conic2d::Parabola(x) => Conic::from_parabola(x),
        Conic2d::Hyperbola(h) => Conic::from_hyperbola(h),
    }
}

fn int_point(output: &Output, point: IntPoint) {
    p::pnt2d(output, point.value());
    real(output, point.param_on_first());
    flag(output, point.second_is_implicit());
    p::show(output, point.param_on_second(), |value| real(output, value));
}

fn ana(output: &Output, a: &AnaIntersection) -> Result<(), Failure> {
    flag(output, a.is_done());
    if !a.is_done() {
        p::show(output, a.nb_points(), |value| int(output, value));
        return Ok(());
    }
    flag(output, a.is_empty()?);
    flag(output, a.identical_elements()?);
    flag(output, a.parallel_elements()?);
    let n = a.nb_points()?;
    int(output, n);
    for i in 1..=n {
        int_point(output, a.point(i)?);
    }
    p::show(output, a.point(n + 1), |value| int_point(output, value));
    Ok(())
}

pub fn ia2_lines_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let point = r.pnt2d();
        let l1 = Line2::new(point, r.dir2d());
        let point = r.pnt2d();
        let l2 = Line2::new(point, r.dir2d());
        ana(output, &AnaIntersection::lines(l1, l2))
    });
    end(output);
}

pub fn ia2_circles_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let a1 = r.ax22d();
        let c1 = Circle2::new(a1, r.real())?;
        let a2 = r.ax22d();
        let c2 = Circle2::new(a2, r.real())?;
        let mut a = AnaIntersection::new();
        a.perform_circles(c1, c2)?;
        ana(output, &a)
    });
    end(output);
}

pub fn ia2_line_circle_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let point = r.pnt2d();
        let l = Line2::new(point, r.dir2d());
        let a1 = r.ax22d();
        let c = Circle2::new(a1, r.real())?;
        let mut a = AnaIntersection::new();
        a.perform_line_circle(l, c)?;
        ana(output, &a)
    });
    end(output);
}

pub fn ia2_conic_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let first = conic2d(&mut r)?;
        let conic = implicit(&conic2d(&mut r)?);
        let mut a = AnaIntersection::new();
        match first {
            Conic2d::Line(l) => a.perform_line_conic(l, &conic)?,
            Conic2d::Circle(c) => a.perform_circle_conic(c, &conic)?,
            Conic2d::Ellipse(e) => a.perform_ellipse_conic(e, &conic)?,
            Conic2d::Parabola(x) => a.perform_parabola_conic(x, &conic)?,
            Conic2d::Hyperbola(h) => a.perform_hyperbola_conic(h, &conic)?,
        }
        ana(output, &a)
    });
    end(output);
}

pub fn ia2_conic_eval_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let conic = implicit(&conic2d(&mut r)?);
        let (x, y) = (r.real(), r.real());
        let axis = r.ax2d();
        conic
            .coefficients()
            .into_iter()
            .for_each(|value| real(output, value));
        real(output, conic.value(x, y));
        p::xy(output, conic.grad(x, y));
        let (value, gradient) = conic.val_and_grad(x, y);
        real(output, value);
        p::xy(output, gradient);
        conic
            .new_coefficients(conic.coefficients(), axis)
            .into_iter()
            .for_each(|value| real(output, value));
        Ok(())
    });
    end(output);
}

pub fn ia2_roots_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let roots = if r.integer() == 0 {
            let (a4, a3, a2, a1) = (r.real(), r.real(), r.real(), r.real());
            MyDirectPolynomialRoots::quartic(a4, a3, a2, a1, r.real())?
        } else {
            let (a2, a1) = (r.real(), r.real());
            MyDirectPolynomialRoots::quadratic(a2, a1, r.real())?
        };
        flag(output, roots.is_done());
        flag(output, roots.infinite_roots());
        let n = roots.nb_solutions();
        int(output, n);
        for i in 1..=n {
            real(output, roots.value(i)?);
        }
        Ok(())
    });
    end(output);
}

pub fn ia2_outils_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (x1, y1, x2, y2) = (r.real(), r.real(), r.real(), r.real());
    flag(output, points_confondus(x1, y1, x2, y2));
    let (x, y) = (r.real(), r.real());
    let (x, y) = coord_ancien_repere(x, y, r.ax2d());
    real(output, x);
    real(output, y);
    let n = r.integer();
    let mut pts = [IntPoint::default(); 4];
    for point in pts.iter_mut().take(n as usize) {
        let (px, py, u1) = (r.real(), r.real(), r.real());
        point.set_value(px, py, u1, r.real());
    }
    int(output, traitement_points_confondus(n, &mut pts));
    pts.into_iter().for_each(|value| int_point(output, value));
    end(output);
}

pub fn ia2_int_point_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (x, y, u1, u2) = (r.real(), r.real(), r.real(), r.real());
    let mut d = IntPoint::default();
    int_point(output, d);
    int_point(output, IntPoint::new(x, y, u1, u2));
    int_point(output, IntPoint::implicit(x, y, u1));
    d.set_implicit_value(y, x, u2);
    int_point(output, d);
    d.set_value(y, x, u2, u1);
    int_point(output, d);
    let empty = AnaIntersection::new();
    attempt(output, || ana(output, &empty));
    p::show(output, empty.is_empty(), |value| flag(output, value));
    p::show(output, empty.identical_elements(), |value| {
        flag(output, value)
    });
    p::show(output, empty.parallel_elements(), |value| {
        flag(output, value)
    });
    p::show(output, empty.point(1), |value| int_point(output, value));
    end(output);
}
