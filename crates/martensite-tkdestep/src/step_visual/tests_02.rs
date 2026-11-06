#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_basic::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn init_round_trip_step_visual_complex_triangulated_surface_set() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_tessellated_surface_set_coordinates = Handle::new(StepVisualCoordinatesList::new());
    let the_tessellated_surface_set_pnmax = 7;
    let the_tessellated_surface_set_normals =
        Handle::new(HArray2::<f64>::with_bounds(1, 2, 1, 2).unwrap());
    let the_pnindex = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let the_triangle_strips =
        Handle::new(HArray1::<Handle<StandardTransient>>::with_bounds(1, 2).unwrap());
    let the_triangle_fans =
        Handle::new(HArray1::<Handle<StandardTransient>>::with_bounds(1, 2).unwrap());
    let object = StepVisualComplexTriangulatedSurfaceSet::new();
    object.init(
        &the_representation_item_name,
        &the_tessellated_surface_set_coordinates,
        the_tessellated_surface_set_pnmax,
        &the_tessellated_surface_set_normals,
        &the_pnindex,
        &the_triangle_strips,
        &the_triangle_fans,
    );
    assert!(object.pnindex() == the_pnindex);
    assert!(object.triangle_strips() == the_triangle_strips);
    assert!(object.triangle_fans() == the_triangle_fans);
}

#[test]
fn runtime_type_step_visual_composite_text() {
    let object = Handle::new(StepVisualCompositeText::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCompositeText"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_visual_composite_text() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_collected_text =
        Handle::new(HArray1::<StepVisualTextOrCharacter>::with_bounds(1, 2).unwrap());
    let object = StepVisualCompositeText::new();
    object.init(&a_name, &a_collected_text);
    assert!(object.collected_text() == a_collected_text);
}

#[test]
fn runtime_type_step_visual_composite_text_with_extent() {
    let object = Handle::new(StepVisualCompositeTextWithExtent::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCompositeTextWithExtent"
    );
    assert!(object.is_kind_name("StepVisualCompositeText"));
}

#[test]
fn init_round_trip_step_visual_composite_text_with_extent() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_collected_text =
        Handle::new(HArray1::<StepVisualTextOrCharacter>::with_bounds(1, 2).unwrap());
    let a_extent = Handle::new(StepVisualPlanarExtent::new());
    let object = StepVisualCompositeTextWithExtent::new();
    object.init(&a_name, &a_collected_text, &a_extent);
    assert!(object.extent() == a_extent);
}

#[test]
fn runtime_type_step_visual_context_dependent_invisibility() {
    let object = Handle::new(StepVisualContextDependentInvisibility::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualContextDependentInvisibility"
    );
    assert!(object.is_kind_name("StepVisualInvisibility"));
}

#[test]
fn init_round_trip_step_visual_context_dependent_invisibility() {
    let a_invisible_items =
        Handle::new(HArray1::<StepVisualInvisibleItem>::with_bounds(1, 2).unwrap());
    let a_presentation_context = StepVisualInvisibilityContext::default();
    let object = StepVisualContextDependentInvisibility::new();
    object.init(&a_invisible_items, &a_presentation_context);
}

#[test]
fn runtime_type_step_visual_context_dependent_over_riding_styled_item() {
    let object = Handle::new(StepVisualContextDependentOverRidingStyledItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualContextDependentOverRidingStyledItem"
    );
    assert!(object.is_kind_name("StepVisualOverRidingStyledItem"));
}

#[test]
fn runtime_type_step_visual_coordinates_list() {
    let object = Handle::new(StepVisualCoordinatesList::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCoordinatesList"
    );
    assert!(object.is_kind_name("StepVisualTessellatedItem"));
}

#[test]
fn init_round_trip_step_visual_coordinates_list() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_points = Handle::new(HArray1::<Xyz>::with_bounds(1, 2).unwrap());
    let object = StepVisualCoordinatesList::new();
    object.init(&the_name, &the_points);
    assert!(object.points() == the_points);
}

