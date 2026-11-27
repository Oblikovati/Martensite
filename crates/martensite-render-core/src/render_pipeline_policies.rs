//! One owned selection composes policies without positional constructor ladders.
/// Select independent rendering behavior with owned values.
/// Example: `RenderPipelinePolicies { raster: RenderRasterPolicy::LastProvokingVertexV2, ..Default::default() }`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderPipelinePolicies {
    /// Texture coordinates retain their independently selected generation rules.
    pub texture_coordinates: crate::RenderTextureCoordinatePipeline,
    /// Mesh edges retain their independent polygon compatibility selection.
    pub mesh_edges: crate::RenderMeshEdgePipeline,
    /// Programmable or fixed-function lighting remains independent of rasterization.
    pub lighting: crate::RenderLightingPipeline,
    /// Unlit face behavior is validated against the selected lighting family.
    pub unlit_faces: crate::RenderUnlitFaceColorPolicy,
    /// Window and composition failures retain their explicit GUI policy.
    pub gui_surface: crate::RenderGuiSurfacePolicy,
    /// Device admission requires this exact raster behavior, without fallback.
    pub raster: crate::RenderRasterPolicy,
}
