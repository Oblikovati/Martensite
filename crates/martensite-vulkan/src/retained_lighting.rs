//! Resolve native material and normal policy at live Graphic3d extraction, not presentation creation.
use crate::{
    PrimitiveDraw, RenderError, RenderLayer, RenderLighting, RenderMaterial,
    RenderMaterialColorSpace, RenderShading, error::rejected,
};
use crate::{primitive::PrimitiveSubmission, text_submission::TextSceneContext};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
pub(crate) fn apply_surface_lighting(
    draws: &mut [PrimitiveDraw],
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &TextSceneContext<'_>,
) -> Result<(), RenderError> {
    let pipeline =
        crate::lighting_pipeline::structure_lighting_pipeline(&context.group.structure());
    let model =
        primitive_shading_model(draws, &**aspects, context.rendering.shading_model, pipeline);
    let model = if primitive.kind == Graphic3dTypeOfPrimitiveArray::Points
        && aspects.to_map_texture()
        && primitive
            .attributes
            .attribute_data(Graphic3dTypeOfAttribute::Norm)
            .is_none()
    {
        Graphic3dTypeOfShadingModel::Unlit
    } else {
        model
    };
    if primitive.kind == Graphic3dTypeOfPrimitiveArray::Points
        && aspects.to_map_texture()
        && is_pbr_model(model)
    {
        return Err(rejected(
            model,
            "Unlit, Gouraud or Phong mapped points; PBR mapping is unsupported",
        ));
    }
    crate::point_surface_texture::resolve_point_shading_base(draws, model);
    if model == Graphic3dTypeOfShadingModel::Unlit {
        if pipeline == crate::RenderLightingPipeline::FixedFunction {
            return Ok(());
        }
        let policy =
            crate::lighting_pipeline::structure_unlit_face_policy(&context.group.structure());
        return crate::unlit_face_color::apply_unlit_face_policy(draws, &**aspects, policy);
    }
    apply_resolved_lighting(draws, primitive, aspects, context, model)
}
fn primitive_shading_model(
    draws: &[PrimitiveDraw],
    aspect: &dyn Graphic3dAspectsSource,
    view: Graphic3dTypeOfShadingModel,
    pipeline: crate::RenderLightingPipeline,
) -> Graphic3dTypeOfShadingModel {
    let model = fixed_compatible_shading(resolved_model(aspect, view), pipeline);
    if draws.iter().any(texture_replaces_lighting) {
        return Graphic3dTypeOfShadingModel::Unlit;
    }
    if matches!(
        model,
        Graphic3dTypeOfShadingModel::PhongFacet | Graphic3dTypeOfShadingModel::PbrFacet
    ) && draws
        .iter()
        .all(|draw| draw.topology() != crate::PrimitiveTopology::Triangles)
    {
        return Graphic3dTypeOfShadingModel::Unlit;
    }
    model
}
fn fixed_compatible_shading(
    model: Graphic3dTypeOfShadingModel,
    pipeline: crate::RenderLightingPipeline,
) -> Graphic3dTypeOfShadingModel {
    if pipeline != crate::RenderLightingPipeline::FixedFunction {
        return model;
    }
    // FFP binds common materials; accepting a PBR request here does not implement a PBR shader.
    match model {
        Graphic3dTypeOfShadingModel::Pbr => Graphic3dTypeOfShadingModel::Phong,
        Graphic3dTypeOfShadingModel::PbrFacet => Graphic3dTypeOfShadingModel::PhongFacet,
        _ => model,
    }
}
pub(crate) fn validate_unlit_draws(
    draws: &[PrimitiveDraw],
    aspect: &dyn Graphic3dAspectsSource,
) -> Result<(), RenderError> {
    // OpenGL 4.6 §15.2 gives actual Points and Lines no back-facing fragments.
    if draws
        .iter()
        .all(|draw| draw.topology() != crate::PrimitiveTopology::Triangles)
        || draws.iter().any(|draw| {
            draw.surface_texture.is_some() || draw.surface_texture_unavailable.is_some()
        })
    {
        return Ok(());
    }
    validate_unlit_surface(aspect)
}
fn apply_resolved_lighting(
    draws: &mut [PrimitiveDraw],
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &TextSceneContext<'_>,
    model: Graphic3dTypeOfShadingModel,
) -> Result<(), RenderError> {
    let normals = primitive.normals()?;
    if draws
        .iter()
        .any(|draw| draw.topology() != crate::PrimitiveTopology::Triangles)
    {
        return lower_nodal_lighting(draws, primitive, aspects, context, model, normals);
    }
    let lighting = surface_lighting(primitive, &**aspects, context, model, normals.as_deref())?;
    for draw in draws {
        let mut snapshot = lighting.clone();
        snapshot.normals = normals
            .clone()
            .unwrap_or_else(|| vec![[0., 0., 1.]; draw.vertices.len()]);
        draw.lighting = Some(snapshot);
    }
    Ok(())
}
fn lower_nodal_lighting(
    draws: &mut [PrimitiveDraw],
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &TextSceneContext<'_>,
    model: Graphic3dTypeOfShadingModel,
    normals: Option<Vec<[f32; 3]>>,
) -> Result<(), RenderError> {
    if draws
        .iter()
        .all(|draw| draw.point_style.is_some() || draw.line_style.is_some())
    {
        return apply_nodal_lighting(draws, primitive, aspects, context, model, normals);
    }
    validate_non_surface_lighting(model, normals.is_some())
}

