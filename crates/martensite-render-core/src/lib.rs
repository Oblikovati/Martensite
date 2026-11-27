//! Backend-agnostic Graphic3d lowering, immutable frame submissions, and CPU ray-tracing
//! structures shared by `martensite-vulkan` and `martensite-webgpu` (#633).
//!
//! # Rust-native handles (#592)
//!
//! Renderer classes keep their native descriptor names and use typed shared owners.
//! Framebuffer identities use `Handle<dyn Transient>`; images, primitive arrays and
//! clipping collections use TKService's state-lending interfaces. A layer captures
//! an owning clipping snapshot after culling, before building its draw passes.
#![cfg_attr(test, allow(clippy::duplicate_mod))] // Unit tests share scene fixtures by path.

/// Typed backend traits and opaque caches implemented by concrete GPU backends.
pub mod device_backend;
/// Backend-agnostic frame-work resource counters and exclusive CPU phase clock.
pub mod frame_work_ledger;

pub use device_backend::{
    CapturePlanes, MIN_PHONG_BATCH_VERTICES, OpaqueMemo, RenderDeferredFrame, RenderDeviceBackend,
    RenderDeviceOptions, RenderGeometryBatch, RenderHiZHistory, RenderWindowPresenter,
    RenderedFrame, directional_lights, directional_phong_draw, ordinary_surface,
};
pub use frame_work_ledger::{DeviceCount, FrameWorkLedger, PhaseAccounts, PhaseGuard, WorkPhase};

mod device_close;
mod mesh_lod;
pub use mesh_lod::RenderMeshLevel;
mod assembly_streaming;
pub use assembly_streaming::{RenderAssemblyStreaming, RenderAssemblyStreamingProgress};
#[doc(hidden)]
#[allow(missing_docs)]
pub mod draw_instances;
mod immediate_elision;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod immediate_main;
mod main_layer_notices;
pub use device_close::RenderDeviceCloseOutcome;
mod device_failure;
mod portability_failure;
mod validation_admission_failure;
pub use portability_failure::RenderPortabilityFailure;
pub use validation_admission_failure::{
    RenderValidationAdmissionFailure, VERIFIED_VALIDATION_LAYER, ValidationLayerVersion,
};
#[doc(hidden)]
#[allow(missing_docs)]
pub mod surface_platform;
pub use surface_platform::{RenderSurfacePlatform, RenderSurfacePlatformFailure};
mod driver_gui_surface;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod error;
pub use device_failure::{RenderDeviceFailure, RenderDeviceFailureClass, RenderDeviceState};
#[cfg(test)]
mod gui_surface_error_tests;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod gui_surface_outcome;
mod gui_target_stamp;
pub use gui_surface_outcome::{
    RenderGuiFreshPacket, RenderGuiSurfaceCause, RenderGuiSurfaceChange, RenderGuiSurfaceOutcome,
    RenderGuiSurfacePhase, RenderGuiSurfacePolicy,
};
mod render_driver_identity;
pub use render_driver_identity::RenderDriverIdentity;
mod driver_device_lost;
mod driver_lighting_capability;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod driver_options;
pub use driver_options::RenderDriverOptions;
#[cfg(feature = "lit-only-command-diagnostics")]
pub use error::LitOnlyFailure;
pub use error::lighting_selection::RenderLightingSelectionChanged;
#[cfg(feature = "raytrace-input-diagnostics")]
#[doc(hidden)]
#[allow(missing_docs)]
pub mod raytrace_input_observation;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod scene;
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
mod mesh_vertex_behavior;
pub use mesh_vertex_behavior::RenderMeshVertexBehavior;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod clipped_depth_constants;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod fixed_modelview_inverse;
mod projection_factors;
pub use projection_factors::{RenderProjectionBehavior, RenderProjectionFactors};
mod text_grid_interpolation;
pub use text_grid_interpolation::{
    RenderTextInterpolationBehavior, RenderTextInterpolationFailure,
};
mod thin_line_coverage;
pub use thin_line_coverage::RenderThinLineCoverage;
mod raster_policy_failure;
pub use raster_policy_failure::{RenderRasterCapabilityFailure, RenderRasterPolicyFailure};
#[doc(hidden)]
#[allow(missing_docs)]
pub mod raster_policy_admission;

