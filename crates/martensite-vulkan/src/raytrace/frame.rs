//! The ray-tracing part of one view frame (GlView::render with Graphic3dRM_RAYTRACING):
//! the scene buffers, materials, lights and uniforms of the pass that follows the raster passes
//! of the ray-traced layers.
use super::lights::{RaytraceLights, raytrace_lights};
use super::material::RaytraceMaterial;
use super::scene::collect_scene;
use super::scene_buffers::{RaytraceSceneBuffers, build};
use super::uniforms::{RaytraceCamera, RaytraceUniforms};
use crate::{RenderError, RenderMaterialColorSpace};
use martensite_tkernel::quantity::Color;
use martensite_tkservice::aspect::AspectGradientFillMethod;
use martensite_tkservice::graphic3d::*;

/// `THE_DEFAULT_STACK_SIZE` of GlView.
const DEFAULT_STACK_SIZE: i32 = 10;

/// The ray-tracing pass of a frame.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RaytraceFrame {
    /// The frame layer index the pass precedes: after the last ray-traced layer.
    pub span: usize,
    pub buffers: RaytraceSceneBuffers,
    pub materials: Vec<RaytraceMaterial>,
    /// The sampler array of textured materials (`myRaytraceGeometry.myTextures`).
    pub textures: super::RaytraceTextures,
    /// The environment map or cube map the frame samples.
    pub environment: super::environment::RaytraceEnvironment,
    pub lights: RaytraceLights,
    pub uniforms: RaytraceUniforms,
    /// The specialization constants of the generateShaderPrefix entries.
    pub constants: RaytraceConstants,
    /// `IsAntialiasingEnabled`: the image is smoothed by three FSAA passes.
    pub antialiasing: bool,
    /// Global illumination: the path-traced frame and its output parameters.
    pub path_tracing: Option<super::pathtrace::PathtraceFrame>,
    /// runPathtrace's `aFBOIdx`: 1 for the right eye (Projection_MonoRightEye), else 0. Each
    /// eye keeps its own accumulation, smoothing and adaptive images.
    pub eye_targets: usize,
    /// The structure check rebuilt the scene this frame (uploadRaytraceData restarts the
    /// accumulation), even when the rebuilt buffers equal the last ones.
    pub rebuilt: bool,
    /// The camera's change counters (Graphic3dWorldViewProjState): GlView::render restarts
    /// the accumulation when they differ from the last frame's, even for equal matrices.
    pub camera_state: Graphic3dWorldViewProjState,
}

/// `STACK_SIZE`, `NB_BOUNCES` and the boolean prefix defines.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct RaytraceConstants {
    pub stack_size: i32,
    pub bounces: i32,
    pub zero_to_one_depth: bool,
    pub transparent_shadows: bool,
    /// TWO_SIDED_BXDF, TONE_MAPPING_FILMIC and IGNORE_NORMAL_MAP of path tracing.
    pub two_sided_bxdf: bool,
    pub filmic: bool,
    pub ignore_normal_map: bool,
    /// DEPTH_OF_FIELD: a perspective path-traced view generates its rays through the lens
    /// (initRaytraceResources enables it for every perspective camera with global
    /// illumination, whatever the aperture).
    pub depth_of_field: bool,
    /// USE_TEXTURES: some material is textured (`UseBindlessTextures`); selects the programs
    /// with the sampler array.
    pub textures: bool,
    /// BACKGROUND_CUBEMAP: the view has a cubemap background (`CubemapForBack`); the
    /// environment is the cube map at binding 13 instead of the 2D map.
    pub cubemap_background: bool,
}

/// What the frame takes from the device and the view's frame environment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct RaytraceFrameInputs {
    /// The device's HasRayTracingTextures: materials take their textures.
    pub textured: bool,
    /// CubemapForBack: `[uYCoeff, uZCoeff]` of the view's cube map and `uEnvMapForBack` (the
    /// background type is still the cube map), when a cubemap background was ever set.
    pub cube: Option<([i32; 2], bool)>,
}

