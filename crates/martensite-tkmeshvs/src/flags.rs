// SPDX-License-Identifier: GPL-2.0-only
//! MeshVS keys and bit masks; arbitrary integer drawer keys stay valid.
/// Display-mode mask, including combinations; `let mode:MeshVSDisplayModeFlags=MESHVS_DMF_SHADING`.
pub type MeshVSDisplayModeFlags = i32;
/// Mesh entity mask admits unnamed combinations; `MeshVSEntityType(0x09)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct MeshVSEntityType(pub i32);
/// Appearance attribute key; `MeshVSDrawerAttribute::InteriorColor as i32`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum MeshVSDrawerAttribute {
    /// `MeshVSDrawerAttribute::InteriorStyle` appearance key.
    InteriorStyle = 0,
    /// `MeshVSDrawerAttribute::InteriorColor` appearance key.
    InteriorColor = 1,
    /// `MeshVSDrawerAttribute::BackInteriorColor` appearance key.
    BackInteriorColor = 2,
    /// `MeshVSDrawerAttribute::EdgeColor` appearance key.
    EdgeColor = 3,
    /// `MeshVSDrawerAttribute::EdgeType` appearance key.
    EdgeType = 4,
    /// `MeshVSDrawerAttribute::EdgeWidth` appearance key.
    EdgeWidth = 5,
    /// `MeshVSDrawerAttribute::HatchStyle` appearance key.
    HatchStyle = 6,
    /// `MeshVSDrawerAttribute::FrontMaterial` appearance key.
    FrontMaterial = 7,
    /// `MeshVSDrawerAttribute::BackMaterial` appearance key.
    BackMaterial = 8,
    /// `MeshVSDrawerAttribute::BeamType` appearance key.
    BeamType = 9,
    /// `MeshVSDrawerAttribute::BeamWidth` appearance key.
    BeamWidth = 10,
    /// `MeshVSDrawerAttribute::BeamColor` appearance key.
    BeamColor = 11,
    /// `MeshVSDrawerAttribute::MarkerType` appearance key.
    MarkerType = 12,
    /// `MeshVSDrawerAttribute::MarkerColor` appearance key.
    MarkerColor = 13,
    /// `MeshVSDrawerAttribute::MarkerScale` appearance key.
    MarkerScale = 14,
    /// `MeshVSDrawerAttribute::TextColor` appearance key.
    TextColor = 15,
    /// `MeshVSDrawerAttribute::TextHeight` appearance key.
    TextHeight = 16,
    /// `MeshVSDrawerAttribute::TextFont` appearance key.
    TextFont = 17,
    /// `MeshVSDrawerAttribute::TextExpansionFactor` appearance key.
    TextExpansionFactor = 18,
    /// `MeshVSDrawerAttribute::TextSpace` appearance key.
    TextSpace = 19,
    /// `MeshVSDrawerAttribute::TextStyle` appearance key.
    TextStyle = 20,
    /// `MeshVSDrawerAttribute::TextDisplayType` appearance key.
    TextDisplayType = 21,
    /// `MeshVSDrawerAttribute::TextTexFont` appearance key.
    TextTexFont = 22,
    /// `MeshVSDrawerAttribute::TextFontAspect` appearance key.
    TextFontAspect = 23,
    /// `MeshVSDrawerAttribute::VectorColor` appearance key.
    VectorColor = 24,
    /// `MeshVSDrawerAttribute::VectorMaxLength` appearance key.
    VectorMaxLength = 25,
    /// `MeshVSDrawerAttribute::VectorArrowPart` appearance key.
    VectorArrowPart = 26,
    /// `MeshVSDrawerAttribute::IsAllowOverlapped` appearance key.
    IsAllowOverlapped = 27,
    /// `MeshVSDrawerAttribute::Reflection` appearance key.
    Reflection = 28,
    /// `MeshVSDrawerAttribute::ColorReflection` appearance key.
    ColorReflection = 29,
    /// `MeshVSDrawerAttribute::ShrinkCoeff` appearance key.
    ShrinkCoeff = 30,
    /// `MeshVSDrawerAttribute::MaxFaceNodes` appearance key.
    MaxFaceNodes = 31,
    /// `MeshVSDrawerAttribute::ComputeTime` appearance key.
    ComputeTime = 32,
    /// `MeshVSDrawerAttribute::ComputeSelectionTime` appearance key.
    ComputeSelectionTime = 33,
    /// `MeshVSDrawerAttribute::DisplayNodes` appearance key.
    DisplayNodes = 34,
    /// `MeshVSDrawerAttribute::SelectableAuto` appearance key.
    SelectableAuto = 35,
    /// `MeshVSDrawerAttribute::ShowEdges` appearance key.
    ShowEdges = 36,
    /// `MeshVSDrawerAttribute::SmoothShading` appearance key.
    SmoothShading = 37,
    /// `MeshVSDrawerAttribute::SupressBackFaces` appearance key.
    SupressBackFaces = 38,
    /// `MeshVSDrawerAttribute::User` appearance key.
    User = 39,
}
// TextExpansionFactor/TextSpace/TextDisplayType have no effect; retain their keys.
/// `WireFrame` display mode; combine integer masks freely.
pub const MESHVS_DMF_WIRE_FRAME: MeshVSDisplayModeFlags = 0x0001;
/// `shading` display mode; combine integer masks freely.
pub const MESHVS_DMF_SHADING: MeshVSDisplayModeFlags = 0x0002;
/// `shrink` display mode; combine integer masks freely.
pub const MESHVS_DMF_SHRINK: MeshVSDisplayModeFlags = 0x0003;
/// `VectorDataPrs` display mode; combine integer masks freely.
pub const MESHVS_DMF_VECTOR_DATA_PRS: MeshVSDisplayModeFlags = 0x0004;
/// `NodalColorDataPrs` display mode; combine integer masks freely.
pub const MESHVS_DMF_NODAL_COLOR_DATA_PRS: MeshVSDisplayModeFlags = 0x0008;
/// `ElementalColorDataPrs` display mode; combine integer masks freely.
pub const MESHVS_DMF_ELEMENTAL_COLOR_DATA_PRS: MeshVSDisplayModeFlags = 0x0010;
/// `TextDataPrs` display mode; combine integer masks freely.
pub const MESHVS_DMF_TEXT_DATA_PRS: MeshVSDisplayModeFlags = 0x0020;
/// `EntitiesWithData` display mode; combine integer masks freely.
pub const MESHVS_DMF_ENTITIES_WITH_DATA: MeshVSDisplayModeFlags = 0x0040;
/// `DeformedPrsWireFrame` display mode; combine integer masks freely.
pub const MESHVS_DMF_DEFORMED_PRS_WIRE_FRAME: MeshVSDisplayModeFlags = 0x0080;
/// `DeformedPrsShading` display mode; combine integer masks freely.
pub const MESHVS_DMF_DEFORMED_PRS_SHADING: MeshVSDisplayModeFlags = 0x0100;
/// `DeformedPrsShrink` display mode; combine integer masks freely.
pub const MESHVS_DMF_DEFORMED_PRS_SHRINK: MeshVSDisplayModeFlags = 0x0180;
/// `SelectionPrs` display mode; combine integer masks freely.
pub const MESHVS_DMF_SELECTION_PRS: MeshVSDisplayModeFlags = 0x0200;
/// `HilightPrs` display mode; combine integer masks freely.
pub const MESHVS_DMF_HILIGHT_PRS: MeshVSDisplayModeFlags = 0x0400;
/// `user` display mode; combine integer masks freely.
pub const MESHVS_DMF_USER: MeshVSDisplayModeFlags = 0x0800;
/// Mask selecting the base display modes (wireframe, shaded, shrunk); `mode & MESHVS_DMF_BASE_MODE_MASK`.
pub const MESHVS_DMF_BASE_MODE_MASK: MeshVSDisplayModeFlags =
    MESHVS_DMF_WIRE_FRAME | MESHVS_DMF_SHADING | MESHVS_DMF_SHRINK;
