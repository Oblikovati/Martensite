//! `bugs_modalg_1` case bodies (the inputs of buc60684, bug108 and bug1243_* are
//! public dataset members).
use crate::support::{
    bounding::{Bounding, bounding},
    checkshape::checkshape,
    curves::{edge, mkplane, shape_set, vertex_set},
    log,
    logged_checks::checknbshapes,
    prim::{PrismFlag, prism, revol},
    props_view::{Expected, checkprops},
    restore::restore_as,
    session::{Session, ViewStep, checkview_display},
    text::numbers_after,
    topology::{maxtolerance_text, wire},
};

/// The revolution of a planar triangle by 180 degrees.
pub fn bug1226(s: &mut dyn Session) {
    log::puts("================");
    log::puts("OCC1226");
    log::puts("The revolution problem reproduced in OCC Samples");
    log::puts("================");
    log::puts("");
    let v1 = vertex_set(s, "v1", [0., 0., 0.]);
    let v2 = vertex_set(s, "v2", [50., 0., 0.]);
    let v3 = vertex_set(s, "v3", [50., 50., 0.]);
    let e1 = shape_set(s, "e1", edge(&v1, &v2));
    let e2 = shape_set(s, "e2", edge(&v2, &v3));
    let e3 = shape_set(s, "e3", edge(&v3, &v1));
    let w = shape_set(s, "w", wire(&[&e1, &e2, &e3]).unwrap());
    let f = shape_set(s, "f", mkplane(&w, false).unwrap());
    let result = revol(&f, [0., 0., 30.], [0., 1., 0.], 180., false).expect("revol result");
    let result = shape_set(s, "result", result.shape);
    checkprops(s, "result", &result, Expected::s(23211.3));
    checkshape(&result);
    checkview_display(s, Some("result"), "bug1226");
}

/// `regexp { +<kind> +: +Min +[-0-9.+eE]+ +Max +([-0-9.+eE]+)} $tolerance full Max`.
fn printed_max(tolerance: &str, kind: &str) -> f64 {
    let at = tolerance
        .find(kind)
        .unwrap_or_else(|| panic!("maxtolerance prints no {kind}"));
    numbers_after(&tolerance[at..], "Max ", 1, "").expect("the Max value")[0]
}

fn bug1243(s: &mut dyn Session, input: &str, expected_area: f64, image: &str) {
    log::puts("================");
    log::puts("OCC1243");
    log::puts("================");
    log::puts("Bad solid created from face with command 'revol'");
    log::puts("");
    let f1 = restore_as(s, input, "f1").shape();
    checkshape(&f1);
    let result = revol(&f1, [-5000., 0., 0.], [10000., 0., 0.], 360., false).expect("revol");
    let result = shape_set(s, "result", result.shape);
    let tolerance = maxtolerance_text("result", &result);
    let max_face = printed_max(&tolerance, "Face ");
    let max_edge = printed_max(&tolerance, "Edge ");
    let max_vertex = printed_max(&tolerance, "Vertex");
    let max_tolerance = 1.00000e-06;
    if max_face > max_tolerance || max_edge > max_tolerance || max_vertex > max_tolerance {
        log::puts("OCC1243 - Error");
    } else {
        log::puts("OCC1243 - OK");
    }
    checkprops(s, "result", &result, Expected::s(expected_area));
    checkshape(&result);
    checkview_display(s, Some("result"), image);
}

/// The full revolution of the first face keeps the tolerances at 1e-6.
pub fn bug1243_1(s: &mut dyn Session) {
    bug1243(s, "OCC1243-1.brep", 350733., "bug1243_1");
}

/// The same for the second face.
pub fn bug1243_2(s: &mut dyn Session) {
    bug1243(s, "OCC1243-2.brep", 3.06024e+06, "bug1243_2");
}

/// The revolution of a planar profile (only-plane face) about a tilted axis.
pub fn bug15036(s: &mut dyn Session) {
    log::puts("============");
    log::puts("OCC15936");
    log::puts("============");
    log::puts("");
    let coords = [
        [0., 725., 20.],
        [0., 709., 20.],
        [0., 709., 230.],
        [0., 645., 230.],
        [0., 645., 320.],
        [0., 805., 320.],
        [0., 805., 230.],
        [0., 741., 230.],
        [0., 741., 20.],
    ];
    let v: Vec<_> = coords
        .iter()
        .enumerate()
        .map(|(i, p)| vertex_set(s, &format!("v{}", i + 1), *p))
        .collect();
    let names = [
        "e12", "e23", "e34", "e45", "e56", "e67", "e78", "e89", "e91",
    ];
    let edges: Vec<_> = names
        .iter()
        .enumerate()
        .map(|(i, name)| shape_set(s, name, edge(&v[i], &v[(i + 1) % 9])))
        .collect();
    let refs: Vec<_> = edges.iter().collect();
    let w = shape_set(s, "w", wire(&refs).unwrap());
    let fr = shape_set(s, "fr", mkplane(&w, true).unwrap());
    let result = revol(
        &fr,
        [0.001428571428572667, 640.5285714285715, 20.],
        [-1.691188905560395E-05, 0.9999999998569939, 0.],
        360.,
        false,
    )
    .expect("revol");
    let result = shape_set(s, "result", result.shape);
    checkprops(s, "result", &result, Expected::s(1.03597e+06));
    checkshape(&result);
    checknbshapes(&result, [9, 15, 13, 9, 1, 1, 0, 0, 48].map(Some), None);
    checkview_display(s, Some("result"), "bug15036");
}

/// The prism of the restored wire.
pub fn buc60684(s: &mut dyn Session) {
    log::puts("=============");
    log::puts("BUC60684");
    log::puts("=============");
    let w = restore_as(s, "buc60684a.brep", "w").shape();
    checkshape(&w);
    let result = prism(&w, [0., 0., 10.], PrismFlag::None).unwrap().shape;
    let result = shape_set(s, "result", result);
    checkprops(s, "result", &result, Expected::s(148.875));
    checkshape(&result);
    checkview_display(s, Some("result"), "buc60684");
}

/// The bounding box of a planar face bounded by a circle is not too small.
pub fn bug108(s: &mut dyn Session) {
    log::puts("================");
    log::puts("BUC61037");
    log::puts("OCC108");
    log::puts("================");
    log::puts("");
    let sh1 = restore_as(s, "bug61037.brep", "sh1").shape();
    checkshape(&sh1);
    let out = bounding(
        s,
        Bounding {
            print: true,
            result_shape: Some("result"),
            ..Bounding::of(&sh1)
        },
    );
    let result = out.shape.expect("bounding -shape result");
    checkprops(
        s,
        "result",
        &result,
        Expected {
            volume: Some(5.78576e-005),
            area: Some(0.042162),
            ..Expected::default()
        },
    );
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Display(&["sh1", "result"]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug108"));
}
