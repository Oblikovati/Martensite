//! `GeomEvalRepCurveDesc`: descriptors that redirect the evaluation of a curve
//! to another curve (its "evaluation representation").
//!
//! The retained descriptors expose public mutable fields through shared handles;
//! here each descriptor keeps them behind a lock with getters and setters.
martensite_tkg2d::define_rep_curve_descriptors!(dyn crate::geom::GeomCurve);
