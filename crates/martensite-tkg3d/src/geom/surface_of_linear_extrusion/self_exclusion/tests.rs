//! Independent exact loops and actual delegated evaluators exercise the public adapter.
use super::*;
use crate::geom::{GeomBSplineCurve, GeomCurve, GeomSurface, GeomSweptSurface};
use crate::geom_eval::{RepCurveDescFull, RepSurfaceDescFull};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkmath::gp::{Direction3, Point3};

fn array<T: Copy + Default>(values: &[T]) -> Array1<'static, T> {
    let mut result = Array1::with_bounds(1, values.len() as i32).unwrap();
    result.as_mut_slice().copy_from_slice(values);
    result
}
fn curve() -> GeomBSplineCurve {
    let points = diamond_points();
    GeomBSplineCurve::new(
        &array(&points),
        &array(&[0., 1., 2., 3., 4.]),
        &array(&[3, 2, 2, 2, 3]),
        2,
        false,
    )
    .unwrap()
}
fn diamond_points() -> [Point3; 9] {
    [
        [1., 0., 0.],
        [1., 1., 0.],
        [0., 1., 0.],
        [-1., 1., 0.],
        [-1., 0., 0.],
        [-1., -1., 0.],
        [0., -1., 0.],
        [1., -1., 0.],
        [1., 0., 0.],
    ]
    .map(|p| Point3::new(p[0], p[1], p[2]))
}

fn surface() -> GeomSurfaceOfLinearExtrusion {
    let basis = Handle::new(curve()).cast::<dyn GeomCurve>();
    GeomSurfaceOfLinearExtrusion::new(&basis, Direction3::new(0., 0., 1.).unwrap()).unwrap()
}
fn snapshot(surface: &GeomSurfaceOfLinearExtrusion) -> ClosedSplineExtrusionSnapshot {
    match surface
        .closed_quadratic_self_exclusion([-1., 1.], ClosedExtrusionPolicy::bounded(), || false)
        .unwrap()
    {
        ClosedSplineExtrusionExclusion::Captured(snapshot) => snapshot,
        other => panic!("expected numeric snapshot, got {other:?}"),
    }
}

#[test]
fn public_adapter_captures_the_actual_basis_and_evaluator_domain() {
    let surface = surface();
    let result = snapshot(&surface);
    assert!(result.numeric_report().certificate().is_some());
    assert_eq!(result.numeric_report().input().curve_interval(), [0., 4.]);
    assert_eq!(result.layout().extrusion_interval, [-1., 1.]);
    assert!(!result.layout().basis_override);
    assert_eq!(
        surface.value(0.5, 0.25).unwrap(),
        Point3::new(0.75, 0.75, 0.25)
    );
    assert_eq!(result.numeric_report().work().finished_obligations, 10);
}

#[test]
fn owned_snapshot_survives_actual_basis_and_direction_mutation() {
    let surface = surface();
    let before = snapshot(&surface);
    let basis = surface.basis_curve().cast::<GeomBSplineCurve>();
    basis
        .get()
        .unwrap()
        .set_pole(2, Point3::new(9., 9., 0.))
        .unwrap();
    surface.set_direction(Direction3::new(1., 2., 3.).unwrap());
    let after = snapshot(&surface);
    assert_eq!(before.numeric_report().input().poles()[1], [1., 1., 0.]);
    assert_eq!(after.numeric_report().input().poles()[1], [9., 9., 0.]);
    assert_ne!(before.layout().direction, after.layout().direction);
}

#[test]
fn exact_callback_runs_after_both_geometry_read_guards_are_dropped() {
    let surface = surface();
    let mut changed = false;
    let outcome = surface
        .closed_quadratic_self_exclusion([-1., 1.], ClosedExtrusionPolicy::bounded(), || {
            if !changed {
                surface.set_direction(Direction3::new(1., 0., 0.).unwrap());
                changed = true;
            }
            false
        })
        .unwrap();
    let ClosedSplineExtrusionExclusion::Captured(before) = outcome else {
        panic!("expected owned capture");
    };
    assert_eq!(before.layout().direction, [0., 0., 1.]);
    assert_eq!(snapshot(&surface).layout().direction, [1., 0., 0.]);
}

#[test]
fn an_actual_curve_evaluator_override_does_not_certify_the_stored_basis() {
    let surface = surface();
    let before = snapshot(&surface);
    replace_curve_evaluator(&surface);
    assert_ne!(surface.value(0.5, 0.).unwrap(), Point3::new(0.75, 0.75, 0.));
    assert!(matches!(
        surface
            .closed_quadratic_self_exclusion([-1., 1.], ClosedExtrusionPolicy::bounded(), || false)
            .unwrap(),
        ClosedSplineExtrusionExclusion::Inconclusive(
            ClosedSplineExtrusionInconclusive::BasisEvaluationOverride(_)
        )
    ));
    assert_eq!(before.numeric_report().input().poles()[1], [1., 1., 0.]);
}

