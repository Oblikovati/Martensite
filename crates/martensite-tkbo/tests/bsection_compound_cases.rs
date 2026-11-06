//! Direct-member fixture regressions for the Boolean section algorithm.
use super::*;

#[test]
fn BSection__I9() {
    compound_section_case(
        "tests/boolean/bsection/I9",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN525_3367_1.rle"
        ),
        356.984,
    );
}

#[test]
fn BSection__J1() {
    compound_section_case(
        "tests/boolean/bsection/J1",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN525_3367_1_nofog.rle"
        ),
        356.984,
    );
}

#[test]
fn BSection__J2() {
    compound_section_case(
        "tests/boolean/bsection/J2",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN525_3367_2.rle"
        ),
        356.984,
    );
}

#[test]
fn BSection__J3() {
    compound_section_case(
        "tests/boolean/bsection/J3",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN525_3367_2_nofog.rle"
        ),
        356.984,
    );
}

#[test]
fn BSection__J4() {
    compound_section_case(
        "tests/boolean/bsection/J4",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN525_3403.rle"),
        1093.37,
    );
}

#[test]
fn BSection__J5() {
    compound_section_case(
        "tests/boolean/bsection/J5",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN525_3403_nofog.rle"
        ),
        1093.37,
    );
}

#[test]
fn BSection__J6() {
    compound_section_case(
        "tests/boolean/bsection/J6",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN539.rle"),
        2225.71,
    );
}

#[test]
fn BSection__J7() {
    compound_section_case(
        "tests/boolean/bsection/J7",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN539_nofog.rle"
        ),
        2224.79,
    );
}

#[test]
fn BSection__J8() {
    compound_section_case(
        "tests/boolean/bsection/J8",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN585.rle"),
        2225.7,
    );
}

#[test]
fn BSection__J9() {
    compound_section_case(
        "tests/boolean/bsection/J9",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN585_nofog.rle"
        ),
        2224.79,
    );
}

#[test]
fn BSection__K1() {
    compound_section_case(
        "tests/boolean/bsection/K1",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN594.rle"),
        30.8671,
    );
}

#[test]
fn BSection__K2() {
    compound_section_case(
        "tests/boolean/bsection/K2",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN594_nofog.rle"
        ),
        30.8671,
    );
}

#[test]
fn BSection__K3() {
    compound_section_case(
        "tests/boolean/bsection/K3",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN605.rle"),
        8.41959,
    );
}

#[test]
fn BSection__K4() {
    compound_section_case(
        "tests/boolean/bsection/K4",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN605_nofog.rle"
        ),
        8.41959,
    );
}

#[test]
fn BSection__K5() {
    compound_section_case(
        "tests/boolean/bsection/K5",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN664.rle"),
        718.013,
    );
}

#[test]
fn BSection__K6() {
    compound_section_case(
        "tests/boolean/bsection/K6",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN664_nofog.rle"
        ),
        718.013,
    );
}

#[test]
fn BSection__K7() {
    compound_section_case(
        "tests/boolean/bsection/K7",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN676.rle"),
        88.9434,
    );
}

#[test]
fn BSection__K8() {
    compound_section_case(
        "tests/boolean/bsection/K8",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN676_nofog.rle"
        ),
        88.9434,
    );
}

#[test]
fn BSection__L3() {
    compound_section_case(
        "tests/boolean/bsection/L3",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN745.rle"),
        96.7669,
    );
}

#[test]
fn BSection__L4() {
    compound_section_case(
        "tests/boolean/bsection/L4",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN745_nofog.rle"
        ),
        96.7669,
    );
}

#[test]
fn BSection__L5() {
    compound_section_case(
        "tests/boolean/bsection/L5",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN746.rle"),
        556.6,
    );
}

#[test]
fn BSection__L6() {
    compound_section_case(
        "tests/boolean/bsection/L6",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN746_nofog.rle"
        ),
        556.6,
    );
}

#[test]
fn BSection__L7() {
    compound_section_case(
        "tests/boolean/bsection/L7",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN747.rle"),
        72.4212,
    );
}

#[test]
fn BSection__L8() {
    compound_section_case(
        "tests/boolean/bsection/L8",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN747_nofog.rle"
        ),
        72.4222,
    );
}

#[test]
fn BSection__L9() {
    compound_section_case(
        "tests/boolean/bsection/L9",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN757.rle"),
        683.183,
    );
}

#[test]
fn BSection__M1() {
    compound_section_case(
        "tests/boolean/bsection/M1",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN757_nofog.rle"
        ),
        683.183,
    );
}

