//! The proof that lets an immediate redraw skip the main layers (#594).
//!
//! A retained main image stays valid while its view has had no main-layer notice
//! (main_layer_notices.rs) and the view state that changes without a notice still matches:
//! the extent, the camera, the lights, the rendering parameters, the background colour, the
//! layer selection and the clip planes. Then an immediate redraw builds only the immediate
//! layers and draws them over a copy of the main image, as OCCT's `RedrawImmediate` does.
use crate::{RenderFrame, RenderLayer, RenderLightSet, RenderPolygonOffset};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;

/// What a retained main image was drawn from, beside its draws; kept with the image.
pub(crate) struct MainElision {
    notices: u64,
    inputs: ImmediateViewInputs,
    shells: Vec<MainLayerShell>,
}

/// The depth policy and polygon offset of one main layer span.
#[derive(Clone, Copy)]
struct MainLayerShell {
    depth: (bool, bool, bool),
    offset: RenderPolygonOffset,
}

impl MainElision {
    /// The proof for the main layers `frame.layers[..start]`, drawn after `notices` notices
    /// from the view state `inputs`; `MainElision::capture(notices, inputs, &frame, start)`.
    pub(crate) fn capture(
        notices: u64,
        inputs: ImmediateViewInputs,
        frame: &RenderFrame,
        start: usize,
    ) -> Self {
        let shells = frame.layers[..start]
            .iter()
            .map(|span| MainLayerShell {
                depth: span.depth_policy(),
                offset: span.polygon_offset,
            })
            .collect();
        Self {
            notices,
            inputs,
            shells,
        }
    }

    /// Whether the main image is still what the view would draw; `elision.holds(n, &inputs)`.
    pub(crate) fn holds(&self, notices: u64, inputs: &ImmediateViewInputs) -> bool {
        self.notices == notices && self.inputs == *inputs
    }

    /// The span index of the first immediate layer.
    pub(crate) fn start(&self) -> usize {
        self.shells.len()
    }

    /// Layers without draws in place of the main layers, so the immediate layers keep their
    /// span indices; `elision.shell_layers()`.
    pub(crate) fn shell_layers(&self) -> Vec<RenderLayer> {
        self.shells
            .iter()
            .map(|shell| {
                let (test, write, clear) = shell.depth;
                RenderLayer::new(Vec::new(), test, write, clear).with_polygon_offset(shell.offset)
            })
            .collect()
    }
}

/// The view state that main layers read and that changes without a main-layer notice.
#[derive(PartialEq)]
pub(crate) struct ImmediateViewInputs {
    extent: [u32; 2],
    window: (i32, i32),
    clear: [u32; 4],
    camera: CameraBits,
    lights: Option<RenderLightSet>,
    params: MainRenderingParams,
    backfacing: Graphic3dTypeOfBackfacingModel,
    layer_selection: (Graphic3dZLayerId, bool),
    clipped: bool,
    environment: usize,
}

impl ImmediateViewInputs {
    /// The current state of `view` for a frame of `extent`;
    /// `ImmediateViewInputs::capture(view, extent)`.
    /// Whether everything but the camera and the camera-relative lights matches: a retained
    /// placement frame derives those again on every frame; `inputs.matches_but_camera(&now)`.
    pub(crate) fn matches_but_camera(&self, other: &Self) -> bool {
        self.extent == other.extent
            && self.window == other.window
            && self.clear == other.clear
            && self.params == other.params
            && self.backfacing == other.backfacing
            && self.layer_selection == other.layer_selection
            && self.clipped == other.clipped
            && self.environment == other.environment
    }

