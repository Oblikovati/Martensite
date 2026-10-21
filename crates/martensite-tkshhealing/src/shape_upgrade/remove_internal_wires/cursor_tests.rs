use super::*;

#[test]
fn removed_face_cursor_advances_after_a_successful_match() {
    let mut compound = martensite_tkbrep::topods::Compound::default();
    martensite_tkbrep::brep::BRepBuilder::default().make_compound(&mut compound);
    let candidate = compound.into_shape();
    let mut removed = List::default();
    removed.append(candidate.clone());
    let context = ShapeUpgradeRemoveInternalWires::new();
    let mut cursor = 0;
    assert!(matches_removed_face(&context, &candidate, &removed, &mut cursor).unwrap());
    assert_eq!(cursor, 1);
    assert!(!matches_removed_face(&context, &candidate, &removed, &mut cursor).unwrap());
}
