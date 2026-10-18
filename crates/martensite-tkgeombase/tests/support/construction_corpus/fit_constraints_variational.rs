//! The `ad_var` requests: Variational.
use super::approx_compute_shared::{array, bspline};
use super::common::{self as p, Reader, attempt, end};
use super::print::Output;
use super::{
    app_par_fit::read_constraints, constraint_line_fixtures::read_line, linear_tokens::dump,
};
use super::{finite_element_driver::shape, linear_tokens::read_vector, matrix_tokens::matrix};
use martensite_tkernel::ncollection::{Array1, HArray1};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeombase::app_def::{Variational, VariationalOptions};
use martensite_tkmath::math::Matrix;

fn harray(output: &Output, a: &Handle<HArray1<f64>>) {
    array(output, &a.array1());
}

fn read_harray(r: &mut Reader<'_>) -> Handle<HArray1<f64>> {
    let v = read_vector(r);
    let values: Vec<f64> = (v.lower()..=v.upper()).map(|i| v[i]).collect();
    Handle::new(HArray1::from_array(&Array1::from_shared(
        &values,
        v.lower(),
    )))
}

fn flags(output: &Output, v: &Variational) {
    p::flag(output, v.is_created());
    p::flag(output, v.is_done());
    p::flag(output, v.is_over_constrained());
}

fn settings(output: &Output, v: &Variational) {
    p::int(output, v.max_degree());
    p::int(output, v.max_segment());
    p::int(output, v.continuity() as i32);
    p::flag(output, v.with_min_max());
    p::flag(output, v.with_cutting());
    p::real(output, v.tolerance());
    p::int(output, v.nb_iterations());
    v.criterium_weight()
        .into_iter()
        .for_each(|value| p::real(output, value));
}

fn results(output: &Output, v: &Variational) {
    attempt(output, || bspline(output, &v.value()?));
    attempt(output, || {
        let e = v.max_error()?;
        p::real(output, e);
        if e != 0. {
            p::int(output, v.max_error_index()?);
        }
        Ok(())
    });
    attempt(output, || {
        v.quadratic_error().map(|value| p::real(output, value))
    });
    attempt(output, || {
        v.average_error().map(|value| p::real(output, value))
    });
    attempt(output, || v.parameters().map(|value| harray(output, value)));
    attempt(output, || v.knots().map(|value| harray(output, value)));
    attempt(output, || {
        v.criterium()?
            .into_iter()
            .for_each(|value| p::real(output, value));
        Ok(())
    });
}

fn var_command(
    output: &Output,
    r: &mut Reader<'_>,
    v: &mut Variational,
    npoints: i32,
) -> Result<(), Failure> {
    let code = r.integer();
    match code {
        1 => {
            v.approximate()?;
            flags(output, v);
            Ok(())
        }
        2 => {
            settings(output, v);
            Ok(())
        }
        3 => {
            results(output, v);
            Ok(())
        }
        4 => {
            let (lr, lc, rows) = (r.integer(), r.integer(), r.integer());
            let mut m = Matrix::filled(lr, lr + rows - 1, lc, lc + npoints - 1, -7.).unwrap();
            v.distance(&mut m)?;
            matrix(output, &m);
            Ok(())
        }
        5 => {
            if v.is_done() && v.max_error()? == 0. {
                p::int(output, -1);
            } else {
                let mut text = String::new();
                let _ = v.dump(&mut text);
                dump(output, &text);
            }
            Ok(())
        }
        6 => v
            .set_constraints(&read_constraints(r))
            .map(|value| p::flag(output, value)),
        7 => v.set_parameters(&read_harray(r)),
        8 => v
            .set_knots(&read_harray(r))
            .map(|value| p::flag(output, value)),
        9 => v
            .set_max_degree(r.integer())
            .map(|value| p::flag(output, value)),
        10 => {
            p::flag(output, v.set_max_segment(r.integer()));
            Ok(())
        }
        11 => v
            .set_continuity(shape(r.integer()))
            .map(|value| p::flag(output, value)),
        12 => v.set_with_min_max(r.flag()),
        13 => v
            .set_with_cutting(r.flag())
            .map(|value| p::flag(output, value)),
        14 => {
            let (a, b, c) = (r.real(), r.real(), r.real());
            v.set_criterium_weight(a, b, c)
        }
        15 => {
            let order = r.integer();
            v.set_criterium_weight_of(order, r.real())
        }
        16 => v.set_tolerance(r.real()),
        _ => {
            v.set_nb_iterations(r.integer());
            Ok(())
        }
    }
}

pub fn var_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let line = read_line(&mut r);
    let (first, last) = (r.integer(), r.integer());
    let constraints = read_constraints(&mut r);
    let options = VariationalOptions {
        max_degree: r.integer(),
        max_segment: r.integer(),
        continuity: shape(r.integer()),
        with_min_max: r.flag(),
        with_cutting: r.flag(),
        tolerance: r.real(),
        nb_iterations: r.integer(),
    };
    let count = r.integer();
    attempt(output, || {
        let mut v = Variational::new(&line, first, last, &constraints, options)?;
        flags(output, &v);
        for _ in 0..count {
            attempt(output, || {
                var_command(output, &mut r, &mut v, last - first + 1)
            });
        }
        Ok(())
    });
    end(output);
}
