//! The location tool, the basic editor and the start lines of a model.
use super::{
    IgesDataBasicEditor, IgesDataIgesModel, IgesDataProtocolOps, IgesDataToolLocation,
    iges_data_protocol,
};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{GeneralTransform3, Xyz};
use martensite_tkxsbase::interface::InterfaceInterfaceModel;

#[test]
fn the_tool_converts_locations_of_a_similarity_only() {
    let mut result = martensite_tkmath::gp::Transform3::default();
    let mut general = GeneralTransform3::default();
    assert!(IgesDataToolLocation::convert_location(1e-5, &general, &mut result, 1.0).unwrap());
    assert!(result.is_identity());
    general.set_value(1, 4, 5.0).unwrap();
    assert!(IgesDataToolLocation::convert_location(1e-5, &general, &mut result, 2.0).unwrap());
    assert_eq!(result.apply_coordinates(Xyz::new(0.0, 0.0, 0.0)).x(), 10.0);
    general.set_value(1, 1, 2.0).unwrap();
    assert!(!IgesDataToolLocation::convert_location(1e-5, &general, &mut result, 1.0).unwrap());
    general.set_value(1, 1, 0.0).unwrap();
    general.set_value(1, 2, 1.0).unwrap();
    general.set_value(2, 2, 0.0).unwrap();
    general.set_value(2, 1, -1.0).unwrap();
    assert!(IgesDataToolLocation::convert_location(1e-5, &general, &mut result, 1.0).unwrap());
    let moved = result.apply_coordinates(Xyz::new(1.0, 0.0, 0.0));
    assert!((moved.x() - 5.0).abs() < 1e-12 && (moved.y() + 1.0).abs() < 1e-12);
}

#[test]
fn the_editor_knows_the_units_and_versions() {
    assert_eq!(IgesDataBasicEditor::get_flag_by_value(25.4), 1);
    assert_eq!(IgesDataBasicEditor::get_flag_by_value(1000.0), 6);
    assert_eq!(IgesDataBasicEditor::get_flag_by_value(3.0), 0);
    assert_eq!(IgesDataBasicEditor::unit_flag_name(4), "FT");
    assert_eq!(IgesDataBasicEditor::unit_name_flag(b"INCH"), 1);
    assert_eq!(IgesDataBasicEditor::drafting_name(3), "ANSI");
    let model = IgesDataIgesModel::new();
    let protocol: Handle<dyn IgesDataProtocolOps> = iges_data_protocol().cast();
    let mut editor = IgesDataBasicEditor::with_model(&model, &protocol);
    assert!(!editor.set_unit_flag(12));
    assert!(editor.set_unit_flag(2));
    let live = model.get().unwrap();
    assert_eq!(live.global_section().unit_flag(), 2);
    assert!(editor.set_unit_name(b"FT"));
    assert_eq!(live.global_section().unit_flag(), 4);
    assert!(!editor.set_unit_value(-1.0));
    live.change_global_section(|header| {
        header.set_cascade_unit(1.0);
        header.set_resolution(304.8);
    });
    editor.apply_unit(true);
    assert!((live.global_section().resolution() - 1.0).abs() < 1e-12);
}

#[test]
fn start_lines_of_a_model_copy_on_demand() {
    let first = IgesDataIgesModel::new();
    let second = IgesDataIgesModel::new();
    first.get().unwrap().add_start_line("line", 0);
    let live = second.get().unwrap();
    live.get_from_another(&first.cast()).unwrap();
    assert_eq!(live.start_line(1), b"line");
    live.clear_start_section();
    assert_eq!(live.nb_start_lines(), 0);
    assert_eq!(first.get().unwrap().nb_start_lines(), 1);
    assert!(live.new_empty_model().is_some());
    assert!(live.apply_static("x"));
}
