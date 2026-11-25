// SPDX-License-Identifier: GPL-2.0-only
//! Exercise a retained nonuniform native rational spline together with an analytic arc.
use super::*;
use martensite_tkbo::{
    body_check::{CheckMode, ValidationStage},
    profile_extrusion::PlanarProofVersion,
};

// Independent 70-decimal de Boor evaluation and boundary integral in centered
// coordinates, splitting at native u=1 and adding the analytic quarter sector.
const AUTHORED_AREA: f64 = 0.006_322_412_778_399_602;

#[test]
fn rational_reference_native_knots_and_conic_extrude_on_a_cylinder() {
    let mut request = cylinder();
    request.profile = spline_arc_profile();
    request.selections[0].outer.uses.pop();
    let result = normal_extrude_rational_cylinder(&request, 2, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    let cylinder_volume = result.authored_volume();
    assert!(
        cylinder_volume.lower <= AUTHORED_AREA / 5. && AUTHORED_AREA / 5. <= cylinder_volume.upper
    );
    check_native_lineage(&result);
}

#[test]
fn rational_reference_native_knots_and_conic_extrude_with_exact_cone_checks() {
    let mut request = cone();
    request.proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 100_000,
    };
    request.profile = spline_arc_profile();
    request.selections[0].outer.uses.pop();
    let result =
        normal_extrude_rational_cone_with_selection(&request, 2, strict_cone::V8_FAMILY_V3, || {
            false
        })
        .unwrap_or_else(|error| panic!("{error}"));
    check_native_lineage(&result);
    assert_eq!(
        result
            .validation()
            .stage(ValidationStage::SelfIntersection)
            .mode,
        CheckMode::Exact
    );
}

fn spline_arc_profile() -> SketchProfile {
    let r = QUARTER_RADIUS;
    let mut profile = mixed_curves::half_round(true);
    profile.curves[0] = ProfileCurve::Spline {
        id: ProfileCurveId(1),
        start: ProfileVertexId(1),
        end: ProfileVertexId(2),
        degree: 2,
        poles: vec![[4. + r, 0.], [4. + r, r / 2.], [4. + r / 2., r], [4., r]],
        weights: Some(vec![1., 1., 1.5, 1.75]),
        knots: vec![-2., -2., -2., 1., 7., 7., 7.],
    };
    profile
}

fn check_native_lineage(result: &RationalWrappedResult) {
    let area = result.authored_area();
    assert!(
        area.lower <= AUTHORED_AREA && AUTHORED_AREA <= area.upper,
        "{area:?}"
    );
    let area_transport = result.area_transport_error().upper;
    assert!(area_transport > 0. && area_transport < 1e-13);
    assert!(result.validation().is_valid());
    assert_eq!(
        result.export_native().unwrap().entities.len(),
        result.history().len()
    );
    let sources: Vec<_> = result
        .history()
        .iter()
        .map(|record| record.result.source)
        .collect();
    for interval in [[-2., 1.], [1., 7.]] {
        assert!(sources.iter().any(
            |source| matches!(source, WrappedEntitySource::NativeSplineCurve {
            curve: SurfaceCurveId(1), map, span: WrappedSourceSpan { level: 2, .. },
        } if map.knots.map(crate::surface_sketch::WrappedNativeKnot::value) == interval)
        ));
        assert!(sources.iter().any(
            |source| matches!(source, WrappedEntitySource::NativeSplineSplitVertex {
            curve: SurfaceCurveId(1), map, point: WrappedSourcePoint { level: 1, numerator: 1 },
        } if map.knots.map(crate::surface_sketch::WrappedNativeKnot::value) == interval)
        ));
    }
    assert!(sources.iter().any(
        |source| matches!(source, WrappedEntitySource::NativeSplineKnot {
        curve: SurfaceCurveId(1), knot,
    } if knot.value() == 1.)
    ));
    assert!(sources.iter().any(|source| matches!(
        source,
        WrappedEntitySource::MappedCurve {
            curve: SurfaceCurveId(2),
            ..
        }
    )));
    assert_eq!(
        result.authored_profile().curves[0],
        spline_arc_profile().curves[0]
    );
}
