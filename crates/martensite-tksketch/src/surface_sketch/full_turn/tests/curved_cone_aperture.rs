// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbo::profile_extrusion::{
    LoopUse, PlanarProofWorkLimit, ProfileCurve, ProfileCurveId, ProfileLoop, ProfilePlane,
    ProfileVertex, ProfileVertexId, SketchProfile,
};

static_assertions::assert_impl_all!(CurvedBandResult<WrappedConeMemberReport>: Send, Sync);

pub(super) fn fixture() -> (FullTurnBand, WrappedConeNormalRequest) {
    let mut band = cone(NormalSense::Outward, 0.);
    let FullTurnSupport::Cone { support, slant } = &mut band.support else {
        panic!("conical stock");
    };
    *slant = [3., 8.];
    let points = [[4., 0.25], [4., -0.1875], [6., -0.1875], [6., 0.25]];
    let vertices = points
        .into_iter()
        .enumerate()
        .map(|(i, point)| ProfileVertex {
            id: ProfileVertexId(i as u64 + 1),
            point,
        })
        .collect();
    let mut curves = vec![ProfileCurve::Spline {
        id: ProfileCurveId(1),
        start: ProfileVertexId(1),
        end: ProfileVertexId(2),
        degree: 2,
        poles: vec![points[0], [3.75, 0.03125], points[1]],
        weights: None,
        knots: vec![0., 0., 0., 1., 1., 1.],
    }];
    curves.extend((2..=4).map(|i| ProfileCurve::Line {
        id: ProfileCurveId(i),
        start: ProfileVertexId(i),
        end: ProfileVertexId(i % 4 + 1),
    }));
    let request = WrappedConeNormalRequest {
        support: *support,
        depths: [-0.25, 0.25],
        profile: SketchProfile {
            plane: ProfilePlane::world_xy(),
            vertices,
            curves,
        },
        selections: vec![WrappedRegionSelection {
            region: SurfaceRegionId(23),
            outer: ProfileLoop {
                uses: (1..=4)
                    .map(|i| LoopUse {
                        curve: ProfileCurveId(i),
                        reversed: false,
                    })
                    .collect(),
            },
            holes: vec![],
        }],
        options: WrappedNormalOptions {
            spatial_error: 1e-5,
            planar_work: PlanarProofWorkLimit::CurrentDefault,
            maximum_fit_level: 8,
            volume: WrappedVolumePolicy::Refuse(1e-4),
        },
        proof: WrappedConeProofOptions::default(),
    };
    (band, request)
}

#[test]
fn curved_cone_aperture_binds_the_decoded_split_to_the_original_ray() {
    let (band, request) = fixture();
    let result = normal_extrude_cone_band_with_curved_hole(&band, &request, options()).unwrap();
    let publication = result.publication();
    let geometry = publication.geometry();
    let [census] = geometry.validation().census.as_slice() else {
        panic!("one perforated cone")
    };
    assert_eq!(
        [
            census.vertices,
            census.edges,
            census.faces,
            census.loops,
            census.shells
        ],
        [60, 92, 30, 30, 1]
    );
    assert_eq!(census.genus, Some(2));
    // The depth interval is symmetric, so the integral of the linear cone normal
    // Jacobian in depth gives exactly the half-unit thickness times developed area.
    let expected = 55. * PI / 4. - 175. / 384.;
    let volume = geometry.volume();
    assert!(
        (volume.value - expected).abs()
            <= volume.error_bound + result.tool().volume.discrepancy.upper
    );
    assert!(result.trace_bound() > 0. && result.trace_bound() <= 1e-7);
    assert!(result.native_parameter_error() > 0. && result.native_parameter_error() < 0.005);
    assert_conical_intervals(publication);
}

#[test]
fn curved_cone_aperture_refuses_uncertain_rotational_contact() {
    let (band, mut request) = fixture();
    request.profile.vertices[0].point[1] = 0.;
    if let ProfileCurve::Spline { poles, .. } = &mut request.profile.curves[0] {
        poles[0][1] = 0.;
    }
    let failure =
        normal_extrude_cone_band_with_curved_hole(&band, &request, options()).unwrap_err();
    assert_eq!(failure.stage, SurfaceStage::Admission);
    assert!(matches!(
        failure.cause,
        SurfaceFailureCause::Incomplete {
            context: "transverse crossing clear of authored corners",
            ..
        }
    ));
}

#[test]
fn curved_cone_aperture_archive_replays_across_processes() {
    const CHILD: &str = "MARTENSITE_CURVED_CONE_PERIODIC_REPLAY_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let (band, request) = fixture();
        let result = normal_extrude_cone_band_with_curved_hole(&band, &request, options()).unwrap();
        let publication = result.publication();
        println!(
            "FULL_TURN={:x?}|{:?}|{:?}|{}|{}",
            publication.geometry().bytes,
            publication.geometry().history(),
            publication.crossings(),
            result.trace_bound(),
            result.native_parameter_error()
        );
        return;
    }
    let test = "surface_sketch::full_turn::tests::curved_cone_aperture::curved_cone_aperture_archive_replays_across_processes";
    assert_eq!(fresh_archive(CHILD, test), fresh_archive(CHILD, test));
}

fn assert_conical_intervals(publication: &CrossedBandResult) {
    let roots = publication.crossings();
    assert_eq!(roots.len(), 2);
    let half = PeriodicSourceEnd::Dyadic(WrappedSourcePoint {
        numerator: 1,
        level: 1,
    });
    let five_eighths = PeriodicSourceEnd::Dyadic(WrappedSourcePoint {
        numerator: 5,
        level: 3,
    });
    for (root, curve, ratio, transition, ends) in [
        (
            roots[0],
            1,
            4. / 7.,
            SheetTransition::Decreasing,
            [half, five_eighths],
        ),
        (
            roots[1],
            3,
            3. / 7.,
            SheetTransition::Increasing,
            [PeriodicSourceEnd::Start, half],
        ),
    ] {
        assert_eq!(root.id.curve, SurfaceCurveId(curve));
        assert_eq!(root.id.transition, transition);
        let [a, b] = root.normalized_parameter;
        assert!(a <= ratio && ratio <= b);
        let middle = PeriodicSourceEnd::Cut(root.id);
        for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
            for interval in [[ends[0], middle], [middle, ends[1]]] {
                let key = PeriodicImageKey::AperturePiece {
                    curve: root.id.curve,
                    depth,
                    interval,
                };
                assert!(
                    publication.geometry().history().binary_search(&key).is_ok(),
                    "{key:?}"
                );
            }
        }
    }
}
