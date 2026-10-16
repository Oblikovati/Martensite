//! Mathutils functor tests.
#![allow(non_snake_case, unused_mut, clippy::field_reassign_with_default)]
use martensite_tkmath::math::{
    Function, FunctionWithDerivative, Matrix, MultipleVarFunction, MultipleVarFunctionWithGradient,
    Vector,
};
use martensite_tkmath::math_opt::{bfgs, powell};
use martensite_tkmath::math_root::{brent, newton};
use martensite_tkmath::math_utils::scalar::*;
use martensite_tkmath::math_utils::vector::*;
use martensite_tkmath::math_utils::{
    Config, NDimConfig, TWO_PI, make_vector, make_vector_with_gradient,
};
const THE_TOLERANCE: f64 = 1e-10;
fn near(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() <= tolerance, "{a} != {b}");
}
include!("mathutils_functor/tests_0.rs");
include!("mathutils_functor/tests_1.rs");
include!("mathutils_functor/tests_2.rs");
include!("mathutils_functor/tests_3.rs");
include!("mathutils_functor/tests_4.rs");
include!("mathutils_functor/tests_5.rs");
include!("mathutils_functor/tests_6.rs");
