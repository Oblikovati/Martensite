//! Fixed capacities refuse incomplete or changing representations instead of retrying/truncating.
use super::{
    api::ExecutableApi,
    metadata, properties,
    records::{
        ExecutableGap, ExecutableWitness, OwnedRepresentation, RepresentationMetadata, contract,
    },
};
use ash::{vk, vk::Handle};
const REPRESENTATION_LIMIT: usize = 16;
const BYTE_LIMIT: usize = 8 * 1024 * 1024;

/// Read this pipeline without rebuilding it; `capture(api, pipeline, budget)?`.
pub(super) fn capture(
    api: &mut dyn ExecutableApi,
    pipeline: vk::Pipeline,
    budget: &mut super::CaptureByteBudget,
) -> Result<ExecutableWitness, ExecutableGap> {
    let mut raw = super::raw::RawQueryEvidence::default();
    match capture_inner(api, pipeline, budget, &mut raw) {
        Ok(mut witness) => {
            witness.raw = raw;
            Ok(witness)
        }
        Err(cause) => Err(ExecutableGap::PartialQuery {
            evidence: Box::new(raw),
            cause: Box::new(cause),
        }),
    }
}
fn capture_inner(
    api: &mut dyn ExecutableApi,
    pipeline: vk::Pipeline,
    budget: &mut super::CaptureByteBudget,
    raw: &mut super::raw::RawQueryEvidence,
) -> Result<ExecutableWitness, ExecutableGap> {
    let properties = properties::query(api, pipeline, raw)?;
    let selected = properties::fragment_index(&properties)?;
    let representations = read_representations(api, pipeline, selected, budget, raw)?;
    require_assembly(ExecutableWitness {
        pipeline: pipeline.as_raw(),
        properties,
        selected,
        representations,
        raw: Default::default(),
    })
}
fn require_assembly(witness: ExecutableWitness) -> Result<ExecutableWitness, ExecutableGap> {
    if !witness.representations.iter().any(is_assembly) {
        return Err(ExecutableGap::MissingAssembly(Box::new(witness)));
    }
    Ok(witness)
}

