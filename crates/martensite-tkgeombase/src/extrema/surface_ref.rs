// The `const Adaptor3dSurface*` members of the Extrema surface classes at

//! The surface an Extrema object keeps. Natively a raw pointer to an adaptor that
//! the caller owns, or that a handle held by an enclosing object keeps alive
//! (ExtPExtS and ExtPRevS keep the handle of the adaptor that their
//! GenExtPS points to); here a borrow or a shared handle.
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;

/// A borrowed or a shared surface adaptor; `SurfaceRef::Borrowed(&surface).get()`.
#[derive(Clone)]
pub(crate) enum SurfaceRef<'a> {
    /// An adaptor the caller keeps alive.
    Borrowed(&'a dyn Adaptor3dSurface),
    /// An adaptor kept alive by the handle.
    Shared(Handle<dyn Adaptor3dSurface>),
}

impl SurfaceRef<'_> {
    /// The adaptor; `surface.get().value(0.0, 0.0)?`.
    pub fn get(&self) -> &dyn Adaptor3dSurface {
        match self {
            SurfaceRef::Borrowed(surface) => *surface,
            SurfaceRef::Shared(handle) => &**handle,
        }
    }
}
