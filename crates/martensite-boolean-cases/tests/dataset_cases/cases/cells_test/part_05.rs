//! The `cells_test` case bodies, part 5; each statement is one line of its script.
use super::*;

pub fn J2(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC28528");
    println!("========");
    println!();
    println!("Arguments: solid and a face");
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    let geom_p = plane_along([5., 5., 5.], [1., 0., 0.]);
    mkface_bounded(s, "f", &geom_p.surface, (0., 10., 0., 10.));
    bop.arguments(s, &["b", "f"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    println!("Results");
    println!("All split shapes");
    cells.bcaddall(s, "result", 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(1)],
        "Number of SOLIDs in the result containing all splits",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(8)],
        "Number of FACEs in the result containing all splits",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(725.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(1000.));
    println!("Cut solid from face");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f", 1), ("b", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(1)],
        "Number of FACEs in the result of CUT(f, b) operation",
    );
    println!("Cut face from solid");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b", 1), ("f", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(1)],
        "Number of SOLIDs in the result of CUT(b, f) operation",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(7)],
        "Number of FACEs in the result of CUT(b, f) operation",
    );
    println!("Common between face and solid");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b", 1), ("f", 1)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(1)],
        "Number of FACEs in the result of COMMON(b, f) operation",
    );
    println!("Fuse operation between face and solid");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b", 1)], 1, true);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(7)],
        "Number of faces in unified split of solid",
    );
    cells.bcadd(s, "result", &[("f", 1), ("b", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(1)],
        "Number of SOLIDs in the result of FUSE(b, f) operation",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(8)],
        "Number of FACEs in the result of FUSE(b, f) operation",
    );
}

pub fn J4(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC28528");
    println!("========");
    println!();
    println!("Arguments: solid and an edge");
    psphere(s, "s", 10.);
    let l_curve = line([0., 0., 0.], [0., 1., 0.]);
    mkedge_bounded(s, "e", &l_curve, -15., 15.);
    bop.arguments(s, &["s", "e"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    println!("Results");
    println!("All split shapes");
    cells.bcaddall(s, "result", 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(1)],
        "Number of SOLIDs in the result containing all splits",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(6)],
        "Number of EDGEs in the result containing all splits",
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(112.832));
    checkprops(s, "result", Prop::Volume, Expected::Mass(4188.79));
    println!("Cut solid from edge");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("e", 1), ("s", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(2)],
        "Number of EDGEs in the result of CUT(e, s) operation",
    );
    println!("Cut edge from solid");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s", 1), ("e", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(1)],
        "Number of SOLIDs in the result of CUT(s, e) operation",
    );
    println!("Common between edge and solid");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s", 1), ("e", 1)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(1)],
        "Number of EDGEs in the result of COMMON(s, e) operation",
    );
    println!("Fuse operation between edge and solid");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s", 1)], 1, true);
    cells.bcadd(s, "result", &[("e", 1), ("s", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(1)],
        "Number of SOLIDs in the result of FUSE(s, e) operation",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(6)],
        "Number of EDGEs in the result of FUSE(s, e) operation",
    );
}

pub fn J5(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC28528");
    println!("========");
    println!();
    println!("Arguments: face and an edge");
    let geom_c = cylinder([0., 0., 0.], [0., 0., 1.], 10.);
    mkface_bounded(s, "f", &geom_c, (0., 2. * std::f64::consts::PI, -20., 20.));
    let l_curve = viso(&geom_c, 0.);
    mkedge(s, "e", &l_curve);
    bop.arguments(s, &["f", "e"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    println!("Results");
    println!("All split shapes");
    cells.bcaddall(s, "result", 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(2)],
        "Number of FACEs in the result containing all splits",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(5)],
        "Number of EDGEs in the result containing all splits",
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(394.159));
    checkprops(s, "result", Prop::Area, Expected::Mass(2513.27));
    println!("Cut face from edge");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("e", 1), ("f", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(0)],
        "Number of EDGEs in the result of CUT(e, f) operation",
    );
    println!("Cut edge from face");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f", 1), ("e", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(2)],
        "Number of FACE in the result of CUT(f, e) operation",
    );
    println!("Common between face and edge");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f", 1), ("e", 1)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(1)],
        "Number of EDGEs in the result of COMMON(f, e) operation",
    );
    println!("Fuse operation between face and edge");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f", 1)], 0, false);
    cells.bcadd(s, "result", &[("e", 1), ("s", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(2)],
        "Number of FACEs in the result of FUSE(f, e) operation",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(5)],
        "Number of EDGEs in the result of FUSE(f, e) operation",
    );
}

pub fn J6(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC28528");
    println!("========");
    println!();
    println!("Arguments: face and an edge");
    let geom_c = cylinder([0., 0., 0.], [0., 0., 1.], 10.);
    mkface_bounded(s, "f", &geom_c, (0., 2. * std::f64::consts::PI, -20., 20.));
    let l_curve = uiso(&geom_c, std::f64::consts::PI);
    mkedge_bounded(s, "e", &l_curve, -25., 25.);
    bop.arguments(s, &["f", "e"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    println!("Results");
    println!("All split shapes");
    cells.bcaddall(s, "result", 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(2)],
        "Number of FACEs in the result containing all splits",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(8)],
        "Number of EDGEs in the result containing all splits",
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(335.664));
    checkprops(s, "result", Prop::Area, Expected::Mass(2513.27));
    println!("Cut face from edge");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("e", 1), ("f", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(2)],
        "Number of EDGEs in the result of CUT(e, f) operation",
    );
    println!("Cut edge from face");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f", 1), ("e", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(2)],
        "Number of FACE in the result of CUT(f, e) operation",
    );
    println!("Common between face and edge");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f", 1), ("e", 1)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(1)],
        "Number of EDGEs in the result of COMMON(f, e) operation",
    );
    println!("Fuse operation between face and edge");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f", 1)], 0, false);
    cells.bcadd(s, "result", &[("e", 1), ("s", 0)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(2)],
        "Number of FACEs in the result of FUSE(f, e) operation",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(8)],
        "Number of EDGEs in the result of FUSE(f, e) operation",
    );
}

pub fn K1(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("==========");
    println!("OCC28245");
    println!("==========");
    println!();
    let c1_curve = circle_along([0., 0., 0.], [0., 0., 1.], 10.);
    let c2_curve = circle_along([10., 0., 0.], [0., 0., 1.], 10.);
    let c3_curve = circle_along([5., 9., 0.], [0., 0., 1.], 10.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    bop.arguments(s, &["f1", "f2", "f3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "rx");
    cells.bcadd(s, "r1", &[("f1", 1)], 1, true);
    cells.bcadd(s, "r1", &[("f2", 1)], 2, true);
    checknbshapes(s, "r1", Counting::Located, &[NbCount::Faces(2)]);
    checkprops(s, "r1", Prop::Area, Expected::Mass(505.482));
    cells.bcremoveall();
    cells.bcadd(s, "r2", &[("f1", 1)], 1, true);
    cells.bcadd(s, "r2", &[("f2", 1)], 1, true);
    checknbshapes(s, "r2", Counting::Located, &[NbCount::Faces(1)]);
    checkprops(s, "r2", Prop::Area, Expected::Mass(505.482));
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f1", 1)], 1, false);
    cells.bcadd(s, "result", &[("f2", 1), ("f3", 0)], 1, false);
    cells.bcadd(s, "result", &[("f3", 1), ("f1", 0)], 2, false);
    cells.bcremoveint(s, "result");
    require_bopcheck_ok(s, "result", "Error: result shape is self intersected");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(8), NbCount::Faces(2)],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(651.238));
}