/// The ray-tracing pass of `view`, or `None` when the view rasterizes: another method, or a
/// scene without ray-traced triangles (`myIsRaytraceDataValid` false, OpenGL fallback).
/// `bvh` keeps the bottom-level trees between frames of one view. Example:
/// `prepare_raytrace(&*view, [256, 256], &mut RaytraceBvhCache::default(), inputs)?`.
pub(crate) fn prepare_raytrace(
    view: &dyn Graphic3dCView,
    extent: [u32; 2],
    bvh: &mut super::RaytraceBvhCache,
    inputs: RaytraceFrameInputs,
) -> Result<Option<RaytraceFrame>, RenderError> {
    let params = view.rendering_params();
    if params.method != Graphic3dRenderingMode::Raytracing {
        return Ok(None);
    }
    let rebuilt = bvh.structure_check.check(view);
    let scene = collect_scene(
        view,
        view.identification(),
        RenderMaterialColorSpace::Linear,
        inputs.textured,
    )?;
    let buffers = build(scene.sets, bvh)?;
    if buffers.transforms.is_empty() {
        return Ok(None);
    }
    let camera = crate::view_render_camera::owned_render_camera(view);
    let orientation = camera.orientation_matrix_f();
    let inverse = orientation.inverted().unwrap_or(orientation);
    let lights = view.lights();
    let lights = raytrace_lights(lights.get(), params.shading_model, &inverse);
    let environment = super::environment::raytrace_environment(view, inputs.cube)?;
    let mut uniforms = frame_uniforms(view, &params, (&camera, extent), (&buffers, &lights));
    uniforms.environment = environment.uniforms;
    // updateRaytraceUniforms: an orthographic or ray-traced view still takes the eye frame of
    // updatePerspCameraPT for an environment shown as the background or a cubemap background.
    if params.ray_tracing.use_environment_map_background || environment.cube {
        let window = [extent[0] as i32, extent[1] as i32];
        uniforms.eye = super::pathtrace::perspective_eye(&camera, window);
    }
    let constants = RaytraceConstants {
        stack_size: DEFAULT_STACK_SIZE.max(buffers.tree_depth),
        bounces: params.ray_tracing.raytracing_depth,
        zero_to_one_depth: camera.is_zero_to_one_depth(),
        transparent_shadows: params.ray_tracing.is_transparent_shadow_enabled,
        two_sided_bxdf: params.ray_tracing.two_sided_bsdf_models,
        filmic: super::pathtrace::is_filmic(&params),
        ignore_normal_map: params.ray_tracing.to_ignore_normal_map_in_ray_tracing,
        depth_of_field: params.ray_tracing.is_global_illumination_enabled
            && !camera.is_orthographic(),
        textures: scene.textures.has_textures(),
        cubemap_background: environment.cube,
    };
    let materials = scene.materials;
    Ok(Some(RaytraceFrame {
        span: 0,
        buffers,
        materials,
        textures: scene.textures,
        environment,
        lights,
        uniforms,
        constants,
        antialiasing: params.ray_tracing.is_antialiasing_enabled,
        path_tracing: super::pathtrace::pathtrace_frame(&params),
        eye_targets: super::pathtrace::eye_targets(camera.projection_type()),
        rebuilt,
        camera_state: camera.world_view_proj_state(),
    }))
}

