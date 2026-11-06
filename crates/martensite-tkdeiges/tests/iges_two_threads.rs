//! Transfers on two threads: the reading actor is one per process, and the lock of the format keeps
//! each transfer whole. One thread fills documents, the other reads the same public files directly.
#[path = "support/public_file_path.rs"]
mod public_file_path;
#[path = "support/public_file_results.rs"]
mod public_file_results;

use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkdeiges::iges_control::{IgesControlReader, IgesControlWriter};
use martensite_tkdeiges::iges_to_brep::IgesToBRepReader;
use martensite_tkdeiges::igescaf_control::IgesCafControlReader;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdDocument};
use martensite_tkxcaf::xcaf_app::XCAFAppApplication;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use martensite_tkxsbase::xs_control::XSControlReaderOps;
use public_file_path::public_file_path;
use public_file_results::PUBLIC_FILES;
use std::sync::Barrier;

fn path_of(name: &str) -> String {
    public_file_path(name).to_string_lossy().into_owned()
}

fn nb_faces(shape: &Shape) -> i32 {
    let mut explorer =
        Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).expect("an explorer");
    let mut found = 0;
    while explorer.more() {
        found += 1;
        explorer.next().expect("the next one");
    }
    found
}

/// The faces of a file through the reader on a session, its shape then written back as faces.
fn through_control_reader(name: &str) -> i32 {
    let mut reader = IgesControlReader::new().expect("the norm");
    reader.read_file(&path_of(name)).expect("the reading");
    reader
        .transfer_roots(&ProgressRange::default())
        .expect("the transfer");
    let shape = reader.xs_reader().one_shape();
    if !shape.is_null() {
        let mut writer = IgesControlWriter::with_unit(b"MM", 0).expect("the writer");
        let _ = writer
            .add_shape(&shape, &ProgressRange::default())
            .expect("the translation");
    }
    nb_faces(&shape)
}

fn through_direct_reader(name: &str) -> i32 {
    let mut reader = IgesToBRepReader::new().expect("the reader");
    assert_eq!(reader.load_file(&path_of(name)).expect("the reading"), 0);
    reader
        .transfer_roots(false, &ProgressRange::default())
        .expect("the transfer");
    nb_faces(&reader.one_shape().expect("the shape"))
}

/// The shapes a file gives a document.
fn shapes_of_a_document(name: &str) -> usize {
    let application = XCAFAppApplication::get_application();
    let mut document = Handle::<TDocStdDocument>::default();
    application
        .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
        .expect("a document");
    let mut reader = IgesCafControlReader::new().expect("the norm");
    reader
        .perform(&path_of(name), &document, &ProgressRange::default())
        .expect("the transfer");
    let main = document.get().expect("a document").main().expect("a label");
    let tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
    let shapes = tool.get().expect("a shape tool").get_shapes();
    shapes.expect("the shapes").len()
}

fn wrong_shapes(transfer: fn(&str) -> i32) -> Vec<String> {
    let wrong = |row: &public_file_results::PublicFile| {
        let found = transfer(row.0);
        (found != row.1[0]).then(|| format!("{}: {found} faces, not {}", row.0, row.1[0]))
    };
    PUBLIC_FILES.iter().filter_map(wrong).collect()
}

fn wrong_documents() -> Vec<String> {
    let wrong = |row: &public_file_results::PublicFile| {
        let found = shapes_of_a_document(row.0);
        (found != row.3[0]).then(|| format!("{}: {found} shapes, not {}", row.0, row.3[0]))
    };
    PUBLIC_FILES.iter().filter_map(wrong).collect()
}

fn alongside_documents(transfer: fn(&str) -> i32) {
    let start = Barrier::new(2);
    let wrong: Vec<String> = std::thread::scope(|scope| {
        let shapes = scope.spawn(|| {
            start.wait();
            wrong_shapes(transfer)
        });
        let documents = scope.spawn(|| {
            start.wait();
            wrong_documents()
        });
        [shapes, documents]
            .into_iter()
            .flat_map(|thread| thread.join().expect("a thread"))
            .collect()
    });
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn control_readers_alongside_documents_give_whole_shapes() {
    alongside_documents(through_control_reader);
}

#[test]
fn direct_readers_alongside_documents_give_whole_shapes() {
    alongside_documents(through_direct_reader);
}
