//! The Rust twin of the implementation.
use super::common::{self as p, Reader, attempt, end};
use super::constraint_line_fixtures::read_line;
use super::fixture_input::GeometryFixtureInput;
use super::linear_tokens::{ivector, read_vector, vector};
use super::matrix_tokens::matrix;
use super::multi_curve_tokens::curve;
use super::print::Output;
use martensite_tkernel::ncollection::{Array1, HArray1};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeombase::app_def::MultiLine;
use martensite_tkgeombase::app_par_curves::{
    BSpFunction, BSpGradient, BSpGradientInput, Constraint, ConstraintCouple, Constraints,
    Function, Gradient, LeastSquare, MultiCurveEval, ResolConstraint,
};
use martensite_tkmath::math::Vector;

fn constraint(code: i32) -> Constraint {
    Constraint::from_code(code).expect("probe constraint")
}

/// `gb_apc_read_constraints`: `n` then `index constraint` couples.
pub fn read_constraints(r: &mut impl GeometryFixtureInput) -> Constraints {
    let n = r.integer();
    let mut couples = Array1::with_bounds(1, n).unwrap();
    for i in 1..=n {
        let index = r.integer();
        couples[i] = ConstraintCouple::new(index, constraint(r.integer()));
    }
    Handle::new(HArray1::from_owned(couples))
}

