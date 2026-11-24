//! Preserve actual selection and sector receipts without rebuilding their owners.
use super::*;
use crate::surface_sketch::wrapped_profile::selection::PlanarSelectionEvidence;
use martensite_tkbo::profile_extrusion::{
    PlanarIntegralEvidence, PlanarProofVersion, PlanarRegionSet,
};

pub(super) struct AuthoredPlanarSelection {
    pub evidence: PlanarSelectionEvidence,
    program: Option<PlanarIntegralEvidence>,
}

pub(super) fn planar(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    execution: WrappedConeExecutionSelection,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<AuthoredPlanarSelection> {
    let (captured, program) = match execution.authored_planar_program() {
        Some(version) => {
            let (captured, program) =
                capture_program(request, originals, version, work, cancelled)?;
            (captured, Some(program))
        }
        None => (capture_legacy(request, originals, work, cancelled)?, None),
    };
    let members = planar::select_members(originals, &captured).map_err(|(region, cause)| {
        failure(Some(region), None, ConeCause::Planar(Box::new(cause)))
    })?;
    Ok(AuthoredPlanarSelection {
        evidence: PlanarSelectionEvidence {
            members,
            certificate: captured.certificate(),
            proof_work: captured.proof_work(),
        },
        program,
    })
}

fn capture_legacy(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<PlanarRegionSet> {
    let captured = planar::capture_with_work_and_cancel(
        originals,
        request.options.spatial_error,
        work.planar_remaining(request.options.planar_work),
        &mut *cancelled,
    )
    .map_err(|cause| failure(None, None, ConeCause::Planar(Box::new(cause))))?;
    work.charge_planar(first_region(originals)?, captured.proof_work())?;
    Ok(captured)
}

fn capture_program(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    version: PlanarProofVersion,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<(PlanarRegionSet, PlanarIntegralEvidence)> {
    let region = first_region(originals)?;
    let captured = material::capture_planar_program(
        originals,
        request.options.planar_work,
        version,
        work,
        request.proof.integration,
        cancelled,
        |_, cause| failure(None, None, ConeCause::Planar(Box::new(cause))),
    )?;
    let program = captured
        .integral_program_evidence()
        .filter(|program| planar::matches_integral_profile(program, originals))
        .ok_or_else(|| material::mismatch(region, None, "full-family authored planar program"))?
        .clone();
    Ok((captured, program))
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
    planar: AuthoredPlanarSelection,
    prepared: Vec<Prepared>,
    domain: domain::DomainEvidence,
    clearance: CertifiedCylindricalClearance,
    arithmetic_operations: u32,
) -> Selection {
    Selection {
        executed_selection,
        prepared,
        planar: planar.evidence.members,
        census: planar.evidence.certificate,
        work: planar.evidence.proof_work,
        authored_program: planar.program,
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
