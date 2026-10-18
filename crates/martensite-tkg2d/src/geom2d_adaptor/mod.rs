//! The Geom2dAdaptor package: Geom2d curves behind the Adaptor2d interface, and the
//! way back from an adapted curve to a Geom2d curve.
mod curve;
mod curve_eval;
mod curve_eval_order;
mod curve_query;
mod curve_shape;
mod curve_state;
pub mod knot_continuity;
mod make_curve;
mod public_curve_data;
pub mod span_cache;
pub use curve::Geom2dAdaptorCurve;
pub use make_curve::make_curve;
pub use martensite_tkmath::bsplclib::SharedBSplineCache;
pub use public_curve_data::{BSplineData, BezierData, CurveDataVariant, OffsetData};
