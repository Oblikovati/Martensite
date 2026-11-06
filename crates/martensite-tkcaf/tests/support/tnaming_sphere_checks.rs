//! Real sphere driver, including recomputation, failure ordering and primitive naming.
use super::naming_function_values as values;
use super::{naming_function_graph, naming_sphere_driver, naming_transform_arguments};
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkcaf::tnaming::{TNaming, TNamingBuilder, TNamingNamedShape, TNamingTool};
use martensite_tkernel::{standard::Handle, tcollection::ExtendedString};
use martensite_tkg3d::{
    geom::{GeomElementarySurface, GeomSphericalSurface},
    top_abs::ShapeEnum,
};
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::TDataStdUAttribute,
    tdf::{TdfAttributeState, TdfLabel, TdfTagSource},
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriver, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::{
    gp::{Point3, Transform3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeOneAxis, BRepPrimAPIMakeSphere};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex};
use naming_function_graph::{add_function, add_object};
use naming_sphere_driver::{NamingSphereDriver, load_sphere_names};

fn sphere_fixture(
    center: Option<topods::Shape>,
) -> (Handle<TDocStdDocument>, Handle<TFunctionFunction>) {
    let doc = TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap();
    let center_object = center_object(&doc, center);
    let object = add_object(&doc).unwrap();
    let function = add_function(
        &object,
        &guid_from_text("12e94544-6dbc-11d4-b9c8-0060b0ee281b"),
    )
    .unwrap();
    naming_transform_arguments::set_object_argument(&function, 1, &center_object).unwrap();
    values::real_argument(&function, 2)
        .unwrap()
        .set_value(8.0)
        .unwrap();
    (doc, function)
}

fn center_object(
    doc: &Handle<TDocStdDocument>,
    center: Option<topods::Shape>,
) -> Handle<TDataStdUAttribute> {
    let center_object = add_object(doc).unwrap();
    let center_function = add_function(
        &center_object,
        &guid_from_text("00000000-0000-0000-0000-000000000000"),
    )
    .unwrap();
    if let Some(shape) = center {
        TNamingBuilder::new(&values::result_label(&center_function).unwrap())
            .unwrap()
            .generated(&shape)
            .unwrap();
    }
    center_object
}

fn centered_fixture() -> (Handle<TDocStdDocument>, Handle<TFunctionFunction>) {
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(3.0, 4.0, 5.0))
        .unwrap()
        .vertex()
        .unwrap();
    sphere_fixture(Some(vertex.into_shape()))
}

fn run_sphere(function: &Handle<TFunctionFunction>) -> (i32, Handle<TFunctionLogbook>) {
    let driver = NamingSphereDriver::default();
    driver.init(&function.label());
    let mut log = TFunctionLogbook::set(&function.label()).unwrap();
    log.clear().unwrap();
    (driver.execute(&mut log).unwrap(), log)
}

fn named_at(label: &TdfLabel) -> topods::Shape {
    let mut named: Handle<TNamingNamedShape> = Handle::default();
    assert!(
        label
            .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
            .unwrap()
    );
    TNamingTool::get_shape(&named).unwrap()
}

fn sphere_surface(function: &Handle<TFunctionFunction>) -> Handle<GeomSphericalSurface> {
    let result = values::result_label(function).unwrap();
    let face = named_at(&result.find_child(1, false).unwrap());
    BRepTool::surface(&topods::face(&face).unwrap())
        .unwrap()
        .cast()
}

#[test]
fn sphere_driver_records_closed_form_center_radius_and_meridian_labels() {
    let (_document, function) = centered_fixture();
    function.set_failure(9999).unwrap();
    let (status, log) = run_sphere(&function);
    assert_eq!((status, function.get_failure()), (0, 0));
    let surface = sphere_surface(&function);
    assert_eq!(surface.radius(), 8.0);
    assert_eq!(surface.position().location(), Point3::new(3.0, 4.0, 5.0));
    let result = values::result_label(&function).unwrap();
    assert_eq!(named_at(&result).shape_type().unwrap(), ShapeEnum::Solid);
    assert_full_sphere_tags(&result);
    assert!(log.get_valid().contains(&result));
    assert_sphere_poles(&result, 5.0, 8.0);
}

fn assert_full_sphere_tags(label: &TdfLabel) {
    assert_sphere_tags(
        label,
        &[
            ShapeEnum::Face,
            ShapeEnum::Edge,
            ShapeEnum::Vertex,
            ShapeEnum::Vertex,
        ],
    );
}

