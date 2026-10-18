//! One matrix token serializer serves owned replay and retained stdout consumers.
use super::{common as p, print::Output};
use martensite_tkmath::math::Matrix;

pub fn matrix(output: &Output, m: &Matrix<'_>) {
    p::int(output, m.lower_row());
    p::int(output, m.upper_row());
    p::int(output, m.lower_col());
    p::int(output, m.upper_col());
    for i in m.lower_row()..=m.upper_row() {
        (m.lower_col()..=m.upper_col()).for_each(|j| p::real(output, m[(i, j)]));
    }
}
