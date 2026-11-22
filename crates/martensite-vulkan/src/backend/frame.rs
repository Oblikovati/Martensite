//! Frame resources are private and never outlive the device that created them.
#[cfg(test)]
use super::transform_constants::encode_transform;
#[path = "frame_draw_upload.rs"]
mod draw_upload;
#[path = "frame_upload_point.rs"]
mod upload_point;
use draw_upload::upload_draw;
#[path = "frame_setup.rs"]
mod frame_setup;
use super::{
    buffer::DeviceBuffer, geometry_transfer::GeometryTransfers,
    host_upload_pages::FrameUploadPages, image::TargetImage, pipeline::DrawPipeline,
    pipeline_variants::RasterPipelines, runtime::VulkanRuntime,
};
use crate::{FrameCapture, PrimitiveDraw, RenderError, RenderFrame, error::rejected};
use ash::vk;
use std::sync::Arc;

struct FrameDrawUploads<'a> {
    geometry: GeometryTransfers,
    storage: FrameUploadPages<'a>,
}
pub(super) struct UploadedDraw {
    pub window_depth_expected: bool,
    pub window_depth_status: Option<super::window_depth_status::WindowDepthStatus>,
    #[cfg(feature = "vertex-output-diagnostics")]
    pub vertex_output: Option<super::vertex_output::VertexOutputResources>,
    #[cfg(feature = "command-state-diagnostics")]
    pub geometry_arguments: super::geometry_command_diagnostic::GeometryArguments,
    pub native_alpha: Option<Arc<super::oit_descriptor::OitPassDescriptors>>,
    pub oit_alpha: Vec<(
        vk::DescriptorSet,
        Arc<super::oit_descriptor::OitPassDescriptors>,
    )>,
    pub sample_alpha_cutoff: bool,
    pub oit: Option<super::oit_upload::OitUploadedVariants>,
    pub vertices: Arc<DeviceBuffer>,
    pub indices: Arc<DeviceBuffer>,
    pub count: u32,
    pub instances: u32,
    pub point_viewports: Option<[vk::Viewport; 2]>,
    pub transform: Vec<u8>,
    pub pipeline: Arc<DrawPipeline>,
    pub line: DrawLine,
    pub presentation: DrawPresentation,
    pub bindings: DrawBindings,
    /// The registered user program replacing the built-in pipeline (`user_program_draw`).
    pub user_program: Option<super::user_program_draw::UploadedUserProgram>,
}
pub(super) struct DrawLine {
    pub width: Option<f32>,
    pub stipple: Option<(u32, u16)>,
    /// Replicas of a line wider than the device range; see `crate::wide_line`.
    pub wide: Option<WideLineDraw>,
}
/// The x-major index count and the GL replica offsets of an emulated wide line.
pub(super) struct WideLineDraw {
    pub x_major: u32,
    pub offsets: Vec<f32>,
}
pub(super) struct DrawPresentation {
    pub annotation: bool,
    /// Dimension label rectangle: stencil writes only, depth test off.
    pub stencil_rect: bool,
    pub text_shadow_bias: bool,
    pub text_logic_xor: bool,
    pub encoded_surface: bool,
    pub polygon_offset: Option<crate::RenderPolygonOffset>,
    pub polygon: bool,
    pub polygon_edge: bool,
    pub surface_blend: bool,
    pub capping: crate::RenderCappingPass,
    pub mirrored: bool,
    pub face_culling: crate::RenderFaceCulling,
}
pub(super) struct DrawBindings {
    pub clipping: Option<super::storage_descriptor::UploadedStorage>,
    pub lighting: Option<Arc<super::storage_descriptor::UploadedStorage>>,
    pub sprite: Option<super::storage_descriptor::UploadedStorage>,
    pub surface_image: Option<Arc<super::surface_image_descriptor::SurfaceImageBinding>>,
    pub pbr_environment: Option<Arc<super::pbr_environment::PbrEnvironment>>,
    pub material_maps: Option<Arc<super::material_map_descriptor::MaterialMapBinding>>,
    pub sphere_map: Option<super::storage_descriptor::UploadedStorage>,
    pub shadow_maps: Option<Arc<super::shadow_maps::ShadowMapSet>>,
}
pub(super) struct FrameResources {
    attachment_coordinates: super::attachment_coordinates::AttachmentCoordinates,
    pub stencil: crate::stencil_plan::StencilPlan,
    pub stencil_clear_value: u8,
    pub fragment_alpha: super::fragment_alpha_plan::FragmentAlphaPlan,
    pub multisample: Option<Arc<super::multisample_targets::MultisampleTargets>>,
    pub oit: Option<super::oit_frame::OitFramePass>,
    /// The logical frame extent: viewports, captures and shader frame sizes.
    pub extent: vk::Extent2D,
    /// The attachment extent: images, render areas and copies (see `leading_rows`).
    pub attachment_extent: vk::Extent2D,
    pub color: Arc<TargetImage>,
    pub depth: Arc<TargetImage>,
    pub(super) immediate_source: Option<(usize, super::immediate_main_record::MainLayerImages)>,
    pub(super) main_publication: Option<super::immediate_main_record::MainLayerTicket>,
    pub(super) main_snapshot: Option<super::immediate_main_record::MainLayerSnapshot>,
    pub color_read: DeviceBuffer,
    pub depth_read: DeviceBuffer,
    pub draws: Vec<UploadedDraw>,
    pub(super) first_draw: usize,
    pub(super) geometry_transfers: GeometryTransfers,
    /// The shader grid draw, attached after allocation when the frame has one.
    pub grid: Option<super::shader_grid_pass::GridFrameDraw>,
    /// The ray-tracing pass, attached after allocation when the frame ray-traces.
    pub raytrace: Option<super::raytrace_pass::RaytraceFrameDraw>,
    /// The cubemap or corner gradient background, recorded before every layer
    /// (GlView::drawBackground).
    pub background: Option<FrameBackgroundDraw>,
    /// The subview copies a presented frame applies before its window blit.
    pub subview_upload: Option<super::subview_upload::SubviewUpload>,
}
/// The background a frame records as its first, clearing rendering scope.
pub(super) enum FrameBackgroundDraw {
    Cube(super::environment_cube::background::BackgroundFrameDraw),
    ColoredQuad(super::colored_quad_pass::ColoredQuadFrameDraw),
    StereoPair(super::stereo_pair_pass::StereoPairFrameDraw),
}
impl FrameResources {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        frame: &RenderFrame,
        pipeline: &RasterPipelines,
        initial_alpha: crate::legacy_fragment_alpha::LegacyFragmentAlpha,
        initial_stencil: crate::RenderStencilState,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<Self, RenderError> {
        let setup = frame_setup::FrameSetup {
            runtime,
            frame,
            pipeline,
            retirements,
        };
        let plan = setup.prepare(initial_alpha, initial_stencil)?;
        let uploads = setup.upload(&plan)?;
        setup.allocate(plan, uploads)
    }
    pub(super) fn validate_completion(&self) -> Result<(), RenderError> {
        super::window_depth_status::validate(self)
    }
    pub(super) fn scene_empty(&self) -> bool {
        self.first_draw == 0 && self.draws.is_empty()
    }
    pub(super) fn attachment_bytes(&self) -> u64 {
        self.color.allocation_bytes()
            + self.depth.allocation_bytes()
            + self
                .immediate_source
                .as_ref()
                .map_or(0, |(_, main)| main.allocation_bytes())
            + self
                .main_snapshot
                .as_ref()
                .map_or(0, |snapshot| snapshot.images.allocation_bytes())
            + self
                .multisample
                .as_ref()
                .map_or(0, |targets| targets.allocation_bytes())
            + self
                .oit
                .as_ref()
                .map_or(0, |pass| pass.targets.allocation_bytes())
    }
    pub fn capture(&self) -> Result<FrameCapture, RenderError> {
        self.validate_completion()?;
        if let Some(raytrace) = &self.raytrace {
            raytrace.publish_feedback()?;
        }
        let capture = self.read_logical_capture()?;
        #[cfg(feature = "command-state-diagnostics")]
        self.observe_readback(&capture);
        Ok(capture)
    }
    fn read_logical_capture(&self) -> Result<FrameCapture, RenderError> {
        let size =
            self.attachment_extent.width as usize * self.attachment_extent.height as usize * 4;
        let rgba = self.logical_capture_rows(self.color_read.read(size)?)?;
        let depth = self.capture_depth(size)?;
        Ok(FrameCapture {
            width: self.extent.width,
            height: self.extent.height,
            rgba,
            depth,
        })
    }
    #[cfg(feature = "command-state-diagnostics")]
    fn observe_readback(&self, capture: &FrameCapture) {
        use crate::scene::replay_diagnostic::{self, ReadbackArguments};
        let [color, depth] = self.readback_image_handles();
        let arguments = ReadbackArguments {
            color,
            depth,
            formats: [self.color.format.as_raw(), self.depth.format.as_raw()],
            samples: [self.color.samples.as_raw(), self.depth.samples.as_raw()],
            attachment_extent: [self.attachment_extent.width, self.attachment_extent.height],
            leading_rows: self.attachment_coordinates.leading_rows(self.extent.height),
            coordinates: self.attachment_coordinates,
        };
        replay_diagnostic::readback(capture, arguments);
    }
    #[cfg(feature = "command-state-diagnostics")]
    fn readback_image_handles(&self) -> [[u64; 3]; 2] {
        use ash::vk::Handle;
        [
            [
                self.color.handle.as_raw(),
                self.color.view.as_raw(),
                self.color_read.handle.as_raw(),
            ],
            [
                self.depth.handle.as_raw(),
                self.depth.view.as_raw(),
                self.depth_read.handle.as_raw(),
            ],
        ]
    }
    fn capture_depth(&self, size: usize) -> Result<Vec<f32>, RenderError> {
        let bytes = self.logical_capture_rows(self.depth_read.read(size)?)?;
        Ok(super::depth_equivalence::depth_plane(
            self.depth.format,
            bytes.as_chunks::<4>().0,
        ))
    }
    /// Crop the leading attachment rows, then orient the logical rows top-left.
    fn logical_capture_rows(&self, bytes: Vec<u8>) -> Result<Vec<u8>, RenderError> {
        let coordinates = self.attachment_coordinates;
        let bytes = coordinates.logical_rows(bytes, self.extent);
        coordinates.capture_rows(bytes, self.extent.width)
    }
    pub fn render_color(&self) -> &TargetImage {
        self.multisample
            .as_ref()
            .map_or(&self.color, |targets| &targets.color)
    }
    pub fn render_depth(&self) -> &TargetImage {
        self.multisample
            .as_ref()
            .map_or(&self.depth, |targets| &targets.depth)
    }
}
fn check_extent(runtime: &VulkanRuntime, frame: &RenderFrame) -> Result<(), RenderError> {
    if frame.width > runtime.limits.max_framebuffer_width
        || frame.height > runtime.limits.max_framebuffer_height
        || frame.width > runtime.limits.max_image_dimension2_d
        || frame.height > runtime.limits.max_image_dimension2_d
    {
        return Err(rejected(
            [frame.width, frame.height],
            "extent within device image and framebuffer limits",
        ));
    }
    Ok(())
}