pub(crate) fn texture_replaces_lighting(draw: &PrimitiveDraw) -> bool {
    // The TextureEnv program is the unlit program for every shading model.
    let environment = draw
        .surface_texture
        .as_ref()
        .is_some_and(|texture| texture.sphere_map.is_some());
    environment
        || !draw.surface_texture_binding_only
            && (draw
                .surface_texture
                .as_ref()
                .is_some_and(|texture| !texture.modulate)
                || draw
                    .surface_texture_unavailable
                    .as_ref()
                    .is_some_and(|texture| !texture.modulate))
}

fn apply_nodal_lighting(
    draws: &mut [PrimitiveDraw],
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &TextSceneContext<'_>,
    model: Graphic3dTypeOfShadingModel,
    normals: Option<Vec<[f32; 3]>>,
) -> Result<(), RenderError> {
    let Some(normals) = normals else {
        return Ok(());
    };
    if !matches!(
        model,
        Graphic3dTypeOfShadingModel::Gouraud
            | Graphic3dTypeOfShadingModel::Phong
            | Graphic3dTypeOfShadingModel::Pbr
    ) {
        return validate_non_surface_lighting(model, true);
    }
    let mut lighting = surface_lighting(primitive, &**aspects, context, model, Some(&normals))?;
    if draws
        .iter()
        .any(|draw| draw.topology() == crate::PrimitiveTopology::Points)
        && crate::lighting_pipeline::structure_lighting_pipeline(&context.group.structure())
            != crate::RenderLightingPipeline::FixedFunction
    {
        lighting.validate_opaque_marker_lighting()?;
    }
    lighting.normals = normals;
    for draw in draws {
        draw.lighting = Some(lighting.clone());
    }
    Ok(())
}

pub(crate) fn validate_unlit_surface(
    aspects: &dyn Graphic3dAspectsSource,
) -> Result<(), RenderError> {
    if aspects.distinguish() || !aspects.to_use_vertex_color_for_back_faces() {
        return Err(rejected(
            (
                aspects.distinguish(),
                aspects.to_use_vertex_color_for_back_faces(),
            ),
            "unlit distinction/back-color policy is not implemented yet",
        ));
    }
    Ok(())
}
fn surface_lighting(
    primitive: &PrimitiveSubmission,
    aspects: &dyn Graphic3dAspectsSource,
    context: &TextSceneContext<'_>,
    model: Graphic3dTypeOfShadingModel,
    normals: Option<&[[f32; 3]]>,
) -> Result<RenderLighting, RenderError> {
    let shading = surface_algorithm(model, normals.is_some())?;
    let (world, view) = crate::persistent_transform::lighting_matrices(context)?;
    let mut snapshot = RenderLighting::new(
        Vec::new(),
        aspect_material(aspects),
        world,
        view,
        !context.camera.is_orthographic(),
        shading,
    )?
    .with_vertex_colors(
        primitive_has_lighting_colors(primitive)
            || (primitive.kind == Graphic3dTypeOfPrimitiveArray::Points
                && aspects.to_map_texture()),
        !aspects.to_use_vertex_color_for_back_faces(),
    )
    .with_pbr(is_pbr_model(model));
    snapshot.normalize_fixed = crate::fixed_normalization::native_normalize_state(
        context.camera,
        context.structure,
        view,
        !context.group.transform_persistence().is_null(),
    );
    crate::fixed_lighting::snapshot_fixed_material_opacity(&mut snapshot, aspects);
    Ok(snapshot)
}

