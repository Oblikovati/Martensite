//! Shared fixtures of the martensite-tkxcaf tests: XDE documents through a `TDocStdApplication` and
//! box shapes.
#![allow(dead_code)]

use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdoc_std::{
    TDocStdApplication, TDocStdDocument, TDocStdStandardApplication,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;

/// `anApp = new TDocStdApplication(); anApp->NewDocument(format, aDoc)`.
pub fn new_document(format: &str) -> (Handle<TDocStdStandardApplication>, Handle<TDocStdDocument>) {
    let application = TDocStdStandardApplication::new();
    let document = new_document_in(&application, format);
    (application, document)
}

/// Open a document in an existing application session.
/// Example: `let document = new_document_in(&application, "BinXCAF");`.
pub fn new_document_in(
    application: &Handle<TDocStdStandardApplication>,
    format: &str,
) -> Handle<TDocStdDocument> {
    let mut document: Handle<TDocStdDocument> = Handle::default();
    application
        .new_tdoc_std_document(&ExtendedString::from_utf8(format), &mut document)
        .unwrap();
    document
}

/// `BRepPrimAPIMakeBox(dx, dy, dz).Shape()`.
pub fn make_box(dx: f64, dy: f64, dz: f64) -> Shape {
    BRepPrimAPIMakeBox::from_sizes(dx, dy, dz)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape()
}

// Keep one reviewed comparator implementation across dependent toolkit tests.
#[allow(unused_imports)] // Each source suite compiles support with its own fixture subset.
pub use float_assertions::expect_double_eq;
use martensite_test_support::floating as float_assertions;

pub mod assembly_fixture;
pub mod case_policy;
pub mod compound_shapes;
pub mod document_viewer;
pub mod script_stream;
pub mod shape_document;
