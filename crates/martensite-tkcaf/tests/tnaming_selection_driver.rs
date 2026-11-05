//! Complete real selection-driver operation, a prerequisite rather than the full E2 chain.
#[path = "support/naming_box_driver.rs"]
mod naming_box_driver;
#[path = "support/naming_function_graph.rs"]
mod naming_function_graph;
#[path = "support/naming_function_values.rs"]
mod naming_function_values;
#[path = "support/naming_selection_driver.rs"]
mod naming_selection_driver;
mod support;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
use martensite_tkcaf::tnaming::{TNamingNamedShape, TNamingTool};
use martensite_tkernel::{
    standard::{Handle, Transient},
    tcollection::ExtendedString,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::{TDataStdTreeNode, TDataStdUAttribute},
    tdf::{TdfAttributeState, TdfLabel},
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriver, TFunctionFunction, TFunctionLogbook},
};
use naming_box_driver::{NamingBoxDriver, real_argument, result_label};
use naming_function_graph::{add_function, add_object, add_selection, solve_selection};
use naming_selection_driver::{NamingSelectionDriver, NamingSelectionWarnings};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct RecordedSelectionWarnings(Mutex<Vec<String>>);
impl NamingSelectionWarnings for RecordedSelectionWarnings {
    fn warning(&self, message: &str) {
        self.0.lock().unwrap().push(message.to_owned());
    }
}

fn document() -> Handle<TDocStdDocument> {
    TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap()
}
fn set_box_dimensions(function: &Handle<TFunctionFunction>, x: f64) {
    for (position, value) in [(1, x), (2, 290.0), (3, 390.0)] {
        real_argument(function, position)
            .unwrap()
            .set_value(value)
            .unwrap();
    }
}
fn execute_box(function: &Handle<TFunctionFunction>) {
    let driver = NamingBoxDriver::default();
    driver.init(&function.label());
    let mut log = TFunctionLogbook::set(&function.label()).unwrap();
    log.clear().unwrap();
    assert_eq!(driver.execute(&mut log).unwrap(), 0);
}
fn named_shape(label: &TdfLabel) -> topods::Shape {
    let mut named: Handle<TNamingNamedShape> = Handle::default();
    assert!(
        label
            .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
            .unwrap()
    );
    TNamingTool::get_shape(&named).unwrap()
}
fn assert_front_plane(shape: &topods::Shape, x: f64) {
    assert_eq!(shape.shape_type().unwrap(), ShapeEnum::Face);
    let vertices: Vec<topods::Shape> = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)
        .unwrap()
        .collect();
    assert!(!vertices.is_empty());
    for vertex in vertices {
        assert_eq!(
            BRepTool::pnt(&topods::vertex(&vertex).unwrap())
                .unwrap()
                .x(),
            x
        );
    }
}
fn selection_driver() -> (NamingSelectionDriver, Arc<RecordedSelectionWarnings>) {
    let warnings = Arc::new(RecordedSelectionWarnings::default());
    (
        NamingSelectionDriver::with_warnings(warnings.clone()),
        warnings,
    )
}

fn modeled_box() -> (
    Handle<TDocStdDocument>,
    Handle<TDataStdUAttribute>,
    Handle<TFunctionFunction>,
) {
    let document = document();
    let object = add_object(&document).unwrap();
    let function = add_function(
        &object,
        &guid_from_text("12e94543-6dbc-11d4-b9c8-0060b0ee281b"),
    )
    .unwrap();
    set_box_dimensions(&function, 190.0);
    execute_box(&function);
    (document, object, function)
}

#[test]
fn real_function_graph_selection_tracks_recomputed_box_front_face() {
    let (document, object, function) = modeled_box();
    let face_label = result_label(&function)
        .unwrap()
        .find_child(3, false)
        .unwrap();
    let original = named_shape(&face_label);
    let (selected_object, selection) = add_selection(&document, &object, &original, true).unwrap();
    assert_selection_graph(&selected_object, &selection, &object);
    let (driver, warnings) = selection_driver();
    assert_front_plane(&solve_selection(&selection, &driver).unwrap(), 190.0);
    set_box_dimensions(&function, 290.0);
    execute_box(&function);
    let solved = solve_selection(&selection, &driver).unwrap();
    assert_front_plane(&solved, 290.0);
    assert!(solved.is_same(&named_shape(&face_label)));
    assert!(!solved.is_same(&original));
    assert_eq!(selection.get_failure(), 0);
    assert!(warnings.0.lock().unwrap().is_empty());
}
fn assert_selection_graph(
    object: &Handle<TDataStdUAttribute>,
    function: &Handle<TFunctionFunction>,
    context: &Handle<TDataStdUAttribute>,
) {
    let object_node = TDataStdTreeNode::set(&object.label()).unwrap();
    let function_node = TDataStdTreeNode::set(&function.label()).unwrap();
    assert_eq!(object_node.father().label(), context.label());
    assert_eq!(function_node.father().label(), object.label());
}

