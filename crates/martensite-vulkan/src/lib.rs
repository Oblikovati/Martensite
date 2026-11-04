//! Martensite's Vulkan 1.3 renderer with immutable submissions and offscreen captures.
#![cfg_attr(test, allow(clippy::duplicate_mod))] // Unit tests share scene fixtures by path.
#[allow(unsafe_code)]
mod backend;
mod device_close;
pub use device_close::RenderDeviceCloseOutcome;
mod device_failure;
mod portability_failure;
pub use portability_failure::RenderPortabilityFailure;
mod surface_platform;
pub use surface_platform::{RenderSurfacePlatform, RenderSurfacePlatformFailure};
mod driver_gui_surface;
mod error;
pub use device_failure::{RenderDeviceFailure, RenderDeviceFailureClass, RenderDeviceState};
#[cfg(test)]
mod gui_surface_error_tests;
mod gui_surface_outcome;
mod gui_target_stamp;
pub use gui_surface_outcome::{
    RenderGuiFreshPacket, RenderGuiSurfaceCause, RenderGuiSurfaceChange, RenderGuiSurfaceOutcome,
    RenderGuiSurfacePhase, RenderGuiSurfacePolicy,
};
mod render_driver_identity;
pub use backend::{VERIFIED_VALIDATION_LAYER, ValidationLayerVersion, active_validation_layer};
pub use render_driver_identity::RenderDriverIdentity;
mod driver_device_lost;
mod driver_lighting_capability;
mod driver_options;
pub use driver_options::RenderDriverOptions;
pub use error::lighting_selection::RenderLightingSelectionChanged;
#[cfg(feature = "raytrace-input-diagnostics")]
mod raytrace_input_observation;
mod scene;
pub use backend::{VulkanOptions, VulkanRenderer};
pub use error::RenderError;
mod validation_observation;
pub use validation_observation::{
    RenderValidationObservationCause, RenderValidationObservationFailure,
};
mod line_clipping_behavior;
pub use line_clipping_behavior::RenderLineClippingBehavior;
mod line_clipping_failure;
pub use line_clipping_failure::{RenderLineClippingFailure, RenderLineClippingRequirement};
mod world_clipping_behavior;
pub use world_clipping_behavior::RenderWorldClippingBehavior;
mod world_clipping_failure;
pub use world_clipping_failure::{RenderWorldClippingFailure, RenderWorldClippingRequirement};
mod world_clip_interpolation;
pub use world_clip_interpolation::RenderWorldClipInterpolationBehavior;
mod world_clip_interpolation_failure;
pub use world_clip_interpolation_failure::RenderWorldClipInterpolationFailure;
mod line_width_policy;
pub use line_width_policy::RenderLineWidthPolicy;
mod render_pipeline_policies;
pub use render_pipeline_policies::RenderPipelinePolicies;
mod render_raster_policy;
pub use render_raster_policy::RenderRasterPolicy;
mod render_coordinate_convention;
pub use render_coordinate_convention::RenderCoordinateConvention;
mod depth_bias_behavior;
pub use depth_bias_behavior::{RenderDepthBiasBehavior, RenderDepthBiasFailure};
mod render_behavior_configuration;
pub use render_behavior_configuration::RenderBehaviorConfiguration;
mod clipped_depth_constants;
#[cfg(test)]
mod clipped_depth_plane;
#[cfg(test)]
mod clipped_depth_polygon;
mod fixed_modelview_inverse;
mod projection_factors;
pub use projection_factors::{RenderProjectionBehavior, RenderProjectionFactors};
mod text_grid_interpolation;
pub use text_grid_interpolation::{
    RenderTextInterpolationBehavior, RenderTextInterpolationFailure,
};
mod thin_line_coverage;
pub use thin_line_coverage::RenderThinLineCoverage;
mod raster_policy_failure;
mod thin_line;
pub use raster_policy_failure::{RenderRasterCapabilityFailure, RenderRasterPolicyFailure};
mod raster_policy_admission;

pub use scene::{FrameCapture, RenderBackend, RenderFrame, RenderVertex, identity_clip};
mod element_stats;
mod element_upload;
mod frame_stats;
mod frame_stats_chart;
mod frame_stats_overlay;
mod shader_grid;
pub use shader_grid::{RenderGridEcho, RenderGridPixel, RenderShaderGrid, RenderShaderGridBlock};
mod frame_stats_structures;
mod graduated_trihedron;
pub use graduated_trihedron::RenderGraduatedTrihedron;
mod graphic_group;
mod group_flipping;
mod primitive;
mod primitive_ranges;
mod primitive_topology;
pub use frame_stats::{RenderFrameStats, RenderImageMemory};
pub use graphic_group::Graphic3dDrawGroup;

