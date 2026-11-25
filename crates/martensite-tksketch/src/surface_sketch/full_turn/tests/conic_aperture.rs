// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbo::profile_extrusion::{
    ArcSense, ProfileCurve, ProfileCurveId, ProfileVertexId, SketchProfile,
};

#[test]
fn analytic_circular_aperture_on_cylinder_uses_original_angular_map() {
    let (band, mut request) = curved_aperture::fixture();
    cylinder_profile(&mut request.profile);
    let result = normal_extrude_cylinder_band_with_rational_hole(&band, &request, options(), 4)
        .unwrap_or_else(|cause| panic!("{cause:?}"));
    let area = (7. / 8. + PI / 4.) / 4.;
    check_conic(&result, -1, area, 198. * PI - 3.3 * area);
}

#[test]
fn analytic_circular_aperture_on_cone_uses_original_angular_map() {
    let (mut band, mut request) = curved_cone_aperture::fixture();
    let FullTurnSupport::Cone { slant, .. } = &mut band.support else {
        panic!("conical stock");
    };
    *slant = [7., 12.];
    cone_profile(&mut request.profile);
    let result = normal_extrude_cone_band_with_rational_hole(&band, &request, options(), 3)
        .unwrap_or_else(|cause| panic!("{cause:?}"));
    let area = (21. / 16. + PI / 4.) / 256.;
    check_conic(&result, 2, area, 95. * PI / 4. - 0.5 * area);
}

#[test]
fn exact_nonquarter_arc_on_cylinder_binds_its_native_angle_endpoints() {
    let (band, mut request) = curved_aperture::fixture();
    short_arc(&mut request.profile, |[x, y]| [x / 4., 2. + (y - 2.) / 4.]);
    let result = normal_extrude_cylinder_band_with_rational_hole(&band, &request, options(), 4)
        .unwrap_or_else(|cause| panic!("{cause:?}"));
    let area = native_short_area() / 16.;
    check_short_arc(&result, area, 198. * PI - 3.3 * area);
}

#[test]
fn exact_nonquarter_arc_on_cone_binds_its_native_angle_endpoints() {
    let (mut band, mut request) = curved_cone_aperture::fixture();
    let FullTurnSupport::Cone { slant, .. } = &mut band.support else {
        panic!("conical stock");
    };
    *slant = [7., 12.];
    short_arc(&mut request.profile, |[x, y]| [7.5 + y / 32., -x / 32.]);
    let result = normal_extrude_cone_band_with_rational_hole(&band, &request, options(), 3)
        .unwrap_or_else(|cause| panic!("{cause:?}"));
    let area = native_short_area() / 1024.;
    check_short_arc(&result, area, 95. * PI / 4. - 0.5 * area);
}

#[test]
fn exact_nonquarter_arc_body_and_native_map_replay_in_fresh_processes() {
    const CHILD: &str = "MARTENSITE_SHORT_CONIC_APERTURE_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let (band, mut request) = curved_aperture::fixture();
        short_arc(&mut request.profile, |[x, y]| [x / 4., 2. + (y - 2.) / 4.]);
        let answer = normal_extrude_cylinder_band_with_rational_hole(&band, &request, options(), 4)
            .unwrap_or_else(|cause| panic!("{cause:?}"));
        let cut = answer.publication();
        println!(
            "FULL_TURN={:x?}|{:?}|{:?}",
            cut.geometry().bytes,
            cut.geometry().history(),
            cut.crossings()
        );
        return;
    }
    let test = "surface_sketch::full_turn::tests::conic_aperture::exact_nonquarter_arc_body_and_native_map_replay_in_fresh_processes";
    assert_eq!(fresh_archive(CHILD, test), fresh_archive(CHILD, test));
}

#[test]
fn native_arc_with_nonbinary_stereographic_weights_refuses() {
    let (band, mut request) = curved_aperture::fixture();
    short_arc(&mut request.profile, |[x, y]| [x / 4., 2. + (y - 2.) / 4.]);
    // This exact 3-4-5 endpoint instead has half-angle tangent 1/3 and last weight 10/9.
    request.profile.vertices[1].point = [0.25, 2.25];
    let failure = normal_extrude_cylinder_band_with_rational_hole(&band, &request, options(), 4)
        .err()
        .unwrap();
    assert_eq!(failure.stage, SurfaceStage::Admission);
    assert!(matches!(
        failure.cause,
        SurfaceFailureCause::Incomplete {
            context: "exactly representable analytic conic cut controls",
            ..
        }
    ));
}

pub(in crate::surface_sketch::full_turn) fn short_arc(
    profile: &mut SketchProfile,
    place: impl Fn([f64; 2]) -> [f64; 2],
) {
    let boundary = [[-2., 2.], [2., 4.], [1.5, 5.], [-2., 5.]];
    arc_profile(profile, boundary.map(&place), place([-2., 7.]));
}

pub(in crate::surface_sketch::full_turn) fn native_short_area() -> f64 {
    25. * martensite_cmath::atan(0.5) - 9. / 4.
}

