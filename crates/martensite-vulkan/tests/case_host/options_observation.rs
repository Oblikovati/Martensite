//! Explicit outputs refuse whole-view geometry claims where only a last pass was observed.
use super::capture_witness::{CaseWitnessDirectory, DumpWitnessWriter};
use super::*;
use martensite_tkservice::graphic3d::Graphic3dBufferType;
use martensite_tkv3d::v3d::V3dImageDumpOptions;
use std::{ffi::OsStr, io};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IncompleteObservation {
    TiledLastPassOnly,
    ShadowMapProjection,
}

impl SessionRun {
    pub(super) fn dump_observation(
        &mut self,
        view: &Graphic3dRenderView,
        record: &SubmittedDraws,
        basename: &'static str,
        options: Option<&V3dImageDumpOptions>,
    ) -> Result<(), Box<dyn Error>> {
        let size = observation_extent(self.session().source_window_size(), options);
        let Some(reason) = incomplete_reason(record.observed_tile(), options) else {
            return self.dump_witnesses(view, record, basename, size);
        };
        let result = unavailable_observation(
            reason,
            size,
            std::env::var_os(capture_witness::WITNESS_FLAG).as_deref(),
            &mut UnavailableDirectory { basename },
        );
        self.retain_observation(result.map_err(Into::into));
        Ok(())
    }
}

fn incomplete_reason(
    tile: bool,
    options: Option<&V3dImageDumpOptions>,
) -> Option<IncompleteObservation> {
    if tile {
        return Some(IncompleteObservation::TiledLastPassOnly);
    }
    if options.is_some_and(|p| p.buffer_type == Graphic3dBufferType::ShadowMap) {
        return Some(IncompleteObservation::ShadowMapProjection);
    }
    None
}

fn unavailable_observation(
    reason: IncompleteObservation,
    size: (i32, i32),
    flag: Option<&OsStr>,
    writer: &mut impl UnavailableDumpWriter,
) -> io::Result<()> {
    let requested = capture_witness::witness_requested(flag)?;
    writer.require_no_ties()?;
    let text = unavailable_text(reason, size);
    writer.write_piece("unavailable.json", text.as_bytes())?;
    require_complete_observation(requested, reason, size)
}

fn unavailable_text(reason: IncompleteObservation, size: (i32, i32)) -> String {
    format!(
        "{{\"status\":\"unavailable\",\"measurement_kind\":\"unknown\",\"reason\":\"{reason:?}\",\"requested_extent\":[{},{}],\"whole_frame_geometry\":false,\"reference_eligible\":false}}\n",
        size.0, size.1
    )
}

fn require_complete_observation(
    requested: bool,
    reason: IncompleteObservation,
    size: (i32, i32),
) -> io::Result<()> {
    if requested {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            format!(
                "dump observation={reason:?}, extent={size:?}; expected a complete non-tiled same-projection frame witness"
            ),
        ));
    }
    Ok(())
}

pub(super) fn observation_extent(
    window: (i32, i32),
    options: Option<&V3dImageDumpOptions>,
) -> (i32, i32) {
    options.map_or(window, |p| (p.width, p.height))
}

#[cfg(test)]
#[path = "options_observation_controls.rs"]
mod controls;

trait UnavailableDumpWriter: DumpWitnessWriter {
    fn require_no_ties(&self) -> io::Result<()>;
}

struct UnavailableDirectory<'name> {
    basename: &'name str,
}
impl DumpWitnessWriter for UnavailableDirectory<'_> {
    fn write_piece(&mut self, name: &str, bytes: &[u8]) -> io::Result<()> {
        CaseWitnessDirectory::new(self.basename).write_piece(name, bytes)
    }
}
impl UnavailableDumpWriter for UnavailableDirectory<'_> {
    fn require_no_ties(&self) -> io::Result<()> {
        let path = source_output::capture_path(&format!("{}.ties.json", self.basename))
            .map_err(|error| io::Error::other(error.to_string()))?;
        match std::fs::symlink_metadata(&path) {
            Ok(_) => Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "tie path={path:?}; expected no geometry artifact for unavailable observation"
                ),
            )),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }
}
