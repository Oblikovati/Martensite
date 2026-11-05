//! The `bugs_modalg_1` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn bug19071(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO CR33225 Linux: Error : The command is not valid. The area is 0.");
    println!("TODO CR33225 Linux: Error : The area of result shape is 0, expected 177462");
    println!("TODO CR33225 Linux: is WRONG because number of ");
    println!("============");
    println!("OCC19071");
    println!("============");
    println!();
    println!("Load first shape ...");
    restore(s, "OCC19071_Rotation_2.brep", "b1");
    println!("Load second shape ...");
    restore(s, "OCC19071_Torus_2.brep", "b2");
    println!("Prepare boolean operation ...");
    two_step.bop(s, "b1", "b2");
    println!("Start boolean operation ...");
    two_step.result(s, "result", Operation::Fuse);
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(177462.));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(6),
            NbCount::Edges(16),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(37),
        ],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "result", Prop::Area, "177244");
}
