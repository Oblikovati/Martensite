//! The `bugs/modalg_7` scripts that branch on what a command answers.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{explode, make_box},
    feat_definitions::featprism,
    feat_perform::{Kas, featperformval},
    feat_script::FeatScript,
    inspect::{generated, isdeleted, modified, savehistory},
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    splitshape::splitshape,
    surface_variables::{mkface, mksurface, trim},
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug23925(s: &mut FeatScript<'_>) {
    println!("============");
    println!("23925");
    println!("============");
    println!();
    restore(s, "bug23925_E.brep", "e");
    restore(s, "bug23925_F.brep", "f");
    splitshape(s, "result", "f", &["f", "e"], &[]);
    if explode(s, "result", Some(ShapeEnum::Face)).len() == 2 {
        println!("OK: BRepFeat_SplitShape produces good shape");
    } else {
        println!("Error: BRepFeat_SplitShape produces invalid shape");
    }
    checkview(s, &CheckView::of("result"));
}

pub fn bug31469(s: &mut FeatScript<'_>) {
    use NbCount::{Edges, Faces, Solids, Vertices, Wires};
    println!("========");
    println!(
        "0031469: Modeling Algorithms - 0031469: BRepFeatMakePrism has problem with method IsDeleted(...)"
    );
    println!("========");
    println!();
    make_box(s, "S", [0., 0., 0.], [400., 250., 300.]);
    explode(s, "S", Some(ShapeEnum::Face));
    mksurface(s, "pl2", "S_2");
    trim(s, "pl2", "pl2", [100., 200., -200., -100.]);
    mkface(s, "pbase", "pl2");
    featprism(s, "S", "pbase", "S_2", [-1., 0., 0.], 0, 1);
    featperformval(s, Kas::Prism, "pp", 200.);
    checkshape_named(s, "pp");
    let whole = [Vertices(12), Edges(20), Faces(11), Solids(1)];
    checknbshapes(s, "pp", Counting::Located, &whole);
    let history = savehistory(s);
    if !isdeleted(s, &history, "S").contains("Not deleted") {
        println!("Error: S has wrong delete status");
    }
    if !isdeleted(s, &history, "S_2").contains("Not deleted") {
        println!("Error: S_2 has wrong delete status");
    }
    if modified(s, "m1", &history, "S").contains("The shape has not been modified.") {
        println!("Error: S has wrong modified list");
    } else {
        checknbshapes(s, "m1", Counting::Located, &whole);
    }
    if modified(s, "m2", &history, "S_2").contains("The shape has not been modified.") {
        println!("Error: S_2 has wrong modified list");
    } else {
        let split = [Vertices(8), Edges(8), Wires(2), Faces(1)];
        checknbshapes(s, "m2", Counting::Located, &split);
    }
    explode(s, "pbase", Some(ShapeEnum::Edge));
    if generated(s, "g1", &history, "pbase_1").contains("No shapes were generated from the shape.")
    {
        println!("Error: pbase_1 has wrong generated list");
    } else {
        let side = [Vertices(2), Edges(3), Wires(1), Faces(1)];
        checknbshapes(s, "g1", Counting::Located, &side);
        checkprops(s, "g1", Prop::Area, Expected::Mass(20000.));
    }
}
