// SPDX-License-Identifier: GPL-2.0-only
//! Recorded declaration fixtures.
use martensite_tkmeshvs::selection_policy::*;
use std::mem::size_of;

const NAMED_POLICIES: &[(&str, i32)] = &[
    ("MESHVS_BP_MESH", MESHVS_BP_MESH),
    ("MESHVS_BP_NODAL_COLOR", MESHVS_BP_NODAL_COLOR),
    ("MESHVS_BP_ELEM_COLOR", MESHVS_BP_ELEM_COLOR),
    ("MESHVS_BP_TEXT", MESHVS_BP_TEXT),
    ("MESHVS_BP_VECTOR", MESHVS_BP_VECTOR),
    ("MESHVS_BP_USER", MESHVS_BP_USER),
    ("MESHVS_BP_DEFAULT", MESHVS_BP_DEFAULT),
    ("MESHVS_SMF_MESH", MESHVS_SMF_MESH.0),
    ("MESHVS_SMF_NODE", MESHVS_SMF_NODE.0),
    ("MESHVS_SMF_0D", MESHVS_SMF_0D.0),
    ("MESHVS_SMF_LINK", MESHVS_SMF_LINK.0),
    ("MESHVS_SMF_FACE", MESHVS_SMF_FACE.0),
    ("MESHVS_SMF_VOLUME", MESHVS_SMF_VOLUME.0),
    ("MESHVS_SMF_ELEMENT", MESHVS_SMF_ELEMENT.0),
    ("MESHVS_SMF_ALL", MESHVS_SMF_ALL.0),
    ("MESHVS_SMF_GROUP", MESHVS_SMF_GROUP.0),
    (
        "MeshVSMeshSelectionMethod::Precise",
        MeshVSMeshSelectionMethod::Precise as i32,
    ),
    (
        "MeshVSMeshSelectionMethod::Nodes",
        MeshVSMeshSelectionMethod::Nodes as i32,
    ),
    (
        "MeshVSMeshSelectionMethod::Box",
        MeshVSMeshSelectionMethod::Box as i32,
    ),
    (
        "MeshVSMeshSelectionMethod::unnamed3",
        MeshVSMeshSelectionMethod::Unnamed3 as i32,
    ),
];

#[test]
fn all_named_values_and_default_priority_alias_match() {
    let rows = recorded_policy_rows();
    assert_eq!(NAMED_POLICIES.len(), 20);
    for ((name, value), recorded) in NAMED_POLICIES.iter().zip(&rows[..20]) {
        assert_eq!(*recorded, format!("{name} {value}"));
    }
    let negative_priority: MeshVSBuilderPriority = -7;
    assert!(negative_priority < MESHVS_BP_MESH);
    assert_eq!(MESHVS_BP_DEFAULT, MESHVS_BP_USER);
}

#[test]
fn scalar_layout_matches_each_declared_type() {
    let rows = recorded_policy_rows();
    let layouts = [
        ("priority", size_of::<MeshVSBuilderPriority>()),
        ("selection_mask", size_of::<MeshVSSelectionModeFlags>()),
        ("selection_method", size_of::<MeshVSMeshSelectionMethod>()),
    ];
    for ((name, size), recorded) in layouts.iter().zip(&rows[20..23]) {
        assert_eq!(*recorded, format!("sizeof_{name} {size}"));
    }
}

#[test]
fn every_mask_keeps_group_and_entity_combinations() {
    let rows = recorded_policy_rows();
    for (value, recorded) in (0..512).zip(&rows[23..]) {
        let flags = MeshVSSelectionModeFlags(value);
        assert_eq!(*recorded, format!("mask {value} {}", flags.0));
    }
    assert_eq!(MESHVS_SMF_ALL.0, MESHVS_SMF_ELEMENT.0 | MESHVS_SMF_NODE.0);
    let grouped_faces = MeshVSSelectionModeFlags(MESHVS_SMF_GROUP.0 | MESHVS_SMF_FACE.0);
    assert_eq!(grouped_faces.0, 264);
}

#[test]
fn mesh_selection_conversion_preserves_unnamed_value_without_coercion() {
    for value in 0..4 {
        assert_eq!(mesh_selection_method_from_raw(value).unwrap() as i32, value);
    }
    for outside_declared_range in [-1, 4, i32::MIN, i32::MAX] {
        assert_eq!(mesh_selection_method_from_raw(outside_declared_range), None);
    }
}

fn recorded_policy_rows() -> Vec<&'static str> {
    let recorded = concat!(
        include_str!("fixtures/selection-policy-recorded.txt"),
        include_str!("fixtures/selection-policy-masks-0-recorded.txt"),
        include_str!("fixtures/selection-policy-masks-1-recorded.txt")
    );
    let rows: Vec<&str> = recorded.lines().collect();
    assert_eq!(rows.len(), 535);
    rows
}

#[test]
fn conversion_recovers_after_rejected_values() {
    for raw_value in [0, -1, 1, 4, 2, i32::MAX, 3, i32::MIN, 0] {
        let converted = mesh_selection_method_from_raw(raw_value);
        assert_eq!(
            converted.map(|method| method as i32),
            (0..4).contains(&raw_value).then_some(raw_value)
        );
    }
}