pub use scene::{FrameCapture, RenderBackend, RenderFrame, RenderVertex, identity_clip};
mod element_stats;
mod element_upload;
mod frame_stats;
mod frame_stats_chart;
mod frame_stats_overlay;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod shader_grid;
pub use shader_grid::{RenderGridEcho, RenderGridPixel, RenderShaderGrid, RenderShaderGridBlock};
mod frame_stats_structures;
mod graduated_trihedron;
pub use graduated_trihedron::RenderGraduatedTrihedron;
mod graphic_group;
mod group_flipping;
mod group_record_frame;
mod primitive;
mod primitive_buffer_read;
mod primitive_cache;
mod primitive_ranges;
mod primitive_topology;
pub use frame_stats::{RenderFrameStats, RenderImageMemory};
mod driver_frame_flight;
mod frame_work;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod redraw_capture;
pub use frame_work::{RenderDeviceWork, RenderWorkTimes};
pub use graphic_group::Graphic3dDrawGroup;

mod style;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod user_program;
pub use user_program::{
    RenderUserProgramFailure, RenderUserProgramLibrary, SamplerBinding, StageReflection,
    UniformMember, UniformScalar, UniformShape, UserProgramKey, UserProgramModules, UserStage,
};

#[doc(hidden)]
#[allow(missing_docs)]
pub mod depth_prepass;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod render_layers;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod wide_line;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod wide_line_columns;
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
#[doc(hidden)]
#[allow(missing_docs)]
pub mod driver_backend;
mod driver_limits;
pub use device_limits::RenderDeviceLimits;
mod driver_structure;
mod driver_text;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod driver_texture_limit;
mod driver_view;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod view_capture;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod view_diagnostics;
mod view_shader_grid;
mod view_statistics;
pub use capture_image_error::{
    CaptureImageError, CaptureImageFailure, CaptureImageLayout, CaptureImageOperation,
};
mod background_chart;
mod bound_box_highlight;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod colored_quad_background;
mod environment_cube_pixels;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod environment_frame;
mod environment_texture_map;
mod fixed_background;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod gradient_background;
mod image_background;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod raytrace;
pub use raytrace::{RaytraceHaltonSampler, RaytraceTileOffset, RaytraceTileSampler};
mod render_scale;
mod shadow_map_dump;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod shadow_maps;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod stereo_pair;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod subview_composition;
pub use shadow_maps::{RenderShadowMap, RenderShadowMaps};
mod silhouette_outline;
mod view_environment;
mod view_forward;
mod view_graduated_trihedron;
mod view_gui_attachment;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod view_gui_composition;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod view_gui_layer;
mod view_layer_clear;
mod view_output_modes;
mod view_raytrace;
mod view_scene;
mod view_target;
pub use view_target::RenderFrameBuffer;
mod view_window_lifecycle;
mod view_window_observation;
pub use driver::Graphic3dRenderDriver;
pub use driver_structure::Graphic3dRenderStructure;
pub use driver_view::{Graphic3dRenderView, RenderRedrawCursor, RenderRedrawFailure};

mod persistent_transform;

#[doc(hidden)]
#[allow(missing_docs)]
pub mod draw;
pub use draw::{LineStyle, PrimitiveDraw, PrimitiveTopology, TriangleDraw};

mod line_style;

#[doc(hidden)]
#[allow(missing_docs)]
pub mod highlight_style;

#[doc(hidden)]
#[allow(missing_docs)]
pub mod point_style;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod polygon_point_raster;
pub use point_style::PointStyle;
pub use polygon_point_raster::RenderPolygonPointFailure;

mod raster_image;
pub use raster_image::RasterImage;
mod textured_draw;
pub use textured_draw::TexturedStyle;
mod pixel_text;
mod text_layout;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod text_logic;
pub use pixel_text::{PixelTextView, lower_pixel_text};
pub use text_layout::{GlyphLayout, TextLayoutSource};
mod text_font;
pub use text_font::FreeTypeTextSource;
mod text_atlas;

mod text_background;
mod text_culling;
mod text_drawing;
mod text_label_record;
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
#[doc(hidden)]
#[allow(missing_docs)]
pub mod mesh_edges;
mod retained_aspect_textures;
mod retained_cap_textures;
mod retained_capping;
mod retained_eye_edge_matrix;
mod retained_mesh_edges;
mod surface_alpha;
pub use mesh_edges::{RenderMeshEdgePipeline, RenderMeshEdges};

mod material;
mod material_components;
pub mod openpbr_env_cdf;
pub mod openpbr_luts;
mod openpbr_material;
pub mod openpbr_math;
pub use material::{RenderMaterial, RenderMaterialColorSpace};
pub use material_components::RenderMaterialCommon;
pub use openpbr_material::{
    OpenPbrGpuMaterial, OpenPbrTextureIndices, lobe_flags as openpbr_lobe_flags,
};

