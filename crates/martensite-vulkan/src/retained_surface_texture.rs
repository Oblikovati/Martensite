//! Surface extraction snapshots content by revision while reading UV/sampler policy live.
use crate::{
    PrimitiveDraw, RenderError, primitive::PrimitiveSubmission, surface_texture::*,
    text_submission::TextSceneContext,
};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;
/// Share the driver's image cache across groups; `driver_texture_images(&structure)`.
pub(crate) fn driver_texture_images(
    structure: &Handle<dyn Graphic3dStructureSource>,
) -> Arc<crate::retained_texture_images::RetainedTextureImages> {
    let driver = structure
        .structure_manager()
        .graphic_driver()
        .cast::<crate::Graphic3dRenderDriver>();
    driver
        .get()
        .map_or_else(Arc::default, |driver| driver.texture_images.clone())
}
/// Resolve actual retained surface textures; `apply_surface_texture(draws, primitive, aspects, context)?`.
pub(crate) fn apply_surface_texture(
    draws: &mut [PrimitiveDraw],
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &TextSceneContext<'_>,
) -> Result<(), RenderError> {
    if !aspects.to_map_texture() {
        return crate::environment_texture_map::apply_environment_texture(
            draws, primitive, aspects, context,
        );
    }
    crate::point_surface_texture::require_plain_points(draws)?;
    if primitive.kind == Graphic3dTypeOfPrimitiveArray::Points
        && crate::lighting_pipeline::structure_lighting_pipeline(&context.group.structure())
            == crate::RenderLightingPipeline::FixedFunction
    {
        return Err(crate::error::rejected(
            "fixed-function UV-mapped points",
            "programmable mapped points; fixed-function mapping is unsupported",
        ));
    }
    let Some(style) = retained_surface_style(primitive, aspects, context)? else {
        return apply_unavailable_surface_texture(
            draws,
            primitive,
            &**aspects,
            primitive.kind != Graphic3dTypeOfPrimitiveArray::Points
                && context.texture_coordinates(&**aspects)
                    == crate::RenderTextureCoordinatePipeline::GeneratedCompatibility,
        );
    };
    for draw in draws {
        style.validate(draw.vertices.len())?;
        draw.surface_texture = Some(style.clone());
        draw.surface_texture_binding_only = draw.topology() == crate::PrimitiveTopology::Lines;
        crate::point_surface_texture::finish_point_mapping(draw)?;
    }
    Ok(())
}
fn apply_unavailable_surface_texture(
    draws: &mut [PrimitiveDraw],
    primitive: &PrimitiveSubmission,
    aspects: &dyn Graphic3dAspectsSource,
    fixed_environment: bool,
) -> Result<(), RenderError> {
    let handle = aspects.texture_map();
    let Some(texture) = handle.get() else {
        return Ok(());
    };
    let vertex_override = primitive
        .attributes
        .attribute_data(Graphic3dTypeOfAttribute::Color)
        .is_some();
    let unavailable = SurfaceTextureUnavailable {
        resource_id: texture.get_id().as_bytes().to_vec(),
        fixed_environment,
        modulate: texture.get_params().is_modulate(),
        vertex_override,
        back_vertex: aspects.to_use_vertex_color_for_back_faces(),
    };
    for draw in draws {
        draw.surface_texture_unavailable = Some(unavailable.clone());
        draw.surface_texture_binding_only = draw.topology() == crate::PrimitiveTopology::Lines;
        crate::point_surface_texture::finish_point_mapping(draw)?;
    }
    Ok(())
}
fn retained_surface_style(
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &TextSceneContext<'_>,
) -> Result<Option<SurfaceTextureStyle>, RenderError> {
    let handle = aspects.texture_map();
    let Some(texture) = handle.get() else {
        return Ok(None);
    };
    validate_texture_family(texture, &**aspects)?;
    let image = context
        .group
        .aspect_textures
        .snapshot(aspects, &context.group.texture_images)?;
    let Some(image) = image else {
        return Ok(None);
    };
    complete_surface_style(image, primitive, aspects, context).map(Some)
}
fn complete_surface_style(
    image: Arc<SurfaceTextureImage>,
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &TextSceneContext<'_>,
) -> Result<SurfaceTextureStyle, RenderError> {
    let parameters = aspects.texture_map().get_params();
    let uv = if primitive.kind == Graphic3dTypeOfPrimitiveArray::Points {
        crate::surface_texture_uv::point_texture_coordinates(primitive)?
    } else {
        crate::surface_texture_uv::surface_texture_coordinates(
            primitive,
            &parameters,
            image.top_down,
        )?
    };
    let sampler = context
        .group
        .texture_images
        .samplers
        .policy(&image, &parameters)?;
    let mut style = extracted_surface_style(image, uv, &parameters, sampler, primitive, &**aspects);
    style.material_maps = crate::surface_material_maps::snapshot_material_maps(aspects, context)?;
    snapshot_fixed_surface_binding(
        &mut style,
        primitive,
        &parameters,
        context,
        context.texture_coordinates(&**aspects),
    )?;
    Ok(style)
}
fn snapshot_fixed_surface_binding(
    style: &mut SurfaceTextureStyle,
    primitive: &PrimitiveSubmission,
    parameters: &Graphic3dTextureParams,
    context: &TextSceneContext<'_>,
    coordinates: crate::RenderTextureCoordinatePipeline,
) -> Result<(), RenderError> {
    if primitive.kind == Graphic3dTypeOfPrimitiveArray::Points {
        return Ok(());
    }
    if coordinates == crate::RenderTextureCoordinatePipeline::GeneratedCompatibility {
        let input = crate::surface_texture_uv::surface_input_uv(primitive)?;
        style.fixed_environment = true;
        apply_fixed_sprite_environment(style, parameters.gen_mode());
        style.pending_texgen = Some(fixed_texgen_snapshot(
            input,
            parameters,
            style.image.top_down,
            primitive,
            context,
        )?);
    }
    Ok(())
}
/// The fixed-function generation snapshot of a retained draw, with the `GL_SPHERE_MAP` normals
/// and `GL_NORMALIZE` state of a Sphere-mode texture;
/// `fixed_texgen_snapshot(uv, &parameters, top_down, primitive, context)?`.
pub(crate) fn fixed_texgen_snapshot(
    input: Vec<[f32; 4]>,
    parameters: &Graphic3dTextureParams,
    top_down: bool,
    primitive: &PrimitiveSubmission,
    context: &TextSceneContext<'_>,
) -> Result<Box<crate::surface_texgen::SurfaceTexgen>, RenderError> {
    let (world, view) = crate::persistent_transform::lighting_matrices(context)?;
    let mut snapshot =
        crate::surface_texgen::SurfaceTexgen::snapshot(input, parameters, top_down, world, view);
    let normalize = crate::fixed_normalization::native_normalize_state(
        context.camera,
        context.structure,
        view,
        !context.group.transform_persistence().is_null(),
    );
    snapshot.attach_sphere(primitive.normals()?.unwrap_or_default(), normalize)?;
    Ok(snapshot)
}
pub(crate) fn extracted_surface_style(
    image: Arc<SurfaceTextureImage>,
    uv: Vec<[f32; 4]>,
    parameters: &Graphic3dTextureParams,
    sampler: SurfaceSamplerPolicy,
    primitive: &PrimitiveSubmission,
    aspects: &dyn Graphic3dAspectsSource,
) -> SurfaceTextureStyle {
    let vertex_override = primitive
        .attributes
        .attribute_data(Graphic3dTypeOfAttribute::Color)
        .is_some();
    SurfaceTextureStyle {
        sphere_map: None,
        image,
        pending_texgen: None,
        fixed_environment: false,
        uv,
        sampler,
        back_vertex: aspects.to_use_vertex_color_for_back_faces(),
        vertex_override,
        modulate: parameters.is_modulate(),
        native_gen_mode: parameters.gen_mode(),
        material_maps: None,
    }
}
pub(crate) fn validate_texture_family(
    _texture: &dyn Graphic3dTextureMap,
    aspects: &dyn Graphic3dAspectsSource,
) -> Result<(), RenderError> {
    // The programmable pipeline ignores GenMode (Graphic3dShaderManager reads only the UVs).
    let set = aspects.texture_set();
    let textures = set.get().map(|set| set.textures()).unwrap_or_default();
    let programmable = !aspects.shader_program().is_null();
    crate::surface_material_maps::validate_texture_set(&textures, programmable)
}
/// A 3D base colour is read only by a user program's `sampler3D`: the built-in programs declare
/// `sampler2D occSampler0`, and edges keep built-in programs. Refuses such draws without a user
/// program; `refuse_builtin_volume_textures(&draws)?`.
pub(crate) fn refuse_builtin_volume_textures(draws: &[PrimitiveDraw]) -> Result<(), RenderError> {
    let builtin = draws.iter().find(|draw| {
        let volume = draw
            .surface_texture
            .as_ref()
            .and_then(|style| style.image.depth);
        volume.is_some() && draw.user_program.is_none()
    });
    match builtin {
        Some(draw) => Err(crate::error::rejected(
            (draw.topology(), "3D texture without a user program"),
            "a 3D texture sampled by a user program (built-in programs sample 2D textures)",
        )),
        None => Ok(()),
    }
}
/// GlSampler::applyGlobalTextureParams binds a Graphic3dTOTM_SPRITE texture with
/// GL_REPLACE whatever IsModulate says; the decal environment equals it for opaque texels.
/// Example: `apply_fixed_sprite_environment(&mut style, parameters.gen_mode)`.
pub(crate) fn apply_fixed_sprite_environment(
    style: &mut SurfaceTextureStyle,
    generation: Graphic3dTypeOfTextureMode,
) {
    if generation == Graphic3dTypeOfTextureMode::Sprite {
        style.modulate = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::ncollection::Vec4;
    #[test]
    fn programmable_textures_accept_every_generation_mode() {
        let texture = Handle::new(Graphic3dTexture2D::from_pix_map(Handle::default()));
        let aspects = Graphic3dAspects::default();
        aspects.set_texture_map(texture.cast());
        for mode in [
            Graphic3dTypeOfTextureMode::Sphere,
            Graphic3dTypeOfTextureMode::Sprite,
        ] {
            let plane = Vec4::new(1., 0., 0., 0.);
            texture.get_params().set_gen_mode(mode, plane, plane);
            assert!(validate_texture_family(&*texture, &aspects).is_ok());
        }
    }
}
