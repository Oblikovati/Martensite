//! The two templates that find the start points of a walking
//! on a surface domain, on its restrictions (`IntStart_SearchOnBoundaries.gxx`) and
//! inside it (`IntStart_SearchInside.gxx`). The package has no header of its own:
//! IntPatch and Contap instantiate the templates. Each template is one Rust generic
//! over the tool traits of [`tools`].
pub(crate) mod failures;
mod search_inside;
mod search_on_boundaries;
pub mod tools;

pub use search_inside::SearchInside;
pub use search_on_boundaries::SearchOnBoundaries;
pub use tools::{
    SearchInsideTypes, SearchOnBoundariesTypes, TheArc, TheArcFunction, TheInsideFunction,
    ThePathPoint, TheSOBTool, TheSegment, TheTopolTool, TheVertex,
};