fn read_knots(r: &mut Reader<'_>) -> Array1<'static, f64> {
    let n = r.integer();
    let mut k = Array1::with_bounds(1, n).unwrap();
    (1..=n).for_each(|i| k[i] = r.real());
    k
}

fn read_mults(r: &mut Reader<'_>) -> Array1<'static, i32> {
    let n = r.integer();
    let mut m = Array1::with_bounds(1, n).unwrap();
    (1..=n).for_each(|i| m[i] = r.integer());
    m
}

fn ls_results(output: &Output, ls: &mut LeastSquare, bspline: bool, lambdas: i32) {
    p::flag(output, ls.is_done());
    attempt(output, || ls.function_matrix().map(|m| matrix(output, m)));
    attempt(output, || {
        ls.derivative_function_matrix().map(|m| matrix(output, m))
    });
    ivector(output, ls.k_index());
    matrix(output, ls.points());
    if !ls.is_done() {
        return;
    }
    matrix(output, ls.poles());
    attempt(output, || {
        let (f, e3, e2) = ls.error()?;
        [f, e3, e2]
            .into_iter()
            .for_each(|value| p::real(output, value));
        Ok(())
    });
    matrix(output, ls.distance());
    attempt(output, || {
        let mut g = Vector::filled(ls.points().lower_row(), ls.points().upper_row(), -7.)?;
        let (f, e3, e2) = ls.error_gradient(&mut g)?;
        vector(output, &g);
        [f, e3, e2]
            .into_iter()
            .for_each(|value| p::real(output, value));
        Ok(())
    });
    matrix(output, ls.distance());
    if lambdas & 1 != 0 {
        p::real(output, ls.first_lambda());
    }
    if lambdas & 2 != 0 {
        p::real(output, ls.last_lambda());
    }
    attempt(output, || curve(output, &ls.bezier_value()?));
    attempt(output, || {
        let c = ls.bspline_value()?;
        curve(output, c)?;
        if bspline {
            p::int(output, MultiCurveEval::degree(c)?);
        }
        Ok(())
    });
}

struct LsRequest {
    line: MultiLine,
    points: [i32; 2],
    cons: [Constraint; 2],
    nbpol: i32,
    mode: i32,
    knots: Array1<'static, f64>,
    mults: Array1<'static, i32>,
    u: Vector<'static>,
    variant: i32,
    vectors: Vec<Vector<'static>>,
    lambdas: [f64; 2],
}

fn read_ls(r: &mut Reader<'_>) -> LsRequest {
    let line = read_line(r);
    let points = [r.integer(), r.integer()];
    let cons = [constraint(r.integer()), constraint(r.integer())];
    let (nbpol, mode) = (r.integer(), r.integer());
    let (knots, mults) = if mode >= 2 {
        (read_knots(r), read_mults(r))
    } else {
        (
            Array1::with_bounds(1, 1).unwrap(),
            Array1::with_bounds(1, 1).unwrap(),
        )
    };
    let u = read_vector(r);
    let variant = r.integer();
    let count = match variant {
        2 => 2,
        3 => 4,
        _ => 0,
    };
    let vectors = (0..count).map(|_| read_vector(r)).collect();
    let lambdas = if variant > 0 {
        [r.real(), r.real()]
    } else {
        [0., 0.]
    };
    LsRequest {
        line,
        points,
        cons,
        nbpol,
        mode,
        knots,
        mults,
        u,
        variant,
        vectors,
        lambdas,
    }
}

fn run_ls(output: &Output, q: &LsRequest, lambdas: i32) -> Result<(), Failure> {
    let [fp, lp] = q.points;
    let [fc, lc] = q.cons;
    let mut ls = match q.mode {
        0 => LeastSquare::new(&q.line, fp, lp, fc, lc, &q.u, q.nbpol)?,
        1 => LeastSquare::without_parameters(&q.line, fp, lp, fc, lc, q.nbpol)?,
        2 => LeastSquare::with_knots(&q.line, &q.knots, &q.mults, fp, lp, fc, lc, &q.u, q.nbpol)?,
        _ => LeastSquare::with_knots_without_parameters(
            &q.line, &q.knots, &q.mults, fp, lp, fc, lc, q.nbpol,
        )?,
    };
    if q.mode == 1 || q.mode == 3 {
        let [l1, l2] = q.lambdas;
        match q.variant {
            0 => ls.perform(&q.u)?,
            1 => ls.perform_lambdas(&q.u, l1, l2)?,
            2 => ls.perform_tangents(&q.u, &q.vectors[0], &q.vectors[1], l1, l2)?,
            _ => {
                let v = &q.vectors;
                ls.perform_curvatures(&q.u, [&v[0], &v[1], &v[2], &v[3]], l1, l2)?
            }
        }
    }
    ls_results(output, &mut ls, q.mode >= 2, lambdas);
    Ok(())
}

pub fn ls_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let q = read_ls(&mut r);
    let lambdas = r.integer();
    attempt(output, || run_ls(output, &q, lambdas));
    end(output);
}

pub fn resol_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let line = read_line(&mut r);
    let (fp, lp) = (r.integer(), r.integer());
    let (fc, lc) = (constraint(r.integer()), constraint(r.integer()));
    let cons = read_constraints(&mut r);
    let nbpol = r.integer();
    let u = read_vector(&mut r);
    let tol = r.real();
    attempt(output, || {
        let mut ls = LeastSquare::new(&line, fp, lp, fc, lc, &u, nbpol)?;
        let mut c = ls.bezier_value()?;
        let (a, da) = (
            ls.function_matrix()?.clone(),
            ls.derivative_function_matrix()?.clone(),
        );
        let mut resol = ResolConstraint::new(&line, &mut c, fp, lp, &cons, &a, &da, tol)?;
        p::flag(output, resol.is_done());
        matrix(output, resol.constraint_matrix());
        if resol.is_done() {
            matrix(output, resol.inverse_matrix());
            vector(output, resol.duale());
        }
        curve(output, &c)?;
        curve(output, &ls.bezier_value()?)?;
        attempt(output, || {
            matrix(
                output,
                resol.constraint_derivative(&line, &u, nbpol - 1, &da)?,
            );
            matrix(output, resol.constraint_matrix());
            Ok(())
        });
        Ok(())
    });
    end(output);
}

