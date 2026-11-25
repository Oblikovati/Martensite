// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbo::profile_extrusion::ProfileCurve;

static_assertions::assert_impl_all!(CurvedBandResult<RationalWrappedResult>: Send, Sync);

#[test]
fn rational_cylinder_cut_retains_the_original_weighted_root_and_signed_volume() {
    let (band, mut request) = curved_aperture::fixture();
    set_weights(&mut request.profile.curves[0], [1., 1., 2.]);
    let result = normal_extrude_cylinder_band_with_rational_hole(&band, &request, options(), 4)
        .unwrap_or_else(|cause| panic!("{cause:?}"));
    let area = 21. / 16. + 7. * PI / 32.;
    check_weighted(&result, area, (97_f64.sqrt() - 7.) / 8.);
    check_volume(&result, 198. * PI - 3.3 * area);
    assert_eq!(result.tool().authored_profile(), &request.profile);
}

#[test]
fn rational_cone_cut_retains_the_original_weighted_root_and_signed_volume() {
    let (mut band, mut request) = curved_cone_aperture::fixture();
    let FullTurnSupport::Cone { slant, .. } = &mut band.support else {
        panic!("conical stock");
    };
    // Use a compact chart and preserve the original conical work grant.
    *slant = [7., 12.];
    let compact = |[x, y]: [f64; 2]| [7. + x / 4., y / 4.];
    for vertex in &mut request.profile.vertices {
        vertex.point = compact(vertex.point);
    }
    if let ProfileCurve::Spline { poles, .. } = &mut request.profile.curves[0] {
        for pole in poles {
            *pole = compact(*pole);
        }
    }
    set_weights(&mut request.profile.curves[0], [1., 1., 2.]);
    let result = normal_extrude_cone_band_with_rational_hole(&band, &request, options(), 3)
        .unwrap_or_else(|cause| panic!("{cause:?}"));
    let area = (105. / 128. + 7. * PI / 256.) / 16.;
    check_weighted(&result, area, (97_f64.sqrt() - 7.) / 6.);
    check_volume(&result, 95. * PI / 4. - 0.5 * area);
}

#[test]
fn rational_periodic_aperture_refuses_nonpositive_authored_weights() {
    let (band, mut request) = curved_aperture::fixture();
    set_weights(&mut request.profile.curves[0], [1., 0., 2.]);
    let failure = normal_extrude_cylinder_band_with_rational_hole(&band, &request, options(), 4)
        .err()
        .unwrap();
    assert_eq!(failure.stage, SurfaceStage::Admission);
    assert!(matches!(
        failure.cause,
        SurfaceFailureCause::Incomplete {
            context: "three positive rational quadratic weights",
            ..
        }
    ));
}

fn set_weights(curve: &mut ProfileCurve, values: [f64; 3]) {
    let ProfileCurve::Spline { weights, .. } = curve else {
        panic!("rational source");
    };
    *weights = Some(values.into());
}

fn check_weighted(result: &CurvedBandResult<RationalWrappedResult>, area: f64, parameter: f64) {
    let bound = result.tool().authored_area();
    assert!(bound.lower <= area && area <= bound.upper, "{bound:?}");
    let roots = result.publication().crossings();
    assert_eq!(roots.len(), 2);
    let root = roots
        .iter()
        .find(|root| root.id.curve == SurfaceCurveId(1))
        .unwrap();
    let [lo, hi] = root.normalized_parameter;
    assert!(lo <= parameter && parameter <= hi, "{root:?}");
    assert!(result.trace_bound() > 0. && result.trace_bound() <= 1e-7);
    let bound = result
        .publication()
        .geometry()
        .history()
        .iter()
        .filter(|key| {
            matches!(key,
        PeriodicImageKey::AperturePiece { curve, interval, .. }
            if *curve == root.id.curve && interval.contains(&PeriodicSourceEnd::Cut(root.id)))
        })
        .count();
    assert_eq!(bound, 4);
}

fn check_volume(result: &CurvedBandResult<RationalWrappedResult>, expected: f64) {
    let geometry = result.publication().geometry();
    assert_eq!(geometry.validation().census[0].genus, Some(2));
    let discrepancy = result.tool().volume_discrepancy().upper;
    let volume = geometry.volume();
    assert!((volume.value - expected).abs() <= volume.error_bound + discrepancy);
}
