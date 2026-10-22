//! SWscenario_ShapeProcessAPI.cxx `DT_ApplySeq result shape rscfilename [prefix]`
//! (ShapeProcessAPI_ApplySequence), and the `set env(CSF_<name>Defaults) dir` lines the
//! heal grids run before it, with the grids' resource files (tests/heal/data/MDTV and
//! tests/heal/data/data at 3d097a0328e71b826377d4814ab05ec3c3d23871, copied unchanged
//! into tests/draw_dataset_cases/data/heal and pinned by SHA-256 here).
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    message::ProgressRange,
    osd::{EnvironmentSource, NativeEnvironment},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_process_api::ShapeProcessAPIApplySequence;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

/// The heal grids' resource files and their SHA-256.
const HEAL_RESOURCES: [(&str, &str); 2] = [
    (
        "MDTV",
        "e92aaecfec8fc0a118e4ad6bc6ce14a0e8bb1b73d8a0c46c083b7bcf268b1e8f",
    ),
    (
        "data",
        "e465a659f75312a8ddd490d9e8ad4076c057dbccc7c5e327fa542153ae7d05e4",
    ),
];

/// tests/heal/data: the directory the heal grids point `CSF_<name>Defaults` to (the same
/// path from crates/tkshhealing and crates/tkdraw, which include this module).
pub fn heal_data_directory() -> PathBuf {
    let directory = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/dataset_cases/data/heal"
    ));
    for (name, sha256) in HEAL_RESOURCES {
        let bytes = std::fs::read(directory.join(name))
            .unwrap_or_else(|error| panic!("heal resource file {name}: {error}"));
        let digest: String = Sha256::digest(&bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        assert_eq!(digest, sha256, "SHA-256 of heal resource file {name}");
    }
    directory
}

/// `set env(CSF_<name>Defaults) dir`: the resource directory the next ShapeProcess
/// context of `name` loads from; `set_resource_defaults("MDTV", &heal_data_directory())`.
pub fn set_resource_defaults(name: &str, directory: &std::path::Path) {
    let variable = format!("CSF_{name}Defaults");
    NativeEnvironment
        .put(
            variable.as_bytes(),
            directory.as_os_str().as_encoded_bytes(),
        )
        .unwrap_or_else(|code| panic!("resource default {variable:?}: environment status {code}"));
}

/// `DT_ApplySeq result shape rscfilename prefix`: PrepareShape (history map for faces and
/// above) then PrintPreparationResult, whose statistics go to the default messenger.
pub fn dt_apply_seq(shape: &Shape, resource: &str, prefix: &str) -> Shape {
    let mut sequence = ShapeProcessAPIApplySequence::new(resource, prefix)
        .unwrap_or_else(|failure| panic!("DT_ApplySeq: {failure:?}"));
    let result = sequence
        .prepare_shape(shape, true, ShapeEnum::Face, &ProgressRange::default())
        .unwrap_or_else(|failure| panic!("DT_ApplySeq PrepareShape: {failure:?}"));
    sequence
        .print_preparation_result()
        .unwrap_or_else(|failure| panic!("DT_ApplySeq PrintPreparationResult: {failure:?}"));
    result
}
