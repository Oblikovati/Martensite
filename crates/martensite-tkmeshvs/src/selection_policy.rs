// SPDX-License-Identifier: GPL-2.0-only
//! Martensite mesh selection policies: stable values and mask combinations.
//!
//! ```
//! use martensite_tkmeshvs::selection_policy::*;
//! let grouped_faces = MeshVSSelectionModeFlags(MESHVS_SMF_GROUP.0 | MESHVS_SMF_FACE.0);
//! assert_eq!(grouped_faces.0, 264);
//! assert_eq!(MESHVS_BP_DEFAULT, MESHVS_BP_USER);
//! ```

/// Original signed priority; `let priority: MeshVSBuilderPriority = -7` remains representable.
pub type MeshVSBuilderPriority = i32;

/// Bit mask, including unnamed combinations; `MeshVSSelectionModeFlags(0x109)`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct MeshVSSelectionModeFlags(pub i32);

/// Whole-mesh sensitive-entity policy; `MeshVSMeshSelectionMethod::Nodes as i32 == 1`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum MeshVSMeshSelectionMethod {
    /// `MeshVSMeshSelectionMethod::Precise`.
    Precise = 0,
    /// `MeshVSMeshSelectionMethod::Nodes`.
    Nodes = 1,
    /// `MeshVSMeshSelectionMethod::Box`.
    Box = 2,
    /// permits unnamed value 3 within this enumeration's range.
    Unnamed3 = 3,
}

/// Preserve the complete defined enum range; `mesh_selection_method_from_raw(3).is_some()`.
pub fn mesh_selection_method_from_raw(value: i32) -> Option<MeshVSMeshSelectionMethod> {
    match value {
        0 => Some(MeshVSMeshSelectionMethod::Precise),
        1 => Some(MeshVSMeshSelectionMethod::Nodes),
        2 => Some(MeshVSMeshSelectionMethod::Box),
        3 => Some(MeshVSMeshSelectionMethod::Unnamed3),
        _ => None,
    }
}

/// `MESHVS_BP_MESH` priority.
pub const MESHVS_BP_MESH: MeshVSBuilderPriority = 5;
/// `MESHVS_BP_NODAL_COLOR` priority.
pub const MESHVS_BP_NODAL_COLOR: MeshVSBuilderPriority = 10;
/// `MESHVS_BP_ELEM_COLOR` priority.
pub const MESHVS_BP_ELEM_COLOR: MeshVSBuilderPriority = 15;
/// `MESHVS_BP_TEXT` priority.
pub const MESHVS_BP_TEXT: MeshVSBuilderPriority = 20;
/// `MESHVS_BP_VECTOR` priority.
pub const MESHVS_BP_VECTOR: MeshVSBuilderPriority = 25;
/// `MESHVS_BP_USER` extension priority.
pub const MESHVS_BP_USER: MeshVSBuilderPriority = 30;
/// `MESHVS_BP_DEFAULT` aliases the user priority.
pub const MESHVS_BP_DEFAULT: MeshVSBuilderPriority = MESHVS_BP_USER;

/// `MESHVS_SMF_MESH` selects the whole mesh.
pub const MESHVS_SMF_MESH: MeshVSSelectionModeFlags = MeshVSSelectionModeFlags(0);
/// `MESHVS_SMF_NODE` selects nodes.
pub const MESHVS_SMF_NODE: MeshVSSelectionModeFlags = MeshVSSelectionModeFlags(1);
/// `MESHVS_SMF_0D` selects zero-dimensional elements.
pub const MESHVS_SMF_0D: MeshVSSelectionModeFlags = MeshVSSelectionModeFlags(2);
/// `MESHVS_SMF_LINK` selects links.
pub const MESHVS_SMF_LINK: MeshVSSelectionModeFlags = MeshVSSelectionModeFlags(4);
/// `MESHVS_SMF_FACE` selects faces.
pub const MESHVS_SMF_FACE: MeshVSSelectionModeFlags = MeshVSSelectionModeFlags(8);
/// `MESHVS_SMF_VOLUME` selects volumes.
pub const MESHVS_SMF_VOLUME: MeshVSSelectionModeFlags = MeshVSSelectionModeFlags(16);
/// `MESHVS_SMF_ELEMENT` combines all element kinds.
pub const MESHVS_SMF_ELEMENT: MeshVSSelectionModeFlags = MeshVSSelectionModeFlags(2 | 4 | 8 | 16);
/// `MESHVS_SMF_ALL` combines nodes and elements.
pub const MESHVS_SMF_ALL: MeshVSSelectionModeFlags =
    MeshVSSelectionModeFlags(MESHVS_SMF_ELEMENT.0 | MESHVS_SMF_NODE.0);
/// `MESHVS_SMF_GROUP` can qualify entity-kind masks.
pub const MESHVS_SMF_GROUP: MeshVSSelectionModeFlags = MeshVSSelectionModeFlags(0x100);
