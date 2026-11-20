//! Native boss/pocket composition is qualified separately from profile construction.
use super::*;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
#[path = "body_controls/multiline.rs"]
mod multiline;

fn cylinder_body() -> martensite_tkbrep::topods::Shape {
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
    BRepPrimAPIMakeCylinder::new(10., 4.)
        .shape()
        .unwrap()
        .clone()
}
fn attach_tool(
    base: &martensite_tkbrep::topods::Shape,
    tool: &martensite_tkbrep::topods::Shape,
    operation: martensite_tkbo::bop_algo::Operation,
) -> martensite_tkbo::brep_algo_api::BRepAlgoAPIBooleanOperation {
    let before = (native_snapshot(base), native_snapshot(tool));
    let mut boolean = configured_boolean(base, tool, operation);
    boolean.build().unwrap();
    assert!(boolean.is_done());
    assert_eq!(native_snapshot(base), before.0);
    assert_eq!(native_snapshot(tool), before.1);
    boolean
}
fn configured_boolean(
    base: &martensite_tkbrep::topods::Shape,
    tool: &martensite_tkbrep::topods::Shape,
    operation: martensite_tkbo::bop_algo::Operation,
) -> martensite_tkbo::brep_algo_api::BRepAlgoAPIBooleanOperation {
    use martensite_tkbo::brep_algo_api::BRepAlgoAPIBooleanOperation;
    use martensite_tkernel::ncollection::List;
    let (mut arguments, mut tools) = (List::default(), List::default());
    arguments.append(base.clone());
    tools.append(tool.clone());
    let mut boolean = BRepAlgoAPIBooleanOperation::new();
    boolean.set_arguments(&arguments);
    boolean.set_tools(&tools);
    boolean.set_operation(operation);
    boolean.set_non_destructive(true);
    boolean.set_to_fill_history(true);
    boolean
}
fn native_snapshot(shape: &martensite_tkbrep::topods::Shape) -> Vec<u8> {
    use martensite_tkbrep::bintools::{self, WriteOptions};
    use martensite_tkernel::{message::ProgressRange, storage::IoStream};
    let mut stream = IoStream::from_bytes(Vec::new());
    bintools::write(
        shape,
        &mut stream,
        WriteOptions {
            with_normals: true,
            ..Default::default()
        },
        &ProgressRange::default(),
    )
    .unwrap();
    let mut bytes = vec![0; usize::try_from(stream.tellp()).unwrap()];
    stream.seekg(0).read(&mut bytes);
    assert!(!stream.fail());
    bytes
}
fn validate_attached_body(shape: &martensite_tkbrep::topods::Shape) {
    use martensite_tkbo::body_check::validate_body;
    let resolution = Resolution::new(
        1e-5,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    );
    let report = validate_body(shape, &resolution).unwrap();
    assert!(report.is_valid(), "{report:?}");
    check_one_solid(shape);
}
#[test]
fn wrapped_glyph_boss_joins_its_cylindrical_support_body() {
    let mut request = glyph_request(0);
    shift_developed_profile(&mut request, 2.);
    let result = normal_extrude_profile(&request).unwrap();
    let bytes = result.encoded_brep().to_vec();
    let native = result.export_native().unwrap();
    let base = cylinder_body();
    let attached = attach_tool(
        &base,
        &native.bodies,
        martensite_tkbo::bop_algo::Operation::Fuse,
    );
    validate_attached_body(attached.shape());
    assert!(body_volume_estimate(attached.shape()) > body_volume_estimate(&base));
    check_tool_history(&attached, &native.entities);
    assert_eq!(result.encoded_brep(), bytes);
}

fn check_one_solid(shape: &martensite_tkbrep::topods::Shape) {
    use martensite_tkbrep::{topexp, toptools::IndexedMapOfShape};
    let mut solids = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Solid, &mut solids).unwrap();
    assert_eq!(solids.extent(), 1);
}

#[test]
fn inward_wrapped_glyph_cuts_a_pocket_in_its_support_body() {
    let mut request = glyph_request(0);
    request.support.normal = NormalSense::Inward;
    shift_developed_profile(&mut request, 2.);
    let original = request.clone();
    let result = normal_extrude_profile(&request).unwrap();
    let bytes = result.encoded_brep().to_vec();
    let native = result.export_native().unwrap();
    let base = cylinder_body();
    let attached = attach_tool(
        &base,
        &native.bodies,
        martensite_tkbo::bop_algo::Operation::Cut,
    );
    validate_attached_body(attached.shape());
    assert!(body_volume_estimate(attached.shape()) < body_volume_estimate(&base));
    check_tool_history(&attached, &native.entities);
    assert_eq!(result.encoded_brep(), bytes);
    assert_eq!(request, original);
}

fn body_volume_estimate(shape: &martensite_tkbrep::topods::Shape) -> f64 {
    use martensite_tkgeombase::gprop::GProps;
    use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};
    let mut properties = GProps::default();
    let error = BRepGProp::volume_properties_gk(
        shape,
        &mut properties,
        1e-8,
        true,
        VolumeGKFlags::default(),
    )
    .unwrap();
    assert!(error.is_finite() && (0. ..=1e-8).contains(&error));
    assert!(properties.mass().is_finite() && properties.mass() > 0.);
    properties.mass()
}

fn check_tool_history(
    boolean: &martensite_tkbo::brep_algo_api::BRepAlgoAPIBooleanOperation,
    entities: &BTreeMap<WrappedHistoryKey, martensite_tkbrep::topods::Shape>,
) {
    use martensite_tkbrep::{topexp, toptools::IndexedMapOfShape};
    let mut output = IndexedMapOfShape::default();
    topexp::map_shapes(boolean.shape(), &mut output, true, true);
    let mut checked = 0;
    for (key, source) in entities {
        if !matches!(
            source.shape_type().unwrap(),
            ShapeEnum::Face | ShapeEnum::Edge | ShapeEnum::Vertex
        ) {
            continue;
        }
        check_entity_history(boolean, key, source, &output);
        checked += 1;
    }
    assert!(checked > 0);
}
fn check_entity_history(
    boolean: &martensite_tkbo::brep_algo_api::BRepAlgoAPIBooleanOperation,
    key: &WrappedHistoryKey,
    source: &martensite_tkbrep::topods::Shape,
    output: &martensite_tkbrep::toptools::IndexedMapOfShape,
) {
    let mut images = boolean.modified(source).unwrap();
    images.extend(boolean.generated(source).unwrap());
    let deleted = boolean.is_deleted(source).unwrap();
    if images.is_empty() && !deleted {
        assert!(output.contains(source), "unaccounted tool source={key:?}");
    }
    assert!(
        images.iter().all(|image| output.contains(image)),
        "foreign history image for={key:?}"
    );
}
