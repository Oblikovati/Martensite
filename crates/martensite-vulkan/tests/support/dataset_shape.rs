//! Boundary-representation input files of the regression dataset, located by file name.
//!
//! The dataset is the pinned `external/martensite-test-data` submodule: the published test
//! dataset plus the inputs attached to public issues. A file is looked up by its bare name in
//! every folder, as the regressions name their inputs.
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle, RuntimeTyped},
    storage::IoStream,
};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkgeombase::geom_tools::{CurveSet, SurfaceSet};
use std::path::{Path, PathBuf};

/// The header word of a shape file written with its drawable type.
const SHAPE_HEADER: &str = "DBRep_DrawableShape";

/// The dataset root inside the repository; `dataset_root().join("dataset-7.9.0")`.
pub fn dataset_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../external/martensite-test-data")
}

/// The path of dataset file `name`, searched in every dataset folder. A name with folders
/// (`occ/Top.brep`) is looked up relative to each folder, breadth first, the way
/// cases locate their data files.
/// Example: `locate_dataset_file("bug23400.brep")?`.
pub fn locate_dataset_file(name: &str) -> Result<PathBuf, Failure> {
    let root = dataset_root();
    let found = if name.contains('/') {
        find_relative(&root, name)?
    } else {
        find_named(&root, name)?
    };
    found.ok_or_else(|| {
        martensite_tkernel::standard::exceptions::program_error(&format!(
            "dataset file {name:?} not found under {}; expected the initialized test-data submodule",
            root.display()
        ))
    })
}

/// The first folder, breadth first in name order, holding the relative path `name`.
fn find_relative(root: &Path, name: &str) -> Result<Option<PathBuf>, Failure> {
    let mut queue = std::collections::VecDeque::from([root.to_path_buf()]);
    while let Some(folder) = queue.pop_front() {
        if folder.join(name).is_file() {
            return Ok(Some(folder.join(name)));
        }
        let entries = std::fs::read_dir(&folder).map_err(|error| io_failure(&folder, &error))?;
        let mut children: Vec<PathBuf> = entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_dir())
            .collect();
        children.sort();
        queue.extend(children.into_iter().filter(|p| {
            !p.file_name()
                .is_some_and(|f| f.to_string_lossy().starts_with('.'))
        }));
    }
    Ok(None)
}

fn find_named(folder: &Path, name: &str) -> Result<Option<PathBuf>, Failure> {
    let entries = std::fs::read_dir(folder).map_err(|error| io_failure(folder, &error))?;
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    paths.sort(); // Directory order is not portable; search in name order.
    for path in paths {
        if path.is_dir() {
            if let Some(found) = find_named(&path, name)? {
                return Ok(Some(found));
            }
        } else if path.file_name().is_some_and(|file| file == name) {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

/// Read the shape of dataset file `name`, with or without its drawable-type header.
/// Example: `let shape = read_dataset_shape("OCC22786-c.brep")?;`.
pub fn read_dataset_shape(name: &str) -> Result<Shape, Failure> {
    let path = locate_dataset_file(name)?;
    let bytes = std::fs::read(&path).map_err(|error| io_failure(&path, &error))?;
    read_shape_bytes(&bytes)
}

/// Read the surface of dataset geometry file `name`: a drawable-type header line
/// (`GeometrySurface`, `GeometryBSplineSurface`, ...) and one surface record in the
/// compact geometry text format. Example: `let s = read_dataset_surface("s.rle")?;`.
pub fn read_dataset_surface(name: &str) -> Result<Handle<dyn GeomSurface>, Failure> {
    read_geometry_record(name, SurfaceSet::read_surface)
}

/// Read the curve of dataset geometry file `name` (`GeometryCurve`,
/// `GeometryBSplineCurve`, ...). Example: `let c = read_dataset_curve("path.brep")?;`.
pub fn read_dataset_curve(name: &str) -> Result<Handle<dyn GeomCurve>, Failure> {
    read_geometry_record(name, CurveSet::read_curve)
}

fn read_geometry_record<T: ?Sized + RuntimeTyped>(
    name: &str,
    read: fn(&mut IoStream) -> Handle<T>,
) -> Result<Handle<T>, Failure> {
    let path = locate_dataset_file(name)?;
    let bytes = std::fs::read(&path).map_err(|error| io_failure(&path, &error))?;
    let record = geometry_record(&bytes).ok_or_else(|| {
        Failure::new(&format!(
            "geometry file {} has no header line; expected `DrawTrSurf_<type>` and a record",
            path.display()
        ))
    })?;
    let geometry = read(&mut IoStream::from_bytes(record.to_vec()));
    if geometry.is_null() {
        return Err(Failure::new(&format!(
            "geometry file {} holds no readable record of the requested kind",
            path.display()
        )));
    }
    Ok(geometry)
}

/// The bytes after a `geometry file ` header line, or `None` without that header.
fn geometry_record(bytes: &[u8]) -> Option<&[u8]> {
    let rest = bytes.strip_prefix(b"DrawTrSurf_")?;
    let line_end = rest.iter().position(|b| *b == b'\n')?;
    Some(&rest[line_end + 1..])
}

/// Read a shape from file bytes, skipping a leading drawable-type header line.
/// Example: `read_shape_bytes(b"TopologyDrawableShape\n...")?`.
pub fn read_shape_bytes(bytes: &[u8]) -> Result<Shape, Failure> {
    let body = strip_header(bytes);
    let mut stream = IoStream::from_bytes(body.to_vec());
    let mut shape = Shape::default();
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )?;
    Ok(shape)
}

fn strip_header(bytes: &[u8]) -> &[u8] {
    let text_start = bytes
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(0);
    let rest = &bytes[text_start..];
    match rest.strip_prefix(SHAPE_HEADER.as_bytes()) {
        Some(after) => after,
        None => bytes,
    }
}

fn io_failure(path: &Path, error: &std::io::Error) -> Failure {
    martensite_tkernel::standard::exceptions::program_error(&format!(
        "dataset path {}: {error}; expected a readable dataset entry",
        path.display()
    ))
}

/// Points the predefined texture lookup at the pinned dataset; first line of a case body.
pub(crate) fn select_predefined_textures() {
    let folder = dataset_root().join("occt-resource-textures");
    #[allow(unsafe_code)]
    // SAFETY: a case body runs alone in its worker process, and this runs before the body starts
    // the renderer, so no other thread reads the environment.
    unsafe {
        std::env::set_var("CSF_MDTVTexturesDirectory", folder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_header_skips_only_the_drawable_type_word() {
        assert_eq!(strip_header(b"DBRep_DrawableShape\nCASCADE"), b"\nCASCADE");
        assert_eq!(strip_header(b"CASCADE Topology"), b"CASCADE Topology");
    }

    #[test]
    fn geometry_record_follows_only_a_drawable_geometry_header() {
        assert_eq!(
            geometry_record(b"DrawTrSurf_Surface\n5 1 2"),
            Some(&b"5 1 2"[..])
        );
        assert_eq!(geometry_record(b"DBRep_DrawableShape\n5"), None);
    }

    #[test]
    fn relative_names_are_found_below_a_dataset_folder() {
        let path = locate_dataset_file("occ/Top.brep").unwrap();
        assert!(
            path.ends_with("occt-source-data/occ/Top.brep"),
            "found {}",
            path.display()
        );
        assert!(locate_dataset_file("occ/no-such-input.brep").is_err());
    }

    #[test]
    fn missing_dataset_file_names_the_file() {
        let failure = locate_dataset_file("no-such-input.brep").unwrap_err();
        assert!(format!("{failure:?}").contains("no-such-input.brep"));
    }
}
