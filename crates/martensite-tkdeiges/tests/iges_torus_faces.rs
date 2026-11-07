//! A torus written as faces, read back and written again. Its four edges meet at one point in space,
//! so only their order on the surface tells them apart.
use martensite_tkbrep::topods::Shape;
use martensite_tkdeiges::iges_control::{IgesControlReader, IgesControlWriter};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeTorus;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxsbase::xs_algo::XSAlgoShapeProcessor;
use martensite_tkxsbase::xs_control::XSControlReaderOps;
use std::sync::{Mutex, MutexGuard, PoisonError};

const RELATIVE_AREA_TOLERANCE: f64 = 1.0e-5;

/// The settings of the format are process-wide.
fn locked() -> MutexGuard<'static, ()> {
    static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
    ONE_AT_A_TIME.lock().unwrap_or_else(PoisonError::into_inner)
}

fn torus() -> Shape {
    let mut maker = BRepPrimAPIMakeTorus::new(5.0, 1.0).expect("torus");
    maker.shape().expect("the shape").clone()
}

fn area(shape: &Shape) -> f64 {
    let mut properties = GProps::new();
    BRepGProp::surface_properties(shape, &mut properties, false, false).expect("the area");
    properties.mass()
}

fn as_faces(shape: &Shape) -> Vec<u8> {
    let mut writer = IgesControlWriter::with_unit(b"MM", 0).expect("the writer");
    assert!(
        writer
            .add_shape(shape, &ProgressRange::default())
            .expect("the translation")
    );
    let mut text = Vec::new();
    assert!(writer.write_stream(&mut text, false).expect("the writing"));
    text
}

fn loaded(text: &[u8]) -> IgesControlReader {
    let mut reader = IgesControlReader::new().expect("the norm");
    let mut bytes = text;
    reader
        .read_stream("written.igs", &mut bytes)
        .expect("the reading");
    reader
}

fn read_back(text: &[u8]) -> Shape {
    let mut reader = loaded(text);
    reader
        .transfer_roots(&ProgressRange::default())
        .expect("the transfer");
    reader.xs_reader().one_shape()
}

/// The reading with the edges of each wire left in the order the file gives them.
fn read_back_in_file_order(text: &[u8]) -> Shape {
    let mut reader = loaded(text);
    let (mut parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
        &AsciiString::from("read.iges.resource.name"),
        &AsciiString::from("read.iges.sequence"),
    )
    .expect("the settings of the healing");
    parameters.bind(
        AsciiString::from("FixShape.FixReorderMode"),
        AsciiString::from("0"),
    );
    reader.xs_reader().set_shape_fix_parameters(parameters);
    reader.xs_reader().set_shape_process_flags(flags);
    reader
        .transfer_roots(&ProgressRange::default())
        .expect("the transfer");
    reader.xs_reader().one_shape()
}

fn assert_same_area(found: &Shape, expected: f64) {
    let area = area(found);
    assert!(
        (area - expected).abs() <= RELATIVE_AREA_TOLERANCE * expected,
        "an area of {area}, expected {expected}"
    );
}

#[test]
fn a_torus_written_as_faces_keeps_its_area() {
    let _guard = locked();
    let shape = torus();
    assert_same_area(&read_back(&as_faces(&shape)), area(&shape));
}

/// The second file and the transfer are right: only the reordering of the healing loses the surface.
#[test]
fn the_second_file_of_a_torus_holds_its_whole_surface() {
    let _guard = locked();
    let shape = torus();
    let second = as_faces(&read_back(&as_faces(&shape)));
    assert_same_area(&read_back_in_file_order(&second), area(&shape));
}

#[test]
fn a_torus_read_and_written_as_faces_keeps_its_area() {
    let _guard = locked();
    let shape = torus();
    let second = as_faces(&read_back(&as_faces(&shape)));
    assert_same_area(&read_back(&second), area(&shape));
}
