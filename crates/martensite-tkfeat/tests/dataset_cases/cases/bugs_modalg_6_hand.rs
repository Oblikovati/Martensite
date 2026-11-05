//! The `bugs/modalg_6` scripts that branch on what a command answers or use a command's answer.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{explode, psphere},
    feat_script::FeatScript,
    inspect::firsthole,
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bsection,
    properties::Prop,
    restore::restore,
    session::{Session, ViewStep},
    splitshape::splitshape,
    surface_variables::{mkface_bounded, plane},
    tolerance_check::{checkreal, tolerance_report},
};
use martensite_tkg3d::top_abs::ShapeEnum;

fn banner(title: &str) {
    println!("============");
    println!("{title}");
    println!("============");
    println!();
}

pub fn bug22037(s: &mut FeatScript<'_>) {
    banner("22037");
    restore(s, "bug22037a.brep", "a");
    restore(s, "bug22037b.brep", "b");
    let mut two_step = std::mem::take(&mut s.two_step);
    two_step.bop(s, "a", "b");
    two_step.section(s, "edges");
    s.two_step = two_step;
    explode(s, "edges", None);
    splitshape(s, "result", "b", &["edges_1", "edges_2"], &[]);
    checkshape_named(s, "result");
    tolerance_report(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    checknbshapes(s, "result", Counting::Located, &[NbCount::Faces(2)]);
    checkview(s, &CheckView::of("result"));
}

fn bug27045(s: &mut FeatScript<'_>, at: [f64; 3], along: [f64; 3], radius: f64, area: f64) {
    println!("==========");
    println!("27045");
    println!("==========");
    println!();
    // The script's `dall` finds nothing bound yet.
    restore(s, "bug27045_bean.brep", "a");
    firsthole(s, "r", "a", at, along, radius);
    checkprops(s, "r", Prop::Area, Expected::Mass(area));
    checkshape_named(s, "r");
}

pub fn bug27045_1(s: &mut FeatScript<'_>) {
    let (at, along) = (
        [7.31762745781211, 0., 22.6223587092621],
        [-0.309016994374947, -0., -0.951056516295154],
    );
    bug27045(s, at, along, 5., 21558.2);
}

pub fn bug27045_2(s: &mut FeatScript<'_>) {
    bug27045(s, [5., 0., 22.], [0., 0., -1.], 2., 21543.3);
}

pub fn bug28094(s: &mut FeatScript<'_>) {
    banner("28094");
    s.view(ViewStep::SmallView);
    psphere(s, "a", 10.);
    plane(s, "p2", [2., 0., 0.], [1., 0., 0.]);
    mkface_bounded(s, "f2", "p2", (-100., 100., -100., 100.));
    bsection(s, "s2", "a", "f2");
    explode(s, "a", Some(ShapeEnum::Face));
    explode(s, "s2", Some(ShapeEnum::Edge));
    splitshape(s, "res", "a_1", &["a_1", "s2_1", "a_1", "s2_2"], &[]);
    let largest = tolerance_report(s, "res");
    checkreal("MaxTolerance", &largest, "2.e-07", 1.0e-6, 1.0e-6);
    s.view(ViewStep::Clear);
    s.view(ViewStep::Donly(&["res"]));
    s.view(ViewStep::Fit);
    s.view(ViewStep::Xwd("photo"));
}
