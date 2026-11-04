//! Identity-bound topology controls for the two operands of an identic relation.
use crate::boolean_typed_explode::all_distinct;
use crate::session_run::SessionRun;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::error::Error;

/// A relation endpoint cannot be used to make a source-only control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EndpointControlRefusal {
    NotAnEdge,
    ForeignOrAmbiguousEdge,
    DuplicateEndpoints,
    EmptyRemainder,
}

impl std::fmt::Display for EndpointControlRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason = match self {
            Self::NotAnEdge => "relation endpoint is not an edge",
            Self::ForeignOrAmbiguousEdge => "relation endpoint is not one unique source edge",
            Self::DuplicateEndpoints => "relation endpoints alias the same edge",
            Self::EmptyRemainder => "edge removal leaves an empty source",
        };
        formatter.write_str(reason)
    }
}

impl std::error::Error for EndpointControlRefusal {}

/// Validate both role-tagged relation edges against the displayed source by topological identity.
/// Example: `relation_endpoints(&box_shape, first_shape, second_shape)?`.
pub(crate) fn relation_endpoints(
    source: &Shape,
    first: Shape,
    second: Shape,
) -> Result<[Shape; 2], EndpointControlRefusal> {
    validate_endpoint_pair(source, [&first, &second])?;
    Ok([first, second])
}

fn validate_endpoint_pair(
    source: &Shape,
    endpoints: [&Shape; 2],
) -> Result<(), EndpointControlRefusal> {
    if endpoints
        .iter()
        .any(|edge| edge.shape_type().ok() != Some(ShapeEnum::Edge))
    {
        return Err(EndpointControlRefusal::NotAnEdge);
    }
    if endpoints[0].is_same(endpoints[1]) {
        return Err(EndpointControlRefusal::DuplicateEndpoints);
    }
    for endpoint in endpoints {
        validate_source_edge(source, endpoint)?;
    }
    Ok(())
}

fn validate_source_edge(source: &Shape, endpoint: &Shape) -> Result<(), EndpointControlRefusal> {
    if endpoint.shape_type().ok() != Some(ShapeEnum::Edge) {
        return Err(EndpointControlRefusal::NotAnEdge);
    }
    let matches = all_distinct(source, ShapeEnum::Edge)
        .iter()
        .filter(|edge| edge.is_same(endpoint))
        .count();
    if matches != 1 {
        return Err(EndpointControlRefusal::ForeignOrAmbiguousEdge);
    }
    Ok(())
}

/// Build the source's edge-only wireframe after removing exactly one identity-bound edge.
/// Example: `edge_remainder(&box_shape, &relation_first_edge)?`.
pub(crate) fn edge_remainder(
    source: &Shape,
    removed: &Shape,
) -> Result<(Shape, Vec<Shape>), EndpointControlRefusal> {
    validate_source_edge(source, removed)?;
    build_edge_remainder(source, removed)
}

/// Capture relation-only and each source-edge control, restoring and byte-checking the scene.
/// Example: `capture_controls(&mut run, &source, &roles, &[(87,170),(165,170)])?`.
pub(crate) fn capture_controls(
    run: &mut SessionRun,
    source: &Shape,
    roles: &[Shape; 2],
    picks: &[(i32, i32); 2],
) -> Result<(), Box<dyn Error>> {
    let selected = run.session().selected_sub_shapes_on("b")?;
    if !identic_pair_matches(&selected, roles, &selected) {
        return Err("redges endpoints and current selected owner order; expected two identity-matched edges".into());
    }
    capture_relation_erased(run)?;
    capture_edge_erased(run, source, roles, picks, &selected, 0)?;
    capture_edge_erased(run, source, roles, picks, &selected, 1)
}

fn capture_relation_erased(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    run_control_with_restore(
        run,
        |run| {
            run.session().erase(&["redges"])?;
            crate::source_output::control_capture::control_dump(
                run.session(),
                "identic_edges.png",
                "locus-1",
            )
        },
        restore_relation_control,
    )
}

fn restore_relation_control(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    let shown = run.session().show_erased(&["redges"]).map_err(Into::into);
    let image = restore_dump_and_check(run, "restored-relation");
    merge_cleanup_results(shown, image)
}

fn capture_edge_erased(
    run: &mut SessionRun,
    source: &Shape,
    roles: &[Shape; 2],
    picks: &[(i32, i32); 2],
    case_selection: &[Shape],
    removed_role: usize,
) -> Result<(), Box<dyn Error>> {
    let (without_edge, _) = edge_remainder(source, &roles[removed_role])?;
    run_control_with_restore(
        run,
        |run| capture_removed_edge(run, &without_edge, roles, picks, removed_role),
        |run| restore_edge_control(run, source, roles, picks, case_selection, removed_role),
    )
}

fn restore_edge_control(
    run: &mut SessionRun,
    source: &Shape,
    roles: &[Shape; 2],
    picks: &[(i32, i32); 2],
    case_selection: &[Shape],
    removed_role: usize,
) -> Result<(), Box<dyn Error>> {
    let source = restore_source_and_selection(run, source, roles, picks, case_selection);
    let suffix = format!("restored-edge-{removed_role}");
    let image = restore_dump_and_check(run, &suffix);
    merge_cleanup_results(source, image)
}

fn capture_removed_edge(
    run: &mut SessionRun,
    replacement: &Shape,
    roles: &[Shape; 2],
    picks: &[(i32, i32); 2],
    removed_role: usize,
) -> Result<(), Box<dyn Error>> {
    run.session().replace_shape_for_control("b", replacement)?;
    run.session().set_object_selection_mode("b", 2, true)?;
    prove_removed_owner_and_select_remainder(run, roles, picks, removed_role)?;
    let suffix = if removed_role == 0 {
        "locus-2"
    } else {
        "locus-3"
    };
    crate::source_output::control_capture::control_dump(run.session(), "identic_edges.png", suffix)
}

