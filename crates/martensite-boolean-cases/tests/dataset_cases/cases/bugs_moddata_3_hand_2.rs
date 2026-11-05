//! `bugs_moddata_3`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::options_bop::ScriptBop,
    bugs_commands::repair::removeinternals,
    checkshape::checkshape_named,
    commands::{compound, explode, make_box},
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_like},
    session::Session,
    surface_commands::polyline,
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug31587_1(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("================");
    println!("0031587: Modeling Data - Remove internal sub-shapes from the shape");
    println!("================");
    println!();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b1", [2., 2., 2.], [3., 2., 1.]);
    explode(s, "b1", Some(ShapeEnum::Face));
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b"]);
    bop.add_tools(s, &["b1_1"]);
    bop.add_exploded_tools(s, "b1_2", ShapeEnum::Edge);
    bop.bfillds();
    bop.bsplit(s, "r");
    removeinternals(s, "r", false);
    checkshape_named(s, "r");
    checknbshapes(
        s,
        "r",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(12),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
}

pub fn bug31587_2(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("================");
    println!("0031587: Modeling Data - Remove internal sub-shapes from the shape");
    println!("================");
    println!();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b1", [2., 2., 0.], [3., 2., 1.]);
    explode(s, "b1", Some(ShapeEnum::Face));
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b"]);
    bop.add_tools(s, &["b1_1"]);
    bop.add_exploded_tools(s, "b1_2", ShapeEnum::Edge);
    bop.add_exploded_tools(s, "b1_3", ShapeEnum::Vertex);
    bop.bfillds();
    bop.bsplit(s, "r");
    removeinternals(s, "r", false);
    checkshape_named(s, "r");
    checknbshapes(
        s,
        "r",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(12),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
}

pub fn bug31587_3(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("================");
    println!("0031587: Modeling Data - Remove internal sub-shapes from the shape");
    println!("================");
    println!();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b1", [2., 2., -1.], [3., 2., 2.]);
    explode(s, "b1", Some(ShapeEnum::Face));
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b"]);
    bop.add_tools(s, &["b1_1", "b1_2"]);
    bop.bfillds();
    bop.bsplit(s, "r");
    removeinternals(s, "r", false);
    checkshape_named(s, "r");
    checknbshapes(
        s,
        "r",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(12),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    bop.bbuild(s, "r");
    bop.bbuild(s, "r1");
    removeinternals(s, "r1", false);
    checkshape_named(s, "r1");
    checknbshapes_like(s, "r1", "r");
    removeinternals(s, "r1", true);
    checkshape_named(s, "r1");
    checknbshapes(
        s,
        "r1",
        Counting::Located,
        &[
            NbCount::Vertices(20),
            NbCount::Edges(26),
            NbCount::Wires(10),
            NbCount::Faces(10),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    explode(s, "r1", Some(ShapeEnum::Solid));
    checknbshapes(
        s,
        "r1_1",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(12),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
}

pub fn bug31587_4(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("================");
    println!("0031587: Modeling Data - Remove internal sub-shapes from the shape");
    println!("================");
    println!();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b1", [2., 2., 0.], [5., 5., 5.]);
    explode(s, "b1", Some(ShapeEnum::Face));
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b"]);
    bop.add_tools(s, &["b1_2", "b1_3"]);
    bop.add_exploded_tools(s, "b1_1", ShapeEnum::Wire);
    bop.bfillds();
    bop.bsplit(s, "r");
    removeinternals(s, "r", false);
    checkshape_named(s, "r");
    checknbshapes(
        s,
        "r",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(12),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
}

pub fn bug31587_5(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("================");
    println!("0031587: Modeling Data - Remove internal sub-shapes from the shape");
    println!("================");
    println!();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b1", [2., 2., 0.], [5., 5., 5.]);
    make_box(s, "b2", [3., 2., 2.], [4., 5., 2.]);
    explode(s, "b1", Some(ShapeEnum::Face));
    explode(s, "b2", Some(ShapeEnum::Face));
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b"]);
    bop.add_tools(s, &["b1_2", "b1_3"]);
    bop.add_exploded_tools(s, "b1_1", ShapeEnum::Wire);
    bop.add_tools(s, &["b2_5", "b2_6"]);
    bop.bfillds();
    bop.bsplit(s, "r");
    removeinternals(s, "r", false);
    checkshape_named(s, "r");
    checknbshapes(
        s,
        "r",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(12),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
}

pub fn bug31587_6(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("================");
    println!("0031587: Modeling Data - Remove internal sub-shapes from the shape");
    println!("================");
    println!();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    polyline(s, "e1", &[[2., 2., 0.], [8., 2., 0.]]);
    polyline(s, "e2", &[[3., 2., -5.], [3., 2., 0.]]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b"]);
    bop.add_tools(s, &["e1", "e2"]);
    bop.bfillds();
    bop.bsplit(s, "r");
    compound(s, &["r", "e2"], "c");
    removeinternals(s, "c", false);
    checkshape_named(s, "c");
    checknbshapes(
        s,
        "c",
        Counting::Located,
        &[
            NbCount::Vertices(10),
            NbCount::Edges(13),
            NbCount::Wires(7),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
}
