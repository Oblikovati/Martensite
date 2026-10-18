//! The Rust twin of the implementation and
//! the implementation.

use super::approx_compute_shared::{build_compute, compute_results, constraint, read_options};
use super::common::{Reader, attempt, end};
use super::constraint_line_fixtures::read_line;
use super::linear_tokens::read_vector;
use super::print::Output;
use martensite_tkgeombase::app_def::Compute;

/// `ad_compute line variant options firstC lastC params repeat`: Compute.
pub fn compute_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let line = read_line(&mut r);
    let variant = r.integer();
    let o = read_options(&mut r);
    let (fc, lc) = (constraint(r.integer()), constraint(r.integer()));
    let u = read_vector(&mut r);
    let repeat = r.flag();
    attempt(output, || {
        let mut c: Compute = build_compute(&line, variant, o, &u)?;
        if variant >= 2 {
            c.set_constraints(fc, lc);
            c.perform(&line)?;
        }
        compute_results(output, &mut c)?;
        if repeat {
            let mut again = o;
            again.degree_max += 1;
            again.tolerance_3d *= 10.;
            again.tolerance_2d *= 10.;
            c.init(again);
            c.perform(&line)?;
            compute_results(output, &mut c)?;
        }
        Ok(())
    });
    end(output);
}
