//! The IGES format in the data exchange: its settings and its provider.
mod modes;
mod node;
mod parameters;
mod provider;
mod runtime;
mod serialization;
mod shape_fix;
pub use modes::{
    DeIgesReadModeBSplineContinuity, DeIgesReadModeMaxPrecision, DeIgesReadModePrecision,
    DeIgesReadModeSurfaceCurve, DeIgesWriteModeBRep, DeIgesWriteModeConvertSurface,
    DeIgesWriteModePlaneMode, DeIgesWriteModePrecisionMode,
};
pub use node::DeIgesConfigurationNode;
pub use parameters::{DeIgesParameters, xs_shape_fix_parameters};
pub use provider::DeIgesProvider;
pub use shape_fix::transfer_shape_fix_parameters;
mod static_initialization;
