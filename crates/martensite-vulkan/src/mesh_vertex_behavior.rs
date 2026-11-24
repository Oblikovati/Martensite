//! Versioned storage of submitted mesh vertices. This controls raster input only.

/// The version of the mesh vertex stream used for eligible raster draws.
/// Example: `RenderBehaviorConfiguration::default().with_mesh_vertices(RenderMeshVertexBehavior::Quantized16OctV2)`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RenderMeshVertexBehavior {
    /// Preserve submitted binary32 positions, homogeneous weights, and normals.
    #[default]
    Float32V1,
    /// Quantize unit-weight positions to part bounds and encode normals as octahedral pairs.
    ///
    /// Specialized draws and inputs that cannot use the packed format keep the binary32 path.
    Quantized16OctV2,
}
