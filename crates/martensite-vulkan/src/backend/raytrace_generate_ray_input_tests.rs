//! Independent std140 identity-camera bytes and named read-only input fake.
use super::*;
use std::collections::BTreeMap;
#[derive(Clone)]
struct NamedRayInputReader {
    files: BTreeMap<String, Vec<u8>>,
}
impl RayInputReader for NamedRayInputReader {
    fn read(&self, name: &str) -> io::Result<Vec<u8>> {
        self.files.get(name).cloned().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("fake input={name}; expected fixture file"),
            )
        })
    }
}
fn vector_bytes(bytes: &mut [u8], offset: usize, vector: [f32; 3]) {
    for (index, value) in vector.into_iter().enumerate() {
        bytes[offset + index * 4..offset + index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
}
fn identity_uniform_fixture() -> Vec<u8> {
    let mut bytes = vec![0; 416];
    for (offset, point) in [
        (176, [-1., 1., -1.]),
        (192, [-1., -1., -1.]),
        (208, [1., 1., -1.]),
        (224, [1., -1., -1.]),
    ] {
        vector_bytes(&mut bytes, offset, point);
    }
    for offset in [240, 256, 272, 288] {
        vector_bytes(&mut bytes, offset, [0., 0., 2.]);
    }
    for offset in [188, 204] {
        bytes[offset..offset + 4].copy_from_slice(&409i32.to_le_bytes());
    }
    bytes
}
fn reader_fixture() -> NamedRayInputReader {
    let uniforms = identity_uniform_fixture();
    let constants: Vec<u8> = [10u32, 3, 0, 0, 0, 0, 0, 0, 0, 0]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    let rows: Vec<_> = ["bug28890_1.png","bug28890_2.png"].map(|image| serde_json::json!({"image":image,"uniform_sha256":digest(&uniforms),"specialization_sha256":digest(&constants)})).into();
    let manifest = serde_json::json!({"schema":"martensite-generate-ray-inputs-v1","qualification":false,"rows":rows});
    let mut files = BTreeMap::from([
        ("inputs.json".into(), serde_json::to_vec(&manifest).unwrap()),
        ("specialization.bin".into(), constants),
    ]);
    for image in ["bug28890_1.png", "bug28890_2.png"] {
        files.insert(format!("{image}.uniforms.bin"), uniforms.clone());
    }
    NamedRayInputReader { files }
}
fn load_fake(reader: &NamedRayInputReader) -> io::Result<Vec<RayProbeInput>> {
    load_inputs(reader, &digest(&reader.files["inputs.json"]))
}
#[test]
fn actual_bytes_and_labels_are_owned_and_do_not_alias_the_reader() {
    let mut reader = reader_fixture();
    let loaded = load_fake(&reader).unwrap();
    reader.files.get_mut("bug28890_1.png.uniforms.bin").unwrap()[192] ^= 1;
    assert_eq!(loaded[0].image, "bug28890_1.png");
    assert_eq!(loaded[0].uniforms, identity_uniform_fixture());
    assert_eq!(loaded[1].constants, [10, 3, 0, 0, 0, 0, 0, 0, 0, 0]);
}
#[test]
fn manifest_digest_missing_file_and_changed_payload_refuse() {
    let mut reader = reader_fixture();
    assert!(load_inputs(&reader, &"0".repeat(64)).is_err());
    reader.files.get_mut("bug28890_1.png.uniforms.bin").unwrap()[0] ^= 1;
    assert!(load_fake(&reader).is_err());
    reader.files.remove("specialization.bin");
    assert!(load_fake(&reader).is_err());
}
fn replace_manifest(reader: &mut NamedRayInputReader, change: impl FnOnce(&mut serde_json::Value)) {
    let mut value: serde_json::Value =
        serde_json::from_slice(&reader.files["inputs.json"]).unwrap();
    change(&mut value);
    reader
        .files
        .insert("inputs.json".into(), serde_json::to_vec(&value).unwrap());
}
#[test]
fn ambiguous_labels_and_qualification_claims_refuse_even_with_rehashed_manifest() {
    let mut reader = reader_fixture();
    replace_manifest(&mut reader, |v| {
        v["rows"][1]["image"] = "bug28890_1.png".into()
    });
    assert!(load_fake(&reader).is_err());
    let mut reader = reader_fixture();
    replace_manifest(&mut reader, |v| v["qualification"] = true.into());
    assert!(load_fake(&reader).is_err());
}
#[test]
fn invalid_window_nonfinite_or_zero_direction_refuse() {
    let original = identity_uniform_fixture();
    for (offset, value) in [(188, 408u32), (176, f32::INFINITY.to_bits()), (248, 0)] {
        let mut bytes = original.clone();
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        assert!(validate_uniforms(&bytes).is_err());
    }
}
#[test]
fn actual_specialization_requires_its_own_hash_and_non_dof_contract() {
    let mut reader = reader_fixture();
    reader.files.get_mut("specialization.bin").unwrap()[32] = 1;
    let sha = digest(&reader.files["specialization.bin"]);
    replace_manifest(&mut reader, |v| {
        for row in v["rows"].as_array_mut().unwrap() {
            row["specialization_sha256"] = sha.clone().into();
        }
    });
    assert!(load_fake(&reader).is_err());
}
#[test]
fn digest_matches_independent_known_sha256_vectors() {
    assert_eq!(
        digest(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        digest(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}