#[test]
fn BSection__M2() {
    compound_section_case(
        "tests/boolean/bsection/M2",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN758.rle"),
        22.2575,
    );
}

#[test]
fn BSection__M4() {
    compound_section_case(
        "tests/boolean/bsection/M4",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN776.rle"),
        226.488,
    );
}

#[test]
fn BSection__M5() {
    compound_section_case(
        "tests/boolean/bsection/M5",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN776_nofog.rle"
        ),
        226.487,
    );
}

#[test]
fn BSection__M7() {
    compound_section_case(
        "tests/boolean/bsection/M7",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN778_nofog.rle"
        ),
        456.707,
    );
}

#[test]
fn BSection__P2() {
    compound_section_case(
        "tests/boolean/bsection/P2",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CIN901_intcqhmj.rle"
        ),
        126.535,
    );
}

#[test]
fn BSection__P7() {
    compound_section_case(
        "tests/boolean/bsection/P7",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CIN902_intcqhmr.rle"
        ),
        202.944,
    );
}

#[test]
fn BSection__O9() {
    let source = "tests/boolean/bsection/O9";
    let fixture = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60302.rle"
    ));
    let plane = BRepBuilderAPIMakeFace::from_plane_bounds(
        Plane3::default(),
        (-100.0, 100.0, -100.0, 100.0),
    )
    .unwrap_or_else(|failure| panic!("{source}: plane face failed: {failure:?}"))
    .face()
    .unwrap_or_else(|failure| panic!("{source}: expected plane face: {failure:?}"))
    .into_shape();
    let section = BRepAlgoAPISection::from_shapes(&fixture, &plane, None, true)
        .unwrap_or_else(|failure| panic!("{source}: bsection failed: {failure:?}"));
    assert!(section.is_done(), "{source}: bsection incomplete");
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: length failed: {failure:?}"));
    let length = properties.mass();
    assert!(
        (length - 25.708).abs() / 25.708 <= 0.01,
        "{source}: length {length}; expected 25.708 within 1%"
    );
}

fn compound_from_members(members: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut compound = martensite_tkbrep::topods::Compound::default();
    builder.make_compound(&mut compound);
    for member in members {
        builder.add(&mut compound, member).unwrap();
    }
    compound.into_shape()
}

#[test]
fn BSection__P1() {
    let source = "tests/boolean/bsection/P1";
    let fixture = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CIN901_intcqhmi.rle"
    ));
    let members: Vec<_> = ShapeIterator::new(&fixture, false, true).collect();
    assert_eq!(members.len(), 4, "{source}: expected four direct members");
    let first = compound_from_members(&[members[0].clone(), members[2].clone()]);
    let second = compound_from_members(&[members[1].clone(), members[3].clone()]);
    let section = BRepAlgoAPISection::from_shapes(&first, &second, None, true)
        .unwrap_or_else(|failure| panic!("{source}: bsection failed: {failure:?}"));
    assert!(section.is_done(), "{source}: bsection incomplete");
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: length failed: {failure:?}"));
    let length = properties.mass();
    assert!(
        (length - 40.5172).abs() / 40.5172 <= 0.01,
        "{source}: length {length}; expected 40.5172 within 1%"
    );
}

fn completed_section(source: &str, first: &Shape, second: &Shape) -> Shape {
    let section = BRepAlgoAPISection::from_shapes(first, second, None, true)
        .unwrap_or_else(|failure| panic!("{source}: bsection failed: {failure:?}"));
    assert!(section.is_done(), "{source}: bsection incomplete");
    section.shape().clone()
}

#[test]
fn BSection__P4() {
    let source = "tests/boolean/bsection/P4";
    let fixture = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/intcq004.brep"
    ));
    let members: Vec<_> = ShapeIterator::new(&fixture, false, true).collect();
    assert!(members.len() >= 5, "{source}: expected five direct members");
    let grouped =
        compound_from_members(&[members[0].clone(), members[2].clone(), members[4].clone()]);
    let primary = completed_section(source, &grouped, &members[1]);
    let secondary = completed_section(source, &members[3], &members[1]);
    let result = compound_from_members(&[primary, secondary]);
    let mut properties = GProps::default();
    BRepGProp::linear_properties(&result, &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: length failed: {failure:?}"));
    let length = properties.mass();
    assert!(
        (length - 682.142).abs() / 682.142 <= 0.01,
        "{source}: length {length}; expected 682.142 within 1%"
    );
}
