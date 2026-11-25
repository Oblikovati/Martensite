//! Full analytic curves use their native angle anchors instead of synthetic authored vertices.
use super::*;

#[test]
fn rational_reference_cylinder_keeps_closed_circle_angular_seam() {
    let mut request = cylinder();
    request.profile = closed(ProfileCurve::Circle {
        id: ProfileCurveId(1),
        center: [4., 0.],
        radius: QUARTER_RADIUS,
    });
    request.selections[0].outer.uses.truncate(1);
    let result = normal_extrude_rational_cylinder(&request, 3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    validate_closed(&result, QUARTER_RADIUS, QUARTER_RADIUS);
}

#[test]
#[ignore = "legacy conical native search is an expensive comparison control"]
fn rational_reference_cone_keeps_closed_ellipse_native_anchors() {
    let [major, minor] = [1. / 256., 1. / 512.];
    let request = ellipse_request(major, minor);
    let result = normal_extrude_rational_cone(&request, 2, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    validate_closed(&result, major, minor);
}

#[test]
#[ignore = "larger full ellipse at 32 reference spans retains its original work refusal"]
fn rational_reference_large_ellipse_cone_keeps_the_typed_uv_work_refusal() {
    let request = ellipse_request(QUARTER_RADIUS, QUARTER_RADIUS / 2.);
    let failure = normal_extrude_rational_cone(&request, 3, || false)
        .err()
        .unwrap();
    assert!(matches!(failure, RationalWrappedFailure::Wrapped(cause)
        if cause.class() == WrappedFailureClass::Incomplete && cause.stage() == WrappedStage::Construction));
}

pub(super) fn ellipse_request(major: f64, minor: f64) -> WrappedConeNormalRequest {
    let mut request = cone();
    request.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: 100_000,
        };
    request.profile = closed(ProfileCurve::Ellipse {
        id: ProfileCurveId(1),
        center: [4., 0.],
        major_axis: [major, 0.],
        minor_radius: minor,
    });
    request.selections[0].outer.uses.truncate(1);
    request
}

fn closed(curve: ProfileCurve) -> SketchProfile {
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: vec![],
        curves: vec![curve],
    }
}

pub(super) fn validate_closed(result: &RationalWrappedResult, major: f64, minor: f64) {
    let expected = std::f64::consts::PI * major * minor;
    let bounds = result.authored_area();
    assert!(bounds.lower <= expected && expected <= bounds.upper);
    assert!(result.validation().is_valid());
    assert!(result.authored_profile().vertices.is_empty());
    for quarter in 0..4 {
        assert!(result.history().iter().any(|record| record.result.source
            == WrappedEntitySource::ConicAnchor {
                curve: SurfaceCurveId(1),
                quarter,
            }));
        assert!(
            result
                .history()
                .iter()
                .any(|record| matches!(record.result.source,
            WrappedEntitySource::MappedCurve {curve: SurfaceCurveId(1), map, .. }
            if map.first_quarter == quarter && map.increasing))
        );
    }
    assert!(
        !result
            .history()
            .iter()
            .any(|record| matches!(record.result.source, WrappedEntitySource::Vertex(_)))
    );
    assert_eq!(
        result.export_native().unwrap().entities.len(),
        result.history().len()
    );
}
