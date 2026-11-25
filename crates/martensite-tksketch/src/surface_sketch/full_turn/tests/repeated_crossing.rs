// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbo::profile_extrusion::ProfileCurve;

#[test]
fn two_roots_of_one_curve_keep_authored_ordinals_against_reverse_meridian_order() {
    let (band, request) = two_crossings(-0.6875);
    let result = normal_extrude_cylinder_band_with_curved_hole(&band, &request, options())
        .unwrap_or_else(|cause| panic!("{cause:?}"));
    let publication = result.publication();
    let roots = publication.crossings();
    let [lower, upper] = roots else {
        panic!("{roots:?}");
    };
    let v = 0.375_f64.sqrt();
    for (event, ordinal, transition, t) in [
        (lower, 1, SheetTransition::Increasing, (1. + v) / 2.),
        (upper, 0, SheetTransition::Decreasing, (1. - v) / 2.),
    ] {
        assert_eq!(event.id.curve, SurfaceCurveId(1));
        assert_eq!(event.id.turn, 0);
        assert_eq!(event.id.ordinal, ordinal);
        assert_eq!(event.id.transition, transition);
        let [lo, hi] = event.normalized_parameter;
        assert!(lo <= t && t <= hi);
        assert_piece_history(publication, event.id);
    }
    let volume = publication.geometry().volume();
    let fit = result.tool().measures.retained_ideal_discrepancy.upper;
    assert!((volume.value - (198. * PI - 5.5)).abs() <= volume.error_bound + fit);
    assert_eq!(publication.geometry().validation().census[0].genus, Some(2));
    assert!(result.trace_bound() <= 1e-7);
}

#[test]
fn repeated_source_tangent_contact_refuses_at_admission() {
    let (band, request) = two_crossings(-0.3125);
    let failure =
        normal_extrude_cylinder_band_with_curved_hole(&band, &request, options()).unwrap_err();
    assert_eq!(failure.stage, SurfaceStage::Admission);
    assert!(matches!(
        failure.cause,
        SurfaceFailureCause::Incomplete {
            context: "expected distinct transverse source roots",
            ..
        }
    ));
}

fn two_crossings(middle: f64) -> (FullTurnBand, WrappedNormalRequest) {
    let (band, mut request) = curved_aperture::fixture();
    let profile = &mut request.profile;
    let [top, bottom] = [[0.3125, 4.], [0.3125, 2.]];
    for (vertex, point) in
        profile
            .vertices
            .iter_mut()
            .zip([top, bottom, [0.8125, 2.], [0.8125, 4.]])
    {
        vertex.point = point;
    }
    let ProfileCurve::Spline { poles, .. } = &mut profile.curves[0] else {
        panic!("one original quadratic");
    };
    *poles = vec![top, [middle, 3.], bottom];
    (band, request)
}

fn assert_piece_history(publication: &CrossedBandResult, id: PeriodicCrossingId) {
    let pieces = publication.geometry().history().iter().filter(|key| {
        matches!(key, PeriodicImageKey::AperturePiece { curve, interval, .. }
            if *curve == id.curve && interval.contains(&PeriodicSourceEnd::Cut(id)))
    });
    assert_eq!(pieces.count(), 4);
}
