//! The `removefeatures` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn B4(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex1.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_4"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(462.815),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(195.248),
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
            NbCount::Vertices(30),
            NbCount::Edges(47),
            NbCount::Wires(22),
            NbCount::Faces(19),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "s_4",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    let rf_hist = removal.savehistory();
    modified(s, "m1", &rf_hist, "s_1");
    checkprops(s, "m1", Prop::Area, Expected::Mass(169.122));
    checknbshapes(
        s,
        "m1",
        Counting::Located,
        &[
            NbCount::Vertices(11),
            NbCount::Edges(11),
            NbCount::Wires(3),
            NbCount::Faces(1),
        ],
    );
    modified(s, "m3", &rf_hist, "s_3");
    modified(s, "m5", &rf_hist, "s_5");
    require_same_shapes(s, "m3", "m5", "Error: incorrect feature removal");
    checkprops(s, "m3", Prop::Area, Expected::Mass(9.75));
    checknbshapes(
        s,
        "m3",
        Counting::Located,
        &[
            NbCount::Vertices(6),
            NbCount::Edges(6),
            NbCount::Wires(1),
            NbCount::Faces(1),
        ],
    );
    explode(s, "s_3", Some(ShapeEnum::Edge));
    explode(s, "s_5", Some(ShapeEnum::Edge));
    modified(s, "m3_4", &rf_hist, "s_3_4");
    modified(s, "m5_2", &rf_hist, "s_5_2");
    require_same_shapes(s, "m3_4", "m5_2", "Error: incorrect feature removal");
    checkprops(s, "m3_4", Prop::Length, Expected::Mass(9.5));
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "m1", Prop::Area, "169.122");
    kind_is(s, "m1", ShapeEnum::Face);
    printed_mass(s, "m3", Prop::Area, "9.75");
    kind_is(s, "m3", ShapeEnum::Face);
    printed_mass(s, "m5", Prop::Area, "9.75");
    kind_is(s, "m5", ShapeEnum::Face);
    printed_mass(s, "m3_4", Prop::Area, "0");
    kind_is(s, "m3_4", ShapeEnum::Edge);
    printed_mass(s, "m5_2", Prop::Area, "0");
    kind_is(s, "m5_2", ShapeEnum::Edge);
    part_count(s, "s", Some(ShapeEnum::Face), 22);
    part_count(s, "s_3", Some(ShapeEnum::Edge), 4);
    part_count(s, "s_5", Some(ShapeEnum::Edge), 6);
}

pub fn B5(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex1.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_6", "s_18"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(461.315),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(196.178),
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
            NbCount::Vertices(29),
            NbCount::Edges(47),
            NbCount::Wires(22),
            NbCount::Faces(19),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "s_6",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    removal.check_removed(
        s,
        "s_18",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    let rf_hist = removal.savehistory();
    modified(s, "m1", &rf_hist, "s_1");
    checkprops(s, "m1", Prop::Area, Expected::Mass(172.551));
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
    modified(s, "m19", &rf_hist, "s_19");
    checkprops(s, "m19", Prop::Area, Expected::Mass(10.));
    checknbshapes(
        s,
        "m19",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(4),
            NbCount::Wires(1),
            NbCount::Faces(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "m1", Prop::Area, "172.551");
    kind_is(s, "m1", ShapeEnum::Face);
    printed_mass(s, "m19", Prop::Area, "10");
    kind_is(s, "m19", ShapeEnum::Face);
    part_count(s, "s", Some(ShapeEnum::Face), 22);
}

pub fn B6(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex1.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_8", "s_9"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(460.84),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(194.168),
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
            NbCount::Edges(49),
            NbCount::Wires(23),
            NbCount::Faces(20),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "s_8",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    removal.check_removed(
        s,
        "s_9",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 22);
}

pub fn B7(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex1.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_7", "s_10"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(466.126),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(197.014),
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
            NbCount::Vertices(28),
            NbCount::Edges(43),
            NbCount::Wires(22),
            NbCount::Faces(18),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(s, "s_7", &[ShapeEnum::Edge, ShapeEnum::Face]);
    removal.check_removed(s, "s_10", &[ShapeEnum::Edge, ShapeEnum::Face]);
    let rf_hist = removal.savehistory();
    modified(s, "m12", &rf_hist, "s_12");
    modified(s, "m19", &rf_hist, "s_19");
    require_same_shapes(s, "m12", "m19", "Error: incorrect feature removal");
    checkprops(s, "m12", Prop::Area, Expected::Mass(13.9463));
    checknbshapes(
        s,
        "m12",
        Counting::Located,
        &[
            NbCount::Vertices(7),
            NbCount::Edges(7),
            NbCount::Wires(1),
            NbCount::Faces(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "m12", Prop::Area, "13.9463");
    kind_is(s, "m12", ShapeEnum::Face);
    printed_mass(s, "m19", Prop::Area, "13.9463");
    kind_is(s, "m19", ShapeEnum::Face);
    part_count(s, "s", Some(ShapeEnum::Face), 22);
}

pub fn B8(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex1.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_14", "s_20"]);
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
        Expected::Mass(197.735),
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
            NbCount::Edges(48),
            NbCount::Wires(20),
            NbCount::Faces(19),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "s_14",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    removal.check_removed(
        s,
        "s_20",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    let rf_hist = removal.savehistory();
    modified(s, "m1", &rf_hist, "s_1");
    checkprops(s, "m1", Prop::Area, Expected::Mass(174.62));
    checknbshapes(
        s,
        "m1",
        Counting::Located,
        &[
            NbCount::Vertices(12),
            NbCount::Edges(12),
            NbCount::Wires(2),
            NbCount::Faces(1),
        ],
    );
    modified(s, "m16", &rf_hist, "s_16");
    checkprops(s, "m16", Prop::Area, Expected::Mass(194.429));
    checknbshapes(
        s,
        "m16",
        Counting::Located,
        &[
            NbCount::Vertices(10),
            NbCount::Edges(10),
            NbCount::Wires(1),
            NbCount::Faces(1),
        ],
    );
    require_not_generated(s, "g1", &rf_hist, "s_1", "Error: incorrect feature removal");
    require_not_generated(
        s,
        "g16",
        &rf_hist,
        "s_16",
        "Error: incorrect feature removal",
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "m1", Prop::Area, "174.62");
    kind_is(s, "m1", ShapeEnum::Face);
    printed_mass(s, "m16", Prop::Area, "194.429");
    kind_is(s, "m16", ShapeEnum::Face);
    part_count(s, "s", Some(ShapeEnum::Face), 22);
}
