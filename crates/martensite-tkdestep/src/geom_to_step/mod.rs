//! GeomToStep: gp and Geom objects to the STEP geometric entities. Each class is a
//! constructor per overload (`from_*`), `is_done` and `Value`. Lengths are divided by the length
//! factor of the transfer; 2D geometry is written unscaled.
mod bspline_curves;
mod bspline_surfaces;
mod curves;
mod placements;
mod surfaces;

pub use bspline_curves::{
    GeomToStepMakeBSplineCurveWithKnots,
    GeomToStepMakeBSplineCurveWithKnotsAndRationalBSplineCurve, GeomToStepMakeBoundedCurve,
};
pub use bspline_surfaces::{
    GeomToStepMakeBSplineSurfaceWithKnots,
    GeomToStepMakeBSplineSurfaceWithKnotsAndRationalBSplineSurface,
};
pub use curves::{
    GeomToStepMakeCircle, GeomToStepMakeConic, GeomToStepMakeCurve, GeomToStepMakeEllipse,
    GeomToStepMakeHyperbola, GeomToStepMakeLine, GeomToStepMakeParabola,
};
pub use placements::{
    GeomToStepMakeAxis1Placement, GeomToStepMakeAxis2Placement2d, GeomToStepMakeAxis2Placement3d,
    GeomToStepMakeCartesianPoint, GeomToStepMakeCartesianTransformationOperator,
    GeomToStepMakeDirection, GeomToStepMakeVector,
};
pub use surfaces::{
    GeomToStepMakeBoundedSurface, GeomToStepMakeConicalSurface, GeomToStepMakeCylindricalSurface,
    GeomToStepMakeElementarySurface, GeomToStepMakePlane, GeomToStepMakeRectangularTrimmedSurface,
    GeomToStepMakeSphericalSurface, GeomToStepMakeSurface, GeomToStepMakeSurfaceOfLinearExtrusion,
    GeomToStepMakeSurfaceOfRevolution, GeomToStepMakeSweptSurface, GeomToStepMakeToroidalSurface,
};

use martensite_tkernel::ncollection::{Array1, HArray1};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::HAsciiString;

/// A 1-based array of the given values.
pub(crate) fn array1_of<T: Clone + Default>(values: Vec<T>) -> Array1<'static, T> {
    // `with_size` starts at 0; STEP and Geom arrays are 1-based.
    let Ok(mut array) = Array1::with_bounds(1, values.len() as i32) else {
        return Array1::default();
    };
    for (slot, value) in array.as_mut_slice().iter_mut().zip(values) {
        *slot = value;
    }
    array
}

/// A shared 1-based array of the given values.
pub(crate) fn shared_array1<T: Clone + Default + Send + Sync + 'static>(
    values: Vec<T>,
) -> Handle<HArray1<T>> {
    Handle::new(HArray1::from_owned(array1_of(values)))
}

/// `new HAsciiString("")`, the name of every entity these classes create.
pub(crate) fn empty_step_name() -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(""))
}

/// Declares a GeomToStep class: its result, `is_done` (`the conversions::is_done`) and `Value`,
/// which raises a not done failure while there is no result.
macro_rules! geom_to_step_class {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, $entity:ty) => {
        $(#[$meta])*
        #[derive(Clone)]
        pub struct $class {
            done: bool,
            result: martensite_tkernel::standard::Handle<$entity>,
        }
        impl $class {
            /// The result of a successful conversion.
            fn done_with(result: martensite_tkernel::standard::Handle<$entity>) -> Self {
                Self { done: true, result }
            }
            /// The state of a conversion that has no result.
            #[allow(dead_code, reason = "not every class has a failing conversion")]
            fn not_done() -> Self {
                Self { done: false, result: martensite_tkernel::standard::Handle::default() }
            }
            /// `is_done`: true when the conversion gave a result.
            pub fn is_done(&self) -> bool {
                self.done
            }
            /// `Value`: the entity.
            ///
            /// # Errors
            /// a not done failure when the conversion gave no result.
            pub fn value(
                &self,
            ) -> Result<martensite_tkernel::standard::Handle<$entity>, martensite_tkernel::standard::Failure>
            {
                if self.done {
                    return Ok(self.result.clone());
                }
                Err(martensite_tkernel::std_fail::not_done(concat!(
                    $recorded,
                    "::Value() - no result"
                )))
            }
        }
    };
}
pub(crate) use geom_to_step_class;