fn assert_sphere_tags(label: &TdfLabel, kinds: &[ShapeEnum]) {
    assert_eq!(
        TdfTagSource::set(label).unwrap().get().unwrap().get(),
        kinds.len() as i32
    );
    for (index, kind) in kinds.iter().enumerate() {
        assert_eq!(
            named_at(&label.find_child(index as i32 + 1, false).unwrap())
                .shape_type()
                .unwrap(),
            *kind
        );
    }
}

fn assert_sphere_poles(label: &TdfLabel, center_z: f64, radius: f64) {
    let mut heights = [0.0; 2];
    for (index, tag) in [3, 4].into_iter().enumerate() {
        let vertex = topods::vertex(&named_at(&label.find_child(tag, false).unwrap())).unwrap();
        let point = BRepTool::pnt(&vertex).unwrap();
        assert!((point.x() - 3.0).abs() <= 32.0 * f64::EPSILON * radius);
        assert!((point.y() - 4.0).abs() <= 32.0 * f64::EPSILON * radius);
        heights[index] = point.z();
    }
    heights.sort_by(f64::total_cmp);
    assert_eq!(heights, [center_z - radius, center_z + radius]);
}

#[test]
fn sphere_recompute_restores_location_and_reuses_naming_tags() {
    let (_document, function) = centered_fixture();
    assert_eq!(run_sphere(&function).0, 0);
    let result = values::result_label(&function).unwrap();
    let displacement = Location::new(Transform3::translation([10.0, 20.0, 30.0]));
    TNaming::displace(&result, &displacement, true).unwrap();
    values::real_argument(&function, 2)
        .unwrap()
        .set_value(4.0)
        .unwrap();
    assert_eq!(run_sphere(&function).0, 0);
    let shape = named_at(&result);
    assert_eq!(shape.location(), &displacement);
    assert_full_sphere_tags(&result);
    assert_eq!(sphere_surface(&function).radius(), 4.0);
}

#[test]
fn sphere_missing_center_reports_wrong_argument_without_creating_a_result() {
    let (_document, function) = sphere_fixture(None);
    function.set_failure(9999).unwrap();
    let (status, log) = run_sphere(&function);
    assert_eq!((status, function.get_failure()), (-1, 14));
    assert!(log.is_empty());
    assert!(values::function_result(&function).unwrap().is_null());
    assert_eq!(
        values::real_argument(&function, 2)
            .unwrap()
            .get()
            .unwrap()
            .get(),
        8.0
    );
}

#[test]
fn sphere_nonvertex_center_preserves_native_origin_fallback() {
    let shape = sphere_seed_solid(2.0);
    let (_document, function) = sphere_fixture(Some(shape));
    assert_eq!(run_sphere(&function).0, 0);
    assert_eq!(
        sphere_surface(&function).position().location(),
        Point3::new(0.0, 0.0, 0.0)
    );
}

#[test]
fn sphere_driver_without_function_and_virtual_defaults_match_native_fixture() {
    let document = TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap();
    let label = document.main().unwrap();
    let driver = NamingSphereDriver::default();
    driver.init(&label);
    let mut log = TFunctionLogbook::set(&label).unwrap();
    assert!(driver.must_execute(&log).unwrap());
    driver.validate(&log).unwrap();
    assert!(log.is_empty());
    assert_eq!(driver.execute(&mut log).unwrap(), -1);
}

#[test]
fn sphere_partial_latitudes_and_sweep_name_all_five_actual_faces() {
    let document = TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap();
    let label = document.main().unwrap();
    let mut maker = BRepPrimAPIMakeSphere::with_latitudes_angle(8.0, -0.5, 0.5, 1.0).unwrap();
    maker.build(&Default::default()).unwrap();
    load_sphere_names(&label, &mut maker).unwrap();
    assert_sphere_tags(&label, &[ShapeEnum::Face; 5]);
    assert_eq!(named_at(&label).shape_type().unwrap(), ShapeEnum::Solid);
}

#[test]
fn sphere_missing_center_retains_previous_named_result_and_location() {
    let (_document, function) = sphere_fixture(None);
    let label = values::result_label(&function).unwrap();
    let old = sphere_seed_solid(3.0);
    TNamingBuilder::new(&label)
        .unwrap()
        .generated(&old)
        .unwrap();
    let displacement = Location::new(Transform3::translation([2.0, 3.0, 4.0]));
    TNaming::displace(&label, &displacement, true).unwrap();
    let previous = named_at(&label);
    let (status, log) = run_sphere(&function);
    assert_eq!((status, function.get_failure()), (-1, 14));
    assert!(log.is_empty());
    assert!(named_at(&label).is_equal(&previous));
    assert_eq!(named_at(&label).location(), &displacement);
}

fn sphere_seed_solid(radius: f64) -> topods::Shape {
    BRepPrimAPIMakeSphere::new(radius)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape()
}
