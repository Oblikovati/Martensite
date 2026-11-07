//! `RwObjSubMesh` and why a new one starts.

use martensite_tkernel::tcollection::AsciiString;

/// The names active while reading; `RwObjSubMesh::default()`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RwObjSubMesh {
    /// The name of the active object.
    pub object: AsciiString,
    /// The name of the active group.
    pub group: AsciiString,
    /// The name of the active smoothing group.
    pub smooth_group: AsciiString,
    /// The name of the active material.
    pub material: AsciiString,
}

/// Why a new sub-mesh starts; `RwObjSubMeshReason::NewGroup`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RwObjSubMeshReason {
    /// A new object, which in a valid file occurs once, at the very beginning.
    NewObject,
    /// A new group (`g` item).
    NewGroup,
    /// A new material (`usemtl` item).
    NewMaterial,
    /// A new smoothing group (`s` item).
    NewSmoothGroup,
}
