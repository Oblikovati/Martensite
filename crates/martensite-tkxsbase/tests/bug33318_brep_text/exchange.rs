//! The case's configuration, read and write, on the BREP file operations a shape file provider calls.
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::{self, BRepWriteOptions},
    topods::Shape,
    toptools::format_version,
};
use martensite_tkde::de::DeConfigurationContext;
use martensite_tkernel::{message::ProgressRange, tcollection::AsciiString};
use std::path::Path;

/// The literal `-conf` value of the case's `writefile`.
const WRITE_CONFIGURATION: &str = "provider.BREP.Martensite.write.binary : 0";
const BREP_SCOPE: &str = "provider.BREP.Martensite";
const ASCII_HEADER: &str = "DBRep_DrawableShape";

/// Whether a configuration asks for binary output, default on; `writes_binary("a : 1")`.
fn writes_binary(configuration: &str) -> bool {
    let context = DeConfigurationContext::new();
    let loaded = context.load(&AsciiString::new(configuration)).unwrap();
    assert!(
        loaded,
        "{configuration:?}: expected a loadable configuration"
    );
    context.boolean_val("write.binary", true, BREP_SCOPE)
}

/// Read the ASCII file a text header selects, with its header kept; `read(path, text)`.
pub(super) fn read(path: &Path, text: &str) -> Shape {
    assert!(
        text.starts_with(ASCII_HEADER),
        "readfile {path:?}: expected the {ASCII_HEADER} header of the ASCII reader"
    );
    let mut shape = Shape::default();
    let done = brep_tools::read_file(
        &mut shape,
        &file_name(path),
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .unwrap();
    assert!(
        done,
        "readfile {path:?}: expected successful original import"
    );
    shape
}

/// Write the text the configuration selects, triangles and normals on as default; `write(path, &shape)`.
pub(super) fn write(path: &Path, shape: &Shape) {
    assert!(
        !writes_binary(WRITE_CONFIGURATION),
        "original binary=0 configuration must load"
    );
    let options = BRepWriteOptions {
        with_triangles: true,
        with_normals: true,
        version: format_version::CURRENT,
    };
    let done = brep_tools::write_file_with_options(
        shape,
        &file_name(path),
        options,
        &ProgressRange::default(),
    )
    .unwrap();
    assert!(
        done,
        "writefile {path:?}: expected successful original export"
    );
}

fn file_name(path: &Path) -> AsciiString {
    AsciiString::new(
        path.to_str()
            .unwrap_or_else(|| panic!("BREP path {path:?}: expected UTF-8 filename")),
    )
}

#[test]
fn only_the_literal_value_turns_binary_output_off() {
    assert!(!writes_binary(WRITE_CONFIGURATION));
    assert!(writes_binary("provider.BREP.Martensite.write.binary : 1"));
    assert!(writes_binary("provider.STEP.Martensite.write.binary : 0"));
}
