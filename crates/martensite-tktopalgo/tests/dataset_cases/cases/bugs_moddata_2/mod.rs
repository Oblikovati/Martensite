//! `bugs_moddata_2` case bodies whose
//! inputs are in the public dataset. The checkshape stability, projection and
//! diagnostic-command cases are in `checks.rs` and `qa.rs`.
mod checks;
mod qa;

pub use checks::*;
pub use qa::*;

use crate::support::{
    checkshape::checkshape,
    checkshape_structural::checkshape_structural,
    commands::{explode_type, mksurface, offset_surface},
    log,
    props::{Prop, checkprops},
    restore::restore_as,
    session::{Session, ViewStep, checkview_display, set, set_exploded},
    topology::{checksection, compound, mkface, mkplane, nurbsconvert, setflags_locked},
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// The first shape is valid, the second reports a
/// bad subshape orientation and an unorientable shape.
pub fn bug25(s: &mut dyn Session) {
    log::required("Faulty shapes in variables faulty_1 to faulty_");
    let sh1 = restore_as(s, "OCC25-1.brep", "sh1").shape();
    if checkshape(&sh1).has_faulty() {
        log::puts(
            "Faulty OCC25 (case 1): Checkshape says that the wire on face has bad orientation, but it is not truth",
        );
    }
    let sh2 = restore_as(s, "OCC25-2.brep", "sh2").shape();
    let che = checkshape(&sh2).text;
    if !(che.contains("BRepCheck_BadOrientationOfSubshape")
        && che.contains("BRepCheck_UnorientableShape"))
    {
        log::puts("Faulty OCC25 (case 2): Checkshape works wrongly");
    }
    set(s, "result", compound(&[&sh1, &sh2]));
    checkview_display(s, Some("result"), "bug25");
}

/// Lprops of a shape with degenerated edges.
pub fn bug268(s: &mut dyn Session) {
    let result = restore_as(s, "OCC268.brep", "result").shape();
    checkshape(&result);
    checkprops(&result, Prop::Length, 56.6039);
    checksection(&result);
    checkview_display(s, Some("result"), "bug268");
}

/// Mkplane on the wire, checkshape and the area.
fn bug278(s: &mut dyn Session, file: &str, area: f64, image: &str) {
    let a = restore_as(s, file, "a").shape();
    let result = match mkplane(&a) {
        Ok(face) => set(s, "result", face),
        Err(message) => panic!("{message}"),
    };
    checkshape(&result);
    checkprops(&result, Prop::Area, area);
    checkview_display(s, Some("result"), image);
}

pub fn bug278_1(s: &mut dyn Session) {
    bug278(s, "OCC278a.brep", 2914.99, "bug278_1");
}

pub fn bug278_2(s: &mut dyn Session) {
    bug278(s, "OCC278b.brep", 22740.0, "bug278_2");
}

/// Case 3: REQUIRED faulty_1 to faulty_ (the self-crossing wire on its plane).
pub fn bug278_3(s: &mut dyn Session) {
    log::required("Faulty shapes in variables faulty_1 to faulty_");
    bug278(s, "OCC278c.brep", 43.3075, "bug278_3");
}

/// Checkshape.
pub fn bug35(s: &mut dyn Session) {
    log::required("Faulty shapes in variables faulty_1 to faulty_");
    let result = restore_as(s, "OCC35.brep", "result").shape();
    checkshape(&result);
    checkview_display(s, Some("result"), "bug35");
}

/// `offset su1 su ±1` (Geom_OffsetSurface) of the B-spline surface, mkface and
/// the area.
fn bug374(s: &mut dyn Session, distance: f64, area: f64, image: &str) {
    let su = restore_as(s, "OCC374.draw", "su").surface();
    let offset = offset_surface(&su, distance).unwrap_or_else(|e| panic!("offset: {e}"));
    let name = if distance > 0.0 { "su1" } else { "su2" };
    let offset = set(s, name, offset);
    let result = set(s, "result", mkface(&offset));
    checkprops(&result, Prop::Area, area);
    checkview_display(s, Some("result"), image);
}

pub fn bug374_1(s: &mut dyn Session) {
    bug374(s, 1.0, 7960.45, "bug374_1");
}

pub fn bug374_2(s: &mut dyn Session) {
    bug374(s, -1.0, 7181.83, "bug374_2");
}

/// The offset of the surface can be created; mkface of the surface and its area.
pub fn bug376(s: &mut dyn Session) {
    let surface = restore_as(s, "OCC376.rle", "s").surface();
    let r = offset_surface(&surface, 1.0).unwrap_or_else(|e| panic!("offset: {e}"));
    set(s, "r", r);
    let result = set(s, "result", mkface(&surface));
    checkprops(&result, Prop::Area, 1256.64);
    checkview_display(s, Some("result"), "bug376");
}

/// Checkshape does not detect the FORWARD
/// face of the solid; the view with the normals.
pub fn bug3896(s: &mut dyn Session) {
    log::todo("Faulty OCC3896");
    let a = restore_as(s, "OCC3896-pipe.brep", "a").shape();
    let faces = set_exploded(s, "a", explode_type(&a, ShapeEnum::Face));
    let result = set(s, "result", compound(&[&faces[0], &faces[12]]));
    set_exploded(s, "a_1", explode_type(&faces[0], ShapeEnum::Edge));
    set_exploded(s, "a_13", explode_type(&faces[12], ShapeEnum::Edge));
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Donly(&["result"]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::RotateRight);
    s.step(ViewStep::Normals(&result, 200.0));
    log::puts("Normal of a_13 face is red (FORWARD)");
    if checkshape(&result).has_faulty() {
        log::puts("OK OCC3896 : checkshape detect faulty faces in solid shape");
    } else {
        log::puts("Faulty OCC3896 : checkshape does not detect faulty faces in solid shape");
    }
    checkview_display(s, Some("result"), "bug3896");
}

/// The structural checkshape must not raise.
pub fn bug432(s: &mut dyn Session) {
    let result = restore_as(s, "OCC432.brep", "result").shape();
    checkshape_structural(&result, "result", "r", Default::default());
    checkview_display(s, Some("result"), "bug432");
}

/// The structural checkshape must find the face bad.
pub fn bug433(s: &mut dyn Session) {
    let result = restore_as(s, "OCC433.brep", "result").shape();
    let err = checkshape_structural(&result, "result", "r", Default::default());
    if err.contains("OK") {
        log::puts(" Faulty OCC433: function CHECKSHAPE works wrongly");
    }
    checkview_display(s, Some("result"), "bug433");
}

/// Nurbsconvert of the locked shape, its area.
pub fn bug466(s: &mut dyn Session) {
    let res = restore_as(s, "OCC466.brep", "res").shape();
    checkshape(&res);
    setflags_locked(&res);
    let result = set(s, "result", nurbsconvert(&res));
    checkprops(&result, Prop::Area, 693.577);
    checkview_display(s, Some("result"), "bug466");
}

/// `sprops result 0.001` must end (in `catch`) within the cpulimit.
pub fn bug503(s: &mut dyn Session) {
    let result = restore_as(s, "OCC503.brep", "result").shape();
    let done = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::support::props::props_logged(Prop::Area, &result, Some(0.001));
    }));
    if done.is_err() {
        log::puts("Faulty OCC503 : Script hangs up during sprops operation ");
    } else {
        log::puts("OCC503 OK: function SPROPS works without hangs up ");
    }
    checkview_display(s, Some("result"), "bug503");
}

/// Checkshape.
pub fn bug889(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_");
    let result = restore_as(s, "OCC889.rle", "result").shape();
    checkshape(&result);
    checkview_display(s, Some("result"), "bug889");
}

/// Checkshape of the valid shape.
pub fn bug956_2(s: &mut dyn Session) {
    let result = restore_as(s, "OCC956_2.brep", "result").shape();
    checkshape(&result);
    checkview_display(s, Some("result"), "bug956_2");
}

/// The surface of a restored face (`mksurface`), set as `name`.
pub(crate) fn face_surface(
    s: &mut dyn Session,
    face: &martensite_tkbrep::topods::Shape,
    name: &str,
) -> martensite_tkernel::standard::Handle<dyn martensite_tkg3d::geom::GeomSurface> {
    set(s, name, mksurface(face))
}