pub(super) fn encode_vertices(draw: &PrimitiveDraw) -> Vec<u8> {
    encode_vertex_sources(draw, (0..draw.vertices.len()).map(|index| (index, index)))
}

fn encode_vertex_patch(
    draw: &PrimitiveDraw,
    vertices: std::ops::Range<usize>,
    full_size: u64,
) -> Option<(u64, Vec<u8>)> {
    let stride = VertexEncoding::from_draw(draw).stride();
    if vertices.end > draw.vertices.len() || (draw.vertices.len() * stride) as u64 != full_size {
        return None;
    }
    let offset = (vertices.start * stride) as u64;
    let bytes = encode_vertex_sources(draw, vertices.map(|index| (index, index)));
    Some((offset, bytes))
}

#[cfg(test)]
mod fixed_pack_tests {
    use super::*;
    #[test]
    fn fixed_provoking_position_keeps_homogeneous_weight_and_full_stride() {
        let vertex = crate::RenderVertex::homogeneous([1., 2., 3., 4.], [0.25, 0.5, 0.75, 1.]);
        let mut draw =
            PrimitiveDraw::new(vec![vertex; 3], vec![0, 1, 2], crate::identity_clip()).unwrap();
        let mut lighting = crate::RenderLighting::new(
            vec![[0., 0., 1.]; 3],
            crate::RenderMaterial::default(),
            crate::identity_clip(),
            crate::identity_clip(),
            false,
            crate::RenderShading::Phong,
        )
        .unwrap();
        lighting.prepare_fixed_lighting().unwrap();
        draw.lighting = Some(lighting);
        let bytes = encode_vertices(&draw);
        assert_eq!(bytes.len(), 3 * 88);
        assert_eq!(f32::from_ne_bytes(bytes[56..60].try_into().unwrap()), 4.);
        assert_eq!(f32::from_ne_bytes(bytes[72..76].try_into().unwrap()), 0.25);
    }
}
fn encode_vertex_sources(
    draw: &PrimitiveDraw,
    sources: impl ExactSizeIterator<Item = (usize, usize)>,
) -> Vec<u8> {
    let layout = VertexEncoding::from_draw(draw);
    let mut bytes = Vec::with_capacity(sources.len() * layout.stride());
    for (index, source) in sources {
        layout.append(&mut bytes, &draw.vertices, index, source);
    }
    bytes
}

