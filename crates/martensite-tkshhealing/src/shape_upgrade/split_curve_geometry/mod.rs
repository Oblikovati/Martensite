//! Shared 2D/3D splitter control flow; family policies retain their distinct rules.
use super::SplitCurveData;
use crate::shape_extend::{ShapeExtend, ShapeExtendStatus};
use martensite_tkernel::{
    ncollection::{HArray1, HSequence},
    precision,
    standard::{Failure, Handle, SharedState, Transient},
};
mod build;
mod families;
mod pieces;
pub(super) use families::{CurveFamily2d, CurveFamily3d};
pub(super) trait CurveGeometry {
    type Curve: Transient + ?Sized;
    const NAME: &'static str;
    const CLAMP_BUILD: bool;
    const SEGMENT_SINGLE: bool;
    fn copy(curve: &Handle<Self::Curve>) -> Result<Handle<Self::Curve>, Failure>;
    fn bounds(curve: &Handle<Self::Curve>) -> (f64, f64);
    fn periodic(curve: &Handle<Self::Curve>) -> Result<bool, Failure>;
    fn trimmed_basis(curve: &Handle<Self::Curve>) -> Option<Handle<Self::Curve>>;
    fn offset_basis(curve: &Handle<Self::Curve>) -> Option<Handle<Self::Curve>>;
    fn offset(
        curve: &Handle<Self::Curve>,
        basis: &Handle<Self::Curve>,
    ) -> Result<Handle<Self::Curve>, Failure>;
    fn trim(
        curve: &Handle<Self::Curve>,
        first: f64,
        last: f64,
    ) -> Result<Handle<Self::Curve>, Failure>;
    fn spline(curve: &Handle<Self::Curve>) -> bool;
    fn segment(curve: &Handle<Self::Curve>, first: f64, last: f64) -> Result<(), Failure>;
    fn knots(curve: &Handle<Self::Curve>) -> Result<Option<Vec<f64>>, Failure>;
}
pub(super) type CurveArray<G> = Handle<HArray1<Handle<<G as CurveGeometry>::Curve>>>;
struct GeometryState<G: CurveGeometry> {
    curve: Handle<G::Curve>,
    result: CurveArray<G>,
}
impl<G: CurveGeometry> Default for GeometryState<G> {
    fn default() -> Self {
        Self {
            curve: Handle::default(),
            result: Handle::default(),
        }
    }
}
pub(super) struct CurveSplitKernel<G: CurveGeometry> {
    split: SplitCurveData,
    state: SharedState<GeometryState<G>>,
}
impl<G: CurveGeometry> Default for CurveSplitKernel<G> {
    fn default() -> Self {
        Self {
            split: SplitCurveData::default(),
            state: SharedState::default(),
        }
    }
}
impl<G: CurveGeometry + 'static> CurveSplitKernel<G> {
    pub(super) fn split_curve_data(&self) -> &SplitCurveData {
        &self.split
    }

    fn split_values(&self) -> Handle<HSequence<f64>> {
        self.split.split_values()
    }

    fn set_split_values(&self, values: &Handle<HSequence<f64>>) -> Result<(), Failure> {
        self.split.set_split_values(values)
    }

    fn status_flag(&self, status: ShapeExtendStatus) -> bool {
        self.split.status_flag(status)
    }

    pub(super) fn init_natural(&self, curve: &Handle<G::Curve>) -> Result<(), Failure> {
        require_curve::<G>(curve)?;
        let (first, last) = G::bounds(curve);
        self.init_curve_range(curve, first, last)
    }
    pub(super) fn init_curve_range(
        &self,
        curve: &Handle<G::Curve>,
        first: f64,
        last: f64,
    ) -> Result<(), Failure> {
        require_curve::<G>(curve)?;
        let copy = G::copy(curve)?;
        self.state.write().curve = copy.clone();
        let basis = G::trimmed_basis(&copy).unwrap_or(copy);
        let (first, last) = if G::periodic(curve)? {
            (first, last)
        } else {
            init_bounds(G::bounds(&basis), first, last)
        };
        self.split.init(first, last);
        Ok(())
    }
    pub(super) fn source_curve(&self) -> Handle<G::Curve> {
        self.state.read().curve.clone()
    }
    pub(super) fn curves(&self) -> CurveArray<G> {
        self.state.read().result.clone()
    }
    pub(super) fn set_result_curves(&self, curves: CurveArray<G>) {
        self.state.write().result = curves;
    }
    fn allocate_curves(&self, count: i32) -> Result<(), Failure> {
        self.split_curve_data()
            .with_mut(|fields| fields.nb_curves = count);
        let array = HArray1::with_value(1, count, Handle::default())
            .map_err(|e| e.into_failure(G::NAME))?;
        self.state.write().result = Handle::new(array);
        Ok(())
    }
    fn set_curve(&self, index: i32, curve: Handle<G::Curve>) -> Result<(), Failure> {
        self.curves()
            .change_array1()
            .set_value(index, curve)
            .map_err(|e| e.into_failure(G::NAME))
    }
    fn add_status(&self, status: ShapeExtendStatus) {
        self.split_curve_data()
            .with_mut(|fields| fields.status |= ShapeExtend::encode_status(status));
    }
}
fn require_curve<G: CurveGeometry>(curve: &Handle<G::Curve>) -> Result<(), Failure> {
    if curve.is_null() {
        return Err(Failure::with_type(
            "Standard_NullObject",
            &format!("{}: null curve; expected initialized curve", G::NAME),
            "",
        ));
    }
    Ok(())
}
fn init_bounds((lower, upper): (f64, f64), mut first: f64, mut last: f64) -> (f64, f64) {
    (first, last) = snap_bounds(first, last, lower, upper);
    if first < lower {
        first = lower;
    }
    if last > upper {
        last = upper;
    }
    // OCC966 expands nonperiodic ranges after clamping, including reversed input.
    if last - first < precision::p_confusion() {
        last = first + 2. * precision::p_confusion();
    }
    (first, last)
}
fn snap_bounds(mut first: f64, mut last: f64, lower: f64, upper: f64) -> (f64, f64) {
    if (first - lower).abs() < precision::p_confusion() {
        first = lower;
    }
    if (last - upper).abs() < precision::p_confusion() {
        last = upper;
    }
    (first, last)
}

