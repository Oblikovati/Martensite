//! This executable owns process-wide exception capture settings independently.
#[path = "geom_eval_exception_contracts/fixtures.rs"]
mod fixtures;
use fixtures::{curve_fixtures, rational_trigonometric_patch, surface_fixtures, transformations};
use martensite_tkernel::standard::Failure;

struct CaptureConfiguration(i32);
impl Drop for CaptureConfiguration {
    fn drop(&mut self) {
        Failure::set_default_stack_trace_length(self.0);
    }
}

#[test]
fn evaluation_failures_obey_disabled_and_enabled_capture_configuration() {
    let _restore = CaptureConfiguration(Failure::default_stack_trace_length());
    let mut failures = Vec::new();
    for length in [-1, 0, 4] {
        Failure::set_default_stack_trace_length(length);
        inspect_curve_captures(length > 0, &mut failures);
        inspect_constructor_captures(length > 0, &mut failures);
        inspect_surface_captures(length > 0, &mut failures);
        let failure = rational_trigonometric_patch()
            .eval_dn(0.25, 0.5, 4, 0)
            .unwrap_err();
        record_capture(failure, length > 0, &mut failures);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn record_capture(failure: Failure, enabled: bool, failures: &mut Vec<String>) {
    if failure.stack_string().is_empty() == enabled {
        failures.push(format!(
            "{}: capture expected {enabled}, got {:?}",
            failure.message(),
            failure.stack_string()
        ));
    }
}

fn inspect_curve_captures(enabled: bool, failures: &mut Vec<String>) {
    for (_, curve) in curve_fixtures() {
        for transform in transformations() {
            record_capture(curve.transform(transform).unwrap_err(), enabled, failures);
        }
        record_capture(curve.eval_dn(0.25, 0).unwrap_err(), enabled, failures);
        record_capture(curve.period().unwrap_err(), enabled, failures);
    }
}

fn inspect_surface_captures(enabled: bool, failures: &mut Vec<String>) {
    for (_, surface) in surface_fixtures() {
        record_capture(surface.v_period().unwrap_err(), enabled, failures);
        if !surface.is_u_periodic() {
            record_capture(surface.u_period().unwrap_err(), enabled, failures);
        }
        for transform in transformations() {
            record_capture(surface.transform(transform).unwrap_err(), enabled, failures);
        }
        record_capture(
            surface.eval_dn(0.25, 0.5, 0, 0).unwrap_err(),
            enabled,
            failures,
        );
    }
}

fn inspect_constructor_captures(enabled: bool, failures: &mut Vec<String>) {
    for build in CONSTRUCTION_FAILURES {
        record_capture(build(), enabled, failures);
    }
}
use martensite_tkg3d::geom_eval::*;
use martensite_tkmath::gp::{Axis2, Axis3};
const CONSTRUCTION_FAILURES: &[fn() -> Failure] = &[
    || {
        GeomEvalCircularHelixCurve::new(Axis2::default(), 0., 1.)
            .err()
            .unwrap()
    },
    || {
        GeomEvalSineWaveCurve::new(Axis2::default(), 0., 1., 0.)
            .err()
            .unwrap()
    },
    || {
        GeomEvalTBezierCurve::new(&fixtures::curve_poles(), 0.)
            .err()
            .unwrap()
    },
    || {
        GeomEvalAHTBezierCurve::new(&fixtures::curve_poles(), -1, 0., 0.)
            .err()
            .unwrap()
    },
    || {
        GeomEvalHyperboloidSurface::new(Axis3::default(), 0., 1., HyperboloidSheetMode::OneSheet)
            .err()
            .unwrap()
    },
    || {
        GeomEvalEllipsoidSurface::new(Axis3::default(), 0., 1., 1.)
            .err()
            .unwrap()
    },
    || {
        GeomEvalHypParaboloidSurface::new(Axis3::default(), 0., 1.)
            .err()
            .unwrap()
    },
    || {
        GeomEvalParaboloidSurface::new(Axis3::default(), 0.)
            .err()
            .unwrap()
    },
    || {
        GeomEvalCircularHelicoidSurface::new(Axis3::default(), 0.)
            .err()
            .unwrap()
    },
    || {
        GeomEvalTBezierSurface::new(&fixtures::surface_poles(), 0., 1.)
            .err()
            .unwrap()
    },
    || {
        GeomEvalAHTBezierSurface::new(&fixtures::surface_poles(), [-1, 2], [0.; 2], [0.; 2])
            .err()
            .unwrap()
    },
];
