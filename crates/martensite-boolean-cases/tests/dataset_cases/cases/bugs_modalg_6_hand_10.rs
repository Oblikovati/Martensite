//! `bugs_modalg_6` bodies written by hand: the scripts with variables, loops and read-back values.
use crate::support::bugs_commands::geometry::mksurface;
use crate::support::{
    bugs_commands::{
        geometry::offset_curve,
        inspect::{checklength_curve, checksection_count, vertex_dump, whatis, whatis_has},
        options_bop::ScriptBop,
        repair::{edgestowire, settolerance},
        view::{fit, smallview},
    },
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview, screenshot},
    commands::{add, compound, explode, make_box, orientation, pcylinder, trotate, ttranslate},
    construction::{copy, edge, nurbsconvert, tcopy, vertex},
    nbshapes::{Counting, NbCount, checknbshapes, nbshapes},
    one_step::{bcommon, bfuse, bsection},
    properties::Prop,
    session::Session,
    surface_commands::{circle, mkedge, mkface_bounded, polyline},
    tolerance_check::{checkreal, tolerance_report},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::{Glue, Operation};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};

pub fn bug24803(s: &mut dyn Session) {
    println!("========");
    println!("OCC24803");
    println!("========");
    println!();
    vertex(s, "v1", [0., 0., 0.]);
    settolerance(s, "v1", 1.0);
    vertex(s, "v2", [4., 0., 0.]);
    settolerance(s, "v2", 3.5);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "v1", "v2");
    two_step.result(s, "res", Operation::Common);
    let largest = tolerance_report(s, "res");
    println!("MaxTolerance={largest}");
    checkreal("MaxTolerance", &largest, "4.25", 0.1, 0.1);
    let ([x, y, z], tolerance) = vertex_dump(s, "res");
    println!("tolerance_res={tolerance}");
    println!("x_res={x} y_res={y} z_res={z}");
    checkreal("tolerance", &tolerance.to_string(), "4.25", 0.1, 0.1);
    checkreal("x_res", &x.to_string(), "3.25", 0.1, 0.1);
    checkreal("y_res", &y.to_string(), "0", 0.1, 0.1);
    checkreal("z_res", &z.to_string(), "0", 0.1, 0.1);
    smallview(s);
    fit(s);
    screenshot(s, "photo");
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::part_count(s, "res", ShapeEnum::Vertex, 1);
}

/// The script's last lines: the first wire or shell of `r` carries the flag `Closed`.
fn first_part_is_closed(s: &mut dyn Session, kind: ShapeEnum) {
    explode(s, "r", Some(kind));
    if whatis(s, "r_1").split(' ').nth(8) != Some("Closed") {
        println!("ERROR: OCC26420 is reproduced. Flag CLOSED is not set.");
    }
}

fn general_fuse(s: &mut dyn Session, objects: &[&str]) {
    let mut bop = ScriptBop::default();
    bop.add_objects(s, objects);
    bop.bfillds();
    bop.bbuild(s, "r");
}

pub fn bug26420_1(s: &mut dyn Session) {
    println!("========");
    println!("OCC26420");
    println!("========");
    println!();
    polyline(
        s,
        "pp",
        &[
            [0., 4., 0.],
            [7., 4., 0.],
            [7., 6., 0.],
            [0., 6., 0.],
            [0., 4., 0.],
        ],
    );
    vertex(s, "v0", [5., 4., 0.]);
    vertex(s, "v1", [4., 6., 0.]);
    general_fuse(s, &["v0", "v1", "pp"]);
    first_part_is_closed(s, ShapeEnum::Wire);
}

pub fn bug26420_2(s: &mut dyn Session) {
    println!("========");
    println!("OCC26420");
    println!("========");
    println!();
    polyline(
        s,
        "pp",
        &[
            [0., 4., 0.],
            [7., 4., 0.],
            [7., 6., 0.],
            [0., 6., 0.],
            [0., 4., 0.],
        ],
    );
    crate::support::construction::mkplane(s, "ff", "pp");
    vertex(s, "v0", [5., 4., 0.]);
    vertex(s, "v1", [4., 6., 0.]);
    general_fuse(s, &["v0", "v1", "ff"]);
    first_part_is_closed(s, ShapeEnum::Wire);
}