    pub(crate) fn capture(view: &crate::Graphic3dRenderView, extent: [u32; 2]) -> Self {
        let camera = crate::view_render_camera::owned_render_camera(view);
        let params = view.rendering_params();
        Self {
            extent,
            window: crate::view_scene::frame_window_size(view, Some(view)),
            clear: crate::view_scene::view_clear_color(view).map(f32::to_bits),
            camera: CameraBits::capture(&camera),
            lights: view_lights(view, &camera, &params),
            params: MainRenderingParams(params),
            backfacing: view.backfacing_model(),
            layer_selection: (view.z_layer_target(), view.z_layer_redraw_mode()),
            clipped: !view.retained.render_clip_planes().is_null(),
            environment: view
                .cview_base()
                .fields
                .read()
                .texture_env_data
                .object_address()
                .addr(),
        }
    }
}

/// The view's lights in world space, as the main layers' lit draws received them.
fn view_lights(
    view: &dyn Graphic3dCView,
    camera: &Graphic3dCamera,
    params: &Graphic3dRenderingParams,
) -> Option<RenderLightSet> {
    let inverse = camera.orientation_matrix_f().inverted().ok()?;
    RenderLightSet::for_view(params.shading_model, &view.lights(), inverse.data(), None).ok()
}

/// The camera's float matrices and projection, which the main draws were transformed with.
#[derive(PartialEq)]
struct CameraBits {
    orientation: [u32; 16],
    projection: [u32; 16],
    kind: Graphic3dCameraProjection,
}

impl CameraBits {
    fn capture(camera: &Handle<Graphic3dCamera>) -> Self {
        let bits =
            |matrix: martensite_tkernel::ncollection::Mat4<f32>| matrix.data().map(f32::to_bits);
        let Some(camera) = camera.get() else {
            return Self {
                orientation: [0; 16],
                projection: [0; 16],
                kind: Graphic3dCameraProjection::Orthographic,
            };
        };
        Self {
            orientation: bits(camera.orientation_matrix_f()),
            projection: bits(camera.projection_matrix_f()),
            kind: camera.projection_type(),
        }
    }
}

/// The rendering parameters compared field by field. The statistics fields only reach the
/// statistics overlay, which every immediate redraw builds again.
struct MainRenderingParams(Graphic3dRenderingParams);

impl PartialEq for MainRenderingParams {
    fn eq(&self, other: &Self) -> bool {
        main_params_bits(&self.0) == main_params_bits(&other.0)
            && self.0.ray_tracing == other.0.ray_tracing
            && self.0.stereo == other.0.stereo
    }
}

/// Every field but the structured ones, as comparable bits. The destructuring names every
/// field, so a new field of the parameters does not compile until it is placed here.
fn main_params_bits(params: &Graphic3dRenderingParams) -> impl PartialEq + use<> {
    let Graphic3dRenderingParams {
        method,
        shading_model,
        transparency_method,
        resolution,
        font_hinting,
        line_feather,
        pbr_env_pow2_size,
        pbr_env_spec_map_nb_levels,
        pbr_env_baking_diff_nb_samples,
        pbr_env_baking_spec_nb_samples,
        pbr_env_baking_probability,
        oit_depth_factor,
        nb_oit_depth_peeling_layers,
        nb_msaa_samples,
        render_resolution_scale,
        shadow_map_resolution,
        shadow_map_bias,
        to_enable_depth_prepass,
        to_enable_alpha_to_coverage,
        ray_tracing: _,
        stereo: _,
        stats: _,
    } = params.clone();
    (
        (
            method,
            shading_model,
            transparency_method,
            resolution,
            font_hinting,
        ),
        [
            line_feather,
            pbr_env_baking_probability,
            oit_depth_factor,
            render_resolution_scale,
            shadow_map_bias,
        ]
        .map(f32::to_bits),
        [
            pbr_env_pow2_size,
            pbr_env_spec_map_nb_levels,
            pbr_env_baking_diff_nb_samples,
            pbr_env_baking_spec_nb_samples,
            nb_oit_depth_peeling_layers,
            nb_msaa_samples,
            shadow_map_resolution,
        ],
        (to_enable_depth_prepass, to_enable_alpha_to_coverage),
    )
}
