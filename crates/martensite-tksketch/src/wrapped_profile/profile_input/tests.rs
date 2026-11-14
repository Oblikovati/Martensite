//! Directed selection normalizes traversal while preserving the original source interval.
use super::*;
use martensite_tkbo::profile_extrusion::ProfileVertex;
const AUTHORED_INTERVAL_CASES: [(u8, u32, u64); 5] = [
    (0, 0, 0),
    (1, 0, 1),
    (3, 2, 5),
    (31, u32::MAX / 2, 0),
    (63, u32::MAX, 9_223_372_032_559_808_512),
];

fn fixture() -> (Request, SketchProfile, ProfileSelection) {
    let request = super::super::tests::request();
    let profile = profile(&request);
    let selection = ProfileSelection {
        region: request.region,
        outer: contour(&request.loops[0]),
        holes: request.loops[1..].iter().map(|c| contour(c)).collect(),
    };
    (request, profile, selection)
}
fn profile(request: &Request) -> SketchProfile {
    let vertices: BTreeMap<_, _> = request
        .loops
        .iter()
        .flatten()
        .map(|p| (vertex(p.start), p.poles[0]))
        .collect();
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: vertices
            .into_iter()
            .map(|(id, point)| ProfileVertex { id, point })
            .collect(),
        curves: request.loops.iter().flatten().map(profile_curve).collect(),
    }
}
fn vertex(corner: CornerSource) -> ProfileVertexId {
    match corner {
        CornerSource::Authored(id) => ProfileVertexId(id.0),
        other => panic!("fixture corner={other:?}; expected an authored endpoint"),
    }
}
fn profile_curve(piece: &AuthoredPiece) -> ProfileCurve {
    let id = ProfileCurveId(piece.span.curve.0);
    let (start, end) = (vertex(piece.start), vertex(piece.end));
    if piece.poles.len() == 2 {
        return ProfileCurve::Line { id, start, end };
    }
    let count = piece.poles.len();
    ProfileCurve::Spline {
        id,
        start,
        end,
        degree: (count - 1) as u32,
        poles: piece.poles.clone(),
        weights: None,
        knots: std::iter::repeat_n(0., count)
            .chain(std::iter::repeat_n(1., count))
            .collect(),
    }
}
fn contour(pieces: &[AuthoredPiece]) -> ProfileLoop {
    ProfileLoop {
        uses: pieces
            .iter()
            .map(|p| LoopUse {
                curve: ProfileCurveId(p.span.curve.0),
                reversed: false,
            })
            .collect(),
    }
}
fn assert_geometry(actual: &OrientedRegion, expected: &Request) {
    assert_eq!(actual.region, expected.region);
    assert_eq!(actual.loops.len(), expected.loops.len());
    for (actual, expected) in actual.loops.iter().zip(&expected.loops) {
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.iter().zip(expected) {
            assert_piece(actual, expected);
        }
    }
}
fn assert_piece(actual: &AuthoredPiece, expected: &AuthoredPiece) {
    assert_eq!(actual.span, expected.span);
    assert_eq!(actual.start, expected.start);
    assert_eq!(actual.end, expected.end);
    let bits = |poles: &[[f64; 2]]| {
        poles
            .iter()
            .map(|p| p.map(f64::to_bits))
            .collect::<Vec<_>>()
    };
    assert_eq!(bits(&actual.poles), bits(&expected.poles));
}

#[test]
fn mixed_curves_and_holes_retain_exact_selected_source_controls() {
    let (request, profile, selection) = fixture();
    let oriented = checked_input(&profile, &selection).unwrap();
    assert_geometry(&oriented, &request);
    assert!(oriented.directions.0.values().all(|reversed| !reversed));
}

#[test]
fn later_source_edits_do_not_change_owned_geometry_or_parameter_directions() {
    let (request, mut profile, mut selection) = fixture();
    let oriented = checked_input(&profile, &selection).unwrap();
    profile.vertices[0].point = [99.; 2];
    selection.outer.uses[0].reversed = true;
    assert_geometry(&oriented, &request);
    assert!(oriented.directions.0.values().all(|reversed| !reversed));
    drop(profile);
    drop(selection);
    assert_geometry(&oriented, &request);
}

#[test]
fn solved_profile_selection_drives_the_existing_budgeted_constructor() {
    use martensite_tkmath::predicates::roots::metric_bounds::CylindricalMeasureLimit;
    let (mut request, profile, selection) = fixture();
    let oriented = checked_input(&profile, &selection).unwrap();
    request.loops = oriented.loops;
    let result = family::construct_budgeted(&[request], CylindricalMeasureLimit::Refuse(1e-4))
        .unwrap_or_else(|cause| panic!("{cause}"));
    assert_eq!(result.family.members.len(), 1);
    assert!(result.budget.volume.discrepancy <= 1e-4);
    assert!(result.budget.volume.representative_error <= 1e-4);
    assert_authored_intervals(&result, &oriented.directions);
}

fn assert_authored_intervals(result: &family::BudgetedFamily, directions: &CurveDirections) {
    for key in &result.family.keys {
        if let Source::Span(span) = key.source {
            let original = directions.source_interval(span).unwrap();
            assert_eq!((original.curve, original.level), (span.curve, span.level));
            assert_eq!(original.index, u64::from(span.index));
            assert!(!original.reversed);
        }
    }
}