/// The calls shared by Function and BSpFunction.
trait ProbeFunction {
    fn value(&mut self, x: &Vector<'_>, f: &mut f64) -> Result<bool, Failure>;
    fn gradient(&mut self, x: &Vector<'_>, g: &mut Vector<'_>) -> Result<bool, Failure>;
    fn values(&mut self, x: &Vector<'_>, f: &mut f64, g: &mut Vector<'_>) -> Result<bool, Failure>;
    fn print_curve(&mut self, output: &Output) -> Result<(), Failure>;
    fn error(&mut self, i: i32, k: i32) -> Result<f64, Failure>;
    fn errors(&self) -> [f64; 2];
    fn parameters(&self) -> &Vector<'static>;
    fn extra(&mut self, output: &Output, code: i32) -> Result<(), Failure>;
}

impl ProbeFunction for Function<MultiLine> {
    fn value(&mut self, x: &Vector<'_>, f: &mut f64) -> Result<bool, Failure> {
        self.evaluate(x, f)
    }
    fn gradient(&mut self, x: &Vector<'_>, g: &mut Vector<'_>) -> Result<bool, Failure> {
        self.evaluate_gradient(x, g)
    }
    fn values(&mut self, x: &Vector<'_>, f: &mut f64, g: &mut Vector<'_>) -> Result<bool, Failure> {
        self.evaluate_values(x, f, g)
    }
    fn print_curve(&mut self, output: &Output) -> Result<(), Failure> {
        curve(output, self.curve_value()?)
    }
    fn error(&mut self, i: i32, k: i32) -> Result<f64, Failure> {
        Function::error(self, i, k)
    }
    fn errors(&self) -> [f64; 2] {
        [self.max_error_3d(), self.max_error_2d()]
    }
    fn parameters(&self) -> &Vector<'static> {
        self.new_parameters()
    }
    fn extra(&mut self, _output: &Output, _: i32) -> Result<(), Failure> {
        Ok(())
    }
}

impl ProbeFunction for BSpFunction<MultiLine> {
    fn value(&mut self, x: &Vector<'_>, f: &mut f64) -> Result<bool, Failure> {
        self.evaluate(x, f)
    }
    fn gradient(&mut self, x: &Vector<'_>, g: &mut Vector<'_>) -> Result<bool, Failure> {
        self.evaluate_gradient(x, g)
    }
    fn values(&mut self, x: &Vector<'_>, f: &mut f64, g: &mut Vector<'_>) -> Result<bool, Failure> {
        self.evaluate_values(x, f, g)
    }
    fn print_curve(&mut self, output: &Output) -> Result<(), Failure> {
        let c = self.curve_value()?;
        curve(output, &c)?;
        p::int(output, MultiCurveEval::degree(&c)?);
        Ok(())
    }
    fn error(&mut self, i: i32, k: i32) -> Result<f64, Failure> {
        BSpFunction::error(self, i, k)
    }
    fn errors(&self) -> [f64; 2] {
        [self.max_error_3d(), self.max_error_2d()]
    }
    fn parameters(&self) -> &Vector<'static> {
        self.new_parameters()
    }
    fn extra(&mut self, output: &Output, code: i32) -> Result<(), Failure> {
        match code {
            8 => matrix(output, self.function_matrix()?),
            9 => matrix(output, self.derivative_function_matrix()?),
            _ => ivector(output, self.index()),
        }
        Ok(())
    }
}

fn function_call(
    output: &Output,
    r: &mut Reader<'_>,
    f: &mut dyn ProbeFunction,
    [first, last, curves]: [i32; 3],
) -> Result<(), Failure> {
    let code = r.integer();
    match code {
        1..=3 => {
            let x = read_vector(r);
            let (mut value, mut g) = (-7., Vector::filled(first, last, -7.)?);
            match code {
                1 => {
                    p::flag(output, f.value(&x, &mut value)?);
                    p::real(output, value);
                }
                2 => {
                    p::flag(output, f.gradient(&x, &mut g)?);
                    vector(output, &g);
                }
                _ => {
                    p::flag(output, f.values(&x, &mut value, &mut g)?);
                    p::real(output, value);
                    vector(output, &g);
                }
            }
        }
        4 => f.print_curve(output)?,
        5 => {
            for i in first..=last {
                for k in 1..=curves {
                    p::real(output, f.error(i, k)?);
                }
            }
        }
        6 => f
            .errors()
            .into_iter()
            .for_each(|value| p::real(output, value)),
        7 => vector(output, f.parameters()),
        _ => f.extra(output, code)?,
    }
    Ok(())
}

