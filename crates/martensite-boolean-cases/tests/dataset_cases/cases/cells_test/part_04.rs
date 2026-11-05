//! The `cells_test` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn G2(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    make_box(s, "s1", [0., 0., 0.], [10., 10., 10.]);
    psphere(s, "s2", 12.);
    pcylinder(s, "s3", 10., 20.);
    pcone(s, "s4", 10., 0., 30.);
    bop.arguments(s, &["s1", "s2", "s3", "s4"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 1)], 1, false);
    cells.bcremove(s, "result", &[("s1", 1), ("s2", 0), ("s3", 0), ("s4", 0)]);
    cells.bcremoveint(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(545.266));
}

pub fn G3(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    make_box(s, "s1", [0., 0., 0.], [10., 10., 10.]);
    psphere(s, "s2", 12.);
    pcylinder(s, "s3", 10., 20.);
    pcone(s, "s4", 10., 0., 30.);
    bop.arguments(s, &["s1", "s2", "s3", "s4"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s2", 1)], 1, false);
    cells.bcremove(s, "result", &[("s1", 0), ("s2", 1), ("s3", 0), ("s4", 0)]);
    cells.bcremoveint(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1166.69));
}

pub fn G4(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    make_box(s, "s1", [0., 0., 0.], [10., 10., 10.]);
    psphere(s, "s2", 12.);
    pcylinder(s, "s3", 10., 20.);
    pcone(s, "s4", 10., 0., 30.);
    bop.arguments(s, &["s1", "s2", "s3", "s4"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s3", 1)], 1, false);
    cells.bcremove(s, "result", &[("s1", 0), ("s2", 0), ("s3", 1), ("s4", 0)]);
    cells.bcremoveint(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1393.05));
}

pub fn G5(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    make_box(s, "s1", [0., 0., 0.], [10., 10., 10.]);
    psphere(s, "s2", 12.);
    pcylinder(s, "s3", 10., 20.);
    pcone(s, "s4", 10., 0., 30.);
    bop.arguments(s, &["s1", "s2", "s3", "s4"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s4", 1)], 1, false);
    cells.bcremove(s, "result", &[("s1", 0), ("s2", 0), ("s3", 0), ("s4", 1)]);
    cells.bcremoveint(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1232.14));
}

pub fn G6(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    make_box(s, "s1", [0., 0., 0.], [10., 10., 10.]);
    psphere(s, "s2", 12.);
    pcylinder(s, "s3", 10., 20.);
    pcone(s, "s4", 10., 0., 30.);
    bop.arguments(s, &["s1", "s2", "s3", "s4"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcaddall(s, "result", 1, false);
    cells.bcremove(s, "result", &[("s1", 1), ("s2", 0), ("s3", 0), ("s4", 0)]);
    cells.bcremove(s, "result", &[("s1", 0), ("s2", 1), ("s3", 0), ("s4", 0)]);
    cells.bcremove(s, "result", &[("s1", 0), ("s2", 0), ("s3", 1), ("s4", 0)]);
    cells.bcremove(s, "result", &[("s1", 0), ("s2", 0), ("s3", 0), ("s4", 1)]);
    cells.bcremoveint(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1424.16));
}

pub fn G7(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    make_box(s, "s1", [0., 0., 0.], [10., 10., 10.]);
    psphere(s, "s2", 12.);
    pcylinder(s, "s3", 10., 20.);
    pcone(s, "s4", 10., 0., 30.);
    bop.arguments(s, &["s1", "s2", "s3", "s4"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 1)], 1, false);
    cells.bcadd(s, "result", &[("s2", 1)], 2, false);
    cells.bcadd(s, "result", &[("s3", 1)], 3, false);
    cells.bcadd(s, "result", &[("s4", 1)], 4, false);
    cells.bcremoveint(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(4326.81));
}

pub fn G8(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    make_box(s, "s1", [0., 0., 0.], [10., 10., 10.]);
    psphere(s, "s2", 12.);
    pcylinder(s, "s3", 10., 20.);
    pcone(s, "s4", 10., 0., 30.);
    bop.arguments(s, &["s1", "s2", "s3", "s4"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcaddall(s, "result", 0, false);
    cells.bcmakecontainers(s, "result");
    cells.bcremove(s, "result", &[("s1", 1)]);
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(7299.17));
}

pub fn H1(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 30., 0.], [10., 10., 10.]);
    compound(s, &["b1", "b2"], "b1");
    make_box(s, "b3", [5., 5., 5.], [10., 10., 10.]);
    make_box(s, "b4", [30., 5., 5.], [10., 10., 10.]);
    compound(s, &["b3", "b4"], "b2");
    s.view(ViewStep::Disp(&["b1", "b2"]));
    bop.arguments(s, &["b1", "b2"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b1", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1350.));
}

pub fn H2(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 30., 0.], [10., 10., 10.]);
    compound(s, &["b1", "b2"], "b1");
    make_box(s, "b3", [5., 5., 5.], [10., 10., 10.]);
    make_box(s, "b4", [30., 5., 5.], [10., 10., 10.]);
    compound(s, &["b3", "b4"], "b2");
    s.view(ViewStep::Disp(&["b1", "b2"]));
    bop.arguments(s, &["b1", "b2"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b2", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1350.));
}

pub fn H3(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 30., 0.], [10., 10., 10.]);
    compound(s, &["b1", "b2"], "b1");
    make_box(s, "b3", [5., 5., 5.], [10., 10., 10.]);
    make_box(s, "b4", [30., 5., 5.], [10., 10., 10.]);
    compound(s, &["b3", "b4"], "b2");
    s.view(ViewStep::Disp(&["b1", "b2"]));
    bop.arguments(s, &["b1", "b2"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b1", 1), ("b2", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(150.));
}

pub fn H4(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 30., 0.], [10., 10., 10.]);
    compound(s, &["b1", "b2"], "b1");
    make_box(s, "b3", [5., 5., 5.], [10., 10., 10.]);
    make_box(s, "b4", [30., 5., 5.], [10., 10., 10.]);
    compound(s, &["b3", "b4"], "b2");
    s.view(ViewStep::Disp(&["b1", "b2"]));
    bop.arguments(s, &["b1", "b2"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b1", 1), ("b2", 0)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1200.));
}

pub fn I1(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    pcylinder(s, "s1", 10., 20.);
    let geom_p = plane_along([0., 0., 0.], [1., 0., 0.]);
    mkface(s, "f", &geom_p.surface);
    bop.arguments(s, &["s1", "f"], &[]);
    bop.bfillds();
    bop.bbuild(s, "r");
    explode(s, "r", Some(ShapeEnum::Solid));
    copy(s, "r_1", "s1");
    copy(s, "r_2", "s2");
    pcylinder(s, "s3", 5., 20.);
    ttranslate(s, "s3", [0., -10., 0.]);
    s.view(ViewStep::Donly(&["s1", "s2", "s3"]));
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 0), ("s2", 1), ("s3", 0)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1238.14));
}

pub fn I2(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    pcylinder(s, "s1", 10., 20.);
    let geom_p = plane_along([0., 0., 0.], [1., 0., 0.]);
    mkface(s, "f", &geom_p.surface);
    bop.arguments(s, &["s1", "f"], &[]);
    bop.bfillds();
    bop.bbuild(s, "r");
    explode(s, "r", Some(ShapeEnum::Solid));
    copy(s, "r_1", "s1");
    copy(s, "r_2", "s2");
    pcylinder(s, "s3", 5., 20.);
    ttranslate(s, "s3", [0., -10., 0.]);
    s.view(ViewStep::Donly(&["s1", "s2", "s3"]));
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcaddall(s, "result", 1, false);
    cells.bcremove(s, "result", &[("s1", 0), ("s2", 1), ("s3", 0)]);
    cells.bcadd(s, "result", &[("s1", 0), ("s2", 1), ("s3", 0)], 0, false);
    cells.bcremoveint(s, "result");
    cells.bcmakecontainers(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(2998.06));
}

pub fn J1(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC28528");
    println!("========");
    println!();
    println!("Arguments: solid and a face");
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    let geom_p = plane_along([5., 5., 5.], [1., 0., 0.]);
    mkface_bounded(s, "f", &geom_p.surface, (-10., 10., -10., 10.));
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
        &[NbCount::Solids(2)],
        "Number of SOLIDs in the result containing all splits",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(12)],
        "Number of FACEs in the result containing all splits",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(1200.));
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
        &[NbCount::Solids(2)],
        "Number of SOLIDs in the result of CUT(b, f) operation",
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
        &[NbCount::Faces(11)],
        "Number of FACEs in the result of FUSE(b, f) operation",
    );
}
