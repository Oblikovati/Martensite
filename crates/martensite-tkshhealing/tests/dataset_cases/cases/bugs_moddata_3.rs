//! bugs moddata 3 regression cases owned by
//! TKShHealing whose inputs are in the public dataset or built by the script.
use super::bugs_common::banner;
use crate::support::{
    build::{edge, whatis},
    commands::explode_type,
    commands::{mkcurve, mksurface},
    intersect::{intersect_curve_surface, projponf},
    log,
    qa_wires::occ25202,
    restore::Restored,
    restore::restore_as,
    session::{Session, checkview_display, set, set_exploded},
    sewing::{checkmaxtol, checkmaxtol_min},
    swdraw::removeloc,
    text::numbers_after,
    topology::{mkplane, nurbsconvert, trotate, vertex, wire},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn box100(s: &mut dyn Session) -> martensite_tkbrep::topods::Shape {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(100., 100., 100.).unwrap();
    set(s, "b", maker.shape().unwrap().clone())
}

/// `whatis result` must (bug25202_2) or must not (bug25202_1) report Closed.
fn check_closed(info: &str, closed_expected: bool) {
    if info.contains("Closed") == closed_expected {
        log::puts("OK : value of IsClosed flag is correct");
    } else {
        log::puts("Error : value of IsClosed flag is not correct");
    }
}

/// tests/bugs/moddata_3/bug25202_1.
/// Replacing two faces of a box by open planar faces clears the Closed flag (`OCC25202`).
pub fn bug25202_1(s: &mut dyn Session) {
    banner(&["========", "CR25202", "========", ""]);
    let b = box100(s);
    let v1 = set(s, "v1", vertex(Point3::new(0., 0., 50.)));
    let vertices = set_exploded(s, "b", explode_type(&b, ShapeEnum::Vertex));
    let ee1 = set(s, "ee1", edge(&vertices[1], &v1));
    let ee2 = set(s, "ee2", edge(&v1, &vertices[0]));
    let edges = set_exploded(s, "b", explode_type(&b, ShapeEnum::Edge));
    let ww1 = wire(&[&ee1, &ee2, &edges[1], &edges[2], &edges[3]]).unwrap();
    let ww1 = set(s, "ww1", ww1);
    let ww2 = wire(&[&ee1, &ee2, &edges[8], &edges[9], &edges[4]]).unwrap();
    let ww2 = set(s, "ww2", ww2);
    let ff1 = set(
        s,
        "ff1",
        mkplane(&ww1).unwrap_or_else(|error| panic!("{error}")),
    );
    let ff2 = set(
        s,
        "ff2",
        mkplane(&ww2).unwrap_or_else(|error| panic!("{error}")),
    );
    set_exploded(s, "b", explode_type(&b, ShapeEnum::Face));
    let (result, text) = occ25202(&b, [(1, &ff1), (3, &ff2)]);
    log::result(&text);
    let result = set(s, "result", result);
    let info = whatis("result", &result);
    log::result(&info);
    check_closed(&info, false);
    checkview_display(s, Some("result"), "bug25202_1");
}

/// tests/bugs/moddata_3/bug25202_2.
/// Removing the locations of a rotated shell keeps its Closed flag.
pub fn bug25202_2(s: &mut dyn Session) {
    banner(&["========", "CR25202", "========", ""]);
    let b = box100(s);
    let shells = set_exploded(s, "b", explode_type(&b, ShapeEnum::Shell));
    let rotated = trotate(&shells[0], [0., 0., 0.], [1., 0., 0.], 45., false);
    let b_1 = set(s, "b_1", rotated);
    let result = set(s, "result", removeloc(&b_1));
    let info = whatis("result", &result);
    log::result(&info);
    check_closed(&info, true);
    checkview_display(s, Some("result"), "bug25202_2");
}

/// tests/bugs/moddata_3/bug32557.
/// BRepBuilderAPI_NurbsConvert keeps the 2d p-curves without gaps (three faces).
pub fn bug32557(s: &mut dyn Session) {
    let rule = "===========================================================";
    banner(&[
        rule,
        "0032557: Modeling Data - Use of BRepBuilderAPI_NurbsConvert create 2d p-curves with gaps",
        rule,
        "",
    ]);
    for (file, reference) in [
        ("bug32557.brep", 1.1e-7),
        ("bug32557_1.brep", 1.1e-5),
        ("bug32557_2.brep", 0.0002570),
    ] {
        let f = restore_as(s, file, "f").shape();
        let bf = set(s, "bf", nurbsconvert(&f));
        checkmaxtol(&bf, "bf", reference);
    }
}

/// tests/bugs/moddata_3/bug24065.
/// The second intersection point of the edge curve and the face surface projects on the
/// face at 1.86835288e-8 (within 1e-7), and the face tolerance stays at most 1e-7.
pub fn bug24065(s: &mut dyn Session) {
    banner(&["========", "OCC24065", "========", ""]);
    let f = restore_as(s, "bug24065_f.brep", "f").shape();
    let e = restore_as(s, "bug24065_e.brep", "e").shape();
    let cmp_tol = 1.0e-7;
    checkmaxtol_min(&f, "f", cmp_tol);
    let surface = mksurface(&f);
    s.set("s", &Restored::Surface(surface.clone()));
    let c = mkcurve(&e);
    s.set("c", &Restored::Curve(c.clone()));
    let points = intersect_curve_surface(&c, &surface).unwrap_or_else(|error| panic!("{error}"));
    assert!(
        points.len() >= 2,
        "projponf: i_2 is not a point ({} points)",
        points.len()
    );
    for (index, point) in points.iter().enumerate() {
        s.set(&format!("i_{}", index + 1), &Restored::Point(*point));
    }
    let distmax = projponf(&f, points[1]);
    let max_dist = numbers_after(&distmax, "proj dist = ", 1, "").expect("proj dist")[0];
    if (max_dist - 1.86835288e-8).abs() > cmp_tol {
        log::puts("Error: Wrong intersection point");
    }
}
