//! Fixtures shared by the martensite-tkcaf unit test translations: documents, boxes and attribute lookups.
#![allow(dead_code)]
pub mod dnaming;
pub mod naming;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkcaf::tnaming::TNamingNamedShape;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::{
    TDocStdApplication, TDocStdDocument, TDocStdStandardApplication,
};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;

/// `myApp->NewDocument("BinOcaf", myDoc)` of the fixtures (the Bin drivers are not implemented
/// and no test reads or writes a file).
pub fn new_application_document() -> (Handle<TDocStdStandardApplication>, Handle<TDocStdDocument>) {
    let application = TDocStdStandardApplication::new();
    let mut document: Handle<TDocStdDocument> = Handle::default();
    application
        .new_tdoc_std_document(&ExtendedString::new("BinOcaf"), &mut document)
        .unwrap();
    (application, document)
}

/// `new TDocStdDocument("BinOcaf")`.
pub fn new_document() -> Handle<TDocStdDocument> {
    TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap()
}

/// `Main().FindChild(tag, true)`.
pub fn main_child(document: &Handle<TDocStdDocument>, tag: i32) -> TdfLabel {
    document.main().unwrap().find_child(tag, true).unwrap()
}

/// `BRepPrimAPIMakeBox(dx, dy, dz).Shape()`.
pub fn make_box(dx: f64, dy: f64, dz: f64) -> Shape {
    BRepPrimAPIMakeBox::from_sizes(dx, dy, dz)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape()
}

/// `BRepPrimAPIMakeBox(P1, P2).Shape()`.
pub fn make_box_between(first: [f64; 3], second: [f64; 3]) -> Shape {
    let first = Point3::new(first[0], first[1], first[2]);
    let second = Point3::new(second[0], second[1], second[2]);
    BRepPrimAPIMakeBox::from_points(first, second)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape()
}

/// `BRepPrimAPIMakeBox(P, dx, dy, dz).Shape()`.
pub fn make_box_at(corner: [f64; 3], dx: f64, dy: f64, dz: f64) -> Shape {
    let corner = Point3::new(corner[0], corner[1], corner[2]);
    BRepPrimAPIMakeBox::at_point(corner, dx, dy, dz)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape()
}

/// `label.FindAttribute(TNamingNamedShape::GetID(), aNS)`.
pub fn find_named_shape(label: &TdfLabel) -> Option<Handle<TNamingNamedShape>> {
    let mut named: Handle<TNamingNamedShape> = Handle::default();
    label
        .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
        .unwrap()
        .then_some(named)
}

/// The sub-shapes of a type in exploration order (`TopExpExplorer`).
pub fn explore(shape: &Shape, shape_type: ShapeEnum) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut explorer = Explorer::new(shape, shape_type, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        shapes.push(explorer.current().clone());
        explorer.next().unwrap();
    }
    shapes
}

/// The `index`-th distinct sub-shape of a type (1-based), skipping the shape itself.
pub fn shape_at(shape: &Shape, shape_type: ShapeEnum, index: usize) -> Shape {
    let mut seen = MapOfShape::default();
    seen.add(shape.clone());
    let mut count = 0;
    for current in explore(shape, shape_type) {
        if !seen.add(current.clone()) {
            continue;
        }
        count += 1;
        if count == index {
            return current;
        }
    }
    Shape::default()
}
