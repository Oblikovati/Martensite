//! Failure and publication checks use an injected writer with independently fixed bytes.
use super::{PrivateModel, extended, scratch, serial};
use martensite_tkcdf::{
    cdm::CdmDocument,
    pcdm::{PcdmStorageDriver, PcdmStorageDriverBase, PcdmStoreStatus, PcdmWriter},
};
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    standard::{Failure, Handle, StandardTransient, exceptions},
    tcollection::{AsciiString, ExtendedString},
};
use martensite_tklcaf::{tdf::TdfLabel, tdoc_std::TDocStdApplication};
use martensite_tktobj::object_model::{
    DocumentApplicationApi, DocumentModel, DocumentModelState, ModelObject, ModelObjectBase,
};
use std::io;

#[derive(Clone, Copy)]
enum WriterOutcome {
    Complete,
    Error,
    Rejected,
    MissingOutput,
}

struct FixedDocumentWriter {
    transient: StandardTransient,
    base: PcdmStorageDriverBase,
    outcome: WriterOutcome,
}
impl_transient!(FixedDocumentWriter, "FixedDocumentWriter", dyn PcdmStorageDriver, transient,
    interfaces: [dyn PcdmStorageDriver, dyn PcdmWriter]);
impl PcdmStorageDriver for FixedDocumentWriter {
    fn storage_driver_base(&self) -> &PcdmStorageDriverBase {
        &self.base
    }
}
impl PcdmWriter for FixedDocumentWriter {
    fn write_file(
        &self,
        _document: &Handle<dyn CdmDocument>,
        file: &ExtendedString,
        _range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        if matches!(self.outcome, WriterOutcome::MissingOutput) {
            return Ok(());
        }
        std::fs::write(file.to_utf8(), b"fixed document bytes").unwrap();
        match self.outcome {
            WriterOutcome::Complete | WriterOutcome::MissingOutput => Ok(()),
            WriterOutcome::Error => Err(exceptions::program_error("injected writer failure")),
            WriterOutcome::Rejected => {
                self.set_store_status(PcdmStoreStatus::WriteFailure);
                Ok(())
            }
        }
    }
    fn write_stream(
        &self,
        _document: &Handle<dyn CdmDocument>,
        _stream: &mut dyn io::Write,
        _range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        Err(exceptions::not_implemented("expected file writer"))
    }
}

fn model_with_writer(outcome: WriterOutcome) -> Handle<PrivateModel> {
    let model = PrivateModel::new();
    let writer = Handle::new(FixedDocumentWriter {
        transient: StandardTransient::default(),
        base: PcdmStorageDriverBase::default(),
        outcome,
    });
    model
        .application
        .define_format(
            &AsciiString::from("ObjectModelBinary"),
            &AsciiString::from("Fixed document fixture"),
            &AsciiString::from("object_model"),
            &Handle::default(),
            &writer.cast(),
        )
        .unwrap();
    assert!(model.load(&ExtendedString::default()).unwrap());
    model
}

#[test]
fn partially_written_driver_failures_preserve_existing_and_absent_targets() {
    let _serial = serial();
    let directory = scratch("driver_write_failure");
    for (index, outcome) in [
        WriterOutcome::Error,
        WriterOutcome::Rejected,
        WriterOutcome::MissingOutput,
    ]
    .into_iter()
    .enumerate()
    {
        let model = model_with_writer(outcome);
        assert_failed_writer_targets(&model, &directory, index);
        assert!(model.close().unwrap());
    }
    assert_eq!(std::fs::read_dir(&directory).unwrap().count(), 3);
}

fn assert_failed_writer_targets(
    model: &Handle<PrivateModel>,
    directory: &std::path::Path,
    index: usize,
) {
    for exists in [false, true] {
        let file = directory.join(format!("model-{index}-{exists}.object_model"));
        if exists {
            std::fs::write(&file, b"existing document sentinel").unwrap();
        }
        assert!(!model.save_as(&extended(&file)).unwrap());
        assert_target_unchanged(&file, exists);
        assert!(model.get_file().unwrap().is_null());
    }
}

fn assert_target_unchanged(file: &std::path::Path, exists: bool) {
    if exists {
        assert_eq!(std::fs::read(file).unwrap(), b"existing document sentinel");
    } else {
        assert!(!file.exists());
    }
}

#[test]
fn successful_save_publishes_bytes_and_final_path_then_supports_save_again() {
    let _serial = serial();
    let directory = scratch("successful_file_save");
    let model = model_with_writer(WriterOutcome::Complete);
    let file = directory.join("model.object_model");
    assert!(model.save_as(&extended(&file)).unwrap());
    assert_eq!(std::fs::read(&file).unwrap(), b"fixed document bytes");
    assert_eq!(
        model.get_file().unwrap().string().to_utf8(),
        file.to_str().unwrap()
    );
    std::fs::write(&file, b"old bytes").unwrap();
    assert!(model.save().unwrap());
    assert_eq!(std::fs::read(&file).unwrap(), b"fixed document bytes");
    assert_eq!(std::fs::read_dir(&directory).unwrap().count(), 1);
    assert!(model.close().unwrap());
}