#[test]
fn runtime_type_step_visual_cubic_bezier_tessellated_edge() {
    let object = Handle::new(StepVisualCubicBezierTessellatedEdge::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCubicBezierTessellatedEdge"
    );
    assert!(object.is_kind_name("StepVisualTessellatedEdge"));
}

#[test]
fn runtime_type_step_visual_cubic_bezier_triangulated_face() {
    let object = Handle::new(StepVisualCubicBezierTriangulatedFace::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCubicBezierTriangulatedFace"
    );
    assert!(object.is_kind_name("StepVisualTessellatedFace"));
}

#[test]
fn init_round_trip_step_visual_cubic_bezier_triangulated_face() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_tessellated_face_coordinates = Handle::new(StepVisualCoordinatesList::new());
    let the_tessellated_face_pnmax = 7;
    let the_tessellated_face_normals =
        Handle::new(HArray2::<f64>::with_bounds(1, 2, 1, 2).unwrap());
    let the_has_tessellated_face_geometric_link = true;
    let the_tessellated_face_geometric_link = StepVisualFaceOrSurface::default();
    let the_ctriangles = Handle::new(HArray2::<i32>::with_bounds(1, 2, 1, 2).unwrap());
    let object = StepVisualCubicBezierTriangulatedFace::new();
    object.init(
        &the_representation_item_name,
        &the_tessellated_face_coordinates,
        the_tessellated_face_pnmax,
        &the_tessellated_face_normals,
        the_has_tessellated_face_geometric_link,
        &the_tessellated_face_geometric_link,
        &the_ctriangles,
    );
    assert!(object.ctriangles() == the_ctriangles);
}