pub fn bug26420_3(s: &mut dyn Session) {
    println!("========");
    println!("OCC26420");
    println!("========");
    println!();
    make_box(s, "bb", [0., 0., 0.], [5., 5., 5.]);
    explode(s, "bb", Some(ShapeEnum::Shell));
    polyline(
        s,
        "pp",
        &[
            [-1., -1., 2.],
            [-1., 6., 2.],
            [6., 6., 2.],
            [6., -1., 2.],
            [-1., -1., 2.],
        ],
    );
    crate::support::construction::mkplane(s, "pl", "pp");
    general_fuse(s, &["pl", "bb_1"]);
    first_part_is_closed(s, ShapeEnum::Shell);
}

fn kind_lines(s: &dyn Session, rows: &[(&str, &str)]) {
    for (name, kind) in rows {
        if whatis_has(s, name, kind) {
            println!("OK : There is {kind}; Compsolid is good");
        } else {
            println!("Error : There is not {kind}; Compsolid is bad");
        }
    }
}

pub fn bug26565_1(s: &mut dyn Session) {
    println!("============");
    println!("OCC26565");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    explode(s, "b1", Some(ShapeEnum::Face));
    copy(s, "b1_1", "f");
    explode(s, "b1", Some(ShapeEnum::Edge));
    copy(s, "b1_8", "e");
    make_box(s, "b2", [0., 0., 0.], [10., 5., 5.]);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["f", "e"]);
    bop.add_tools(s, &["b2"]);
    bop.bfillds();
    bop.bbop(s, "r", Operation::Common);
    nbshapes(s, "r", Counting::Located);
    bop.bbop(s, "r", Operation::Cut);
    nbshapes(s, "r", Counting::Located);
    explode(s, "r", None);
    kind_lines(s, &[("r_1", "FACE"), ("r_2", "EDGE")]);
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::child_kinds(s, "r", &[ShapeEnum::Face, ShapeEnum::Edge]);
}

pub fn bug26565_7(s: &mut dyn Session) {
    println!("============");
    println!("OCC26565");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [10., 0., 0.], [10., 5., 5.]);
    make_box(s, "b3", [10., 5., 0.], [10., 5., 5.]);
    make_box(s, "b4", [10., 0., 5.], [10., 10., 5.]);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1", "b2", "b3", "b4"]);
    bop.bfillds();
    bop.bbuild(s, "r");
    explode(s, "r", Some(ShapeEnum::Solid));
    crate::support::bugs_commands::repair::empty_compsolid(s, "b1");
    for part in ["r_1", "r_2", "r_3", "r_4"] {
        add(s, part, "b1");
    }
    make_box(s, "b2", [5., 0., 2.], [10., 10., 10.]);
    make_box(s, "b3", [-5., 2., 3.], [30., 6., 4.]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b1", "b2"]);
    bop.add_tools(s, &["b3"]);
    bop.bfillds();
    bop.bbop(s, "r", Operation::Cut);
    explode(s, "r", None);
    kind_lines(s, &[("r_1", "COMPSOLID"), ("r_2", "SOLID")]);
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::child_kinds(
        s,
        "r",
        &[ShapeEnum::CompSolid, ShapeEnum::Solid],
    );
}

pub fn bug26673(s: &mut dyn Session) {
    println!("============");
    println!("OCC26673");
    println!("============");
    println!();
    make_box(s, "b", [-1., -1., 0.], [2., 2., 2e-7 + 1e-14]);
    let out = crate::support::bugs_commands::inspect::bopargcheck_full(s, "b", "RFPCS");
    if out.contains("Self-Intersections              : YES") {
        println!("OK: bopargcheck found the shape self-intersecting");
    } else {
        println!("Error: bopargcheck must report self-intersections, but it did not");
    }
}

pub fn bug26796(s: &mut dyn Session) {
    println!("========");
    println!("OCC26796");
    println!("========");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [10.00000018, 0., 0.], [10., 10., 10.]);
    explode(s, "b2", Some(ShapeEnum::Face));
    copy(s, "b2_1", "f");
    trotate(s, "f", [0., 5., 5.], [0., 1., 0.], 10.);
    let surface = mksurface(s, "f");
    mkface_bounded(s, "f", &surface, (-100., 100., -100., 100.));
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1", "b2", "f"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    crate::support::bugs_commands::inspect::bopcheck_text(s, "result", &bop.options);
    checkshape_named(s, "result");
    checknbshapes(s, "result", Counting::Located, &[NbCount::Solids(4)]);
    checkprops(s, "result", Prop::Area, Expected::Mass(41403.1));
    checkprops(s, "result", Prop::Volume, Expected::Mass(2852.9));
    checkview(s, &CheckView::of("result"));
}