mod style;
mod user_program;
pub use user_program::{
    RenderUserProgramFailure, RenderUserProgramLibrary, SamplerBinding, StageReflection,
    UniformMember, UniformScalar, UniformShape, UserProgramKey, UserProgramModules, UserStage,
    phong_shading_program,
};

mod depth_prepass;
mod render_layers;
mod wide_line;
mod wide_line_columns;
pub use render_layers::{RenderLayer, RenderLayerSpan};

mod capture_image_destination;
#[cfg(test)]
mod capture_image_destination_fixture;
#[cfg(test)]
mod capture_image_destination_tests;
mod capture_image_error;
mod capture_image_layout;
#[cfg(test)]
mod capture_image_rejection_tests;
#[cfg(test)]
mod capture_owned_fixture;
mod device_limits;
mod driver_dump;
mod layer_origin;
mod render_context_queries;
mod render_context_state;
mod render_element;
mod render_element_context;
mod render_element_pass;
mod render_primitive_array;
mod render_view_frame_hooks;
pub use render_context_state::RenderContextState;
pub use render_element::{RenderElement, RenderElementContext, RenderElementFilter};
pub use render_primitive_array::RenderPrimitiveArray;
mod render_matrix_state;
pub use render_matrix_state::RenderMatrixState;
mod render_text;
mod render_texture;
pub use render_text::RenderText;
pub use render_texture::{RenderTexture, RenderTextureAttachment};
pub use render_view_frame_hooks::{RenderFrameStep, RenderViewFrameHooks};
mod driver;
mod driver_backend;
mod driver_limits;
pub use device_limits::RenderDeviceLimits;
mod driver_structure;
mod driver_text;
mod driver_texture_limit;
mod driver_view;
mod view_capture;
mod view_diagnostics;
mod view_shader_grid;
mod view_statistics;
pub use capture_image_error::{
    CaptureImageError, CaptureImageFailure, CaptureImageLayout, CaptureImageOperation,
};
mod background_chart;
mod bound_box_highlight;
mod colored_quad_background;
mod environment_cube_pixels;
mod environment_frame;
mod environment_texture_map;
mod fixed_background;
mod gradient_background;
mod image_background;
mod raytrace;
pub use raytrace::{RaytraceHaltonSampler, RaytraceTileOffset, RaytraceTileSampler};
mod render_scale;
mod shadow_map_dump;
mod shadow_maps;
mod stereo_pair;
mod subview_composition;
pub use shadow_maps::{RenderShadowMap, RenderShadowMaps};
mod silhouette_outline;
#[cfg(test)]
mod view_capture_owned_tests;
mod view_environment;
mod view_forward;
mod view_graduated_trihedron;
mod view_gui_attachment;
mod view_gui_composition;
mod view_gui_layer;
mod view_layer_clear;
mod view_output_modes;
mod view_raytrace;
mod view_scene;
mod view_target;
pub use view_target::RenderFrameBuffer;
mod view_window_lifecycle;
pub use driver::Graphic3dRenderDriver;
mod driver_factory;
pub use driver_factory::{VULKAN_DRIVER_NAME, VulkanDriverFactory};
pub use driver_structure::Graphic3dRenderStructure;
pub use driver_view::{Graphic3dRenderView, RenderRedrawCursor, RenderRedrawFailure};

mod persistent_transform;

mod draw;
pub use draw::{LineStyle, PrimitiveDraw, PrimitiveTopology, TriangleDraw};

mod line_style;

mod highlight_style;

mod point_style;
mod polygon_point_raster;
pub use point_style::PointStyle;
pub use polygon_point_raster::RenderPolygonPointFailure;

mod raster_image;
pub use raster_image::RasterImage;
mod textured_draw;
pub use textured_draw::TexturedStyle;
mod pixel_text;
mod text_layout;
mod text_logic;
pub use pixel_text::{PixelTextView, lower_pixel_text};
pub use text_layout::{GlyphLayout, TextLayoutSource};
mod text_font;
pub use text_font::FreeTypeTextSource;
mod text_atlas;

mod text_background;
mod text_culling;
mod text_drawing;
mod text_submission;
mod text_transform;

mod clipping;
pub use clipping::RenderClipping;
mod clipping_state;
pub use clipping_state::{RenderClippingIterator, RenderClippingState};

mod retained_clipping;

mod capping_order;
mod capping_pass;
pub use capping_pass::RenderCappingPass;

mod face_culling;
pub use face_culling::RenderFaceCulling;

mod capping_geometry;
mod capping_planes;
mod capping_style;
mod capping_texture;
mod mesh_edges;
mod retained_aspect_textures;
mod retained_cap_textures;
mod retained_capping;
mod retained_eye_edge_matrix;
mod retained_mesh_edges;
mod surface_alpha;
pub use mesh_edges::{RenderMeshEdgePipeline, RenderMeshEdges};

