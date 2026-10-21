//! Exact exception contracts include diagnostics and mutation before failure.
mod support;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkhelix::{helix_brep::HelixBRepBuilderHelix, helix_geom::HelixGeomHelixCurve};
use std::f64::consts::FRAC_PI_2;
use support::brep::{array, my_axis};

fn construction_failure(result: Result<(), Failure>, expected: &str) {
    let failure = result.unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_ConstructionError");
    assert_eq!(failure.message(), expected);
}

const INVALID_LOADS: &[(f64, f64, f64, f64, f64)] = &[
    (2., 1., 1., 1., 0.),
    (1., 1., 1., 1., 0.),
    (0., 2., -1., 1., 0.),
    (0., 2., 1., -1., 0.),
    (0., 2., 1., 1., -FRAC_PI_2),
    (0., 2., 1., 1., FRAC_PI_2),
];

#[test]
fn rejected_loads_keep_exact_diagnostic_and_assigned_range() {
    for &(first, last, pitch, radius, taper) in INVALID_LOADS {
        let curve = HelixGeomHelixCurve::new();
        construction_failure(
            curve.load(first, last, pitch, radius, taper, true),
            "HelixGeom_HelixCurve::Load",
        );
        assert_eq!(curve.first_parameter().unwrap(), first);
        assert_eq!(curve.last_parameter().unwrap(), last);
        construction_failure(curve.load_current(), "HelixGeom_HelixCurve::Load");
    }
}

#[test]
fn dimension_overloads_keep_exact_diagnostic() {
    let mut builder = HelixBRepBuilderHelix::new();
    let expected = "HelixBRep_BuilderHelix::SetParameters: wrong array dimension";
    let axis = my_axis();
    let diameters = array(&[1., 2., 3.]);
    let heights = array(&[1.]);
    let flags = array(&[true]);
    let pitches = array(&[1., 2.]);
    let failure = builder.set_parameters(&axis, &diameters, &heights, &heights, &flags);
    construction_failure(failure, expected);
    assert_eq!((builder.error_status(), builder.warning_status()), (1, 1));
    let failure = builder.set_parameters_turns(&axis, &diameters, &heights, &heights);
    construction_failure(failure, expected);
    let failure = builder.set_parameters_diam_turns(&axis, 2., &pitches, &heights);
    construction_failure(failure, expected);
    let failure = builder.set_parameters_spiral_turns(&axis, 2., 3., &pitches, &heights);
    construction_failure(failure, expected);
}

#[test]
fn empty_parts_keep_exact_list_exception() {
    let mut builder = HelixBRepBuilderHelix::new();
    builder
        .set_parameters(
            &my_axis(),
            &array(&[1.]),
            &array(&[]),
            &array(&[]),
            &array(&[]),
        )
        .unwrap();
    let failure = builder.perform().unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NoSuchObject");
    assert_eq!(failure.message(), "NCollection_List::First");
}

#[test]
fn unsupported_derivative_orders_keep_exact_diagnostic() {
    let curve = HelixGeomHelixCurve::new();
    for order in [i32::MIN, -1, 0, 3, i32::MAX] {
        let failure = curve.dn(0.5, order).unwrap_err();
        assert_eq!(failure.exception_type(), "Standard_NotImplemented");
        assert_eq!(failure.message(), "HelixGeom_HelixCurve::EvalDN");
    }
}
