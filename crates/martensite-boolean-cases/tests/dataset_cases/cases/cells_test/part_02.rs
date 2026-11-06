//! The `cells_test` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn C3(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f1", 1), ("f2", 0), ("f3", 1)], 0, false);
    cells.bcadd(s, "result", &[("f1", 0), ("f2", 1), ("f3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(351.984));
}

pub fn C4(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f1", 1), ("f2", 1), ("f3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(122.886));
}

pub fn C5(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f1", 1), ("f3", 1)], 1, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(298.878));
}

pub fn C6(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f2", 1), ("f3", 1)], 1, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(298.878));
}

pub fn C7(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f1", 1), ("f3", 1)], 1, false);
    cells.bcadd(s, "result", &[("f2", 1), ("f3", 1)], 1, false);
    cells.bcremoveint(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(474.87));
}

pub fn C8(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f1", 0), ("f2", 0), ("f3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(231.988));
}

pub fn C9(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f2", 0), ("f3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(407.98));
}

pub fn D1(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f1", 0), ("f3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(407.98));
}

pub fn D2(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f2", 0), ("f3", 1)], 0, false);
    cells.bcadd(s, "result", &[("f1", 0), ("f3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(583.972));
}

pub fn D3(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(706.858));
}

pub fn D4(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcaddall(s, "result", 0, false);
    cells.bcremove(s, "result", &[("f1", 1), ("f2", 1), ("f3", 1)]);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1367.94));
}

pub fn D5(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    let c1_curve = circle([0., 10., 0.], 15.);
    let c2_curve = circle([20., 10., 0.], 15.);
    let c3_curve = circle([10., 0., 0.], 15.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    mkedge(s, "e3", &c3_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    wire(s, "w3", &["e3"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    s.view(ViewStep::Donly(&["f1", "f2", "f3"]));
    s.view(ViewStep::Fit);
    bop.arguments(s, &["f1", "f2"], &["f3"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f1", 1), ("f2", 0), ("f3", 0)], 1, false);
    cells.bcadd(s, "result", &[("f1", 1), ("f2", 0), ("f3", 1)], 1, false);
    cells.bcadd(s, "result", &[("f1", 1), ("f2", 1), ("f3", 0)], 2, false);
    cells.bcadd(s, "result", &[("f1", 1), ("f2", 1), ("f3", 1)], 2, false);
    cells.bcadd(s, "result", &[("f1", 0), ("f2", 1), ("f3", 1)], 2, false);
    cells.bcremoveint(s, "result");
    require_bopcheck_ok(s, "result", "Error: result shape is self intersected");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(882.85));
}

pub fn E1(s: &mut dyn Session) {
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
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(7729.72));
}

pub fn E2(s: &mut dyn Session) {
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
    cells.bcaddall(s, "result", 1, true);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(2689.14));
}
