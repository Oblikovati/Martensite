//! `bugs_modalg_6` case bodies whose
//! inputs are in the public dataset.
use crate::support::{
    checks::{checklength2d, nbshapes},
    commands::{explode_type, mkcurve, mksurface, project},
    restore::restore_as,
    session::{Session, set, set_exploded},
};
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::ShapeEnum;

/// A file written by the other `BRepTools::Write` overload has no shape-set
/// header; `restore` falls back to reading it as a shape and must give a
/// null shape, so `nbshapes a` prints nothing.
fn restores_null_shape(s: &mut dyn Session, file: &str) {
    let a = restore_as(s, file, "a").shape();
    assert_eq!(nbshapes(&a), "", "ERROR: null shape is expected here");
}

pub fn bug27264_3(s: &mut dyn Session) {
    restores_null_shape(s, "bug27264_3.brep");
}

pub fn bug27264_4(s: &mut dyn Session) {
    restores_null_shape(s, "bug27264_4.brep");
}

pub fn bug27264_5(s: &mut dyn Session) {
    restores_null_shape(s, "bug27264_5.brep");
}

pub fn bug27264_6(s: &mut dyn Session) {
    restores_null_shape(s, "bug27264_6.brep");
}

/// The projection of the closed edge curve onto its cylinder face.
pub fn bug28223(s: &mut dyn Session) {
    let ff = restore_as(s, "bug28223_face.brep", "ff").shape();
    let edges = set_exploded(s, "ff", explode_type(&ff, ShapeEnum::Edge));
    let cc = set(s, "cc", mkcurve(&edges[0]));
    let ss = set(s, "ss", mksurface(&ff));
    let pc = project(&cc, &ss, precision::confusion()).expect("Projection Failed");
    checklength2d(&set(s, "pc", pc), 7.7546866164306829);
}
