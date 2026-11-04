//! Optional owned frame observations do not reinterpret geometry as a coverage proof.
use crate::tie_edges::SubmittedDraws;
use martensite_vulkan::{FrameCapture, Graphic3dRenderView, PrimitiveDraw};
use std::{ffi::OsStr, io, io::Write};

#[path = "capture_witness/camera_state.rs"]
mod camera_state;
#[cfg(test)]
#[path = "capture_witness/controls.rs"]
mod controls;
#[path = "capture_witness/encoding.rs"]
mod encoding;
#[path = "capture_witness/sequence_storage.rs"]
mod sequence_storage;

#[cfg(feature = "command-state-diagnostics")]
#[path = "capture_witness/publication.rs"]
mod publication;
#[cfg(feature = "command-state-diagnostics")]
pub(super) use publication::DumpScope;

pub(super) const WITNESS_FLAG: &str = "MARTENSITE_CASE_CAPTURE_WITNESS";

/// Keep original END and its failure priority; after_end(source, pending_observer_error).
pub(super) fn after_end(
    source: Result<(), Box<dyn std::error::Error>>,
    pending: Option<io::Error>,
) -> Result<(), Box<dyn std::error::Error>> {
    source?;
    if let Some(error) = pending {
        return Err(error.into());
    }
    Ok(())
}

/// Retain observation failures until original END; `retain_error(&mut pending, result)`.
pub(crate) fn retain_error(
    pending: &mut Option<io::Error>,
    observation: Result<(), Box<dyn std::error::Error>>,
) {
    let Err(error) = observation else { return };
    let message = match pending.take() {
        Some(previous) => format!("{previous}; additional observation=[{error}]"),
        None => format!("observation=[{error}]"),
    };
    *pending = Some(io::Error::other(message));
}

struct OwnedDumpSnapshot {
    capture: FrameCapture,
    draws: Vec<PrimitiveDraw>,
    camera: camera_state::CameraStateSnapshot,
    #[cfg(feature = "raytrace-input-diagnostics")]
    raytrace_inputs: Option<String>,
}

trait DumpSnapshotSource {
    fn read_snapshot(&mut self) -> io::Result<OwnedDumpSnapshot>;
}

pub(super) trait DumpWitnessWriter {
    fn write_piece(&mut self, name: &str, bytes: &[u8]) -> io::Result<()>;
}

struct AcceptedFrameSource<'a> {
    view: &'a Graphic3dRenderView,
    record: &'a SubmittedDraws,
    #[cfg(feature = "command-state-diagnostics")]
    basename: &'a str,
}

impl DumpSnapshotSource for AcceptedFrameSource<'_> {
    fn read_snapshot(&mut self) -> io::Result<OwnedDumpSnapshot> {
        #[cfg(feature = "command-state-diagnostics")]
        let before = publication::cursor(self.view);
        #[cfg(feature = "raytrace-input-diagnostics")]
        let (capture, raytrace_inputs) =
            self.view
                .capture_with_raytrace_input_witness()
                .map_err(|error| {
                    io::Error::other(format!(
                        "raytrace witness={error}; expected uniquely bound accepted inputs"
                    ))
                })?;
        #[cfg(not(feature = "raytrace-input-diagnostics"))]
        let capture = self.view.capture().map_err(|error| {
            io::Error::other(format!(
                "capture={error:?}; expected the last accepted frame without redraw"
            ))
        })?;
        #[cfg(feature = "command-state-diagnostics")]
        publication::snapshot(self.view, self.basename, &capture, before);
        Ok(OwnedDumpSnapshot {
            capture,
            draws: self.record.draws(),
            camera: camera_state::snapshot(self.view)?,
            #[cfg(feature = "raytrace-input-diagnostics")]
            raytrace_inputs,
        })
    }
}

pub(super) struct CaseWitnessDirectory<'a> {
    basename: &'a str,
}

