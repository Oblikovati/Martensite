//! The images of a view framebuffer: the counterpart `GlTexture`
//! objects that `GlFrameBuffer::ColorTexture` and `DepthStencilTexture` return for a
//! framebuffer of `Graphic3dCView::FBOCreate`. The device allocates the images for each frame
//! rendered into the framebuffer; this value answers the native texture queries.

/// Which attachment of a framebuffer a texture is; `RenderTextureAttachment::Color`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RenderTextureAttachment {
    /// The colour image (sRGB RGBA, 8 bits per channel).
    Color,
    /// The depth-stencil image.
    DepthStencil,
}

/// One image of a framebuffer; `fbo.color_texture(0).map(|texture| texture.size())`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderTexture {
    attachment: RenderTextureAttachment,
    size: [i32; 2],
}

impl RenderTexture {
    pub(crate) fn new(attachment: RenderTextureAttachment, size: [i32; 2]) -> Self {
        Self { attachment, size }
    }
    /// The attachment the image serves; `texture.attachment()`.
    pub fn attachment(&self) -> RenderTextureAttachment {
        self.attachment
    }
    /// A framebuffer image is always allocated (`IsValid`); `texture.is_valid()`.
    pub fn is_valid(&self) -> bool {
        true
    }
    /// Width, height and depth (`Size`): 2D images have depth 1; `texture.size() == [64, 48, 1]`.
    pub fn size(&self) -> [i32; 3] {
        [self.size[0], self.size[1], 1]
    }
    /// Width (`SizeX`); `texture.size_x()`.
    pub fn size_x(&self) -> i32 {
        self.size[0]
    }
    /// Height (`SizeY`); `texture.size_y()`.
    pub fn size_y(&self) -> i32 {
        self.size[1]
    }
    /// Depth (`SizeZ`), 1 for a 2D image; `texture.size_z()`.
    pub fn size_z(&self) -> i32 {
        1
    }
    /// Samples per pixel (`NbSamples`): framebuffer images are single-sampled (1).
    pub fn nb_samples(&self) -> i32 {
        1
    }
    /// The upper mipmap level (`MaxMipmapLevel`): framebuffer images have none (0).
    pub fn max_mipmap_level(&self) -> i32 {
        0
    }
    /// Whether the image has mipmaps (`HasMipmaps`); `!texture.has_mipmaps()`.
    pub fn has_mipmaps(&self) -> bool {
        self.max_mipmap_level() > 0
    }
    /// Whether the image is an alpha-only format (`IsAlpha`); never for framebuffer images.
    pub fn is_alpha(&self) -> bool {
        false
    }
    /// Rows are stored top-down (`IsTopDown`), the native default; `texture.is_top_down()`.
    pub fn is_top_down(&self) -> bool {
        true
    }
    /// Whether the image is a point-sprite texture (`IsPointSprite`); never.
    pub fn is_point_sprite(&self) -> bool {
        false
    }
    /// Content revision (`Revision`): framebuffer images are not uploaded from an image (0).
    pub fn revision(&self) -> usize {
        0
    }
}

#[cfg(test)]
#[path = "render_texture_tests.rs"]
mod tests;
