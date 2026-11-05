//! `bcut_complex` bodies written by hand: they keep checks an earlier test made during the script.
use crate::support::{
    beyond_script::{kind_is, part_count, printed_mass, tolerances, view_mass_is_nonzero},
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::explode,
    construction::{breducetolerance, btolx, copy},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bcut,
    properties::Prop,
    restore::restore,
    session::Session,
    surface_commands::{mkface_bounded, mksurface, untrimmed},
    volume_commands::mkvolume,
};
use martensite_tkbrep::{
    brep::{BRepTool, access::edge_tshape},
    topexp::{self, Explorer},
    topods,
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;

/// A native run's tolerance of every vertex, edge and face of both operands of F7.
const F7_NATIVE_TOLERANCES: &str = include_str!("../../../src/support/f7_native_tolerances.txt");

/// Beyond the script: every tolerance prints as a native run printed it at this stage.
fn f7_tolerances_at(s: &dyn Session, operand: &str, stage: &str) {
    let mut records = Vec::new();
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        for (index, value) in tolerances(s, operand, kind).into_iter().enumerate() {
            let printed = format_general(value, 6);
            records.push(format!(
                "{operand} {stage} {} {} {printed}",
                kind as i32,
                index + 1
            ));
        }
    }
    let prefix = format!("{operand} {stage} ");
    let expected: Vec<&str> = F7_NATIVE_TOLERANCES
        .lines()
        .filter(|line| line.starts_with(&prefix))
        .collect();
    assert_eq!(records, expected, "F7 {operand} {stage}: native tolerances");
}

/// Beyond the script: `breducetolerance` changes the vertices alone.
fn f7_reduced(s: &dyn Session, operand: &str) {
    f7_tolerances_at(s, operand, "before");
    let edges = tolerances(s, operand, ShapeEnum::Edge);
    let faces = tolerances(s, operand, ShapeEnum::Face);
    breducetolerance(s, operand);
    f7_tolerances_at(s, operand, "after");
    assert_eq!(
        edges,
        tolerances(s, operand, ShapeEnum::Edge),
        "edges of {operand}"
    );
    assert_eq!(
        faces,
        tolerances(s, operand, ShapeEnum::Face),
        "faces of {operand}"
    );
}

/// Beyond the script: `explode` binds the solid the explorer finds first.
fn f7_first_solid(s: &dyn Session, operand: &str) {
    let shape = s.shape(operand).expect("restored operand");
    let first = Explorer::new(shape, ShapeEnum::Solid, ShapeEnum::Shape).unwrap();
    assert!(first.more(), "F7 {operand}: expected a solid");
    let bound = s.shape(&format!("{operand}_1")).expect("exploded solid");
    assert!(
        bound.is_same(first.current()),
        "F7 {operand}_1: first solid"
    );
}

pub fn F7(s: &mut dyn Session) {
    restore(s, "CTO900_buc60219a.rle", "a");
    restore(s, "CTO900_buc60219b.rle", "b");
    f7_reduced(s, "a");
    f7_reduced(s, "b");
    explode(s, "a", Some(ShapeEnum::Solid));
    explode(s, "b", Some(ShapeEnum::Solid));
    f7_first_solid(s, "a");
    f7_first_solid(s, "b");
    bcut(s, "result", "a_1", "b_1");
    checkprops(s, "result", Prop::Area, Expected::Mass(3020.55));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "b_1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a_1", Prop::Area, "4096.47");
    kind_is(s, "a_1", ShapeEnum::Solid);
    printed_mass(s, "b_1", Prop::Area, "5051.05");
    kind_is(s, "b_1", ShapeEnum::Solid);
    part_count(s, "a", Some(ShapeEnum::Solid), 1);
    part_count(s, "b", Some(ShapeEnum::Solid), 1);
    view_mass_is_nonzero(s, "result");
}

/// Beyond the script: the tool's tolerances as a native run reads them.
fn h7_tool_tolerances(s: &dyn Session, vertex: f64, edge: f64) {
    assert_eq!(
        tolerances(s, "tool", ShapeEnum::Vertex),
        [vertex; 2],
        "H7 vertices"
    );
    assert_eq!(
        tolerances(s, "tool", ShapeEnum::Edge),
        [edge, 1.0e-7, 1.0e-7],
        "H7 edges"
    );
    assert_eq!(
        tolerances(s, "tool", ShapeEnum::Face),
        [1.0e-7; 3],
        "H7 faces"
    );
}

