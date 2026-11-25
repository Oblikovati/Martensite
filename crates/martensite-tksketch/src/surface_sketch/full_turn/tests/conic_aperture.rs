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
            context: "exact analytic conic quarter",
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

fn arc_profile(profile: &mut SketchProfile, points: [[f64; 2]; 4], center: [f64; 2]) {
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
    let map = root.conic_map.expect("original analytic quarter angle");
    assert_eq!(map.first_quarter.rem_euclid(4), quarter.rem_euclid(4));
    assert!(map.increasing);
    check_conic_parts(publication, root.id, map);
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
    map: WrappedConicParameterMap,
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