impl<'a> CaseWitnessDirectory<'a> {
    pub(super) fn new(basename: &'a str) -> Self {
        Self { basename }
    }
}
impl DumpWitnessWriter for CaseWitnessDirectory<'_> {
    fn write_piece(&mut self, name: &str, bytes: &[u8]) -> io::Result<()> {
        let directory = super::source_output::capture_path(&format!("{}.witness", self.basename))
            .map_err(|error| {
            io::Error::other(format!(
                "witness basename={:?}: {error}; expected a process capture directory",
                self.basename
            ))
        })?;
        std::fs::create_dir_all(&directory)
            .map_err(|error| witness_directory_error(&directory, error))?;
        let path = directory.join(name);
        let result = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path);
        let mut file = result.map_err(|error| witness_write_error(&path, error))?;
        file.write_all(bytes)
            .map_err(|error| witness_write_error(&path, error))
    }
}

fn witness_directory_error(path: &std::path::Path, error: io::Error) -> io::Error {
    io::Error::new(
        error.kind(),
        format!("witness directory={path:?}: {error}; expected a writable directory"),
    )
}

fn witness_write_error(path: &std::path::Path, error: io::Error) -> io::Error {
    io::Error::new(
        error.kind(),
        format!("witness path={path:?}: {error}; expected a new writable regular file"),
    )
}

/// Observe actual owned values only when requested; observe(view, record, "case.png").
pub(crate) fn observe(
    view: &Graphic3dRenderView,
    record: &SubmittedDraws,
    basename: &str,
) -> io::Result<()> {
    let flag = std::env::var_os(WITNESS_FLAG);
    write_requested(
        flag.as_deref(),
        &mut AcceptedFrameSource {
            view,
            record,
            #[cfg(feature = "command-state-diagnostics")]
            basename,
        },
        &mut CaseWitnessDirectory { basename },
    )
}

pub(super) fn witness_requested(flag: Option<&OsStr>) -> io::Result<bool> {
    match flag {
        None => Ok(false),
        Some(value) if value == "1" => Ok(true),
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{WITNESS_FLAG}={actual:?}; expected unset or literal 1"),
        )),
    }
}

fn write_requested(
    flag: Option<&OsStr>,
    source: &mut impl DumpSnapshotSource,
    writer: &mut impl DumpWitnessWriter,
) -> io::Result<()> {
    if !witness_requested(flag)? {
        return Ok(());
    }
    for piece in encoding::snapshot_pieces(source.read_snapshot()?)? {
        writer.write_piece(piece.name, &piece.bytes)?;
    }
    Ok(())
}

#[path = "capture_witness/sequence.rs"]
mod sequence;
/// Retain each requested diagnostic snapshot; `observe_ordinal(view, record, "case.png", 1)?`.
pub(super) fn observe_ordinal(
    view: &Graphic3dRenderView,
    record: &SubmittedDraws,
    basename: &str,
    ordinal: usize,
) -> io::Result<()> {
    let flag = std::env::var_os(sequence::SEQUENCE_FLAG);
    sequence::validate_sequence_flag(flag.as_deref())?;
    let flag = std::env::var_os(WITNESS_FLAG);
    write_ordinal_requested(
        flag.as_deref(),
        &mut AcceptedFrameSource {
            view,
            record,
            #[cfg(feature = "command-state-diagnostics")]
            basename,
        },
        basename,
        ordinal,
    )
}

fn write_ordinal_requested(
    flag: Option<&OsStr>,
    source: &mut impl DumpSnapshotSource,
    basename: &str,
    ordinal: usize,
) -> io::Result<()> {
    if !witness_requested(flag)? {
        return Ok(());
    }
    let pieces = encoding::snapshot_pieces(source.read_snapshot()?)?;
    let image_path = super::source_output::capture_path(basename)
        .map_err(|error| io::Error::other(format!("image={basename:?}: {error}")))?;
    let root = super::source_output::capture_path(&format!("{basename}.witness"))
        .map_err(|error| io::Error::other(format!("witness basename={basename:?}: {error}")))?;
    sequence_storage::publish(&root, basename, ordinal, &image_path, &pieces)
}
