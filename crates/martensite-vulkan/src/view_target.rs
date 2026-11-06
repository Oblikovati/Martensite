//! Logical framebuffer targets let V3d's dump lifecycle select offscreen dimensions.
use crate::{RenderError, error::rejected};
use martensite_tkernel::standard::{Handle, SharedState, StandardTransient};
use martensite_tkservice::graphic3d::Graphic3dCView;
/// An offscreen framebuffer of a view (`GlFrameBuffer` as `Graphic3dCView::FBOCreate`
/// returns it): the allocated size, the viewport rendered into it and the viewport given at
/// creation. The colour and depth-stencil images are allocated by the device for each frame
/// rendered into it. Example:
/// `view.fbo_create(64, 48).cast::<RenderFrameBuffer>.get.map(|fbo| fbo.size)`.
pub struct RenderFrameBuffer {
    transient: StandardTransient,
    viewport: SharedState<([i32; 2], std::sync::Arc<()>)>,
    size: [i32; 2],
    init_viewport: [i32; 2],
}
martensite_tkernel::impl_transient!(
    RenderFrameBuffer,
    "Martensite_RenderFrameBuffer",
    StandardTransient,
    transient
);
impl RenderFrameBuffer {
    /// `FBOCreate`: a non-positive side allocates 2 pixels (native `GlFrameBuffer::Init`)
    /// and keeps the given viewport; a side beyond the device image limit is refused (null).
    pub(crate) fn create(width: i32, height: i32, limit: u32) -> Handle<StandardTransient> {
        let allocated = |side: i32| if side > 0 { side } else { 2 };
        let size = [allocated(width), allocated(height)];
        if size.iter().any(|&side| side as u32 > limit) {
            return Handle::default();
        }
        Handle::new(Self {
            transient: StandardTransient::default(),
            viewport: SharedState::new(([width, height], std::sync::Arc::new(()))),
            size,
            init_viewport: [width, height],
        })
        .cast()
    }
    /// `FBOGetDimensions`: viewport width and height, then allocated width and height.
    pub(crate) fn dimensions(&self) -> [i32; 4] {
        let [w, h] = self.viewport.read().0;
        [w, h, self.size[0], self.size[1]]
    }
    /// Allocated width (`GetSizeX`); `fbo.size_x()`.
    pub fn size_x(&self) -> i32 {
        self.size[0]
    }
    /// Allocated height (`GetSizeY`); `fbo.size_y()`.
    pub fn size_y(&self) -> i32 {
        self.size[1]
    }
    /// Allocated size (`GetSize`); `fbo.size() == [64, 48]`.
    pub fn size(&self) -> [i32; 2] {
        self.size
    }
    /// Viewport width (`GetVPSizeX`); `fbo.viewport_size_x()`.
    pub fn viewport_size_x(&self) -> i32 {
        self.viewport.read().0[0]
    }
    /// Viewport height (`GetVPSizeY`); `fbo.viewport_size_y()`.
    pub fn viewport_size_y(&self) -> i32 {
        self.viewport.read().0[1]
    }
    /// Viewport size (`GetVPSize`); `fbo.viewport_size()`.
    pub fn viewport_size(&self) -> [i32; 2] {
        self.viewport.read().0
    }
    /// Viewport width given at creation (`GetInitVPSizeX`); `fbo.init_viewport_size_x()`.
    pub fn init_viewport_size_x(&self) -> i32 {
        self.init_viewport[0]
    }
    /// Viewport height given at creation (`GetInitVPSizeY`); `fbo.init_viewport_size_y()`.
    pub fn init_viewport_size_y(&self) -> i32 {
        self.init_viewport[1]
    }
    /// Viewport size given at creation (`GetInitVPSize`); `fbo.init_viewport_size()`.
    pub fn init_viewport_size(&self) -> [i32; 2] {
        self.init_viewport
    }
    /// `ChangeViewport`: any size, as native; a frame is captured only while the viewport is
    /// positive and inside the allocated size. Example: `fbo.change_viewport(32, 16)`.
    pub fn change_viewport(&self, width: i32, height: i32) {
        *self.viewport.write() = ([width, height], std::sync::Arc::new(()));
    }
    pub(crate) fn viewport_stamp(&self) -> ([i32; 4], std::sync::Arc<()>) {
        let viewport = self.viewport.read();
        let [width, height] = viewport.0;
        (
            [width, height, self.size[0], self.size[1]],
            viewport.1.clone(),
        )
    }
    /// `IsValid`: a created framebuffer is valid; `fbo.is_valid()`.
    pub fn is_valid(&self) -> bool {
        true
    }
    /// `NbSamples`: single-sampled images (0, as native); `fbo.nb_samples() == 0`.
    pub fn nb_samples(&self) -> i32 {
        0
    }
    /// `NbColorBuffers`: one colour image; `fbo.nb_color_buffers() == 1`.
    pub fn nb_color_buffers(&self) -> i32 {
        1
    }
    /// `HasColor`: the framebuffer has a colour image; `fbo.has_color()`.
    pub fn has_color(&self) -> bool {
        true
    }
    /// `HasDepth`: the framebuffer has a depth-stencil image; `fbo.has_depth()`.
    pub fn has_depth(&self) -> bool {
        true
    }
    /// `IsColorRenderBuffer`: the colour image is sampled, not a render buffer (false).
    pub fn is_color_render_buffer(&self) -> bool {
        false
    }
    /// `IsDepthStencilRenderBuffer`: the depth image is sampled, not a render buffer (false).
    pub fn is_depth_stencil_render_buffer(&self) -> bool {
        false
    }
    /// `ColorTexture`: the colour image of the allocated size for index 0, else `None`;
    /// `fbo.color_texture(0).map(|texture| texture.size_x())`.
    pub fn color_texture(&self, index: usize) -> Option<crate::RenderTexture> {
        let color = crate::RenderTextureAttachment::Color;
        (index == 0).then(|| crate::RenderTexture::new(color, self.size))
    }
    /// `DepthStencilTexture`: the depth-stencil image of the allocated size.
    pub fn depth_stencil_texture(&self) -> crate::RenderTexture {
        let depth = crate::RenderTextureAttachment::DepthStencil;
        crate::RenderTexture::new(depth, self.size)
    }
}
pub(crate) fn capture_extent(
    view: &dyn Graphic3dCView,
    limit: u32,
) -> Result<[u32; 2], RenderError> {
    let dimensions = target_dimensions(view);
    let Some((width, height)) = dimensions else {
        return Err(rejected(
            "missing window or foreign framebuffer",
            "a Martensite capture target",
        ));
    };
    if width <= 0 || height <= 0 || width as u32 > limit || height as u32 > limit {
        return Err(rejected(
            (width, height),
            &format!("positive extent at most {limit}"),
        ));
    }
    Ok([width as u32, height as u32])
}

fn target_dimensions(view: &dyn Graphic3dCView) -> Option<(i32, i32)> {
    let fbo = view.fbo();
    if fbo.is_null() {
        return view.window().get().map(|window| window.size());
    }
    fbo.cast::<RenderFrameBuffer>().get().map(|target| {
        let [width, height, max_width, max_height] = target.dimensions();
        // A viewport beyond the allocated images is refused like a non-positive one.
        if width > max_width || height > max_height {
            (0, 0)
        } else {
            (width, height)
        }
    })
}
