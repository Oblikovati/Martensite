//! The actual descendant's destructor inspects an actual source slot, without owning either source.
use martensite_tkernel::{standard::*, tcollection::AsciiString};
use martensite_tkservice::{
    graphic3d::Graphic3dNameOfTexture2D,
    image::{ImagePixMap, ImagePixMapObject},
};
use martensite_tkv3d::ais::*;
use std::sync::{Arc, Weak};
pub struct TextureSlotCallbackPixels {
    pub(super) self_owner: Weak<Self>,
    pub base: ImagePixMap,
    pub owner: WeakHandle<AisTexturedShape>,
    pub incoming: WeakHandle<dyn ImagePixMapObject>,
    pub label: String,
    pub trace: Arc<SharedState<Vec<String>>>,
    pub callback: Option<Box<dyn Fn() + Send + Sync>>,
}
martensite_tkernel::impl_transient!(
    TextureSlotCallbackPixels,
    "NativeTextureSlotPixels",
    ImagePixMap, interfaces: [dyn ImagePixMapObject]
);
impl ImagePixMapObject for TextureSlotCallbackPixels {
    fn pix_map_base(&self) -> &ImagePixMap {
        &self.base
    }
}
impl Drop for TextureSlotCallbackPixels {
    fn drop(&mut self) {
        if let Some(callback) = &self.callback {
            callback();
        }
        let owner = self.owner.upgrade();
        let slot = owner.texture_pix_map();
        let same = slot.object_address() == std::ptr::from_ref(self).cast();
        let count = self.self_owner.strong_count();
        let incoming = incoming_texture_count(&self.incoming);
        let prefix = format!(
            "MARTENSITE_TEXTURE_SLOT {} {} {} {count} {incoming}",
            self.label,
            i32::from(same),
            i32::from(slot.is_null())
        );
        self.trace
            .write()
            .push(texture_state_suffix(&owner, prefix));
    }
}
fn incoming_texture_count(incoming: &WeakHandle<dyn ImagePixMapObject>) -> i32 {
    if incoming.is_null() {
        return -1;
    }
    incoming.ref_count().try_into().unwrap()
}
pub fn texture_state_suffix(owner: &AisTexturedShape, prefix: String) -> String {
    let predefined = owner.textured_shape_base().fields.read().predefined_texture as i32;
    let file = owner.texture_file();
    format!("{prefix} {predefined} {}", file.to_string_lossy())
        .trim_end()
        .to_string()
}
pub fn seed_callback_texture_file(owner: &AisTexturedShape) {
    let mut fields = owner.textured_shape_base().fields.write();
    fields.texture_file = AsciiString::from("prior.raw");
    fields.predefined_texture = Graphic3dNameOfTexture2D::Matra;
}
pub fn callback_texture_pixels(
    owner: &Handle<AisTexturedShape>,
    incoming: &Handle<dyn ImagePixMapObject>,
    label: &str,
    trace: &Arc<SharedState<Vec<String>>>,
) -> Handle<dyn ImagePixMapObject> {
    Handle::from_arc(Arc::new_cyclic(|self_owner| TextureSlotCallbackPixels {
        self_owner: self_owner.clone(),
        base: ImagePixMap::default(),
        owner: owner.downgrade(),
        incoming: incoming.downgrade(),
        label: label.to_string(),
        trace: trace.clone(),
        callback: None,
    }))
    .into_upcast()
}
