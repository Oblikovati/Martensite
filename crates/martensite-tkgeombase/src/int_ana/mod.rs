//! IntAna: the analytic intersections of conics and quadrics.
//!
//! [`QuadQuadGeo`] is the geometric intersection of two natural quadrics (and
//! tori), [`IntQuadQuad`] the analytic cylinder/cone-quadric intersection as
//! [`Curve`] pieces, [`IntConicQuad`] the conic-quadric intersection (with the
//! plane special cases) and [`IntLinTorus`] the line-torus intersection.
//! [`Quadric`] is a quadric equation and [`ResultType`] the kind of a geometric
//! result. Each retained constructor overload is a named constructor and each
//! Perform overload a `perform_*` method; C++ exceptions are `Err` values of
//! the same type. The deprecated alias ListOfCurve is [`ListOfCurve`].
mod curve;
mod int_conic_quad;
mod int_lin_torus;
mod int_quad_quad;
mod quad_quad_geo;
mod quadric;
mod result_type;

pub use curve::Curve;
pub use int_conic_quad::IntConicQuad;
pub use int_lin_torus::IntLinTorus;
pub use int_quad_quad::IntQuadQuad;
pub use quad_quad_geo::QuadQuadGeo;
pub use quadric::Quadric;
pub use result_type::ResultType;

/// ListOfCurve (Deprecated/NCollectionAliases): a list of [`Curve`].
#[deprecated(note = "Use martensite_tkernel::ncollection::List<Curve> directly")]
pub type ListOfCurve = martensite_tkernel::ncollection::List<Curve>;
