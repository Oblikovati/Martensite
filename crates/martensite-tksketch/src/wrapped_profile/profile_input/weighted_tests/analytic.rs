use super::*;

#[test]
fn rational_reference_analytic_circle_quarter_maps_to_its_native_angle() {
    let mut request = cylinder();
    request.profile = analytic_quarter(None);
    request.selections[0].outer.uses.pop();
    let result = normal_extrude_rational_cylinder(&request, 3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    assert_analytic_quarter(&result, QUARTER_RADIUS, true);
}

#[test]
fn rational_reference_analytic_ellipse_quarter_maps_reversed_source() {
    let mut request = cone();
    request.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: 100_000,
        };
    let minor = QUARTER_RADIUS * 0.5;
    request.profile = analytic_quarter(Some(minor));
    request.selections[0].outer.uses.pop();
    request.selections[0].outer.uses[0].reversed = true;
    let result = normal_extrude_rational_cone(&request, 3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    assert_analytic_quarter(&result, minor, false);
}

fn assert_analytic_quarter(result: &RationalWrappedResult, vertical_radius: f64, increasing: bool) {
    let expected = std::f64::consts::FRAC_PI_4 * QUARTER_RADIUS * vertical_radius;
    let area = result.authored_area();
    assert!(area.lower <= expected && expected <= area.upper);
    assert!(result.validation().is_valid());
    let expected_map = WrappedConicParameterMap {
        first_quarter: i8::from(!increasing),
        increasing,
    };
    assert!(result.history().iter().any(|record| {
        record.result.source
            == WrappedEntitySource::MappedSplitVertex {
                curve: SurfaceCurveId(1),
                point: WrappedSourcePoint {
                    numerator: 1,
                    level: 1,
                },
                map: expected_map,
            }
    }));
    assert!(result.history().iter().any(|record| matches!(record.result.source,
        WrappedEntitySource::MappedCurve { curve: SurfaceCurveId(1), span, map } if map == expected_map && span.reversed != increasing)));
    assert_eq!(
        result.export_native().unwrap().entities.len(),
        result.history().len()
    );
}

fn analytic_quarter(minor_radius: Option<f64>) -> SketchProfile {
    use martensite_tkbo::profile_extrusion::ArcSense;
    let mut profile = quarter(QUARTER_RADIUS);
    let center = [4., 0.];
    profile.curves[0] = match minor_radius {
        None => ProfileCurve::Arc {
            id: ProfileCurveId(1),
            start: ProfileVertexId(1),
            end: ProfileVertexId(2),
            center,
            sense: ArcSense::CounterClockwise,
        },
        Some(minor_radius) => {
            profile.vertices[1].point[1] = minor_radius;
            ProfileCurve::EllipticArc {
                id: ProfileCurveId(1),
                start: ProfileVertexId(2),
                end: ProfileVertexId(1),
                center,
                major_axis: [QUARTER_RADIUS, 0.],
                minor_radius,
                sense: ArcSense::Clockwise,
            }
        }
    };
    profile
}

#[test]
fn rational_reference_analytic_nonquarter_cylinder_preserves_nondyadic_angle() {
    let mut request = cylinder();
    request.profile = native_short_arc();
    request.selections[0].outer.uses.pop();
    let result = normal_extrude_rational_cylinder(&request, 2, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    assert_short_arc(&result);
}

#[test]
fn rational_reference_analytic_nonquarter_cone_has_independent_actual_checks() {
    use martensite_tkbo::body_check::{CheckMode, ValidationStage};
    let mut request = cone();
    request.profile = native_short_arc();
    request.selections[0].outer.uses.pop();
    request.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: 100_000,
        };
    let result = normal_extrude_rational_cone_with_selection(
        &request,
        2,
        super::strict_cone::V8_FAMILY_V3,
        || false,
    )
    .unwrap_or_else(|error| panic!("{error}"));
    assert_short_arc(&result);
    assert_eq!(
        result
            .validation()
            .stage(ValidationStage::SelfIntersection)
            .mode,
        CheckMode::Exact
    );
}

fn native_short_arc() -> SketchProfile {
    let mut profile = analytic_quarter(None);
    profile.vertices[0].point = [4. + 5. / 128., 0.];
    profile.vertices[1].point = [4. + 4. / 128., 3. / 128.];
    profile
}

fn assert_short_arc(result: &RationalWrappedResult) {
    let radius = 5. / 128.;
    let expected = 0.5 * radius * radius * martensite_cmath::atan2(3., 4.);
    let bounds = result.authored_area();
    assert!(bounds.lower <= expected && expected <= bounds.upper);
    assert!(result.area_transport_error().upper > 0.);
    assert!(result.area_transport_error().upper < 1e-13);
    assert!(result.validation().is_valid());
    let map = WrappedAnalyticArcMap {
        first: WrappedAnalyticArcEndpoint::Start,
        last: WrappedAnalyticArcEndpoint::End,
        increasing: true,
    };
    assert!(result.history().iter().any(|record| record.result.source
        == WrappedEntitySource::AnalyticArcSplitVertex {
            curve: SurfaceCurveId(1),
            point: WrappedSourcePoint {
                numerator: 1,
                level: 1
            },
            map,
        }));
    assert_eq!(
        result.export_native().unwrap().entities.len(),
        result.history().len()
    );
}
