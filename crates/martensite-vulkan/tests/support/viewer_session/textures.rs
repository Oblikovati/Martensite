//! Textures on a displayed shape: one 2D texture in unit 0, mapped with
//! generated UV coordinates in shaded mode.
//!
//! The image path:
//! the object gets its own shading aspect, the texture set replaces none, UV coordinates are
//! computed once and the object switches to shaded mode.
use super::ViewerSession;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::Point2;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectFillArea3d, Graphic3dAspectsRenderMembers as _, Graphic3dNameOfTexture2D,
    Graphic3dTexture2D, Graphic3dTextureMap, Graphic3dTextureRootMembers as _, Graphic3dTextureSet,
    Graphic3dTextureUnit,
};
use martensite_tkv3d::ais::{
    AisInteractiveContext, AisInteractiveObject, AisShapeParameters as _, AisShapeSource,
};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use std::path::Path;

/// `AisShaded`.
const INTERACTIVE_SHADED: i32 = 1;

/// The texture image: an image file or a predefined texture.
pub enum TextureImage<'a> {
    /// An image file path (any format the image reader accepts, SGI included).
    File(&'a Path),
    /// A predefined texture by name (index 0 is `Graphic3dNOT_2D_MATRA`).
    Predefined(Graphic3dNameOfTexture2D),
}

impl TextureImage<'_> {
    /// The texture map of the image, bound to texture unit 0.
    fn texture(&self) -> Result<Handle<dyn Graphic3dTextureMap>, Failure> {
        let texture = match self {
            TextureImage::File(path) => {
                Graphic3dTexture2D::from_file(&AsciiString::from(path.to_string_lossy().as_ref()))
            }
            TextureImage::Predefined(name) => Graphic3dTexture2D::from_name(*name)?,
        };
        if let Some(params) = texture.get_params().get() {
            params.set_texture_unit(Graphic3dTextureUnit::Unit0);
        }
        Ok(Handle::new(texture).cast())
    }
}

fn missing(name: &str) -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        &format!("object {name:?} not displayed; expected a displayed shape name"),
    )
}

/// The shading fill aspect of the object's drawer.
fn fill_aspect(io: &dyn AisInteractiveObject) -> Handle<Graphic3dAspectFillArea3d> {
    io.attributes()
        .get()
        .and_then(|drawer| {
            drawer
                .shading_aspect()
                .get()
                .map(|shading| shading.aspect())
        })
        .unwrap_or_default()
}

/// The `toSetImage` block for an object without a texture: own shading aspect, the new set,
/// mapping on. With no earlier set, UV coordinates are always computed afterwards.
fn set_image(
    context: &AisInteractiveContext,
    object: &Handle<dyn AisInteractiveObject>,
    image: &TextureImage,
) -> Result<(), Failure> {
    let io = object.get().ok_or_else(|| missing("<null>"))?;
    let set = Graphic3dTextureSet::with_size(1)?;
    set.set_value(0, image.texture()?)?;
    let drawer = io.attributes();
    let drawer = drawer
        .get()
        .ok_or_else(|| Failure::with_message("Standard_NullObject", "Prs3dDrawer"))?;
    let shape = object.cast::<dyn AisShapeSource>();
    if drawer.setup_own_shading_aspect(&context.default_drawer())? && shape.is_null() {
        io.set_to_update(-1);
    }
    if let Some(aspect) = fill_aspect(io).get() {
        aspect.set_texture_map_on(true);
        aspect.set_texture_set(Handle::new(set));
    }
    Ok(())
}

/// The end of `VTexture` after a UV change: recompute the shaded presentation, switch to
/// shaded mode when an image was set, redisplay and synchronize the aspects.
fn recompute_shaded(
    context: &AisInteractiveContext,
    object: &Handle<dyn AisInteractiveObject>,
    image_set: bool,
) -> Result<(), Failure> {
    let io = object.get().ok_or_else(|| missing("<null>"))?;
    if let Some(shape) = object.cast::<dyn AisShapeSource>().get() {
        shape.set_to_update(INTERACTIVE_SHADED);
        let not_shaded = io.has_display_mode() && io.display_mode() != INTERACTIVE_SHADED;
        if image_set && (not_shaded || context.display_mode() != INTERACTIVE_SHADED) {
            context.set_display_mode(object, INTERACTIVE_SHADED, false)?;
        }
    }
    context.display(object, false)?;
    io.synchronize_aspects();
    Ok(())
}

impl ViewerSession {
    /// Set the image on an object without a texture.
    /// Example: `session.set_texture("box", TextureImage::File(&floor))?`.
    pub fn set_texture(&self, name: &str, image: TextureImage) -> Result<(), Failure> {
        let object = self.object(name).cloned().ok_or_else(|| missing(name))?;
        self.with_update(|context| {
            set_image(context, &object, &image)?;
            recompute_shaded(context, &object, true)
        })
    }

    /// The UV repeat of a textured shape.
    /// Example: `session.set_texture_repeat("b_2", [3., 2.])?`.
    pub fn set_texture_repeat(&self, name: &str, [u, v]: [f64; 2]) -> Result<(), Failure> {
        let object = self.object(name).cloned().ok_or_else(|| missing(name))?;
        let shape = object.cast::<dyn AisShapeSource>();
        let shape = shape.get().ok_or_else(|| missing(name))?;
        shape.set_texture_repeat_uv(Point2::new(u, v));
        self.with_update(|context| recompute_shaded(context, &object, false))
    }
}