struct VertexEncoding<'a> {
    text: Option<&'a crate::TexturedStyle>,
    surface: Option<SurfaceCoordinates<'a>>,
    sphere_normals: Option<&'a [[f32; 3]]>,
    lighting: Option<&'a crate::RenderLighting>,
    xor: bool,
    font_replace: bool,
    fixed_light: bool,
}

enum SurfaceCoordinates<'a> {
    Array(&'a [[f32; 4]]),
    Constant([f32; 4]),
}

fn geometry_key(
    runtime: &VulkanRuntime,
    draw: &PrimitiveDraw,
) -> Option<super::geometry_residency::GeometryKey> {
    use super::geometry_residency::{GeometryEncoding, GeometryKey};
    if cfg!(any(
        feature = "vertex-output-diagnostics",
        feature = "command-state-diagnostics"
    )) || draw.vertices.is_empty()
    {
        return None;
    }
    let layout = VertexEncoding::from_draw(draw);
    // Live formatters and injected sources build one-frame geometry. Publishing a device lease
    // for those streams would add cache traffic and retain buffers that cannot be reused.
    if layout.text.is_some_and(|style| !style.retained_geometry) {
        return None;
    }
    let surface_coordinates = match layout.surface {
        Some(SurfaceCoordinates::Constant(coordinates)) => Some(coordinates.map(f32::to_bits)),
        _ => None,
    };
    Some(GeometryKey::new(
        draw,
        GeometryEncoding {
            surface_coordinates,
            surface_uv: matches!(layout.surface, Some(SurfaceCoordinates::Array(_))),
            sphere_normals: layout.sphere_normals.is_some(),
            text_uv: layout.text.is_some(),
            font_replace: layout.font_replace,
            flat_text: layout.text.is_some()
                && runtime.text_interpolation
                    == crate::RenderTextInterpolationBehavior::ProjectedGrid12FlatCornersV4,
            topology: draw.topology() as u8,
            has_lighting: layout.lighting.is_some(),
            fixed_lighting: layout.fixed_light,
            provoking_vertices: super::fixed_vertex_sources::uses_fixed_flat_vertices(draw),
            fixed_grid_points: draw.fixed_grid_point,
            xor_color: layout.xor,
        },
    ))
}

