//! GC: construction of Geom and Geom2d objects.
//!
//! Each `Make*` class is a struct built by one named constructor per retained
//! overload; the 2D classes (`MakeCircle2d`...) build Geom2d objects. The
//! algorithms that derive from Root implement [`Root`] and return their handle
//! through `value()` (not-done failure when the construction failed) and the
//! `TryFrom` conversion that replaces `operator const occ::handle<T>&()`; the
//! transformation makers always succeed. A retained constructor that can raise
//! through gp, Geom or Geom2d returns `Result`.

/// The members that every Root algorithm shares: `Status` through [`Root`],
/// `Value()` and the conversion operator to the handle.
macro_rules! gc_result {
    ($maker:ident, $result:ty, $native:literal) => {
        impl $crate::gce::Root for $maker {
            fn status(&self) -> $crate::gce::ErrorType {
                self.the_error
            }
        }
        impl $maker {
            #[doc = concat!("`", $native, "::Value`: the constructed handle, or StdFail_NotDone ")]
            #[doc = "when the construction failed; `maker.value()?`."]
            pub fn value(
                &self,
            ) -> Result<
                martensite_tkernel::standard::Handle<$result>,
                martensite_tkernel::standard::Failure,
            > {
                $crate::gce::check_done(self.the_error, concat!($native, "::Value()"))?;
                Ok(self.the_result.clone())
            }
            // Not every algorithm has a failure path or builds its own result.
            #[allow(dead_code)]
            fn failed(error: $crate::gce::ErrorType) -> Self {
                Self {
                    the_result: martensite_tkernel::standard::Handle::default(),
                    the_error: error,
                }
            }
            #[allow(dead_code)]
            fn built(result: $result) -> Self {
                Self {
                    the_result: martensite_tkernel::standard::Handle::new(result),
                    the_error: $crate::gce::ErrorType::Done,
                }
            }
        }
        #[doc = concat!("The conversion operator of `", $native, "`; `Handle::try_from(&maker)?`.")]
        impl TryFrom<&$maker> for martensite_tkernel::standard::Handle<$result> {
            type Error = martensite_tkernel::standard::Failure;
            fn try_from(maker: &$maker) -> Result<Self, Self::Error> {
                maker.value()
            }
        }
    };
}

/// `Value()` and the conversion operator of a transformation maker.
macro_rules! gc_transformation {
    ($maker:ident, $result:ty, $native:literal) => {
        impl $maker {
            #[doc = concat!("`", $native, "::Value`: the transformation; `maker.value()`.")]
            pub fn value(&self) -> martensite_tkernel::standard::Handle<$result> {
                self.the_result.clone()
            }
        }
        #[doc = concat!("The conversion operator of `", $native, "`; `Handle::from(&maker)`.")]
        impl From<&$maker> for martensite_tkernel::standard::Handle<$result> {
            fn from(maker: &$maker) -> Self {
                maker.value()
            }
        }
    };
}

/// `from_gce`: the status of a gce construction, and the Geom object built from its
/// value when it is done (the `TheError = X.Status(); if (TheError == Done)`
/// pattern of the GC constructors).
macro_rules! gc_from_gce {
    ($maker:ident, $gce:ty, $geom:path) => {
        impl $maker {
            fn from_gce(maker: $gce) -> Result<Self, martensite_tkernel::standard::Failure> {
                use $crate::gce::Root;
                if maker.status() != $crate::gce::ErrorType::Done {
                    return Ok(Self::failed(maker.status()));
                }
                Ok(Self::built($geom(maker.value()?)))
            }
        }
    };
}