fn is_assembly(row: &OwnedRepresentation) -> bool {
    let label = format!("{} {}", row.metadata.name, row.metadata.description).to_ascii_lowercase();
    row.metadata.text
        && ["assembly", "disassembly", "isa"]
            .iter()
            .any(|word| label.contains(word))
}
fn read_representations(
    api: &mut dyn ExecutableApi,
    pipeline: vk::Pipeline,
    index: u32,
    budget: &mut super::CaptureByteBudget,
    raw: &mut super::raw::RawQueryEvidence,
) -> Result<Vec<OwnedRepresentation>, ExecutableGap> {
    let mut prepared = prepare_descriptors(api, pipeline, index, raw)?;
    allocate(&prepared.expected, budget, raw)?;
    bind_buffers(&mut prepared.slots, &mut raw.buffers);
    let (status, written) = api.representations(pipeline, index, &mut prepared.slots);
    record_ir_call(raw, "IR bytes", status, written, &prepared.slots);
    clear_data_pointers(&mut prepared.slots);
    properties::require_complete("IR bytes", pipeline, status, written, prepared.slots.len())?;
    decode(&prepared.slots, prepared.expected, written, raw)
}
struct PreparedIr {
    slots: Vec<vk::PipelineExecutableInternalRepresentationKHR<'static>>,
    expected: Vec<RepresentationMetadata>,
}
fn prepare_descriptors(
    api: &mut dyn ExecutableApi,
    pipeline: vk::Pipeline,
    index: u32,
    raw: &mut super::raw::RawQueryEvidence,
) -> Result<PreparedIr, ExecutableGap> {
    let mut slots =
        vec![vk::PipelineExecutableInternalRepresentationKHR::default(); REPRESENTATION_LIMIT];
    let (status, count) = api.representations(pipeline, index, &mut slots);
    record_ir_call(raw, "IR descriptors", status, count, &slots);
    properties::require_complete("IR descriptors", pipeline, status, count, slots.len())?;
    slots.truncate(count as usize);
    let expected = slots
        .iter()
        .map(metadata::representation)
        .collect::<Result<_, _>>()?;
    Ok(PreparedIr { slots, expected })
}
fn record_ir_call(
    raw: &mut super::raw::RawQueryEvidence,
    operation: &'static str,
    status: vk::Result,
    count: u32,
    slots: &[vk::PipelineExecutableInternalRepresentationKHR<'_>],
) {
    raw.calls.push(super::raw::representation_call(
        operation, status, count, slots,
    ));
}
fn clear_data_pointers(slots: &mut [vk::PipelineExecutableInternalRepresentationKHR<'_>]) {
    // Snapshot presence first; pointer-bearing API descriptions never enter an owned result.
    for slot in slots {
        slot.p_data = std::ptr::null_mut();
    }
}
fn allocate(
    rows: &[RepresentationMetadata],
    budget: &mut super::CaptureByteBudget,
    raw: &mut super::raw::RawQueryEvidence,
) -> Result<(), ExecutableGap> {
    let total = allocation_total(rows)?;
    raw.requested_bytes = Some(total);
    require_allocation_size(rows, total)?;
    budget.charge(total)?;
    raw.reserved_bytes = total;
    raw.allocation_sizes = rows.iter().map(|row| row.bytes).collect();
    raw.buffers = rows.iter().map(|row| vec![0xa5; row.bytes]).collect();
    Ok(())
}
fn allocation_total(rows: &[RepresentationMetadata]) -> Result<usize, ExecutableGap> {
    rows.iter()
        .try_fold(0usize, |sum, row| sum.checked_add(row.bytes))
        .ok_or_else(|| contract("IR sizes", rows, "nonoverflowing aggregate byte size"))
}
fn require_allocation_size(
    rows: &[RepresentationMetadata],
    total: usize,
) -> Result<(), ExecutableGap> {
    if total > BYTE_LIMIT || rows.iter().any(|row| row.bytes == 0) {
        return Err(contract(
            "IR byte sizes",
            (total, rows),
            "positive buffers and aggregate <=8MiB",
        ));
    }
    Ok(())
}

fn bind_buffers(
    slots: &mut [vk::PipelineExecutableInternalRepresentationKHR<'_>],
    buffers: &mut [Vec<u8>],
) {
    for (slot, bytes) in slots.iter_mut().zip(buffers) {
        // All vectors are fully sized before pointers are borrowed; no resize occurs until return.
        slot.p_data = bytes.as_mut_ptr().cast();
        slot.data_size = bytes.len();
    }
}
fn decode(
    slots: &[vk::PipelineExecutableInternalRepresentationKHR<'_>],
    expected: Vec<RepresentationMetadata>,
    written: u32,
    raw: &mut super::raw::RawQueryEvidence,
) -> Result<Vec<OwnedRepresentation>, ExecutableGap> {
    require_count(written, expected.len())?;
    require_metadata(slots, &expected)?;
    for (metadata, bytes) in expected.iter().zip(&raw.buffers) {
        metadata::require_text(metadata, bytes)?;
    }
    let buffers = std::mem::take(&mut raw.buffers);
    Ok(expected
        .into_iter()
        .zip(buffers)
        .map(|(metadata, bytes)| OwnedRepresentation { metadata, bytes })
        .collect())
}

fn require_count(written: u32, expected: usize) -> Result<(), ExecutableGap> {
    if written as usize != expected {
        return Err(contract(
            "IR count",
            (written, expected),
            "unchanged full descriptor count",
        ));
    }
    Ok(())
}
fn require_metadata(
    slots: &[vk::PipelineExecutableInternalRepresentationKHR<'_>],
    expected: &[RepresentationMetadata],
) -> Result<(), ExecutableGap> {
    let actual: Vec<_> = slots
        .iter()
        .map(metadata::representation)
        .collect::<Result<_, _>>()?;
    if actual != expected {
        return Err(contract(
            "IR metadata",
            (&actual, expected),
            "identical names/descriptions/text flags/sizes",
        ));
    }
    Ok(())
}
