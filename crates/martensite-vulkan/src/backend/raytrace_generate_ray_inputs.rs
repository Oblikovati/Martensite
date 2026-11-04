//! Explicit immutable packet binding; failed metadata never falls back to invented camera values.
use sha2::{Digest, Sha256};
use std::{
    io,
    path::{Path, PathBuf},
};
#[derive(Clone)]
pub(super) struct RayProbeInput {
    pub image: String,
    pub uniforms: Vec<u8>,
    pub constants: [u32; 10],
}
pub(super) trait RayInputReader {
    fn read(&self, name: &str) -> io::Result<Vec<u8>>;
}
pub(super) struct PreparedRayDirectory(pub PathBuf);
impl RayInputReader for PreparedRayDirectory {
    fn read(&self, name: &str) -> io::Result<Vec<u8>> {
        let path = self.0.join(name);
        let metadata = std::fs::symlink_metadata(&path)?;
        if Path::new(name).file_name().and_then(|part| part.to_str()) != Some(name)
            || !metadata.is_file()
            || metadata.file_type().is_symlink()
            || metadata.len() > 65536
        {
            return Err(invalid(format!(
                "path={path:?}, size={}; expected bounded regular direct input",
                metadata.len()
            )));
        }
        std::fs::read(path)
    }
}
pub(super) fn load_inputs(
    reader: &impl RayInputReader,
    expected: &str,
) -> io::Result<Vec<RayProbeInput>> {
    let manifest = read_bound_manifest(reader, expected)?;
    let rows = manifest_rows(&manifest)?;
    ["bug28890_1.png", "bug28890_2.png"]
        .into_iter()
        .map(|image| load_row(reader, rows, image))
        .collect()
}
fn read_bound_manifest(
    reader: &impl RayInputReader,
    expected: &str,
) -> io::Result<serde_json::Value> {
    let raw = reader.read("inputs.json")?;
    let actual = digest(&raw);
    if actual != expected {
        return Err(invalid(format!("manifest={actual}; expected {expected}")));
    }
    let manifest: serde_json::Value = serde_json::from_slice(&raw).map_err(io::Error::other)?;
    if manifest["schema"] != "martensite-generate-ray-inputs-v1"
        || manifest["qualification"] != false
    {
        return Err(invalid(
            "manifest schema/scope; expected explicit unqualified GenerateRay inputs".into(),
        ));
    }
    Ok(manifest)
}
fn manifest_rows(manifest: &serde_json::Value) -> io::Result<&[serde_json::Value]> {
    let rows = manifest["rows"]
        .as_array()
        .ok_or_else(|| invalid("rows; expected two captured packets".into()))?;
    if rows.len() != 2 {
        return Err(invalid(format!(
            "rows={}; expected exactly two",
            rows.len()
        )));
    }
    Ok(rows)
}

fn load_row(
    reader: &impl RayInputReader,
    rows: &[serde_json::Value],
    image: &str,
) -> io::Result<RayProbeInput> {
    let row = matching_image(rows, image)?;
    let bytes = bound_uniform_bytes(reader, row, image)?;
    validate_uniforms(&bytes)?;
    let constants = read_constants(reader, row)?;
    Ok(RayProbeInput {
        image: image.into(),
        uniforms: bytes,
        constants,
    })
}
fn matching_image<'a>(
    rows: &'a [serde_json::Value],
    image: &str,
) -> io::Result<&'a serde_json::Value> {
    let matching: Vec<_> = rows.iter().filter(|row| row["image"] == image).collect();
    if matching.len() != 1 {
        return Err(invalid(format!(
            "image={image}, matches={}; expected exactly one",
            matching.len()
        )));
    }
    Ok(matching[0])
}
fn bound_uniform_bytes(
    reader: &impl RayInputReader,
    row: &serde_json::Value,
    image: &str,
) -> io::Result<Vec<u8>> {
    let bytes = reader.read(&format!("{image}.uniforms.bin"))?;
    if bytes.len() != 416 || row["uniform_sha256"].as_str() != Some(digest(&bytes).as_str()) {
        return Err(invalid(format!(
            "image={image}, bytes={}; expected bound416-byte upload",
            bytes.len()
        )));
    }
    Ok(bytes)
}

fn read_constants(reader: &impl RayInputReader, row: &serde_json::Value) -> io::Result<[u32; 10]> {
    let bytes = reader.read("specialization.bin")?;
    if bytes.len() != 40 || row["specialization_sha256"].as_str() != Some(digest(&bytes).as_str()) {
        return Err(invalid(format!(
            "specialization bytes={}; expected bound40-byte actual block",
            bytes.len()
        )));
    }
    let values =
        std::array::from_fn(|i| u32::from_le_bytes(bytes[i * 4..i * 4 + 4].try_into().unwrap()));
    if values[8] != 0 || values[2..].iter().any(|value| *value > 1) {
        return Err(invalid(format!(
            "specialization={values:?}; expected actual booleans and depth-of-field off"
        )));
    }
    Ok(values)
}
fn validate_uniforms(bytes: &[u8]) -> io::Result<()> {
    validate_ray_window(bytes)?;
    for offset in [176, 192, 208, 224, 240, 256, 272, 288] {
        validate_ray_vector(bytes, offset)?;
    }
    Ok(())
}
fn validate_ray_window(bytes: &[u8]) -> io::Result<()> {
    for offset in [188, 204] {
        let dimension = i32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        if dimension != 409 {
            return Err(invalid(format!(
                "window offset{offset}={dimension}; expected409"
            )));
        }
    }
    Ok(())
}
fn validate_ray_vector(bytes: &[u8], offset: usize) -> io::Result<()> {
    let vector = std::array::from_fn::<_, 3, _>(|i| {
        f32::from_le_bytes(
            bytes[offset + i * 4..offset + i * 4 + 4]
                .try_into()
                .unwrap(),
        )
    });
    if vector.iter().any(|v| !v.is_finite()) || offset >= 240 && vector == [0.0; 3] {
        return Err(invalid(format!(
            "ray vector offset{offset}={vector:?}; expected finite origin/nonzero direction"
        )));
    }
    Ok(())
}

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
fn invalid(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
#[cfg(test)]
#[path = "raytrace_generate_ray_input_tests.rs"]
mod tests;
