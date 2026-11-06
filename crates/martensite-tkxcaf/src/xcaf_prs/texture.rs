//! `XCAFPrsTexture`: a 2D texture whose image comes from the `ImageTexture` of a
//! visualization material, bound to a texture unit.
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::graphic3d::{
    Graphic3dTexture2D, Graphic3dTexture2DBase, Graphic3dTexture2DSource, Graphic3dTextureMap,
    Graphic3dTextureRoot, Graphic3dTextureRootBase, Graphic3dTextureUnit,
    graphic3d_convert_to_compatible,
};
use martensite_tkservice::image::{
    ImageCompressedPixMap, ImagePixMap, ImageSupportedFormats, ImageTextureMembers,
    ImageTextureSource,
};

/// `XCAFPrsTexture`.
///
/// ```
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkservice::graphic3d::{Graphic3dTextureRoot, Graphic3dTextureUnit};
/// use martensite_tkxcaf::xcaf_prs::XCAFPrsTexture;
/// let texture = XCAFPrsTexture::new(Handle::default(), Graphic3dTextureUnit::BASE_COLOR);
/// assert!(texture.texture_root_base().fields.read().is_color_map);
/// ```
pub struct XCAFPrsTexture {
    texture: Graphic3dTexture2D,
    image_source: Handle<dyn ImageTextureSource>,
}
martensite_tkernel::impl_transient!(XCAFPrsTexture, "XCAFPrsTexture", Graphic3dTexture2D,
    texture, interfaces: [dyn Graphic3dTextureRoot, dyn Graphic3dTextureMap, dyn Graphic3dTexture2DSource]);

impl Graphic3dTextureRoot for XCAFPrsTexture {
    fn texture_root_base(&self) -> &Graphic3dTextureRootBase {
        self.texture.texture_root_base()
    }
    /// `GetCompressedImage`: the compressed image of the source, null without one.
    fn get_compressed_image(
        &self,
        supported: &Handle<ImageSupportedFormats>,
    ) -> Handle<ImageCompressedPixMap> {
        self.image_source
            .get()
            .map_or_else(Handle::default, |source| {
                source.read_compressed_image(supported)
            })
    }
    /// `GetImage`: the image of the source converted to a supported format, null without one.
    fn get_image(&self, supported: &Handle<ImageSupportedFormats>) -> Handle<ImagePixMap> {
        let Some(source) = self.image_source.get() else {
            return Handle::default();
        };
        let image = source.read_image(supported);
        graphic3d_convert_to_compatible(supported, &image);
        image
    }
}
impl Graphic3dTextureMap for XCAFPrsTexture {}
impl Graphic3dTexture2DSource for XCAFPrsTexture {
    fn texture_2d_base(&self) -> &Graphic3dTexture2DBase {
        self.texture.texture_2d_base()
    }
}

impl XCAFPrsTexture {
    /// `XCAFPrsTexture(theImageSource, theUnit)`: an empty file texture (with the public
    /// defaults) identified as its source, on the unit, a color map for base color and
    /// emission.
    pub fn new(source: Handle<dyn ImageTextureSource>, unit: Graphic3dTextureUnit) -> Self {
        let texture = Graphic3dTexture2D::from_file(&AsciiString::default());
        let base = texture.texture_2d_base();
        if let Some(id) = source
            .get()
            .map(|s| s.texture_id())
            .filter(|id| !id.is_empty())
        {
            base.root.fields.write().tex_id = id;
        }
        live_params(base).set_texture_unit(unit);
        base.root.fields.write().is_color_map =
            unit == Graphic3dTextureUnit::BASE_COLOR || unit == Graphic3dTextureUnit::EMISSIVE;
        Self {
            texture,
            image_source: source,
        }
    }
    /// `GetImageSource`.
    pub fn get_image_source(&self) -> &Handle<dyn ImageTextureSource> {
        &self.image_source
    }
}

/// The texture parameters, always set by the base constructors.
fn live_params(
    base: &Graphic3dTexture2DBase,
) -> &martensite_tkservice::graphic3d::Graphic3dTextureParams {
    base.root
        .params
        .get()
        .expect("the texture constructors create the parameters")
}
