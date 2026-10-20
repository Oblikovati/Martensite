//! `bugs_moddata_1` case bodies.
use crate::support::{
    curves::{edge, polyline, shape_set, vertex_set},
    distance::distmini,
    geometry::line,
    log,
    misc::{buc60852 as buc60852_command, buc60889 as buc60889_command},
    prim::{box_at, psphere},
    printed::g17,
    restore::Restored,
    session::Session,
    topology::mkedge,
};
use martensite_tkmath::gp::Point3;

const BNDBOX: [f64; 6] = [0., 0., 0., 30., 10., 20.];

fn bndbox(s: &mut dyn Session) {
    let [x1, y1, z1, x2, y2, z2] = BNDBOX;
    shape_set(s, "bndbox", box_at([x1, y1, z1], [x2, y2, z2]));
}

/// `Bnd_Box::IsOut(gp_Lin)` of the lines of two edges against a box.
pub fn buc60852(s: &mut dyn Session) {
    log::puts("========");
    log::puts("BUC60852");
    log::puts("BUC60923");
    log::puts("========");
    let p11 = [-20., -5., 10.];
    let p12 = [50., 15., 10.];
    let v11 = vertex_set(s, "vertex_1_1", p11);
    let v12 = vertex_set(s, "vertex_1_2", p12);
    let edge_1 = shape_set(s, "edge_1", edge(&v11, &v12));
    log::puts("");
    let p21 = [-20., -10., 10.];
    let p22 = [50., -10., 10.];
    let v21 = vertex_set(s, "vertex_2_1", p21);
    let v22 = vertex_set(s, "vertex_2_2", p22);
    let edge_2 = shape_set(s, "edge_2", edge(&v21, &v22));
    log::puts("");
    bndbox(s);
    let minus = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    s.set("line_1", &Restored::Curve(line(p11, minus(p11, p12))));
    s.set("line_2", &Restored::Curve(line(p21, minus(p21, p22))));
    log::puts("");
    let result1 = buc60852_command(&edge_1, BNDBOX);
    if result1 != "Line that lies on edge intersects the box\n" {
        log::puts("BUC60852: Error; (case 1)");
    } else {
        log::puts("BUC60852: OK; (case 1)");
    }
    log::puts("");
    let result2 = buc60852_command(&edge_2, BNDBOX);
    if result2 != "Line that lies on edge does not intersect the box\n" {
        log::puts("BUC60852: Error; (case 2)");
    } else {
        log::puts("BUC60852: OK; (case 2)");
    }
}

/// `Bnd_Box::IsOut(P1, P2, D)` of two bands against a box.
pub fn buc60889(s: &mut dyn Session) {
    log::puts("========");
    log::puts("BUC60889");
    log::puts("BUC60947");
    log::puts("========");
    let p11 = [-30., -10., 5.];
    let p12 = [-30., -10., 15.];
    let p13 = [-15., -5., 5.];
    let d1 = [p13[0] - p11[0], p13[1] - p11[1], p13[2] - p11[2]];
    let point = |p: [f64; 3]| Point3::new(p[0], p[1], p[2]);
    s.set("point_1_1", &Restored::Point(point(p11)));
    s.set("point_1_2", &Restored::Point(point(p12)));
    shape_set(s, "p_line_1", polyline(&[p11, p12]));
    let v11 = vertex_set(s, "vertex_1_1", p11);
    let v12 = vertex_set(s, "vertex_1_2", p13);
    let edge_1 = shape_set(s, "edge_1", edge(&v11, &v12));
    log::puts("");
    let p21 = [30., 10., -5.];
    let p22 = [30., 10., -15.];
    let p23 = [0., 10., -5.];
    let d2 = [p23[0] - p21[0], p23[1] - p21[1], p23[2] - p21[2]];
    s.set("point_2_1", &Restored::Point(point(p21)));
    s.set("point_2_2", &Restored::Point(point(p22)));
    shape_set(s, "p_line_2", polyline(&[p21, p22]));
    let v21 = vertex_set(s, "vertex_2_1", p21);
    let v22 = vertex_set(s, "vertex_2_2", p23);
    let edge_2 = shape_set(s, "edge_2", edge(&v21, &v22));
    log::puts("");
    bndbox(s);
    s.set("line_1_1", &Restored::Curve(line(p11, d1)));
    s.set("line_1_2", &Restored::Curve(line(p12, d1)));
    s.set("line_2_1", &Restored::Curve(line(p21, d2)));
    s.set("line_2_2", &Restored::Curve(line(p22, d2)));
    log::puts("");
    let result1 = buc60889_command(point(p11), point(p12), &edge_1, BNDBOX);
    if result1 != "The band intersects the box\n" {
        log::puts("BUC60889: Error; (case 1)");
    } else {
        log::puts("BUC60889: OK; (case 1)");
    }
    log::puts("");
    let result2 = buc60889_command(point(p21), point(p22), &edge_2, BNDBOX);
    if result2 != "The band lies out of the box\n" {
        log::puts("BUC60889: Error; (case 2)");
    } else {
        log::puts("BUC60889: OK; (case 2)");
    }
}

/// BRepExtrema_DistShapeShape between an infinite edge (no vertices) and a
/// sphere gives 10 without an exception.
pub fn bug22303(s: &mut dyn Session) {
    log::puts("========================");
    log::puts("OCC22303");
    log::puts("========================");
    log::puts("");
    let bug_number = "OCC22303";
    let ll = line([0., 20., 0.], [1., 0., 0.]);
    s.set("ll", &Restored::Curve(ll.clone()));
    let ee = shape_set(s, "ee", mkedge(&ll));
    let ss = shape_set(s, "ss", psphere(10.));
    let dd = distmini(s, "dd", &ee, &ss, None).expect("distmini dd ee ss");
    log::puts(&format!("Distance={}", g17(dd.value)));
    let status = i32::from(dd.value - 10. > 1.0e-3);
    log::puts("");
    if status != 0 {
        log::puts(&format!("Faulty {bug_number}"));
    } else {
        log::puts(&format!("OK {bug_number}"));
    }
}
