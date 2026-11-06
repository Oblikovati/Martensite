//! User programs on retained surfaces (GlAspects::ShaderProgram): a program-bearing
//! aspect draws its faces with the precompiled SPIR-V registered in the driver's
//! `RenderUserProgramLibrary`, its default uniform block filled per draw from the scene state
//! the native GlShaderManager pushes (matrices, lights, material, colour, clip planes) and
//! from the program variables. GLSL source without registered SPIR-V is refused with
//! `RenderUserProgramFailure::GlslSourceUnsupported` (user decision 2026-10-28).
use crate::text_submission::TextSceneContext;
use crate::user_program::{
    RenderUserProgramLibrary, UserProgramDraw, UserProgramModules, UserUniformScene,
    program_variables,
};
use crate::{PrimitiveDraw, PrimitiveTopology, RenderError, error::rejected};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;

/// The user-program library of the structure's driver, or an empty one without a Martensite
/// driver; `driver_user_programs(&structure)`.
pub(crate) fn driver_user_programs(
    structure: &Handle<dyn Graphic3dStructureSource>,
) -> Arc<RenderUserProgramLibrary> {
    let driver = structure
        .structure_manager()
        .graphic_driver()
        .cast::<crate::Graphic3dRenderDriver>();
    driver
        .get()
        .map_or_else(Arc::default, |driver| driver.user_programs().clone())
}

/// The registered modules of the aspect's program, `None` without a program, or the typed
/// refusal of an unregistered one; `resolve_user_program(&library, &aspects)?`.
pub(crate) fn resolve_user_program(
    library: &RenderUserProgramLibrary,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
) -> Result<Option<Arc<UserProgramModules>>, RenderError> {
    let program = aspects.shader_program();
    let Some(program) = program.get() else {
        return Ok(None);
    };
    library.modules(program).map(Some)
}

/// Attach the program to every triangle draw of `draws` (edges and outlines keep their
/// built-in programs, as GlPrimitiveArray draws them with the edge aspect);
/// `apply_user_program(&mut draws, modules, primitive, &aspects, context)?`.
pub(crate) fn apply_user_program(
    draws: &mut [PrimitiveDraw],
    modules: Option<Arc<UserProgramModules>>,
    primitive: &crate::primitive::PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &TextSceneContext<'_>,
) -> Result<(), RenderError> {
    let Some(modules) = modules else {
        return Ok(());
    };
    let matrices = scene_matrices(context)?;
    let program = aspects.shader_program();
    let program = program.get().expect("resolved user program");
    let tex_coords = primitive_tex_coords(primitive)?;
    for draw in draws.iter_mut() {
        if draw.topology() != PrimitiveTopology::Triangles {
            continue;
        }
        let scene = draw_scene(draw, matrices, aspects, program);
        let mut user = UserProgramDraw::new(modules.clone(), scene);
        user.tex_coords = draw_tex_coords(draw, &tex_coords)?;
        user.broadcasts_color = crate::user_program::broadcasts_frag_color(program);
        draw.user_program = Some(Arc::new(user));
    }
    Ok(())
}

/// The primitive's texture coordinate array, or none without one.
fn primitive_tex_coords(
    primitive: &crate::primitive::PrimitiveSubmission,
) -> Result<Vec<[f32; 4]>, RenderError> {
    let attributes = &*primitive.attributes;
    if attributes
        .attribute_data(Graphic3dTypeOfAttribute::Uv)
        .is_none()
    {
        return Ok(Vec::new());
    }
    crate::surface_texture_uv::surface_input_uv(primitive)
}

/// `coordinates` for `draw`, whose vertices are the primitive's vertices.
fn draw_tex_coords(
    draw: &PrimitiveDraw,
    coordinates: &[[f32; 4]],
) -> Result<Vec<[f32; 4]>, RenderError> {
    if !coordinates.is_empty() && coordinates.len() != draw.vertices.len() {
        return Err(rejected(
            (coordinates.len(), draw.vertices.len()),
            "one primitive texture coordinate per user-program vertex",
        ));
    }
    Ok(coordinates.to_vec())
}

/// ModelWorld, WorldView and Projection of the group. Under transform persistence the group loads
/// the persistent model-world, world-view and projection into the context states before the
/// program reads them (GlStructure::Render, Graphic3dTransformPers::Apply); witness
/// tests/user_program_persistence_native.rs.
fn scene_matrices(context: &TextSceneContext<'_>) -> Result<[[f32; 16]; 3], RenderError> {
    let (model, world_view) = crate::persistent_transform::lighting_matrices(context)?;
    let persistent =
        !context.structure.trsf_pers.is_null() || !context.group.transform_persistence().is_null();
    let projection = match persistent {
        true => crate::persistent_transform::group_projection_only(context)?,
        false => context.camera.projection_matrix_f().data(),
    };
    Ok([model, world_view, projection])
}