#[test]
fn runtime_type_step_visual_curve_style() {
    let object = Handle::new(StepVisualCurveStyle::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCurveStyle"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_curve_style() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_curve_font = StepVisualCurveStyleFontSelect::default();
    let a_curve_width = StepBasicSizeSelect::default();
    let a_curve_colour = Handle::new(StepVisualColour::new());
    let object = StepVisualCurveStyle::new();
    object.init(&a_name, &a_curve_font, &a_curve_width, &a_curve_colour);
    assert!(object.name() == a_name);
    assert!(object.curve_colour() == a_curve_colour);
}

#[test]
fn runtime_type_step_visual_curve_style_font() {
    let object = Handle::new(StepVisualCurveStyleFont::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCurveStyleFont"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_curve_style_font() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_pattern_list =
        Handle::new(HArray1::<Handle<StepVisualCurveStyleFontPattern>>::with_bounds(1, 2).unwrap());
    let object = StepVisualCurveStyleFont::new();
    object.init(&a_name, &a_pattern_list);
    assert!(object.name() == a_name);
    assert!(object.pattern_list() == a_pattern_list);
}

#[test]
fn runtime_type_step_visual_curve_style_font_pattern() {
    let object = Handle::new(StepVisualCurveStyleFontPattern::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualCurveStyleFontPattern"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_curve_style_font_pattern() {
    let a_visible_segment_length = 2.5;
    let a_invisible_segment_length = 2.5;
    let object = StepVisualCurveStyleFontPattern::new();
    object.init(a_visible_segment_length, a_invisible_segment_length);
    assert!(object.visible_segment_length() == a_visible_segment_length);
    assert!(object.invisible_segment_length() == a_invisible_segment_length);
}

#[test]
fn runtime_type_step_visual_draughting_annotation_occurrence() {
    let object = Handle::new(StepVisualDraughtingAnnotationOccurrence::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualDraughtingAnnotationOccurrence"
    );
    assert!(object.is_kind_name("StepVisualAnnotationOccurrence"));
}

#[test]
fn runtime_type_step_visual_draughting_callout() {
    let object = Handle::new(StepVisualDraughtingCallout::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualDraughtingCallout"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_visual_draughting_callout() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_contents =
        Handle::new(HArray1::<StepVisualDraughtingCalloutElement>::with_bounds(1, 2).unwrap());
    let object = StepVisualDraughtingCallout::new();
    object.init(&the_name, &the_contents);
    assert!(object.contents() == the_contents);
}

#[test]
fn runtime_type_step_visual_draughting_model() {
    let object = Handle::new(StepVisualDraughtingModel::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualDraughtingModel"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn runtime_type_step_visual_draughting_pre_defined_colour() {
    let object = Handle::new(StepVisualDraughtingPreDefinedColour::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualDraughtingPreDefinedColour"
    );
    assert!(object.is_kind_name("StepVisualPreDefinedColour"));
}

#[test]
fn runtime_type_step_visual_draughting_pre_defined_curve_font() {
    let object = Handle::new(StepVisualDraughtingPreDefinedCurveFont::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualDraughtingPreDefinedCurveFont"
    );
    assert!(object.is_kind_name("StepVisualPreDefinedCurveFont"));
}

#[test]
fn runtime_type_step_visual_externally_defined_curve_font() {
    let object = Handle::new(StepVisualExternallyDefinedCurveFont::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualExternallyDefinedCurveFont"
    );
    assert!(object.is_kind_name("StepBasicExternallyDefinedItem"));
}

#[test]
fn runtime_type_step_visual_externally_defined_text_font() {
    let object = Handle::new(StepVisualExternallyDefinedTextFont::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualExternallyDefinedTextFont"
    );
    assert!(object.is_kind_name("StepBasicExternallyDefinedItem"));
}

#[test]
fn runtime_type_step_visual_fill_area_style() {
    let object = Handle::new(StepVisualFillAreaStyle::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualFillAreaStyle"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_fill_area_style() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_fill_styles =
        Handle::new(HArray1::<StepVisualFillStyleSelect>::with_bounds(1, 2).unwrap());
    let object = StepVisualFillAreaStyle::new();
    object.init(&a_name, &a_fill_styles);
    assert!(object.name() == a_name);
    assert!(object.fill_styles() == a_fill_styles);
}

#[test]
fn runtime_type_step_visual_fill_area_style_colour() {
    let object = Handle::new(StepVisualFillAreaStyleColour::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualFillAreaStyleColour"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_fill_area_style_colour() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_fill_colour = Handle::new(StepVisualColour::new());
    let object = StepVisualFillAreaStyleColour::new();
    object.init(&a_name, &a_fill_colour);
    assert!(object.name() == a_name);
    assert!(object.fill_colour() == a_fill_colour);
}

#[test]
fn runtime_type_step_visual_invisibility() {
    let object = Handle::new(StepVisualInvisibility::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualInvisibility"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_invisibility() {
    let a_invisible_items =
        Handle::new(HArray1::<StepVisualInvisibleItem>::with_bounds(1, 2).unwrap());
    let object = StepVisualInvisibility::new();
    object.init(&a_invisible_items);
    assert!(object.invisible_items() == a_invisible_items);
}

#[test]
fn runtime_type_step_visual_mechanical_design_geometric_presentation_area() {
    let object = Handle::new(StepVisualMechanicalDesignGeometricPresentationArea::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualMechanicalDesignGeometricPresentationArea"
    );
    assert!(object.is_kind_name("StepVisualPresentationArea"));
}

#[test]
fn runtime_type_step_visual_mechanical_design_geometric_presentation_representation() {
    let object = Handle::new(StepVisualMechanicalDesignGeometricPresentationRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualMechanicalDesignGeometricPresentationRepresentation"
    );
    assert!(object.is_kind_name("StepVisualPresentationRepresentation"));
}

#[test]
fn runtime_type_step_visual_over_riding_styled_item() {
    let object = Handle::new(StepVisualOverRidingStyledItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualOverRidingStyledItem"
    );
    assert!(object.is_kind_name("StepVisualStyledItem"));
}
