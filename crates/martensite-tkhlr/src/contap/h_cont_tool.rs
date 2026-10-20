//! Native contour sampling and arc queries.
/// Contap_HContTool: static queries. The arc and vertex members are the ones of
/// `IntPatch_HInterTool` (the two native classes have the same bodies); the surface
/// sampling is Contap's own and keeps its range in process-wide statics.
///
/// ```
/// use martensite_tkhlr::contap::ContapHContTool;
/// assert!(!ContapHContTool::has_been_seen(&Default::default()));
/// assert!(ContapHContTool::value(&Default::default(), 1).is_err());
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct ContapHContTool;

mod arcs;
pub(super) mod sampling;
mod traits;
