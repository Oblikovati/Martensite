//! Regression of a cone-face prism common regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkmath::gp::Vector3;

#[test]
fn BOPCommonSimpleTest__ConeFacePrism_ZP7() {
    let cone = CreateCone(10.0, 0.0, 20.0);
    let second_face = GetFaceByIndex(&cone, 2);
    let prism = CreatePrism(&second_face.into_shape(), Vector3::new(0.0, 0.0, 10.0));
    let result = PerformCommon(&cone, &prism);
    ValidateResult(&result, 919.56, -1.0, false);
}