fn primitive_has_lighting_colors(primitive: &PrimitiveSubmission) -> bool {
    primitive
        .attributes
        .attribute_data(Graphic3dTypeOfAttribute::Color)
        .is_some()
}

pub(crate) fn resolved_model(
    aspect: &dyn Graphic3dAspectsSource,
    view: Graphic3dTypeOfShadingModel,
) -> Graphic3dTypeOfShadingModel {
    let model = if aspect.shading_model() == Graphic3dTypeOfShadingModel::Default {
        view
    } else {
        aspect.shading_model()
    };
    pbr_shading_fallback(model, is_pbr_model(view))
}
/// Whether the model selects metallic-roughness shading; `is_pbr_model(Model::PbrFacet)`.
pub(crate) fn is_pbr_model(model: Graphic3dTypeOfShadingModel) -> bool {
    matches!(
        model,
        Graphic3dTypeOfShadingModel::Pbr | Graphic3dTypeOfShadingModel::PbrFacet
    )
}
/// GlShaderManager::PBRShadingModelFallback: PBR aspects shade with Phong unless the view's
/// own shading model is PBR (`IsPbrAllowed`, which also creates the view's PBR environment).
fn pbr_shading_fallback(
    model: Graphic3dTypeOfShadingModel,
    allowed: bool,
) -> Graphic3dTypeOfShadingModel {
    match model {
        Graphic3dTypeOfShadingModel::Pbr if !allowed => Graphic3dTypeOfShadingModel::Phong,
        Graphic3dTypeOfShadingModel::PbrFacet if !allowed => {
            Graphic3dTypeOfShadingModel::PhongFacet
        }
        _ => model,
    }
}
pub(crate) fn surface_algorithm(
    model: Graphic3dTypeOfShadingModel,
    normals: bool,
) -> Result<RenderShading, RenderError> {
    use Graphic3dTypeOfShadingModel as Model;
    match model {
        Model::PhongFacet | Model::PbrFacet => Ok(RenderShading::PhongFacet),
        Model::Gouraud | Model::Phong | Model::Pbr if !normals => Ok(RenderShading::PhongFacet),
        Model::Gouraud => Ok(RenderShading::Gouraud),
        Model::Phong | Model::Pbr => Ok(RenderShading::Phong),
        _ => Err(rejected(
            model,
            "PhongFacet, Gouraud, Phong, Pbr or PbrFacet surface lighting",
        )),
    }
}
fn validate_non_surface_lighting(
    model: Graphic3dTypeOfShadingModel,
    normals: bool,
) -> Result<(), RenderError> {
    if normals
        && matches!(
            model,
            Graphic3dTypeOfShadingModel::Gouraud
                | Graphic3dTypeOfShadingModel::Phong
                | Graphic3dTypeOfShadingModel::Pbr
        )
    {
        return Err(rejected(
            (model, normals),
            "unlit non-surface geometry or Gouraud/Phong nodal lighting; PBR remains unsupported",
        ));
    }
    Ok(())
}
pub(crate) fn aspect_material(aspect: &dyn Graphic3dAspectsSource) -> RenderMaterial {
    let (front, back) = surface_material_pair(aspect);
    let mut material = RenderMaterial::default();
    // The target is an sRGB attachment; coefficients stay linear, matching active framebuffer sRGB.
    material.init(
        RenderMaterialColorSpace::Linear,
        &front,
        *aspect.interior_color_rgba().rgb(),
        if aspect.distinguish() { &back } else { &front },
        *aspect.back_interior_color_rgba().rgb(),
    );
    material
}

