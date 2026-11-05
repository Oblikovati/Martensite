//! The `removefeatures` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn C9(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ANC101.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(s, &["s_51", "s_17", "s_14", "s_15", "s_16"], "cyls");
    removal.remove(s, "result", "s", &["cyls"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(422114.),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(8.34108e+006),
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
            NbCount::Vertices(121),
            NbCount::Edges(183),
            NbCount::Wires(102),
            NbCount::Faces(82),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "cyls",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 87);
}

pub fn D1(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ANC101.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(
        s,
        &[
            "s_66", "s_60", "s_63", "s_77", "s_76", "s_62", "s_68", "s_67", "s_65", "s_75", "s_61",
            "s_78", "s_64", "s_73", "s_79", "s_81", "s_80", "s_74",
        ],
        "cyls",
    );
    removal.remove(s, "result", "s", &["cyls"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(415106.),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(8.28188e+006),
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
            NbCount::Vertices(113),
            NbCount::Edges(171),
            NbCount::Wires(90),
            NbCount::Faces(69),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "cyls",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 87);
}

pub fn D2(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ANC101.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(
        s,
        &[
            "s_45", "s_57", "s_32", "s_59", "s_43", "s_31", "s_33", "s_47",
        ],
        "gaps",
    );
    removal.remove(s, "result", "s", &["gaps"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(440693.),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(8.18581e+006),
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
            NbCount::Vertices(115),
            NbCount::Edges(174),
            NbCount::Wires(109),
            NbCount::Faces(79),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "gaps",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 87);
}

pub fn D3(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ANC101.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(
        s,
        &[
            "s_28", "s_26", "s_32", "s_35", "s_47", "s_57", "s_12", "s_31", "s_58", "s_48", "s_34",
            "s_33", "s_39", "s_44", "s_29", "s_43", "s_41", "s_56", "s_11", "s_46", "s_59", "s_40",
            "s_38", "s_27", "s_45", "s_30", "s_37", "s_42", "s_36", "s_4", "s_10", "s_5", "s_3",
            "s_13",
        ],
        "gaps",
    );
    removal.remove(s, "result", "s", &["gaps"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(433947.),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(8.26107e+006),
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
            NbCount::Vertices(71),
            NbCount::Edges(107),
            NbCount::Wires(80),
            NbCount::Faces(53),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "gaps",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 87);
}

pub fn D4(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ANC101.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(s, &["s_86", "s_25", "s_72", "s_55"], "bottom");
    removal.remove(s, "result", "s", &["bottom"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(439076.),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(8.27381e+006),
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
            NbCount::Wires(111),
            NbCount::Faces(83),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(s, "bottom", &[ShapeEnum::Edge, ShapeEnum::Face]);
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 87);
}

pub fn D5(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ANC101.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(s, &["s_87", "s_69", "s_82"], "cyl");
    removal.remove(s, "result", "s", &["cyl"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(410701.),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(8.98858e+006),
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
            NbCount::Wires(111),
            NbCount::Faces(84),
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

pub fn D6(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ANC101.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_83", "s_70", "s_85"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(427464.),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(8.0484e+006),
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
            NbCount::Vertices(128),
            NbCount::Edges(193),
            NbCount::Wires(113),
            NbCount::Faces(84),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "s_83",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    removal.check_removed(
        s,
        "s_70",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    removal.check_removed(
        s,
        "s_85",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 87);
}

pub fn G2(s: &mut dyn Session) {
    let mut removal = FeatureRemoval::default();
    restore(s, "bug29481_ANC101.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    removal.remove(s, "result", "s", &["s_49"]);
    checkshape_named(s, "result");
    checkprops_within(
        s,
        "result",
        Prop::Area,
        Expected::Mass(501833.),
        CheckTolerances {
            deviation: 1.0e-7,
            ..CheckTolerances::default()
        },
    );
    checkprops_within(
        s,
        "result",
        Prop::Volume,
        Expected::Mass(8.60751e+006),
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
            NbCount::Vertices(129),
            NbCount::Edges(195),
            NbCount::Wires(116),
            NbCount::Faces(86),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    removal.check_removed(
        s,
        "s_49",
        &[ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "s", Some(ShapeEnum::Face), 87);
}
