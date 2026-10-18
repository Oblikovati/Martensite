//! Curve interfaces and bounded line and offset adaptors.
//!
//! Adaptors expose a common evaluation contract through shared handles.
//! `Adaptor2dCurve2dBase` supplies defaults for unsupported curve capabilities.
mod curve2d;
pub use curve2d::{Adaptor2dCurve2d, Adaptor2dCurve2dBase, Adaptor2dD3};
mod line2d;
pub use line2d::Adaptor2dLine2d;
mod offset_curve;
mod offset_curve_kind;
mod offset_curve_shape;
pub use offset_curve::Adaptor2dOffsetCurve;