fn check_short_arc(
    result: &CurvedBandResult<RationalWrappedResult>,
    area: f64,
    expected_volume: f64,
) {
    let evidence = result.tool().authored_area();
    assert!(
        evidence.lower <= area && area <= evidence.upper,
        "{evidence:?}"
    );
    let publication = result.publication();
    let root = publication
        .crossings()
        .iter()
        .find(|root| root.id.curve == SurfaceCurveId(1))
        .unwrap();
    // For the original 3-4-5 endpoints the native angle is -π/2 + 2 atan(t/2).
    // Its physical zero-arc occurrence solves the unrounded equation t²−10t+4 = 0.
    let parameter = 5. - 21_f64.sqrt();
    let [lo, hi] = root.normalized_parameter;
    assert!(lo <= parameter && parameter <= hi, "{root:?}");
    let map = PeriodicConicMap::Arc(WrappedAnalyticArcMap {
        first: WrappedAnalyticArcEndpoint::Start,
        last: WrappedAnalyticArcEndpoint::End,
        increasing: true,
    });
    assert_eq!(root.conic_map, Some(map));
    check_conic_parts(publication, root.id, map);
    let geometry = publication.geometry();
    assert_eq!(geometry.validation().census[0].genus, Some(2));
    let volume = geometry.volume();
    assert!(
        (volume.value - expected_volume).abs()
            <= volume.error_bound + result.tool().volume_discrepancy().upper
    );
    assert!(result.trace_bound() <= 1e-7);
}

#[test]
fn exact_conic_quarter_refuses_an_unproved_long_original_arc() {
    let (band, mut request) = curved_aperture::fixture();
    cylinder_profile(&mut request.profile);
    let ProfileCurve::Arc { sense, .. } = &mut request.profile.curves[0] else {
        panic!("authored circle");
    };
    *sense = ArcSense::Clockwise;
    let failure = normal_extrude_cylinder_band_with_rational_hole(&band, &request, options(), 4)
        .err()
        .unwrap();
    assert_eq!(failure.stage, SurfaceStage::Admission);
    assert!(matches!(
        failure.cause,
        SurfaceFailureCause::Incomplete {
            context: "exact short analytic conic source",
            ..
        }
    ));
}

fn cylinder_profile(profile: &mut SketchProfile) {
    let boundary = [[-0.5, 2.], [0.5, 3.], [0.25, 4.], [-0.5, 4.]];
    let place = |[x, y]: [f64; 2]| [x / 2., 1. + y / 2.];
    arc_profile(profile, boundary.map(place), place([-0.5, 3.]));
}

fn cone_profile(profile: &mut SketchProfile) {
    let boundary = [[3.5, 0.5], [4.5, -0.5], [6., -0.25], [6., 0.5]];
    let place = |[x, y]: [f64; 2]| [7. + x / 16., y / 16.];
    arc_profile(profile, boundary.map(place), place([4.5, 0.5]));
}

pub(in crate::surface_sketch::full_turn) fn arc_profile(
    profile: &mut SketchProfile,
    points: [[f64; 2]; 4],
    center: [f64; 2],
) {
    for (vertex, point) in profile.vertices.iter_mut().zip(points) {
        vertex.point = point;
    }
    profile.curves[0] = ProfileCurve::Arc {
        id: ProfileCurveId(1),
        start: ProfileVertexId(1),
        end: ProfileVertexId(2),
        center,
        sense: ArcSense::CounterClockwise,
    };
}

fn check_conic(
    result: &CurvedBandResult<RationalWrappedResult>,
    quarter: i8,
    area: f64,
    volume: f64,
) {
    let area_bound = result.tool().authored_area();
    assert!(area_bound.lower <= area && area <= area_bound.upper);
    let publication = result.publication();
    let [root] = publication
        .crossings()
        .iter()
        .filter(|root| root.id.curve == SurfaceCurveId(1))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|roots: Vec<_>| panic!("{roots:?}"));
    let parameter = 2. - 3_f64.sqrt();
    let [lo, hi] = root.normalized_parameter;
    assert!(lo <= parameter && parameter <= hi, "{root:?}");
    let Some(PeriodicConicMap::Quarter(map)) = root.conic_map else {
        panic!("original analytic quarter angle");
    };
    assert_eq!(map.first_quarter.rem_euclid(4), quarter.rem_euclid(4));
    assert!(map.increasing);
    check_conic_parts(publication, root.id, PeriodicConicMap::Quarter(map));
    let integration = publication.geometry().volume();
    assert!(
        (integration.value - volume).abs()
            <= integration.error_bound + result.tool().volume_discrepancy().upper
    );
    assert!(result.trace_bound() <= 1e-7);
}

fn check_conic_parts(
    publication: &CrossedBandResult,
    root: PeriodicCrossingId,
    map: PeriodicConicMap,
) {
    let found = publication.geometry().history().iter().filter(|key| match key {
        PeriodicImageKey::AperturePiece { curve, interval, .. } if *curve == root.curve => {
            interval.contains(&PeriodicSourceEnd::Cut(root))
                && interval.iter().any(|end| matches!(end, PeriodicSourceEnd::ConicDyadic { map: actual, .. } if *actual == map))
        }
        _ => false,
    });
    assert_eq!(found.count(), 4);
}
