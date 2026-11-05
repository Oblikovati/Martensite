//! `removefeatures` bodies written by hand: their scripts loop over shapes and read histories.
use crate::support::{
    beyond_script::{kind_is, part_count, printed_mass},
    bop_session::BopSession,
    checkprops::{CheckTolerances, Expected, checkprops, checkprops_equal, checkprops_within},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{add, explode, pcylinder, prism, ttranslate},
    construction::mkplane,
    hooks::{require_bopcheck_ok, require_whatis},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bcommon,
    properties::Prop,
    removal_commands::{FeatureRemoval, modified},
    restore::restore,
    session::Session,
    surface_commands::polyline,
    volume_commands::compound,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn C1(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex1.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(s, &["s_6", "s_18"], "step");
    removal.remove(s, "res", "s", &["step"]);
    compound(
        s,
        &[
            "s_4", "s_15", "s_21", "s_22", "s_7", "s_11", "s_8", "s_9", "s_10", "s_14", "s_20",
            "s_13",
        ],
        "features",
    );
    let rf_hist = removal.savehistory();
    compound(s, &[], "mf");
    for f in explode(s, "features", Some(ShapeEnum::Face)) {
        if !modified(s, "m", &rf_hist, &f) {
            add(s, &f, "mf");
        } else {
            add(s, "m", "mf");
        }
    }
    removal.remove(s, "result", "res", &["mf"]);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(460.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(200.));
    checknbshapes(
        s,
        "result",
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
    removal.check_removed(
        s,
        "mf",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 22);
}

pub fn C4(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_boxes.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(
        s,
        &[
            "s_30", "s_31", "s_32", "s_60", "s_39", "s_54", "s_48", "s_58", "s_7", "s_12", "s_46",
            "s_25", "s_21", "s_40", "s_9", "s_38", "s_45", "s_13", "s_59", "s_8", "s_22", "s_51",
            "s_52", "s_18",
        ],
        "hole",
    );
    removal.remove(s, "result", "s", &["hole"]);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(4800.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(8000.));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(27),
            NbCount::Edges(54),
            NbCount::Wires(36),
            NbCount::Faces(36),
            NbCount::Shells(8),
            NbCount::Solids(8),
        ],
    );
    removal.check_removed(
        s,
        "hole",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    require_whatis(
        s,
        "result",
        "COMPSOLID",
        "Error: the type of input shape is lost during feature removal",
    );
    require_bopcheck_ok(s, "result", "Error: sharing is lost after feature removal");
    let rf_hist = removal.savehistory();
    for solid in explode(s, "s", Some(ShapeEnum::Solid)) {
        modified(s, "msol", &rf_hist, &solid);
        checkprops(s, "msol", Prop::Area, Expected::Mass(600.));
        checkprops(s, "msol", Prop::Volume, Expected::Mass(1000.));
        checknbshapes(
            s,
            "msol",
            Counting::Typed,
            &[
                NbCount::Vertices(8),
                NbCount::Edges(12),
                NbCount::Wires(6),
                NbCount::Faces(6),
                NbCount::Shells(1),
                NbCount::Solids(1),
            ],
        );
        bcommon(s, "com", "msol", &solid);
        checkprops_equal(s, "com", &solid);
    }
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "msol", Prop::Area, "600");
    kind_is(s, "msol", ShapeEnum::Solid);
    printed_mass(s, "s_8", Prop::Area, "600");
    kind_is(s, "s_8", ShapeEnum::Solid);
    part_count(s, "s", Some(ShapeEnum::Solid), 8);
}

pub fn G1(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    let mut bop = BopSession::default();
    polyline(
        s,
        "p",
        &[
            [0., 0., 0.],
            [10., 0., 0.],
            [10., 0., 5.],
            [5., 0., 5.],
            [0., 0., 0.],
        ],
    );
    mkplane(s, "f", "p");
    prism(s, "s", "f", [0., 5., 0.]);
    pcylinder(s, "c", 2., 3.);
    ttranslate(s, "c", [7.5, 2.5, 2.]);
    bop.arguments(s, &["s"], &["c"]);
    bop.bfillds();
    bop.bbop(s, "model", Operation::Cut);
    let bop_hist = bop.savehistory();
    explode(s, "s", Some(ShapeEnum::Face));
    modified(s, "feature", &bop_hist, "s_3");
    removal.remove(s, "result", "model", &["feature"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(334.621),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(180.885),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(12),
            NbCount::Wires(8),
            NbCount::Faces(7),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(s, "feature", &[ShapeEnum::Edge, ShapeEnum::Face]);
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "feature", Prop::Area, "12.4336");
    kind_is(s, "feature", ShapeEnum::Face);
    part_count(s, "s", Some(ShapeEnum::Face), 6);
}
