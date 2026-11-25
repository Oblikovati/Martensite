//! A curved selected region keeps its independently authored clockwise inner boundary.
use super::*;

#[test]
fn rational_reference_cylinder_preserves_a_directed_hole() {
    let mut request = cylinder();
    (request.profile, request.selections) = holed_quarter();
    let result = normal_extrude_rational_cylinder(&request, 3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    assert_hole(&result);
}

#[test]
fn rational_reference_cone_preserves_a_directed_hole() {
    let mut request = cone();
    request.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: 100_000,
        };
    (request.profile, request.selections) = holed_quarter();
    let result = normal_extrude_rational_cone(&request, 3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    assert_hole(&result);
}

fn assert_hole(result: &RationalWrappedResult) {
    let outer = std::f64::consts::FRAC_PI_4 * QUARTER_RADIUS * QUARTER_RADIUS;
    let opening = 0.5 * (QUARTER_RADIUS / 4.).powi(2);
    let expected = outer - opening;
    let area = result.authored_area();
    assert!(area.lower <= expected && expected <= area.upper);
    let volume = result.authored_volume();
    assert!(volume.lower <= expected * 0.2 && expected * 0.2 <= volume.upper);
    assert!(result.validation().is_valid());
    assert_eq!(result.authored_selection().holes.len(), 1);
    for id in 4..=6 {
        assert!(
            result
                .history()
                .iter()
                .any(|record| matches!(record.result.source,
            WrappedEntitySource::Curve { curve, .. } if curve == SurfaceCurveId(id)))
        );
    }
    assert_eq!(
        result.export_native().unwrap().entities.len(),
        result.history().len()
    );
}

fn holed_quarter() -> (SketchProfile, Vec<WrappedRegionSelection>) {
    let mut profile = quarter(QUARTER_RADIUS);
    let mut selection = selection();
    selection.outer.uses.pop();
    let spacing = QUARTER_RADIUS / 4.;
    let points = [
        [4. + spacing, spacing],
        [4. + spacing, 2. * spacing],
        [4. + 2. * spacing, spacing],
    ];
    let uses = (4..=6)
        .zip(points)
        .map(|(id, point)| {
            let end = if id == 6 { 4 } else { id + 1 };
            profile.vertices.push(ProfileVertex {
                id: ProfileVertexId(id),
                point,
            });
            profile.curves.push(ProfileCurve::Line {
                id: ProfileCurveId(id),
                start: ProfileVertexId(id),
                end: ProfileVertexId(end),
            });
            LoopUse {
                curve: ProfileCurveId(id),
                reversed: false,
            }
        })
        .collect();
    selection.holes.push(ProfileLoop { uses });
    (profile, vec![selection])
}

#[test]
fn rational_reference_cylinder_preserves_an_authored_closed_spline_hole() {
    let mut request = cylinder();
    (request.profile, request.selections) = closed_spline_hole();
    let result = normal_extrude_rational_cylinder(&request, 3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    let expected = 0.25 - std::f64::consts::PI * QUARTER_RADIUS * QUARTER_RADIUS;
    let area = result.authored_area();
    assert!(area.lower <= expected && expected <= area.upper);
    assert!(result.validation().is_valid());
    assert!(result.authored_selection().holes[0].uses[0].reversed);
    assert_closed_spline_history(&result);
}

fn closed_spline_hole() -> (SketchProfile, Vec<WrappedRegionSelection>) {
    let mut profile = closed_rational_spline();
    let points = [[3.75, -0.25], [4.25, -0.25], [4.25, 0.25], [3.75, 0.25]];
    let outer = (2..=5)
        .zip(points)
        .map(|(id, point)| {
            let end = if id == 5 { 2 } else { id + 1 };
            profile.vertices.push(ProfileVertex {
                id: ProfileVertexId(id),
                point,
            });
            profile.curves.push(ProfileCurve::Line {
                id: ProfileCurveId(id),
                start: ProfileVertexId(id),
                end: ProfileVertexId(end),
            });
            LoopUse {
                curve: ProfileCurveId(id),
                reversed: false,
            }
        })
        .collect();
    let selection = WrappedRegionSelection {
        region: REGION,
        outer: ProfileLoop { uses: outer },
        holes: vec![ProfileLoop {
            uses: vec![LoopUse {
                curve: ProfileCurveId(1),
                reversed: true,
            }],
        }],
    };
    (profile, vec![selection])
}
