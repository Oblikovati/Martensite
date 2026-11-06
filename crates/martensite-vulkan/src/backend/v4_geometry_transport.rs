use crate::{RenderError, RenderTriangleCoverageFailure as Failure};

type VertexPair = (&'static [u8], &'static [u8]);
const V4_BUILTIN_OUTPUT_COMPONENTS: u32 = 8;

macro_rules! v4_vertex_pair {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../shaders/", $name)).as_slice(),
            include_bytes!(concat!(
                "../../shaders/triangle-grid-v4.unsnapped-v2.",
                $name
            ))
            .as_slice(),
        )
    };
}

const VERTICES: &[VertexPair] = &[
    v4_vertex_pair!("lighting.vert.spv"),
    v4_vertex_pair!("numeric-max-v2.lighting.vert.spv"),
];

const CLIPPED_VERTICES: &[VertexPair] = &[
    (
        include_bytes!("../../shaders/lighting.clipped.vert.spv").as_slice(),
        include_bytes!("../../shaders/world-clip-grid12-v2.triangle-grid-v4.unsnapped-v2.lighting.clipped.vert.spv").as_slice(),
    ),
    (
        include_bytes!("../../shaders/numeric-max-v2.lighting.clipped.vert.spv").as_slice(),
        include_bytes!("../../shaders/world-clip-grid12-v2.triangle-grid-v4.unsnapped-v2.numeric-max-v2.lighting.clipped.vert.spv").as_slice(),
    ),
];

pub(super) fn vertex(stock: &'static [u8]) -> Option<&'static [u8]> {
    VERTICES
        .iter()
        .find(|(before, _)| *before == stock)
        .map(|(_, v4)| *v4)
}

pub(super) fn clipped_vertex(stock: &'static [u8]) -> Option<&'static [u8]> {
    CLIPPED_VERTICES
        .iter()
        .find(|(before, _)| *before == stock)
        .map(|(_, selected)| *selected)
}

pub(super) const fn required_limits_for(
    clipped: bool,
    original_clip_positions: bool,
) -> V4GeometryLimits {
    let clipped_components = if clipped { 4 } else { 0 };
    let original_clip_components = if original_clip_positions { 12 } else { 0 };
    let output_component_limit = geometry_output_component_limit();
    V4GeometryLimits {
        geometry_shader: true,
        input_components: 44 + clipped_components,
        output_components: output_component_limit,
        output_vertices: 3,
        total_output_components: (92 + V4_BUILTIN_OUTPUT_COMPONENTS) * 3
            + (clipped_components + original_clip_components) * 3,
        fragment_input_components: 112 + clipped_components + original_clip_components,
    }
}

const fn geometry_output_component_limit() -> u32 {
    // V4's highest occupied user slot is location 27, component 3. Vulkan's
    // The declared built-in interface includes Position, PointSize, both distance arrays,
    // and PrimitiveId; count all eight conservatively, including unwritten members.
    27 * 4 + 3 + V4_BUILTIN_OUTPUT_COMPONENTS + 1
}

pub(super) fn limits_from_runtime(runtime: &super::runtime::VulkanRuntime) -> V4GeometryLimits {
    V4GeometryLimits {
        geometry_shader: runtime.geometry_shader,
        input_components: runtime.limits.max_geometry_input_components,
        output_components: runtime.limits.max_geometry_output_components,
        output_vertices: runtime.limits.max_geometry_output_vertices,
        total_output_components: runtime.limits.max_geometry_total_output_components,
        fragment_input_components: runtime.limits.max_fragment_input_components,
    }
}

pub(super) fn require_pipeline(state: V4GeometryPipelineState) -> Result<(), RenderError> {
    if state.supported_surface && state.shader_modules == 2 {
        return Ok(());
    }
    Err(unsupported(&format!(
        "pipeline={state:?}; expected one selected Phong surface pair before V4 transport"
    )))
}

pub(super) fn require_limits(
    actual: V4GeometryLimits,
    clipped: bool,
    original_clip_positions: bool,
) -> Result<(), RenderError> {
    if !actual.geometry_shader {
        return Err(unsupported("geometryShader=false; expected true"));
    }
    require_geometry_limits(actual, clipped, original_clip_positions)?;
    require_fragment_limits(actual, clipped, original_clip_positions)
}

fn require_geometry_limits(
    actual: V4GeometryLimits,
    clipped: bool,
    original_clip_positions: bool,
) -> Result<(), RenderError> {
    require_geometry_components(actual, clipped, original_clip_positions)?;
    require_geometry_vertices(actual, clipped, original_clip_positions)
}

fn require_geometry_components(
    actual: V4GeometryLimits,
    clipped: bool,
    original_clip_positions: bool,
) -> Result<(), RenderError> {
    let required = required_limits_for(clipped, original_clip_positions);
    require_at_least(
        "maxGeometryInputComponents",
        actual.input_components,
        required.input_components,
    )?;
    require_at_least(
        "maxGeometryOutputComponents",
        actual.output_components,
        required.output_components,
    )
}

fn require_geometry_vertices(
    actual: V4GeometryLimits,
    clipped: bool,
    original_clip_positions: bool,
) -> Result<(), RenderError> {
    let required = required_limits_for(clipped, original_clip_positions);
    require_at_least(
        "maxGeometryOutputVertices",
        actual.output_vertices,
        required.output_vertices,
    )?;
    require_at_least(
        "maxGeometryTotalOutputComponents",
        actual.total_output_components,
        required.total_output_components,
    )
}

fn require_fragment_limits(
    actual: V4GeometryLimits,
    clipped: bool,
    original_clip_positions: bool,
) -> Result<(), RenderError> {
    let required = required_limits_for(clipped, original_clip_positions);
    require_at_least(
        "maxFragmentInputComponents",
        actual.fragment_input_components,
        required.fragment_input_components,
    )
}

fn require_at_least(name: &str, actual: u32, required: u32) -> Result<(), RenderError> {
    if actual >= required {
        return Ok(());
    }
    Err(unsupported(&format!(
        "{name}={actual}; expected >= {required} for V4 transport"
    )))
}

fn unsupported(actual: &str) -> RenderError {
    Failure::UnsupportedInterface {
        actual: actual.to_owned(),
    }
    .into()
}

pub(super) fn shader_bytes() -> &'static [u8] {
    include_bytes!("../../shaders/triangle-grid-v4-passthrough.geom.spv")
}

pub(super) fn clipped_shader_bytes() -> &'static [u8] {
    include_bytes!("../../shaders/triangle-grid-v4-passthrough.clipped.geom.spv")
}

