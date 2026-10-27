//! Both archive encodings exercise the same existing file and stream entry points.
use super::fakes::{RecordingConsole, written};
use martensite_tkbrep::{bintools, brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, osd::FileSystem, standard::Failure};
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub(super) enum ArchiveFormat {
    Text,
    Binary,
}
impl ArchiveFormat {
    /// Select a literal record name; `ArchiveFormat::Text.name()` labels an observation.
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Binary => "binary",
        }
    }
    /// Write through the existing file API; `format.write_file(&files, &shape)`.
    pub(super) fn write_file(self, files: &dyn FileSystem, shape: &Shape) -> Result<bool, Failure> {
        match self {
            Self::Text => text_file(files, shape),
            Self::Binary => binary_file(files, shape),
        }
    }
    /// Restore through the existing file API; `format.read_file(&files, &mut shape, console)`.
    pub(super) fn read_file(
        self,
        files: &dyn FileSystem,
        shape: &mut Shape,
        console: Arc<RecordingConsole>,
    ) -> Result<bool, Failure> {
        match self {
            Self::Text => read_text(files, shape, console),
            Self::Binary => read_binary(files, shape, console),
        }
    }
    /// Build an independent stream-entry byte reference; `format.stream_record(&shape)`.
    pub(super) fn stream_record(self, shape: &Shape) -> Vec<u8> {
        match self {
            Self::Text => text_record(shape),
            Self::Binary => written(|stream| {
                bintools::write(
                    shape,
                    stream,
                    bintools::WriteOptions::default(),
                    &ProgressRange::default(),
                )
                .unwrap();
            }),
        }
    }
}
fn text_file(files: &dyn FileSystem, shape: &Shape) -> Result<bool, Failure> {
    let result: Result<bool, Failure> = brep_tools::write_file_on(
        files,
        shape,
        &"owned.shape".into(),
        brep_tools::BRepWriteOptions::default(),
        &ProgressRange::default(),
    );
    result
}
fn binary_file(files: &dyn FileSystem, shape: &Shape) -> Result<bool, Failure> {
    let result: Result<bool, Failure> = bintools::write_file_on(
        files,
        shape,
        &"owned.shape".into(),
        bintools::WriteOptions::default(),
        &ProgressRange::default(),
    );
    result
}
fn read_text(
    files: &dyn FileSystem,
    shape: &mut Shape,
    console: Arc<RecordingConsole>,
) -> Result<bool, Failure> {
    let result: Result<bool, Failure> = brep_tools::read_file_on(
        files,
        shape,
        &"owned.shape".into(),
        &BRepBuilder::default(),
        &ProgressRange::default(),
        console,
    );
    result
}
fn read_binary(
    files: &dyn FileSystem,
    shape: &mut Shape,
    console: Arc<RecordingConsole>,
) -> Result<bool, Failure> {
    let result: Result<bool, Failure> = bintools::read_file_on(
        files,
        shape,
        &"owned.shape".into(),
        &ProgressRange::default(),
        console,
    );
    result
}
fn text_record(shape: &Shape) -> Vec<u8> {
    let mut bytes = b"DBRep_DrawableShape\n".to_vec();
    bytes.extend(written(|stream| {
        brep_tools::write(shape, stream, &ProgressRange::default()).unwrap()
    }));
    bytes
}
