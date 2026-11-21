//! Actual Rust image interfaces preserve the owning allocation across subclass views (#592).
use super::{ImageAlienPixMap, ImagePixMap};
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped, Transient};

/// An image object lending its original pixel storage; `image.pix_map_base().width()`.
pub trait ImagePixMapObject: Transient {
    /// Borrow this object's pixel image; `image.pix_map_base().height()`.
    fn pix_map_base(&self) -> &ImagePixMap;
}

impl RuntimeTyped for dyn ImagePixMapObject {
    const TYPE_NAME: &'static str = <ImagePixMap as RuntimeTyped>::TYPE_NAME;
    fn static_type() -> &'static RuntimeType {
        ImagePixMap::static_type()
    }
}
martensite_tkernel::impl_transient_interface_target!(dyn ImagePixMapObject);

impl std::ops::Deref for dyn ImagePixMapObject {
    type Target = ImagePixMap;
    fn deref(&self) -> &Self::Target {
        self.pix_map_base()
    }
}

impl ImagePixMapObject for ImagePixMap {
    fn pix_map_base(&self) -> &ImagePixMap {
        self
    }
}

impl ImagePixMapObject for ImageAlienPixMap {
    fn pix_map_base(&self) -> &ImagePixMap {
        self
    }
}
