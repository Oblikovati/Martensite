//! BooleanUtilities: the BRepAlgoAPITestBase and BOPAlgoTestBase fixture methods.

#![allow(dead_code, non_snake_case)]

use super::*;
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::f64::consts::PI;

/// `BRepAlgoAPITestBase::PerformCut`.
pub fn PerformCut(object: &Shape, tool: &Shape) -> Shape {
    let cutter = BRepAlgoAPICut::from_shapes(object, tool).unwrap();
    assert!(cutter.is_done(), "BRepAlgoAPICut operation failed");
    cutter.shape().clone()
}

/// `BRepAlgoAPITestBase::PerformFuse`.
pub fn PerformFuse(first: &Shape, second: &Shape) -> Shape {
    let fuser = BRepAlgoAPIFuse::from_shapes(first, second).unwrap();
    assert!(fuser.is_done(), "BRepAlgoAPIFuse operation failed");
    fuser.shape().clone()
}

/// Applies a the `trotate` command axis-angle transform used by Boolean regressions.
pub fn RotateAroundAxis(shape: &mut Shape, pivot: Point3, axis: [f64; 3], angle_degrees: f64) {
    let direction = Direction3::new(axis[0], axis[1], axis[2]).unwrap();
    let mut transform = Transform3::default();
    transform
        .set_rotation(Axis1::new(pivot, direction), angle_degrees * PI / 180.0)
        .unwrap();
    shape.move_by(&Location::new(transform), false).unwrap();
}

/// Applies the `trotate` command Z-axis form used by Boolean regressions.
pub fn RotateAboutZ(shape: &mut Shape, pivot: Point3, angle_degrees: f64) {
    RotateAroundAxis(shape, pivot, [0.0, 0.0, 1.0], angle_degrees);
}

/// `BRepAlgoAPITestBase::PerformCommon`.
pub fn PerformCommon(first: &Shape, second: &Shape) -> Shape {
    let common = BRepAlgoAPICommon::from_shapes(first, second).unwrap();
    assert!(common.is_done(), "BRepAlgoAPICommon operation failed");
    common.shape().clone()
}

/// `BOPAlgoTestBase::PerformDirectBOP`.
pub fn PerformDirectBOP(first: &Shape, second: &Shape, operation: Operation) -> Shape {
    let mut bop = BOPAlgoBOP::new();
    bop.add_argument(first);
    bop.add_tool(second);
    bop.set_operation(operation);
    bop.options_mut().set_fuzzy_value(DefaultFuzzyValue());
    bop.options_mut().set_run_parallel(false);
    bop.set_non_destructive(false);
    bop.perform();
    assert!(!bop.options().has_errors(), "Direct BOP operation failed");
    bop.shape().clone()
}

/// `BOPAlgoTestBase::PreparePaveFiller`.
pub fn PreparePaveFiller(first: &Shape, second: &Shape) -> BOPAlgoPaveFiller {
    let arguments: List<Shape> = [first.clone(), second.clone()].into_iter().collect();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_fuzzy_value(DefaultFuzzyValue());
    filler.options_mut().set_run_parallel(false);
    filler.set_non_destructive(false);
    filler.perform();
    assert!(
        !filler.options().has_errors(),
        "PaveFiller preparation failed"
    );
    filler
}

/// `BOPAlgoTestBase::PerformBOPWithPaveFiller`.
pub fn PerformBOPWithPaveFiller(filler: &mut BOPAlgoPaveFiller, operation: Operation) -> Shape {
    let arguments: Vec<Shape> = filler.arguments().iter().cloned().collect();
    assert_eq!(arguments.len(), 2, "Wrong number of arguments");
    let mut bop = BOPAlgoBOP::new();
    bop.add_argument(&arguments[0]);
    bop.add_tool(&arguments[1]);
    bop.set_operation(operation);
    bop.options_mut().set_run_parallel(false);
    bop.perform_with_filler(filler);
    assert!(
        !bop.options().has_errors(),
        "BOP operation with PaveFiller failed"
    );
    bop.shape().clone()
}

/// `BOPAlgoTestBase::PerformTwoStepBOP`: like that fixture it returns only the shape and drops
/// the warning reports of both steps. A case script that must report them runs
/// `tests/dataset_cases/support/two_step.rs` instead.
pub fn PerformTwoStepBOP(first: &Shape, second: &Shape, operation: Operation) -> Shape {
    let mut filler = PreparePaveFiller(first, second);
    PerformBOPWithPaveFiller(&mut filler, operation)
}
