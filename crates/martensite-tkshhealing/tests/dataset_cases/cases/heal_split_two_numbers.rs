//! tests/heal/split_two_numbers case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871
//! that build their inputs: the hemisphere `psphere a 10 0 90` of tests/heal/split_number,
//! its first face split by `DT_SplitByNumber result a_1 nu nv` and `checknbshapes result
//! -t -face k`, then the grid end `puts [checkshape result]` and tests/heal/end.
use super::heal_split_number::hemisphere;
use crate::support::session::Session;

/// tests/heal/split_two_numbers/A1.
pub fn a1(s: &mut dyn Session) {
    hemisphere(s, 3, Some(5), 15, "A1");
}

/// tests/heal/split_two_numbers/A2.
pub fn a2(s: &mut dyn Session) {
    hemisphere(s, 1, Some(5), 5, "A2");
}

/// tests/heal/split_two_numbers/A3.
pub fn a3(s: &mut dyn Session) {
    hemisphere(s, 5, Some(1), 5, "A3");
}