/// Beyond the script: the tool's edges keep their flags and curve representations.
fn h7_tool_flags(s: &dyn Session) {
    let mut edges = IndexedMapOfShape::default();
    let tool = s.shape("tool").expect("restored tool");
    topexp::map_shapes_of_type(tool, ShapeEnum::Edge, &mut edges).unwrap();
    assert_eq!(edges.extent(), 3, "H7 edges of the tool");
    for (index, representations) in [2, 5, 5].into_iter().enumerate() {
        let shape = edges.find_key(index + 1).unwrap();
        let edge = topods::edge(shape).unwrap();
        assert!(
            BRepTool::same_parameter(&edge).unwrap(),
            "H7 same parameter"
        );
        assert!(BRepTool::same_range(&edge).unwrap(), "H7 same range");
        assert!(!BRepTool::degenerated(&edge).unwrap(), "H7 not degenerated");
        let curves = edge_tshape(shape, "H7 edge").unwrap().curves().len();
        assert_eq!(
            curves, representations,
            "H7 representations of edge {index}"
        );
    }
}

pub fn H7(s: &mut dyn Session) {
    restore(s, "CTO900_pro14240c.rle", "part");
    restore(s, "pro14240d.rle", "tool");
    h7_tool_tolerances(s, 60.370_926_938_429, 60.370_926_938_429);
    h7_tool_flags(s);
    btolx(s, "tool");
    // A vertex keeps the increment a written file would lose.
    h7_tool_tolerances(s, 1.000_000_000_000_000_2e-7, 1.0e-7);
    h7_tool_flags(s);
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.23524e6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "part", Prop::Area, "2.22043e+06");
    kind_is(s, "part", ShapeEnum::Compound);
    printed_mass(s, "tool", Prop::Area, "18347.3");
    kind_is(s, "tool", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

/// `foreach s [explode r so] {foreach f [explode $s f] {if same {copy $s part; break}}}`.
/// Beyond the script: the search must find a solid, and the copy keeps its identity.
fn copy_solid_holding(s: &mut dyn Session, whole: &str, face: &str, copied: &str) {
    let held = s.shape(face).expect("the face searched for").clone();
    for solid in explode(s, whole, Some(ShapeEnum::Solid)) {
        for part in explode(s, &solid, Some(ShapeEnum::Face)) {
            if s.shape(&part).is_some_and(|found| found.is_same(&held)) {
                copy(s, &solid, copied);
                let same = s.shape(copied).unwrap().is_same(s.shape(&solid).unwrap());
                assert!(same, "F2 copy: expected the same topology");
                return;
            }
        }
    }
    panic!("F2 mkvolume: expected a solid holding {face}");
}

pub fn F2(s: &mut dyn Session) {
    restore(s, "buc60127-part.rle", "part");
    restore(s, "buc60127-tool.rle", "tool");
    explode(s, "part", Some(ShapeEnum::Face));
    let c_in = mksurface(s, "part_1");
    let c_in = untrimmed(&c_in);
    mkface_bounded(s, "f_in", &c_in, (0., 2. * std::f64::consts::PI, 0., 150.));
    let s1 = mksurface(s, "part_3");
    let s2 = mksurface(s, "part_4");
    mkface_bounded(s, "f1", &s1, (-10., 10., -10., 10.));
    mkface_bounded(s, "f2", &s2, (-10., 10., -10., 10.));
    mkvolume(s, "r", &["part_2", "f_in", "f1", "f2"]);
    copy_solid_holding(s, "r", "part_2", "part");
    bcut(s, "result", "part", "tool");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(5382.41));
    checkprops(s, "result", Prop::Volume, Expected::Mass(2643.38));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Wires(14),
            NbCount::Faces(11),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "part", Prop::Area, "5692.57");
    kind_is(s, "part", ShapeEnum::Solid);
    printed_mass(s, "tool", Prop::Area, "3769.91");
    kind_is(s, "tool", ShapeEnum::Solid);
    part_count(s, "part", Some(ShapeEnum::Face), 4);
}
