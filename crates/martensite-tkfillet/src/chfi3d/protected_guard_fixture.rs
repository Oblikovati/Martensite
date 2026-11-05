//! Real owned geometry for internal guard witnesses, without widening the extension API.
use super::{ChFi3dFilBuilder, FilletShape};
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Face},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// Keep a non-null output sentinel independent of the guard; `protected_guard_builder_face()`.
pub(crate) fn protected_guard_builder_face() -> (ChFi3dFilBuilder, Face) {
    let solid = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let builder = ChFi3dFilBuilder::new(&solid, FilletShape::Rational, 1.0e-2).unwrap();
    let faces = Explorer::new(&solid, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    // Any real face works: this guard must preserve its supplied output identity.
    let sentinel = topods::face(faces.current()).unwrap();
    assert!(
        !sentinel.as_ref().is_null(),
        "guard sentinel null=true; expected a real box Face"
    );
    (builder, sentinel)
}
