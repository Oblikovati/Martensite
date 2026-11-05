//! The `cells_test` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn E3(s: &mut dyn Session) {
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
    cells.bcadd(
        s,
        "result",
        &[("s1", 1), ("s2", 1), ("s3", 1), ("s4", 1)],
        0,
        false,
    );
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(417.967));
}

pub fn E4(s: &mut dyn Session) {
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
    cells.bcadd(s, "result", &[("s1", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1416.31));
}

pub fn E5(s: &mut dyn Session) {
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
    cells.bcadd(s, "result", &[("s2", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(4993.56));
}

pub fn E6(s: &mut dyn Session) {
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
    cells.bcadd(s, "result", &[("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(5017.47));
}

pub fn E7(s: &mut dyn Session) {
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
    cells.bcadd(s, "result", &[("s4", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(2086.49));
}

pub fn E8(s: &mut dyn Session) {
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
    cells.bcremove(s, "result", &[("s1", 1), ("s2", 1), ("s3", 1), ("s4", 1)]);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(7311.75));
}

pub fn E9(s: &mut dyn Session) {
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
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 1)], 1, true);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(502.043));
}

pub fn F1(s: &mut dyn Session) {
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
    cells.bcadd(s, "result", &[("s1", 1), ("s3", 1)], 1, true);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(514.159));
}

pub fn F2(s: &mut dyn Session) {
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
    cells.bcadd(s, "result", &[("s1", 1), ("s4", 1)], 1, true);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(418.093));
}

pub fn F3(s: &mut dyn Session) {
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
    cells.bcadd(s, "result", &[("s2", 1), ("s3", 1)], 1, true);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1135.58));
}

pub fn F4(s: &mut dyn Session) {
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
    cells.bcadd(s, "result", &[("s2", 1), ("s4", 1)], 1, true);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1017.77));
}

pub fn F5(s: &mut dyn Session) {
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
    cells.bcadd(s, "result", &[("s3", 1), ("s4", 1)], 1, true);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1232.14));
}

pub fn F6(s: &mut dyn Session) {
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
    cells.bcadd(
        s,
        "result",
        &[("s1", 1), ("s2", 0), ("s3", 0), ("s4", 0)],
        0,
        false,
    );
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(259.774));
}

pub fn F7(s: &mut dyn Session) {
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
    cells.bcadd(
        s,
        "result",
        &[("s1", 0), ("s2", 1), ("s3", 0), ("s4", 0)],
        0,
        false,
    );
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(2067.69));
}

pub fn F8(s: &mut dyn Session) {
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
    cells.bcadd(
        s,
        "result",
        &[("s1", 0), ("s2", 0), ("s3", 1), ("s4", 0)],
        0,
        false,
    );
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1640.54));
}

pub fn F9(s: &mut dyn Session) {
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
    cells.bcadd(
        s,
        "result",
        &[("s1", 0), ("s2", 0), ("s3", 0), ("s4", 1)],
        0,
        false,
    );
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(145.291));
}

pub fn G1(s: &mut dyn Session) {
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
    cells.bcadd(
        s,
        "result",
        &[("s1", 1), ("s2", 0), ("s3", 0), ("s4", 0)],
        0,
        false,
    );
    cells.bcadd(
        s,
        "result",
        &[("s1", 0), ("s2", 1), ("s3", 0), ("s4", 0)],
        0,
        false,
    );
    cells.bcadd(
        s,
        "result",
        &[("s1", 0), ("s2", 0), ("s3", 1), ("s4", 0)],
        0,
        false,
    );
    cells.bcadd(
        s,
        "result",
        &[("s1", 0), ("s2", 0), ("s3", 0), ("s4", 1)],
        0,
        false,
    );
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(4113.3));
}
