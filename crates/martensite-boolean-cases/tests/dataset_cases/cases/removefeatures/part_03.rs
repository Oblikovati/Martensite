//! The `removefeatures` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn B9(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex1.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_15", "s_21", "s_22"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(449.707),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(186.844),
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
            NbCount::Edges(48),
            NbCount::Wires(20),
            NbCount::Faces(18),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "s_15",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    removal.check_removed(
        s,
        "s_21",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    removal.check_removed(
        s,
        "s_22",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    let rf_hist = removal.savehistory();
    modified(s, "m1", &rf_hist, "s_1");
    checkprops(s, "m1", Prop::Area, Expected::Mass(175.593));
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
    require_not_generated(s, "g1", &rf_hist, "s_1", "Error: incorrect feature removal");
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "m1", Prop::Area, "175.593");
    kind_is(s, "m1", ShapeEnum::Face);
    part_count(s, "s", Some(ShapeEnum::Face), 22);
}

pub fn C2(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex2.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(s, &["s_7", "s_8", "s_9", "s_10"], "gap");
    removal.remove(s, "res1", "s", &["gap"]);
    checkshape_named(s, "res1");
    checkprops(s, "res1", Prop::Area, Expected::Mass(624.));
    checkprops(s, "res1", Prop::Volume, Expected::Mass(992.));
    checknbshapes(
        s,
        "res1",
        Counting::Located,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(24),
            NbCount::Wires(12),
            NbCount::Faces(12),
            NbCount::Shells(2),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "gap",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    explode(s, "s", None);
    removal.remove(s, "res2", "s", &["s_2"]);
    checkshape_named(s, "res2");
    checkprops(s, "res2", Prop::Area, Expected::Mass(608.));
    checkprops(s, "res2", Prop::Volume, Expected::Mass(992.));
    checknbshapes(
        s,
        "res2",
        Counting::Located,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(24),
            NbCount::Wires(10),
            NbCount::Faces(10),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "s_2",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    removal.remove(s, "result", "s", &["gap", "s_2"]);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(1000.));
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
        "gap",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    removal.check_removed(
        s,
        "s_2",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 16);
    part_count(s, "s", None, 2);
}

pub fn C3(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex3.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(s, &["s_7", "s_8", "s_9", "s_11", "s_12", "s_17"], "gap");
    removal.remove(s, "result", "s", &["gap"]);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(1200.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(2000.));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(12),
            NbCount::Edges(20),
            NbCount::Wires(11),
            NbCount::Faces(11),
            NbCount::Shells(2),
            NbCount::Solids(2),
        ],
    );
    removal.check_removed(
        s,
        "gap",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    let rf_hist = removal.savehistory();
    require_bopcheck_ok(s, "result", "Error: sharing is lost after removal");
    modified(s, "m6", &rf_hist, "s_6");
    checkprops(s, "m6", Prop::Area, Expected::Mass(100.));
    checknbshapes(
        s,
        "m6",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(4),
            NbCount::Wires(1),
            NbCount::Faces(1),
        ],
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "m6", Prop::Area, "100");
    kind_is(s, "m6", ShapeEnum::Face);
    part_count(s, "s", Some(ShapeEnum::Face), 17);
}

pub fn C5(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ex3.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(s, &["s_7", "s_8", "s_9"], "gap1");
    compound(s, &["s_11", "s_12", "s_17"], "gap2");
    removal.remove(s, "res1", "s", &["gap1"]);
    checkshape_named(s, "res1");
    checkprops(s, "res1", Prop::Area, Expected::Mass(1200.));
    checkprops(s, "res1", Prop::Volume, Expected::Mass(1875.));
    checknbshapes(
        s,
        "res1",
        Counting::Located,
        &[
            NbCount::Vertices(19),
            NbCount::Edges(31),
            NbCount::Wires(15),
            NbCount::Faces(15),
            NbCount::Shells(2),
            NbCount::Solids(2),
        ],
    );
    removal.check_removed(s, "gap1", &[ShapeEnum::Edge, ShapeEnum::Face]);
    require_bopcheck_ok(s, "res1", "Error: sharing is lost after removal");
    let rf_hist = removal.savehistory();
    modified(s, "m6", &rf_hist, "s_6");
    checkprops(s, "m6", Prop::Area, Expected::Mass(100.));
    explode(s, "s", None);
    modified(s, "m1", &rf_hist, "s_1");
    checkprops(s, "m1", Prop::Area, Expected::Mass(600.));
    checkprops(s, "m1", Prop::Volume, Expected::Mass(1000.));
    removal.remove(s, "res2", "s", &["gap2"]);
    checkshape_named(s, "res2");
    checkprops(s, "res2", Prop::Area, Expected::Mass(1200.));
    checkprops(s, "res2", Prop::Volume, Expected::Mass(1875.));
    checknbshapes(
        s,
        "res2",
        Counting::Located,
        &[
            NbCount::Vertices(19),
            NbCount::Edges(31),
            NbCount::Wires(15),
            NbCount::Faces(15),
            NbCount::Shells(2),
            NbCount::Solids(2),
        ],
    );
    removal.check_removed(s, "gap2", &[ShapeEnum::Edge, ShapeEnum::Face]);
    let rf_hist2 = removal.savehistory();
    require_bopcheck_ok(s, "res2", "Error: sharing is lost after removal");
    modified(s, "m6", &rf_hist2, "s_6");
    checkprops(s, "m6", Prop::Area, Expected::Mass(100.));
    explode(s, "s", None);
    modified(s, "m2", &rf_hist2, "s_2");
    checkprops(s, "m2", Prop::Area, Expected::Mass(600.));
    checkprops(s, "m2", Prop::Volume, Expected::Mass(1000.));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "m6", Prop::Area, "100");
    kind_is(s, "m6", ShapeEnum::Compound);
    printed_mass(s, "m1", Prop::Area, "600");
    kind_is(s, "m1", ShapeEnum::Solid);
    printed_mass(s, "m2", Prop::Area, "600");
    kind_is(s, "m2", ShapeEnum::Solid);
    part_count(s, "s", Some(ShapeEnum::Face), 17);
    part_count(s, "s", None, 2);
    part_count(s, "s", None, 2);
}

pub fn C6(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ANC101.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_22", "s_24"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(441102.),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(8.18199e+006),
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
            NbCount::Vertices(127),
            NbCount::Edges(192),
            NbCount::Wires(115),
            NbCount::Faces(85),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "s_22",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    removal.check_removed(
        s,
        "s_24",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 87);
}

pub fn C7(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ANC101.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(s, &["s_52", "s_23", "s_22", "s_54", "s_24", "s_53"], "cyl");
    removal.remove(s, "result", "s", &["cyl"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(439513.),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(8.11395e+006),
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
            NbCount::Vertices(119),
            NbCount::Edges(180),
            NbCount::Wires(111),
            NbCount::Faces(81),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "cyl",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 87);
}

pub fn C8(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ANC101.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_83"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(441340.),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(8.18282e+006),
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
            NbCount::Vertices(130),
            NbCount::Edges(196),
            NbCount::Wires(116),
            NbCount::Faces(86),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(s, "s_83", &[ShapeEnum::Edge, ShapeEnum::Face]);
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 87);
}