impl<'a> VertexEncoding<'a> {
    fn from_draw(draw: &'a PrimitiveDraw) -> Self {
        let sphere_normals = draw
            .surface_texture
            .as_ref()
            .and_then(|style| style.sphere_map.as_ref())
            .filter(|_| super::surface_texture_pipeline::draw_uses_sphere_map(draw))
            .map(|transform| &*transform.normals);
        let lighting = draw.lighting.as_ref();
        Self {
            text: draw.textured_style.as_ref(),
            surface: surface_coordinates(draw),
            sphere_normals,
            lighting,
            xor: draw.text_logic_xor && !draw.compatibility_color_target,
            font_replace: draw.compatibility_color_target && draw.textured_style.is_some(),
            fixed_light: lighting.is_some_and(crate::RenderLighting::is_fixed_lighting),
        }
    }

    fn stride(&self) -> usize {
        32 + 8 * usize::from(self.text.is_some())
            + 16 * usize::from(self.surface.is_some())
            + 12 * usize::from(self.sphere_normals.is_some())
            + 12 * usize::from(self.lighting.is_some())
            + 44 * usize::from(self.fixed_light)
    }

    fn append(
        &self,
        bytes: &mut Vec<u8>,
        vertices: &[crate::RenderVertex],
        i: usize,
        source: usize,
    ) {
        let header = encoded_vertex_header(&vertices[i], self.xor, self.font_replace);
        bytes.extend_from_slice(header.as_flattened());
        if let Some(style) = self.text {
            append_f32s(bytes, style.uv[i]);
        }
        if let Some(surface) = &self.surface {
            append_f32s(bytes, surface.at(i));
        }
        if let Some(normals) = self.sphere_normals {
            append_f32s(bytes, normals[i]);
        }
        if let Some(lighting) = self.lighting {
            append_f32s(bytes, lighting.normals[i]);
            if self.fixed_light {
                append_fixed_light(bytes, vertices[source], lighting.normals[source]);
            }
        }
    }
}

