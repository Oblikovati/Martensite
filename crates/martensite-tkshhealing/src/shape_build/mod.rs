//! ShapeBuild: the low-level builders of Shape Healing.
//!
//! [`ShapeBuildVertex`] combines two vertices into one that holds both,
//! [`ShapeBuildEdge`] copies edges with other vertices, ranges or p-curves, moves and
//! transforms p-curves and makes edges on curves, and [`ShapeBuildReShape`] is the
//! re-shaper that records replacements and removals of shapes and rebuilds any shape that
//! contains them. Replacements are applied recursively through that context.
mod edge;
mod reshape;
mod vertex;

pub use edge::ShapeBuildEdge;
pub use reshape::ShapeBuildReShape;
pub use vertex::ShapeBuildVertex;
