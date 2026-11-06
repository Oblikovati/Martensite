#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_geom::*;

#[test]
fn runtime_type_step_visual_annotation_curve_occurrence() {
    let object = Handle::new(StepVisualAnnotationCurveOccurrence::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualAnnotationCurveOccurrence"
    );
    assert!(object.is_kind_name("StepVisualAnnotationOccurrence"));
}

#[test]
fn runtime_type_step_visual_annotation_curve_occurrence_and_geom_repr_item() {
    let object = Handle::new(StepVisualAnnotationCurveOccurrenceAndGeomReprItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualAnnotationCurveOccurrenceAndGeomReprItem"
    );
    assert!(object.is_kind_name("StepVisualAnnotationCurveOccurrence"));
}

#[test]
fn runtime_type_step_visual_annotation_fill_area() {
    let object = Handle::new(StepVisualAnnotationFillArea::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualAnnotationFillArea"
    );
    assert!(object.is_kind_name("StepShapeGeometricCurveSet"));
}

#[test]
fn runtime_type_step_visual_annotation_fill_area_occurrence() {
    let object = Handle::new(StepVisualAnnotationFillAreaOccurrence::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualAnnotationFillAreaOccurrence"
    );
    assert!(object.is_kind_name("StepVisualAnnotationOccurrence"));
}

#[test]
fn runtime_type_step_visual_annotation_occurrence() {
    let object = Handle::new(StepVisualAnnotationOccurrence::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualAnnotationOccurrence"
    );
    assert!(object.is_kind_name("StepVisualStyledItem"));
}

#[test]
fn runtime_type_step_visual_annotation_plane() {
    let object = Handle::new(StepVisualAnnotationPlane::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualAnnotationPlane"
    );
    assert!(object.is_kind_name("StepVisualAnnotationOccurrence"));
}

#[test]
fn runtime_type_step_visual_annotation_text() {
    let object = Handle::new(StepVisualAnnotationText::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualAnnotationText"
    );
    assert!(object.is_kind_name("StepReprMappedItem"));
}

#[test]
fn runtime_type_step_visual_annotation_text_occurrence() {
    let object = Handle::new(StepVisualAnnotationTextOccurrence::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualAnnotationTextOccurrence"
    );
    assert!(object.is_kind_name("StepVisualAnnotationOccurrence"));
}

#[test]
fn runtime_type_step_visual_area_in_set() {
    let object = Handle::new(StepVisualAreaInSet::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepVisualAreaInSet");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_area_in_set() {
    let a_area = Handle::new(StepVisualPresentationArea::new());
    let a_in_set = Handle::new(StepVisualPresentationSet::new());
    let object = StepVisualAreaInSet::new();
    object.init(&a_area, &a_in_set);
    assert!(object.area() == a_area);
    assert!(object.in_set() == a_in_set);
}

#[test]
fn runtime_type_step_visual_background_colour() {
    let object = Handle::new(StepVisualBackgroundColour::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualBackgroundColour"
    );
    assert!(object.is_kind_name("StepVisualColour"));
}

#[test]
fn init_round_trip_step_visual_background_colour() {
    let a_presentation = StepVisualAreaOrView::default();
    let object = StepVisualBackgroundColour::new();
    object.init(&a_presentation);
}

#[test]
fn runtime_type_step_visual_camera_image() {
    let object = Handle::new(StepVisualCameraImage::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCameraImage"
    );
    assert!(object.is_kind_name("StepReprMappedItem"));
}

#[test]
fn runtime_type_step_visual_camera_image2d_with_scale() {
    let object = Handle::new(StepVisualCameraImage2dWithScale::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCameraImage2dWithScale"
    );
    assert!(object.is_kind_name("StepVisualCameraImage"));
}

#[test]
fn runtime_type_step_visual_camera_image3d_with_scale() {
    let object = Handle::new(StepVisualCameraImage3dWithScale::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCameraImage3dWithScale"
    );
    assert!(object.is_kind_name("StepVisualCameraImage"));
}

#[test]
fn runtime_type_step_visual_camera_model() {
    let object = Handle::new(StepVisualCameraModel::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCameraModel"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn runtime_type_step_visual_camera_model_d2() {
    let object = Handle::new(StepVisualCameraModelD2::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCameraModelD2"
    );
    assert!(object.is_kind_name("StepVisualCameraModel"));
}

#[test]
fn init_round_trip_step_visual_camera_model_d2() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_view_window = Handle::new(StepVisualPlanarBox::new());
    let a_view_window_clipping = true;
    let object = StepVisualCameraModelD2::new();
    object.init(&a_name, &a_view_window, a_view_window_clipping);
    assert!(object.view_window() == a_view_window);
    assert!(object.view_window_clipping() == a_view_window_clipping);
}

#[test]
fn runtime_type_step_visual_camera_model_d3() {
    let object = Handle::new(StepVisualCameraModelD3::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCameraModelD3"
    );
    assert!(object.is_kind_name("StepVisualCameraModel"));
}

#[test]
fn init_round_trip_step_visual_camera_model_d3() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_view_reference_system = Handle::new(StepGeomAxis2Placement3d::new());
    let a_perspective_of_volume = Handle::new(StepVisualViewVolume::new());
    let object = StepVisualCameraModelD3::new();
    object.init(&a_name, &a_view_reference_system, &a_perspective_of_volume);
    assert!(object.view_reference_system() == a_view_reference_system);
    assert!(object.perspective_of_volume() == a_perspective_of_volume);
}

#[test]
fn runtime_type_step_visual_camera_model_d3_multi_clipping() {
    let object = Handle::new(StepVisualCameraModelD3MultiClipping::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCameraModelD3MultiClipping"
    );
    assert!(object.is_kind_name("StepVisualCameraModelD3"));
}

