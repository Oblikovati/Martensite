//! Preserve direct-member order, accumulated orientation/location and compound aliases.
#[path = "boolean_unconditional_case.rs"]
pub(super) mod boolean_unconditional_case;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape, ShapeIterator},
};
use martensite_tkg3d::top_abs::ShapeEnum;
fn restored_members(bytes: &[u8], count: usize) -> Vec<Shape> {
    let root = boolean_unconditional_case::restored_shape(bytes, ShapeEnum::Compound);
    // Plain topology-command explode uses TopoDSIterator with both cumulative flags enabled.
    let members: Vec<Shape> = ShapeIterator::new(&root, true, true).collect();
    assert_eq!(
        members.len(),
        count,
        "native fixture: expected {count} direct members"
    );
    members
}
fn ordered_compound(members: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    for member in members {
        builder
            .add(&mut compound, member)
            .expect("source compound member insertion");
    }
    let shape = compound.into_shape();
    check_compound_aliases(&shape, members);
    shape
}
fn check_compound_aliases(compound: &Shape, expected: &[Shape]) {
    let actual: Vec<Shape> = ShapeIterator::new(compound, true, true).collect();
    assert_eq!(
        actual.len(),
        expected.len(),
        "compound must retain all source members"
    );
    // Native compare confirms same/equal members after compound in all four positions.
    for (member, original) in actual.iter().zip(expected) {
        assert!(
            member.is_equal(original),
            "compound must preserve ordered identity and orientation"
        );
    }
}
/// Cut the two direct shell members; `checked_o2()`.
pub fn checked_o2() -> Shape {
    let members = restored_members(
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro15042.rle"),
        2,
    );
    boolean_unconditional_case::checked_operation(&members[0], &members[1], Operation::Cut, 8387.96)
}
/// Cut the first solid by the ordered compound of members2..5; `checked_q9()`.
pub fn checked_q9() -> Shape {
    let members = restored_members(
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO907_topo001.brep"
        ),
        5,
    );
    let tool = ordered_compound(&members[1..5]);
    boolean_unconditional_case::checked_operation(&members[0], &tool, Operation::Cut, 57618.4)
}
