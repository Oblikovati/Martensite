//! The view's environment texture (`GlView::SetTextureEnv`).
//!
//! `GlWorkspace::TextureSet` substitutes the environment texture for an aspect without a
//! texture set (`!ToMapTexture` and no marker sprite) while a layer with
//! `UseEnvironmentTexture` renders (`GlLayerList::renderLayer`). `GlPrimitiveArray`
//! then binds the `Graphic3dShaderFlags_TextureEnv` program for faces, which is the unlit program
//! for every shading model: its vertex stage reflects the view direction about the view-space
//! normal and maps the reflection onto the sphere map (`shaders/sphere_map.glsl`). Line programs
//! bind no texture. Under the fixed-function option (ffpEnable) the environment is bound as an
//! ordinary texture with `GL_SPHERE_MAP` generation (`surface_sphere_texgen`).
use crate::{
    PrimitiveDraw, PrimitiveTopology, RenderError, error::rejected, primitive::PrimitiveSubmission,
    surface_texture::SurfaceTextureStyle, text_submission::TextSceneContext,
};
use martensite_tkernel::ncollection::Mat4;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;

/// The float matrices the native program reads and the nodal normals of one draw.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SphereMapTransform {
    /// `occNormal` per vertex; zero where the array has none (the generic attribute default).
    pub normals: Vec<[f32; 3]>,
    /// occModelWorldMatrix, its inverse transpose, occWorldViewMatrix, its inverse transpose.
    matrices: [[f32; 16]; 4],
}
impl SphereMapTransform {
    /// Invert the float matrices as `NCollectionMat4<float>::Inverted` does for the state
    /// uniforms; `SphereMapTransform::new(normals, model_world, world_view)?`.
    pub fn new(
        normals: Vec<[f32; 3]>,
        model_world: [f32; 16],
        world_view: [f32; 16],
    ) -> Result<Self, RenderError> {
        Ok(Self {
            normals,
            matrices: [
                model_world,
                inverse_transpose(model_world)?,
                world_view,
                inverse_transpose(world_view)?,
            ],
        })
    }
    /// The std430 `SphereMapMatrices` block; `transform.encode().len() == 256`.
    pub fn encode(&self) -> Vec<u8> {
        self.matrices
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect()
    }
}
fn inverse_transpose(matrix: [f32; 16]) -> Result<[f32; 16], RenderError> {
    Mat4::from_data(matrix)
        .inverted()
        .map(|inverse| inverse.transposed().data())
        .map_err(|error| rejected((matrix, error), "an invertible environment-mapping matrix"))
}

/// Map the view's environment texture onto the faces of an aspect without textures; the
/// caller has checked `!aspects.to_map_texture()`. Example:
/// `apply_environment_texture(&mut draws, primitive, aspects, context)?`.
pub(crate) fn apply_environment_texture(
    draws: &mut [PrimitiveDraw],
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &TextSceneContext<'_>,
) -> Result<(), RenderError> {
    let Some(environment) = context.environment else {
        return Ok(());
    };
    if !draws.iter().any(is_face_draw) {
        return Ok(());
    }
    let Some(style) = environment_style(environment, primitive, aspects, context)? else {
        return Ok(());
    };
    for draw in draws.iter_mut().filter(|draw| is_face_draw(draw)) {
        draw.surface_texture = Some(style.clone());
    }
    Ok(())
}
fn is_face_draw(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Triangles
}
fn environment_style(
    environment: &Handle<Graphic3dTextureEnv>,
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &TextSceneContext<'_>,
) -> Result<Option<SurfaceTextureStyle>, RenderError> {
    let images = &context.group.texture_images;
    let Some(image) = images.snapshot_environment(environment)? else {
        return Ok(None);
    };
    let parameters = environment.get_params();
    let sampler = images.samplers.policy(&image, &parameters)?;
    let transform = sphere_map_transform(primitive, context)?;
    let uv = vec![[0., 0., 0., 1.]; transform.normals.len()];
    let mut style = crate::retained_surface_texture::extracted_surface_style(
        image,
        uv,
        &parameters,
        sampler,
        primitive,
        &**aspects,
    );
    if context.texture_coordinates(&**aspects)
        == crate::RenderTextureCoordinatePipeline::GeneratedCompatibility
    {
        // ffpEnable binds the environment as an ordinary FFP texture: GL_SPHERE_MAP generation
        // (Graphic3dTextureEnv's GenMode) and its own GL_MODULATE/GL_DECAL environment.
        let input = vec![[0., 0., 0., 1.]; transform.normals.len()];
        let top_down = style.image.top_down;
        style.pending_texgen = Some(crate::retained_surface_texture::fixed_texgen_snapshot(
            input,
            &parameters,
            top_down,
            primitive,
            context,
        )?);
        style.fixed_environment = true;
        return Ok(Some(style));
    }
    // The TextureEnv program samples the texture alone: no material modulation.
    style.modulate = false;
    style.sphere_map = Some(Arc::new(transform));
    Ok(Some(style))
}
fn sphere_map_transform(
    primitive: &PrimitiveSubmission,
    context: &TextSceneContext<'_>,
) -> Result<SphereMapTransform, RenderError> {
    let normals = match primitive.normals()? {
        Some(normals) => normals,
        None => vec![[0.; 3]; primitive.positions()?.len()],
    };
    let (model_world, world_view) = crate::persistent_transform::lighting_matrices(context)?;
    SphereMapTransform::new(normals, model_world, world_view)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sphere_map_block_holds_the_matrices_and_their_float_inverse_transposes() {
        let mut model = Mat4::<f32>::identity().data();
        model[12] = 2.;
        model[0] = 4.;
        let view = Mat4::<f32>::identity().data();
        let transform = SphereMapTransform::new(vec![[0., 0., 1.]], model, view).unwrap();
        let block = transform.encode();
        assert_eq!(block.len(), 256);
        let floats: Vec<f32> = block
            .chunks(4)
            .map(|bytes| f32::from_ne_bytes(bytes.try_into().unwrap()))
            .collect();
        assert_eq!(&floats[..16], &model);
        // The inverse of a scale by 4 and a translation by 2, transposed.
        assert_eq!(floats[16], 0.25);
        assert_eq!(floats[16 + 3], -0.5);
        assert_eq!(&floats[32..48], &view);
    }
    #[test]
    fn singular_matrices_are_rejected_with_their_values() {
        let error = SphereMapTransform::new(vec![], [0.; 16], [0.; 16]).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("invertible environment-mapping matrix")
        );
    }
}
