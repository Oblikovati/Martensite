//! LProp: the shared vocabulary of the local properties of curves and surfaces.
//!
//! The package holds the property status ([`Status`]), the kinds of particular
//! points of a curve ([`CIType`]) and their sorted store ([`CurAndInf`]), the two
//! retained exceptions (as [`Failure`] constructors), and `CurveUtils`, the
//! algorithms of the curve property classes ([`curve_utils`]). `CLProps3d` is
//! the curve property class on an `Adaptor3dCurve` handle ([`CLProps3d`]).
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkmath::gp::Vector3;

mod cur_and_inf;
pub mod curve_access;
pub mod curve_utils;
pub use cur_and_inf::CurAndInf;

/// `Status`: the state of a lazily computed local property.
///
/// The order matters: the property classes test `status >= Defined`;
/// `Status::default() == Status::Undecided`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    /// Not computed yet.
    #[default]
    Undecided,
    /// Computed, and found undefined.
    Undefined,
    /// Known to be defined.
    Defined,
    /// Defined and stored.
    Computed,
}

/// `CIType`: the kind of a particular point of a curve;
/// `CIType::Inflection as i32 == 0`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CIType {
    /// A point of inflection.
    Inflection,
    /// A minimum of curvature.
    MinCur,
    /// A maximum of curvature.
    MaxCur,
}

/// `NotDefined`: the requested property does not exist at this point;
/// `not_defined("CLProps::Tangent: tangent undefined")`.
pub fn not_defined(message: &str) -> Failure {
    Failure::with_type("LProp_NotDefined", message, "")
}

/// `BadContinuity`: the curve is not continuous enough for the property.
///
/// No port path raises it (nor does the retained package since the continuity test
/// left CLProps); it keeps the retained type name;
/// `bad_continuity("CLProps: continuity C0; expected at least C2")`.
pub fn bad_continuity(message: &str) -> Failure {
    Failure::with_type("LProp_BadContinuity", message, "")
}

/// `CLProps3d`: the curve local properties of an `Adaptor3dCurve` handle;
/// `CLProps3d::with_parameter(adaptor, 0.5, 2, 1e-7)?.curvature()?`.
pub type CLProps3d = crate::geom_lprop::CLPropsBase<Vector3, Handle<dyn Adaptor3dCurve>>;