fn surface_coordinates(draw: &PrimitiveDraw) -> Option<SurfaceCoordinates<'_>> {
    if draw.surface_texture_binding_only {
        return None;
    }
    if let Some(style) = &draw.surface_texture {
        return Some(SurfaceCoordinates::Array(&style.uv));
    }
    let uses_placeholder = draw.surface_cutoff.is_some()
        || draw.surface_texture_unavailable.is_some()
        || draw.mesh_edges.is_some()
        || draw.unlit_back_color.is_some();
    uses_placeholder
        .then(|| SurfaceCoordinates::Constant(draw.unlit_back_color.unwrap_or([0., 0., 0., 1.])))
}

impl SurfaceCoordinates<'_> {
    fn at(&self, index: usize) -> [f32; 4] {
        match self {
            Self::Array(values) => values[index],
            Self::Constant(value) => *value,
        }
    }
}

fn append_fixed_light(bytes: &mut Vec<u8>, vertex: crate::RenderVertex, normal: [f32; 3]) {
    append_f32s(bytes, vertex.position);
    append_f32s(bytes, [vertex.weight]);
    append_f32s(bytes, normal);
    append_f32s(bytes, vertex.color);
}

fn append_f32s<const N: usize>(bytes: &mut Vec<u8>, values: [f32; N]) {
    bytes.extend_from_slice(values.map(f32::to_ne_bytes).as_flattened());
}

fn encoded_vertex_header(
    vertex: &crate::RenderVertex,
    xor: bool,
    font_replace: bool,
) -> [[u8; 4]; 8] {
    // Native framebuffer conversion precedes XOR; public submissions remain linear RGBA.
    let mut color = crate::text_logic::text_target_rgba(xor, vertex.color);
    // Font textures use ALPHA/REPLACE, which ignores vertex alpha.
    if font_replace {
        color[3] = 1.;
    }
    let [x, y, z] = vertex.position;
    let [r, g, b, a] = color;
    [x, y, z, vertex.weight, r, g, b, a].map(f32::to_ne_bytes)
}

#[cfg(test)]
#[path = "frame_tests.rs"]
mod xor_tests;
