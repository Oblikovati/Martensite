//! The intersection of a line or a 3d curve with a face
//! ([`IntCurvesFaceIntersector`]) or with the faces of a shape
//! ([`IntCurvesFaceShapeIntersector`]), through IntCurveSurface_HInter and the
//! BRepTopAdaptor classification of the UV points.
mod internal_call;
mod intersector;
mod perform;
mod sample_pars;
mod shape_intersector;

pub use intersector::IntCurvesFaceIntersector;
pub use shape_intersector::IntCurvesFaceShapeIntersector;
