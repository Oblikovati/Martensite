//! Bounded C fields and driver text stay distinct from binary representations.
use super::records::{ExecutableGap, RepresentationMetadata, contract};
use ash::vk;

/// Decode only within the API field; `bounded_text(&property.name)?`.
pub(super) fn bounded_text(bytes: &[std::ffi::c_char]) -> Result<String, ExecutableGap> {
    let end = bytes
        .iter()
        .position(|byte| *byte == 0)
        .ok_or_else(|| contract("C string", bytes, "NUL within fixed API field"))?;
    let owned: Vec<u8> = bytes[..end].iter().map(|byte| *byte as u8).collect();
    String::from_utf8(owned).map_err(|error| contract("C string", error, "valid UTF-8"))
}
/// Retain actual driver metadata; `representation(&descriptor)?`.
pub(super) fn representation(
    row: &vk::PipelineExecutableInternalRepresentationKHR<'_>,
) -> Result<RepresentationMetadata, ExecutableGap> {
    if row.is_text != vk::FALSE && row.is_text != vk::TRUE {
        return Err(contract("isText", row.is_text, "VK_FALSE or VK_TRUE"));
    }
    Ok(RepresentationMetadata {
        name: bounded_text(&row.name)?,
        description: bounded_text(&row.description)?,
        text: row.is_text == vk::TRUE,
        bytes: row.data_size,
    })
}
/// Refuse malformed text while preserving binary bytes; `require_text(metadata, bytes)?`.
pub(super) fn require_text(
    metadata: &RepresentationMetadata,
    bytes: &[u8],
) -> Result<(), ExecutableGap> {
    if !metadata.text {
        return Ok(());
    }
    if bytes.last() != Some(&0) {
        return Err(contract(
            "text bytes",
            bytes.len(),
            "complete NUL-terminated representation",
        ));
    }
    std::str::from_utf8(bytes).map_err(|error| contract("text UTF-8", error, "valid full text"))?;
    Ok(())
}
