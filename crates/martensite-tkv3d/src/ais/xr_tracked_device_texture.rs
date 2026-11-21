//! Native private XR texture holder delegates image reads while preserving resource identity.
use super::AisXrTrackedDeviceSource;
use crate::inherited_native_type::inherited_native_type;
use crate::{prs_mgr::PrsMgrPresentableObjectMembers, prs3d::Prs3dShadingAspect};
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tkservice::{aspect::AspectTypeOfFacingModel, graphic3d::*, image::*};
pub(super) fn initialize_xr_shading<O: AisXrTrackedDeviceSource + ?Sized>(
    object: &O,
    texture: &Handle<dyn ImageTextureSource>,
) -> Result<(), Failure> {
    let shading = Handle::new(Prs3dShadingAspect::new()?);
    object.attributes().set_shading_aspect(shading.clone());
    configure_xr_shading_material(&shading)?;
    if !texture.is_null() {
        shading
            .aspect()
            .set_texture_map(Handle::new(XrDeviceTexture::new(texture)).cast());
        shading.aspect().set_texture_map_on(true);
    }
    Ok(())
}
fn configure_xr_shading_material(shading: &Handle<Prs3dShadingAspect>) -> Result<(), Failure> {
    shading.set_material(
        &Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Default),
        AspectTypeOfFacingModel::BothSide,
    );
    shading.set_color(
        Color::named(NamedColor::WHITE),
        AspectTypeOfFacingModel::BothSide,
    )?;
    Ok(())
}
struct XrDeviceTexture {
    core: Graphic3dTexture2D,
    image: Handle<dyn ImageTextureSource>,
}
impl XrDeviceTexture {
    fn new(image: &Handle<dyn ImageTextureSource>) -> Self {
        let core = Graphic3dTexture2D::from_file(&AsciiString::default());
        let identity = image.texture_id();
        if !identity.is_empty() {
            core.texture_2d_base().root.fields.write().tex_id = identity;
        }
        core.texture_2d_base()
            .root
            .params
            .set_texture_unit(Graphic3dTextureUnit::BASE_COLOR);
        Self {
            core,
            image: image.clone(),
        }
    }
}
// Native private XRTexture inherits Texture2D's descriptor; Rust caster metadata is separate.
inherited_native_type!(
    XrDeviceTexture,
    Graphic3dTexture2D,
    "Martensite_XrDeviceTexture",
    named,
    [dyn Graphic3dTextureRoot, dyn Graphic3dTextureMap, dyn Graphic3dTexture2DSource]
);
impl Graphic3dTexture2DSource for XrDeviceTexture {
    fn texture_2d_base(&self) -> &Graphic3dTexture2DBase {
        // Preserve public texture parent state and immutable source identity.
        self.core.texture_2d_base()
    }
}
impl Graphic3dTextureMap for XrDeviceTexture {}
impl Graphic3dTextureRoot for XrDeviceTexture {
    fn texture_root_base(&self) -> &Graphic3dTextureRootBase {
        // Texture reads use the retained image source while resource parameters remain native.
        self.core.texture_root_base()
    }
    fn get_image(
        &self,
        supported: &Handle<ImageSupportedFormats>,
    ) -> Handle<dyn ImagePixMapObject> {
        // The XR texture bypasses the root file/image path and forwards the exact handle.
        self.image.read_image(supported)
    }
}
