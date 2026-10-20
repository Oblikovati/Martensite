//! Supplemental BRepClass failure behavior; legacy cases are in brep_class_face_explorer.rs.
use martensite_tkbrep::topods::Face;
use martensite_tkmath::gp::Point2;
use martensite_tktopalgo::brep_class::{BRepClassFaceExplorer, FClassifier};

/// Deviation: the face bounds of a face without a
/// surface cannot be computed. Natively CheckPoint dereferences the null surface; this
/// crate keeps the failure in the explorer and the classification returns it.
#[test]
fn face_without_surface_classification_returns_the_bounds_failure() {
    let mut explorer = BRepClassFaceExplorer::new(&Face::default()).unwrap();
    let classified = FClassifier::with_point(&mut explorer, &Point2::new(1.0, 1.0), 1e-7);
    let failure = classified.err().expect("the null face has no bounds");
    assert!(!failure.exception_type().is_empty());
    let mut point = Point2::new(1.0, 1.0);
    assert!(explorer.check_point(&mut point).is_err());
}
