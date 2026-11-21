// SPDX-License-Identifier: GPL-2.0-only
//! The consumer retains original mapped/census outcomes even when contact binding cannot complete.
use super::*;
use crate::sketch_region::{ArrangementSource, PartitionObservation};
use crate::sketch_region_partition_bindings::{inspect_outputs, required_partition_proofs};

/// Join actual owned receipts; no partial region or accepted candidate is returned.
/// Example: `transport_partition_contacts(census, mapped, policy, &sources, |_|false)`.
pub(crate) fn transport_partition_contacts(
    census: crate::sketch_region_certification::BoundaryCensus,
    mapped: crate::sketch_region_partition_bindings::PartitionBoundOutputs,
    policy: ContactTransportPolicy,
    sources: &[ArrangementSource<'_>],
    cancelled: impl FnMut(ContactTransportPhase) -> bool,
) -> Result<PairedContactTransaction, ContactTransportStop> {
    transport_partition_contacts_with_access(
        census,
        mapped,
        policy,
        sources,
        cancelled,
        crate::sketch_region_certification::SourceAccess::Live,
    )
}
pub(crate) fn transport_partition_contacts_with_access(
    census: crate::sketch_region_certification::BoundaryCensus,
    mapped: crate::sketch_region_partition_bindings::PartitionBoundOutputs,
    policy: ContactTransportPolicy,
    sources: &[ArrangementSource<'_>],
    mut cancelled: impl FnMut(ContactTransportPhase) -> bool,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) -> Result<PairedContactTransaction, ContactTransportStop> {
    let mut progress = super::progress::ContactProgress::new(&mut cancelled);
    let mut retained =
        checked_contact_transaction(empty_contact_transaction(census, mapped, policy))?;
    for phase in CONTACT_PHASES {
        retained = enter_contact_phase(retained, phase, &mut progress)?;
        execute_contact_phase(&mut retained, sources, phase, &mut progress, access);
        retained = retain_in_stage_stop(retained, &progress)?;
    }
    match access {
        crate::sketch_region_certification::SourceAccess::Live => {
            super::admission::inspect_contact_admission(&mut retained)
        }
        crate::sketch_region_certification::SourceAccess::Owned(_) => {
            super::admission::inspect_contact_admission_with_access(&mut retained, access)
        }
    }
    Ok(retained)
}
const CONTACT_PHASES: [ContactTransportPhase; 6] = [
    ContactTransportPhase::SourceBinding,
    ContactTransportPhase::CanonicalContacts,
    ContactTransportPhase::TopologyBinding,
    ContactTransportPhase::ParameterConstruction,
    ContactTransportPhase::GeneratedBinding,
    ContactTransportPhase::WorldConstruction,
];
fn execute_contact_phase(
    retained: &mut PairedContactTransaction,
    sources: &[ArrangementSource<'_>],
    phase: ContactTransportPhase,
    cancelled: &mut super::progress::ContactProgress<'_>,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) {
    if phase == ContactTransportPhase::SourceBinding {
        source_phase(retained, sources);
        return super::exact_sources::capture_exact_sources(retained, cancelled);
    }
    if phase == ContactTransportPhase::WorldConstruction {
        return match access {
            crate::sketch_region_certification::SourceAccess::Live => {
                super::world::retain_world_bounds(retained, cancelled)
            }
            crate::sketch_region_certification::SourceAccess::Owned(_) => {
                super::world::retain_world_bounds_with_access(retained, cancelled, access)
            }
        };
    }
    execute_observation_phase(retained, phase, access);
}
fn execute_observation_phase(
    retained: &mut PairedContactTransaction,
    phase: ContactTransportPhase,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) {
    use super::{generated::retain_generated_vertices, topology::*};
    match phase {
        ContactTransportPhase::CanonicalContacts => retain_canonical_contacts(retained, access),
        ContactTransportPhase::TopologyBinding => retain_topological_incidences(retained),
        ContactTransportPhase::GeneratedBinding => retain_generated_vertices(retained),
        ContactTransportPhase::ParameterConstruction => match access {
            crate::sketch_region_certification::SourceAccess::Live => {
                retain_parameter_measurements(retained)
            }
            crate::sketch_region_certification::SourceAccess::Owned(_) => {
                retain_parameter_measurements_with_access(retained, access)
            }
        },
        ContactTransportPhase::SourceBinding | ContactTransportPhase::WorldConstruction => {}
    }
}

fn enter_contact_phase(
    retained: PairedContactTransaction,
    phase: ContactTransportPhase,
    cancelled: &mut super::progress::ContactProgress<'_>,
) -> Result<PairedContactTransaction, ContactTransportStop> {
    if cancelled.poll(phase) {
        return Err(ContactTransportStop::Cancelled {
            phase,
            completed: Box::new(retained),
        });
    }
    Ok(retained)
}

fn checked_contact_transaction(
    retained: PairedContactTransaction,
) -> Result<PairedContactTransaction, ContactTransportStop> {
    if let Err((field, value)) = validate_transport_policy(&retained.requested) {
        return Err(ContactTransportStop::Policy {
            field,
            value,
            completed: Box::new(retained),
        });
    }
    Ok(retained)
}

fn empty_contact_transaction(
    census: crate::sketch_region_certification::BoundaryCensus,
    mapped: crate::sketch_region_partition_bindings::PartitionBoundOutputs,
    requested: ContactTransportPolicy,
) -> PairedContactTransaction {
    PairedContactTransaction {
        census,
        mapped,
        requested,
        current_partition: None,
        exact_sources: Vec::new(),
        inspection: ContactTransportInspection {
            completion: ContactTransportCompletion::Incomplete,
            contacts: Vec::new(),
            missing: Vec::new(),
            remaining_operation_proofs: required_partition_proofs(),
            completed_bisections: 0,
        },
    }
}
fn validate_transport_policy(policy: &ContactTransportPolicy) -> Result<(), (&'static str, f64)> {
    for (field, value) in [
        (
            "native t parameter limit",
            policy.native_parameter_limits[0],
        ),
        (
            "native u parameter limit",
            policy.native_parameter_limits[1],
        ),
        ("world endpoint limit", policy.world_endpoint_limit),
    ] {
        if !value.is_finite() || value < 0. {
            return Err((field, value));
        }
    }
    Ok(())
}
fn source_phase(retained: &mut PairedContactTransaction, sources: &[ArrangementSource<'_>]) {
    // Issue #10: contact coordinates can survive an edit; join DS/current sources to the census revision.
    retained.current_partition = Some(inspect_outputs(
        &retained.mapped.arrangement.partition,
        retained.mapped.copied.as_ref(),
        &retained.census.inputs.originals,
        sources,
    ));
    match &retained.mapped.arrangement.partition {
        PartitionObservation::NotReached => retained
            .inspection
            .missing
            .push(ContactTransportGap::MissingPartitionCapture),
        PartitionObservation::Incomplete { .. } => retained
            .inspection
            .missing
            .push(ContactTransportGap::IncompletePartitionCapture),
        PartitionObservation::Captured(_) => {}
    }
}
fn retain_canonical_contacts(
    retained: &mut PairedContactTransaction,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) {
    let certificates = access.certificates(&retained.census);
    for (index, domain) in retained.census.domains.iter().enumerate() {
        retain_domain_contacts(
            &mut retained.inspection,
            certificates.as_ref(),
            domain,
            index,
        );
    }
}
fn retain_domain_contacts(
    inspection: &mut ContactTransportInspection,
    certificates: Option<&crate::sketch_region_certification::DomainCertificates<'_>>,
    domain: &crate::sketch_region_certification::BoundaryDomain,
    index: usize,
) {
    let Some(proof) = certificates.and_then(|current| current.unified_certificate(domain)) else {
        inspection
            .missing
            .push(ContactTransportGap::MissingSourceBoundDomain { domain: index });
        return;
    };
    let report = match domain.evidence {
        crate::sketch_region_certification::DomainEvidence::PairedReport { report, .. }
        | crate::sketch_region_certification::DomainEvidence::BernsteinReport { report, .. } => {
            report
        }
        _ => return,
    };
    retain_report_contacts(inspection, &proof, report, index);
}

fn retain_report_contacts(
    inspection: &mut ContactTransportInspection,
    proof: &crate::sketch_region_certification::CertifiedPairDomain<'_>,
    report: usize,
    domain_index: usize,
) {
    for contact in 0..proof.contact_count() {
        let identity = ReportContactIdentity {
            report,
            candidate_root: proof.contact(contact).unwrap().candidate_index,
        };
        retain_contact_occurrence(inspection, identity, domain_index);
    }
}

fn retain_contact_occurrence(
    inspection: &mut ContactTransportInspection,
    identity: ReportContactIdentity,
    domain: usize,
) {
    if let Some(previous) = inspection
        .contacts
        .iter_mut()
        .find(|row| row.identity == identity)
    {
        previous.occurrence_domains.push(domain);
        return;
    }
    inspection
        .contacts
        .push(new_contact_occurrence(identity, domain));
}
fn new_contact_occurrence(identity: ReportContactIdentity, domain: usize) -> ObservedPairedContact {
    ObservedPairedContact {
        identity,
        bounded_construction: None,
        occurrence_domains: vec![domain],
        observed_vertices: Vec::new(),
        world_measurements: Vec::new(),
        missing: vec![ContactTransportGap::MissingActualPairIncidence { identity }],
    }
}

fn retain_in_stage_stop(
    retained: PairedContactTransaction,
    progress: &super::progress::ContactProgress<'_>,
) -> Result<PairedContactTransaction, ContactTransportStop> {
    if let Some(phase) = progress.stopped() {
        return Err(ContactTransportStop::Cancelled {
            phase,
            completed: Box::new(retained),
        });
    }
    Ok(retained)
}
