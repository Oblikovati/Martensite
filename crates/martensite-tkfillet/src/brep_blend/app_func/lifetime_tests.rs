//! A real constructed sweep root exposes the existing owned-versus-borrowed function contract.
use super::{BRepBlendAppFunc, SharedAppFunction};
use crate::blend::BlendAppFunction;
use crate::blend_func::ConstRad;
use crate::brep_blend::{BRepBlendLine, BRepBlendLineHandle};
use crate::chfi_ds::shared;
use martensite_tkgeombase::approx::SweepFunction;
use std::sync::{Arc, Mutex, Weak};

type WeakAppFunction = Weak<Mutex<dyn BlendAppFunction>>;
type WeakBlendLine = Weak<Mutex<BRepBlendLine>>;

struct AppRootInputs {
    line: BRepBlendLineHandle,
    function: SharedAppFunction,
}

impl AppRootInputs {
    fn new() -> Self {
        let (first, second, guide) = crate::blend_func::test_fixtures::right_angle();
        let mut function = ConstRad::new(&first, &second, &guide).unwrap();
        function.set_radius(1., 1);
        let function: SharedAppFunction = shared(function);
        Self {
            line: shared(BRepBlendLine::new()),
            function,
        }
    }

    fn root(&self) -> BRepBlendAppFunc {
        let root = BRepBlendAppFunc::new(&self.line, &self.function, 1.0e-4, 1.0e-5).unwrap();
        assert!(Arc::ptr_eq(root.line(), &self.line));
        root
    }
}

fn observe_count(case: &str, event: &str, value: usize) {
    let observation = format!("{{\"case\":\"{case}\",\"event\":\"{event}\",\"value\":{value}}}");
    println!("{observation}");
}

fn observe_live_root(case: &str, root: &BRepBlendAppFunc) {
    let shape = root.section_shape().unwrap();
    let curves = root.nb_2d_curves().unwrap();
    // Independent real native instance: radius1, original right-angle planes and X guide.
    assert_eq!(shape, (7, 2, 6));
    assert_eq!(curves, 2);
    println!(
        "{{\"case\":\"{case}\",\"event\":\"section_shape\",\"poles\":{},\"knots\":{},\"degree\":{}}}",
        shape.0, shape.1, shape.2
    );
    observe_count(case, "parameter_curves", curves as usize);
}

fn initial_shares(case: &str, inputs: &AppRootInputs) {
    assert_eq!(Arc::strong_count(&inputs.function), 2);
    assert_eq!(Arc::strong_count(&inputs.line), 2);
    observe_count(
        case,
        "rust_function_owners_during",
        Arc::strong_count(&inputs.function),
    );
    observe_count(case, "line_owners_during", Arc::strong_count(&inputs.line));
}

fn released_root_shares(case: &str, inputs: &AppRootInputs) {
    assert_eq!(Arc::strong_count(&inputs.function), 1);
    assert_eq!(Arc::strong_count(&inputs.line), 1);
    observe_count(
        case,
        "rust_function_owners_after",
        Arc::strong_count(&inputs.function),
    );
    observe_count(case, "line_owners_after", Arc::strong_count(&inputs.line));
}

fn assert_callers_released(function: &WeakAppFunction, line: &WeakBlendLine) {
    assert_eq!(function.strong_count(), 0);
    assert_eq!(line.strong_count(), 0);
    let function_gone = usize::from(function.upgrade().is_none());
    let line_gone = usize::from(line.upgrade().is_none());
    observe_count("root_first", "function_released_with_caller", function_gone);
    observe_count("root_first", "line_released_with_caller", line_gone);
}

fn retire_caller_line(inputs: AppRootInputs) -> SharedAppFunction {
    let AppRootInputs { line, function } = inputs;
    drop(line);
    function
}

fn assert_line_retained(line: &WeakBlendLine) {
    assert_eq!(line.strong_count(), 1);
    let alive = usize::from(line.upgrade().is_some());
    observe_count("line_first", "line_retained_without_caller", alive);
}

fn assert_root_line_released(line: &WeakBlendLine, function: &SharedAppFunction) {
    assert_eq!(line.strong_count(), 0);
    assert_eq!(Arc::strong_count(function), 1);
    let gone = usize::from(line.upgrade().is_none());
    observe_count("line_first", "line_released_with_root", gone);
    observe_count(
        "line_first",
        "rust_function_survives_root",
        Arc::strong_count(function),
    );
}

#[test]
fn actual_root_releases_owned_shares_before_caller_retirement() {
    let inputs = AppRootInputs::new();
    let function: WeakAppFunction = Arc::downgrade(&inputs.function);
    let line: WeakBlendLine = Arc::downgrade(&inputs.line);
    let root = inputs.root();
    initial_shares("root_first", &inputs);
    observe_live_root("root_first", &root);
    drop(root);
    released_root_shares("root_first", &inputs);
    drop(inputs);
    assert_callers_released(&function, &line);
}

#[test]
fn actual_root_keeps_the_line_after_its_caller_retires() {
    let inputs = AppRootInputs::new();
    let root = inputs.root();
    let line: WeakBlendLine = Arc::downgrade(&inputs.line);
    let function = retire_caller_line(inputs);
    assert_line_retained(&line);
    observe_live_root("line_first", &root);
    drop(root);
    assert_root_line_released(&line, &function);
}

#[test]
fn rust_owned_function_survives_caller_retirement_without_a_native_dangling_call() {
    let inputs = AppRootInputs::new();
    let root = inputs.root();
    let function: WeakAppFunction = Arc::downgrade(&inputs.function);
    let AppRootInputs {
        line,
        function: caller_function,
    } = inputs;
    drop(caller_function);
    assert_eq!(function.strong_count(), 1);
    observe_live_root("rust_owned_only", &root);
    drop(root);
    assert_eq!(function.strong_count(), 0);
    assert_eq!(Arc::strong_count(&line), 1);
    observe_count(
        "rust_owned_only",
        "function_released_with_root",
        usize::from(function.upgrade().is_none()),
    );
}