fn frame_uniforms(
    view: &dyn Graphic3dCView,
    params: &Graphic3dRenderingParams,
    (camera, extent): (&Graphic3dCamera, [u32; 2]),
    (buffers, lights): (&RaytraceSceneBuffers, &RaytraceLights),
) -> RaytraceUniforms {
    let ray_camera = RaytraceCamera::new(
        &camera.orientation_matrix_f(),
        &camera.projection_matrix_f(),
    );
    let (back_top, back_bottom) = background_colors(view, &camera.tile());
    let (min, max) = (buffers.bounds.corner_min(), buffers.bounds.corner_max());
    let largest = |v: &martensite_tkmath::bvh::BvhVec3f| v.x.abs().max(v.y.abs()).max(v.z.abs());
    let size = buffers.bounds.size();
    let modulus = (size.x * size.x + size.y * size.y + size.z * size.z).sqrt();
    RaytraceUniforms {
        camera: ray_camera,
        window: [extent[0] as i32, extent[1] as i32],
        ambient: lights.ambient,
        back_top,
        back_bottom,
        light_count: lights.sources.len() as i32,
        shadows: params.ray_tracing.is_shadow_enabled,
        reflections: params.ray_tracing.is_reflection_enabled,
        scene_radius: 2. * largest(min).max(largest(max)),
        scene_epsilon: 1.0e-6f32.max(1.0e-4 * modulus),
        aperture_radius: params.ray_tracing.camera_aperture_radius,
        focal_plane_distance: params.ray_tracing.camera_focal_plane_dist,
        path_tracing: super::uniforms::PathtraceUniforms {
            max_radiance: params.ray_tracing.radiance_clamping_value,
            blocked_rng: params.ray_tracing.coherent_path_tracing_mode,
            ..Default::default()
        },
        // updatePerspCameraPT: a perspective path-traced view takes the eye frame.
        eye: match params.ray_tracing.is_global_illumination_enabled && !camera.is_orthographic() {
            true => super::pathtrace::perspective_eye(camera, [extent[0] as i32, extent[1] as i32]),
            false => Default::default(),
        },
        environment: Default::default(),
    }
}

/// The gradient part of a valid tile: the bottom moves up by the tile's lower offset and the top
/// lies one tile height above it, in the native float order; `tile_gradient(top, bottom, &tile)`.
fn tile_gradient(
    top: [f32; 4],
    bottom: [f32; 4],
    tile: &Graphic3dCameraTile,
) -> ([f32; 4], [f32; 4]) {
    if !tile.is_valid() {
        return (top, bottom);
    }
    let view = tile.total_size.y() as f32;
    let offset = tile.offset_lower_left().y() as f32 / view;
    let size = tile.tile_size.y() as f32 / view;
    let range: [f32; 4] = std::array::from_fn(|i| top[i] - bottom[i]);
    let low: [f32; 4] = std::array::from_fn(|i| bottom[i] + range[i] * offset);
    (std::array::from_fn(|i| low[i] + range[i] * size), low)
}

fn rgba(color: Color, alpha: f32) -> [f32; 4] {
    let [r, g, b] = color.rgb();
    [r, g, b, alpha]
}

/// `uBackColorTop` and `uBackColorBot`: the gradient colours when a gradient is defined, else
/// the background colour for both (linear, as the target is sRGB). A camera tile takes its own
/// part of the vertical gradient, as GlView::setUniformState interpolates it.
fn background_colors(
    view: &dyn Graphic3dCView,
    tile: &Graphic3dCameraTile,
) -> ([f32; 4], [f32; 4]) {
    let gradient = view.gradient_background();
    if gradient.bg_gradient_fill_method() != AspectGradientFillMethod::None {
        let (top, bottom) = gradient.colors();
        return tile_gradient(rgba(top, 1.), rgba(bottom, 1.), tile);
    }
    let background = view.cview_base().fields.read().bg_color;
    let color = rgba(*background.rgb(), background.alpha());
    (color, color)
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::ncollection::Vec2;
    #[test]
    fn a_camera_tile_takes_its_part_of_the_vertical_gradient() {
        let (top, bottom) = ([1., 0.5, 0., 1.], [0., 0.5, 1., 1.]);
        let whole = Graphic3dCameraTile::default();
        assert_eq!(tile_gradient(top, bottom, &whole), (top, bottom));
        let tile = Graphic3dCameraTile {
            total_size: Vec2::new(100, 200),
            tile_size: Vec2::new(100, 50),
            offset: Vec2::new(0, 100),
            is_top_down: false,
        };
        // Rows 100..150 of 200 from the bottom: from 0.5 to 0.75 of the range.
        let (tile_top, tile_bottom) = tile_gradient(top, bottom, &tile);
        assert_eq!(tile_bottom, [0.5, 0.5, 0.5, 1.]);
        assert_eq!(tile_top, [0.75, 0.5, 0.25, 1.]);
    }
}