fn function_calls(
    output: &Output,
    r: &mut Reader<'_>,
    f: &mut dyn ProbeFunction,
    bounds: [i32; 3],
) {
    for _ in 0..r.integer() {
        attempt(output, || function_call(output, r, f, bounds));
    }
}

pub fn function_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let line = read_line(&mut r);
    let (fp, lp) = (r.integer(), r.integer());
    let cons = read_constraints(&mut r);
    let u = read_vector(&mut r);
    let deg = r.integer();
    let curves = line.nb_points().expect("probe line");
    attempt(output, || {
        let mut f = Function::new(&line, fp, lp, &cons, &u, deg)?;
        p::int(output, f.nb_variables());
        function_calls(output, &mut r, &mut f, [fp, lp, curves]);
        Ok(())
    });
    end(output);
}

pub fn bspfunction_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let line = read_line(&mut r);
    let (fp, lp) = (r.integer(), r.integer());
    let cons = read_constraints(&mut r);
    let u = read_vector(&mut r);
    let (knots, mults) = (read_knots(&mut r), read_mults(&mut r));
    let nbpol = r.integer();
    let (l1, l2) = (r.real(), r.real());
    let curves = line.nb_points().expect("probe line");
    attempt(output, || {
        let mut f = BSpFunction::new(&line, fp, lp, &cons, &u, &knots, &mults, nbpol)?;
        p::int(output, f.nb_variables());
        f.set_first_lambda(l1);
        f.set_last_lambda(l2);
        function_calls(output, &mut r, &mut f, [fp, lp, curves]);
        Ok(())
    });
    end(output);
}

pub fn gradient_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let line = read_line(&mut r);
    let (fp, lp) = (r.integer(), r.integer());
    let cons = read_constraints(&mut r);
    let mut u = read_vector(&mut r);
    let (deg, t3, t2, iterations) = (r.integer(), r.real(), r.real(), r.integer());
    attempt(output, || {
        let g = Gradient::new(&line, fp, lp, &cons, &mut u, deg, t3, t2, iterations)?;
        p::flag(output, g.is_done());
        curve(output, &g.value())?;
        let errors = (fp..=lp)
            .map(|j| g.error(j))
            .collect::<Result<Vec<_>, _>>()?;
        gradient_results_tail(
            output,
            errors,
            [g.average_error(), g.max_error_3d(), g.max_error_2d()],
            &u,
        );
        Ok(())
    });
    end(output);
}

pub fn bspgradient_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let line = read_line(&mut r);
    let (fp, lp) = (r.integer(), r.integer());
    let cons = read_constraints(&mut r);
    let mut u = read_vector(&mut r);
    let (knots, mults) = (read_knots(&mut r), read_mults(&mut r));
    let (deg, t3, t2, iterations, with) = (r.integer(), r.real(), r.real(), r.integer(), r.flag());
    let (l1, l2) = (r.real(), r.real());
    attempt(output, || {
        let input = BSpGradientInput {
            ssp: &line,
            points: [fp, lp],
            constraints: &cons,
            knots: &knots,
            mults: &mults,
            deg,
            tolerances: [t3, t2],
            nb_iterations: iterations,
        };
        let g = if with {
            BSpGradient::with_lambdas(&input, &mut u, l1, l2)?
        } else {
            BSpGradient::new(&input, &mut u)?
        };
        p::flag(output, g.is_done());
        curve(output, &g.value())?;
        let errors = (fp..=lp)
            .map(|j| g.error(j))
            .collect::<Result<Vec<_>, _>>()?;
        gradient_results_tail(
            output,
            errors,
            [g.average_error(), g.max_error_3d(), g.max_error_2d()],
            &u,
        );
        p::int(output, MultiCurveEval::degree(&g.value())?);
        Ok(())
    });
    end(output);
}

fn gradient_results_tail(output: &Output, errors: Vec<f64>, stats: [f64; 3], u: &Vector<'_>) {
    errors.into_iter().for_each(|value| p::real(output, value));
    stats.into_iter().for_each(|value| p::real(output, value));
    vector(output, u);
}
