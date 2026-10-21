//! `bugs_moddata_2` case bodies whose
//! inputs are in the public dataset.
use crate::support::{
    checks::checklength2d,
    commands::{dump_curve2d, explode_type, mkcurve, project, segment, trim_to_basis},
    dataset::located_bytes,
    restore::restore_as,
    session::{Session, checkview_display, set, set_exploded},
};
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorCompCurve as CompCurve, brep_tools::WireExplorer, topexp::Explorer,
    topods,
};
use martensite_tkernel::precision;
use martensite_tkg3d::{adaptor3d::Adaptor3dCurve, top_abs::ShapeEnum};
use martensite_tkgeombase::gcpnts::AbscissaPoint;

/// `catch { segment result 1 2.5 } msg` must not report an exception.
pub fn bug23175(s: &mut dyn Session) {
    let result = restore_as(s, "OCC23175-bspline-per-3_draw", "result").curve();
    segment(&result, 1., 2.5).expect("Faulty OCC23175");
    checkview_display(s, Some("result"), "bug23175");
}

fn projection_length(s: &mut dyn Session, inputs: [(&str, &str); 2], expected: f64) {
    let curve = restore_as(s, inputs[0].0, inputs[0].1).curve();
    let surface = restore_as(s, inputs[1].0, inputs[1].1).surface();
    let result = project(&curve, &surface, precision::confusion()).expect("Projection Failed");
    checklength2d(&set(s, "result", result), expected);
}

pub fn bug23464_1(s: &mut dyn Session) {
    let inputs = [("bug23464_c41.draw", "c41"), ("bug23464_s46.draw", "s46")];
    projection_length(s, inputs, 3.1415926535898007);
}

pub fn bug23464_2(s: &mut dyn Session) {
    let inputs = [("bug23464_c50.draw", "c50"), ("bug23464_s37.draw", "s37")];
    projection_length(s, inputs, 3.1415926535898002);
}

pub fn bug23464_3(s: &mut dyn Session) {
    let inputs = [("bug23464_c57.draw", "c57"), ("bug23464_s37.draw", "s37")];
    projection_length(s, inputs, 3.1415926535897993);
}

pub fn bug23464_4(s: &mut dyn Session) {
    let inputs = [("bug23464_c58.draw", "c58"), ("bug23464_s37.draw", "s37")];
    projection_length(s, inputs, 3.1415926535898038);
}

pub fn bug23464_5(s: &mut dyn Session) {
    let inputs = [("bug23464_c57.draw", "c57"), ("bug23464_s46.draw", "s46")];
    projection_length(s, inputs, 3.1415926535897998);
}

pub fn bug23464_6(s: &mut dyn Session) {
    let inputs = [("bug23464_c58.draw", "c58"), ("bug23464_s46.draw", "s46")];
    projection_length(s, inputs, 3.1415926535898016);
}

/// `explode sh1 e`, `mkcurve`, `trim result c` and a `segment` that must not fail.
fn segment_of_edge(s: &mut dyn Session, file: &str, edge: usize, range: (f64, f64), image: &str) {
    let sh1 = restore_as(s, file, "sh1").shape();
    let edges = set_exploded(s, "sh1", explode_type(&sh1, ShapeEnum::Edge));
    let curve = set(s, &format!("c{edge}"), mkcurve(&edges[edge - 1]));
    let result = set(s, "result", trim_to_basis(&curve));
    segment(&result, range.0, range.1).expect("Faulty OCC303: function SEGMENT works wrongly");
    checkview_display(s, Some("result"), image);
}

pub fn bug303_1(s: &mut dyn Session) {
    let range = (22.07126804854423, 23.6320483505636);
    segment_of_edge(s, "OCC303a.brep", 5, range, "bug303_1");
}

pub fn bug303_2(s: &mut dyn Session) {
    let range = (23.09944997429109, 23.6320483505636);
    segment_of_edge(s, "OCC303b.brep", 1, range, "bug303_2");
}

