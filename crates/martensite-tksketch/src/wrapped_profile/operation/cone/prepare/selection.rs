//! Preserve actual selection and sector receipts without rebuilding their owners.
use super::*;
use crate::surface_sketch::wrapped_profile::selection::PlanarSelectionEvidence;
pub(super) fn planar(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<PlanarSelectionEvidence> {
    let captured = planar::capture_with_work_and_cancel(
        originals,
        request.options.spatial_error,
        work.planar_remaining(request.options.planar_work),
        &mut *cancelled,
    )
    .map_err(|cause| failure(None, None, ConeCause::Planar(Box::new(cause))))?;
    work.charge_planar(first_region(originals)?, captured.proof_work())?;
    let members = planar::select_members(originals, &captured).map_err(|(region, cause)| {
        failure(Some(region), None, ConeCause::Planar(Box::new(cause)))
    })?;
    Ok(PlanarSelectionEvidence {
        members,
        certificate: captured.certificate(),
        proof_work: captured.proof_work(),
    })
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn sector(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalSectorEvidence> {
    let spans: Vec<_> = originals
        .iter()
        .flat_map(|r| r.loops.iter().flatten())
        .map(|p| p.poles.as_slice())
        .collect();
    let query = ConicalSectorQuery {
        spans: &spans,
        semi_angle: request.support.semi_angle,
        options: work.remaining(),
    };
    let proof = certify_conical_sector(query, cancelled)
        .map_err(|cause| sector_failure(originals, cause))?;
    work.charge(first_region(originals)?, None, proof.operations())?;
    Ok(proof)
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn first_region(
    originals: &[DevelopedRegionRequest],
) -> ConeChecked<SurfaceRegionId> {
    originals
        .first()
        .map(|source| source.region)
        .ok_or_else(|| {
            failure(
                None,
                None,
                ConeCause::Scalar {
                    field: "selected-regions",
                    actual: 0.,
                    expected: "at least one region",
                },
            )
        })
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn wedge(
    sector: &ConicalSectorEvidence,
) -> ConeChecked<ConicalSectorWedge> {
    sector.wedge().ok_or_else(|| {
        failure(
            None,
            None,
            ConeCause::Scalar {
                field: "sector-wedge",
                actual: 0.,
                expected: "one certified strict wedge",
            },
        )
    })
}
pub(super) fn completed(
    executed_selection: WrappedConeExecutionSelection,
    planar: PlanarSelectionEvidence,
    prepared: Vec<Prepared>,
    domain: domain::DomainEvidence,
    clearance: CertifiedCylindricalClearance,
    arithmetic_operations: u32,
) -> Selection {
    Selection {
        executed_selection,
        prepared,
        planar: planar.members,
        census: planar.certificate,
        work: planar.proof_work,
        domain,
        clearance,
        arithmetic_operations,
    }
}

fn sector_failure(
    originals: &[DevelopedRegionRequest],
    cause: ConicalSectorFailure,
) -> ConeFailure {
    let index = sector_index(&cause);
    let owner = index.and_then(|index| {
        originals
            .iter()
            .flat_map(|region| {
                region
                    .loops
                    .iter()
                    .flatten()
                    .map(move |piece| (region.region, piece.span))
            })
            .nth(index)
    });
    failure(
        owner.map(|(region, _)| region),
        owner.map(|(_, span)| span),
        ConeCause::Sector(Box::new(cause)),
    )
}

pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn sector_index(
    cause: &ConicalSectorFailure,
) -> Option<usize> {
    match cause {
        ConicalSectorFailure::PoleCount { span, .. }
        | ConicalSectorFailure::Point { span, .. }
        | ConicalSectorFailure::UnresolvedWedge { span, .. } => Some(*span),
        ConicalSectorFailure::Policy { .. }
        | ConicalSectorFailure::SpanCount(_)
        | ConicalSectorFailure::Angle(_)
        | ConicalSectorFailure::UnresolvedSine
        | ConicalSectorFailure::Trigonometric(_)
        | ConicalSectorFailure::Arithmetic(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surface_sketch::HistorySource;
    #[test]
    fn sector_failure_maps_only_the_actual_flattened_source_owner() {
        let request = super::super::super::tests::request();
        let inputs = input::capture(&request, &mut || false).unwrap();
        let failure = sector_failure(
            &inputs.originals,
            ConicalSectorFailure::UnresolvedWedge { span: 5, pole: 2 },
        )
        .public();
        assert_eq!(failure.class(), WrappedFailureClass::Unsupported);
        assert_eq!(
            failure.entities(),
            &[
                HistorySource::Region(SurfaceRegionId(1)),
                HistorySource::Curve(SurfaceCurveId(6))
            ]
        );
    }
}
