//! Mathinteg new tests.
// Keep the configuration mutation sequence visible in the tests.
#![allow(non_snake_case, clippy::field_reassign_with_default)]
use martensite_tkmath::{
    math::{Function, GaussSingleIntegration, KronrodSingleIntegration},
    math_integ::*,
    math_utils::IntegResult,
};
include!("mathinteg_new/fixtures.rs");
include!("mathinteg_new/tests_0.rs");
include!("mathinteg_new/tests_1.rs");
include!("mathinteg_new/tests_2.rs");
include!("mathinteg_new/tests_3.rs");
include!("mathinteg_new/tests_4.rs");
include!("mathinteg_new/tests_5.rs");
include!("mathinteg_new/tests_6.rs");
include!("mathinteg_new/tests_7.rs");