macro_rules! define_curve_splitter {
    ($leaf:ident, $interface:ident, $data:ident, $curves:ident, $curve:ty, $family:ty, $native:literal) => {
        /// Geometry and split values shared by the concrete and derived criteria.
        #[derive(Default)]
        pub struct $data {
            kernel: CurveSplitKernel<$family>,
        }
        impl $data {
            /// Copy a curve over its natural bounds. Example: `state.init_curve(&curve)?`.
            pub fn init_curve(&self, curve: &Handle<$curve>) -> Result<(), Failure> {
                self.kernel.init_natural(curve)
            }
            /// Copy and clamp the range as in OCC966. Example: `state.init_curve_range(&curve, 0., 1.)?`.
            pub fn init_curve_range(&self, curve: &Handle<$curve>, first: f64, last: f64) -> Result<(), Failure> {
                self.kernel.init_curve_range(curve, first, last)
            }
            /// Access shared split parameters and status. Example: `state.split_curve_data()`.
            pub fn split_curve_data(&self) -> &SplitCurveData {
                self.kernel.split_curve_data()
            }
            /// Apply the base geometric build to the current split parameters.
            /// Example: `state.build_geometry(true)?`.
            pub fn build_geometry(&self, segment: bool) -> Result<(), Failure> {
                self.kernel.build_curves(segment)
            }
            /// Read the shared output array, which is null before Build. Example: `state.curves()`.
            pub fn curves(&self) -> $curves {
                self.kernel.curves()
            }
            pub(super) fn source_curve(&self) -> Handle<$curve> {
                self.kernel.source_curve()
            }
            pub(super) fn set_result_curves(&self, curves: $curves) {
                self.kernel.set_result_curves(curves);
            }
        }

        /// The family-specific curve operations on the original splitter.
        pub trait $interface: SplitCurve {
            /// Access curve state. Example: `splitter.curve_geometry().curves()`.
            fn curve_geometry(&self) -> &$data;
        }
        parent_interface!(dyn $interface, $leaf, [dyn SplitCurve]);
        impl std::ops::Deref for dyn $interface {
            type Target = $data;
            fn deref(&self) -> &Self::Target {
                self.curve_geometry()
            }
        }

        #[doc = concat!("The standard `", $native, "` criterion.")]
        #[derive(Default)]
        pub struct $leaf {
            geometry: $data,
        }
        martensite_tkernel::impl_transient!($leaf, $native, dyn SplitCurve, interfaces: [dyn SplitCurve, dyn $interface]);
        impl $leaf {
            #[doc = concat!("Construct without an input curve. Example: `", stringify!($leaf), "::new()`.")]
            pub fn new() -> Self {
                Self::default()
            }
        }
        impl SplitCurve for $leaf {
            fn split_curve_data(&self) -> &SplitCurveData {
                self.geometry.split_curve_data()
            }
            fn build(&self, segment: bool) -> Result<(), Failure> {
                self.geometry.build_geometry(segment)
            }
        }
        impl $interface for $leaf {
            fn curve_geometry(&self) -> &$data {
                &self.geometry
            }
        }
        impl std::ops::Deref for $leaf {
            type Target = $data;
            fn deref(&self) -> &Self::Target {
                &self.geometry
            }
        }
    };
}
pub(super) use define_curve_splitter;