fn surface_material_pair(
    aspect: &dyn Graphic3dAspectsSource,
) -> (Graphic3dMaterialAspect, Graphic3dMaterialAspect) {
    let mut front = aspect.front_material();
    let mut back = aspect.back_material();
    if !crate::style::surface_uses_blending(aspect) {
        front.set_transparency(0.).expect("valid opacity");
        back.set_transparency(0.).expect("valid opacity");
    }
    (front, back)
}
pub(crate) fn apply_view_lights(
    layers: &mut [RenderLayer],
    view: &dyn Graphic3dCView,
) -> Result<(), RenderError> {
    if crate::fixed_view_lighting::uses_fixed_view_lighting(view) {
        return crate::fixed_view_lighting::apply_fixed_view_lighting(layers, view);
    }
    if !layers
        .iter()
        .flat_map(|layer| &layer.draws)
        .any(|draw| draw.lighting.is_some())
    {
        return Ok(());
    }
    // Headlights use the inverse of the float OrientationMatrixF (WorldViewMatrixInverse).
    let camera_view = crate::view_render_camera::owned_render_camera(view).orientation_matrix_f();
    let inverse = camera_view
        .inverted()
        .map_err(|e| rejected(e, "an invertible camera world-view matrix"))?;
    let shading = view.rendering_params().shading_model;
    let lights = crate::RenderLightSet::for_view(shading, &view.lights(), inverse.data(), None)?;
    for layer in layers.iter_mut() {
        // Every structure of a layer with a local origin sees its lights relative to it
        // (GlLayerList::renderLayer sets it before UpdateLightSourceState).
        let origin = layer.local_origin.map(|origin| origin.coords());
        for lighting in layer
            .draws
            .iter_mut()
            .filter_map(|draw| draw.lighting.as_mut())
        {
            lighting.lights = if lighting.view == camera_view.data() && origin.is_none() {
                lights.clone()
            } else {
                persistent_view_lights(shading, view, lighting.view, origin)?
            };
        }
    }
    Ok(())
}
/// Lights of a transform-persistent structure.
/// GlShaderManager::PushLightSourceState pushes the lights again whenever the world-view
/// state changes, and pushLightSourceState rotates headlights by the current
/// WorldViewMatrixInverse: the structure's persistent world view (e.g. the 2D frame of
/// Graphic3dTMF_2d), not the camera's (bug27739).
/// A layer's local origin (`origin`) also shifts the positional lights that are not headlights.
fn persistent_view_lights(
    shading: Graphic3dTypeOfShadingModel,
    view: &dyn Graphic3dCView,
    world_view: [f32; 16],
    origin: Option<[f64; 3]>,
) -> Result<crate::RenderLightSet, RenderError> {
    let inverse = martensite_tkernel::ncollection::Mat4::<f32>::from_data(world_view)
        .inverted()
        .map_err(|e| rejected(e, "an invertible persistent world-view matrix"))?;
    crate::RenderLightSet::for_view(shading, &view.lights(), inverse.data(), origin)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_surface_fallbacks_and_unsupported_models_are_explicit() {
        use Graphic3dTypeOfShadingModel as M;
        assert_eq!(
            surface_algorithm(M::Gouraud, false).unwrap(),
            RenderShading::PhongFacet
        );
        assert_eq!(
            surface_algorithm(M::Phong, false).unwrap(),
            RenderShading::PhongFacet
        );
        assert_eq!(
            surface_algorithm(M::Gouraud, true).unwrap(),
            RenderShading::Gouraud
        );
        assert_eq!(
            surface_algorithm(M::Phong, true).unwrap(),
            RenderShading::Phong
        );
        assert_eq!(
            surface_algorithm(M::Pbr, true).unwrap(),
            RenderShading::Phong
        );
        assert_eq!(
            surface_algorithm(M::Pbr, false).unwrap(),
            RenderShading::PhongFacet
        );
        assert_eq!(
            surface_algorithm(M::PbrFacet, true).unwrap(),
            RenderShading::PhongFacet
        );
        assert!(surface_algorithm(M::Unlit, true).is_err());
        assert_eq!(pbr_shading_fallback(M::Pbr, false), M::Phong);
        assert_eq!(pbr_shading_fallback(M::PbrFacet, false), M::PhongFacet);
        assert_eq!(pbr_shading_fallback(M::Pbr, true), M::Pbr);
        assert!(validate_non_surface_lighting(M::Phong, true).is_err());
        assert!(validate_non_surface_lighting(M::Phong, false).is_ok());
    }
}

#[cfg(test)]
mod opacity_tests {
    use super::*;
    #[test]
    fn opaque_lighting_overrides_transparency_without_mutating_aspects() {
        let aspect = Graphic3dAspects::default();
        let mut source = aspect.front_material();
        source.set_transparency(0.8).unwrap();
        aspect.set_front_material(source);
        assert!((aspect_material(&aspect).common[0].diffuse[3] - 0.2).abs() < 1e-6);
        aspect.set_alpha_mode(Graphic3dAlphaMode::Opaque, 0.5);
        assert_eq!(aspect_material(&aspect).common[0].diffuse[3], 1.);
        assert_eq!(aspect.front_material().transparency(), 0.8);
    }
}
