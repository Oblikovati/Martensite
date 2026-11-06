//! Retain raw fields and allocated bytes before interpreting an incomplete call.
use super::raw::{RawCall, RawProperty, RawQueryEvidence, RawRepresentation};
use std::{io, path::Path};

/// Preserve every owned raw field and byte; `retain(raw, directory)?`.
pub(super) fn retain(raw: &RawQueryEvidence, directory: &Path) -> io::Result<()> {
    std::fs::create_dir_all(directory)?;
    retain_allocation(raw, directory)?;
    for (index, call) in raw.calls.iter().enumerate() {
        retain_call(call, &directory.join(format!("call-{index}")))?;
    }
    for (index, bytes) in raw.buffers.iter().enumerate() {
        std::fs::write(
            directory.join(format!("allocated-buffer-{index}.bin")),
            bytes,
        )?;
    }
    Ok(())
}
fn retain_allocation(raw: &RawQueryEvidence, directory: &Path) -> io::Result<()> {
    let requested = raw
        .requested_bytes
        .map_or_else(|| "null".into(), |value| value.to_string());
    let summary = format!(
        "{{\"requested_bytes\":{requested},\"reserved_bytes\":{},\"allocation_sizes\":{:?},\"retained_buffers\":{}}}\n",
        raw.reserved_bytes,
        raw.allocation_sizes,
        raw.buffers.len()
    );
    std::fs::write(directory.join("allocation.json"), summary)
}
fn retain_call(call: &RawCall, directory: &Path) -> io::Result<()> {
    std::fs::create_dir_all(directory)?;
    let summary = format!(
        "{{\"operation\":\"{}\",\"status\":{},\"allocated_slots\":{},\"reported_count\":{},\"validity\":\"{}\"}}\n",
        call.operation, call.status, call.allocated_slots, call.reported_count, call.validity
    );
    std::fs::write(directory.join("call.json"), summary)?;
    for (index, row) in call.properties.iter().enumerate() {
        retain_property(row, index, directory)?;
    }
    for (index, row) in call.representations.iter().enumerate() {
        retain_representation(row, index, directory)?;
    }
    Ok(())
}
fn retain_property(row: &RawProperty, index: usize, directory: &Path) -> io::Result<()> {
    retain_labels("property", index, directory, &row.name, &row.description)?;
    let summary = format!(
        "{{\"slot\":{index},\"structure_type\":{},\"next_present\":{},\"stages\":{},\"subgroup_size\":{}}}\n",
        row.structure_type, row.next_present, row.stages, row.subgroup_size
    );
    std::fs::write(directory.join(format!("property-{index}.json")), summary)
}
fn retain_representation(
    row: &RawRepresentation,
    index: usize,
    directory: &Path,
) -> io::Result<()> {
    retain_labels(
        "representation",
        index,
        directory,
        &row.name,
        &row.description,
    )?;
    let summary = representation_summary(row, index);
    std::fs::write(
        directory.join(format!("representation-{index}.json")),
        summary,
    )
}

fn retain_labels(
    kind: &str,
    index: usize,
    directory: &Path,
    name: &[u8],
    description: &[u8],
) -> io::Result<()> {
    std::fs::write(directory.join(format!("{kind}-{index}-name.bin")), name)?;
    std::fs::write(
        directory.join(format!("{kind}-{index}-description.bin")),
        description,
    )
}

fn representation_summary(row: &RawRepresentation, index: usize) -> String {
    format!(
        "{{\"slot\":{index},\"structure_type\":{},\"next_present\":{},\"is_text\":{},\"reported_bytes\":{},\"data_present\":{}}}\n",
        row.structure_type, row.next_present, row.text, row.reported_bytes, row.data_present
    )
}
