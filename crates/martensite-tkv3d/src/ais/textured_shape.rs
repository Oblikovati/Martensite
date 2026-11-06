//! AisTexturedShape native sources and mapping state.
use super::shape::AisShapeBase;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    message::{Message, Messenger},
    standard::{Handle, IntrusiveReferenceSlot, SharedState},
    tcollection::AsciiString,
};
use martensite_tkservice::{graphic3d::*, image::ImagePixMap};
use std::sync::Arc;

/// Native textured presentation/source fields; `object.textured_shape_base().fields.read().texture`.
#[derive(Clone)]
pub struct AisTexturedShapeFields {
    /// `myTexture`: preserved when mapping is disabled or construction fails.
    pub texture: Handle<dyn Graphic3dTexture2DSource>,
    /// `myAspect`: replaced before texture construction in updateAttributes.
    pub aspect: Handle<Graphic3dAspectFillArea3d>,
    /// `myTextureFile`.
    pub texture_file: AsciiString,
    /// `myPredefTexture`.
    pub predefined_texture: Graphic3dNameOfTexture2D,
    /// `myToMapTexture`.
    pub map_texture: bool,
    /// `myModulate`.
    pub modulate: bool,
    /// `myIsCustomOrigin`.
    pub custom_origin: bool,
    /// `myToRepeat`: deliberately does not control sampler repeat in pinned native code.
    pub repeat: bool,
    /// `myToScale`.
    pub scale: bool,
    /// `myToShowTriangles`.
    pub show_triangles: bool,
}
impl Default for AisTexturedShapeFields {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            aspect: Handle::default(),
            texture_file: AsciiString::default(),
            predefined_texture: Graphic3dNameOfTexture2D::Matra,
            map_texture: true,
            modulate: true,
            custom_origin: true,
            repeat: true,
            scale: true,
            show_triangles: false,
        }
    }
}
/// Embedded native textured shape fields; `AisTexturedShapeBase::new(&shape)` keeps shared pixmap storage.
pub struct AisTexturedShapeBase {
    /// Genuine inherited AisShape state.
    pub shape: AisShapeBase,
    /// Native texture source and mapping fields.
    pub fields: SharedState<AisTexturedShapeFields>,
    /// `myTexturePixMap`: separate storage preserves const handle-reference aliases across assignments.
    pub texture_pix_map: IntrusiveReferenceSlot<ImagePixMap>,
    /// Native diagnostics are injected so tests can retain exact message ordering.
    pub messenger: Arc<Messenger>,
    /// Native texture construction and filesystem queries are an owned injectable boundary.
    pub texture_factory: Arc<dyn super::textured_shape_factory::AisTexturedTextureFactory>,
    /// Pixmap destructor borrows lent to the base setters (no native field: native code
    /// reaches the retiring target through the handle field itself).
    pub retirement: super::textured_shape_retirement::AisTexturedRetirementStash,
}
impl Drop for AisTexturedShapeBase {
    fn drop(&mut self) {
        // Destroy derived fields before AisShape, reversing their declarations.
        self.fields.write().texture_file.clear();
        self.texture_pix_map.clear().unwrap();
        let aspect = std::mem::take(&mut self.fields.write().aspect);
        drop(aspect);
        let texture = std::mem::take(&mut self.fields.write().texture);
        drop(texture);
    }
}
impl AisTexturedShapeBase {
    /// Initialize actual native source/parent defaults; `AisTexturedShapeBase::new(&shape)`.
    pub fn new(shape: &Shape) -> Self {
        // Default construction uses the native diagnostic destination through the owned seam.
        Self::with_messenger(shape, Message::default_messenger().clone())
    }
    /// Inject the native diagnostic destination; `AisTexturedShapeBase::with_messenger(&shape,messenger)`.
    pub fn with_messenger(shape: &Shape, messenger: Arc<Messenger>) -> Self {
        Self::with_services(
            shape,
            messenger,
            Arc::new(super::textured_shape_factory::AisNativeTexturedTextureFactory),
        )
    }
    /// Inject native diagnostics and texture construction; `AisTexturedShapeBase::with_services(&shape,messenger,factory)`.
    pub fn with_services(
        shape: &Shape,
        messenger: Arc<Messenger>,
        texture_factory: Arc<dyn super::textured_shape_factory::AisTexturedTextureFactory>,
    ) -> Self {
        Self {
            shape: AisShapeBase::new(shape),
            fields: SharedState::default(),
            texture_pix_map: IntrusiveReferenceSlot::default(),
            messenger,
            texture_factory,
            retirement: Default::default(),
        }
    }
}
