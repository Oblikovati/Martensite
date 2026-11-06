//! Runs process-wide InterfaceStatic configuration in its own test executable.
use martensite_tkdeiges::de_iges::{DeIgesConfigurationNode, DeIgesParameters};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkxsbase::interface::InterfaceStatic;

fn register_current_values() {
    for line in include_str!("fixtures/static_values.tsv").lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        assert_eq!(fields.len(), 3);
        assert!(
            InterfaceStatic::init_char(
                "MartensiteStaticTest",
                fields[1],
                fields[0].as_bytes()[0],
                fields[2]
            )
            .unwrap()
        );
    }
}
fn assert_empty_registry_replaces_defaults() {
    let mut p = DeIgesParameters::default();
    p.init_from_static();
    assert_eq!(p.read_b_spline_continuity.as_raw(), 0);
    assert_eq!(p.read_precision_val, 0.0);
    assert_eq!(p.read_max_precision_val, 0.0);
    assert_eq!(p.encode_reg_angle, 0.0);
    assert!(!p.read_color && !p.read_name && !p.read_layer);
    assert!(!p.write_color && !p.write_name && !p.write_layer);
    assert!(p.write_header_author.is_empty());
}
fn assert_numeric_values(p: &DeIgesParameters) {
    assert_eq!(p.read_b_spline_continuity.as_raw(), 2);
    assert_eq!(p.read_precision_mode.as_raw(), 1);
    assert_eq!(p.read_precision_val, 0.125);
    assert_eq!(p.read_max_precision_mode.as_raw(), 1);
    assert_eq!(p.read_max_precision_val, 2.5);
    assert_eq!(p.read_surface_curve_mode.as_raw(), -3);
    assert_eq!(p.encode_reg_angle, 12.5);
    assert_eq!(p.write_b_rep_mode.as_raw(), 1);
    assert_eq!(p.write_convert_surface_mode.as_raw(), 1);
    assert_eq!(p.write_precision_mode.as_raw(), -1);
    assert_eq!(p.write_precision_val, 0.25);
    assert_eq!(p.write_plane_mode.as_raw(), 1);
}
fn assert_boolean_and_header_values(p: &DeIgesParameters) {
    assert!(!p.read_same_param_mode && !p.read_approxd1 && !p.write_offset_mode);
    assert!(p.read_faulty_entities && p.read_only_visible && p.write_color);
    assert!(!p.read_color && !p.read_name && !p.read_layer);
    assert!(!p.write_name && !p.write_layer);
    assert_eq!(p.write_header_author, AsciiString::new("author"));
    assert_eq!(p.write_header_company, AsciiString::new("company"));
    assert_eq!(p.write_header_product, AsciiString::new("product"));
    assert_eq!(p.write_header_reciever, AsciiString::new("receiver"));
}
fn assert_boolean_reads_require_exactly_one() {
    for value in ["-2", "0", "1", "2"] {
        assert!(
            InterfaceStatic::set_i_val("read.stdsameparameter.mode", value.parse().unwrap())
                .unwrap()
        );
        let mut p = DeIgesParameters::default();
        p.init_from_static();
        assert_eq!(p.read_same_param_mode, value == "1");
    }
}
#[test]
fn init_from_static_reads_all_current_parameters_in_isolated_process() {
    assert_empty_registry_replaces_defaults();
    register_current_values();
    let node = DeIgesConfigurationNode::default();
    node.internal_parameters.write().init_from_static();
    assert_numeric_values(&node.internal_parameters.snapshot());
    assert_boolean_and_header_values(&node.internal_parameters.snapshot());
    assert_eq!(
        node.save().as_bytes(),
        include_bytes!("fixtures/static_loaded.conf")
    );
    assert_boolean_reads_require_exactly_one();
}
