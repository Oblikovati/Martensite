//! `GeomBoundedSurface`: surfaces limited by finite parameters.
use super::surface::GeomSurface;
use martensite_tkernel::impl_transient_interface;

/// A surface with finite bounds; the retained class adds no member;
/// `surface.cast::<dyn GeomBoundedSurface>()`.
pub trait GeomBoundedSurface: GeomSurface {}
impl_transient_interface!(
    dyn GeomBoundedSurface,
    "Geom_BoundedSurface",
    dyn GeomSurface
);
