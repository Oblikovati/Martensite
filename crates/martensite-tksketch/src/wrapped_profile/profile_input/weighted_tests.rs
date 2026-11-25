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
