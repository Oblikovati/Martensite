//! `bugs_modalg_7`: each statement is one line of its script.
use crate::support::{
    bop_session::BopSession,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{explode, pcylinder, trotate, ttranslate},
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_named},
    one_step::{bcommon, bcut, bfuse},
    properties::Prop,
    restore::restore,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub use super::bugs_modalg_7_hand::*;
pub use super::bugs_modalg_7_hand_2::*;
pub use super::bugs_modalg_7_hand_3::*;
pub use super::bugs_modalg_7_hand_4::*;
pub use super::bugs_modalg_7_hand_5::*;
pub use super::bugs_modalg_7_hand_6::*;
pub use super::bugs_modalg_7_hand_9::*;
pub use super::bugs_modalg_7_hand_10::*;
pub use super::bugs_modalg_7_hand_11::*;
pub use super::bugs_modalg_7_hand_12::*;
pub use super::bugs_modalg_7_hand_13::*;

pub fn bug22886(s: &mut dyn Session) {
    println!("============");
    println!("OCC22886: Bug in boolean");
    println!("============");
    println!();
    restore(s, "bug22886_solid1.brep", "solid1");
    restore(s, "bug22886_solid2.brep", "solid2");
    bcommon(s, "result", "solid1", "solid2");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.40191e+09));
    checkprops(s, "result", Prop::Volume, Expected::Mass(3.59571e+12));
    checknbshapes_named(
        s,
        "result",
        Counting::Typed,
        &[
            NbCount::Vertices(78),
            NbCount::Edges(135),
            NbCount::Wires(56),
            NbCount::Faces(56),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
        "Common operation",
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25968(s: &mut dyn Session) {
    println!("============");
    println!("OCC25968");
    println!("============");
    println!();
    pcylinder(s, "Cylinder004", 50., 1000.);
    pcylinder(s, "Cylinder005", 45., 1000.);
    bcut(s, "Cut003", "Cylinder004", "Cylinder005");
    pcylinder(s, "Cylinder006", 45., 102.);
    trotate(s, "Cylinder006", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder006", [0., 51., 1000.]);
    bcut(s, "Cut004", "Cut003", "Cylinder006");
    pcylinder(s, "Cylinder007", 45., 102.);
    trotate(s, "Cylinder007", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder007", [0., 51., 0.]);
    bcut(s, "Cut005", "Cut004", "Cylinder007");
    ttranslate(s, "Cut005", [0., 110., 0.]);
    pcylinder(s, "Cylinder044", 50., 800.);
    trotate(s, "Cylinder044", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder044", [0., 610., 0.]);
    pcylinder(s, "Cylinder045", 45., 795.);
    trotate(s, "Cylinder045", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder045", [0., 605., 0.]);
    bcut(s, "Cut032", "Cylinder044", "Cylinder045");
    pcylinder(s, "Cylinder046", 45., 55.);
    pcylinder(s, "Cylinder047", 45., 92.);
    trotate(s, "Cylinder047", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder047", [0., 46., 0.]);
    bcut(s, "Cut033", "Cylinder046", "Cylinder047");
    bcut(s, "Cut039", "Cut032", "Cut033");
    pcylinder(s, "Cylinder048", 45., 55.);
    pcylinder(s, "Cylinder049", 45., 92.);
    trotate(s, "Cylinder049", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder049", [0., 46., 0.]);
    bcut(s, "Cut034", "Cylinder048", "Cylinder049");
    ttranslate(s, "Cut034", [0., 110., 0.]);
    bcut(s, "Cut040", "Cut039", "Cut034");
    pcylinder(s, "Cylinder050", 45., 55.);
    pcylinder(s, "Cylinder051", 45., 92.);
    trotate(s, "Cylinder051", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder051", [0., 46., 0.]);
    bcut(s, "Cut035", "Cylinder050", "Cylinder051");
    ttranslate(s, "Cut035", [0., 220., 0.]);
    bcut(s, "Cut041", "Cut040", "Cut035");
    pcylinder(s, "Cylinder052", 45., 55.);
    pcylinder(s, "Cylinder053", 45., 92.);
    trotate(s, "Cylinder053", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder053", [0., 46., 0.]);
    bcut(s, "Cut036", "Cylinder052", "Cylinder053");
    ttranslate(s, "Cut036", [0., 330., 0.]);
    bcut(s, "Cut042", "Cut041", "Cut036");
    pcylinder(s, "Cylinder054", 45., 55.);
    pcylinder(s, "Cylinder055", 45., 92.);
    trotate(s, "Cylinder055", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder055", [0., 46., 0.]);
    bcut(s, "Cut037", "Cylinder054", "Cylinder055");
    ttranslate(s, "Cut037", [0., 440., 0.]);
    bcut(s, "Cut043", "Cut042", "Cut037");
    pcylinder(s, "Cylinder056", 45., 55.);
    pcylinder(s, "Cylinder057", 45., 92.);
    trotate(s, "Cylinder057", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder057", [0., 46., 0.]);
    bcut(s, "Cut038", "Cylinder056", "Cylinder057");
    ttranslate(s, "Cut038", [0., 550., 0.]);
    bcut(s, "Cut044", "Cut043", "Cut038");
    pcylinder(s, "Cylinder058", 45., 102.);
    trotate(s, "Cylinder058", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "Cylinder058", [-51., -190., 0.]);
    bcut(s, "Cut045", "Cut044", "Cylinder058");
    trotate(s, "Cut045", [0., 0., 0.], [0., 1., 0.], 180.);
    ttranslate(s, "Cut045", [0., 0., 1000.]);
    pcylinder(s, "Cylinder", 50., 1000.);
    pcylinder(s, "Cylinder001", 45., 1000.);
    bcut(s, "Cut", "Cylinder", "Cylinder001");
    pcylinder(s, "Cylinder002", 45., 102.);
    trotate(s, "Cylinder002", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder002", [0., 51., 1000.]);
    bcut(s, "Cut001", "Cut", "Cylinder002");
    pcylinder(s, "Cylinder003", 45., 102.);
    trotate(s, "Cylinder003", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "Cylinder003", [0., 51., 0.]);
    bcut(s, "Cut002", "Cut001", "Cylinder003");
    bfuse(s, "Fusion", "Cut045", "Cut002");
    bfuse(s, "Fusion001", "Cut005", "Fusion");
    checkshape_named(s, "Fusion001");
    checkprops(s, "Fusion001", Prop::Area, Expected::Mass(1.52134e+006));
    checknbshapes(
        s,
        "Fusion001",
        Counting::Located,
        &[
            NbCount::Vertices(73),
            NbCount::Edges(111),
            NbCount::Wires(42),
            NbCount::Faces(32),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
}

pub fn bug26795(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    println!("================");
    println!("OCC26795");
    println!("================");
    println!();
    restore(s, "bug26795_c.brep", "c");
    explode(s, "c", Some(ShapeEnum::Face));
    bop.arguments(s, &["c_1"], &["c_2"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(38958.7));
    checkprops(s, "result", Prop::Area, Expected::Mass(1.88559e+007));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Wires(4), NbCount::Faces(3)],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug27948(s: &mut dyn Session) {
    println!("========");
    println!("OCC27948");
    println!("========");
    println!();
    restore(s, "bug27948_a.brep", "a");
    restore(s, "bug27948_b.brep", "b");
    bcut(s, "result", "a", "b");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(135.137));
    checkprops(s, "result", Prop::Volume, Expected::Mass(40.573));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Wires(19),
            NbCount::Faces(17),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug29038(s: &mut dyn Session) {
    println!("=======");
    println!("0029038");
    println!("=======");
    println!();
    restore(s, "bug29038_bfuse_hang.brep", "a");
    explode(s, "a", None);
    bfuse(s, "r", "a_1", "a_2");
}

pub fn bug29968(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("========");
    println!(
        "0029968: Modeling Algorithms - BRepAlgoAPICut produces an empty result with half-space"
    );
    println!("========");
    println!();
    restore(s, "bug29968_h.brep", "h");
    restore(s, "bug29968_s.brep", "s");
    two_step.bop(s, "s", "h");
    two_step.result(s, "rcom", Operation::Common);
    checkshape_named(s, "rcom");
    checknbshapes(
        s,
        "rcom",
        Counting::Located,
        &[NbCount::Wires(1), NbCount::Faces(1)],
    );
    checkprops(s, "rcom", Prop::Area, Expected::Mass(12348.));
    two_step.result(s, "rcut", Operation::Cut);
    checkshape_named(s, "rcut");
    checknbshapes(
        s,
        "rcut",
        Counting::Located,
        &[NbCount::Wires(1), NbCount::Faces(1)],
    );
    checkprops(s, "rcut", Prop::Area, Expected::Mass(12348.));
    two_step.section(s, "rsec");
    checkshape_named(s, "rsec");
    checknbshapes(
        s,
        "rsec",
        Counting::Located,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
    checkprops(s, "rsec", Prop::Length, Expected::Mass(117.6));
}
