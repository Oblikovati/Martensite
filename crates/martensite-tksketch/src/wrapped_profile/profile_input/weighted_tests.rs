//! Certify an authored weighted cubic whose denominator cancels to a parabolic span.
use super::*;
use crate::surface_sketch::{CylinderSupport, HistorySource, NormalSense};
use martensite_tkbo::profile_extrusion::{PlanarProofWorkLimit, ProfileVertex};

const REGION: SurfaceRegionId = SurfaceRegionId(1);
const AREA: f64 = 215. / 192.;
const VOLUME: f64 = 43. / 192.;

#[test]
fn weighted_polynomial_extrudes_on_a_cylinder_with_authored_history() {
    let request = cylinder();
    let result = normal_extrude_profile(&request).unwrap_or_else(|error| panic!("{error}"));
    let report = &result.reports()[&REGION];
    assert!(report.validation.is_valid());
    let volume = report.measures.authored_volume;
    assert!(volume.lower <= VOLUME && volume.upper >= VOLUME);
    assert_authored_curve(result.history());

    let previous = result.encoded_brep().to_vec();
    let replayed = normal_extrude_profile(&request).unwrap();
    assert_eq!(previous, replayed.encoded_brep());
    assert_eq!(result.history(), replayed.history());
}

#[test]
fn weighted_polynomial_extrudes_on_a_cone_with_authored_history() {
    let request = cone();
    let result = normal_extrude_cone_profile(&request).unwrap_or_else(|error| panic!("{error}"));
    let report = &result.reports()[&REGION];
    assert!(report.validation.is_valid());
    assert!(report.planar_area.encloses(AREA));
    let volume = report.authored_volume;
    assert!(volume.lower <= VOLUME && volume.upper >= VOLUME);
    assert_authored_curve(result.history());
}

#[test]
fn nonpolynomial_source_and_cancellation_are_typed() {
    let mut request = cylinder();
    let ProfileCurve::Spline { poles, .. } = &mut request.profile.curves[0] else {
        panic!("expected weighted fixture");
    };
    poles[2][1] = poles[2][1].next_up();
    let refused = normal_extrude_profile(&request).err().unwrap();
    assert_eq!(refused.class(), WrappedFailureClass::Unsupported);
    assert!(
        refused
            .entities()
            .contains(&HistorySource::Curve(SurfaceCurveId(1)))
    );

    let mut polls = 0;
    let stopped = normal_extrude_profile_with_cancel(&cylinder(), || {
        polls += 1;
        polls == 3
    })
    .err()
    .unwrap();
    assert_eq!(stopped.class(), WrappedFailureClass::Cancelled);
}

#[test]
fn planar_evidence_retains_the_actual_weights_and_reversed_parameterization() {
    let mut request = cylinder();
    let area = request.selections.first_mut().unwrap();
    area.outer.uses.reverse();
    for used in &mut area.outer.uses {
        used.reversed = true;
    }
    let input = super::super::api::input::read(&request, &mut || false).unwrap();
    let source = &input.requests[0].source;
    let captured = planar::combined_profile(&[source]).unwrap();
    let ProfileCurve::Spline {
        weights: None,
        poles,
        ..
    } = &captured.curves[0]
    else {
        panic!("expected the proven polynomial in reversed authored traversal");
    };
    assert_eq!(poles[0], [5.25, 0.3125]);
    assert_eq!(poles[2], [4., 0.]);

    let before = planar::source::SourceSnapshot::capture(source);
    let ProfileCurve::Spline {
        weights: Some(weights),
        ..
    } = &mut request.profile.curves[0]
    else {
        panic!("expected the authored weighted span");
    };
    weights.iter_mut().for_each(|weight| *weight *= 2.);
    let edited = super::super::api::input::read(&request, &mut || false).unwrap();
    assert_eq!(
        source.loops[0][3].poles,
        edited.requests[0].loops[0][3].poles
    );
    assert!(!before.matches(&edited.requests[0].source));
}

fn assert_authored_curve(history: &[WrappedHistoryRecord]) {
    assert!(history.iter().any(|record| matches!(
        record.result.source,
        WrappedEntitySource::Curve {
            curve: SurfaceCurveId(1),
            ..
        }
    )));
}

fn cylinder() -> WrappedNormalRequest {
    WrappedNormalRequest {
        profile: profile(),
        selections: vec![selection()],
        support: CylinderSupport {
            origin: [0.; 3],
            axis: [0., 0., 1.],
            radial: [1., 0., 0.],
            radius: 10.,
            chart_origin: 0.,
            normal: NormalSense::Outward,
        },
        depths: [-0.1, 0.1],
        options: options(),
    }
}