fn replace_curve_evaluator(surface: &GeomSurfaceOfLinearExtrusion) {
    let replacement = Handle::new(curve());
    replacement
        .get()
        .unwrap()
        .set_pole(2, Point3::new(9., 9., 0.))
        .unwrap();
    let basis = surface.basis_curve().cast::<GeomBSplineCurve>();
    basis
        .get()
        .unwrap()
        .set_eval_representation(Handle::new(RepCurveDescFull::new(replacement.cast())).cast())
        .unwrap();
}

#[test]
fn an_actual_surface_evaluator_override_is_explicitly_inconclusive() {
    let surface = surface();
    let other = surface_copy_with_changed_direction();
    surface
        .set_eval_representation(Handle::new(RepSurfaceDescFull::new(other)).cast())
        .unwrap();
    assert!(matches!(
        surface
            .closed_quadratic_self_exclusion([-1., 1.], ClosedExtrusionPolicy::bounded(), || false)
            .unwrap(),
        ClosedSplineExtrusionExclusion::Inconclusive(
            ClosedSplineExtrusionInconclusive::SurfaceEvaluationOverride { .. }
        )
    ));
}
fn surface_copy_with_changed_direction() -> Handle<dyn GeomSurface> {
    let other = surface();
    other.set_direction(Direction3::new(1., 0., 0.).unwrap());
    Handle::new(other).cast()
}

#[test]
fn higher_degree_and_periodic_real_supports_are_not_admitted() {
    let basis = curve();
    basis.increase_degree(3).unwrap();
    let elevated = GeomSurfaceOfLinearExtrusion::new(
        &Handle::new(basis).cast(),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    assert!(matches!(
        elevated
            .closed_quadratic_self_exclusion([-1., 1.], ClosedExtrusionPolicy::bounded(), || false)
            .unwrap(),
        ClosedSplineExtrusionExclusion::Inconclusive(
            ClosedSplineExtrusionInconclusive::UnsupportedLayout(_)
        )
    ));
}

#[test]
fn a_real_periodic_basis_is_not_admitted() {
    let basis = curve();
    basis.set_periodic().unwrap();
    let periodic = GeomSurfaceOfLinearExtrusion::new(
        &Handle::new(basis).cast(),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    assert!(matches!(
        periodic
            .closed_quadratic_self_exclusion([-1., 1.], ClosedExtrusionPolicy::bounded(), || false)
            .unwrap(),
        ClosedSplineExtrusionExclusion::Inconclusive(
            ClosedSplineExtrusionInconclusive::UnsupportedLayout(ClosedSplineExtrusionLayout {
                periodic: true,
                ..
            })
        )
    ));
}

#[test]
fn nonfinite_requested_interval_is_a_typed_input_error() {
    let result = surface().closed_quadratic_self_exclusion(
        [f64::NAN, 1.],
        ClosedExtrusionPolicy::bounded(),
        || false,
    );
    assert!(matches!(result, Err(ClosedSplineExtrusionError::Input(_))));
}

#[test]
fn adapter_outcomes_are_send_sync_without_live_geometry_handles() {
    fn assert_owned<T: Send + Sync>() {}
    assert_owned::<ClosedSplineExtrusionSnapshot>();
    assert_owned::<ClosedSplineExtrusionExclusion>();
    assert_owned::<ClosedSplineExtrusionError>();
    assert_owned::<ClosedSplineExtrusionLayout>();
}

#[test]
fn valid_open_and_other_basis_types_do_not_become_invalid_geometry() {
    let basis = curve();
    basis.set_pole(9, Point3::new(2., 0., 0.)).unwrap();
    let open = extrusion_from(basis);
    assert!(matches!(
        open.closed_quadratic_self_exclusion([-1., 1.], ClosedExtrusionPolicy::bounded(), || false)
            .unwrap(),
        ClosedSplineExtrusionExclusion::Inconclusive(
            ClosedSplineExtrusionInconclusive::NotExactlyClosed { .. }
        )
    ));
}

#[test]
fn other_actual_basis_types_are_explicitly_unsupported() {
    let line =
        crate::geom::GeomLine::from_point(Point3::default(), Direction3::new(1., 0., 0.).unwrap());
    let other = GeomSurfaceOfLinearExtrusion::new(
        &Handle::new(line).cast(),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    assert!(matches!(
        other
            .closed_quadratic_self_exclusion([-1., 1.], ClosedExtrusionPolicy::bounded(), || false)
            .unwrap(),
        ClosedSplineExtrusionExclusion::Inconclusive(
            ClosedSplineExtrusionInconclusive::UnsupportedBasis { .. }
        )
    ));
}

fn extrusion_from(curve: GeomBSplineCurve) -> GeomSurfaceOfLinearExtrusion {
    GeomSurfaceOfLinearExtrusion::new(
        &Handle::new(curve).cast(),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap()
}
