//! Independent STEP configuration and signature contracts.
use martensite_tkde::de::{DeConfigurationContext, DeSectionGlobal, DeShapeFixParameters};
use martensite_tkdestep::de_step::*;
use martensite_tkernel::tcollection::AsciiString;
fn context(text: &str) -> DeConfigurationContext {
    let resource = DeConfigurationContext::new();
    assert!(resource.load_str(&text.into()).unwrap());
    resource
}
fn changed_node() -> DeStepConfigurationNode {
    let node = DeStepConfigurationNode::default();
    assert!(node.load(&context(include_str!("fixtures/changed.conf"))));
    node
}
#[test]
fn common_defaults_and_step_healing_are_exact() {
    let p = DeStepParameters::default();
    assert_eq!(p.read_b_spline_continuity.as_raw(), 1);
    assert_eq!(p.read_precision_mode.as_raw(), 0);
    assert_eq!(p.read_precision_val, 0.0001);
    assert_eq!(p.read_max_precision_mode.as_raw(), 0);
    assert_eq!(p.read_max_precision_val, 1.0);
    assert!(!p.read_same_param_mode);
    assert_eq!(p.read_surface_curve_mode.as_raw(), 0);
    assert_eq!(p.encode_reg_angle, 0.57295779513);
    assert_eq!(p.angle_unit.as_raw(), 0);
    assert_eq!(
        DeStepParameters::default_shape_fix_parameters(),
        DeShapeFixParameters::default()
    );
    assert_eq!(
        DeStepConfigurationNode::default().shape_fix_parameters(),
        DeShapeFixParameters::default()
    );
}
#[test]
fn read_product_and_shape_defaults_are_exact() {
    let p = DeStepParameters::default();
    assert!(p.read_product_mode && p.read_relationship && p.read_shape_aspect);
    assert_eq!(p.read_product_context.as_raw(), 1);
    assert_eq!(p.read_shape_repr.as_raw(), 1);
    assert_eq!(p.read_tessellated.as_raw(), 1);
    assert_eq!(p.read_assembly_level.as_raw(), 1);
    assert!(!p.read_constr_relation && !p.read_subshape_names);
    assert_eq!(p.read_code_page, 4);
    assert!(!p.read_nonmanifold && !p.read_ideas && !p.read_all_shapes);
}
#[test]
fn read_document_defaults_are_exact() {
    let p = DeStepParameters::default();
    assert!(
        p.read_root_transformation
            && p.read_color
            && p.read_name
            && p.read_layer
            && p.read_props
            && p.read_metadata
    );
    assert!(!p.read_product_metadata);
}
#[test]
fn write_mode_defaults_are_exact() {
    let p = DeStepParameters::default();
    assert_eq!(p.write_precision_mode.as_raw(), 0);
    assert_eq!(p.write_precision_val, 0.0001);
    assert_eq!(p.write_assembly.as_raw(), 2);
    assert_eq!(p.write_schema.as_raw(), 4);
    assert_eq!(p.write_tessellated.as_raw(), 2);
    assert!(p.write_product_name.is_empty() && p.write_surface_cur_mode);
    assert_eq!(p.write_unit, 2);
    assert_eq!(p.write_vertex_mode.as_raw(), 0);
    assert!(!p.write_subshape_names && !p.write_nonmanifold);
}
#[test]
fn write_document_defaults_are_exact() {
    let p = DeStepParameters::default();
    assert!(
        p.write_color
            && p.write_name
            && p.write_layer
            && p.write_props
            && p.write_metadata
            && p.write_material
            && p.write_scaling_trsf
    );
    assert!(!p.write_vis_material && !p.clean_duplicates);
    assert_eq!(p.write_model_type.as_raw(), 0);
}
#[test]
fn all_changed_common_fields_reach_their_named_values() {
    let p = changed_node().internal_parameters.snapshot();
    assert_eq!(p.read_b_spline_continuity.as_raw(), 2);
    assert_eq!(p.read_precision_mode.as_raw(), 1);
    assert_eq!(p.read_precision_val, 0.125);
    assert_eq!(p.read_max_precision_mode.as_raw(), 1);
    assert_eq!(p.read_max_precision_val, 2.5);
    assert!(p.read_same_param_mode);
    assert_eq!(p.read_surface_curve_mode.as_raw(), -3);
    assert_eq!(p.encode_reg_angle, 12.5);
    assert_eq!(p.angle_unit.as_raw(), 2);
}
#[test]
fn all_changed_product_and_shape_fields_reach_their_named_values() {
    let p = changed_node().internal_parameters.snapshot();
    assert!(!p.read_product_mode && !p.read_relationship && !p.read_shape_aspect);
    assert_eq!(p.read_product_context.as_raw(), 3);
    assert_eq!(p.read_shape_repr.as_raw(), 7);
    assert_eq!(p.read_tessellated.as_raw(), 2);
    assert_eq!(p.read_assembly_level.as_raw(), 4);
    assert!(p.read_constr_relation && p.read_subshape_names);
    assert_eq!(p.read_code_page, 26);
    assert!(p.read_nonmanifold && p.read_ideas && p.read_all_shapes);
}
#[test]
fn all_changed_read_document_flags_reach_their_named_values() {
    let p = changed_node().internal_parameters.snapshot();
    assert!(
        !p.read_root_transformation
            && !p.read_color
            && !p.read_name
            && !p.read_layer
            && !p.read_props
            && !p.read_metadata
    );
    assert!(p.read_product_metadata);
}
#[test]
fn all_changed_write_modes_reach_their_named_values() {
    let p = changed_node().internal_parameters.snapshot();
    assert_eq!(p.write_precision_mode.as_raw(), -1);
    assert_eq!(p.write_precision_val, 0.25);
    assert_eq!(p.write_assembly.as_raw(), 1);
    assert_eq!(p.write_schema.as_raw(), 5);
    assert_eq!(p.write_tessellated.as_raw(), 1);
    assert_eq!(p.write_product_name, AsciiString::new("product"));
    assert!(!p.write_surface_cur_mode);
    assert_eq!(p.write_unit, 6);
    assert_eq!(p.write_vertex_mode.as_raw(), 1);
    assert!(p.write_subshape_names && p.write_nonmanifold);
}
#[test]
fn all_changed_write_document_flags_reach_their_named_values() {
    let p = changed_node().internal_parameters.snapshot();
    assert!(
        !p.write_color
            && !p.write_name
            && !p.write_layer
            && !p.write_props
            && !p.write_metadata
            && !p.write_material
            && !p.write_scaling_trsf
    );
    assert!(p.write_vis_material && p.clean_duplicates);
    assert_eq!(p.write_model_type.as_raw(), 7);
}
#[test]
fn complete_changed_resource_has_one_hundred_two_keys_and_roundtrips() {
    let source = changed_node();
    assert_eq!(source.shape_fix_parameters().tolerance3d, 0.0254);
    assert_eq!(source.shape_fix_parameters().fix_free_shell_mode.0, 7);
    let saved = source.save();
    let resource = context(std::str::from_utf8(saved.as_bytes()).unwrap());
    assert_eq!(resource.get_internal_map().extent(), 102);
    let restored = DeStepConfigurationNode::default();
    assert!(restored.load(&resource));
    assert_eq!(
        restored.internal_parameters.snapshot(),
        source.internal_parameters.snapshot()
    );
    assert_eq!(
        restored.shape_fix_parameters(),
        source.shape_fix_parameters()
    );
}
#[test]
fn full_default_resource_matches_reviewed_text() {
    assert_eq!(
        DeStepConfigurationNode::default().save().as_bytes(),
        include_bytes!("fixtures/default.conf")
    );
}
#[test]
fn context_labels_reject_unnamed_modes_without_fallback() {
    let p = DeStepParameters::default();
    assert_eq!(
        p.get_string(DeStepReadModeProductContext::ALL),
        AsciiString::new("all")
    );
    assert_eq!(
        p.get_string(DeStepReadModeProductContext::DESIGN),
        AsciiString::new("design")
    );
    assert_eq!(
        p.get_string(DeStepReadModeProductContext::ANALYSIS),
        AsciiString::new("analysis")
    );
}
#[test]
fn context_labels_for_unnamed_modes_are_empty() {
    let p = DeStepParameters::default();
    for raw in [-1, 0, 4, i32::MAX] {
        assert!(
            p.get_string(DeStepReadModeProductContext::from_raw(raw))
                .is_empty()
        );
    }
}
#[test]
fn missing_malformed_and_out_of_named_range_values_preserve_integer_semantics() {
    let node = changed_node();
    let before = node.internal_parameters.snapshot().clone();
    assert!(node.load(&context(
        "provider.STEP.Martensite.read.precision.val : nope\nprovider.STEP.Martensite.write.unit : 1.5\n"
    )));
    assert_eq!(node.internal_parameters.snapshot(), before);
    assert!(node.load(&DeConfigurationContext::new()));
    assert_eq!(node.internal_parameters.snapshot(), before);
    assert!(node.load(&context("provider.STEP.Martensite.read.codepage : 99\nprovider.STEP.Martensite.write.unit : -3\nprovider.STEP.Martensite.write.schema : 12\n")));
    assert_eq!(node.internal_parameters.snapshot().read_code_page, 99);
    assert_eq!(node.internal_parameters.snapshot().write_unit, -3);
    assert_eq!(
        node.internal_parameters.snapshot().write_schema.as_raw(),
        12
    );
}
#[test]
fn copy_and_reset_keep_node_format_and_healing_independent() {
    let node = changed_node();
    node.set_enabled(false);
    node.set_global_parameters(DeSectionGlobal {
        length_unit: 2.0,
        system_unit: 3.0,
    });
    let copied = DeStepConfigurationNode::copied_from(&node);
    node.internal_parameters.write().reset();
    node.set_shape_fix_parameters(DeShapeFixParameters::default());
    node.set_enabled(true);
    assert_eq!(
        node.internal_parameters.snapshot(),
        DeStepParameters::default()
    );
    assert_eq!(
        copied.internal_parameters.snapshot().write_product_name,
        AsciiString::new("product")
    );
    assert_eq!(copied.shape_fix_parameters().tolerance3d, 0.0254);
    assert!(!copied.is_enabled());
    assert_eq!(copied.global_parameters().system_unit, 3.0);
}
#[test]
fn metadata_reports_three_suffixes_and_both_directions() {
    let node = DeStepConfigurationNode::default();
    assert!(node.is_import_supported() && node.is_export_supported());
    assert_eq!(node.get_format(), AsciiString::new("STEP"));
    assert_eq!(node.get_vendor(), AsciiString::new("Martensite"));
    assert_eq!(
        node.get_extensions().iter().cloned().collect::<Vec<_>>(),
        vec![
            AsciiString::new("stp"),
            AsciiString::new("step"),
            AsciiString::new("stpz")
        ]
    );
}
#[test]
fn stream_capability_is_independent_of_enabled_state_and_loaded_settings() {
    let node = DeStepConfigurationNode::default();
    assert!(node.is_stream_supported());
    node.set_enabled(false);
    assert!(node.is_stream_supported());
    assert!(node.load(&context(include_str!("fixtures/changed.conf"))));
    assert!(node.is_stream_supported());
    assert!(DeStepConfigurationNode::copied_from(&node).is_stream_supported());
}
fn signature(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    bytes.resize(100, b' ');
    bytes
}
#[test]
fn step_signature_needs_both_tokens_and_rejects_ifc_before_nul() {
    let node = DeStepConfigurationNode::default();
    assert!(!node.check_content(None));
    assert!(!node.check_content(Some(b"ISO-10303-21 FILE_SCHEMA")));
    assert!(node.check_content(Some(&signature(b"ISO-10303-21 FILE_SCHEMA"))));
    assert!(node.check_content(Some(&signature(b"FILE_SCHEMA ISO-10303-21"))));
    assert!(!node.check_content(Some(&signature(b"ISO-10303-21"))));
    assert!(!node.check_content(Some(&signature(b"FILE_SCHEMA"))));
    assert!(!node.check_content(Some(&signature(b"ISO-10303-21 FILE_SCHEMA IFC"))));
    assert!(!node.check_content(Some(&signature(b"ISO-10303-21\0 FILE_SCHEMA"))));
    assert!(node.check_content(Some(&signature(b"ISO-10303-21 FILE_SCHEMA\0IFC"))));
    assert!(!node.check_content(Some(&signature(b"iso-10303-21 FILE_SCHEMA"))));
}
#[test]
fn all_public_configuration_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DeStepConfigurationNode>();
    assert_send_sync::<DeStepParameters>();
    assert_send_sync::<StepControlStepModelType>();
}