pub fn bug27274(s: &mut dyn Session) {
    println!("============");
    println!("OCC27274");
    println!("============");
    println!();
    pcylinder(s, "b1", 50., 145.);
    tcopy(s, "b1", "b2");
    trotate(s, "b2", [0., 0., 0.], [1., 0., 0.], 45.);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1"]);
    bop.add_tools(s, &["b2"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    let counts = [5, 11, 10, 10, 3, 3, 0, 1, 43];
    crate::support::bugs_commands::inspect::checknbshapes_message(
        s,
        "result",
        Counting::Typed,
        &counts,
        "",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(154518.));
}

pub fn bug27773(s: &mut dyn Session) {
    println!("========");
    println!("OCC27773");
    println!("========");
    println!();
    let c = circle([1., 0., 0.], 20.);
    let c1 = offset_curve(&c, 10., [0., 0., 1.]);
    mkedge(s, "e1", &c1);
    vertex(s, "v1", [0., 35., 0.]);
    vertex(s, "v2", [0., -35., 0.]);
    edge(s, "e2", "v1", "v2");
    bsection(s, "result", "e1", "e2");
    checknbshapes(s, "result", Counting::Located, &[NbCount::Vertices(2)]);
}

pub fn bug27878_6(s: &mut dyn Session) {
    println!("========");
    println!("OCC27878");
    println!("========");
    println!();
    compound(s, &[], "sh");
    for i in 0..4 {
        for j in 0..4 {
            make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
            ttranslate(s, "b", [i as f64, j as f64, 0.]);
            add(s, "b", "sh");
        }
    }
    nurbsconvert(s, "b1", "sh");
    let mut bop = ScriptBop::default();
    bop.glue = Glue::Full;
    let parts = explode(s, "b1", Some(ShapeEnum::Solid));
    let names: Vec<&str> = parts.iter().map(String::as_str).collect();
    bop.add_objects(s, &names);
    bop.bfillds_timed();
    let mut cells = crate::support::bugs_commands::cells::ScriptCells::default();
    cells.bcbuild(s, &mut bop, "rx");
    cells.bcaddall(s, "res1", 1, true);
    copy(s, "res1", "b2");
    ttranslate(s, "b2", [0.5, 0.5, 1.]);
    bop.glue = Glue::Shift;
    bop.bop(s, "res1", "b2");
    bop.bbop(s, "result", Operation::Fuse);
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(78), NbCount::Solids(1)],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(71.5));
    checkprops(s, "result", Prop::Volume, Expected::Mass(32.));
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::part_count(s, "b1", ShapeEnum::Solid, 16);
}

/// `eval compound [explode b1 w] b1` and the same for `b2`, on two unit boxes of ten.
fn wire_compounds(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    for result in ["b1", "b2"] {
        let wires = explode(s, "b1", Some(ShapeEnum::Wire));
        let names: Vec<&str> = wires.iter().map(String::as_str).collect();
        compound(s, &names, result);
    }
}

pub fn bug28189_2(s: &mut dyn Session) {
    println!("========");
    println!("OCC28189");
    println!("========");
    println!();
    wire_compounds(s);
    bcommon(s, "result", "b1", "b2");
    checknbshapes(s, "result", Counting::Located, &[NbCount::Wires(6)]);
}

pub fn bug28189_3(s: &mut dyn Session) {
    println!("========");
    println!("OCC28189");
    println!("========");
    println!();
    wire_compounds(s);
    bfuse(s, "result", "b1", "b2");
    checknbshapes(s, "result", Counting::Located, &[NbCount::Wires(6)]);
}

pub fn bug28189_4(s: &mut dyn Session) {
    println!("========");
    println!("OCC28189");
    println!("========");
    println!();
    wire_compounds(s);
    ttranslate(s, "b2", [5., 0., 0.]);
    bcommon(s, "result", "b1", "b2");
    checknbshapes(s, "result", Counting::Located, &[NbCount::Wires(4)]);
}

