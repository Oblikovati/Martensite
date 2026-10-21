//! Native IntStart_SearchInside specialization with Contap's surface equation.
use super::ContapSurfFunction;
use martensite_tkgeomalgo::{
    int_imp::HSurfaceTool,
    int_start::{SearchInside, SearchInsideTypes, TheTopolTool},
};
/// Type arguments from `Contap_TheSearchInside`.
/// Used through `ContapTheSearchInside::new()`.
pub struct ContapTheSearchInsideTypes;
impl SearchInsideTypes for ContapTheSearchInsideTypes {
    type SurfaceTool = HSurfaceTool;
    type TopolTool = dyn TheTopolTool;
    type Function = ContapSurfFunction;
}
/// Search contour seeds in a surface domain using the original IntStart algorithm.
/// `ContapTheSearchInside::new().is_done()`.
pub type ContapTheSearchInside = SearchInside<ContapTheSearchInsideTypes>;