/// Mask selecting deformation modes; `mode & MESHVS_DMF_DEFORMED_MASK`.
pub const MESHVS_DMF_DEFORMED_MASK: MeshVSDisplayModeFlags = MESHVS_DMF_DEFORMED_PRS_WIRE_FRAME
    | MESHVS_DMF_DEFORMED_PRS_SHADING
    | MESHVS_DMF_DEFORMED_PRS_SHRINK;
/// Entity mask `NONE`; arbitrary unnamed combinations remain representable.
pub const MESHVS_ET_NONE: MeshVSEntityType = MeshVSEntityType(0);
/// Entity mask `NODE`; arbitrary unnamed combinations remain representable.
pub const MESHVS_ET_NODE: MeshVSEntityType = MeshVSEntityType(1);
/// Entity mask `0D`; arbitrary unnamed combinations remain representable.
pub const MESHVS_ET_0D: MeshVSEntityType = MeshVSEntityType(2);
/// Entity mask `LINK`; arbitrary unnamed combinations remain representable.
pub const MESHVS_ET_LINK: MeshVSEntityType = MeshVSEntityType(4);
/// Entity mask `FACE`; arbitrary unnamed combinations remain representable.
pub const MESHVS_ET_FACE: MeshVSEntityType = MeshVSEntityType(8);
/// Entity mask `VOLUME`; arbitrary unnamed combinations remain representable.
pub const MESHVS_ET_VOLUME: MeshVSEntityType = MeshVSEntityType(16);
/// Entity mask `ELEMENT`; arbitrary unnamed combinations remain representable.
pub const MESHVS_ET_ELEMENT: MeshVSEntityType = MeshVSEntityType(30);
/// Entity mask `ALL`; arbitrary unnamed combinations remain representable.
pub const MESHVS_ET_ALL: MeshVSEntityType = MeshVSEntityType(31);
