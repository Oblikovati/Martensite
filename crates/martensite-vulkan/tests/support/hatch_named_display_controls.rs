//! Own-mode construction controls stay independent of a driver or image baseline.
use super::*;

#[test]
fn bare_display_object_keeps_absent_own_mode() {
    let shape = box_shape([1., 2., 3.]);
    let old = AisShape::new(&shape);
    let actual = source_display_object(&shape, None).unwrap();
    assert_eq!(actual.has_display_mode(), old.has_display_mode());
    assert_eq!(actual.display_mode(), old.display_mode());
    assert!(actual.shape().is_same(&shape));
}

#[test]
fn explicit_wireframe_mode_exists_before_publication_or_display() {
    let shape = box_shape([1., 2., 3.]);
    let actual = source_display_object(&shape, Some(0)).unwrap();
    assert!(actual.has_display_mode());
    assert_eq!(actual.display_mode(), 0);
    assert_eq!(actual.display_status(), PrsMgrDisplayStatus::None);
    assert!(actual.shape().is_same(&shape));
}

#[test]
fn unsupported_own_mode_is_refused_before_publication() {
    let shape = box_shape([1., 2., 3.]);
    let result = source_display_object(&shape, Some(99));
    assert!(
        result.is_err(),
        "mode=99; expected unsupported mode refusal"
    );
}
