//! A written AP203 file carries constant identities, not the account or the machine that wrote it:
//! a deliberate difference from the library, see `step_construct/ap203_defaults.rs`.
use martensite_tkbrep::topods::Shape;
use martensite_tkdestep::de_step::{
    DeStepParameters, DeStepWriteModeStepSchema, StepControlStepModelType,
};
use martensite_tkdestep::step_control::{StepControlReader, StepControlWriter};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::osd::Process;
use martensite_tkxsbase::xs_control::XSControlReaderOps;
use std::path::PathBuf;

/// A shape of the dataset.
fn a_shape() -> Shape {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/dataset-7.9.0/step/brep_wv1.stp");
    let mut reader = StepControlReader::new().expect("reader");
    reader
        .read_file(path.to_str().expect("path"))
        .expect("read");
    reader
        .transfer_roots(&ProgressRange::default())
        .expect("transfer");
    reader.xs_reader().one_shape()
}

/// The name of the host, as the file behind the call has it; empty when unknown.
fn host_name() -> String {
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|name| name.trim().to_owned())
        .unwrap_or_default()
}

#[test]
fn a_written_ap203_file_names_neither_the_user_nor_the_machine() {
    let shape = a_shape();
    let writer = StepControlWriter::new().expect("writer");
    let mut parameters = DeStepParameters::default();
    parameters.write_schema = DeStepWriteModeStepSchema::AP203;
    writer
        .transfer_with(
            &shape,
            StepControlStepModelType::AS_IS,
            &parameters,
            true,
            &ProgressRange::default(),
        )
        .expect("transfer");
    let output = std::env::temp_dir().join(format!(
        "martensite-host-identity-{}.stp",
        std::process::id()
    ));
    writer.write(output.to_str().expect("path")).expect("write");
    let text = std::fs::read_to_string(&output).expect("written");
    std::fs::remove_file(&output).expect("remove");
    assert!(
        text.contains("PERSON('Martensite','','Martensite'"),
        "{text}"
    );
    assert!(
        text.contains("ORGANIZATION('Martensite','Martensite',''"),
        "{text}"
    );
    let zone_at_utc = "COORDINATED_UNIVERSAL_TIME_OFFSET(0,$,.EXACT.)";
    assert!(text.contains(zone_at_utc), "{text}");
    let login = String::from_utf8_lossy(Process::new().user_name().as_bytes()).into_owned();
    for private in [login, host_name()] {
        if !private.is_empty() {
            assert!(
                !text.contains(&format!("'{private}'")),
                "{private} is in the file"
            );
            assert!(
                !text.contains(&format!(",{private}'")),
                "{private} is in the file"
            );
        }
    }
}
