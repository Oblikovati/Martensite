//! Model a curved joint while retaining each side's authored curve and parameter map.
use super::*;
use martensite_tkbo::profile_extrusion::ArcSense;

#[test]
fn rational_reference_cylinder_keeps_two_analytic_occurrences() {
    let mut request = cylinder();
    request.profile = half_round(false);
    request.selections[0].outer.uses.pop();
    let result = normal_extrude_rational_cylinder(&request, 3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    check_half_round(&result);
    mapped_source(&result, 1, 0);
    mapped_source(&result, 2, 1);
}

#[test]
fn rational_reference_cone_keeps_rational_and_analytic_occurrences() {
    let mut request = cone();
    request.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: 100_000,
        };
    request.profile = half_round(true);
    request.selections[0].outer.uses.pop();
    let result = normal_extrude_rational_cone(&request, 3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    check_half_round(&result);
    assert_authored_curve(result.history());
    mapped_source(&result, 2, 1);
}

#[test]
fn rational_reference_cylinder_keeps_polynomial_and_analytic_occurrences() {
    let mut request = cylinder();
    request.profile = half_round(true);
    request.selections[0].outer.uses.pop();
    let ProfileCurve::Spline { weights, .. } = &mut request.profile.curves[0] else {
        panic!("expected quadratic Bernstein fixture");
    };
    *weights = None;
    let result = normal_extrude_rational_cylinder(&request, 3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    let expected = (5. / 6. + std::f64::consts::FRAC_PI_4) * QUARTER_RADIUS * QUARTER_RADIUS;
    let area = result.authored_area();
    assert!(area.lower <= expected && expected <= area.upper);
    assert!(result.validation().is_valid());
    assert_authored_curve(result.history());
    mapped_source(&result, 2, 1);
}

fn mapped_source(result: &RationalWrappedResult, expected_id: u64, first_quarter: i8) {
    assert!(result.history().iter().any(|record| matches!(record.result.source,
        WrappedEntitySource::MappedSplitVertex { curve, point: WrappedSourcePoint { numerator: 1, level: 1 },
            map: WrappedConicParameterMap { first_quarter: start, increasing: true } }
        if curve == SurfaceCurveId(expected_id) && start == first_quarter)));
}

pub(super) fn check_half_round(result: &RationalWrappedResult) {
    let expected = std::f64::consts::FRAC_PI_2 * QUARTER_RADIUS * QUARTER_RADIUS;
    let bounds = result.authored_area();
    assert!(bounds.lower <= expected && expected <= bounds.upper);
    assert!(result.validation().is_valid());
    assert!(result.history().iter().any(|record| record.result.source
        == WrappedEntitySource::Vertex(crate::surface_sketch::SurfaceVertexId(2))));
    assert_eq!(
        result.export_native().unwrap().entities.len(),
        result.history().len()
    );
}

pub(super) fn half_round(rational_first: bool) -> SketchProfile {
    let mut profile = quarter(QUARTER_RADIUS);
    profile.vertices[2].point[0] = 4. - QUARTER_RADIUS;
    if !rational_first {
        profile.curves[0] = analytic_arc(1, 1, 2);
    }
    profile.curves[1] = analytic_arc(2, 2, 3);
    profile
}

fn analytic_arc(curve: u64, first: u64, last: u64) -> ProfileCurve {
    ProfileCurve::Arc {
        id: ProfileCurveId(curve),
        start: ProfileVertexId(first),
        end: ProfileVertexId(last),
        center: [4., 0.],
        sense: ArcSense::CounterClockwise,
    }
}
