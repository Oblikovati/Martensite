//! Every direct retained refusal is a literal exception before mutation or evaluation.
use super::fixtures::{curve_fixtures, surface_fixtures, transformations};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkmath::gp::Transform3;

type CurveRefusal = (&'static str, fn(&dyn GeomCurve, f64) -> Result<(), Failure>);
type SurfaceRefusal = (
    &'static str,
    fn(&dyn GeomSurface, f64) -> Result<(), Failure>,
);

fn collect_failure(failures: &mut Vec<String>, result: Result<(), Failure>, member: &str) {
    let _ = super::failures::record_failure(failures, result, ("Standard_NotImplemented", member));
}

#[test]
fn all_curve_refusals_keep_exact_literals_and_unchanged_evaluators() {
    let mut failures = Vec::new();
    for (class, curve) in curve_fixtures() {
        check_curve_refusals(curve.get().unwrap(), class, &mut failures);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

const CURVE_REFUSALS: [CurveRefusal; 3] = [
    ("Reverse", |curve, _| curve.reverse()),
    ("Reverse", |curve, _| curve.reversed().map(|_| ())),
    ("ReversedParameter", |curve, parameter| {
        curve.reversed_parameter(parameter).map(|_| ())
    }),
];

#[test]
fn all_surface_refusals_keep_exact_literals_and_unchanged_evaluators() {
    let mut failures = Vec::new();
    for (class, surface) in surface_fixtures() {
        check_surface_refusals(surface.get().unwrap(), class, &mut failures);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

const SURFACE_REFUSALS: [SurfaceRefusal; 8] = [
    ("UReverse", |surface, _| surface.u_reverse()),
    ("VReverse", |surface, _| surface.v_reverse()),
    ("UReverse", |surface, _| surface.u_reversed().map(|_| ())),
    ("VReverse", |surface, _| surface.v_reversed().map(|_| ())),
    ("UReversedParameter", |surface, parameter| {
        surface.u_reversed_parameter(parameter).map(|_| ())
    }),
    ("VReversedParameter", |surface, parameter| {
        surface.v_reversed_parameter(parameter).map(|_| ())
    }),
    ("UIso", |surface, parameter| {
        surface.u_iso(parameter).map(|_| ())
    }),
    ("VIso", |surface, parameter| {
        surface.v_iso(parameter).map(|_| ())
    }),
];

#[test]
fn all_curve_transformations_reject_identity_and_translation_without_mutation() {
    let mut failures = Vec::new();
    for (class, curve) in curve_fixtures() {
        let before = curve.eval_d1(0.25).unwrap();
        for transform in transformations() {
            curve_transform_failures(curve.get().unwrap(), transform, class, &mut failures);
            assert_eq!(curve.eval_d1(0.25).unwrap(), before);
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn curve_transform_failures(
    curve: &dyn GeomCurve,
    transform: Transform3,
    class: &str,
    failures: &mut Vec<String>,
) {
    let member = format!("{class}::Transform");
    collect_failure(failures, curve.transform(transform), &member);
    collect_failure(failures, curve.transformed(transform).map(|_| ()), &member);
}

#[test]
fn all_surface_transformations_reject_identity_and_translation_without_mutation() {
    let mut failures = Vec::new();
    for (class, surface) in surface_fixtures() {
        let before = surface.eval_d1(0.25, 0.5).unwrap();
        for transform in transformations() {
            surface_transform_failures(surface.get().unwrap(), transform, class, &mut failures);
            assert_eq!(surface.eval_d1(0.25, 0.5).unwrap(), before);
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn surface_transform_failures(
    surface: &dyn GeomSurface,
    transform: Transform3,
    class: &str,
    failures: &mut Vec<String>,
) {
    let member = format!("{class}::Transform");
    collect_failure(failures, surface.transform(transform), &member);
    collect_failure(
        failures,
        surface.transformed(transform).map(|_| ()),
        &member,
    );
}

fn check_curve_refusals(curve: &dyn GeomCurve, class: &str, failures: &mut Vec<String>) {
    let before = curve.eval_d1(0.25).unwrap();
    for (member, operation) in CURVE_REFUSALS {
        for parameter in [-3., 0.25, f64::NAN, f64::INFINITY] {
            collect_failure(
                failures,
                operation(curve, parameter),
                &format!("{class}::{member}"),
            );
            assert_eq!(curve.eval_d1(0.25).unwrap(), before);
        }
    }
}

fn check_surface_refusals(surface: &dyn GeomSurface, class: &str, failures: &mut Vec<String>) {
    let before = surface.eval_d1(0.25, 0.5).unwrap();
    for (member, operation) in SURFACE_REFUSALS {
        for parameter in [-3., 0.25, f64::NAN, f64::INFINITY] {
            collect_failure(
                failures,
                operation(surface, parameter),
                &format!("{class}::{member}"),
            );
            assert_eq!(surface.eval_d1(0.25, 0.5).unwrap(), before);
        }
    }
}
