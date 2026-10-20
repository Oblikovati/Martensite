//! Exact source-embedded shape records from bug30269 and optimal_bndbox_exception.
#[path = "common/geometry_io.rs"]
mod geometry_io;
use geometry_io::{MemoryFileSystem, RecordingConsole};
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, tcollection::AsciiString};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::bnd::{BoundingBox3, OrientedBoundingBox};
use martensite_tktopalgo::{brep_bnd_lib::BRepBndLib, brep_gprop::BRepGProp};
use std::sync::Arc;

fn restored_inline_shape(name: &str, bytes: &[u8]) -> Shape {
    let files = MemoryFileSystem::default();
    files.insert(name, bytes.to_vec());
    // The temporary input owner dies before operations, matching deletion after restore.
    read_original_file(&files, &AsciiString::new(name))
}

fn read_original_file(files: &MemoryFileSystem, name: &AsciiString) -> Shape {
    let mut shape = Shape::default();
    let builder = BRepBuilder::default();
    let range = ProgressRange::default();
    let console = Arc::new(RecordingConsole::default());
    assert!(
        brep_tools::read_file_on(files, &mut shape, name, &builder, &range, console).unwrap(),
        "input {:?}: expected original shape record to restore",
        name.as_bytes()
    );
    shape
}

fn execute_original_property_queries(properties: &GProps) {
    let _mass = properties.mass();
    let _center = properties.centre_of_mass();
    let _matrix = properties.matrix_of_inertia();
    let _moments = properties.principal_properties().unwrap().moments();
}

fn execute_original_bounds(shape: &Shape) {
    BRepBndLib::add(shape, &mut BoundingBox3::default(), true).unwrap();
    BRepBndLib::add_optimal(shape, &mut BoundingBox3::default(), true, false).unwrap();
    BRepBndLib::add_obb(
        shape,
        &mut OrientedBoundingBox::default(),
        true,
        false,
        false,
    )
    .unwrap();
}

#[test]
fn missing_pcurves_face_supports_all_original_bounds_and_properties() {
    let bytes = include_bytes!("fixtures/embedded_shapes/bug30269-inline.brep");
    let shape = restored_inline_shape("bug30269.brep", bytes);
    execute_original_bounds(&shape);
    let mut surface = GProps::default();
    BRepGProp::surface_properties(&shape, &mut surface, false, false).unwrap();
    execute_original_property_queries(&surface);
    let mut volume = GProps::default();
    BRepGProp::volume_properties(&shape, &mut volume, false, false, false).unwrap();
    execute_original_property_queries(&volume);
}

#[test]
fn original_high_tolerance_spline_shape_supports_optimal_bounds() {
    let bytes = include_bytes!("fixtures/embedded_shapes/optimal-bounds-inline.brep");
    let shape = restored_inline_shape("optimal_bndbox_exception.brep", bytes);
    BRepBndLib::add_optimal(&shape, &mut BoundingBox3::default(), true, false).unwrap();
}
