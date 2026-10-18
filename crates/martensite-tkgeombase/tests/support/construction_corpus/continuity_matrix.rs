//! All original continuous-fit matrix-provider requests retain bounds, failures and partial writes.
use super::common::{Reader, attempt, end};
use super::{matrix_tokens::matrix, print::Output};
use martensite_tkgeombase::app_cont::cont_matrices as cm;
use martensite_tkmath::math::Matrix;

pub fn matrix_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (kind, classe, nbpoints) = (r.integer(), r.integer(), r.integer());
    let (rl, cl, rows, cols) = (r.integer(), r.integer(), r.integer(), r.integer());
    let mut m = Matrix::filled(rl, rl + rows - 1, cl, cl + cols - 1, -7.).unwrap();
    attempt(output, || match kind {
        0 => cm::m_matrix(classe, &mut m),
        1 => cm::inv_m_matrix(classe, &mut m),
        2 => cm::ibp_matrix(classe, &mut m),
        3 => cm::ibt_matrix(classe, &mut m),
        _ => cm::v_bernstein(classe, nbpoints, &mut m),
    });
    matrix(output, &m);
    end(output);
}
