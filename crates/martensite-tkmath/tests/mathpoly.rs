//! Mathpoly tests.
#![allow(
    non_snake_case,
    clippy::excessive_precision,
    clippy::needless_range_loop
)]
use martensite_tkmath::{math_poly::*, math_utils::Status};
include!("mathpoly/fixtures.rs");
include!("mathpoly/linear_quadratic.rs");
include!("mathpoly/cubic.rs");
include!("mathpoly/quartic.rs");