#[test]
fn unsolved_naming_reports_case_warning_and_notdone_code() {
    let document = document();
    let object = add_object(&document).unwrap();
    let function = add_function(
        &object,
        &guid_from_text("12e94554-6dbc-11d4-b9c8-0060b0ee281b"),
    )
    .unwrap();
    let (driver, warnings) = selection_driver();
    assert!(solve_selection(&function, &driver).unwrap().is_null());
    assert_eq!(function.get_failure(), 9999);
    assert_eq!(
        *warnings.0.lock().unwrap(),
        ["%%%WARNING: DNamingSelectionDriver::Execute: Selection is Not solved !!!"]
    );
    let log = TFunctionLogbook::set(&function.label()).unwrap();
    assert!(log.get_valid().is_empty());
}

#[test]
fn missing_function_is_silent_and_driver_defaults_match_native_contract() {
    let document = document();
    let label = document.main().unwrap();
    let (driver, warnings) = selection_driver();
    driver.init(&label);
    let mut log = TFunctionLogbook::set(&label).unwrap();
    assert!(driver.must_execute(&log).unwrap());
    driver.validate(&log).unwrap();
    assert_eq!(driver.execute(&mut log).unwrap(), -1);
    assert!(log.is_empty());
    assert!(warnings.0.lock().unwrap().is_empty());
    let default = NamingSelectionDriver::default();
    assert_eq!(default.dynamic_type().name(), "DNamingSelectionDriver");
    assert!(default.is_kind_name("TFunction_Driver"));
}

#[path = "support/naming_transform_arguments.rs"]
mod naming_transform_arguments;
#[path = "support/tnaming_transform_argument_checks.rs"]
mod transform_argument_checks;

#[path = "support/naming_transform/mod.rs"]
mod naming_transform;
#[path = "support/tnaming_transform_checks.rs"]
mod transform_checks;

#[path = "support/naming_fillet.rs"]
mod naming_fillet;
#[path = "support/naming_history.rs"]
mod naming_history;

#[path = "support/tnaming_fillet_checks.rs"]
mod fillet_checks;

#[path = "support/tnaming_boolean_checks.rs"]
mod boolean_checks;
#[path = "support/tnaming_e2_pipeline.rs"]
pub(crate) mod e2_pipeline;
#[path = "support/naming_boolean/mod.rs"]
mod naming_boolean;

#[path = "support/tnaming_cylinder_checks.rs"]
mod cylinder_checks;
#[path = "support/tnaming_draw_e3_checks.rs"]
pub(crate) mod draw_e3_checks;
#[path = "support/tnaming_draw_selection_checks.rs"]
mod draw_selection_checks;
#[path = "support/naming_cylinder_driver.rs"]
mod naming_cylinder_driver;
#[path = "support/naming_draw_selection/mod.rs"]
mod naming_draw_selection;
#[path = "support/naming_driver_registration.rs"]
mod naming_driver_registration;
#[path = "support/naming_prism_driver.rs"]
mod naming_prism_driver;
#[path = "support/naming_sphere_driver.rs"]
mod naming_sphere_driver;
#[path = "support/naming_sweep_basis.rs"]
mod naming_sweep_basis;
#[path = "support/tnaming_prism_checks.rs"]
mod prism_checks;
#[path = "support/tnaming_sphere_checks.rs"]
mod sphere_checks;

#[path = "support/naming_draw_document.rs"]
pub(crate) mod naming_draw_document;
#[path = "support/naming_draw_driver_registration.rs"]
mod naming_draw_driver_registration;
#[path = "support/naming_draw_observations.rs"]
mod naming_draw_observations;
#[path = "support/naming_revolution/mod.rs"]
mod naming_revolution;
#[path = "support/naming_sweep_history.rs"]
mod naming_sweep_history;
#[path = "support/tnaming_revolution_checks.rs"]
mod revolution_checks;

#[path = "support/tnaming_draw_cylinder_cut_checks.rs"]
pub(crate) mod draw_cylinder_cut_checks;
#[path = "support/naming_draw_modeling.rs"]
mod naming_draw_modeling;

#[path = "support/tnaming_draw_fillet_cut_checks.rs"]
pub(crate) mod draw_fillet_cut_checks;
#[path = "support/tnaming_draw_fused_fillet_checks.rs"]
pub(crate) mod draw_fused_fillet_checks;
#[path = "support/tnaming_draw_polygon_cut_checks.rs"]
pub mod draw_polygon_cut_checks;
#[path = "support/tnaming_draw_polygon_fuse_checks.rs"]
pub(crate) mod draw_polygon_fuse_checks;
#[path = "support/tnaming_draw_prism_fuse_checks.rs"]
pub(crate) mod draw_prism_fuse_checks;
#[path = "support/tnaming_draw_revolution_chain_checks.rs"]
pub(crate) mod draw_revolution_chain_checks;
#[path = "support/tnaming_draw_sphere_fuse_checks.rs"]
pub(crate) mod draw_sphere_fuse_checks;
#[path = "support/tnaming_line3d_checks.rs"]
mod line3d_checks;
#[path = "support/naming_draw_point_prism.rs"]
mod naming_draw_point_prism;
#[path = "support/naming_draw_primitives.rs"]
mod naming_draw_primitives;
#[path = "support/naming_line3d/mod.rs"]
mod naming_line3d;
#[path = "support/naming_point_driver.rs"]
mod naming_point_driver;
#[path = "support/tnaming_point_checks.rs"]
mod point_checks;

#[path = "support/naming_document_stage_checks.rs"]
mod naming_document_stage_checks;

#[path = "support/naming_draw_presentation.rs"]
mod naming_draw_presentation;
