// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbo::profile_extrusion::{
    LoopUse, PlanarProofWorkLimit, ProfileCurve, ProfileCurveId, ProfileLoop, ProfilePlane,
    ProfileVertex, ProfileVertexId, SketchProfile,
};

static_assertions::assert_impl_all!(CurvedBandResult: Send, Sync);

pub(in crate::surface_sketch::full_turn) fn fixture() -> (FullTurnBand, WrappedNormalRequest) {
    let mut band = cylinder(NormalSense::Outward, 0.);
    let FullTurnSupport::Cylinder { support, axial } = &mut band.support else {
        panic!("cylindrical stock");
    };
    *axial = [0., 6.];
    let points = [[-0.75, 2.], [1., 2.], [1., 3.], [-0.75, 3.]];
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
        poles: vec![points[0], [0.125, 1.5], points[1]],
        weights: None,
        knots: vec![0., 0., 0., 1., 1., 1.],
    }];
    curves.extend((2..=4).map(|i| ProfileCurve::Line {
        id: ProfileCurveId(i),
        start: ProfileVertexId(i),
        end: ProfileVertexId(i % 4 + 1),
    }));
    let request = WrappedNormalRequest {
        support: *support,
        depths: [-1., 2.],
        profile: SketchProfile {
            plane: ProfilePlane::world_xy(),
            vertices,
            curves,
        },
        selections: vec![WrappedRegionSelection {
            region: SurfaceRegionId(31),
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
    };
    (band, request)
}

#[test]
fn quadratic_aperture_certifies_the_actual_decoded_seam_cut() {
    let (band, request) = fixture();
    let result = normal_extrude_cylinder_band_with_curved_hole(&band, &request, options()).unwrap();
    let publication = result.publication();
    let geometry = publication.geometry();
    let [census] = geometry.validation().census.as_slice() else {
        panic!("one aperture body")
    };
    assert_eq!(
        [
            census.vertices,
            census.edges,
            census.faces,
            census.loops,
            census.shells
        ],
        [28, 42, 14, 16, 1]
    );
    assert_eq!(census.genus, Some(2));
    let expected = 198. * PI - 6.7375;
    let volume = geometry.volume();
    let discrepancy = result.tool().measures.retained_ideal_discrepancy.upper;
    assert!((volume.value - expected).abs() <= volume.error_bound + discrepancy);
    assert!(result.trace_bound() > 0. && result.trace_bound() <= 1e-7);
    assert!(result.native_parameter_error() > 0. && result.native_parameter_error() < 1e-5);
    assert_curved_history(publication);
}

#[test]
fn quadratic_aperture_preserves_positive_and_negative_lifted_roots() {
    for (turn, parameters) in [
        (1, [0.428_571_428_571_429_27, 0.571_428_571_428_570_7]),
        (-1, [0.428_571_428_571_427_9, 0.571_428_571_428_572_2]),
    ] {
        let (band, mut request) = fixture();
        let lift = f64::from(turn) * 10. * PI;
        for vertex in &mut request.profile.vertices {
            vertex.point[0] += lift;
        }
        if let ProfileCurve::Spline { poles, .. } = &mut request.profile.curves[0] {
            for pole in poles {
                pole[0] += lift;
            }
        }
        let result =
            normal_extrude_cylinder_band_with_curved_hole(&band, &request, options()).unwrap();
        assert_lifted_aperture(&result, i64::from(turn), parameters);
    }
}

fn assert_lifted_aperture(result: &CurvedBandResult, turn: i64, parameters: [f64; 2]) {
    let cut = result.publication();
    let roots = cut.crossings();
    assert_eq!(roots.len(), 2);
    // The oracle solves the original binary64 controls against mathematical 10π.
    for (index, root) in roots.iter().enumerate() {
        assert_eq!(root.id.turn, turn);
        let [lo, hi] = root.normalized_parameter;
        assert!(
            lo <= parameters[index] && parameters[index] <= hi,
            "{root:?}"
        );
    }
    assert_eq!(roots[0].id.transition, SheetTransition::Increasing);
    assert_eq!(roots[1].id.transition, SheetTransition::Decreasing);
    let geometry = cut.geometry();
    assert_eq!(geometry.validation().census[0].genus, Some(2));
    let discrepancy = result.tool().measures.retained_ideal_discrepancy.upper;
    let volume = geometry.volume();
    assert!((volume.value - (198. * PI - 6.7375)).abs() <= volume.error_bound + discrepancy);
    assert!(result.trace_bound() <= 1e-7);
}

#[test]
fn curved_aperture_refuses_periodic_overlap_and_ambiguous_corner_contact() {
    let (band, mut request) = fixture();
    request.profile.vertices[1].point[0] = 32.;
    request.profile.vertices[2].point[0] = 32.;
    if let ProfileCurve::Spline { poles, .. } = &mut request.profile.curves[0] {
        poles[1][0] = 15.625;
        poles[2][0] = 32.;
    }
    let failure =
        normal_extrude_cylinder_band_with_curved_hole(&band, &request, options()).unwrap_err();
    assert_eq!(failure.stage, SurfaceStage::Admission);
    assert!(matches!(
        failure.cause,
        SurfaceFailureCause::InvalidInput {
            field: "periodic aperture overlap",
            ..
        }
    ));
    let (_, mut request) = fixture();
    request.profile.vertices[0].point[0] = 0.;
    if let ProfileCurve::Spline { poles, .. } = &mut request.profile.curves[0] {
        poles[0][0] = 0.;
    }
    let failure =
        normal_extrude_cylinder_band_with_curved_hole(&band, &request, options()).unwrap_err();
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
fn curved_cut_archive_and_authored_intervals_replay_in_fresh_processes() {
    const CHILD: &str = "MARTENSITE_CURVED_PERIODIC_REPLAY_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let (band, request) = fixture();
        let result =
            normal_extrude_cylinder_band_with_curved_hole(&band, &request, options()).unwrap();
        let cut = result.publication();
        println!(
            "FULL_TURN={:x?}|{:?}|{:?}|{}|{}",
            cut.geometry().bytes,
            cut.geometry().history(),
            cut.crossings(),
            result.trace_bound(),
            result.native_parameter_error()
        );
        return;
    }
    let test = "surface_sketch::full_turn::tests::curved_aperture::curved_cut_archive_and_authored_intervals_replay_in_fresh_processes";
    assert_eq!(fresh_archive(CHILD, test), fresh_archive(CHILD, test));
}

fn assert_curved_history(publication: &CrossedBandResult) {
    let roots = publication.crossings();
    assert_eq!(roots.len(), 2);
    for (root, curve, ratio, transition, indices) in [
        (roots[0], 1, 3. / 7., SheetTransition::Increasing, [1, 2]),
        (roots[1], 3, 4. / 7., SheetTransition::Decreasing, [2, 3]),
    ] {
        assert_eq!(root.id.curve, SurfaceCurveId(curve));
        assert_eq!(root.id.transition, transition);
        let [lo, hi] = root.normalized_parameter;
        assert!(lo <= ratio && ratio <= hi);
        let cut = PeriodicSourceEnd::Cut(root.id);
        let ends = indices.map(|n| {
            let (numerator, level) = if n == 2 { (1, 1) } else { (n, 2) };
            PeriodicSourceEnd::Dyadic(WrappedSourcePoint { numerator, level })
        });
        for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
            for interval in [[ends[0], cut], [cut, ends[1]]] {
                let key = PeriodicImageKey::AperturePiece {
                    curve: root.id.curve,
                    depth,
                    interval,
                };
                assert!(publication.geometry().history().binary_search(&key).is_ok());
            }
        }
    }
}
