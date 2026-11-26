use super::*;
use crate::primitive_shapes::box_sized;

struct FakeControlSession {
    events: Vec<&'static str>,
    capture_failure: bool,
    restore_failure: bool,
}

impl FakeControlSession {
    fn capture(&mut self) -> Result<(), Box<dyn Error>> {
        self.events.push("capture");
        if self.capture_failure {
            return Err(std::io::Error::other("capture failure").into());
        }
        Ok(())
    }

    fn restore(&mut self) -> Result<(), Box<dyn Error>> {
        self.events.push("restore");
        if self.restore_failure {
            return Err(std::io::Error::other("restore failure").into());
        }
        Ok(())
    }
}

fn fixture() -> (Shape, Vec<Shape>) {
    let source = box_sized([10., 20., 30.]).unwrap();
    let edges = all_distinct(&source, ShapeEnum::Edge);
    (source, edges)
}

#[test]
fn refusal_errors_keep_their_specific_causes() {
    assert_eq!(
        EndpointControlRefusal::NotAnEdge.to_string(),
        "relation endpoint is not an edge"
    );
    assert_eq!(
        EndpointControlRefusal::ForeignOrAmbiguousEdge.to_string(),
        "relation endpoint is not one unique source edge"
    );
    assert_eq!(
        EndpointControlRefusal::DuplicateEndpoints.to_string(),
        "relation endpoints alias the same edge"
    );
    assert_eq!(
        EndpointControlRefusal::EmptyRemainder.to_string(),
        "edge removal leaves an empty source"
    );
}

#[test]
fn restoration_runs_after_capture_failure_and_both_errors_are_reported() {
    let mut fake = FakeControlSession {
        events: Vec::new(),
        capture_failure: true,
        restore_failure: true,
    };
    let error = run_control_with_restore(
        &mut fake,
        FakeControlSession::capture,
        FakeControlSession::restore,
    )
    .unwrap_err();
    assert_eq!(fake.events, ["capture", "restore"]);
    assert!(error.to_string().contains("capture failure"));
    assert!(error.to_string().contains("restore failure"));
}

#[test]
fn restore_pair_validation_requires_two_case_source_identities_in_order() {
    let (_source, edges) = fixture();
    let roles = [edges[2].clone(), edges[8].clone()];
    assert!(identic_pair_matches(&roles, &roles, &roles));
    assert!(!identic_pair_matches(&roles[..1], &roles, &roles));
    assert!(!identic_pair_matches(
        &[roles[0].clone(), roles[1].clone(), roles[0].clone()],
        &roles,
        &roles
    ));
    assert!(!identic_pair_matches(
        &[roles[1].clone(), roles[0].clone()],
        &roles,
        &roles
    ));
}

#[test]
fn roles_bind_by_identity_and_each_remainder_retains_every_other_edge() {
    let (source, edges) = fixture();
    let [first, second] = relation_endpoints(&source, edges[2].clone(), edges[8].clone()).unwrap();
    for removed in [&first, &second] {
        let (control, retained) = edge_remainder(&source, removed).unwrap();
        let actual = all_distinct(&control, ShapeEnum::Edge);
        assert_eq!(actual.len(), edges.len() - 1);
        assert!(actual.iter().all(|edge| !edge.is_same(removed)));
        assert!(
            retained
                .iter()
                .all(|edge| edges.iter().any(|source_edge| source_edge.is_same(edge)))
        );
    }
}

#[test]
fn foreign_edge_refuses_endpoint_binding() {
    let (source, edges) = fixture();
    let foreign = all_distinct(&box_sized([11., 20., 30.]).unwrap(), ShapeEnum::Edge)[2].clone();
    assert!(matches!(
        relation_endpoints(&source, foreign, edges[1].clone()),
        Err(EndpointControlRefusal::ForeignOrAmbiguousEdge)
    ));
}

#[test]
fn wrong_shape_and_duplicate_roles_refuse() {
    let (source, edges) = fixture();
    let vertex = all_distinct(&source, ShapeEnum::Vertex)[0].clone();
    assert!(matches!(
        relation_endpoints(&source, vertex, edges[1].clone()),
        Err(EndpointControlRefusal::NotAnEdge)
    ));
    assert!(matches!(
        relation_endpoints(&source, edges[1].clone(), edges[1].clone()),
        Err(EndpointControlRefusal::DuplicateEndpoints)
    ));
}

#[test]
fn edge_remainder_refuses_a_foreign_source_edge() {
    let (source, _) = fixture();
    let foreign = all_distinct(&box_sized([11., 20., 30.]).unwrap(), ShapeEnum::Edge)[2].clone();
    assert!(matches!(
        edge_remainder(&source, &foreign),
        Err(EndpointControlRefusal::ForeignOrAmbiguousEdge)
    ));
}
