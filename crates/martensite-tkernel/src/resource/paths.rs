//! The path steps a resource manager uses to name `<directory>/<resource name>`.
use crate::{osd::CadPath, tcollection::AsciiString};
use std::path::{Path, PathBuf};

/// Parses `dir`, descends into its name and extension, sets the name, clears the
/// extension and spells the system name: the directory keeps its path spelling and the resource
/// name becomes the file name verbatim.
pub(super) fn resource_file(directory: &[u8], name: &str) -> AsciiString {
    let parsed = CadPath::new(directory);
    let mut system = parsed.system_name().as_bytes().to_vec();
    if parsed.name().is_empty() {
        // Without a name there is no DownTrek, and SetExtension("") drops a lone extension
        // such as the one a path reads from "." or ".hidden".
        system.truncate(system.len() - parsed.extension().len());
    }
    // A trek always ends with '/' in the system spelling, as DownTrek makes it.
    if !system.is_empty() && system.last() != Some(&b'/') {
        system.push(b'/');
    }
    system.extend_from_slice(name.split('\0').next().unwrap_or_default().as_bytes());
    AsciiString::from_raw(&system)
}
/// Bytes of a host path, as the environment returned it.
pub(super) fn path_bytes(path: &Path) -> Vec<u8> {
    path.to_string_lossy().as_bytes().to_vec()
}
/// The host path spelled by an OSD system name.
pub(super) fn host_path(path: &AsciiString) -> PathBuf {
    PathBuf::from(String::from_utf8_lossy(path.as_bytes()).into_owned())
}
