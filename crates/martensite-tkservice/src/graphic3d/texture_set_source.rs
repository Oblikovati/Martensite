//! Texture sets share their native storage through real Rust interfaces (#592).
use super::Graphic3dTextureSet;
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped, Transient};

/// A texture-set object; `source.texture_set_base().size()`.
pub trait Graphic3dTextureSetSource: Transient {
    /// Borrow the original set of textures; `source.texture_set_base().textures()`.
    fn texture_set_base(&self) -> &Graphic3dTextureSet;
}

impl RuntimeTyped for dyn Graphic3dTextureSetSource {
    const TYPE_NAME: &'static str = <Graphic3dTextureSet as RuntimeTyped>::TYPE_NAME;
    fn static_type() -> &'static RuntimeType {
        Graphic3dTextureSet::static_type()
    }
}
martensite_tkernel::impl_transient_interface_target!(dyn Graphic3dTextureSetSource);

impl std::ops::Deref for dyn Graphic3dTextureSetSource {
    type Target = Graphic3dTextureSet;
    fn deref(&self) -> &Self::Target {
        self.texture_set_base()
    }
}

impl Graphic3dTextureSetSource for Graphic3dTextureSet {
    fn texture_set_base(&self) -> &Graphic3dTextureSet {
        self
    }
}