#[test]
fn init_round_trip_step_visual_camera_model_d3_multi_clipping() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_view_reference_system = Handle::new(StepGeomAxis2Placement3d::new());
    let the_perspective_of_volume = Handle::new(StepVisualViewVolume::new());
    let the_shape_clipping = Handle::new(
        HArray1::<StepVisualCameraModelD3MultiClippingInterectionSelect>::with_bounds(1, 2)
            .unwrap(),
    );
    let object = StepVisualCameraModelD3MultiClipping::new();
    object.init(
        &the_name,
        &the_view_reference_system,
        &the_perspective_of_volume,
        &the_shape_clipping,
    );
    assert!(object.shape_clipping() == the_shape_clipping);
}

#[test]
fn runtime_type_step_visual_camera_model_d3_multi_clipping_intersection() {
    let object = Handle::new(StepVisualCameraModelD3MultiClippingIntersection::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCameraModelD3MultiClippingIntersection"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_visual_camera_model_d3_multi_clipping_intersection() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_shape_clipping = Handle::new(
        HArray1::<StepVisualCameraModelD3MultiClippingInterectionSelect>::with_bounds(1, 2)
            .unwrap(),
    );
    let object = StepVisualCameraModelD3MultiClippingIntersection::new();
    object.init(&the_name, &the_shape_clipping);
    assert!(object.shape_clipping() == the_shape_clipping);
}

#[test]
fn runtime_type_step_visual_camera_model_d3_multi_clipping_union() {
    let object = Handle::new(StepVisualCameraModelD3MultiClippingUnion::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCameraModelD3MultiClippingUnion"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_visual_camera_model_d3_multi_clipping_union() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_shape_clipping = Handle::new(
        HArray1::<StepVisualCameraModelD3MultiClippingUnionSelect>::with_bounds(1, 2).unwrap(),
    );
    let object = StepVisualCameraModelD3MultiClippingUnion::new();
    object.init(&the_name, &the_shape_clipping);
    assert!(object.shape_clipping() == the_shape_clipping);
}

#[test]
fn runtime_type_step_visual_camera_usage() {
    let object = Handle::new(StepVisualCameraUsage::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCameraUsage"
    );
    assert!(object.is_kind_name("StepReprRepresentationMap"));
}

#[test]
fn runtime_type_step_visual_characterized_obj_and_representation_and_draughting_model() {
    let object = Handle::new(StepVisualCharacterizedObjAndRepresentationAndDraughtingModel::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCharacterizedObjAndRepresentationAndDraughtingModel"
    );
    assert!(object.is_kind_name("StepVisualDraughtingModel"));
}

#[test]
fn runtime_type_step_visual_colour() {
    let object = Handle::new(StepVisualColour::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepVisualColour");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn runtime_type_step_visual_colour_rgb() {
    let object = Handle::new(StepVisualColourRgb::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepVisualColourRgb");
    assert!(object.is_kind_name("StepVisualColourSpecification"));
}

#[test]
fn init_round_trip_step_visual_colour_rgb() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_red = 2.5;
    let a_green = 2.5;
    let a_blue = 2.5;
    let object = StepVisualColourRgb::new();
    object.init(&a_name, a_red, a_green, a_blue);
    assert!(object.red() == a_red);
    assert!(object.green() == a_green);
    assert!(object.blue() == a_blue);
}

#[test]
fn runtime_type_step_visual_colour_specification() {
    let object = Handle::new(StepVisualColourSpecification::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualColourSpecification"
    );
    assert!(object.is_kind_name("StepVisualColour"));
}

#[test]
fn init_round_trip_step_visual_colour_specification() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let object = StepVisualColourSpecification::new();
    object.init(&a_name);
    assert!(object.name() == a_name);
}

#[test]
fn runtime_type_step_visual_complex_triangulated_face() {
    let object = Handle::new(StepVisualComplexTriangulatedFace::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualComplexTriangulatedFace"
    );
    assert!(object.is_kind_name("StepVisualTessellatedFace"));
}

#[test]
fn init_round_trip_step_visual_complex_triangulated_face() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_tessellated_face_coordinates = Handle::new(StepVisualCoordinatesList::new());
    let the_tessellated_face_pnmax = 7;
    let the_tessellated_face_normals =
        Handle::new(HArray2::<f64>::with_bounds(1, 2, 1, 2).unwrap());
    let the_has_tessellated_face_geometric_link = true;
    let the_tessellated_face_geometric_link = StepVisualFaceOrSurface::default();
    let the_pnindex = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let the_triangle_strips =
        Handle::new(HArray1::<Handle<StandardTransient>>::with_bounds(1, 2).unwrap());
    let the_triangle_fans =
        Handle::new(HArray1::<Handle<StandardTransient>>::with_bounds(1, 2).unwrap());
    let object = StepVisualComplexTriangulatedFace::new();
    object.init(
        &the_representation_item_name,
        &the_tessellated_face_coordinates,
        the_tessellated_face_pnmax,
        &the_tessellated_face_normals,
        the_has_tessellated_face_geometric_link,
        &the_tessellated_face_geometric_link,
        &the_pnindex,
        &the_triangle_strips,
        &the_triangle_fans,
    );
    assert!(object.pnindex() == the_pnindex);
    assert!(object.triangle_strips() == the_triangle_strips);
    assert!(object.triangle_fans() == the_triangle_fans);
}

#[test]
fn runtime_type_step_visual_complex_triangulated_surface_set() {
    let object = Handle::new(StepVisualComplexTriangulatedSurfaceSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualComplexTriangulatedSurfaceSet"
    );
    assert!(object.is_kind_name("StepVisualTessellatedSurfaceSet"));
}