fn cone() -> WrappedConeNormalRequest {
    WrappedConeNormalRequest {
        profile: profile(),
        selections: vec![selection()],
        support: crate::surface_sketch::ConeSupport {
            apex: [0.; 3],
            axis: [0., 0., 1.],
            radial: [1., 0., 0.],
            semi_angle: 0.7,
            chart_origin: 0.,
            normal: NormalSense::Outward,
        },
        depths: [-0.1, 0.1],
        options: options(),
        proof: WrappedConeProofOptions::default(),
    }
}

fn options() -> WrappedNormalOptions {
    WrappedNormalOptions {
        spatial_error: 1e-4,
        planar_work: PlanarProofWorkLimit::CurrentDefault,
        maximum_fit_level: 8,
        volume: WrappedVolumePolicy::Report,
    }
}

fn selection() -> WrappedRegionSelection {
    WrappedRegionSelection {
        region: REGION,
        outer: ProfileLoop {
            uses: (1..=4)
                .map(|id| LoopUse {
                    curve: ProfileCurveId(id),
                    reversed: false,
                })
                .collect(),
        },
        holes: vec![],
    }
}

fn profile() -> SketchProfile {
    let points = [[4., 0.], [5.25, 0.3125], [5.25, 1.], [4., 1.]];
    let vertices = (1..=4)
        .zip(points)
        .map(|(id, point)| ProfileVertex {
            id: ProfileVertexId(id),
            point,
        })
        .collect();
    let mut curves = vec![parabola()];
    curves.extend(
        [(2, 3), (3, 4), (4, 1)].map(|(start, end)| ProfileCurve::Line {
            id: ProfileCurveId(start),
            start: ProfileVertexId(start),
            end: ProfileVertexId(end),
        }),
    );
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices,
        curves,
    }
}

fn parabola() -> ProfileCurve {
    // W=3+3t multiplies x=4+5t/4 and y=5t²/16. Every supplied control is exact.
    ProfileCurve::Spline {
        id: ProfileCurveId(1),
        start: ProfileVertexId(1),
        end: ProfileVertexId(2),
        degree: 3,
        poles: vec![[4., 0.], [4.3125, 0.], [4.75, 0.0625], [5.25, 0.3125]],
        weights: Some(vec![3., 4., 5., 6.]),
        knots: vec![0., 0., 0., 0., 1., 1., 1., 1.],
    }
}

fn quarter(radius: f64) -> SketchProfile {
    let points = [[4. + radius, 0.], [4., radius], [4., 0.]];
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: (1..=3)
            .zip(points)
            .map(|(id, point)| ProfileVertex {
                id: ProfileVertexId(id),
                point,
            })
            .collect(),
        curves: vec![
            ProfileCurve::Spline {
                id: ProfileCurveId(1),
                start: ProfileVertexId(1),
                end: ProfileVertexId(2),
                degree: 2,
                poles: vec![points[0], [4. + radius, radius], points[1]],
                weights: Some(vec![1., 1., 2.]),
                knots: vec![0., 0., 0., 1., 1., 1.],
            },
            ProfileCurve::Line {
                id: ProfileCurveId(2),
                start: ProfileVertexId(2),
                end: ProfileVertexId(3),
            },
            ProfileCurve::Line {
                id: ProfileCurveId(3),
                start: ProfileVertexId(3),
                end: ProfileVertexId(1),
            },
        ],
    }
}

const QUARTER_RADIUS: f64 = 1. / 16.;

#[test]
fn rational_reference_quarter_extrudes_with_original_curve_parameter_history() {
    let mut request = cylinder();
    request.profile = quarter(QUARTER_RADIUS);
    request.selections[0].outer.uses.pop();
    let result = normal_extrude_rational_cylinder(&request, 3, || false).unwrap();
    assert_rational_quarter(&result, QUARTER_RADIUS, request.options.spatial_error);
    let replay = normal_extrude_rational_cylinder(&request, 3, || false).unwrap();
    assert_eq!(result.encoded_brep(), replay.encoded_brep());
    assert_eq!(result.history(), replay.history());
    assert_eq!(result.authored_selection(), &request.selections[0]);
    request.profile.curves.clear();
    assert_eq!(result.authored_profile(), &quarter(QUARTER_RADIUS));
    assert_rational_publication_thread_safety::<RationalWrappedResult>();
}

#[test]
fn rational_reference_refuses_coarse_subdivision_and_cancellation() {
    let mut request = cylinder();
    request.profile = quarter(QUARTER_RADIUS);
    request.selections[0].outer.uses.pop();
    let cancelled = normal_extrude_rational_cylinder(&request, 3, || true)
        .err()
        .unwrap();
    assert!(matches!(cancelled, RationalWrappedFailure::Wrapped(cause)
        if cause.class() == WrappedFailureClass::Cancelled));
    let coarse = normal_extrude_rational_cylinder(&request, 1, || false)
        .err()
        .unwrap();
    assert!(matches!(coarse, RationalWrappedFailure::Reference(cause)
        if matches!(*cause, martensite_tkmath::predicates::roots::metric_bounds::RationalReferenceFailure::Exceeds { .. })));
}

