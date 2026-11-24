//! Native field aliases never retain a lock or acquire pixmap ownership.
use martensite_tkernel::standard::{
    Failure, Handle, HandleObserver, HandleReference, IntrusiveReferenceSlot, Transient,
};
use martensite_tkservice::image::{ImagePixMap, ImagePixMapObject};
/// Borrowed actual pointer slot; `object.texture_pix_map().object_address()`.
#[derive(Clone, Copy)]
pub struct AisTexturedShapePixMapRef<'a> {
    pixels: &'a IntrusiveReferenceSlot<dyn ImagePixMapObject>,
}
impl<'a> AisTexturedShapePixMapRef<'a> {
    pub(super) fn new(pixels: &'a IntrusiveReferenceSlot<dyn ImagePixMapObject>) -> Self {
        // Keep the original slot reference rather than a pointer-value snapshot.
        Self { pixels }
    }
    /// Compatibility inspection holds no guard; `object.texture_pix_map().read().is_null()`.
    pub fn read(&self) -> Self {
        // The returned alias continues observing the same actual field after callbacks.
        *self
    }
    /// Current pointer nullness; `object.texture_pix_map().is_null()`.
    pub fn is_null(&self) -> bool {
        let pixels = self.pixels;
        pixels.is_null()
    }
    /// Current pointer identity, including old-target destruction; `alias.object_address()`.
    pub fn object_address(&self) -> *const () {
        let pixels = self.pixels;
        pixels.object_address()
    }
    /// Current strong count without retaining the target; `alias.ref_count()`.
    pub fn ref_count(&self) -> i32 {
        let pixels = self.pixels;
        i32::try_from(pixels.ref_count()).unwrap_or(i32::MAX)
    }
    /// Inspect the real destructor borrow; `alias.with_retiring_pointee(self, |p| p.width())?`.
    /// The original field identity is checked and no native owning count is acquired.
    pub fn with_retiring_pointee<R>(
        &self,
        retiring: &dyn Transient,
        inspect: impl FnOnce(&ImagePixMap) -> R,
    ) -> Result<R, Failure> {
        // Rust Drop already owns the valid reborrow; an expired Arc must not be resurrected.
        self.pixels
            .with_retiring_pointee(retiring, |pixels| inspect(pixels.pix_map_base()))
    }
    /// Acquire ownership explicitly, independently of the native const field alias; `alias.clone_handle()`.
    pub fn clone_handle(&self) -> Handle<dyn ImagePixMapObject> {
        self.pixels.observe().into_handle()
    }
}
impl HandleReference<dyn ImagePixMapObject> for AisTexturedShapePixMapRef<'_> {
    fn observe(&self) -> HandleObserver<'_, dyn ImagePixMapObject> {
        let pixels = self.pixels;
        pixels.observe()
    }
    fn reference_address(&self) -> *const () {
        let pixels = self.pixels;
        pixels.reference_address()
    }
    fn object_address(&self) -> *const () {
        let pixels = self.pixels;
        pixels.object_address()
    }
    fn is_null(&self) -> bool {
        let pixels = self.pixels;
        pixels.is_null()
    }
}
