//! Bounded face-adaptor seam preserves actual application virtual calls and the native nonvirtual Face core.
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkernel::standard::{HandleUpcast, RuntimeTyped};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use std::sync::Arc;
/// Native BRepAdaptorSurface subclasses embed their real core and override Adaptor3d calls.
/// Example: `impl StdPrsIsolineSurfaceSource for Wrapper { fn isoline_surface_base(&self) -> &BRepAdaptorSurface { &self.core } }`.
pub trait StdPrsIsolineSurfaceSource: Adaptor3dSurface {
    /// Project actual owned BRep adaptor state, not a copied face; `surface.isoline_surface_base().face()`.
    fn isoline_surface_base(&self) -> &BRepAdaptorSurface;
}
impl StdPrsIsolineSurfaceSource for BRepAdaptorSurface {
    fn isoline_surface_base(&self) -> &BRepAdaptorSurface {
        // Native Face is nonvirtual even when algorithms dispatch other adaptor methods virtually.
        self
    }
}

impl martensite_tkernel::standard::RuntimeTyped for dyn StdPrsIsolineSurfaceSource {
    const TYPE_NAME: &'static str = "BRepAdaptor_Surface";
    fn static_type() -> &'static martensite_tkernel::standard::RuntimeType {
        // The dispatch seam shares the actual BRep descriptor, without inventing another native ancestor.
        <BRepAdaptorSurface as martensite_tkernel::standard::RuntimeTyped>::static_type()
    }
}
martensite_tkernel::impl_transient_interface_target!(dyn StdPrsIsolineSurfaceSource);
impl HandleUpcast<dyn Adaptor3dSurface> for dyn StdPrsIsolineSurfaceSource {
    fn upcast_arc(owner: Arc<Self>) -> Arc<dyn Adaptor3dSurface> {
        // Trait upcasting: the seam always is an adaptor, as its supertrait states.
        owner
    }
}

/// A face-adaptor handle target whose owner coerces to `Adaptor3dSurface` at compile time.
///
/// Native passes `Handle(BRepAdaptor_Surface)` where an `Adaptor3d_Surface` handle is
/// expected. That implicit upcast cannot fail, so the isoline algorithms use the Arc coercion
/// rather than a registry cross-cast that could select a null-surface branch (#592).
/// Example: `fn draw<S: StdPrsIsolineSurfaceTarget + ?Sized>(face: &Handle<S>)`.
pub trait StdPrsIsolineSurfaceTarget:
    StdPrsIsolineSurfaceSource + RuntimeTyped + HandleUpcast<dyn Adaptor3dSurface>
{
}
impl<S> StdPrsIsolineSurfaceTarget for S where
    S: StdPrsIsolineSurfaceSource + RuntimeTyped + HandleUpcast<dyn Adaptor3dSurface> + ?Sized
{
}
