//! Field-wise snapshots retain initialized API slots without copying padding or pointers.
use ash::vk;

#[derive(Debug)]
pub(crate) struct RawProperty {
    pub structure_type: i32,
    pub next_present: bool,
    pub stages: u32,
    pub name: Vec<u8>,
    pub description: Vec<u8>,
    pub subgroup_size: u32,
}
#[derive(Debug)]
pub(crate) struct RawRepresentation {
    pub structure_type: i32,
    pub next_present: bool,
    pub name: Vec<u8>,
    pub description: Vec<u8>,
    pub text: u32,
    pub reported_bytes: usize,
    pub data_present: bool,
}
#[derive(Debug)]
pub(crate) struct RawCall {
    pub operation: &'static str,
    pub status: i32,
    pub allocated_slots: usize,
    pub reported_count: u32,
    pub validity: &'static str,
    pub properties: Vec<RawProperty>,
    pub representations: Vec<RawRepresentation>,
}
#[derive(Default)]
pub(crate) struct RawQueryEvidence {
    pub calls: Vec<RawCall>,
    pub requested_bytes: Option<usize>,
    pub reserved_bytes: usize,
    pub allocation_sizes: Vec<usize>,
    pub buffers: Vec<Vec<u8>>,
}
impl std::fmt::Debug for RawQueryEvidence {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let calls = call_summary(&self.calls);
        output
            .debug_struct("RawQueryEvidence")
            .field("calls", &calls)
            .field("requested_bytes", &self.requested_bytes)
            .field("reserved_bytes", &self.reserved_bytes)
            .field("allocation_sizes", &self.allocation_sizes)
            .field("retained_buffers", &self.buffers.len())
            .finish()
    }
}
fn fixed_bytes(field: &[std::ffi::c_char]) -> Vec<u8> {
    // The whole bounded field is retained, including unused trailing bytes.
    field.iter().map(|value| *value as u8).collect()
}
fn validity(status: vk::Result, reported: u32, allocated: usize) -> &'static str {
    if status == vk::Result::SUCCESS && reported as usize <= allocated {
        return "API_SUCCESS_REPORTED_PREFIX_ONLY";
    }
    // Errors and INCOMPLETE do not certify the contents as a complete valid result.
    "UNKNOWN_OR_INCOMPLETE_RETURNED_FIELDS"
}
fn call(operation: &'static str, status: vk::Result, reported: u32, allocated: usize) -> RawCall {
    RawCall {
        operation,
        status: status.as_raw(),
        allocated_slots: allocated,
        reported_count: reported,
        validity: validity(status, reported, allocated),
        properties: Vec::new(),
        representations: Vec::new(),
    }
}
/// Snapshot every allocated property slot; `property_call(status, count, &slots)`.
pub(super) fn property_call(
    status: vk::Result,
    count: u32,
    slots: &[vk::PipelineExecutablePropertiesKHR<'_>],
) -> RawCall {
    let mut raw = call("executable properties", status, count, slots.len());
    raw.properties = slots
        .iter()
        .map(|row| RawProperty {
            structure_type: row.s_type.as_raw(),
            next_present: !row.p_next.is_null(),
            stages: row.stages.as_raw(),
            name: fixed_bytes(&row.name),
            description: fixed_bytes(&row.description),
            subgroup_size: row.subgroup_size,
        })
        .collect();
    raw
}
/// Snapshot scalar/array fields only; `representation_call("IR bytes", status, count, &slots)`.
pub(super) fn representation_call(
    operation: &'static str,
    status: vk::Result,
    count: u32,
    slots: &[vk::PipelineExecutableInternalRepresentationKHR<'_>],
) -> RawCall {
    let mut raw = call(operation, status, count, slots.len());
    raw.representations = slots.iter().map(snapshot_representation).collect();
    raw
}
fn snapshot_representation(
    row: &vk::PipelineExecutableInternalRepresentationKHR<'_>,
) -> RawRepresentation {
    RawRepresentation {
        structure_type: row.s_type.as_raw(),
        next_present: !row.p_next.is_null(),
        name: fixed_bytes(&row.name),
        description: fixed_bytes(&row.description),
        text: row.is_text,
        reported_bytes: row.data_size,
        data_present: !row.p_data.is_null(),
    }
}

/// Preserve unpaired creation inputs without inventing a stage/handle association.
pub(crate) struct RawModuleInputs {
    pub handles: Vec<u64>,
    pub inputs: Vec<&'static [u8]>,
}
impl std::fmt::Debug for RawModuleInputs {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lengths: Vec<_> = self.inputs.iter().map(|bytes| bytes.len()).collect();
        output
            .debug_struct("RawModuleInputs")
            .field("handles", &self.handles)
            .field("input_lengths", &lengths)
            .finish()
    }
}

fn call_summary(calls: &[RawCall]) -> Vec<(&'static str, i32, usize, u32, &'static str)> {
    calls
        .iter()
        .map(|row| {
            (
                row.operation,
                row.status,
                row.allocated_slots,
                row.reported_count,
                row.validity,
            )
        })
        .collect()
}
