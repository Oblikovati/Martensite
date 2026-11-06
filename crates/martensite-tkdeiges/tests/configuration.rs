//! Independent IGES configuration, healing and start-record contracts.
use martensite_tkde::de::{DeConfigurationContext, DeSectionGlobal, DeShapeFixParameters};
use martensite_tkdeiges::de_iges::{
    DeIgesConfigurationNode, DeIgesParameters, DeIgesReadModeBSplineContinuity,
    DeIgesReadModeMaxPrecision, DeIgesReadModePrecision, DeIgesReadModeSurfaceCurve,
    DeIgesWriteModeBRep, DeIgesWriteModeConvertSurface, DeIgesWriteModePlaneMode,
    DeIgesWriteModePrecisionMode,
};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
fn context(text: &str) -> DeConfigurationContext {
    let resource = DeConfigurationContext::new();
    assert!(resource.load_str(&text.into()).unwrap());
    resource
}
fn changed_node() -> DeIgesConfigurationNode {
    let node = DeIgesConfigurationNode::default();
    assert!(node.load(&context(include_str!("fixtures/changed.conf"))));
    node
}
#[test]
fn initial_precision_and_read_modes_are_exact() {
    let p = DeIgesParameters::default();
    assert_eq!(
        p.read_b_spline_continuity,
        DeIgesReadModeBSplineContinuity::C1
    );
    assert_eq!(p.read_precision_mode, DeIgesReadModePrecision::FILE);
    assert_eq!(p.read_precision_val, 0.0001);
    assert_eq!(
        p.read_max_precision_mode,
        DeIgesReadModeMaxPrecision::PREFERRED
    );
    assert_eq!(p.read_max_precision_val, 1.0);
    assert_eq!(
        p.read_surface_curve_mode,
        DeIgesReadModeSurfaceCurve::DEFAULT
    );
    assert_eq!(p.encode_reg_angle, 0.57295779513);
}
#[test]
fn initial_read_flags_are_exact() {
    let p = DeIgesParameters::default();
    assert!(
        !p.read_same_param_mode
            && !p.read_approxd1
            && !p.read_faulty_entities
            && !p.read_only_visible
    );
    assert!(p.read_color && p.read_name && p.read_layer);
}
#[test]
fn initial_write_modes_and_headers_are_exact() {
    let p = DeIgesParameters::default();
    assert_eq!(p.write_b_rep_mode, DeIgesWriteModeBRep::FACES);
    assert_eq!(
        p.write_convert_surface_mode,
        DeIgesWriteModeConvertSurface::OFF
    );
    assert_eq!(
        p.write_precision_mode,
        DeIgesWriteModePrecisionMode::AVERAGE
    );
    assert_eq!(p.write_precision_val, 0.0001);
    assert_eq!(p.write_plane_mode, DeIgesWriteModePlaneMode::PLANE);
    assert!(!p.write_offset_mode && p.write_color && p.write_name && p.write_layer);
    assert!(p.write_header_author.is_empty() && p.write_header_company.is_empty());
    assert!(p.write_header_product.is_empty() && p.write_header_reciever.is_empty());
}
#[test]
fn default_healing_keeps_the_four_iges_specific_values() {
    let expected = DeIgesParameters::default_shape_fix_parameters();
    assert_eq!(expected.detalization_level, ShapeEnum::Edge);
    assert_eq!(expected.create_open_solid_mode.0, 1);
    assert_eq!(expected.fix_tail_mode.0, -1);
    assert_eq!(expected.max_tail_angle.0, -1);
    assert_eq!(
        DeIgesConfigurationNode::default().shape_fix_parameters(),
        expected
    );
}
#[test]
fn changed_read_keys_all_reach_their_named_fields() {
    let p = changed_node().internal_parameters.snapshot();
    assert_eq!(p.read_b_spline_continuity.as_raw(), 2);
    assert_eq!(p.read_precision_mode.as_raw(), 1);
    assert_eq!(p.read_precision_val, 0.125);
    assert_eq!(p.read_max_precision_mode.as_raw(), 1);
    assert_eq!(p.read_max_precision_val, 2.5);
    assert_eq!(p.read_surface_curve_mode.as_raw(), -3);
    assert_eq!(p.encode_reg_angle, 12.5);
    assert!(
        p.read_same_param_mode && p.read_approxd1 && p.read_faulty_entities && p.read_only_visible
    );
    assert!(!p.read_color && !p.read_name && !p.read_layer);
}
#[test]
fn changed_write_keys_all_reach_their_named_fields() {
    let p = changed_node().internal_parameters.snapshot();
    assert_eq!(p.write_b_rep_mode.as_raw(), 1);
    assert_eq!(p.write_convert_surface_mode.as_raw(), 1);
    assert_eq!(p.write_header_author, AsciiString::new("author"));
    assert_eq!(p.write_header_company, AsciiString::new("company"));
    assert_eq!(p.write_header_product, AsciiString::new("product"));
    assert_eq!(p.write_header_reciever, AsciiString::new("receiver"));
    assert_eq!(p.write_precision_mode.as_raw(), -1);
    assert_eq!(p.write_precision_val, 0.25);
    assert_eq!(p.write_plane_mode.as_raw(), 1);
    assert!(p.write_offset_mode && !p.write_color && !p.write_name && !p.write_layer);
}
#[test]
fn healing_follows_format_load_and_all_seventy_nine_keys_roundtrip() {
    let source = changed_node();
    assert_eq!(source.shape_fix_parameters().tolerance3d, 0.0254);
    assert_eq!(source.shape_fix_parameters().fix_free_shell_mode.0, 7);
    let saved = source.save();
    let resource = context(std::str::from_utf8(saved.as_bytes()).unwrap());
    assert_eq!(resource.get_internal_map().extent(), 79);
    let restored = DeIgesConfigurationNode::default();
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
fn default_text_matches_complete_reviewed_baseline() {
    assert_eq!(
        DeIgesConfigurationNode::default().save().as_bytes(),
        include_bytes!("fixtures/default.conf")
    );
}
#[test]
fn malformed_and_missing_values_preserve_state() {
    let node = changed_node();
    let expected = node.internal_parameters.snapshot();
    let healing = node.shape_fix_parameters();
    assert!(node.load(&context("provider.IGES.Martensite.read.precision.val : word\nprovider.IGES.Martensite.write.plane.mode : 1.5\nprovider.IGES.Martensite.healing.free.shell : no\n")));
    assert_eq!(node.internal_parameters.snapshot(), expected);
    assert_eq!(node.shape_fix_parameters(), healing);
    assert!(node.load(&DeConfigurationContext::new()));
    assert_eq!(node.internal_parameters.snapshot(), expected);
    assert_eq!(node.shape_fix_parameters(), healing);
}
#[test]
fn copies_keep_node_format_and_healing_state_independent() {
    let node = changed_node();
    node.set_enabled(false);
    node.set_global_parameters(DeSectionGlobal {
        length_unit: 2.0,
        system_unit: 3.0,
    });
    let copy = DeIgesConfigurationNode::copied_from(&node);
    node.internal_parameters.write().reset();
    node.set_shape_fix_parameters(DeShapeFixParameters::default());
    node.set_enabled(true);
    assert_eq!(
        node.internal_parameters.snapshot(),
        DeIgesParameters::default()
    );
    assert!(!copy.is_enabled());
    assert_eq!(copy.global_parameters().system_unit, 3.0);
    assert_eq!(
        copy.internal_parameters.snapshot().write_header_author,
        AsciiString::new("author")
    );
    assert_eq!(copy.shape_fix_parameters().tolerance3d, 0.0254);
}
#[test]
fn metadata_identifies_both_iges_suffixes() {
    let node = DeIgesConfigurationNode::default();
    assert!(node.is_import_supported() && node.is_export_supported());
    assert_eq!(node.get_format(), AsciiString::new("IGES"));
    assert_eq!(node.get_vendor(), AsciiString::new("Martensite"));
    assert_eq!(
        node.get_extensions().iter().cloned().collect::<Vec<_>>(),
        vec![AsciiString::new("igs"), AsciiString::new("iges")]
    );
}
fn start_record(digit: usize, trailing: u8) -> Vec<u8> {
    let mut bytes = vec![b' '; 83];
    bytes[72] = b'S';
    bytes[digit] = b'1';
    bytes[digit + 1] = trailing;
    bytes
}
#[test]
fn start_record_scan_respects_padding_bounds_and_alphanumeric_tail() {
    let node = DeIgesConfigurationNode::default();
    assert!(!node.check_content(None));
    assert!(!node.check_content(Some(&[b' '; 82])));
    for digit in 73..=80 {
        assert!(node.check_content(Some(&start_record(digit, b' '))));
        assert!(node.check_content(Some(&start_record(digit, b'!'))));
        assert!(!node.check_content(Some(&start_record(digit, b'a'))));
        assert!(!node.check_content(Some(&start_record(digit, b'0'))));
    }
    assert!(!node.check_content(Some(&start_record(81, b' '))));
    let mut zero_padded = start_record(79, b'\n');
    zero_padded[73..79].fill(b'0');
    assert!(node.check_content(Some(&zero_padded)));
    zero_padded[72] = b's';
    assert!(!node.check_content(Some(&zero_padded)));
}
#[test]
fn parameter_and_node_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DeIgesConfigurationNode>();
    assert_send_sync::<DeIgesParameters>();
}
#[test]
fn surface_curve_constants_keep_both_forced_negative_values() {
    assert_eq!(DeIgesReadModeSurfaceCurve::DEFAULT.as_raw(), 0);
    assert_eq!(DeIgesReadModeSurfaceCurve::USE_2D_PREFERRED.as_raw(), 2);
    assert_eq!(DeIgesReadModeSurfaceCurve::USE_2D_FORCED.as_raw(), -2);
    assert_eq!(DeIgesReadModeSurfaceCurve::USE_3D_PREFERRED.as_raw(), 3);
    assert_eq!(DeIgesReadModeSurfaceCurve::USE_3D_FORCED.as_raw(), -3);
    for value in [i32::MIN, -3, 0, 3, i32::MAX] {
        assert_eq!(DeIgesReadModeSurfaceCurve::from_raw(value).as_raw(), value);
    }
}
#[test]
fn read_enum_names_have_exact_integer_values() {
    assert_eq!(DeIgesReadModeBSplineContinuity::C0.as_raw(), 0);
    assert_eq!(DeIgesReadModeBSplineContinuity::C1.as_raw(), 1);
    assert_eq!(DeIgesReadModeBSplineContinuity::C2.as_raw(), 2);
    assert_eq!(DeIgesReadModePrecision::FILE.as_raw(), 0);
    assert_eq!(DeIgesReadModePrecision::USER.as_raw(), 1);
    assert_eq!(DeIgesReadModeMaxPrecision::PREFERRED.as_raw(), 0);
    assert_eq!(DeIgesReadModeMaxPrecision::FORCED.as_raw(), 1);
}
#[test]
fn write_enum_names_have_exact_integer_values() {
    assert_eq!(DeIgesWriteModeBRep::FACES.as_raw(), 0);
    assert_eq!(DeIgesWriteModeBRep::B_REP.as_raw(), 1);
    assert_eq!(DeIgesWriteModeConvertSurface::OFF.as_raw(), 0);
    assert_eq!(DeIgesWriteModeConvertSurface::ON.as_raw(), 1);
    assert_eq!(DeIgesWriteModePrecisionMode::LEAST.as_raw(), -1);
    assert_eq!(DeIgesWriteModePrecisionMode::AVERAGE.as_raw(), 0);
    assert_eq!(DeIgesWriteModePrecisionMode::GREATEST.as_raw(), 1);
    assert_eq!(DeIgesWriteModePrecisionMode::SESSION.as_raw(), 2);
    assert_eq!(DeIgesWriteModePlaneMode::PLANE.as_raw(), 0);
    assert_eq!(DeIgesWriteModePlaneMode::B_SPLINE.as_raw(), 1);
}