/// An arc algorithm (`MakeArcOf*`): the struct, its Root members and the three
/// constructors that every arc class shares, which trim a Geom or Geom2d copy of the
/// gp conic between two points, a point and a parameter, or two parameters.
macro_rules! conic_arc {
    ($maker:ident, $native:literal, $conic:ty, $point:ty, $geom:path, $curve:ident, $trimmed:ident) => {
        #[doc = concat!("`", $native, "`: an arc of a conic, as a trimmed curve.")]
        #[doc = ""]
        #[doc = concat!("`", stringify!($maker), "::from_parameters(conic, 0.0, 1.0, true)?.value()?`.")]
        #[derive(Clone, Default)]
        pub struct $maker {
            the_result: martensite_tkernel::standard::Handle<$trimmed>,
            the_error: $crate::gce::ErrorType,
        }
        gc_result!($maker, $trimmed, $native);

        impl $maker {
            #[doc = concat!("`", $native, "(Conic, P1, P2, Sense)`: the arc from the parameter of `p1`")]
            #[doc = "to that of `p2` (`ElCLib::Parameter`); always done."]
            #[doc = concat!("`", stringify!($maker), "::from_points(conic, p1, p2, true)?`.")]
            pub fn from_points(
                conic: $conic,
                p1: $point,
                p2: $point,
                sense: bool,
            ) -> Result<Self, martensite_tkernel::standard::Failure> {
                let alpha1: f64 = martensite_tkmath::elclib::parameter(&conic, p1)?;
                let alpha2: f64 = martensite_tkmath::elclib::parameter(&conic, p2)?;
                Self::from_parameters(conic, alpha1, alpha2, sense)
            }

            #[doc = concat!("`", $native, "(Conic, P, Alpha, Sense)`: the arc from the parameter of")]
            #[doc = "`point` to `alpha`; always done."]
            #[doc = concat!("`", stringify!($maker), "::from_point_parameter(conic, p, 1.0, true)?`.")]
            pub fn from_point_parameter(
                conic: $conic,
                point: $point,
                alpha: f64,
                sense: bool,
            ) -> Result<Self, martensite_tkernel::standard::Failure> {
                let alpha_first: f64 = martensite_tkmath::elclib::parameter(&conic, point)?;
                Self::from_parameters(conic, alpha_first, alpha, sense)
            }

            #[doc = concat!("`", $native, "(Conic, Alpha1, Alpha2, Sense)`: the arc between two")]
            #[doc = "parameters; the trimmed curve raises Standard_ConstructionError for equal ones."]
            #[doc = concat!("`", stringify!($maker), "::from_parameters(conic, 0.0, 1.0, true)?`.")]
            pub fn from_parameters(
                conic: $conic,
                alpha1: f64,
                alpha2: f64,
                sense: bool,
            ) -> Result<Self, martensite_tkernel::standard::Failure> {
                let basis: martensite_tkernel::standard::Handle<dyn $curve> =
                    martensite_tkernel::standard::Handle::new($geom(conic)).cast();
                Ok(Self::built($trimmed::new(&basis, alpha1, alpha2, sense, true)?))
            }
        }
    };
}

mod arcs;
mod arcs2d;
mod circles;
mod conics;
mod conics2d;
mod lines;
mod lines2d;
mod surfaces;
mod transformations;
mod trimmed_surfaces;

pub use crate::gce::{ErrorType, Root};
pub use arcs::{MakeArcOfCircle, MakeArcOfEllipse, MakeArcOfHyperbola, MakeArcOfParabola};
pub use arcs2d::{
    MakeArcOfCircle2d, MakeArcOfEllipse2d, MakeArcOfHyperbola2d, MakeArcOfParabola2d,
};
pub use circles::{MakeCircle, MakeCircle2d};
pub use conics::{MakeEllipse, MakeHyperbola};
pub use conics2d::{MakeEllipse2d, MakeHyperbola2d, MakeParabola2d};
pub use lines::{MakeLine, MakeSegment};
pub use lines2d::{MakeLine2d, MakeSegment2d};
pub use surfaces::{MakeConicalSurface, MakeCylindricalSurface, MakePlane};
pub use transformations::{
    MakeMirror, MakeMirror2d, MakeRotation, MakeRotation2d, MakeScale, MakeScale2d,
    MakeTranslation, MakeTranslation2d,
};
pub use trimmed_surfaces::{MakeTrimmedCone, MakeTrimmedCylinder};
