use super::*;
fn source() -> StoredSplineInput {
    StoredSplineInput {
        poles: vec![[0., 0., 0.], [1., 0., 0.], [1., 1., 0.]],
        weights: vec![1.; 3],
        knots: vec![0., 1.],
        multiplicities: vec![3, 3],
        flat_knots: vec![0., 0., 0., 1., 1., 1.],
        domain: [0., 1.],
        degree: 2,
        periodic: false,
        rational: false,
        evaluation_override: false,
        lower_bounds: [1; 5],
    }
}
fn admit(input: StoredSplineInput) -> Result<StoredOpenExtrusionInput, StoredOpenExtrusionFailure> {
    input.into_open_extrusion_input([0., 0., 1.], [-1., 1.])
}
#[test]
fn open_admission_preserves_input_and_does_not_identify_endpoints() {
    let original = source();
    let input = admit(original.clone()).unwrap();
    assert_eq!(input.source(), &original);
    assert_eq!(input.numeric().poles(), original.poles());
    assert_eq!(input.layout().extrusion_interval, [-1., 1.]);
    assert!(matches!(
        original.into_closed_extrusion_input([0., 0., 1.], [-1., 1.]),
        Ok(crate::geom::ClosedSplineExtrusionInput::Inconclusive(_))
    ));
}
#[test]
fn unsupported_evaluator_and_layout_refuse() {
    let mut override_input = source();
    override_input.evaluation_override = true;
    assert!(matches!(
        admit(override_input),
        Err(StoredOpenExtrusionFailure::EvaluationOverride(_))
    ));
    let mut periodic = source();
    periodic.periodic = true;
    assert!(matches!(
        admit(periodic),
        Err(StoredOpenExtrusionFailure::UnsupportedLayout(_))
    ));
}
#[test]
fn unsupported_stored_array_origin_refuses() {
    let mut lower = source();
    lower.lower_bounds[4] = 0;
    assert!(matches!(
        admit(lower),
        Err(StoredOpenExtrusionFailure::UnsupportedLayout(_))
    ));
}
#[test]
fn open_endpoints_do_not_bypass_flat_knot_or_domain_checks() {
    let mut flat = source();
    flat.flat_knots[2] = 0.5;
    assert!(matches!(
        admit(flat),
        Err(StoredOpenExtrusionFailure::Storage(
            ClosedSplineExtrusionError::FlatSequence { .. }
        ))
    ));
}
#[test]
fn inconsistent_stored_domain_refuses() {
    let mut domain = source();
    domain.domain[1] = 2.;
    assert_eq!(
        admit(domain).unwrap_err(),
        StoredOpenExtrusionFailure::Domain {
            actual: [0., 2.],
            expected: [0., 1.]
        }
    );
}
#[test]
fn numeric_failures_remain_invalid_instead_of_unsupported() {
    let mut input = source();
    input.poles[1][0] = f64::INFINITY;
    assert!(matches!(
        admit(input),
        Err(StoredOpenExtrusionFailure::Numeric(_))
    ));
    let mut input = source();
    input.weights[1] = 2.;
    assert!(matches!(
        admit(input),
        Err(StoredOpenExtrusionFailure::Numeric(_))
    ));
}
#[test]
fn admitted_storage_is_send_sync_and_isolated_from_source_mutation() {
    fn owned<T: Send + Sync>() {}
    owned::<StoredOpenExtrusionInput>();
    owned::<StoredOpenExtrusionFailure>();
    let mut input = source();
    let admitted = admit(input.clone()).unwrap();
    input.poles[0] = [9.; 3];
    input.flat_knots[0] = 9.;
    assert_eq!(admitted.source().poles()[0], [0.; 3]);
    assert_eq!(admitted.source().flat_knots()[0], 0.);
}
#[test]
fn admitted_numeric_input_executes_the_open_support_operation() {
    use martensite_tkmath::predicates::roots::open_extrusion::*;
    let admitted = admit(source()).unwrap();
    let report = certify_open_quadratic_extrusion(
        admitted.numeric().clone(),
        OpenExtrusionPolicy::bounded(),
        || false,
    );
    assert!(report.certificate().is_some());
    assert_eq!(report.input().poles(), admitted.source().poles());
}
