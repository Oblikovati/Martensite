//! Correspondence failures retain observed values rather than guessing geometric identity.
use super::*;
fn plan() -> Plan {
    let request = super::super::tests::request();
    let inputs = input::read(&request, &mut || false).unwrap();
    *inputs.placement.unwrap()
}
#[test]
fn identity_and_multiple_datum_locations_retain_the_observed_chain_shape() {
    let plan = plan();
    let error = check_location(&Location::default(), &plan).err().unwrap();
    assert!(error.to_string().contains("identity: true"));
    let first = Location::new(plan.transform);
    let second = Location::new(plan.transform);
    let chain = first.multiplied(&second).unwrap();
    let error = check_location(&chain, &plan).err().unwrap();
    assert!(error.to_string().contains("tail_identity: false"));
}
#[test]
fn changed_coefficients_report_actual_expected_and_scale() {
    let plan = plan();
    let wrong = Location::new(Transform3::default());
    let error = check_location(&wrong, &plan).err().unwrap();
    let diagnostic = error.to_string();
    for field in ["Coefficients", "actual:", "expected:", "scale: 1.0"] {
        assert!(diagnostic.contains(field), "{diagnostic}");
    }
}
#[test]
fn reflected_stored_location_cannot_reuse_a_right_handed_frame_certificate() {
    let plan = plan();
    let reflected = Transform3::from_stored_rigid_coefficients([
        [-1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
    ])
    .unwrap();
    let error = check_location(&Location::new(reflected), &plan)
        .err()
        .unwrap();
    assert!(error.to_string().contains("scale: -1.0"));
    assert_eq!(error.class(), WrappedFailureClass::Incomplete);
}
#[test]
fn equal_transform_values_do_not_substitute_for_shared_datum_identity() {
    let plan = plan();
    let root = Location::new(plan.transform);
    let different = Location::new(plan.transform);
    let shape = Shape::default().located(&different, false).unwrap();
    let key = BindingKey {
        source: Source::Region(SurfaceRegionId(5)),
        role: crate::surface_sketch::HistoryRole::Solid,
    };
    let error = strip_bindings(&root, &[(key, shape)]).err().unwrap();
    assert!(error.to_string().contains("SemanticDatum"));
    assert_eq!(
        error.entities(),
        &[crate::surface_sketch::HistorySource::Region(
            SurfaceRegionId(5)
        )]
    );
}
#[test]
fn archive_diagnostics_identify_a_changed_byte_and_a_shortened_prefix() {
    assert!(compare_archive(&[1, 2], &[1, 2]).is_ok());
    let changed = compare_archive(&[1, 2], &[1, 3]).err().unwrap().to_string();
    for field in ["offset: 1", "actual: Some(2)", "expected: Some(3)"] {
        assert!(changed.contains(field), "{changed}");
    }
    let shortened = compare_archive(&[1], &[1, 3]).err().unwrap().to_string();
    for field in ["actual_length: 1", "expected_length: 2", "actual: None"] {
        assert!(shortened.contains(field), "{shortened}");
    }
}
#[test]
fn changed_native_geometry_cannot_reuse_the_canonical_certificate_archive() {
    let plan = plan();
    let result = normal_extrude_profile(&super::super::tests::request()).unwrap();
    let physical = place(&result.family.family, &plan).unwrap();
    let (root, bindings) = pipeline::decode(&physical.bytes, &result.family.family.keys).unwrap();
    mutate_corner(&bindings);
    let error = correspondence(&result.family.family, &plan, &root, &bindings)
        .err()
        .unwrap();
    assert_eq!(error.class(), WrappedFailureClass::Incomplete);
    assert!(error.to_string().contains("FrozenGraph"));
}
fn mutate_corner(bindings: &[(BindingKey, Shape)]) {
    let shape = &bindings
        .iter()
        .find(|(key, _)| key.role == crate::surface_sketch::HistoryRole::FirstDepthVertex)
        .unwrap()
        .1;
    let vertex = martensite_tkbrep::topods::vertex(shape).unwrap();
    martensite_tkbrep::brep::BRepBuilder::default()
        .update_vertex_point(&vertex, Point3::new(100., 100., 100.), 0.)
        .unwrap();
}

#[test]
fn unchanged_publication_preserves_the_complete_canonical_archive() {
    let result = normal_extrude_profile(&super::super::tests::request()).unwrap();
    let family = &result.family.family;
    let physical = unchanged(family).unwrap();
    assert_eq!(physical.bytes, family.bytes);
    assert_eq!(physical.bindings.len(), family.keys.len());
}