pub(super) fn clipped_world_clip_shader_bytes() -> &'static [u8] {
    include_bytes!(
        "../../shaders/world-clip-grid12-v2.triangle-grid-v4-compact-passthrough.geom.spv"
    )
}

pub(super) fn requires_original_clip_positions(
    clipped: bool,
    grid12_world_clip: bool,
    world_clip: crate::RenderWorldClipInterpolationBehavior,
    coverage: crate::RenderTriangleCoverageBehavior,
    phong: crate::RenderPhongInterpolationBehavior,
) -> bool {
    clipped
        && grid12_world_clip
        && world_clip == crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V2
        && matches!(
            coverage,
            crate::RenderTriangleCoverageBehavior::NativeGrid12V4 { .. }
        )
        && phong == crate::RenderPhongInterpolationBehavior::UnsnappedV3
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct V4GeometryPipelineState {
    pub supported_surface: bool,
    pub shader_modules: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct V4GeometryLimits {
    pub geometry_shader: bool,
    pub input_components: u32,
    pub output_components: u32,
    pub output_vertices: u32,
    pub total_output_components: u32,
    pub fragment_input_components: u32,
}

#[cfg(test)]
#[path = "v4_geometry_reflection_tests.rs"]
mod reflection_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_registry_only_selects_the_two_registered_stock_programs() {
        for (stock, v4) in VERTICES {
            assert_eq!(vertex(stock), Some(*v4));
            assert_ne!(stock, v4);
        }
        assert!(vertex(&[0]).is_none());
    }

    #[test]
    fn clipped_vertex_registry_is_separate_from_the_unclipped_transport() {
        for (stock, clipped) in CLIPPED_VERTICES {
            assert_eq!(clipped_vertex(stock), Some(*clipped));
            assert_ne!(vertex(stock), Some(*clipped));
        }
        assert!(clipped_vertex(&[0]).is_none());
        assert_ne!(shader_bytes(), clipped_shader_bytes());
        assert_ne!(clipped_shader_bytes(), clipped_world_clip_shader_bytes());
    }

    #[test]
    fn transport_accepts_the_registered_pipeline_shape() {
        let state = V4GeometryPipelineState {
            supported_surface: true,
            shader_modules: 2,
        };
        assert!(require_pipeline(state).is_ok());
        assert!(require_limits(required_limits_for(false, false), false, false).is_ok());
        assert!(require_limits(required_limits_for(true, false), true, false).is_ok());
        assert!(require_limits(required_limits_for(true, true), true, true).is_ok());
    }

    #[test]
    fn clipped_transport_requires_four_more_components_at_each_stage_boundary() {
        let base = required_limits_for(false, false);
        let clipped = required_limits_for(true, false);
        let world_clipped = required_limits_for(true, true);
        assert_eq!(clipped.input_components, base.input_components + 4);
        assert_eq!(clipped.output_components, base.output_components);
        assert_eq!(
            clipped.total_output_components,
            base.total_output_components + 12
        );
        assert_eq!(
            clipped.fragment_input_components,
            base.fragment_input_components + 4
        );
        assert_eq!(world_clipped.output_components, clipped.output_components);
        assert_eq!(
            world_clipped.total_output_components,
            clipped.total_output_components + 36
        );
        assert_eq!(
            world_clipped.fragment_input_components,
            clipped.fragment_input_components + 12
        );
        assert!(require_limits(clipped, true, false).is_ok());
        assert!(require_limits(world_clipped, true, true).is_ok());
        assert!(require_limits(clipped, true, true).is_err());
        assert!(require_limits(base, true, false).is_err());
    }

    #[test]
    fn only_clipped_native_grid_v4_world_clip_v2_phong_v3_transports_original_clip_positions() {
        use crate::{
            RenderPhongInterpolationBehavior as Phong, RenderTriangleCoverageBehavior as Coverage,
            RenderTriangleGridOrigin as Origin, RenderWorldClipInterpolationBehavior as Clip,
        };
        let coverage = Coverage::NativeGrid12V4 {
            origin: Origin::TopDown,
        };
        assert!(requires_original_clip_positions(
            true,
            true,
            Clip::ProjectedGrid12V2,
            coverage,
            Phong::UnsnappedV3,
        ));
        assert!(!requires_original_clip_positions(
            false,
            true,
            Clip::ProjectedGrid12V2,
            coverage,
            Phong::UnsnappedV3,
        ));
        assert!(!requires_original_clip_positions(
            true,
            false,
            Clip::ProjectedGrid12V2,
            coverage,
            Phong::UnsnappedV3,
        ));
        assert!(!requires_original_clip_positions(
            true,
            true,
            Clip::ProjectedGrid12V1,
            coverage,
            Phong::UnsnappedV3,
        ));
    }

    #[test]
    fn transport_rejects_non_surface_or_extra_stage_compositions() {
        assert_rejects_pipeline(false, 2);
        assert_rejects_pipeline(true, 3);
    }

    #[test]
    fn transport_rejects_missing_geometry_shader_feature() {
        assert_rejects_field(
            |limits| limits.geometry_shader = false,
            "geometryShader=false",
        );
    }

    #[test]
    fn transport_rejects_small_geometry_component_limits() {
        assert_rejects_field(
            |limits| limits.input_components -= 1,
            "maxGeometryInputComponents",
        );
        assert_rejects_field(
            |limits| limits.output_components -= 1,
            "maxGeometryOutputComponents",
        );
    }

    #[test]
    fn output_cap_includes_builtin_components_and_compacted_highest_user_slot() {
        assert_eq!(geometry_output_component_limit(), 120);
        for limits in [
            required_limits_for(false, false),
            required_limits_for(true, false),
            required_limits_for(true, true),
        ] {
            assert_eq!(limits.output_components, 120);
        }
    }

    #[test]
    fn transport_rejects_small_geometry_vertex_limits() {
        assert_rejects_field(
            |limits| limits.output_vertices -= 1,
            "maxGeometryOutputVertices",
        );
        assert_rejects_field(
            |limits| limits.total_output_components -= 1,
            "maxGeometryTotalOutputComponents",
        );
    }

    #[test]
    fn transport_rejects_small_fragment_input_limit() {
        assert_rejects_field(
            |limits| limits.fragment_input_components -= 1,
            "maxFragmentInputComponents",
        );
    }

    fn assert_rejects_pipeline(supported_surface: bool, shader_modules: usize) {
        let state = V4GeometryPipelineState {
            supported_surface,
            shader_modules,
        };
        assert!(require_pipeline(state).is_err());
    }

    fn assert_rejects_field(mut mutate: impl FnMut(&mut V4GeometryLimits), field: &str) {
        let mut limits = required_limits_for(true, false);
        mutate(&mut limits);
        let error = require_limits(limits, true, false).unwrap_err().to_string();
        assert!(error.contains(field), "{error}: expected field {field}");
    }
}
