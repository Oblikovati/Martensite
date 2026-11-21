//! #592: borrow a composed evaluator without projecting an owner into a concrete parent.
use super::GeomAdaptorTransformedSurface;
use crate::adaptor3d::Adaptor3dSurface;
use martensite_tkernel::impl_transient_interface_target;
use martensite_tkernel::standard::{HandleUpcast, RuntimeType, RuntimeTyped};
use std::sync::Arc;

/// An adaptor retaining an actual transformed surface evaluator.
/// The native descriptor stays `GeomAdaptor_TransformedSurface`.
///
/// ```
/// use martensite_tkg3d::geom_adaptor::{GeomAdaptorTransformedSurface, GeomTransformedSurfaceSource};
/// let adaptor = GeomAdaptorTransformedSurface::default();
/// let source: &dyn GeomTransformedSurfaceSource = &adaptor;
/// assert!(std::ptr::eq(source.transformed_surface(), &adaptor));
/// ```
pub trait GeomTransformedSurfaceSource: Adaptor3dSurface {
    /// Borrow the retained evaluator, including its original geometry and placement.
    /// Example: `source.transformed_surface().trsf()`.
    fn transformed_surface(&self) -> &GeomAdaptorTransformedSurface;
}

impl RuntimeTyped for dyn GeomTransformedSurfaceSource {
    const TYPE_NAME: &'static str = GeomAdaptorTransformedSurface::TYPE_NAME;

    fn static_type() -> &'static RuntimeType {
        GeomAdaptorTransformedSurface::static_type()
    }
}
impl_transient_interface_target!(dyn GeomTransformedSurfaceSource);

impl HandleUpcast<dyn Adaptor3dSurface> for dyn GeomTransformedSurfaceSource {
    fn upcast_arc(owner: Arc<Self>) -> Arc<dyn Adaptor3dSurface> {
        owner
    }
}

impl GeomTransformedSurfaceSource for GeomAdaptorTransformedSurface {
    fn transformed_surface(&self) -> &GeomAdaptorTransformedSurface {
        self
    }
}
