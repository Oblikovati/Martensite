//! Original IntWalk algorithm with Contap's surface function and walking-line state.
use super::{ContapSurfFunction, ContapTheIWLineOfTheIWalking};
use martensite_tkgeomalgo::{
    int_imp::HSurfaceTool,
    int_walk::{IWalkingTypes, IntWalkIWalking},
};
/// Native template arguments from `Contap_TheIWalking`.
/// Used through `ContapTheIWalking::new(1e-7,0.01,0.1,true)`.
pub struct ContapTheIWalkingTypes;
impl IWalkingTypes for ContapTheIWalkingTypes {
    type SurfaceTool = HSurfaceTool;
    type Function = ContapSurfFunction;
    type Line = ContapTheIWLineOfTheIWalking;
}
/// Walk contour polylines using the native IntWalk state machine.
/// `ContapTheIWalking::new(1e-7,0.01,0.1,true)?.is_done()`.
pub type ContapTheIWalking = IntWalkIWalking<ContapTheIWalkingTypes>;