#[doc(hidden)]
#[allow(missing_docs)]
pub mod light_snapshot;
pub use light_snapshot::RenderLightSet;

mod phong_interpolation;
pub use phong_interpolation::{RenderPhongInterpolationBehavior, RenderPhongInterpolationFailure};
mod light_dot_behavior;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod lighting;
pub use light_dot_behavior::RenderLightDotBehavior;
mod lighting_pipeline;
pub use lighting::{RenderLighting, RenderShading};
pub use lighting_pipeline::RenderLightingPipeline;

mod retained_lighting;

mod capping_lighting;
mod capping_mask_aspect;

#[doc(hidden)]
#[allow(missing_docs)]
pub mod quad_assembly;

mod fixed_flat_sources;
mod fixed_light_snapshot;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod fixed_lighting;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod fixed_line_stipple;
mod fixed_marker_environment;
mod fixed_normalization;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod fixed_surface_environment;
mod fixed_view_lighting;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod line_guard_band;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod polygon_offset;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod retained_hatch;
mod retained_hollow;
pub use polygon_offset::RenderPolygonOffset;

#[doc(hidden)]
#[allow(missing_docs)]
pub mod point_surface_texture;
mod retained_surface_texture;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod retained_texture_images;
mod retained_texture_samplers;
mod retained_user_program;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod surface_material_maps;
mod surface_sphere_texgen;
mod surface_texgen;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod surface_texture;
mod surface_texture_compressed;
mod surface_texture_lifetime;
mod surface_texture_pixels;
mod surface_texture_uv;
mod texture_coordinate_frame;
mod texture_coordinate_pipeline;
pub use texture_coordinate_pipeline::RenderTextureCoordinatePipeline;
mod view_render_camera;

mod compatibility_text_color;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod fragment_alpha_feedback;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod legacy_fragment_alpha;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod transparency;
pub use transparency::RenderTransparency;

#[doc(hidden)]
#[allow(missing_docs)]
pub mod raster_feedback;
mod retained_transparency;

#[doc(hidden)]
#[allow(missing_docs)]
pub mod stencil_context;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod stencil_feedback;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod stencil_plan;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod stencil_program;
pub use stencil_context::RenderStencilState;

mod unlit_face_policy;
pub use unlit_face_policy::RenderUnlitFaceColorPolicy;
mod driver_unlit_policy;
mod unlit_face_color;

#[doc(hidden)]
#[allow(missing_docs)]
pub mod projected_segments;
pub use projected_segments::{
    DashBehavior, ProjectedFailure, ProjectedFailureClass, ProjectedSegments, SegmentInterval,
    prepare_projected_segments,
};

#[doc(hidden)]
#[allow(missing_docs)]
pub mod memory_report;
pub use memory_report::{
    RenderDeviceMemoryHeap, RenderDeviceMemoryReport, RenderMemoryBudget, RenderMemoryQuantity,
    RenderMemoryQueryError, RenderMemorySource,
};

mod point_raster_behavior;
pub use point_raster_behavior::RenderPointRasterBehavior;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod fixed_grid_point;

#[cfg(feature = "vertex-output-diagnostics")]
mod vertex_output_contract;
#[cfg(feature = "vertex-output-diagnostics")]
pub use vertex_output_contract::{
    PointVertexOutputRecord, TextVertexOutputRecord, VertexOutputFailure, VertexOutputFormat,
    VertexOutputRecord, VertexOutputRequest,
};

mod thin_axis_coverage;
pub use thin_axis_coverage::{RenderLineGrid, RenderLineOrigin, RenderThinAxisCoverage};
#[doc(hidden)]
#[allow(missing_docs)]
pub mod thin_axis;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod triangle_grid_coverage;
mod viewport_grid;
pub use viewport_grid::{RenderViewportGridFailure, RenderViewportGridPrecision};
mod window_depth_policy;
pub use triangle_grid_coverage::{
    RenderTriangleCoverageBehavior, RenderTriangleCoverageFailure, RenderTriangleGridOrigin,
};
pub use window_depth_policy::RenderWindowDepthBehavior;
mod window_depth_failure;
pub use window_depth_failure::{RenderWindowDepthBoundary, RenderWindowDepthFailure};
#[doc(hidden)]
#[allow(missing_docs)]
pub mod window_depth_bounds;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod window_depth_closure;