fn assert_rational_publication_thread_safety<T: Send + Sync>() {}

#[test]
fn rational_reference_quarter_extrudes_on_cone_away_from_apex() {
    let mut request = cone();
    request.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: 100_000,
        };
    request.profile = quarter(QUARTER_RADIUS);
    request.selections[0].outer.uses.pop();
    let result = normal_extrude_rational_cone(&request, 3, || false).unwrap();
    assert_rational_quarter(&result, QUARTER_RADIUS, request.options.spatial_error);
}

#[test]
#[ignore = "unit-radius qualification control; retained exact proof is expensive in debug"]
fn rational_reference_unit_radius_quarter_byte_and_history_replay() {
    let mut request = cylinder();
    request.profile = quarter(1.);
    request.selections[0].outer.uses.pop();
    let result = normal_extrude_rational_cylinder(&request, 4, || false).unwrap();
    assert_rational_quarter(&result, 1., request.options.spatial_error);
    let replay = normal_extrude_rational_cylinder(&request, 4, || false).unwrap();
    assert_eq!(result.encoded_brep(), replay.encoded_brep());
    assert_eq!(result.history(), replay.history());
}

fn assert_rational_quarter(result: &RationalWrappedResult, radius: f64, limit: f64) {
    let expected = std::f64::consts::FRAC_PI_4 * radius * radius;
    let area = result.authored_area();
    assert!(area.lower <= expected && expected <= area.upper);
    let volume = result.authored_volume();
    let expected = expected * 0.2;
    assert!(volume.lower <= expected && expected <= volume.upper);
    assert!(result.spatial_error().upper <= limit);
    assert!(result.validation().is_valid());
    assert!(result.authored_area_receipt().integration_subintervals() > 0);
    assert!(result.authored_planar_work().effective_pair_bisections > 0);
    assert!(result.boundary_flux().lower > 0.);
    assert!(result.volume_discrepancy().upper > 0.);
    assert_original_quarter_history(result);
}

fn assert_original_quarter_history(result: &RationalWrappedResult) {
    assert!(result.history().iter().any(|record| matches!(
        record.result.source,
        WrappedEntitySource::SplitVertex {
            curve: SurfaceCurveId(1),
            point: WrappedSourcePoint {
                numerator: 1,
                level: 1
            }
        }
    )));
    assert!(
        result
            .history()
            .iter()
            .all(|record| match record.result.source {
                WrappedEntitySource::Curve { curve, .. }
                | WrappedEntitySource::SplitVertex { curve, .. } => (1..=3).contains(&curve.0),
                WrappedEntitySource::Vertex(vertex) => (1..=3).contains(&vertex.0),
                _ => true,
            })
    );
    assert_eq!(
        result.export_native().unwrap().entities.len(),
        result.history().len()
    );
}

#[test]
fn rational_reference_closed_multipart_spline_keeps_native_knots_and_seam() {
    let mut request = cylinder();
    request.profile = closed_rational_spline();
    request.selections[0].outer.uses.truncate(1);
    let result = normal_extrude_rational_cylinder(&request, 3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    let expected = std::f64::consts::PI * QUARTER_RADIUS * QUARTER_RADIUS;
    let actual = result.authored_area();
    assert!(actual.lower <= expected && expected <= actual.upper);
    assert_eq!(result.authored_profile(), &request.profile);
    assert!(result.validation().is_valid());
    assert_closed_spline_history(&result);
}

fn assert_closed_spline_history(result: &RationalWrappedResult) {
    for numerator in [1, 3] {
        assert!(result.history().iter().any(|record| record.result.source
            == WrappedEntitySource::SplitVertex {
                curve: SurfaceCurveId(1),
                point: WrappedSourcePoint {
                    numerator,
                    level: 2
                },
            }));
    }
    assert!(result.history().iter().any(|record| record.result.source
        == WrappedEntitySource::Vertex(crate::surface_sketch::SurfaceVertexId(1))));
    assert_eq!(
        result.export_native().unwrap().entities.len(),
        result.history().len()
    );
}

fn closed_rational_spline() -> SketchProfile {
    let r = QUARTER_RADIUS;
    let poles = vec![
        [4. + r, 0.],
        [4. + r, r],
        [4., r],
        [4. - r, r],
        [4. - r, 0.],
        [4. - r, -r],
        [4., -r],
        [4. + r, -r],
        [4. + r, 0.],
    ];
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: vec![ProfileVertex {
            id: ProfileVertexId(1),
            point: poles[0],
        }],
        curves: vec![ProfileCurve::Spline {
            id: ProfileCurveId(1),
            start: ProfileVertexId(1),
            end: ProfileVertexId(1),
            degree: 2,
            poles,
            weights: Some(vec![1., 1., 2., 2., 4., 4., 8., 8., 16.]),
            knots: vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.],
        }],
    }
}

mod analytic;
mod holes;
