//! Decoded trim changes must not reuse a boundary-only certificate.
use super::super::tests::frozen;
use super::*;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkmath::gp::Point2;

#[test]
fn decoded_trim_bindings_cover_both_caps_and_all_four_side_edges() {
    let (original, request, bindings) = frozen();
    let proofs = super::super::certify(&original, &request, &bindings).unwrap();
    for proof in proofs {
        assert_eq!((proof.trims.cap_pcurves, proof.trims.side_pcurves), (2, 4));
        assert!(proof.trims.maximum_tolerance <= request.tolerance / 2.);
    }
}

fn cap_pair(
    request: &Request,
    bindings: &[(BindingKey, Shape)],
) -> (
    martensite_tkbrep::topods::Edge,
    martensite_tkbrep::topods::Face,
) {
    let indexed = index(bindings).unwrap();
    let edge_key = key(request.loops[0][0].span, HistoryRole::FirstDepthEdge);
    let face_key = BindingKey {
        source: Source::Region(request.region),
        role: HistoryRole::FirstDepthFace,
    };
    (
        topods::edge(lookup(edge_key, &indexed).unwrap()).unwrap(),
        topods::face(lookup(face_key, &indexed).unwrap()).unwrap(),
    )
}

#[test]
fn changed_retained_cap_pcurve_refuses_after_decoding() {
    let (original, request, bindings) = frozen();
    let (edge, face) = cap_pair(&request, &bindings);
    let curve = cap_curve(&edge, &face);
    let curve = curve.get().unwrap();
    let pole = curve.pole(1).unwrap();
    curve
        .set_pole(1, Point2::new(pole.x(), pole.y() + 0.01))
        .unwrap();
    let error = super::super::certify(&original, &request, &bindings)
        .err()
        .unwrap();
    assert!(matches!(
        trim_cause(&error),
        StorageCause::Mismatch {
            field: "retained pcurve coefficients/weights",
            ..
        }
    ));
}

#[test]
fn shortened_retained_pcurve_range_refuses_after_decoding() {
    let (original, request, bindings) = frozen();
    let (edge, face) = cap_pair(&request, &bindings);
    BRepBuilder::default()
        .range_on_face(&edge, &face, 0., 0.5)
        .unwrap();
    let error = super::super::certify(&original, &request, &bindings)
        .err()
        .unwrap();
    assert!(matches!(
        trim_cause(&error),
        StorageCause::Mismatch {
            field: "retained pcurve range/storage",
            ..
        }
    ));
}

#[test]
fn widened_retained_edge_tolerance_refuses_instead_of_relaxing_the_budget() {
    let (original, request, bindings) = frozen();
    let (edge, _) = cap_pair(&request, &bindings);
    BRepBuilder::default()
        .update_edge_tolerance(&edge, request.tolerance * 2.)
        .unwrap();
    let error = super::super::certify(&original, &request, &bindings)
        .err()
        .unwrap();
    assert!(matches!(trim_cause(&error), StorageCause::Tolerance { .. }));
}

fn cap_curve(
    edge: &martensite_tkbrep::topods::Edge,
    face: &martensite_tkbrep::topods::Face,
) -> Handle<Geom2dBezierCurve> {
    BRepTool::curve_on_face(edge, face).unwrap().pcurve.cast()
}

#[test]
fn altered_retained_rational_weight_cannot_reparameterize_the_cap_trim() {
    let (original, request, bindings) = frozen();
    let (edge, face) = cap_pair(&request, &bindings);
    cap_curve(&edge, &face)
        .get()
        .unwrap()
        .set_weight(2, 2.)
        .unwrap();
    let error = super::super::certify(&original, &request, &bindings)
        .err()
        .unwrap();
    assert!(matches!(
        trim_cause(&error),
        StorageCause::Mismatch {
            field: "retained pcurve coefficients/weights",
            ..
        }
    ));
}

#[test]
fn altered_ruled_side_trim_refuses_with_unchanged_surface_and_space_curves() {
    let (original, request, bindings) = frozen();
    let (edge, face) = side_pair(&request, &bindings);
    cap_curve(&edge, &face)
        .get()
        .unwrap()
        .set_pole(2, Point2::new(1., 0.1))
        .unwrap();
    let error = super::super::certify(&original, &request, &bindings)
        .err()
        .unwrap();
    assert!(matches!(
        trim_cause(&error),
        StorageCause::Mismatch {
            field: "retained pcurve coefficients/weights",
            ..
        }
    ));
}

fn trim_cause(error: &StorageFailure) -> &StorageCause {
    match &error.cause {
        StorageCause::Trim { cause, .. } => &cause.cause,
        _ => panic!("expected edge/face-scoped trim failure: {error}"),
    }
}