pub fn bug28189_5(s: &mut dyn Session) {
    println!("========");
    println!("OCC28189");
    println!("========");
    println!();
    wire_compounds(s);
    ttranslate(s, "b2", [5., 0., 0.]);
    bfuse(s, "result", "b1", "b2");
    checknbshapes(s, "result", Counting::Located, &[NbCount::Wires(12)]);
}

pub fn bug28189_6(s: &mut dyn Session) {
    println!("========");
    println!("OCC28189");
    println!("========");
    println!();
    for (name, at) in [
        ("v1", [0., 0., 0.]),
        ("v2", [1., 0., 0.]),
        ("v3", [1., 1., 0.]),
        ("v4", [0., 1., 0.]),
    ] {
        vertex(s, name, at);
    }
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v3", "v2");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v1", "v4");
    compound(s, &["e1", "e2", "e3", "e4"], "edges");
    edgestowire(s, "result", "edges");
    if !whatis_has(s, "result", "WIRE") {
        println!("Error: Wire is not created");
    }
    explode(s, "result", None);
    for (name, word) in [
        ("result_1", "FORWARD"),
        ("result_2", "REVERSED"),
        ("result_3", "FORWARD"),
        ("result_4", "REVERSED"),
    ] {
        if !whatis_has(s, name, word) {
            println!("Error: Incorrect orientation of edges in wire");
        }
    }
}

pub fn bug28189_7(s: &mut dyn Session) {
    println!("========");
    println!("OCC28189");
    println!("========");
    println!();
    polyline(s, "p1", &[[0., 0., 0.], [1., 0., 0.]]);
    polyline(s, "p2", &[[0., 0., 0.], [1., 0., 0.]]);
    orientation(s, "p1", Orientation::Reversed);
    orientation(s, "p2", Orientation::Forward);
    bfuse(s, "result1", "p1", "p2");
    explode(s, "result1", None);
    if !whatis_has(s, "result1_1", "REVERSED") {
        println!("Error: Incorrect orientation of wire");
    }
    bcommon(s, "result2", "p2", "p1");
    explode(s, "result2", None);
    if !whatis_has(s, "result2_1", "FORWARD") {
        println!("Error: Incorrect orientation of wire");
    }
}

pub fn bug28776(s: &mut dyn Session) {
    println!("========");
    println!("OCC28776");
    println!("========");
    println!();
    for name in ["b1", "b2", "b3"] {
        make_box(s, name, [0., 0., 0.], [10., 10., 10.]);
    }
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1", "b2", "b3"]);
    bop.bfillds();
    bop.bbuild(s, "r");
    let rows = [
        (ShapeEnum::Vertex, NbCount::Vertices(3)),
        (ShapeEnum::Edge, NbCount::Edges(3)),
        (ShapeEnum::Face, NbCount::Faces(3)),
        (ShapeEnum::Solid, NbCount::Solids(3)),
    ];
    for (kind, count) in rows {
        explode(s, "r", Some(kind));
        bop.boporigin(s, "r_1");
        checknbshapes(s, "r_1_or", Counting::Located, &[count]);
    }
}

pub fn bug8040(s: &mut dyn Session) {
    println!("============");
    println!("OCC8040");
    println!("============");
    println!();
    let c = circle([1., 0., 0.], 20.);
    let c1 = offset_curve(&c, 10., [0., 0., 1.]);
    let c2 = offset_curve(&c, -10., [0., 0., 1.]);
    checklength_curve("c", &c, 125.66370614359172);
    checklength_curve("c1", &c1, 188.49555921538757);
    checklength_curve("c2", &c2, 62.831853071795862);
    mkedge(s, "e", &c);
    mkedge(s, "e1", &c1);
    mkedge(s, "e2", &c2);
    vertex(s, "v1", [0., 35., 0.]);
    vertex(s, "v2", [0., -35., 0.]);
    edge(s, "e3", "v1", "v2");
    let mut alone = 0;
    for (index, name) in ["e", "e1", "e2"].into_iter().enumerate() {
        let mut two_step = TwoStepBoolean::default();
        two_step.bop(s, name, "e3");
        let result = format!("result{}", index + 1);
        two_step.section(s, &result);
        alone += checksection_count(s, &result);
        println!("number_of_vertex = {alone}");
    }
    if alone != 6 {
        println!("Error : Section is incorrect");
    }
    smallview(s);
    fit(s);
    screenshot(s, "photo");
}
