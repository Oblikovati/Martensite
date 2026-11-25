//! Materialize one operation-local cone tool and retain immutable canonical authority.
pub(in crate::surface_sketch::wrapped_profile) use super::actual::attachment::{
    AttachmentAuthority, AttachmentToolWork,
};
use super::*;
use crate::surface_sketch::{encoding, wrapped_profile::operation::stages::SupportStages};

pub(in crate::surface_sketch::wrapped_profile) struct CanonicalTool {
    pub graph: frozen_placement::DecodedGraph,
    pub bytes: Vec<u8>,
    pub keys: Vec<BindingKey>,
    pub history: history_projection::HistoryPublication,
    pub regions: Vec<SurfaceRegionId>,
    pub authority: AttachmentAuthority,
}

pub(super) struct Selection {
    pub execution: WrappedConeExecutionSelection,
    pub prepared: Vec<Prepared>,
    pub domain: domain::DomainEvidence,
    pub clearance: CertifiedCylindricalClearance,
    pub authored: prepare::authored_cycles::Authority,
}

pub(in crate::surface_sketch::wrapped_profile) fn materialize(
    request: &ConeRun<'_>,
    execution_selection: WrappedConeExecutionSelection,
    mut inputs: Inputs,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<CanonicalTool> {
    let stages = stages::ConeStages {
        request,
        execution_selection,
    };
    inputs.grant = input::grant::capture(request, execution_selection, &inputs.originals)
        .map_err(ConeFailure::public)?;
    let mut work = stages.context();
    let selected = select(&stages, &inputs, &mut work, cancelled).map_err(ConeFailure::public)?;
    let mut checkpoint = |_| cancelled();
    let mut control = family::Control::new(&mut checkpoint);
    let (root, bindings) =
        frozen::assemble(&stages, &selected.prepared, &mut control).map_err(ConeFailure::public)?;
    let (_, members) = coverage::certify_developed_family(&selected.prepared, &root, &bindings)
        .map_err(|error| stages::ConeStages::coverage(error).public())?;
    let keys: Vec<_> = bindings.iter().map(|(key, _)| *key).collect();
    let history = history(request, &inputs, &selected, &keys)?;
    let bytes = encoding::encode(&root, &bindings)
        .map_err(stages::ConeStages::encoding)
        .map_err(ConeFailure::public)?;
    let regions = members.iter().map(|member| member.region).collect();
    let authority = actual::attachment::issue(
        request,
        &inputs.originals,
        selected,
        members,
        &mut work,
        &mut control,
    )
    .map_err(ConeFailure::public)?;
    Ok(CanonicalTool {
        graph: (root, bindings),
        bytes,
        keys,
        history,
        regions,
        authority,
    })
}

fn select(
    stages: &stages::ConeStages<'_>,
    inputs: &Inputs,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Selection> {
    let execution = stages.execution_selection;
    let request = stages.request;
    let originals = &inputs.originals;
    let authored =
        prepare::authored_cycles::certify(request, originals, execution, work, cancelled)?;
    let (prepared, domain, clearance) =
        prepare::admitted_geometry(request, originals, execution, work, cancelled)?;
    if let Some(grant) = &inputs.grant {
        grant.validate(request, execution, originals)?;
    }
    Ok(Selection {
        execution,
        prepared,
        domain,
        clearance,
        authored,
    })
}

fn history(
    request: &ConeRun<'_>,
    inputs: &Inputs,
    selection: &Selection,
    keys: &[BindingKey],
) -> Checked<history_projection::HistoryPublication> {
    let history = history::certify_developed(&inputs.originals, &selection.prepared, keys)
        .map_err(|cause| {
            ConeFailure::new(
                None,
                None,
                WrappedFailureClass::Incomplete,
                ConeCause::History(Box::new(cause)),
            )
            .public()
        })?;
    history_projection::prepare_physical(keys, &history, &inputs.directions, request.support.normal)
        .map_err(publication::history_failure)
}

pub(in crate::surface_sketch::wrapped_profile) fn unsupported() -> WrappedNormalFailure {
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Unsupported,
        ConeCause::Binding {
            field: "V9 directed cone attachment with Report volume",
            actual: vec![],
            expected: vec![1],
        },
    )
    .at(WrappedStage::Admission)
    .public()
}