fn side_pair(
    request: &Request,
    bindings: &[(BindingKey, Shape)],
) -> (
    martensite_tkbrep::topods::Edge,
    martensite_tkbrep::topods::Face,
) {
    let indexed = index(bindings).unwrap();
    let span = request.loops[0][0].span;
    let face = topods::face(lookup(key(span, HistoryRole::SideFace), &indexed).unwrap()).unwrap();
    let edge =
        topods::edge(lookup(key(span, HistoryRole::FirstDepthEdge), &indexed).unwrap()).unwrap();
    (edge, face)
}

#[test]
fn invalid_raw_face_tolerance_is_not_hidden_by_the_tool_floor() {
    use martensite_tkbrep::brep::access::face_tshape;
    for actual in [f64::NAN, -1.] {
        let (original, request, bindings) = frozen();
        let (_, face) = cap_pair(&request, &bindings);
        face_tshape(&face, "trim test")
            .unwrap()
            .set_tolerance(actual);
        assert!(BRepTool::face_tolerance(&face).unwrap().is_finite());
        let error = super::super::certify(&original, &request, &bindings)
            .err()
            .unwrap();
        assert!(matches!(trim_cause(&error), StorageCause::Tolerance { .. }));
        assert!(error.to_string().contains("face="));
        assert!(error.to_string().contains("retained tolerance="));
    }
}

#[test]
fn every_cap_has_a_full_interval_bound_within_both_retained_tolerances() {
    let (original, request, bindings) = frozen();
    let proofs = super::super::certify(&original, &request, &bindings).unwrap();
    for stored in proofs {
        for cap in stored.trims.cap_residuals {
            assert!(cap.spatial_residual_bound() <= cap.requested_spatial_limit());
            assert!(cap.requested_spatial_limit() <= request.tolerance / 2.);
        }
    }
}

#[test]
fn a_small_retained_tolerance_must_cover_the_actual_cap_residual() {
    let mut original = super::super::super::tests::request();
    original.tolerance = 1e-2;
    let (original, request, bindings) = super::super::tests::frozen_with(original);
    super::super::certify(&original, &request, &bindings).unwrap();
    let changed = lower_retained_cap(&request, &bindings);
    let error = super::super::certify(&original, &request, &bindings)
        .err()
        .unwrap();
    assert_eq!(error.key, changed);
    assert_numeric_cap_refusal(&error);
}

fn lower_retained_cap(request: &Request, bindings: &[(BindingKey, Shape)]) -> BindingKey {
    let indexed = index(bindings).unwrap();
    let piece = request.loops[0]
        .iter()
        .find(|p| p.span.curve.0 == 12)
        .unwrap();
    let key = key(piece.span, HistoryRole::FirstDepthEdge);
    let edge = topods::edge(lookup(key, &indexed).unwrap()).unwrap();
    martensite_tkbrep::brep::access::edge_tshape(&edge, "trim test")
        .unwrap()
        .set_tolerance(1e-7);
    key
}

fn assert_numeric_cap_refusal(error: &StorageFailure) {
    let StorageCause::CapResidual(cause) = trim_cause(error) else {
        panic!("expected cap residual: {error}");
    };
    assert_eq!(cause.requested_spatial_limit, 1e-7);
    assert!(error.to_string().contains("cylindrical cap trim"));
    assert!(matches!(cause.cause, martensite_tkmath::predicates::roots::metric_bounds::CylindricalProfileFailure::Uncertified { .. }));
}

#[test]
fn recaptured_flux_uses_the_retained_cap_and_side_sources() {
    let (original, request, bindings) = frozen();
    let proofs = super::super::certify(&original, &request, &bindings).unwrap();
    let flux = super::super::certify_flux(&original, &request, &bindings, &proofs).unwrap();
    assert_eq!(flux.span_count(), 19);
    assert!(flux.absolute_error_bound() <= flux.requested_options().absolute_error_limit);
    assert!((flux.representative() - 11.10375).abs() < original.tolerance);
}

#[test]
fn flux_recapture_refuses_stale_side_trim_domains() {
    let (original, request, bindings) = frozen();
    let proofs = super::super::certify(&original, &request, &bindings).unwrap();
    let (edge, face) = side_pair(&request, &bindings);
    BRepBuilder::default()
        .range_on_face(&edge, &face, 0., 0.5)
        .unwrap();
    let error = super::super::certify_flux(&original, &request, &bindings, &proofs)
        .err()
        .unwrap();
    assert!(matches!(
        trim_cause(&error),
        StorageCause::Mismatch {
            field: "retained pcurve range/storage",
            ..
        }
    ));
}

#[test]
fn flux_recapture_requires_complete_proof_membership() {
    let (original, request, bindings) = frozen();
    let mut proofs = super::super::certify(&original, &request, &bindings).unwrap();
    proofs.pop();
    let error = super::super::certify_flux(&original, &request, &bindings, &proofs)
        .err()
        .unwrap();
    assert!(matches!(
        error.cause,
        StorageCause::Mismatch {
            field: "flux proof count",
            ..
        }
    ));
}
