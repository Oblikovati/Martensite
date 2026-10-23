//! bugs modalg 6 regression cases owned by
//! TKShHealing (they read tolerances or heal with SWscenario commands). Each body follows its
//! script line by line; the bugs group end only prints `TEST COMPLETED`.
use super::bugs_common::{banner, tolerance_max};
use crate::support::{
    build::{edge, nexplode, wire_unsorted},
    checkshape::checkshape,
    log,
    logged_checks::{checknbshapes_some, checkreal},
    props::{Prop, checkprops},
    restore::{Restored, restore_as},
    session::{Session, set, set_exploded},
    sewing::{checkmaxtol, checkmaxtol_value, sewing, tolerance_text},
    swdraw::{fixshape, settolerance},
    text::numbers_after,
    topology::{mkface_bounds, vertex},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// OCC26241: sewing two faces of tolerances 2 and 3 at 3 keeps the maximum tolerance 3.
pub fn bug26241(s: &mut dyn Session) {
    banner(&["========", "OCC26241", "========", ""]);
    let p = crate::support::geom_create::plane(&[0., 0., 0., 0., 0., 1., 1., 0., 0.]);
    s.set("p", &Restored::Surface(p.clone()));
    let f1 = set(s, "f1", mkface_bounds(&p, [0., 10., 0., 10.]));
    let f2 = set(s, "f2", mkface_bounds(&p, [0., 10., 11., 21.]));
    log::result(&settolerance(&f1, 2.));
    log::result(&settolerance(&f2, 3.));
    let res = sewing(3., &[&f1, &f2])
        .map(|shape| set(s, "res", shape))
        .expect("tolerance: res is not a shape");
    let max = tolerance_max(&res);
    log::puts(&format!("MaxTolerance={max}"));
    checkreal("MaxTolerance", max, 3.0, 0.1, 0.1);
}

/// 0026627: Shape Healing must end within the `cpulimit 10` the script sets.
pub fn bug26627(s: &mut dyn Session) {
    banner(&[
        "##################################################",
        "0026627: Shape Healing hangs as of OCC 6.8.0",
        "##################################################",
    ]);
    let a = restore_as(s, "bug26627_fixed.brep", "a").shape();
    log::result(&tolerance_text(&a));
    checkshape(&a);
    martensite_process_policy::set_process_limit(Some(10))
        .unwrap_or_else(|error| panic!("cpulimit 10: {error}"));
    let result = set(s, "result", fixshape(&a, &["1e-6"]));
    checkprops(&result, Prop::Volume, 4332.63);
}

/// 0026647: BRepTools::UVBounds of the face (`xbounds`, `%g` each) is -9 9 -9 0.
pub fn bug26647(s: &mut dyn Session) {
    banner(&[
        "##################################################",
        "0026647: BRepTools::UVBounds() computes zero range by V",
        "##################################################",
    ]);
    let f = restore_as(s, "bug26627_face1.brep", "f").shape();
    log::result(&tolerance_text(&f));
    checkshape(&f);
    let face = martensite_tkbrep::topods::face(&f).unwrap();
    let (u_min, u_max, v_min, v_max) = martensite_tkbrep::brep_tools::uv_bounds(&face).unwrap();
    let printed: Vec<f64> = [u_min, u_max, v_min, v_max]
        .iter()
        .map(|value| {
            martensite_tkernel::storage::format_general(*value, 6)
                .parse()
                .unwrap()
        })
        .collect();
    let expected = [("UMin", -9.), ("UMax", 9.), ("VMin", -9.), ("VMax", 0.)];
    for (value, (name, reference)) in printed.iter().zip(expected) {
        checkreal(name, *value, reference, 1e-9, 0.);
    }
}

/// 0027015: sewing nearly plane cones gives a valid shape.
pub fn bug27015(s: &mut dyn Session) {
    banner(&[
        "========",
        "0027015: Sewing returns invalid shape if some faces are nearly plane cones",
        "========",
        "",
    ]);
    let a_s = restore_as(s, "bug27015.brep", "aS").shape();
    let result = sewing(0.2, &[&a_s])
        .map(|shape| set(s, "result", shape))
        .expect("checkshape: result is not a shape");
    checkshape(&result);
    checkmaxtol(&result, "result", 0.029697490042886372);
}

/// OCC27552: an unsorted wire merges the vertices within their tolerances.
pub fn bug27552_2(s: &mut dyn Session) {
    banner(&["==========", "OCC27552", "==========", ""]);
    let points = [
        [0., 0., 0.],
        [5., 0., 0.],
        [12.5, 0., 0.],
        [10., 0., 0.],
        [20., 0., 0.],
    ];
    let names = ["p1", "p2", "p3", "p4", "p5"];
    for (name, [x, y, z]) in names.iter().zip(points) {
        s.set(name, &Restored::Point(Point3::new(x, y, z)));
    }
    let v: Vec<_> = points
        .iter()
        .zip(["v1", "v2", "v3", "v4", "v5"])
        .map(|([x, y, z], name)| set(s, name, vertex(Point3::new(*x, *y, *z))))
        .collect();
    for (index, tolerance) in [(0, 3.), (1, 6.), (2, 3.), (3, 1.)] {
        log::result(&settolerance(&v[index], tolerance));
    }
    s.set("d", &Restored::Point(Point3::new(0., 10., 0.)));
    let dv = set(s, "dv", vertex(Point3::new(0., 10., 0.)));
    let e1 = set(s, "e1", edge(&v[0], &dv));
    let e2 = set(s, "e2", edge(&v[1], &dv));
    let e3 = set(s, "e3", edge(&v[2], &dv));
    let e4 = set(s, "e4", edge(&v[3], &dv));
    let e5 = set(s, "e5", edge(&dv, &v[4]));
    let w2 = wire_unsorted(&[&e1, &e2, &e3, &e4, &e5])
        .map(|wire| set(s, "w2", wire))
        .unwrap_or_else(|error| panic!("{error}"));
    checkshape(&w2);
    let mut counts = [None; 9];
    counts[0] = Some(3);
    counts[1] = Some(5);
    checknbshapes_some(&w2, counts);
    let (vertices, warning) = nexplode(&w2, ShapeEnum::Vertex);
    log::result(&warning);
    let vertices = set_exploded(s, "w2", vertices);
    checkmaxtol(&vertices[1], "w2_2", 9.875);
}

/// OCC28601: fixshape of a box does not increase its tolerance.
pub fn bug28601(s: &mut dyn Session) {
    banner(&["========", "OCC28601", "========", ""]);
    let mut maker = BRepPrimAPIMakeBox::from_sizes(10., 10., 10.).unwrap();
    let b = set(s, "b", maker.shape().unwrap().clone());
    let before = checkmaxtol_value(&b, "b");
    let b1 = set(s, "b1", fixshape(&b, &[]));
    let after = checkmaxtol_value(&b1, "b1");
    log::puts(&format!("MAX tolerance shape before fixshape : {before}"));
    log::puts(&format!("MAX tolerance shape after fixshape : {after}"));
    if after > before {
        log::puts("Error: Export /Import cause increasing tolerances of shape");
    } else {
        log::puts("Export /Import does not cause increasing tolerances of shape");
    }
}
