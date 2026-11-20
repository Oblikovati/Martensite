//! Every selected glyph must retain its exposed cap after attachment (#425).
use super::*;
#[path = "multiline/replay.rs"]
mod replay;
use martensite_tkbo::bop_algo::Operation;
use martensite_tksketch::surface_sketch::HistoryRole as SurfaceHistoryRole;

#[test]
fn multiline_boss_retains_all_eighteen_selected_caps_on_one_support() {
    complete_multiline_attachment(
        NormalSense::Outward,
        Operation::Fuse,
        226802403. / 134217728.,
    );
}

#[test]
fn multiline_pocket_retains_all_eighteen_selected_floors_on_one_support() {
    // The inward/outward radial Jacobian ratio is exactly 79/81 at these depths.
    complete_multiline_attachment(NormalSense::Inward, Operation::Cut, 663607031. / 402653184.);
}

fn multiline_request(normal: NormalSense) -> WrappedNormalRequest {
    let mut request = text_request();
    request.support.normal = normal;
    request.options.planar_work = PlanarProofWorkLimit::PairBisections(8192);
    assert_eq!(request.profile.curves.len(), 239);
    assert_eq!(request.selections.len(), 18);
    request
}

fn multiline_support(radius: f64) -> martensite_tkbrep::topods::Shape {
    use martensite_tkmath::gp::{Axis2, Point3, StandardDirection3};
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
    let axes = Axis2::with_standard_frame(
        Point3::new(0., 0., -4.),
        StandardDirection3::Z,
        StandardDirection3::X,
    );
    BRepPrimAPIMakeCylinder::with_axes(axes, radius, 8.)
        .shape()
        .unwrap()
        .clone()
}

fn complete_multiline_attachment(normal: NormalSense, operation: Operation, expected: f64) {
    let request = multiline_request(normal);
    let original = request.clone();
    let result = normal_extrude_profile(&request).unwrap();
    assert_eq!(result.reports().len(), 18);
    check_volume_oracle(&result, expected);
    let bytes = result.encoded_brep().to_vec();
    let native = result.export_native().unwrap();
    let base = multiline_support(10.);
    let attached = attach_tool(&base, &native.bodies, operation);
    validate_attached_body(attached.shape());
    check_tool_history(&attached, &native.entities);
    selected_depth_caps_survive(&attached, &native.entities, &request);
    check_attachment_volume_direction(attached.shape(), &base, normal);
    assert_eq!(result.encoded_brep(), bytes);
    assert_eq!(request, original);
}

fn selected_depth_caps_survive(
    boolean: &martensite_tkbo::brep_algo_api::BRepAlgoAPIBooleanOperation,
    entities: &BTreeMap<WrappedHistoryKey, martensite_tkbrep::topods::Shape>,
    request: &WrappedNormalRequest,
) {
    use martensite_tkbrep::{topexp, toptools::IndexedMapOfShape};
    let mut output = IndexedMapOfShape::default();
    topexp::map_shapes(boolean.shape(), &mut output, true, true);
    for selection in &request.selections {
        let key = WrappedHistoryKey {
            source: WrappedEntitySource::Region(selection.region),
            role: SurfaceHistoryRole::LastDepthFace,
        };
        retained_cap_has_output_face(boolean, &entities[&key], &output, key);
    }
}

fn retained_cap_has_output_face(
    boolean: &martensite_tkbo::brep_algo_api::BRepAlgoAPIBooleanOperation,
    source: &martensite_tkbrep::topods::Shape,
    output: &martensite_tkbrep::toptools::IndexedMapOfShape,
    key: WrappedHistoryKey,
) {
    assert_eq!(source.shape_type().unwrap(), ShapeEnum::Face, "cap={key:?}");
    let mut images = boolean.modified(source).unwrap();
    images.extend(boolean.generated(source).unwrap());
    assert!(
        retained_face_in_output(source, output)
            || images
                .iter()
                .any(|image| retained_face_in_output(image, output)),
        "selected exposed cap lost during attachment: {key:?}"
    );
}

fn retained_face_in_output(
    image: &martensite_tkbrep::topods::Shape,
    output: &martensite_tkbrep::toptools::IndexedMapOfShape,
) -> bool {
    matches!(image.shape_type(), Ok(ShapeEnum::Face)) && output.contains(image)
}

fn check_attachment_volume_direction(
    result: &martensite_tkbrep::topods::Shape,
    support: &martensite_tkbrep::topods::Shape,
    normal: NormalSense,
) {
    let delta = body_volume_estimate(result) - body_volume_estimate(support);
    match normal {
        NormalSense::Outward => assert!(delta > 0., "boss volume delta={delta}"),
        NormalSense::Inward => assert!(delta < 0., "pocket volume delta={delta}"),
    }
}
