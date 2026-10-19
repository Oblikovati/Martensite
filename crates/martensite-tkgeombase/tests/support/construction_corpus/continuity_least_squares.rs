//! Complete continuous analytic least-squares request driver with local output.
use super::common::{self as p, Reader, attempt, end};
use super::{continuity_function::AccFunction, multi_curve_tokens::curve, print::Output};
use martensite_tkgeombase::app_cont::{Function, LeastSquare};
use martensite_tkgeombase::app_par_curves::Constraint;

fn constraint(code: i32) -> Constraint {
    Constraint::from_code(code).expect("corpus constraint")
}

pub fn ls_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let f = AccFunction::read(&mut r);
    let (fc, lc) = (constraint(r.integer()), constraint(r.integer()));
    let (deg, nbpoints) = (r.integer(), r.integer());
    let (nb3d, nb2d) = f.get_number_of_points();
    [nb3d, nb2d, f.get_nb_of_3d_points(), f.get_nb_of_2d_points()]
        .into_iter()
        .for_each(|v| p::int(output, v));
    attempt(output, || {
        let (u0, u1) = (f.first_parameter(), f.last_parameter());
        let mut ls = LeastSquare::new(&f, u0, u1, fc, lc, deg, nbpoints)?;
        p::flag(output, ls.is_done());
        let (e, e3, e2) = ls.error();
        [e, e3, e2].into_iter().for_each(|v| p::real(output, v));
        curve(output, ls.value()?)
    });
    end(output);
}
