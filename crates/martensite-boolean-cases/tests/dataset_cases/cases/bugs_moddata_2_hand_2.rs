//! `bugs_moddata_2`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::options_bop::{SectionFlags, bsection_with},
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    properties::Prop,
    restore::restore,
    session::Session,
    surface_commands::{mkface_bounded, plane},
};

pub fn bug75_2(s: &mut dyn Session) {
    println!("============");
    println!("BUC60861");
    println!("OCC75");
    println!("============");
    println!();
    println!("case 2");
    restore(s, "BUC60861_gap1.brep", "ss");
    checkshape_named(s, "ss");
    let var_pl1 = plane([25680.2288, 21428., 9995.], [1., 0., 0.]);
    mkface_bounded(s, "f1", &var_pl1, (-1., 11., -5., 5.));
    bsection_with(
        s,
        "result",
        ("ss", "f1"),
        SectionFlags {
            pcurve_on1: false,
            pcurve_on2: false,
            approximation: true,
        },
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(10.0009));
    checkview(s, &CheckView::of("result"));
}