mod material;
mod material_components;
pub use material::{RenderMaterial, RenderMaterialColorSpace};
pub use material_components::{RenderMaterialCommon, RenderMaterialPbr};

mod light_snapshot;
pub use light_snapshot::RenderLightSet;

mod phong_interpolation;
pub use phong_interpolation::{RenderPhongInterpolationBehavior, RenderPhongInterpolationFailure};
mod light_dot_behavior;
mod lighting;
pub use light_dot_behavior::RenderLightDotBehavior;
mod lighting_pipeline;
pub use lighting::{RenderLighting, RenderShading};
pub use lighting_pipeline::RenderLightingPipeline;

mod retained_lighting;

mod capping_lighting;
mod capping_mask_aspect;

mod quad_assembly;

mod fixed_flat_sources;
mod fixed_light_snapshot;
mod fixed_lighting;
mod fixed_line_stipple;
mod fixed_marker_environment;
mod fixed_normalization;
mod fixed_surface_environment;
mod fixed_view_lighting;
mod line_guard_band;
mod polygon_offset;
mod retained_hatch;
mod retained_hollow;
pub use polygon_offset::RenderPolygonOffset;

mod point_surface_texture;
mod retained_surface_texture;
mod retained_texture_images;
mod retained_texture_samplers;
mod retained_user_program;
mod surface_material_maps;
mod surface_sphere_texgen;
mod surface_texgen;
mod surface_texture;
mod surface_texture_compressed;
mod surface_texture_lifetime;
mod surface_texture_pixels;
mod surface_texture_uv;
mod texture_coordinate_frame;
mod texture_coordinate_pipeline;
pub use texture_coordinate_pipeline::RenderTextureCoordinatePipeline;
mod view_render_camera;

mod compatibility_text_color;
mod fragment_alpha_feedback;
mod legacy_fragment_alpha;
mod transparency;
pub use transparency::RenderTransparency;

mod raster_feedback;
mod retained_transparency;

mod stencil_context;
mod stencil_program;
pub use stencil_context::RenderStencilState;
mod stencil_feedback;
mod stencil_plan;

mod unlit_face_policy;
pub use unlit_face_policy::RenderUnlitFaceColorPolicy;
mod driver_unlit_policy;
mod unlit_face_color;

mod projected_segments;
pub use projected_segments::{
    DashBehavior, ProjectedFailure, ProjectedFailureClass, ProjectedSegments, SegmentInterval,
    prepare_projected_segments,
};

mod memory_report;
pub use memory_report::{
    RenderDeviceMemoryHeap, RenderDeviceMemoryReport, RenderMemoryBudget, RenderMemoryQuantity,
    RenderMemoryQueryError, RenderMemorySource,
};

// These aliases exist only inside the private library-test harness; production exports are unchanged.
#[cfg(test)]
extern crate self as martensite_vulkan;
#[cfg(test)]
mod hatch_surface;

#[cfg(test)]
mod point_surface_texture_tests;

#[cfg(test)]
mod point_uv_pixels_tests;

mod point_raster_behavior;
pub use point_raster_behavior::RenderPointRasterBehavior;
mod fixed_grid_point;
#[cfg(test)]
mod fixed_grid_point_gpu_tests;
#[cfg(feature = "vertex-output-diagnostics")]
mod vertex_output_contract;
#[cfg(feature = "vertex-output-diagnostics")]
pub use vertex_output_contract::{
    PointVertexOutputRecord, TextVertexOutputRecord, VertexOutputFailure, VertexOutputFormat,
    VertexOutputRecord, VertexOutputRequest,
};

#[cfg(all(test, feature = "vertex-output-diagnostics"))]
mod vertex_output_fixture;

mod thin_axis_coverage;
pub use thin_axis_coverage::{RenderLineGrid, RenderLineOrigin, RenderThinAxisCoverage};
mod thin_axis;
mod triangle_grid_coverage;
mod viewport_grid;
pub use viewport_grid::{RenderViewportGridFailure, RenderViewportGridPrecision};
#[cfg(test)]
mod window_depth_float64_fixture;
#[cfg(test)]
mod window_depth_float64_tests;
mod window_depth_policy;
pub use triangle_grid_coverage::{
    RenderTriangleCoverageBehavior, RenderTriangleCoverageFailure, RenderTriangleGridOrigin,
};
pub use window_depth_policy::RenderWindowDepthBehavior;
mod window_depth_failure;
pub use window_depth_failure::{RenderWindowDepthBoundary, RenderWindowDepthFailure};
mod window_depth_bounds;
mod window_depth_closure;
#[cfg(test)]
mod window_depth_plane;
#[cfg(test)]
mod window_depth_projection;

#[cfg(test)]
mod clipped_depth_partition;

#[cfg(test)]
mod clipped_depth_shared_reference;

#[cfg(test)]
mod clipped_depth_reference_empty;
