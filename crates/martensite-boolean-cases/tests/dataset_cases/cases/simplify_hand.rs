//! `simplify` bodies written by hand: their scripts cut along a curve in a loop.
use crate::support::{
    beyond_script::part_count,
    bop_session::BopSession,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{explode, make_box, pcylinder, ttranslate},
    construction::copy,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    session::Session,
    surface_commands::mkcurve,
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// `nbp` is the number of tools both scripts cut with.
const NBP: usize = 100;

/// The body of the scripts' loop: the arguments, `bfillds` and `bapibop plate 2`.
fn cut_plate_by_tool(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    bop.arguments(s, &["plate"], &["tool"]);
    bop.bfillds();
    bop.bapibop_cut_simplified(s, "plate");
}

pub fn A3(s: &mut dyn Session) {
    pcylinder(s, "plate", 50., 10.);
    explode(s, "plate", Some(ShapeEnum::Edge));
    copy(s, "plate_1", "e");
    let curve = mkcurve(s, "e");
    let radius = 10.;
    let delta = (curve.last - curve.first) / NBP as f64;
    for i in 0..NBP {
        let t = curve.first + delta * i as f64;
        let [x, y, z] = curve.value(t);
        pcylinder(s, "tool", radius, 10.);
        ttranslate(s, "tool", [x, y, z]);
        ttranslate(s, "tool", [0., 0., -5.]);
        cut_plate_by_tool(s);
    }
    checkshape_named(s, "plate");
    checknbshapes(
        s,
        "plate",
        Counting::Located,
        &[
            NbCount::Vertices(202),
            NbCount::Edges(303),
            NbCount::Wires(105),
            NbCount::Faces(104),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "plate", Prop::Area, Expected::Mass(18541.7));
    checkprops(s, "plate", Prop::Volume, Expected::Mass(64444.2));
    checkview(s, &CheckView::of("plate"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "plate", Some(ShapeEnum::Edge), 303);
}

pub fn A4(s: &mut dyn Session) {
    make_box(s, "plate", [0., 0., 0.], [100., 20., 20.]);
    explode(s, "plate", Some(ShapeEnum::Edge));
    let curve = mkcurve(s, "plate_10");
    let delta = (curve.last - 10. - curve.first) / NBP as f64;
    for i in 0..NBP {
        let t = curve.first + delta * i as f64;
        let [x, y, z] = curve.value(t);
        make_box(s, "tool", [x, y, z], [10., 10., 10.]);
        ttranslate(s, "tool", [0., 0., -5.]);
        cut_plate_by_tool(s);
        checkshape_named(s, "plate");
        // The number of faces is the same from one operation to the next.
        checknbshapes(
            s,
            "plate",
            Counting::Located,
            &[
                NbCount::Vertices(14),
                NbCount::Edges(21),
                NbCount::Wires(9),
                NbCount::Faces(9),
                NbCount::Shells(1),
                NbCount::Solids(1),
            ],
        );
    }
    checkshape_named(s, "plate");
    checkprops(s, "plate", Prop::Area, Expected::Mass(8800.));
    checkprops(s, "plate", Prop::Volume, Expected::Mass(35045.));
    checkview(s, &CheckView::of("plate"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "plate", Some(ShapeEnum::Edge), 21);
}
