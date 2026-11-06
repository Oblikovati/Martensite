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
#[allow(unused_imports)]
use crate::step_geom::*;

#[test]
fn runtime_type_step_visual_planar_box() {
    let object = Handle::new(StepVisualPlanarBox::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepVisualPlanarBox");
    assert!(object.is_kind_name("StepVisualPlanarExtent"));
}

#[test]
fn init_round_trip_step_visual_planar_box() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_size_in_x = 2.5;
    let a_size_in_y = 2.5;
    let a_placement = StepGeomAxis2Placement::default();
    let object = StepVisualPlanarBox::new();
    object.init(&a_name, a_size_in_x, a_size_in_y, &a_placement);
}

#[test]
fn runtime_type_step_visual_planar_extent() {
    let object = Handle::new(StepVisualPlanarExtent::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPlanarExtent"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_visual_planar_extent() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_size_in_x = 2.5;
    let a_size_in_y = 2.5;
    let object = StepVisualPlanarExtent::new();
    object.init(&a_name, a_size_in_x, a_size_in_y);
    assert!(object.size_in_x() == a_size_in_x);
    assert!(object.size_in_y() == a_size_in_y);
}

#[test]
fn runtime_type_step_visual_point_style() {
    let object = Handle::new(StepVisualPointStyle::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPointStyle"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_point_style() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_marker = StepVisualMarkerSelect::default();
    let a_marker_size = StepBasicSizeSelect::default();
    let a_marker_colour = Handle::new(StepVisualColour::new());
    let object = StepVisualPointStyle::new();
    object.init(&a_name, &a_marker, &a_marker_size, &a_marker_colour);
    assert!(object.name() == a_name);
    assert!(object.marker_colour() == a_marker_colour);
}

#[test]
fn runtime_type_step_visual_pre_defined_colour() {
    let object = Handle::new(StepVisualPreDefinedColour::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPreDefinedColour"
    );
    assert!(object.is_kind_name("StepVisualColour"));
}

#[test]
fn runtime_type_step_visual_pre_defined_curve_font() {
    let object = Handle::new(StepVisualPreDefinedCurveFont::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPreDefinedCurveFont"
    );
    assert!(object.is_kind_name("StepVisualPreDefinedItem"));
}

#[test]
fn runtime_type_step_visual_pre_defined_item() {
    let object = Handle::new(StepVisualPreDefinedItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPreDefinedItem"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_pre_defined_item() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let object = StepVisualPreDefinedItem::new();
    object.init(&a_name);
    assert!(object.name() == a_name);
}

#[test]
fn runtime_type_step_visual_pre_defined_text_font() {
    let object = Handle::new(StepVisualPreDefinedTextFont::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPreDefinedTextFont"
    );
    assert!(object.is_kind_name("StepVisualPreDefinedItem"));
}

#[test]
fn runtime_type_step_visual_presentation_area() {
    let object = Handle::new(StepVisualPresentationArea::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPresentationArea"
    );
    assert!(object.is_kind_name("StepVisualPresentationRepresentation"));
}

#[test]
fn runtime_type_step_visual_presentation_layer_assignment() {
    let object = Handle::new(StepVisualPresentationLayerAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPresentationLayerAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_presentation_layer_assignment() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_assigned_items =
        Handle::new(HArray1::<StepVisualLayeredItem>::with_bounds(1, 2).unwrap());
    let object = StepVisualPresentationLayerAssignment::new();
    object.init(&a_name, &a_description, &a_assigned_items);
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(object.assigned_items() == a_assigned_items);
}

#[test]
fn runtime_type_step_visual_presentation_layer_usage() {
    let object = Handle::new(StepVisualPresentationLayerUsage::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPresentationLayerUsage"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_presentation_layer_usage() {
    let a_assignment = Handle::new(StepVisualPresentationLayerAssignment::new());
    let a_presentation = Handle::new(StepVisualPresentationRepresentation::new());
    let object = StepVisualPresentationLayerUsage::new();
    object.init(&a_assignment, &a_presentation);
    assert!(object.assignment() == a_assignment);
    assert!(object.presentation() == a_presentation);
}

#[test]
fn runtime_type_step_visual_presentation_representation() {
    let object = Handle::new(StepVisualPresentationRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPresentationRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn runtime_type_step_visual_presentation_set() {
    let object = Handle::new(StepVisualPresentationSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPresentationSet"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn runtime_type_step_visual_presentation_size() {
    let object = Handle::new(StepVisualPresentationSize::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPresentationSize"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_presentation_size() {
    let a_unit = StepVisualPresentationSizeAssignmentSelect::default();
    let a_size = Handle::new(StepVisualPlanarBox::new());
    let object = StepVisualPresentationSize::new();
    object.init(&a_unit, &a_size);
    assert!(object.size() == a_size);
}

#[test]
fn runtime_type_step_visual_presentation_style_assignment() {
    let object = Handle::new(StepVisualPresentationStyleAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPresentationStyleAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_presentation_style_assignment() {
    let a_styles =
        Handle::new(HArray1::<StepVisualPresentationStyleSelect>::with_bounds(1, 2).unwrap());
    let object = StepVisualPresentationStyleAssignment::new();
    object.init(&a_styles);
    assert!(object.styles() == a_styles);
}

#[test]
fn runtime_type_step_visual_presentation_style_by_context() {
    let object = Handle::new(StepVisualPresentationStyleByContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPresentationStyleByContext"
    );
    assert!(object.is_kind_name("StepVisualPresentationStyleAssignment"));
}

#[test]
fn init_round_trip_step_visual_presentation_style_by_context() {
    let a_styles =
        Handle::new(HArray1::<StepVisualPresentationStyleSelect>::with_bounds(1, 2).unwrap());
    let a_style_context = StepVisualStyleContextSelect::default();
    let object = StepVisualPresentationStyleByContext::new();
    object.init(&a_styles, &a_style_context);
}

#[test]
fn runtime_type_step_visual_presentation_view() {
    let object = Handle::new(StepVisualPresentationView::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPresentationView"
    );
    assert!(object.is_kind_name("StepVisualPresentationRepresentation"));
}

#[test]
fn runtime_type_step_visual_presented_item() {
    let object = Handle::new(StepVisualPresentedItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPresentedItem"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn runtime_type_step_visual_presented_item_representation() {
    let object = Handle::new(StepVisualPresentedItemRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualPresentedItemRepresentation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_presented_item_representation() {
    let a_presentation = StepVisualPresentationRepresentationSelect::default();
    let a_item = Handle::new(StepVisualPresentedItem::new());
    let object = StepVisualPresentedItemRepresentation::new();
    object.init(&a_presentation, &a_item);
    assert!(object.item() == a_item);
}

#[test]
fn runtime_type_step_visual_repositioned_tessellated_geometric_set() {
    let object = Handle::new(StepVisualRepositionedTessellatedGeometricSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualRepositionedTessellatedGeometricSet"
    );
    assert!(object.is_kind_name("StepVisualTessellatedGeometricSet"));
}

#[test]
fn runtime_type_step_visual_repositioned_tessellated_item() {
    let object = Handle::new(StepVisualRepositionedTessellatedItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualRepositionedTessellatedItem"
    );
    assert!(object.is_kind_name("StepVisualTessellatedItem"));
}

#[test]
fn init_round_trip_step_visual_repositioned_tessellated_item() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_location = Handle::new(StepGeomAxis2Placement3d::new());
    let object = StepVisualRepositionedTessellatedItem::new();
    object.init(&the_name, &the_location);
    assert!(object.location() == the_location);
}

#[test]
fn runtime_type_step_visual_styled_item() {
    let object = Handle::new(StepVisualStyledItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualStyledItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn runtime_type_step_visual_surface_side_style() {
    let object = Handle::new(StepVisualSurfaceSideStyle::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceSideStyle"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_surface_side_style() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_styles =
        Handle::new(HArray1::<StepVisualSurfaceStyleElementSelect>::with_bounds(1, 2).unwrap());
    let object = StepVisualSurfaceSideStyle::new();
    object.init(&a_name, &a_styles);
    assert!(object.name() == a_name);
    assert!(object.styles() == a_styles);
}

#[test]
fn runtime_type_step_visual_surface_style_boundary() {
    let object = Handle::new(StepVisualSurfaceStyleBoundary::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleBoundary"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_surface_style_boundary() {
    let a_style_of_boundary = Handle::new(StepVisualCurveStyle::new());
    let object = StepVisualSurfaceStyleBoundary::new();
    object.init(&a_style_of_boundary);
    assert!(object.style_of_boundary() == a_style_of_boundary);
}

#[test]
fn runtime_type_step_visual_surface_style_control_grid() {
    let object = Handle::new(StepVisualSurfaceStyleControlGrid::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleControlGrid"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}
