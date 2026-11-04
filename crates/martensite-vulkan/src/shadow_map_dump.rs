//! `GlView::ShadowMapDump`: the depth texels of the shadow map the last
//! redraw rendered for a named light, read back as a GrayF image.
use crate::view_statistics::RenderRetainedView;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::graphic3d::Graphic3dCViewMembers;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};

impl RenderRetainedView {
    /// Dump the shadow map of the casting light named `light_name` into `image`; false without
    /// rendered maps, for an unknown light, or when the resolution changed since the redraw.
    /// The trait forwarding (`retained.shadow_map_dump`) resolves to this method.
    /// Example: `view.shadow_map_dump(&image, &AsciiString::from("headlight"))`.
    pub fn shadow_map_dump(&self, image: &ImagePixMap, light_name: &AsciiString) -> bool {
        let Some(maps) = self.shadow_cameras.read().rendered.clone() else {
            return false;
        };
        let resolution = maps.resolution();
        let named = maps.maps().iter().find(|map| {
            map.light
                .get()
                .is_some_and(|light| light.name() == *light_name)
        });
        let Some(map) = named else {
            return false;
        };
        // Native compares the map framebuffer with the current ShadowMapResolution.
        if i64::from(resolution) != i64::from(self.rendering_params().shadow_map_resolution) {
            return false;
        }
        write_depth(image, resolution as usize, map.depth())
    }
}

/// `glReadPixels(GL_DEPTH_COMPONENT, GL_FLOAT)` into `image`, bottom row first; the image is
/// reinitialized as GrayF when its size differs (native) or its pixels are not 4 bytes (where
/// native would overrun the buffer).
fn write_depth(image: &ImagePixMap, resolution: usize, depth: &[f32]) -> bool {
    let sized = image.width() == resolution && image.height() == resolution;
    if !sized || image.size_pixel_bytes() != 4 {
        image.init_zero(ImageFormat::GrayF, resolution, resolution, 0, 0);
    }
    let mut state = image.change_data();
    let bytes = state.data.change_data();
    let row_bytes = bytes.len() / resolution.max(1);
    for (y, row) in depth.chunks_exact(resolution).enumerate() {
        let target = &mut bytes[y * row_bytes..y * row_bytes + 4 * resolution];
        for (texel, value) in target.as_chunks_mut::<4>().0.iter_mut().zip(row) {
            texel.copy_from_slice(&value.to_le_bytes());
        }
    }
    true
}
