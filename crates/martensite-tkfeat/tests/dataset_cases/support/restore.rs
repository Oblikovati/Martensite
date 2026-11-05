//! `restore [locate_data_file file] name`: finds an input of the test data submodule and binds
//! the shape it holds. A file that is not there is the script error the reference lookup raises,
//! which the parse rules grade as a skip and the test then fails: a case never passes without
//! its input. The submodule's pinned commit identifies the bytes.
use super::{script_error::script_error, session::Session};
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::path::{Path, PathBuf};

/// `file KIND` per input: what a reference read of the file calls its root shape. Written by
/// `the case generator`; each case's script says nothing of it.
const RESTORED_KINDS: &str = include_str!("restored_kinds.txt");

/// The first word of a file saved from a drawable, before its plain shape text.
const DRAWABLE_HEADER: &[u8] = b"DBRep_DrawableShape";
/// The folders of the submodule's dataset that hold inputs.
const DATA_FOLDERS: [&str; 7] = ["brep", "geom", "iges", "msv", "others", "step", "xbf"];

/// The submodule: `$MARTENSITE_TEST_DATA_DIR` when set, else `external/martensite-test-data`.
fn data_root() -> PathBuf {
    if let Some(directory) = std::env::var_os("MARTENSITE_TEST_DATA_DIR") {
        return PathBuf::from(directory);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../external/martensite-test-data")
}

fn located_in(root: &Path, file: &str) -> Option<PathBuf> {
    DATA_FOLDERS
        .iter()
        .map(|folder| root.join("dataset-7.9.0").join(folder).join(file))
        .find(|path| path.is_file())
}

/// The shape text of a file, with or without the drawable header line.
fn shape_text(bytes: &[u8]) -> &[u8] {
    bytes.strip_prefix(DRAWABLE_HEADER).unwrap_or(bytes)
}

fn read_shape(file: &str, bytes: &[u8]) -> Shape {
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(shape_text(bytes).to_vec());
    let builder = BRepBuilder::default();
    brep_tools::read(&mut shape, &mut stream, &builder, &ProgressRange::default())
        .unwrap_or_else(|failure| panic!("restore {file}: expected a shape: {failure:?}"));
    shape
}

/// The bytes of an input of the submodule; one that is not there is the script error of the
/// reference lookup. Example: `input_bytes("so1")`.
pub fn input_bytes(file: &str) -> Vec<u8> {
    let Some(path) = located_in(&data_root(), file) else {
        script_error(&format!("File {file} could not be found"));
    };
    std::fs::read(&path).unwrap_or_else(|cause| panic!("{path:?}: {cause}"))
}

/// `restore [locate_data_file file] name`; `restore(s, "so1", "b1")`.
pub fn restore(session: &mut dyn Session, file: &str, name: &str) {
    let bytes = input_bytes(file);
    let shape = read_shape(file, &bytes);
    let kind = shape
        .shape_type()
        .unwrap_or_else(|failure| panic!("restore {file}: {failure:?}"));
    assert_eq!(
        kind_word(kind),
        recorded_kind(file),
        "restore {file}: root kind"
    );
    session.bind(name, &shape);
}

fn recorded_kind(file: &str) -> &'static str {
    RESTORED_KINDS
        .lines()
        .filter_map(|row| row.split_once(' '))
        .find(|(name, _)| *name == file)
        .map(|(_, kind)| kind)
        .unwrap_or_else(|| panic!("restore {file}: expected a row in restored_kinds.txt"))
}

fn kind_word(kind: ShapeEnum) -> &'static str {
    match kind {
        ShapeEnum::Compound => "COMPOUND",
        ShapeEnum::CompSolid => "COMPSOLID",
        ShapeEnum::Solid => "SOLID",
        ShapeEnum::Shell => "SHELL",
        ShapeEnum::Face => "FACE",
        ShapeEnum::Wire => "WIRE",
        ShapeEnum::Edge => "EDGE",
        ShapeEnum::Vertex => "VERTEX",
        ShapeEnum::Shape => "SHAPE",
    }
}

#[test]
fn the_drawable_header_is_optional() {
    assert_eq!(shape_text(b"DBRep_DrawableShape\nCASCADE"), b"\nCASCADE");
    assert_eq!(shape_text(b"CASCADE"), b"CASCADE");
}

#[test]
fn a_missing_input_is_found_nowhere() {
    assert_eq!(located_in(&data_root(), "no_such_input.brep"), None);
    assert!(located_in(&data_root(), "so1").is_some());
}