#[test]
fn reversed_authored_curve_keeps_geometry_and_maps_history_to_original_parameters() {
    let (request, mut profile, mut selection) = fixture();
    let id = profile.curves[1].id();
    reverse_source(&mut profile.curves[1]);
    selection.outer.uses[1].reversed = true;
    let oriented = checked_input(&profile, &selection).unwrap();
    assert_geometry(&oriented, &request);
    check_extreme_intervals(&oriented, SurfaceCurveId(id.0), true);
}
fn reverse_source(curve: &mut ProfileCurve) {
    match curve {
        ProfileCurve::Line { start, end, .. } => std::mem::swap(start, end),
        ProfileCurve::Spline {
            start, end, poles, ..
        } => {
            std::mem::swap(start, end);
            poles.reverse();
        }
        other => panic!("fixture curve={other:?}; expected a line or polynomial Bezier"),
    }
}

#[test]
fn dyadic_source_mapping_refuses_unknown_and_out_of_domain_spans() {
    let (_, profile, selection) = fixture();
    let oriented = checked_input(&profile, &selection).unwrap();
    let curve = SurfaceCurveId(profile.curves[0].id().0);
    for (level, index) in [(0, 1), (3, 8), (64, 0), (255, 0)] {
        assert!(matches!(
            oriented
                .directions
                .source_interval(CurveSpan {
                    curve,
                    level,
                    index
                })
                .inspect_err(assert_diagnostic),
            Err(ProfileInputFailure::Interval(_))
        ));
    }
}

#[test]
fn unselected_curve_has_no_authored_parameter_mapping() {
    let (_, profile, selection) = fixture();
    let oriented = checked_input(&profile, &selection).unwrap();
    assert!(matches!(
        oriented
            .directions
            .source_interval(CurveSpan {
                curve: SurfaceCurveId(u64::MAX),
                level: 0,
                index: 0,
            })
            .inspect_err(assert_diagnostic),
        Err(ProfileInputFailure::UnknownSource(_))
    ));
}

#[test]
fn canonical_source_intervals_keep_exact_indices_at_level_63() {
    for reversed in [false, true] {
        let (_, mut profile, mut selection) = fixture();
        let curve = SurfaceCurveId(profile.curves[0].id().0);
        if reversed {
            reverse_source(&mut profile.curves[0]);
            selection.outer.uses[0].reversed = true;
        }
        let oriented = checked_input(&profile, &selection).unwrap();
        check_extreme_intervals(&oriented, curve, reversed);
    }
}

fn check_extreme_intervals(region: &OrientedRegion, curve: SurfaceCurveId, reversed: bool) {
    for (level, index, mapped) in AUTHORED_INTERVAL_CASES {
        let interval = region
            .directions
            .source_interval(CurveSpan {
                curve,
                level,
                index,
            })
            .unwrap();
        let expected = if reversed { mapped } else { u64::from(index) };
        assert_eq!((interval.curve, interval.level), (curve, level));
        assert_eq!((interval.index, interval.reversed), (expected, reversed));
    }
}

#[test]
fn duplicate_and_missing_curve_ids_are_refused() {
    let (_, mut profile, mut selection) = fixture();
    profile.curves.push(profile.curves[0].clone());
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::DuplicateCurve(_))
    ));
    profile.curves.pop();
    selection.outer.uses[0].curve = ProfileCurveId(u64::MAX);
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::MissingCurve(_))
    ));
}

#[test]
fn duplicate_and_missing_vertex_ids_are_refused() {
    let (_, mut profile, selection) = fixture();
    profile.vertices.push(profile.vertices[0]);
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::DuplicateVertex(_))
    ));
    profile.vertices.pop();
    profile.vertices.remove(0);
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::MissingVertex { .. })
    ));
}

#[test]
fn reused_curves_empty_loops_and_world_placement_are_not_silently_normalized() {
    let (_, mut profile, mut selection) = fixture();
    selection.outer.uses.push(selection.outer.uses[0]);
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::ReusedCurve(_))
    ));
    selection.outer.uses.clear();
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::EmptyLoop { .. })
    ));
    profile.plane.origin[0] = 1.;
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::Plane(_))
    ));
}

#[test]
fn control_changes_and_nonfinite_values_do_not_snap_to_vertices() {
    let (_, mut profile, selection) = fixture();
    let ProfileCurve::Spline { poles, .. } = &mut profile.curves[1] else {
        panic!("spline fixture")
    };
    poles[0][0] += 0.01;
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::Endpoint { .. })
    ));
    let ProfileCurve::Spline { poles, .. } = &mut profile.curves[1] else {
        panic!("spline fixture")
    };
    poles[0][0] = f64::NAN;
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::NonFinite { .. })
    ));
}

#[path = "point_tests.rs"]
mod points;
#[path = "spline_tests.rs"]
mod splines;

fn checked_input(profile: &SketchProfile, selection: &ProfileSelection) -> Checked<OrientedRegion> {
    from_profile(profile, selection).inspect_err(assert_diagnostic)
}
fn assert_diagnostic(cause: &ProfileInputFailure) {
    let message = cause.to_string();
    assert!(message.starts_with("wrapped input "));
    assert!(message.contains("; expected "));
}
