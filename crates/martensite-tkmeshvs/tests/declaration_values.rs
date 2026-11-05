// SPDX-License-Identifier: GPL-2.0-only
//! All values replay the recorded declarations.
use martensite_tkmeshvs::flags::*;
const PINNED_DECLARATION_VALUES: &[(&str, i32)] = &[
    (
        "MeshVSDrawerAttribute::InteriorStyle",
        MeshVSDrawerAttribute::InteriorStyle as i32,
    ),
    (
        "MeshVSDrawerAttribute::InteriorColor",
        MeshVSDrawerAttribute::InteriorColor as i32,
    ),
    (
        "MeshVSDrawerAttribute::BackInteriorColor",
        MeshVSDrawerAttribute::BackInteriorColor as i32,
    ),
    (
        "MeshVSDrawerAttribute::EdgeColor",
        MeshVSDrawerAttribute::EdgeColor as i32,
    ),
    (
        "MeshVSDrawerAttribute::EdgeType",
        MeshVSDrawerAttribute::EdgeType as i32,
    ),
    (
        "MeshVSDrawerAttribute::EdgeWidth",
        MeshVSDrawerAttribute::EdgeWidth as i32,
    ),
    (
        "MeshVSDrawerAttribute::HatchStyle",
        MeshVSDrawerAttribute::HatchStyle as i32,
    ),
    (
        "MeshVSDrawerAttribute::FrontMaterial",
        MeshVSDrawerAttribute::FrontMaterial as i32,
    ),
    (
        "MeshVSDrawerAttribute::BackMaterial",
        MeshVSDrawerAttribute::BackMaterial as i32,
    ),
    (
        "MeshVSDrawerAttribute::BeamType",
        MeshVSDrawerAttribute::BeamType as i32,
    ),
    (
        "MeshVSDrawerAttribute::BeamWidth",
        MeshVSDrawerAttribute::BeamWidth as i32,
    ),
    (
        "MeshVSDrawerAttribute::BeamColor",
        MeshVSDrawerAttribute::BeamColor as i32,
    ),
    (
        "MeshVSDrawerAttribute::MarkerType",
        MeshVSDrawerAttribute::MarkerType as i32,
    ),
    (
        "MeshVSDrawerAttribute::MarkerColor",
        MeshVSDrawerAttribute::MarkerColor as i32,
    ),
    (
        "MeshVSDrawerAttribute::MarkerScale",
        MeshVSDrawerAttribute::MarkerScale as i32,
    ),
    (
        "MeshVSDrawerAttribute::TextColor",
        MeshVSDrawerAttribute::TextColor as i32,
    ),
    (
        "MeshVSDrawerAttribute::TextHeight",
        MeshVSDrawerAttribute::TextHeight as i32,
    ),
    (
        "MeshVSDrawerAttribute::TextFont",
        MeshVSDrawerAttribute::TextFont as i32,
    ),
    (
        "MeshVSDrawerAttribute::TextExpansionFactor",
        MeshVSDrawerAttribute::TextExpansionFactor as i32,
    ),
    (
        "MeshVSDrawerAttribute::TextSpace",
        MeshVSDrawerAttribute::TextSpace as i32,
    ),
    (
        "MeshVSDrawerAttribute::TextStyle",
        MeshVSDrawerAttribute::TextStyle as i32,
    ),
    (
        "MeshVSDrawerAttribute::TextDisplayType",
        MeshVSDrawerAttribute::TextDisplayType as i32,
    ),
    (
        "MeshVSDrawerAttribute::TextTexFont",
        MeshVSDrawerAttribute::TextTexFont as i32,
    ),
    (
        "MeshVSDrawerAttribute::TextFontAspect",
        MeshVSDrawerAttribute::TextFontAspect as i32,
    ),
    (
        "MeshVSDrawerAttribute::VectorColor",
        MeshVSDrawerAttribute::VectorColor as i32,
    ),
    (
        "MeshVSDrawerAttribute::VectorMaxLength",
        MeshVSDrawerAttribute::VectorMaxLength as i32,
    ),
    (
        "MeshVSDrawerAttribute::VectorArrowPart",
        MeshVSDrawerAttribute::VectorArrowPart as i32,
    ),
    (
        "MeshVSDrawerAttribute::IsAllowOverlapped",
        MeshVSDrawerAttribute::IsAllowOverlapped as i32,
    ),
    (
        "MeshVSDrawerAttribute::Reflection",
        MeshVSDrawerAttribute::Reflection as i32,
    ),
    (
        "MeshVSDrawerAttribute::ColorReflection",
        MeshVSDrawerAttribute::ColorReflection as i32,
    ),
    (
        "MeshVSDrawerAttribute::ShrinkCoeff",
        MeshVSDrawerAttribute::ShrinkCoeff as i32,
    ),
    (
        "MeshVSDrawerAttribute::MaxFaceNodes",
        MeshVSDrawerAttribute::MaxFaceNodes as i32,
    ),
    (
        "MeshVSDrawerAttribute::ComputeTime",
        MeshVSDrawerAttribute::ComputeTime as i32,
    ),
    (
        "MeshVSDrawerAttribute::ComputeSelectionTime",
        MeshVSDrawerAttribute::ComputeSelectionTime as i32,
    ),
    (
        "MeshVSDrawerAttribute::DisplayNodes",
        MeshVSDrawerAttribute::DisplayNodes as i32,
    ),
    (
        "MeshVSDrawerAttribute::SelectableAuto",
        MeshVSDrawerAttribute::SelectableAuto as i32,
    ),
    (
        "MeshVSDrawerAttribute::ShowEdges",
        MeshVSDrawerAttribute::ShowEdges as i32,
    ),
    (
        "MeshVSDrawerAttribute::SmoothShading",
        MeshVSDrawerAttribute::SmoothShading as i32,
    ),
    (
        "MeshVSDrawerAttribute::SupressBackFaces",
        MeshVSDrawerAttribute::SupressBackFaces as i32,
    ),
    (
        "MeshVSDrawerAttribute::User",
        MeshVSDrawerAttribute::User as i32,
    ),
    ("MESHVS_DMF_WIRE_FRAME", MESHVS_DMF_WIRE_FRAME),
    ("MESHVS_DMF_SHADING", MESHVS_DMF_SHADING),
    ("MESHVS_DMF_SHRINK", MESHVS_DMF_SHRINK),
    ("MESHVS_DMF_BASE_MODE_MASK", MESHVS_DMF_BASE_MODE_MASK),
    ("MESHVS_DMF_VECTOR_DATA_PRS", MESHVS_DMF_VECTOR_DATA_PRS),
    (
        "MESHVS_DMF_NODAL_COLOR_DATA_PRS",
        MESHVS_DMF_NODAL_COLOR_DATA_PRS,
    ),
    (
        "MESHVS_DMF_ELEMENTAL_COLOR_DATA_PRS",
        MESHVS_DMF_ELEMENTAL_COLOR_DATA_PRS,
    ),
    ("MESHVS_DMF_TEXT_DATA_PRS", MESHVS_DMF_TEXT_DATA_PRS),
    (
        "MESHVS_DMF_ENTITIES_WITH_DATA",
        MESHVS_DMF_ENTITIES_WITH_DATA,
    ),
    (
        "MESHVS_DMF_DEFORMED_PRS_WIRE_FRAME",
        MESHVS_DMF_DEFORMED_PRS_WIRE_FRAME,
    ),
    (
        "MESHVS_DMF_DEFORMED_PRS_SHADING",
        MESHVS_DMF_DEFORMED_PRS_SHADING,
    ),
    (
        "MESHVS_DMF_DEFORMED_PRS_SHRINK",
        MESHVS_DMF_DEFORMED_PRS_SHRINK,
    ),
    ("MESHVS_DMF_DEFORMED_MASK", MESHVS_DMF_DEFORMED_MASK),
    ("MESHVS_DMF_SELECTION_PRS", MESHVS_DMF_SELECTION_PRS),
    ("MESHVS_DMF_HILIGHT_PRS", MESHVS_DMF_HILIGHT_PRS),
    ("MESHVS_DMF_USER", MESHVS_DMF_USER),
    ("MESHVS_ET_NONE", MESHVS_ET_NONE.0),
    ("MESHVS_ET_NODE", MESHVS_ET_NODE.0),
    ("MESHVS_ET_0D", MESHVS_ET_0D.0),
    ("MESHVS_ET_LINK", MESHVS_ET_LINK.0),
    ("MESHVS_ET_FACE", MESHVS_ET_FACE.0),
    ("MESHVS_ET_VOLUME", MESHVS_ET_VOLUME.0),
    ("MESHVS_ET_ELEMENT", MESHVS_ET_ELEMENT.0),
    ("MESHVS_ET_ALL", MESHVS_ET_ALL.0),
];
#[test]
fn every_exported_meshvs_value_matches_pinned_declarations() {
    let rows = PINNED_DECLARATION_VALUES
        .iter()
        .map(|(name, value)| format!("{name} {value}\n"))
        .collect::<String>();
    assert_eq!(PINNED_DECLARATION_VALUES.len(), 64);
    assert_eq!(
        rows,
        include_str!("fixtures/declaration-values-recorded.txt")
    );
}
