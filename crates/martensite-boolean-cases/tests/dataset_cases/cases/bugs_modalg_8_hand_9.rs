//! `bugs_modalg_8` bodies written by hand.
use crate::support::{
    bugs_commands::inspect::checkmaxtol_value,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::pcone,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;

pub fn bug32721(s: &mut dyn Session) {
    println!("======================================================");
    println!("OCC32721: BOP wrong results on a cone and an extrusion");
    println!("======================================================");
    println!();
    restore(s, "bug32721.brep", "prism");
    pcone(s, "cone", 6., 0., 10.);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "cone", "prism");
    two_step.result(s, "r1", Operation::Fuse);
    two_step.result(s, "r2", Operation::Common);
    two_step.result(s, "r3", Operation::Cut);
    two_step.result(s, "r4", Operation::Cut21);
    let rows = [
        ("r1", [8, 17, 8, 8, 1, 1]),
        ("r2", [3, 7, 4, 4, 1, 1]),
        ("r3", [4, 10, 4, 4, 1, 1]),
        ("r4", [7, 14, 8, 8, 2, 2]),
    ];
    for (name, [vertices, edges, wires, faces, shells, solids]) in rows {
        checkshape_named(s, name);
        let counts = [
            NbCount::Vertices(vertices),
            NbCount::Edges(edges),
            NbCount::Wires(wires),
            NbCount::Faces(faces),
            NbCount::Shells(shells),
            NbCount::Solids(solids),
        ];
        checknbshapes(s, name, Counting::Typed, &counts);
    }
    if checkmaxtol_value(s, "r1") > 0.0002 {
        println!("Error: bad tolerance of result");
    }
    let masses = [
        ("r1", 388.634, 406.357),
        ("r2", 57.8605, 22.8116),
        ("r3", 358.735, 354.179),
        ("r4", 87.7592, 29.3659),
    ];
    for (name, area, volume) in masses {
        checkprops(s, name, Prop::Area, Expected::Mass(area));
        checkprops(s, name, Prop::Volume, Expected::Mass(volume));
    }
    checkview(s, &CheckView::of("r1"));
}