fn draw_scene(
    draw: &PrimitiveDraw,
    [model_world, world_view, projection]: [[f32; 16]; 3],
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    program: &Graphic3dShaderProgram,
) -> UserUniformScene {
    let color = aspects.interior_color_rgba();
    let rgb = color.rgb();
    let mut scene = UserUniformScene {
        model_world,
        world_view,
        projection,
        max_lights: program.nb_lights_max().max(0) as usize,
        color: [
            rgb.red() as f32,
            rgb.green() as f32,
            rgb.blue() as f32,
            color.alpha(),
        ],
        clip_planes: draw.clipping.as_ref().map_or_else(Vec::new, clip_planes),
        variables: program_variables(program),
        texture_trsf: texture_matrix(draw, aspects),
        ..Default::default()
    };
    if let Some(lighting) = &draw.lighting {
        scene.material = material_rows(&lighting.material);
    }
    scene
}

/// occTexTrsf2d as GlContext::SetTextureMatrix packs it for the first texture of the set
/// (GlPrimitiveArray::Render): (-translation, scale) and the sine and cosine of
/// -rotation, with V flipped (`caps->isTopDownTextureUV` false) for a top-down image; zero
/// when the draw has no texture. Example: `texture_matrix(&draw, &aspects)[0][3]`.
pub(crate) fn texture_matrix(
    draw: &PrimitiveDraw,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
) -> [[f32; 4]; 2] {
    let first = aspects.texture_set();
    let first = first.get().and_then(|set| set.first().ok());
    let first = first.filter(|texture| !texture.is_null());
    let (Some(style), Some(texture)) = (&draw.surface_texture, first) else {
        return [[0.; 4]; 2];
    };
    pack_texture_matrix(&texture.get_params(), style.image.top_down)
}

/// The two occTexTrsf2d rows of `parameters` for an image of the given row order;
/// `pack_texture_matrix(&parameters, true)[0][3] == -parameters.scale().y()`.
pub(crate) fn pack_texture_matrix(
    parameters: &Graphic3dTextureParams,
    top_down: bool,
) -> [[f32; 4]; 2] {
    let (scale, shift) = (parameters.scale(), parameters.translation());
    let angle = -f64::from(parameters.rotation()) * std::f64::consts::PI / 180.;
    let mut packed = [
        [-shift.x(), -shift.y(), scale.x(), scale.y()],
        [
            martensite_cmath::sin(angle) as f32,
            martensite_cmath::cos(angle) as f32,
            0.,
            0.,
        ],
    ];
    if top_down {
        packed[0][1] = -shift.y() + 1. / scale.y();
        packed[0][3] = -scale.y();
    }
    packed
}

/// occCommonMaterial: front then back diffuse, emission, specular+shininess, ambient.
pub(crate) fn material_rows(material: &crate::RenderMaterial) -> [[f32; 4]; 8] {
    let mut rows = [[0.; 4]; 8];
    for (face, common) in material.common.iter().enumerate() {
        let face_rows = [
            common.diffuse,
            common.emission,
            common.specular_shininess,
            common.ambient,
        ];
        rows[4 * face..4 * face + 4].copy_from_slice(&face_rows);
    }
    rows
}

/// Every plane of every chain with the native chain value: the planes left in its chain.
fn clip_planes(clipping: &crate::RenderClipping) -> Vec<([f32; 4], i32)> {
    let chain_planes = clipping.chains().iter().flat_map(|chain| {
        let length = chain.len();
        chain
            .iter()
            .enumerate()
            .map(move |(index, plane)| (*plane, (length - index) as i32))
    });
    chain_planes.collect()
}

/// GlWorkspace::ApplyAspects under a highlight style: the program reads the highlighted
/// aspect, so occColor is the highlight colour and occCommonMaterial the recoloured material;
/// `highlight_user_program(&mut draw, rgba)`.
pub(crate) fn highlight_user_program(draw: &mut PrimitiveDraw, rgba: [f32; 4]) {
    let Some(user) = &draw.user_program else {
        return;
    };
    let mut highlighted = UserProgramDraw::clone(user);
    highlighted.scene.color = rgba;
    if let Some(lighting) = &draw.lighting {
        highlighted.scene.material = material_rows(&lighting.material);
    }
    draw.user_program = Some(Arc::new(highlighted));
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::ncollection::Vec2;
    #[test]
    fn texture_matrix_packs_translation_scale_rotation_and_flips_top_down_rows() {
        let parameters = Graphic3dTextureParams::default();
        assert_eq!(
            pack_texture_matrix(&parameters, false),
            [[-0., -0., 1., 1.], [-0., 1., 0., 0.]]
        );
        parameters.set_translation(Vec2::new(0.25, 0.5));
        parameters.set_scale(Vec2::new(2., 4.));
        parameters.set_rotation(90.);
        let [first, second] = pack_texture_matrix(&parameters, true);
        assert_eq!(first, [-0.25, -0.5 + 0.25, 2., -4.]);
        let angle = -90_f64.to_radians();
        assert_eq!(
            second,
            [
                martensite_cmath::sin(angle) as f32,
                martensite_cmath::cos(angle) as f32,
                0.,
                0.
            ]
        );
    }
}
