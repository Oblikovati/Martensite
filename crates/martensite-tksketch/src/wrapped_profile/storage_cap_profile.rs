//! Use retained angular controls directly; multiplying them by R would introduce new rounding.
use super::*;

pub(super) fn certify(
    request: &Request,
    piece: &AuthoredPiece,
    curve: &OwnedCurve,
    cancelled: &mut CancelPoll<'_>,
) -> Result<CertifiedCylindricalProfile, StorageFailure> {
    // Trim admission already checked these exact bits against both decoded cap pcurves.
    let poles: Vec<_> = cap_coordinates::angular_controls(&piece.poles, request.radius);
    let query = query(request, piece.span, &poles, curve)?;
    certify_cylindrical_profile(query, cancelled).map_err(|cause| failure(piece.span, cause))
}
fn query<'a>(
    request: &Request,
    span: CurveSpan,
    poles: &'a [[f64; 2]],
    curve: &'a OwnedCurve,
) -> Result<CylindricalProfileQuery<'a>, StorageFailure> {
    let limit = ((request.tolerance / 2.).next_down() / request.radius.max(1.)).next_down();
    Ok(CylindricalProfileQuery {
        authored: poles,
        stored: curve.borrowed(),
        interval: whole_interval(span, limit)?,
        support_radius: 1.,
        stored_radius: request.radius + request.depths[0],
        angular_origin: 0.,
        spatial_limit: limit,
        options: Default::default(),
    })
}
fn failure(span: CurveSpan, cause: CylindricalProfileError) -> StorageFailure {
    StorageFailure {
        key: key(span, HistoryRole::FirstDepthEdge),
        cause: StorageCause::Profile(Box::new(cause)),
    }
}

fn whole_interval(
    span: CurveSpan,
    limit: f64,
) -> Result<CylindricalProfileInterval, StorageFailure> {
    CylindricalProfileInterval::new(0, 0).map_err(|cause| {
        failure(
            span,
            CylindricalProfileError {
                requested_options: Default::default(),
                requested_spatial_limit: limit,
                cause,
            },
        )
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cap_certificate_binds_angular_controls_without_developed_reconstruction() {
        let (original, request, bindings) = super::super::tests::frozen();
        let proofs = super::super::certify(&original, &request, &bindings).unwrap();
        let indexed = index(&bindings).unwrap();
        for (piece, proof) in request.loops.iter().flatten().zip(proofs) {
            let curve =
                capture::curve(key(piece.span, HistoryRole::FirstDepthEdge), &indexed).unwrap();
            let mut poles: Vec<_> = cap_coordinates::angular_controls(&piece.poles, request.radius);
            let source = query(&request, piece.span, &poles, &curve).unwrap();
            assert_cap_query(&proof.cap_profile, source);
            poles[0][0] = poles[0][0].next_up();
            assert!(
                !proof
                    .cap_profile
                    .matches_source(query(&request, piece.span, &poles, &curve).unwrap())
            );
        }
    }
    fn assert_cap_query(proof: &CertifiedCylindricalProfile, source: CylindricalProfileQuery<'_>) {
        assert!(proof.matches_source(source));
        assert_eq!(source.support_radius, 1.);
        assert_eq!(
            source.interval,
            CylindricalProfileInterval::new(0, 0).unwrap()
        );
    }
    #[test]
    fn cap_certificate_obeys_operation_cancellation() {
        let (_, request, bindings) = super::super::tests::frozen();
        let piece = &request.loops[0][0];
        let indexed = index(&bindings).unwrap();
        let curve = capture::curve(key(piece.span, HistoryRole::FirstDepthEdge), &indexed).unwrap();
        let error = certify(&request, piece, &curve, &mut || true)
            .err()
            .unwrap();
        let StorageCause::Profile(profile) = error.cause else {
            panic!("expected a typed profile cancellation");
        };
        let martensite_tkmath::predicates::roots::metric_bounds::CylindricalProfileFailure::Arithmetic(cause) = profile.cause else {
            panic!("expected bounded arithmetic cancellation");
        };
        assert!(matches!(*cause, martensite_tkmath::predicates::roots::metric_bounds::BoundedArithmeticFailure::Cancelled));
    }
}
