//! The `bugs_modalg_5` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn bug25354_34(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    ttranslate(s, "b2", [5., -10., -4.]);
    trotate(s, "b2", [5., 0., 0.], [0., 0., 1.], -45.);
    trotate(s, "b2", [5., 0., 0.], [1., 0., 0.], 45.);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(1),
            NbCount::Edges(0),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(2),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_35(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    ttranslate(s, "b2", [-10., -10., -4.]);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(2),
            NbCount::Edges(1),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(4),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_36(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    ttranslate(s, "b2", [5., -10., -4.]);
    trotate(s, "b2", [5., 0., 0.], [0., 0., 1.], -45.);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(2),
            NbCount::Edges(1),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(4),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_37(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    ttranslate(s, "b2", [10., 3., 4.]);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(4),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(9),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_38(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    ttranslate(s, "b2", [5., 3., 4.]);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(6),
            NbCount::Edges(6),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(13),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_39(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    vertex(s, "b1", [0., 0., 0.]);
    vertex(s, "b2", [0., 0., 0.]);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(1),
            NbCount::Edges(0),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(2),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_40(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    vertex(s, "b1", [4., 0., 0.]);
    vertex(s, "ba", [0., 0., 0.]);
    vertex(s, "bb", [10., 0., 0.]);
    edge(s, "b2", "ba", "bb");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(1),
            NbCount::Edges(0),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(2),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_41(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    vertex(s, "b1", [0., 0., 0.]);
    vertex(s, "ba", [0., 0., 0.]);
    vertex(s, "bb", [10., 0., 0.]);
    edge(s, "b2", "ba", "bb");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(1),
            NbCount::Edges(0),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(2),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_45(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    vertex(s, "b1", [0., 0., 0.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(1),
            NbCount::Edges(0),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(2),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_46(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    vertex(s, "b1", [4., 0., 0.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(1),
            NbCount::Edges(0),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(2),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_47(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    vertex(s, "b1", [4., 3., 0.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(1),
            NbCount::Edges(0),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(2),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_48(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    vertex(s, "b1", [4., 3., 2.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(0),
            NbCount::Edges(0),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25715_1(s: &mut dyn Session) {
    println!("==========");
    println!("OCC25715");
    println!("==========");
    println!();
    pcylinder(s, "s1", 0.069, 0.6);
    pcylinder(s, "s2", 0.024, 0.4);
    trotate(s, "s1", [0., 0., 0.], [0., 0., 1.], 90.);
    trotate(s, "s2", [0., 0., 0.], [0., 1., 0.], 90.);
    trotate(s, "s2", [0., 0., 0.], [1., 0., 0.], -45.);
    ttranslate(s, "s2", [-0.2, 0., 0.48]);
    bcut(s, "result", "s1", "s2");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(6),
            NbCount::Wires(6),
            NbCount::Faces(4),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(23),
        ],
    );
    checkview(s, &CheckView::of("result"));
}