struct RejectingStoreObject {
    transient: StandardTransient,
    base: ModelObjectBase,
}
impl_transient!(RejectingStoreObject, "RejectingStoreObject", dyn ModelObject, transient,
    interfaces: [dyn ModelObject]);
impl ModelObject for RejectingStoreObject {
    fn object_base(&self) -> &ModelObjectBase {
        &self.base
    }
    fn before_storing(&self) -> Result<(), Failure> {
        Err(exceptions::program_error("injected before_storing failure"))
    }
}

fn attach_rejecting_object(model: &Handle<PrivateModel>) -> Handle<RejectingStoreObject> {
    model.open_command().unwrap();
    let label: TdfLabel = model.get_main_partition().unwrap().new_label().unwrap();
    let object = Handle::new(RejectingStoreObject {
        transient: StandardTransient::default(),
        base: ModelObjectBase::new(&label),
    });
    ModelObjectBase::attach(&object.cast(), true).unwrap();
    model.commit_command().unwrap();
    object
}

#[test]
fn preparation_callback_errors_preserve_targets_and_document_modification_mode() {
    let _serial = serial();
    let directory = scratch("preparation_failure");
    let model = model_with_writer(WriterOutcome::Complete);
    let _object = attach_rejecting_object(&model);
    let document = model.get_document().unwrap();
    let mode = document.modification_mode();
    for exists in [false, true] {
        let file = directory.join(format!("model-{exists}.object_model"));
        if exists {
            std::fs::write(&file, b"existing document sentinel").unwrap();
        }
        let failure = model.save_as(&extended(&file)).unwrap_err();
        assert!(failure.is_kind("Standard_ProgramError"));
        assert_target_unchanged(&file, exists);
        assert_eq!(document.modification_mode(), mode);
    }
    assert!(model.close().unwrap());
}

#[cfg(unix)]
#[test]
fn saving_through_a_symbolic_link_preserves_the_link_and_file_permissions() {
    use std::os::unix::fs::PermissionsExt;
    let _serial = serial();
    let (target, link) = symbolic_target();
    let model = model_with_writer(WriterOutcome::Complete);
    assert!(model.save_as(&extended(&link)).unwrap());
    let link_type = std::fs::symlink_metadata(&link).unwrap().file_type();
    assert!(link_type.is_symlink());
    assert_eq!(std::fs::read(&target).unwrap(), b"fixed document bytes");
    assert_eq!(
        std::fs::metadata(&target).unwrap().permissions().mode() & 0o777,
        0o640
    );
    assert_eq!(
        model.get_file().unwrap().string().to_utf8(),
        link.to_str().unwrap()
    );
    assert!(model.close().unwrap());
}

#[cfg(unix)]
fn symbolic_target() -> (std::path::PathBuf, std::path::PathBuf) {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let directory = scratch("save_through_symlink");
    let target = directory.join("target.object_model");
    std::fs::write(&target, b"old bytes").unwrap();
    std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o640)).unwrap();
    let link = directory.join("link.object_model");
    symlink(&target, &link).unwrap();
    (target, link)
}

#[test]
fn direct_application_save_refuses_a_read_only_destination() {
    let _serial = serial();
    let directory = scratch("read_only_application_save");
    let file = directory.join("model.object_model");
    std::fs::write(&file, b"existing document sentinel").unwrap();
    let original = std::fs::metadata(&file).unwrap().permissions();
    let mut read_only = original.clone();
    read_only.set_readonly(true);
    std::fs::set_permissions(&file, read_only).unwrap();
    let model = model_with_writer(WriterOutcome::Complete);
    let status = model
        .application
        .save_document(&model.get_document().unwrap(), &extended(&file));
    std::fs::set_permissions(&file, original).unwrap();
    assert!(!status.unwrap());
    assert_target_unchanged(&file, true);
    assert_eq!(std::fs::read_dir(&directory).unwrap().count(), 1);
    assert!(model.close().unwrap());
}

#[cfg(unix)]
#[test]
fn saving_through_a_dangling_relative_symlink_creates_its_target() {
    use std::os::unix::fs::symlink;
    let _serial = serial();
    let directory = scratch("save_through_dangling_symlink");
    let target = directory.join("new-target.object_model");
    let link = directory.join("link.object_model");
    symlink("new-target.object_model", &link).unwrap();
    let model = model_with_writer(WriterOutcome::Complete);
    assert!(model.save_as(&extended(&link)).unwrap());
    assert_eq!(std::fs::read(&target).unwrap(), b"fixed document bytes");
    let link_type = std::fs::symlink_metadata(&link).unwrap().file_type();
    assert!(link_type.is_symlink());
    assert_eq!(
        model.get_file().unwrap().string().to_utf8(),
        link.to_str().unwrap()
    );
    assert!(model.close().unwrap());
}