fn prove_removed_owner_and_select_remainder(
    run: &mut SessionRun,
    roles: &[Shape; 2],
    picks: &[(i32, i32); 2],
    removed_role: usize,
) -> Result<(), Box<dyn Error>> {
    let retained_role = 1 - removed_role;
    run.session()
        .select_point(picks[removed_role].0, picks[removed_role].1)?;
    require_selection_missing(run, &roles[removed_role])?;
    run.session().clear_selected_for_control()?;
    run.session()
        .select_point(picks[retained_role].0, picks[retained_role].1)?;
    require_single_selected_edge(run, &roles[retained_role])
}

fn require_selection_missing(run: &mut SessionRun, removed: &Shape) -> Result<(), Box<dyn Error>> {
    let selected = run.session().selected_sub_shapes_on("b")?;
    if selected.iter().any(|shape| shape.is_same(removed)) {
        return Err(
            "removed relation endpoint remained selectable; expected its exact source edge absent"
                .into(),
        );
    }
    Ok(())
}

fn require_single_selected_edge(
    run: &mut SessionRun,
    expected: &Shape,
) -> Result<(), Box<dyn Error>> {
    let selected = run.session().selected_sub_shapes_on("b")?;
    if selected.len() == 1 && selected[0].is_same(expected) {
        return Ok(());
    }
    Err("remaining relation endpoint selection; expected one exact retained edge identity".into())
}

fn restore_source_and_selection(
    run: &mut SessionRun,
    source: &Shape,
    roles: &[Shape; 2],
    picks: &[(i32, i32); 2],
    case_selection: &[Shape],
) -> Result<(), Box<dyn Error>> {
    run.session().replace_shape_for_control("b", source)?;
    run.session().set_object_selection_mode("b", 2, true)?;
    run.session().clear_selected_for_control()?;
    reselect(run, picks)?;
    require_selection_pair(run, roles, case_selection)
}

fn reselect(run: &mut SessionRun, picks: &[(i32, i32); 2]) -> Result<(), Box<dyn Error>> {
    use martensite_tkv3d::ais::AisSelectionScheme as Scheme;
    run.session().select_point(picks[0].0, picks[0].1)?;
    run.session().select_polyline(&[picks[1]], Scheme::Xor)?;
    Ok(())
}

fn require_selection_pair(
    run: &mut SessionRun,
    roles: &[Shape; 2],
    original: &[Shape],
) -> Result<(), Box<dyn Error>> {
    let selected = run.session().selected_sub_shapes_on("b")?;
    if identic_pair_matches(&selected, roles, original) {
        return Ok(());
    }
    Err("restored selected owners; expected the exact original relation endpoints in original order".into())
}

fn identic_pair_matches(selected: &[Shape], roles: &[Shape; 2], original: &[Shape]) -> bool {
    selected.len() == 2
        && original.len() == 2
        && selected
            .iter()
            .zip(roles)
            .all(|(actual, role)| actual.is_same(role))
        && selected
            .iter()
            .zip(original)
            .all(|(actual, old)| actual.is_same(old))
}

fn run_control_with_restore<C, T>(
    context: &mut C,
    capture: impl FnOnce(&mut C) -> Result<T, Box<dyn Error>>,
    restore: impl FnOnce(&mut C) -> Result<(), Box<dyn Error>>,
) -> Result<T, Box<dyn Error>> {
    let captured = capture(context);
    let restored = restore(context);
    merge_cleanup_results(captured, restored)
}

fn merge_cleanup_results<T>(
    capture: Result<T, Box<dyn Error>>,
    restore: Result<(), Box<dyn Error>>,
) -> Result<T, Box<dyn Error>> {
    match (capture, restore) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(primary), Ok(())) => Err(primary),
        (Ok(_), Err(cleanup)) => Err(cleanup),
        (Err(primary), Err(cleanup)) => Err(std::io::Error::other(format!(
            "control failed: {primary}; restoration failed: {cleanup}"
        ))
        .into()),
    }
}

fn restore_dump_and_check(run: &mut SessionRun, suffix: &str) -> Result<(), Box<dyn Error>> {
    crate::source_output::control_capture::control_dump(
        run.session(),
        "identic_edges.png",
        suffix,
    )?;
    run.verify_restored_control("identic_edges.png", suffix)
}

fn build_edge_remainder(
    source: &Shape,
    removed: &Shape,
) -> Result<(Shape, Vec<Shape>), EndpointControlRefusal> {
    let edges = all_distinct(source, ShapeEnum::Edge);
    let retained: Vec<_> = edges
        .into_iter()
        .filter(|edge| !edge.is_same(removed))
        .collect();
    if retained.is_empty() {
        return Err(EndpointControlRefusal::EmptyRemainder);
    }
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut result = compound.into_shape();
    for edge in &retained {
        builder
            .add(&mut result, edge)
            .map_err(|_| EndpointControlRefusal::ForeignOrAmbiguousEdge)?;
    }
    Ok((result, retained))
}

#[cfg(test)]
mod tests {
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
        let [first, second] =
            relation_endpoints(&source, edges[2].clone(), edges[8].clone()).unwrap();
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
        let foreign =
            all_distinct(&box_sized([11., 20., 30.]).unwrap(), ShapeEnum::Edge)[2].clone();
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
        let foreign =
            all_distinct(&box_sized([11., 20., 30.]).unwrap(), ShapeEnum::Edge)[2].clone();
        assert!(matches!(
            edge_remainder(&source, &foreign),
            Err(EndpointControlRefusal::ForeignOrAmbiguousEdge)
        ));
    }
}
