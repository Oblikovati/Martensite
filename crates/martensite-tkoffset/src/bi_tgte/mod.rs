//! `BiTgte`: the rolling-ball blend (`BiTgteBlend`) with its curve on edge adaptor
//! (`BiTgteCurveOnEdge`) and contact types (`BiTgteContactType`).
mod blend;
mod curve_on_edge;

pub use blend::BiTgteBlend;
pub use curve_on_edge::BiTgteCurveOnEdge;

/// `BiTgteContactType`: the kinds of the two shapes a blend surface touches.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum BiTgteContactType {
    /// `FaceFace`.
    FaceFace,
    /// `BiTgteContactType::face_edge`.
    FaceEdge,
    /// `BiTgteContactType::face_vertex`.
    FaceVertex,
    /// `EdgeEdge`.
    EdgeEdge,
    /// `BiTgteContactType::edge_vertex`.
    EdgeVertex,
    /// `BiTgteContactType::vertex_vertex`.
    VertexVertex,
}
