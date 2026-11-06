//! B6's three required inputs; a private directory is an explicit source, never a fallback.
use martensite_tkernel::standard::{Failure, exceptions::program_error};
use std::path::{Path, PathBuf};

const FONT_NAMES: [&str; 3] = ["OLFTestFont-Regular.ttf", "machtgth.ttf", "DejaVuSans.ttf"];
const PRIVATE_INPUT_DIRECTORY: &str = "MARTENSITE_B6_FONT_DATA_DIR";

pub(super) struct B6FontPaths {
    pub olf: PathBuf,
    pub machine_tool: PathBuf,
    pub sans: PathBuf,
}

trait B6InputFiles {
    fn locate(&self, name: &str) -> Result<PathBuf, Failure>;
    fn is_regular_file(&self, path: &Path) -> bool;
}

struct LocalB6InputFiles;
impl B6InputFiles for LocalB6InputFiles {
    fn locate(&self, name: &str) -> Result<PathBuf, Failure> {
        if name == "DejaVuSans.ttf" {
            return Ok(PathBuf::from(
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ));
        }
        crate::dataset_shape::locate_dataset_file(name)
    }

    fn is_regular_file(&self, path: &Path) -> bool {
        path.is_file()
    }
}

impl B6FontPaths {
    /// Resolve all inputs before opening a font; `let paths = B6FontPaths::resolve()?;`.
    pub fn resolve() -> Result<Self, Failure> {
        let directory = std::env::var_os(PRIVATE_INPUT_DIRECTORY).map(PathBuf::from);
        resolve_b6_inputs(directory.as_deref(), &LocalB6InputFiles)
    }
}

fn resolve_b6_inputs(
    directory: Option<&Path>,
    files: &impl B6InputFiles,
) -> Result<B6FontPaths, Failure> {
    let [olf_name, machine_name, sans_name] = FONT_NAMES;
    Ok(B6FontPaths {
        olf: required_font_path(olf_name, directory, files)?,
        machine_tool: required_font_path(machine_name, directory, files)?,
        sans: required_font_path(sans_name, directory, files)?,
    })
}

fn required_font_path(
    name: &str,
    directory: Option<&Path>,
    files: &impl B6InputFiles,
) -> Result<PathBuf, Failure> {
    let path = match directory {
        Some(root) => root.join(name),
        None => files.locate(name)?,
    };
    if path.file_name().is_none_or(|actual| actual != name) || !files.is_regular_file(&path) {
        return Err(program_error(&format!(
            "B6 font input {path:?}; expected regular file named {name:?} without substitution",
        )));
    }
    Ok(path)
}

#[cfg(test)]
#[path = "single_line_input_tests.rs"]
mod tests;
