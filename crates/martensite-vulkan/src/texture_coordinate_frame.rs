//! Resolve native texture binding state after transparent/capping submission ordering.
//!
//! The fixed-function coordinates of a draw follow GL texture unit 0 as it stands
//! across one frame's draws: `GlContext::BindTextures` applies a new texture's
//! `GlSampler::applyGlobalTextureParams` without resetting the previous one, and resets a
//! texture only when nothing replaces it. A Manual texture therefore inherits the generation an
//! earlier texture enabled, and `resetGlobalTextureParams` keeps generation enabled after an
//! Object texture (it compares GenMode with GL_NONE, the value of Graphic3dTOTM_OBJECT).
use crate::{
    PrimitiveDraw, RenderError, RenderVertex,
    surface_texgen::{SurfaceTexgen, TextureGenerationBinding},
    surface_texture::{SurfaceTextureImage, SurfaceTextureStyle},
};
use martensite_tkservice::graphic3d::Graphic3dTypeOfTextureMode as Mode;
use std::sync::Arc;
/// The fixed-function state of texture unit 0 between draws.
#[derive(Default)]
struct FixedTextureUnit {
    /// The bound texture and its generation mode.
    active: Option<(Arc<SurfaceTextureImage>, Mode)>,
    /// The last `glTexGen` mode and planes; GL's initial eye-linear state when none.
    generation: Option<TextureGenerationBinding>,
    /// `GL_TEXTURE_GEN_S/T`.
    generating: bool,
    /// `GL_POINT_SPRITE`.
    point_sprite: bool,
    /// `GL_COORD_REPLACE`, which no reset clears.
    coordinate_replace: bool,
}
/// Resolve immutable UVs and retire the temporary inputs; `resolve_ordered_texture_coordinates(draws)?`.
pub(crate) fn resolve_ordered_texture_coordinates(
    draws: &mut [PrimitiveDraw],
) -> Result<(), RenderError> {
    let mut unit = FixedTextureUnit::default();
    for draw in draws {
        crate::fixed_surface_environment::apply_sprite_replace(draw);
        unit.resolve(draw.surface_texture.as_mut(), &draw.vertices)?;
    }
    Ok(())
}
impl FixedTextureUnit {
    fn resolve(
        &mut self,
        style: Option<&mut SurfaceTextureStyle>,
        vertices: &[RenderVertex],
    ) -> Result<(), RenderError> {
        let Some(style) = style else {
            self.unbind();
            return Ok(());
        };
        let Some(snapshot) = style.pending_texgen.take() else {
            self.unbind();
            return Ok(());
        };
        self.bind(&style.image, &snapshot)?;
        style.uv = self.binding().coordinates(&snapshot, vertices)?;
        style.validate(vertices.len())
    }
    /// BindTextures: a different texture applies its parameters over the current state.
    fn bind(
        &mut self,
        image: &Arc<SurfaceTextureImage>,
        snapshot: &SurfaceTexgen,
    ) -> Result<(), RenderError> {
        let bound = self.active.as_ref();
        if bound.is_some_and(|(bound, _)| same_texgen_resource(bound, image)) {
            return Ok(());
        }
        match snapshot.mode {
            Mode::Object | Mode::Sphere | Mode::Eye => {
                self.generation = Some(TextureGenerationBinding::new(snapshot)?);
            }
            Mode::Sprite => (self.point_sprite, self.coordinate_replace) = (true, true),
            Mode::Manual => {}
        }
        self.generating |= snapshot.mode != Mode::Manual;
        self.active = Some((image.clone(), snapshot.mode));
        Ok(())
    }
    /// BindTextures without a texture: resetGlobalTextureParams of the bound one.
    fn unbind(&mut self) {
        let Some((_, mode)) = self.active.take() else {
            return;
        };
        if mode != Mode::Object {
            self.generating = false;
            self.point_sprite &= mode != Mode::Sprite;
        }
    }
    /// The coordinates the current state produces: the point-sprite centre, the enabled
    /// generation, or the supplied UVs.
    fn binding(&self) -> TextureGenerationBinding {
        if self.point_sprite && self.coordinate_replace {
            return TextureGenerationBinding::fixed(Mode::Sprite);
        }
        if !self.generating {
            return TextureGenerationBinding::fixed(Mode::Manual);
        }
        self.generation
            .clone()
            .unwrap_or_else(TextureGenerationBinding::initial)
    }
}
fn same_texgen_resource(left: &Arc<SurfaceTextureImage>, right: &Arc<SurfaceTextureImage>) -> bool {
    // Binding compares shared resource pointers, not immutable upload revisions.
    if !left.id.is_empty() {
        return left.id == right.id;
    }
    Arc::ptr_eq(left, right)
}
#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::standard::Handle;
    use martensite_tkservice::{graphic3d::*, image::*};
    #[test]
    fn shared_upload_revisions_keep_binding_identity_but_empty_ids_are_independent() {
        let cache = crate::retained_texture_images::RetainedTextureImages::default();
        let texture = binding_identity_texture();
        let first = cache.snapshot(&texture.cast()).unwrap().unwrap();
        texture.update_revision();
        let second = cache.snapshot(&texture.cast()).unwrap().unwrap();
        assert!(!Arc::ptr_eq(&first, &second));
        assert!(same_texgen_resource(&first, &second));
        texture.texture_root_base().fields.write().tex_id.clear();
        let unshared = cache.snapshot(&texture.cast()).unwrap().unwrap();
        let another = cache.snapshot(&texture.cast()).unwrap().unwrap();
        assert!(!same_texgen_resource(&unshared, &another));
        assert!(same_texgen_resource(&unshared, &unshared));
    }
    fn binding_identity_texture() -> Handle<Graphic3dTexture2D> {
        let image = Handle::new(ImagePixMap::default());
        assert!(image.init_wrapper(ImageFormat::Rgba, Some(vec![255; 4]), 1, 1, 4));
        Handle::new(Graphic3dTexture2D::from_pix_map(image))
    }
    /// A distinct one-texel texture image.
    fn texture_image(
        cache: &crate::retained_texture_images::RetainedTextureImages,
    ) -> Arc<SurfaceTextureImage> {
        cache
            .snapshot(&binding_identity_texture().cast())
            .unwrap()
            .unwrap()
    }
    /// A generation snapshot of `mode` (planes S = x / 4, T = y / 4) for one vertex whose
    /// supplied UV is (0.25, 0.75), under an identity model-view.
    fn snapshot(mode: Graphic3dTypeOfTextureMode) -> Box<SurfaceTexgen> {
        let parameters = Graphic3dTextureParams::default();
        let plane = |axis: usize| {
            let mut plane = [0_f32; 4];
            plane[axis] = 0.25;
            martensite_tkernel::ncollection::Vec4::from_array(plane)
        };
        parameters.set_gen_mode(mode, plane(0), plane(1));
        let identity = martensite_tkernel::ncollection::Mat4::<f32>::identity().data();
        let input = vec![[0.25, 0.75, 0., 1.]];
        let mut snapshot = SurfaceTexgen::snapshot(input, &parameters, false, identity, identity);
        snapshot.attach_sphere(vec![[0., 0., 1.]], false).unwrap();
        snapshot
    }
    fn coordinates(
        unit: &mut FixedTextureUnit,
        image: &Arc<SurfaceTextureImage>,
        mode: Graphic3dTypeOfTextureMode,
    ) -> [f32; 2] {
        let snapshot = snapshot(mode);
        unit.bind(image, &snapshot).unwrap();
        let vertices = [RenderVertex::new([2., 1., -3.], [1.; 4])];
        let uv = unit.binding().coordinates(&snapshot, &vertices).unwrap()[0];
        [uv[0], uv[1]]
    }
    const SUPPLIED: [f32; 2] = [0.25, 0.75];
    #[test]
    fn a_manual_texture_inherits_the_generation_of_the_texture_it_replaces() {
        let cache = crate::retained_texture_images::RetainedTextureImages::default();
        let (first, second) = (texture_image(&cache), texture_image(&cache));
        let mut unit = FixedTextureUnit::default();
        let object = coordinates(&mut unit, &first, Graphic3dTypeOfTextureMode::Object);
        assert_eq!(object, [0.5, 0.25]);
        let manual = coordinates(&mut unit, &second, Graphic3dTypeOfTextureMode::Manual);
        assert_eq!(
            manual, object,
            "BindTextures applies the new texture without a reset"
        );
    }
    #[test]
    fn an_unbind_between_textures_disables_all_generation_but_object() {
        let cache = crate::retained_texture_images::RetainedTextureImages::default();
        let (first, second) = (texture_image(&cache), texture_image(&cache));
        for (mode, expected) in [
            (Graphic3dTypeOfTextureMode::Sphere, SUPPLIED),
            (Graphic3dTypeOfTextureMode::Eye, SUPPLIED),
            (Graphic3dTypeOfTextureMode::Sprite, SUPPLIED),
            // resetGlobalTextureParams compares GenMode with GL_NONE == Graphic3dTOTM_OBJECT.
            (Graphic3dTypeOfTextureMode::Object, [0.5, 0.25]),
        ] {
            let mut unit = FixedTextureUnit::default();
            coordinates(&mut unit, &first, mode);
            unit.unbind();
            let manual = coordinates(&mut unit, &second, Graphic3dTypeOfTextureMode::Manual);
            assert_eq!(manual, expected, "{mode:?}");
        }
    }
    #[test]
    fn a_sprite_keeps_the_point_centre_until_its_own_reset() {
        let cache = crate::retained_texture_images::RetainedTextureImages::default();
        let (first, second) = (texture_image(&cache), texture_image(&cache));
        let mut unit = FixedTextureUnit::default();
        assert_eq!(
            coordinates(&mut unit, &first, Graphic3dTypeOfTextureMode::Sprite),
            [0.5, 0.5]
        );
        let manual = coordinates(&mut unit, &second, Graphic3dTypeOfTextureMode::Manual);
        assert_eq!(manual, [0.5, 0.5]);
    }
    #[test]
    fn the_same_texture_keeps_its_bound_state_and_a_fresh_unit_uses_the_supplied_uvs() {
        let cache = crate::retained_texture_images::RetainedTextureImages::default();
        let image = texture_image(&cache);
        let mut unit = FixedTextureUnit::default();
        assert_eq!(
            coordinates(&mut unit, &image, Graphic3dTypeOfTextureMode::Manual),
            SUPPLIED
        );
        let sphere = coordinates(&mut unit, &image, Graphic3dTypeOfTextureMode::Sphere);
        assert_eq!(sphere, SUPPLIED, "an already bound texture applies nothing");
    }
}
