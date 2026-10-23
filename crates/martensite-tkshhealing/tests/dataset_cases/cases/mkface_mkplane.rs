//! mkface mkplane regression cases whose
//! inputs are public or built: `mkplane result wire` (BRepBuilderAPI_MakeFace on the
//! wire's plane), then the mkface group end ([`super::mkface_grid::mkface_end`]).
use super::mkface_grid::{MaxTolerances, mkface_end};
use crate::support::{
    commands::explode_type,
    log,
    restore::restore,
    session::{Session, set_exploded},
    topology::{mkplane, wire},
    vars::Vars,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

const MAX_1E6: MaxTolerances = MaxTolerances {
    face: Some(1e-6),
    edge: Some(1e-6),
    vertex: Some(1e-6),
};

/// `mkplane result w`: the face, or the command's error line (result then not set).
fn mkplane_result(v: &mut Vars, w: &martensite_tkbrep::topods::Shape) {
    match mkplane(w) {
        Ok(face) => v.set("result", face),
        Err(error) => log::puts(&error),
    }
}

/// tests/mkface/mkplane/A1 (`TODO #23240 ALL: Faulty shapes in variables faulty_1 to faulty_2`).
pub fn a1(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_2");
    let mut v = Vars::new(s);
    let a = restore("pro6668a.rle").shape();
    v.set("a", a.clone());
    let edges = set_exploded(v.session(), "a", explode_type(&a, ShapeEnum::Edge));
    assert!(
        edges.len() >= 16,
        "wire: a_{} is not a shape",
        edges.len() + 1
    );
    let parts: Vec<&martensite_tkbrep::topods::Shape> = edges.iter().take(16).collect();
    let w = wire(&parts).unwrap_or_else(|error| panic!("{error}"));
    v.set("w", w.clone());
    mkplane_result(&mut v, &w);
    mkface_end(&mut v, MAX_1E6, "A1");
}

/// tests/mkface/mkplane/A2: the face of the first wire of a box.
pub fn a2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    let mut maker = BRepPrimAPIMakeBox::from_sizes(10., 10., 10.).unwrap();
    let b = maker.shape().unwrap().clone();
    v.set("b", b.clone());
    let wires = set_exploded(v.session(), "b", explode_type(&b, ShapeEnum::Wire));
    mkplane_result(&mut v, &wires[0]);
    mkface_end(&mut v, MAX_1E6, "A2");
}
