//! Intersection records and face classifiers used by topological operation builders.
mod bipoint;
mod point_classifier;
mod types;

pub use bipoint::TopOpeBRepBipoint;
pub use point_classifier::TopOpeBRepPointClassifier;
pub use types::{TopOpeBRep, TopOpeBRepP2DStatus, TopOpeBRepTypeLineCurve};

mod vpoint_inter;
pub use vpoint_inter::TopOpeBRepVPointInter;

mod wpoint_inter;
pub use wpoint_inter::TopOpeBRepWPointInter;

mod hctxff2d;
pub use hctxff2d::TopOpeBRepHctxff2d;

mod hctxee2d;
pub use hctxee2d::TopOpeBRepHctxee2d;

mod point2d;
pub use point2d::TopOpeBRepPoint2d;

mod edges_intersector;
pub use edges_intersector::TopOpeBRepEdgesIntersector;

mod faces_intersector;
pub use faces_intersector::{TopOpeBRepFacesIntersector, TopOpeBRepFacesLineMut};

mod line_inter;
pub use line_inter::TopOpeBRepLineInter;

mod vpoint_inter_iterator;
pub use vpoint_inter_iterator::TopOpeBRepVPointInterIterator;
mod wpoint_inter_iterator;
pub use wpoint_inter_iterator::TopOpeBRepWPointInterIterator;
mod face_edge_intersector;
pub use face_edge_intersector::TopOpeBRepFaceEdgeIntersector;
mod point_geom_tool;
pub use point_geom_tool::TopOpeBRepPointGeomTool;
mod vpoint_inter_classifier;
pub use vpoint_inter_classifier::TopOpeBRepVPointInterClassifier;
mod ff_transition_tool;
pub use ff_transition_tool::TopOpeBRepFFTransitionTool;
mod ff_transition_geometry;
mod geom_tool;
pub use geom_tool::TopOpeBRepGeomTool;
mod faces_filler;
pub use faces_filler::TopOpeBRepFacesFiller;
mod kpart;
pub use kpart::{TopOpeBRepKPartContext, funbrep_topokpart};

/// Geometry selection state shared by face-intersection vertex operations.
pub use faces_filler::TopOpeBRepFacesFillerGeometrySearch;

pub use faces_filler::{
    TopOpeBRepFacesFillerArrayMut, TopOpeBRepFacesFillerLine, TopOpeBRepFacesFillerLineMut,
    TopOpeBRepFacesFillerTransition,
};

mod ff_dumper;
pub use ff_dumper::{
    TopOpeBRepFFDumper, TopOpeBRepFFDumperBaseType, TopOpeBRepFFDumperView,
    TopOpeBRepFacesFillerContext,
};
