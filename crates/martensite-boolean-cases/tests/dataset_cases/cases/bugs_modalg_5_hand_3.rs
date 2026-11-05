//! `bugs_modalg_5`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::geometry::{cylinder, sphere},
    bugs_commands::options_bop::ScriptBop,
    bugs_commands::repair::{mkvolume, mkvolume_as_given},
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{explode, make_box},
    construction::vertex,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    session::Session,
    surface_commands::{cone, mkface, mkface_bounded, plane, trimv},
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug25232_2(s: &mut dyn Session) {
    println!("============");
    println!("OCC25232");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    explode(s, "b1", Some(ShapeEnum::Face));
    make_box(s, "b2", [4., 4., 4.], [2., 2., 2.]);
    explode(s, "b2", Some(ShapeEnum::Wire));
    mkvolume_as_given(
        s,
        "result",
        &[
            "b1_1", "b1_2", "b1_3", "b1_4", "b1_5", "b1_6", "b2_1", "b2_2", "b2_3", "b2_4", "b2_5",
            "b2_6",
        ],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(24),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(0),
            NbCount::Shapes(54),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25232_3(s: &mut dyn Session) {
    println!("============");
    println!("OCC25232");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    let var_s = sphere([5., 5., 5.], 3.);
    mkface(s, "b2", &var_s);
    vertex(s, "b3", [5., 5., 5.]);
    mkvolume_as_given(s, "result", &["b1", "b2", "b3"]);
    checkprops(s, "result", Prop::Area, Expected::Mass(826.195));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(11),
            NbCount::Edges(15),
            NbCount::Wires(7),
            NbCount::Faces(7),
            NbCount::Shells(3),
            NbCount::Solids(2),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(46),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25232_4(s: &mut dyn Session) {
    println!("============");
    println!("OCC25232");
    println!("============");
    println!();
    let var_cyl = cylinder(10.);
    let var_cyl1 = trimv(&var_cyl, -20., 20.);
    mkface(s, "fcyl", &var_cyl1);
    let var_con = cone([0., 0., 0.], [0., 0., 1.], 45., 0.);
    let var_con12 = trimv(&var_con, 0., 15.);
    let var_con23 = trimv(&var_con, -15., 0.);
    mkface(s, "fcon1", &var_con12);
    mkface(s, "fcon2", &var_con23);
    let var_pl = plane([0., 0., 15.], [0., 0., 1.]);
    mkface_bounded(s, "fp", &var_pl, (-15., 15., -15., 15.));
    let var_sph = sphere([0., 0., 0.], 12.);
    mkface(s, "fsph", &var_sph);
    mkvolume(s, "result", &["fcyl", "fcon1", "fcon2", "fp", "fsph"]);
    checkprops(s, "result", Prop::Area, Expected::Mass(6725.11));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(12),
            NbCount::Edges(26),
            NbCount::Wires(14),
            NbCount::Faces(14),
            NbCount::Shells(7),
            NbCount::Solids(7),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(81),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25232_5(s: &mut dyn Session) {
    println!("============");
    println!("OCC25232");
    println!("============");
    println!();
    let var_cyl = cylinder(10.);
    let var_cyl1 = trimv(&var_cyl, -20., 20.);
    mkface(s, "fcyl", &var_cyl1);
    let var_con = cone([0., 0., 0.], [0., 0., 1.], 45., 0.);
    let var_con12 = trimv(&var_con, 0., 15.);
    let var_con23 = trimv(&var_con, -15., 0.);
    mkface(s, "fcon1", &var_con12);
    mkface(s, "fcon2", &var_con23);
    let var_pl = plane([0., 0., 15.], [0., 0., 1.]);
    mkface_bounded(s, "fp", &var_pl, (-15., 15., -15., 15.));
    let var_sph = sphere([0., 0., 0.], 12.);
    mkface(s, "fsph", &var_sph);
    mkvolume(s, "result", &["fcyl", "fsph"]);
    checkprops(s, "result", Prop::Area, Expected::Mass(3476.67));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(8),
            NbCount::Wires(4),
            NbCount::Faces(4),
            NbCount::Shells(2),
            NbCount::Solids(2),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(25),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25232_6(s: &mut dyn Session) {
    println!("============");
    println!("OCC25232");
    println!("============");
    println!();
    let var_cyl = cylinder(10.);
    let var_cyl1 = trimv(&var_cyl, -20., 20.);
    mkface(s, "fcyl", &var_cyl1);
    let var_con = cone([0., 0., 0.], [0., 0., 1.], 45., 0.);
    let var_con12 = trimv(&var_con, 0., 15.);
    let var_con23 = trimv(&var_con, -15., 0.);
    mkface(s, "fcon1", &var_con12);
    mkface(s, "fcon2", &var_con23);
    let var_pl = plane([0., 0., 15.], [0., 0., 1.]);
    mkface_bounded(s, "fp", &var_pl, (-15., 15., -15., 15.));
    let var_sph = sphere([0., 0., 0.], 12.);
    mkface(s, "fsph", &var_sph);
    mkvolume(s, "result", &["fsph", "fcon1"]);
    checkprops(s, "result", Prop::Area, Expected::Mass(2449.33));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(7),
            NbCount::Wires(3),
            NbCount::Faces(3),
            NbCount::Shells(2),
            NbCount::Solids(2),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(22),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25232_7(s: &mut dyn Session) {
    println!("============");
    println!("OCC25232");
    println!("============");
    println!();
    let var_cyl = cylinder(10.);
    let var_cyl1 = trimv(&var_cyl, -20., 20.);
    mkface(s, "fcyl", &var_cyl1);
    let var_con = cone([0., 0., 0.], [0., 0., 1.], 45., 0.);
    let var_con12 = trimv(&var_con, 0., 15.);
    let var_con23 = trimv(&var_con, -15., 0.);
    mkface(s, "fcon1", &var_con12);
    mkface(s, "fcon2", &var_con23);
    let var_pl = plane([0., 0., 15.], [0., 0., 1.]);
    mkface_bounded(s, "fp", &var_pl, (-15., 15., -15., 15.));
    let var_sph = sphere([0., 0., 0.], 12.);
    mkface(s, "fsph", &var_sph);
    mkvolume(s, "result", &["fcyl", "fp"]);
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

pub fn bug25232_8(s: &mut dyn Session) {
    println!("============");
    println!("OCC25232");
    println!("============");
    println!();
    let var_cyl = cylinder(10.);
    let var_cyl1 = trimv(&var_cyl, -20., 20.);
    mkface(s, "fcyl", &var_cyl1);
    let var_con = cone([0., 0., 0.], [0., 0., 1.], 45., 0.);
    let var_con12 = trimv(&var_con, 0., 15.);
    let var_con23 = trimv(&var_con, -15., 0.);
    mkface(s, "fcon1", &var_con12);
    mkface(s, "fcon2", &var_con23);
    let var_pl = plane([0., 0., 15.], [0., 0., 1.]);
    mkface_bounded(s, "fp", &var_pl, (-15., 15., -15., 15.));
    let var_sph = sphere([0., 0., 0.], 12.);
    mkface(s, "fsph", &var_sph);
    let var_con36 = trimv(&var_con, 0., 30.);
    mkface(s, "fcon3", &var_con36);
    mkvolume(s, "result", &["fcon3", "fp"]);
    checkprops(s, "result", Prop::Area, Expected::Mass(1706.51));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(5),
            NbCount::Edges(6),
            NbCount::Wires(2),
            NbCount::Faces(2),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(0),
            NbCount::Shapes(17),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25722(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("==========");
    println!("OCC25722");
    println!("==========");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [10.0001, 0., 0.], [10., 10., 10.]);
    bop.options.fuzzy = 0.0001;
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b1"]);
    bop.add_tools(s, &["b2"]);
    bop.bfillds();
    bop.bbop(s, "r", Operation::Fuse);
    checknbshapes(s, "r", Counting::Located, &[NbCount::Solids(1)]);
    bop.options.fuzzy = 0.;
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [10.0001, 0., 0.], [10., 10., 10.]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b1"]);
    bop.add_tools(s, &["b2"]);
    bop.bfillds();
    bop.bbop(s, "result", Operation::Fuse);
    checknbshapes(s, "result", Counting::Located, &[NbCount::Solids(2)]);
    checkview(s, &CheckView::of("result"));
}
