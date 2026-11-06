//! Runs process-wide InterfaceStatic configuration in its own test executable.
use martensite_tkdestep::de_step::{DeStepConfigurationNode, DeStepParameters};
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
fn assert_common_values(p: &DeStepParameters) {
    assert_eq!(p.read_b_spline_continuity.as_raw(), 2);
    assert_eq!(p.read_precision_mode.as_raw(), 1);
    assert_eq!(p.read_precision_val, 0.125);
    assert_eq!(p.read_max_precision_mode.as_raw(), 1);
    assert_eq!(p.read_max_precision_val, 2.5);
    assert!(!p.read_same_param_mode);
    assert_eq!(p.read_surface_curve_mode.as_raw(), -3);
    assert_eq!(p.encode_reg_angle, 0.5 * 180.0 / std::f64::consts::PI);
    assert_eq!(p.angle_unit.as_raw(), 2);
}
fn assert_read_values(p: &DeStepParameters) {
    assert!(
        !p.read_product_mode
            && !p.read_relationship
            && !p.read_shape_aspect
            && !p.read_root_transformation
    );
    assert_eq!(p.read_product_context.as_raw(), 3);
    assert_eq!(p.read_shape_repr.as_raw(), 7);
    assert_eq!(p.read_tessellated.as_raw(), 2);
    assert_eq!(p.read_assembly_level.as_raw(), 4);
    assert!(
        p.read_constr_relation
            && p.read_subshape_names
            && p.read_nonmanifold
            && p.read_ideas
            && p.read_all_shapes
    );
    assert_eq!(p.read_code_page, 26);
}
fn assert_write_values(p: &DeStepParameters) {
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
    assert!(!p.write_color && !p.write_name && !p.write_layer && !p.write_props);
    assert_eq!(p.write_model_type.as_raw(), 7);
}
fn assert_untouched_fields(p: &DeStepParameters) {
    assert!(p.read_color && p.read_name && p.read_layer && p.read_props && p.read_metadata);
    assert!(!p.read_product_metadata);
    assert!(p.write_metadata && p.write_material && p.write_scaling_trsf);
    assert!(!p.write_vis_material && !p.clean_duplicates);
}
fn assert_untouched_mutations_survive(p: &mut DeStepParameters) {
    p.read_color = false;
    p.read_name = false;
    p.read_layer = false;
    p.read_props = false;
    p.read_metadata = false;
    p.read_product_metadata = true;
    p.write_metadata = false;
    p.write_material = false;
    p.write_scaling_trsf = false;
    p.write_vis_material = true;
    p.clean_duplicates = true;
    p.init_from_static();
    assert!(!p.read_color && !p.read_name && !p.read_layer && !p.read_props && !p.read_metadata);
    assert!(p.read_product_metadata && p.write_vis_material && p.clean_duplicates);
    assert!(!p.write_metadata && !p.write_material && !p.write_scaling_trsf);
}
fn assert_angle_units_and_strict_boolean_reads() {
    for (text, expected) in [
        ("0", 0.0),
        ("1.5707963267948966", 90.0),
        ("-3.141592653589793", -180.0),
    ] {
        assert!(InterfaceStatic::set_c_val("read.encoderegularity.angle", text).unwrap());
        let mut p = DeStepParameters::default();
        p.init_from_static();
        assert_eq!(p.encode_reg_angle, expected);
    }
    for raw in [-2, 0, 1, 2] {
        assert!(InterfaceStatic::set_i_val("write.surfacecurve.mode", raw).unwrap());
        let mut p = DeStepParameters::default();
        p.init_from_static();
        assert_eq!(p.write_surface_cur_mode, raw == 1);
    }
}
fn assert_empty_registry_zeroes_only_source_touched_fields() {
    let mut p = DeStepParameters::default();
    p.init_from_static();
    assert_eq!(p.read_precision_val, 0.0);
    assert_eq!(p.read_max_precision_val, 0.0);
    assert_eq!(p.encode_reg_angle, 0.0);
    assert_eq!(p.read_product_context.as_raw(), 0);
    assert!(!p.read_product_mode && !p.write_color);
    assert_untouched_fields(&p);
}
#[test]
fn init_from_static_keeps_units_and_untouched_fields_in_isolated_process() {
    assert_empty_registry_zeroes_only_source_touched_fields();
    register_current_values();
    let node = DeStepConfigurationNode::default();
    node.internal_parameters.write().init_from_static();
    assert_common_values(&node.internal_parameters.snapshot());
    assert_read_values(&node.internal_parameters.snapshot());
    assert_write_values(&node.internal_parameters.snapshot());
    assert_untouched_fields(&node.internal_parameters.snapshot());
    assert_eq!(
        node.save().as_bytes(),
        include_bytes!("fixtures/static_loaded.conf")
    );
    assert_untouched_mutations_survive(&mut node.internal_parameters.write());
    assert_angle_units_and_strict_boolean_reads();
}
