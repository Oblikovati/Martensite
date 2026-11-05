//! The `cells_test` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A1(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 0), ("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1345.27));
}

pub fn A2(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 0), ("s2", 1), ("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(1345.27));
}

pub fn A3(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 0), ("s3", 1)], 0, false);
    cells.bcadd(s, "result", &[("s1", 0), ("s2", 1), ("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["r4"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(2690.54));
}

pub fn A4(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 1), ("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(769.835));
}

pub fn A5(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 1), ("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(2115.11));
}

pub fn A6(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s2", 1), ("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(2115.11));
}

pub fn A7(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 1), ("s3", 1)], 0, false);
    cells.bcadd(s, "result", &[("s2", 1), ("s3", 2)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(3460.38));
}

pub fn A8(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 0), ("s2", 0), ("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(2356.19));
}

pub fn A9(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s2", 0), ("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(3701.46));
}

pub fn B1(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 0), ("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(3701.46));
}

pub fn B2(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s2", 0), ("s3", 1)], 0, false);
    cells.bcadd(s, "result", &[("s1", 0), ("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(5046.74));
}

pub fn B3(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(5816.57));
}

pub fn B4(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcaddall(s, "result", 0, false);
    cells.bcremove(s, "result", &[("s1", 1), ("s2", 1), ("s3", 1)]);
    cells.bcremove(s, "result", &[("s2", 0), ("s3", 1)]);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(7172.78));
}

pub fn B5(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 0), ("s3", 0)], 1, false);
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 0), ("s3", 1)], 1, false);
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 1), ("s3", 0)], 2, false);
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 1), ("s3", 1)], 2, false);
    cells.bcadd(s, "result", &[("s1", 0), ("s2", 1), ("s3", 1)], 2, false);
    cells.bcremoveint(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(4494.64));
}

pub fn B6(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC26798");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    psphere(s, "s1", 15.);
    psphere(s, "s2", 15.);
    psphere(s, "s3", 15.);
    ttranslate(s, "s1", [0., 0., 10.]);
    ttranslate(s, "s2", [20., 0., 10.]);
    ttranslate(s, "s3", [10., 0., 0.]);
    bop.arguments(s, &["s1", "s2", "s3"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 0), ("s3", 0)], 1, false);
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 0), ("s3", 1)], 1, true);
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 1), ("s3", 0)], 2, false);
    cells.bcadd(s, "result", &[("s1", 1), ("s2", 1), ("s3", 1)], 2, false);
    cells.bcadd(s, "result", &[("s1", 0), ("s2", 1), ("s3", 1)], 2, true);
    cells.bcremoveint(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(4494.64));
}

pub fn C1(s: &mut dyn Session) {
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
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(175.992));
}

pub fn C2(s: &mut dyn Session) {
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
    cells.bcadd(s, "result", &[("f1", 0), ("f2", 1), ("f3", 1)], 0, false);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    checkprops(s, "result", Prop::Area, Expected::Mass(175.992));
}
