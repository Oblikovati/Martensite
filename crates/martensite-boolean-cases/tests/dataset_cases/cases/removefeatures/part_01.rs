//! The `removefeatures` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A9(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex4.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_2", "s_3"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(334.248),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(241.372),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(10),
            NbCount::Edges(15),
            NbCount::Wires(9),
            NbCount::Faces(8),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "s_2",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    removal.check_removed(
        s,
        "s_3",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 11);
}

pub fn B1(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex1.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_13"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(463.068),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(194.214),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(31),
            NbCount::Edges(50),
            NbCount::Wires(23),
            NbCount::Faces(20),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "s_13",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    let rf_hist = removal.savehistory();
    modified(s, "m2", &rf_hist, "s_2");
    checkprops(s, "m2", Prop::Area, Expected::Mass(20.));
    checknbshapes(
        s,
        "m2",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(4),
            NbCount::Wires(1),
            NbCount::Faces(1),
        ],
    );
    modified(s, "m12", &rf_hist, "s_12");
    checkprops(s, "m12", Prop::Area, Expected::Mass(8.));
    checknbshapes(
        s,
        "m12",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(4),
            NbCount::Wires(1),
            NbCount::Faces(1),
        ],
    );
    modified(s, "m17", &rf_hist, "s_17");
    checkprops(s, "m17", Prop::Area, Expected::Mass(10.));
    checknbshapes(
        s,
        "m17",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(4),
            NbCount::Wires(1),
            NbCount::Faces(1),
        ],
    );
    generated(s, "g17", &rf_hist, "s_17");
    checkprops(s, "g17", Prop::Length, Expected::Mass(12.));
    explode(s, "s_17", Some(ShapeEnum::Edge));
    modified(s, "m17_1", &rf_hist, "s_17_1");
    checkprops(s, "m17_1", Prop::Length, Expected::Mass(1.));
    modified(s, "m17_3", &rf_hist, "s_17_3");
    checkprops(s, "m17_3", Prop::Length, Expected::Mass(1.));
    modified(s, "m1", &rf_hist, "s_1");
    checkprops(s, "m1", Prop::Area, Expected::Mass(172.551));
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "m2", Prop::Area, "20");
    kind_is(s, "m2", ShapeEnum::Face);
    printed_mass(s, "m12", Prop::Area, "8");
    kind_is(s, "m12", ShapeEnum::Face);
    printed_mass(s, "m17", Prop::Area, "10");
    kind_is(s, "m17", ShapeEnum::Face);
    printed_mass(s, "g17", Prop::Area, "0");
    kind_is(s, "g17", ShapeEnum::Compound);
    printed_mass(s, "m17_1", Prop::Area, "0");
    kind_is(s, "m17_1", ShapeEnum::Edge);
    printed_mass(s, "m17_3", Prop::Area, "0");
    kind_is(s, "m17_3", ShapeEnum::Edge);
    printed_mass(s, "m1", Prop::Area, "172.551");
    kind_is(s, "m1", ShapeEnum::Face);
    part_count(s, "s", Some(ShapeEnum::Face), 22);
    part_count(s, "s_17", Some(ShapeEnum::Edge), 4);
}

pub fn B2(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex1.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_14"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(462.33),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(194.594),
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
            NbCount::Vertices(33),
            NbCount::Edges(51),
            NbCount::Wires(23),
            NbCount::Faces(20),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(s, "s_14", &[ShapeEnum::Edge, ShapeEnum::Face]);
    let rf_hist = removal.savehistory();
    modified(s, "m1", &rf_hist, "s_1");
    checkprops(s, "m1", Prop::Area, Expected::Mass(171.478));
    checknbshapes(
        s,
        "m1",
        Counting::Located,
        &[
            NbCount::Vertices(13),
            NbCount::Edges(13),
            NbCount::Wires(3),
            NbCount::Faces(1),
        ],
    );
    generated(s, "g1", &rf_hist, "s_1");
    checknbshapes(
        s,
        "g1",
        Counting::Located,
        &[NbCount::Vertices(1), NbCount::Edges(1)],
    );
    checkprops(s, "g1", Prop::Length, Expected::Mass(6.28319));
    modified(s, "m20", &rf_hist, "s_20");
    checkprops(s, "m20", Prop::Area, Expected::Mass(6.28319));
    checknbshapes(
        s,
        "m20",
        Counting::Located,
        &[
            NbCount::Vertices(2),
            NbCount::Edges(3),
            NbCount::Wires(1),
            NbCount::Faces(1),
        ],
    );
    generated(s, "g20", &rf_hist, "s_20");
    checknbshapes(
        s,
        "g20",
        Counting::Located,
        &[NbCount::Vertices(1), NbCount::Edges(1)],
    );
    checkprops_equal(s, "g20", "g1");
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "m1", Prop::Area, "171.478");
    kind_is(s, "m1", ShapeEnum::Face);
    printed_mass(s, "g1", Prop::Area, "0");
    kind_is(s, "g1", ShapeEnum::Compound);
    printed_mass(s, "m20", Prop::Area, "6.28319");
    kind_is(s, "m20", ShapeEnum::Face);
    printed_mass(s, "g20", Prop::Area, "0");
    kind_is(s, "g20", ShapeEnum::Edge);
    part_count(s, "s", Some(ShapeEnum::Face), 22);
}

pub fn B3(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex1.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_15"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(462.273),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(193.127),
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
            NbCount::Vertices(33),
            NbCount::Edges(51),
            NbCount::Wires(23),
            NbCount::Faces(20),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(s, "s_15", &[ShapeEnum::Edge, ShapeEnum::Face]);
    let rf_hist = removal.savehistory();
    modified(s, "m1", &rf_hist, "s_1");
    checkprops(s, "m1", Prop::Area, Expected::Mass(172.452));
    checknbshapes(
        s,
        "m1",
        Counting::Located,
        &[
            NbCount::Vertices(13),
            NbCount::Edges(13),
            NbCount::Wires(3),
            NbCount::Faces(1),
        ],
    );
    generated(s, "g1", &rf_hist, "s_1");
    checknbshapes(
        s,
        "g1",
        Counting::Located,
        &[NbCount::Vertices(1), NbCount::Edges(1)],
    );
    checkprops(s, "g1", Prop::Length, Expected::Mass(6.28319));
    modified(s, "m21", &rf_hist, "s_21");
    checkprops(s, "m21", Prop::Area, Expected::Mass(12.5664));
    checknbshapes(
        s,
        "m21",
        Counting::Located,
        &[
            NbCount::Vertices(2),
            NbCount::Edges(3),
            NbCount::Wires(1),
            NbCount::Faces(1),
        ],
    );
    generated(s, "g21", &rf_hist, "s_21");
    checknbshapes(
        s,
        "g21",
        Counting::Located,
        &[NbCount::Vertices(1), NbCount::Edges(1)],
    );
    checkprops_equal(s, "g21", "g1");
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "m1", Prop::Area, "172.452");
    kind_is(s, "m1", ShapeEnum::Face);
    printed_mass(s, "g1", Prop::Area, "0");
    kind_is(s, "g1", ShapeEnum::Compound);
    printed_mass(s, "m21", Prop::Area, "12.5664");
    kind_is(s, "m21", ShapeEnum::Face);
    printed_mass(s, "g21", Prop::Area, "0");
    kind_is(s, "g21", ShapeEnum::Edge);
    part_count(s, "s", Some(ShapeEnum::Face), 22);
}
