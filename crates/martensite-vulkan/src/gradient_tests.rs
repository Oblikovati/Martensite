use crate::gradient_background::gradient_layer;
use martensite_tkservice::aspect::AspectGradientFillMethod;

#[path = "gradient_background_device_tests.rs"]
mod device_tests;
#[path = "../tests/support/elliptical_gradient_reference.rs"]
mod elliptical_reference;
#[path = "gradient_elliptical_reference_tests.rs"]
mod elliptical_reference_tests;
