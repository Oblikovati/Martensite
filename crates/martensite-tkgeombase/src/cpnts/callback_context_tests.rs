//! Captured typed context preserves the scalar callback's client-state behavior.
use super::{MyGaussFunction, RealFunction};
use crate::cpnts::MyRootFunction;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::math::{Function, FunctionWithDerivative};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};

struct IntegrandContext {
    scale: AtomicI32,
    calls: AtomicUsize,
    reject: AtomicBool,
}
impl IntegrandContext {
    fn new(scale: i32) -> Self {
        Self {
            scale: AtomicI32::new(scale),
            calls: AtomicUsize::new(0),
            reject: AtomicBool::new(false),
        }
    }
    fn callback(&self) -> RealFunction<'_> {
        Box::new(move |x| {
            self.calls.fetch_add(1, Ordering::Relaxed);
            if self.reject.load(Ordering::Relaxed) {
                return Err(Failure::out_of_range(&format!(
                    "parameter={x}; expected an accepting callback context"
                )));
            }
            Ok(f64::from(self.scale.load(Ordering::Relaxed)) * x)
        })
    }
}

#[test]
fn gauss_callback_rereads_its_client_context() {
    let context = IntegrandContext::new(2);
    let mut function = MyGaussFunction::new();
    function.init(context.callback());
    let mut result = -1.0;
    assert!(function.value(4.0, &mut result));
    assert_eq!(result, 8.0);
    context.scale.store(3, Ordering::Relaxed);
    assert!(function.value(4.0, &mut result));
    assert_eq!(result, 12.0);
    assert_eq!(context.calls.load(Ordering::Relaxed), 2);
}

#[test]
fn gauss_callback_transports_context_failure_without_overwriting_output() {
    let context = IntegrandContext::new(2);
    context.reject.store(true, Ordering::Relaxed);
    let mut function = MyGaussFunction::new();
    function.init(context.callback());
    let mut result = 123.0;
    assert!(!function.value(4.0, &mut result));
    assert_eq!(result, 123.0);
    assert!(
        function
            .take_failure()
            .unwrap_err()
            .is_kind("Standard_OutOfRange")
    );
    context.reject.store(false, Ordering::Relaxed);
    assert!(function.value(4.0, &mut result));
    assert_eq!(result, 8.0);
    assert!(function.take_failure().is_ok());
}

#[test]
fn root_integral_and_derivative_follow_the_same_client_context() {
    let context = IntegrandContext::new(2);
    let mut function = MyRootFunction::new();
    function.init_function(context.callback(), 4);
    function.init_target(1.0, 3.0);
    let (mut value, mut derivative) = (0.0, 0.0);
    assert!(function.values(4.0, &mut value, &mut derivative));
    // Integral(scale*t, t=1..4) - 3 is 7.5*scale - 3.
    assert!((value - 12.0).abs() <= 1e-12);
    assert_eq!(derivative, 8.0);
    context.scale.store(4, Ordering::Relaxed);
    assert!(function.values(4.0, &mut value, &mut derivative));
    assert!((value - 27.0).abs() <= 1e-12);
    assert_eq!(derivative, 16.0);
}
