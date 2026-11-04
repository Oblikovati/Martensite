//! Successful actual API upload inputs, not GPU readback or reconstructed BVH content.
use ash::{vk, vk::Handle};
use sha2::{Digest, Sha256};
pub(super) struct HitTexelUpload {
    pub buffer: u64,
    pub view: u64,
    pub format: i32,
    pub bytes: usize,
    pub sha256: String,
}
impl HitTexelUpload {
    pub fn new(buffer: vk::Buffer, view: vk::BufferView, format: vk::Format, bytes: &[u8]) -> Self {
        Self {
            buffer: buffer.as_raw(),
            view: view.as_raw(),
            format: format.as_raw(),
            bytes: bytes.len(),
            sha256: hit_sha256(bytes),
        }
    }
    fn json(&self, binding: usize) -> String {
        format!(
            "{{\"binding\":{binding},\"buffer\":{},\"view\":{},\"format\":{},\"bytes\":{},\"sha256\":\"{}\"}}",
            self.buffer, self.view, self.format, self.bytes, self.sha256
        )
    }
}
pub(super) fn uploaded(
    resources: &super::raytrace_resources::RaytraceResources,
    token: &crate::raytrace_input_observation::RayInputObservation,
) {
    if resources.hit_scene.is_empty() {
        return;
    }
    if resources.hit_scene.len() != 10 {
        token.refuse("incomplete actual hit scene uploads");
        return;
    }
    eprintln!("{}", scene_marker(resources, token));
}
fn scene_marker(
    resources: &super::raytrace_resources::RaytraceResources,
    token: &crate::raytrace_input_observation::RayInputObservation,
) -> String {
    let rows = texel_rows(&resources.hit_scene);
    format!(
        "{{\"kind\":\"ray-hit-scene-upload\",\"authority\":\"successful-API-upload-inputs\",\"gpu_readback\":false,\"view_address\":{},\"view_id\":{},\"frame_generation\":{},\"scene_set\":{},\"uniform_buffer\":{},\"uniform_sha256\":\"{}\",\"texels\":[{rows}]}}",
        token.owner,
        token.view_id,
        token.generation,
        resources.set.as_raw(),
        resources.uniform_handle.as_raw(),
        hit_sha256(&resources.uniform_bytes)
    )
}

fn texel_rows(rows: &[HitTexelUpload]) -> String {
    rows.iter()
        .enumerate()
        .map(|(index, row)| row.json(index + 1))
        .collect::<Vec<_>>()
        .join(",")
}

fn hit_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hashes_match_independent_sha256_vectors() {
        assert_eq!(
            hit_sha256(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hit_sha256(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
    fn texel_fixture(bytes: &[u8]) -> HitTexelUpload {
        HitTexelUpload::new(
            vk::Buffer::from_raw(17),
            vk::BufferView::from_raw(19),
            vk::Format::R32G32B32_SFLOAT,
            bytes,
        )
    }
    #[test]
    fn texel_rows_bind_owned_bytes_handles_format_and_count() {
        let mut bytes = vec![0; 48];
        let row = texel_fixture(&bytes);
        bytes[0] = 1;
        assert_ne!(row.sha256, hit_sha256(&bytes));
        let json: serde_json::Value = serde_json::from_str(&row.json(5)).unwrap();
        assert_eq!(json["binding"], 5);
        assert_eq!(json["buffer"], 17);
        assert_eq!(json["view"], 19);
        assert_eq!(json["bytes"], 48);
        assert_eq!(json["format"], vk::Format::R32G32B32_SFLOAT.as_raw());
    }
}