/// `REQUIRED All: Projection Failed`; a successful projection is a failure. The
/// checkview then finds no `result` variable.
pub fn bug308(s: &mut dyn Session) {
    let a = restore_as(s, "OCC308a.draw", "a").curve();
    let b = restore_as(s, "OCC308b.draw", "b").surface();
    let result = project(&a, &b, precision::confusion());
    assert!(
        result.is_none(),
        "Faulty OCC308 : function PROJECT works wrongly"
    );
    checkview_display(s, None, "bug308");
}

/// The projection of the circle on the sphere must not fail.
pub fn bug360(s: &mut dyn Session) {
    let c = restore_as(s, "OCC360a.draw", "c").curve();
    let cf = restore_as(s, "OCC360b.draw", "cf").surface();
    let result = project(&c, &cf, precision::confusion());
    set(
        s,
        "result",
        result.expect("Faulty  OCC360: command PROJECT works wrongly"),
    );
    checkview_display(s, Some("result"), "bug360");
}

/// The projection must succeed and its dump must differ from the
/// stored dump of the wrong old result (equal dumps print "FAULTY").
fn projection_differs(s: &mut dyn Session, inputs: [(&str, &str); 3], name: &str, tolerance: f64) {
    let curve = restore_as(s, inputs[0].0, inputs[0].1).curve();
    let surface = restore_as(s, inputs[1].0, inputs[1].1).surface();
    let projected = project(&curve, &surface, tolerance).expect("projection FAULTY");
    let projected = set(s, name, projected);
    let olddump = String::from_utf8(located_bytes(inputs[2].0)).unwrap();
    assert_ne!(dump_curve2d(name, &projected), olddump, "{name}: FAULTY");
}

pub fn bug515(s: &mut dyn Session) {
    let inputs = [
        ("OCC515a.draw", "cx"),
        ("OCC515b.draw", "s"),
        ("OCC515_DUMP", ""),
    ];
    projection_differs(s, inputs, "c2d", 0.0015);
}

pub fn bug542(s: &mut dyn Session) {
    let inputs = [
        ("OCC542_C3D.draw", "c3d"),
        ("OCC542_S.draw", "s"),
        ("OCC542_DUMP", ""),
    ];
    projection_differs(s, inputs, "result", precision::confusion());
    checkview_display(s, Some("result"), "bug542");
}

/// The point at half the curvilinear length of the wire, by the
/// abscissa-parameterised and the plain composite curve. The case carries
/// a known-problem marker: the two points are known to differ by more than
/// Precision::Confusion, so the command prints `error_dist` instead of `OK`.
pub fn bug5698(s: &mut dyn Session) {
    let s1 = topods::wire(&restore_as(s, "OCC5698.brep", "s1").shape()).unwrap();
    let curve = CompCurve::with_wire(&s1, true).unwrap();
    let need_length = curve.last_parameter().unwrap() / 2.;
    let point = curve.eval_d0(need_length).unwrap();
    let check_curve = CompCurve::with_wire(&s1, false).unwrap();
    let adaptor: &dyn Adaptor3dCurve = &check_curve;
    let check = AbscissaPoint::from_abscissa(adaptor, need_length, 0.).unwrap();
    let check_point = check_curve.eval_d0(check.parameter().unwrap()).unwrap();
    let error_dist = point.distance(&check_point);
    assert!(
        error_dist > precision::confusion(),
        "TODO OCC11111: OK, the defect is gone"
    );
}

/// `BRepTools_WireExplorer` over the wire on the face. The case's
/// `regexp {Count = ()}` always captures an empty count, so it always prints
/// a fault line, which its known-problem marker accepts; the command itself must run.
pub fn bug909(s: &mut dyn Session) {
    let f = topods::face(&restore_as(s, "OCC909_f.rle", "f").shape()).unwrap();
    let w = topods::wire(&restore_as(s, "OCC909_w.rle", "w").shape()).unwrap();
    let mut count = 0;
    if Explorer::new(&w, ShapeEnum::Vertex, ShapeEnum::Shape)
        .unwrap()
        .more()
    {
        let mut explorer = WireExplorer::with_wire_on_face(&w, &f).unwrap();
        while explorer.more() {
            count += 1;
            explorer.next().unwrap();
        }
    }
    println!("Count = {count}");
}
