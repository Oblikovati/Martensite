//! The classification of a 2d point (or of the projection of
//! a 3d point) against a face, IN, OUT or ON its boundary.
//!
//! The classifiers are the TopClass templates of the geometric algorithms instantiated on the
//! BRepClass edge, intersector and face explorer:
//! [`FClass2dOfFClassifier`] and [`FacePassiveClassifier`] are
//! `topclass::Classifier2d<BRepClassIntersector>`, [`FClassifier`] is
//! `topclass::FaceClassifier<BRepClassFaceExplorer>`, and
//! [`BRepClassFaceClassifier`] derives from it.
mod edge;
mod face_classifier;
mod face_explorer;
mod face_explorer_build;
mod face_explorer_segment;
mod intersector;
mod intersector_checks;

pub use edge::{BRepClassEdge, BndBoxState};
pub use face_classifier::BRepClassFaceClassifier;
pub use face_explorer::BRepClassFaceExplorer;
pub use intersector::BRepClassIntersector;

use martensite_tkgeomalgo::topclass::{Classifier2d, FaceClassifier};

/// `BRepClass_FClass2dOfFClassifier`: the TopClass 2d classifier over the BRepClass
/// intersector; `let mut classifier = FClass2dOfFClassifier::new();`.
pub type FClass2dOfFClassifier = Classifier2d<BRepClassIntersector>;

/// `BRepClass_FacePassiveClassifier`: the same TopClass 2d classifier (its header is
/// identical to `BRepClass_FClass2dOfFClassifier`);
/// `let mut classifier = FacePassiveClassifier::new();`.
pub type FacePassiveClassifier = Classifier2d<BRepClassIntersector>;

/// `BRepClass_FClassifier`: the TopClass face classifier over the BRepClass face
/// explorer; `FClassifier::with_point(&mut explorer, &point, 1e-7)?`.
pub type FClassifier = FaceClassifier<BRepClassFaceExplorer>;
