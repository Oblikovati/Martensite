//! Raw retained bytes make independent module and representation hashes reproducible.
use super::{CapturedExecutableFrame, records::ExecutableWitness};
use std::{io, path::Path};

/// Retain raw actual module and IR bytes; `retain(result, directory)?`.
pub(super) fn retain(result: &CapturedExecutableFrame, directory: &Path) -> io::Result<()> {
    retain_modules(result, directory)?;
    emit_attachment(result);
    println!(
        "{{\"event\":\"selected-pipeline\",\"pipeline\":{},\"device\":{},\"physical\":{},\"width\":{},\"height\":{},\"capture\":{}}}",
        result.pipeline,
        result.device,
        result.physical,
        result.capture.width,
        result.capture.height,
        result.witness.is_some()
    );
    if let Some(witness) = &result.witness {
        retain_witness(witness, directory)?;
    }
    Ok(())
}
fn retain_modules(result: &CapturedExecutableFrame, directory: &Path) -> io::Result<()> {
    for (index, (stage, module, bytes)) in result.modules.iter().enumerate() {
        std::fs::write(directory.join(format!("module-{index}-{stage}.spv")), bytes)?;
        println!(
            "{{\"event\":\"selected-module\",\"pipeline\":{},\"stage\":{stage},\"module\":{module},\"bytes\":{}}}",
            result.pipeline,
            bytes.len()
        );
    }
    Ok(())
}
/// Keep incomplete witnesses inspectable; `retain_witness(witness, directory)?`.
pub(super) fn retain_witness(witness: &ExecutableWitness, directory: &Path) -> io::Result<()> {
    super::raw_output::retain(&witness.raw, &directory.join("witness-query"))?;
    for property in &witness.properties {
        retain_property(witness, property);
    }
    for (index, row) in witness.representations.iter().enumerate() {
        retain_representation(witness, index, row, directory)?;
    }
    Ok(())
}
fn retain_property(witness: &ExecutableWitness, property: &super::records::ExecutableProperty) {
    println!(
        "{{\"event\":\"pipeline-executable\",\"pipeline\":{},\"index\":{},\"stages\":{},\"subgroup\":{},\"name_hex\":\"{}\",\"description_hex\":\"{}\",\"selected\":{}}}",
        witness.pipeline,
        property.index,
        property.stages,
        property.subgroup_size,
        hex(property.name.as_bytes()),
        hex(property.description.as_bytes()),
        property.index == witness.selected
    );
}
fn retain_representation(
    witness: &ExecutableWitness,
    index: usize,
    row: &super::records::OwnedRepresentation,
    directory: &Path,
) -> io::Result<()> {
    std::fs::write(
        directory.join(format!("representation-{index}.bin")),
        &row.bytes,
    )?;
    emit_ir(witness, index, row);
    Ok(())
}
fn hex(bytes: &[u8]) -> String {
    // Encoding opaque names avoids JSON escaping ambiguity without rewriting the retained bytes.
    let encoded: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    encoded
}

fn emit_ir(witness: &ExecutableWitness, index: usize, row: &super::records::OwnedRepresentation) {
    println!(
        "{{\"event\":\"pipeline-ir\",\"pipeline\":{},\"executable\":{},\"index\":{index},\"bytes\":{},\"text\":{},\"name_hex\":\"{}\",\"description_hex\":\"{}\"}}",
        witness.pipeline,
        witness.selected,
        row.bytes.len(),
        row.metadata.text,
        hex(row.metadata.name.as_bytes()),
        hex(row.metadata.description.as_bytes())
    );
}

fn emit_attachment(result: &CapturedExecutableFrame) {
    let actual = &result.attachment;
    println!(
        "{{\"event\":\"selected-attachment\",\"pipeline\":{},\"color_format\":{},\"depth_format\":{},\"samples\":{},\"raster\":\"{:?}\",\"coordinates\":\"{:?}\"}}",
        result.pipeline,
        actual.color,
        actual.depth,
        actual.samples,
        actual.raster,
        actual.coordinates
    );
}

/// Record a typed incomplete result; `retain_gap(gap, directory)?`.
pub(super) fn retain_gap(gap: &super::ExecutableGap, directory: &Path) -> io::Result<()> {
    std::fs::create_dir_all(directory)?;
    let context = hex(format!("{gap:?}").as_bytes());
    let record =
        format!("{{\"event\":\"selected-executable-gap\",\"context_hex\":\"{context}\"}}\n");
    std::fs::write(directory.join("gap.json"), record.as_bytes())?;
    println!("{}", record.trim_end());
    retain_gap_payload(gap, directory)
}
fn retain_gap_payload(gap: &super::ExecutableGap, directory: &Path) -> io::Result<()> {
    use super::ExecutableGap;
    match gap {
        ExecutableGap::PartialQuery { evidence, cause } => {
            retain_partial(evidence, cause, directory)
        }
        ExecutableGap::CapturedFrame { frame, cause } => retain_captured(frame, cause, directory),
        ExecutableGap::ModuleInputs { evidence, cause } => {
            retain_unpaired_modules(evidence, cause, directory)
        }
        ExecutableGap::BoundPipeline { pipeline, cause } => {
            println!("{{\"event\":\"selected-pipeline-gap\",\"pipeline\":{pipeline}}}");
            retain_gap(cause, &directory.join("cause"))
        }
        ExecutableGap::MissingAssembly(witness) => retain_witness(witness, directory),
        _ => Ok(()),
    }
}
fn retain_partial(
    raw: &super::raw::RawQueryEvidence,
    cause: &super::ExecutableGap,
    directory: &Path,
) -> io::Result<()> {
    super::raw_output::retain(raw, &directory.join("partial-query"))?;
    retain_gap(cause, &directory.join("cause"))
}
fn retain_captured(
    frame: &CapturedExecutableFrame,
    cause: &super::ExecutableGap,
    directory: &Path,
) -> io::Result<()> {
    let path = directory.join("captured-frame");
    std::fs::create_dir_all(&path)?;
    retain(frame, &path)?;
    std::fs::write(path.join("frame.rgba"), &frame.capture.rgba)?;
    let depth: Vec<_> = frame
        .capture
        .depth
        .iter()
        .flat_map(|value| value.to_bits().to_le_bytes())
        .collect();
    std::fs::write(path.join("frame.depth"), depth)?;
    retain_gap(cause, &directory.join("cause"))
}
fn retain_unpaired_modules(
    raw: &super::raw::RawModuleInputs,
    cause: &super::ExecutableGap,
    directory: &Path,
) -> io::Result<()> {
    retain_module_index(raw, directory)?;
    for (index, bytes) in raw.inputs.iter().enumerate() {
        std::fs::write(
            directory.join(format!("unpaired-module-input-{index}.bin")),
            bytes,
        )?;
    }
    retain_gap(cause, &directory.join("cause"))
}

fn retain_module_index(raw: &super::raw::RawModuleInputs, directory: &Path) -> io::Result<()> {
    let summary = format!(
        "{{\"handles\":{:?},\"input_count\":{}}}",
        raw.handles,
        raw.inputs.len()
    );
    std::fs::write(directory.join("unpaired-module-handles.json"), summary)
}
