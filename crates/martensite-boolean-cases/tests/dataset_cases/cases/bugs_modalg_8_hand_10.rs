//! `bugs_modalg_8` bodies written by hand, second file.
use crate::support::{
    beyond_script::printed_mass,
    bugs_commands::repair::restore_issue_input,
    bugs_commands::{
        geometry::frame_along,
        options_bop::ScriptBop,
        repair::pcone_on,
        view::{don, fit},
    },
    checkview::{CheckView, checkview},
    commands::{explode, pcylinder, prism},
    construction::mkplane,
    nbshapes::{Counting, NbCount, checknbshapes, shape_counts},
    properties::Prop,
    session::Session,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug33615(s: &mut dyn Session) {
    println!("================================");
    println!("0033615: Modeling Algorithms - Partition algorithm creates unexpected vertices");
    println!("================================");
    println!();
    pcone_on(
        s,
        "cone",
        frame_along([-5., 0., 4.], [-1., 0., 0.]),
        3.,
        1.,
        10.,
    );
    pcylinder(s, "cylinder", 10., 20.);
    explode(s, "cylinder", Some(ShapeEnum::Face));
    explode(s, "cone", Some(ShapeEnum::Face));
    don(s, &["cylinder_1", "cone_1"]);
    fit(s);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["cone_1"]);
    bop.add_tools(s, &["cylinder_1"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    let counts = [
        NbCount::Vertices(5),
        NbCount::Edges(8),
        NbCount::Wires(5),
        NbCount::Faces(4),
    ];
    checknbshapes(s, "result", Counting::Located, &counts);
    checkview(s, &CheckView::of("result"));
}

/// The input of this script is not in the dataset; it is the one of the issue it comes from.
pub fn bug_gh544(s: &mut dyn Session) {
    restore_issue_input(s, "OCCT-544/solid.brep.txt", "body");
    explode(s, "body", Some(ShapeEnum::Wire));
    mkplane(s, "f", "body_18");
    prism(s, "p", "f", [0., 0., 12.]);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["p"]);
    bop.add_tools(s, &["body"]);
    bop.bfillds();
    bop.simplify_faces = true;
    bop.bapibop(s, "res_simple", Operation::Fuse);
    // Beyond the script: the native result, as an earlier test checked.
    let counts = shape_counts(s, "res_simple", Counting::Located);
    assert_eq!(counts[..6], [27, 51, 19, 17, 1, 1], "counts of res_simple");
    assert_eq!(counts[7], 1, "compounds of res_simple");
    printed_mass(s, "res_simple", Prop::Volume, "37455.4");
    printed_mass(s, "res_simple", Prop::Area, "23042.1");
}
