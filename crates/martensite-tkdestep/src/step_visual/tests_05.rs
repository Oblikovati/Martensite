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
#[allow(unused_imports)]
use crate::step_repr::*;
#[allow(unused_imports)]
use crate::step_shape::*;

#[test]
fn runtime_type_step_visual_tessellated_geometric_set() {
    let object = Handle::new(StepVisualTessellatedGeometricSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedGeometricSet"
    );
    assert!(object.is_kind_name("StepVisualTessellatedItem"));
}

#[test]
fn runtime_type_step_visual_tessellated_item() {
    let object = Handle::new(StepVisualTessellatedItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedItem"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn runtime_type_step_visual_tessellated_point_set() {
    let object = Handle::new(StepVisualTessellatedPointSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedPointSet"
    );
    assert!(object.is_kind_name("StepVisualTessellatedItem"));
}

#[test]
fn init_round_trip_step_visual_tessellated_point_set() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_coordinates = Handle::new(StepVisualCoordinatesList::new());
    let the_point_list = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let object = StepVisualTessellatedPointSet::new();
    object.init(
        &the_representation_item_name,
        &the_coordinates,
        &the_point_list,
    );
    assert!(object.coordinates() == the_coordinates);
    assert!(object.point_list() == the_point_list);
}

#[test]
fn runtime_type_step_visual_tessellated_shape_representation() {
    let object = Handle::new(StepVisualTessellatedShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedShapeRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_visual_tessellated_shape_representation_with_accuracy_parameters() {
    let object = Handle::new(StepVisualTessellatedShapeRepresentationWithAccuracyParameters::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedShapeRepresentationWithAccuracyParameters"
    );
    assert!(object.is_kind_name("StepVisualTessellatedShapeRepresentation"));
}

#[test]
fn init_round_trip_step_visual_tessellated_shape_representation_with_accuracy_parameters() {
    let the_representation_name = Handle::new(HAsciiString::new("sample"));
    let the_representation_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let the_representation_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let the_tessellation_accuracy_parameters =
        Handle::new(HArray1::<f64>::with_bounds(1, 2).unwrap());
    let object = StepVisualTessellatedShapeRepresentationWithAccuracyParameters::new();
    object.init(
        &the_representation_name,
        &the_representation_items,
        &the_representation_context_of_items,
        &the_tessellation_accuracy_parameters,
    );
    assert!(object.tessellation_accuracy_parameters() == the_tessellation_accuracy_parameters);
}

#[test]
fn runtime_type_step_visual_tessellated_shell() {
    let object = Handle::new(StepVisualTessellatedShell::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedShell"
    );
    assert!(object.is_kind_name("StepVisualTessellatedItem"));
}

#[test]
fn init_round_trip_step_visual_tessellated_shell() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_items = Handle::new(
        HArray1::<Handle<StepVisualTessellatedStructuredItem>>::with_bounds(1, 2).unwrap(),
    );
    let the_has_topological_link = true;
    let the_topological_link = Handle::new(StepShapeConnectedFaceSet::new());
    let object = StepVisualTessellatedShell::new();
    object.init(
        &the_representation_item_name,
        &the_items,
        the_has_topological_link,
        &the_topological_link,
    );
    assert!(object.items() == the_items);
    assert!(object.has_topological_link() == the_has_topological_link);
    assert!(object.topological_link() == the_topological_link);
}

#[test]
fn runtime_type_step_visual_tessellated_solid() {
    let object = Handle::new(StepVisualTessellatedSolid::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedSolid"
    );
    assert!(object.is_kind_name("StepVisualTessellatedItem"));
}

#[test]
fn init_round_trip_step_visual_tessellated_solid() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_items = Handle::new(
        HArray1::<Handle<StepVisualTessellatedStructuredItem>>::with_bounds(1, 2).unwrap(),
    );
    let the_has_geometric_link = true;
    let the_geometric_link = Handle::new(StepShapeManifoldSolidBrep::new());
    let object = StepVisualTessellatedSolid::new();
    object.init(
        &the_representation_item_name,
        &the_items,
        the_has_geometric_link,
        &the_geometric_link,
    );
    assert!(object.items() == the_items);
    assert!(object.has_geometric_link() == the_has_geometric_link);
    assert!(object.geometric_link() == the_geometric_link);
}

#[test]
fn runtime_type_step_visual_tessellated_structured_item() {
    let object = Handle::new(StepVisualTessellatedStructuredItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedStructuredItem"
    );
    assert!(object.is_kind_name("StepVisualTessellatedItem"));
}

#[test]
fn runtime_type_step_visual_tessellated_surface_set() {
    let object = Handle::new(StepVisualTessellatedSurfaceSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedSurfaceSet"
    );
    assert!(object.is_kind_name("StepVisualTessellatedItem"));
}

#[test]
fn init_round_trip_step_visual_tessellated_surface_set() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_coordinates = Handle::new(StepVisualCoordinatesList::new());
    let the_pnmax = 7;
    let the_normals = Handle::new(HArray2::<f64>::with_bounds(1, 2, 1, 2).unwrap());
    let object = StepVisualTessellatedSurfaceSet::new();
    object.init(
        &the_representation_item_name,
        &the_coordinates,
        the_pnmax,
        &the_normals,
    );
    assert!(object.coordinates() == the_coordinates);
    assert!(object.pnmax() == the_pnmax);
    assert!(object.normals() == the_normals);
}

#[test]
fn runtime_type_step_visual_tessellated_vertex() {
    let object = Handle::new(StepVisualTessellatedVertex::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedVertex"
    );
    assert!(object.is_kind_name("StepVisualTessellatedStructuredItem"));
}

#[test]
fn init_round_trip_step_visual_tessellated_vertex() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_coordinates = Handle::new(StepVisualCoordinatesList::new());
    let the_has_topological_link = true;
    let the_topological_link = Handle::new(StepShapeVertexPoint::new());
    let the_point_index = 7;
    let object = StepVisualTessellatedVertex::new();
    object.init(
        &the_representation_item_name,
        &the_coordinates,
        the_has_topological_link,
        &the_topological_link,
        the_point_index,
    );
    assert!(object.coordinates() == the_coordinates);
    assert!(object.has_topological_link() == the_has_topological_link);
    assert!(object.topological_link() == the_topological_link);
    assert!(object.point_index() == the_point_index);
}

#[test]
fn runtime_type_step_visual_tessellated_wire() {
    let object = Handle::new(StepVisualTessellatedWire::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedWire"
    );
    assert!(object.is_kind_name("StepVisualTessellatedItem"));
}

#[test]
fn init_round_trip_step_visual_tessellated_wire() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_items =
        Handle::new(HArray1::<StepVisualTessellatedEdgeOrVertex>::with_bounds(1, 2).unwrap());
    let the_has_geometric_model_link = true;
    let the_geometric_model_link = StepVisualPathOrCompositeCurve::default();
    let object = StepVisualTessellatedWire::new();
    object.init(
        &the_representation_item_name,
        &the_items,
        the_has_geometric_model_link,
        &the_geometric_model_link,
    );
    assert!(object.items() == the_items);
    assert!(object.has_geometric_model_link() == the_has_geometric_model_link);
}

#[test]
fn runtime_type_step_visual_text_literal() {
    let object = Handle::new(StepVisualTextLiteral::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTextLiteral"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_visual_text_literal() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_literal = Handle::new(HAsciiString::new("sample"));
    let a_placement = StepGeomAxis2Placement::default();
    let a_alignment = Handle::new(HAsciiString::new("sample"));
    let a_path = StepVisualTextPath::default();
    let a_font = StepVisualFontSelect::default();
    let object = StepVisualTextLiteral::new();
    object.init(
        &a_name,
        &a_literal,
        &a_placement,
        &a_alignment,
        a_path,
        &a_font,
    );
    assert!(object.literal() == a_literal);
    assert!(object.alignment() == a_alignment);
    assert!(object.path() == a_path);
}

#[test]
fn runtime_type_step_visual_text_style() {
    let object = Handle::new(StepVisualTextStyle::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepVisualTextStyle");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_text_style() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_character_appearance = Handle::new(StepVisualTextStyleForDefinedFont::new());
    let object = StepVisualTextStyle::new();
    object.init(&a_name, &a_character_appearance);
    assert!(object.name() == a_name);
    assert!(object.character_appearance() == a_character_appearance);
}

#[test]
fn runtime_type_step_visual_text_style_for_defined_font() {
    let object = Handle::new(StepVisualTextStyleForDefinedFont::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTextStyleForDefinedFont"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_text_style_for_defined_font() {
    let a_text_colour = Handle::new(StepVisualColour::new());
    let object = StepVisualTextStyleForDefinedFont::new();
    object.init(&a_text_colour);
    assert!(object.text_colour() == a_text_colour);
}

#[test]
fn runtime_type_step_visual_text_style_with_box_characteristics() {
    let object = Handle::new(StepVisualTextStyleWithBoxCharacteristics::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTextStyleWithBoxCharacteristics"
    );
    assert!(object.is_kind_name("StepVisualTextStyle"));
}

#[test]
fn init_round_trip_step_visual_text_style_with_box_characteristics() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_character_appearance = Handle::new(StepVisualTextStyleForDefinedFont::new());
    let a_characteristics =
        Handle::new(HArray1::<StepVisualBoxCharacteristicSelect>::with_bounds(1, 2).unwrap());
    let object = StepVisualTextStyleWithBoxCharacteristics::new();
    object.init(&a_name, &a_character_appearance, &a_characteristics);
    assert!(object.characteristics() == a_characteristics);
}

#[test]
fn runtime_type_step_visual_triangulated_face() {
    let object = Handle::new(StepVisualTriangulatedFace::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTriangulatedFace"
    );
    assert!(object.is_kind_name("StepVisualTessellatedFace"));
}

#[test]
fn init_round_trip_step_visual_triangulated_face() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_tessellated_face_coordinates = Handle::new(StepVisualCoordinatesList::new());
    let the_tessellated_face_pnmax = 7;
    let the_tessellated_face_normals =
        Handle::new(HArray2::<f64>::with_bounds(1, 2, 1, 2).unwrap());
    let the_has_tessellated_face_geometric_link = true;
    let the_tessellated_face_geometric_link = StepVisualFaceOrSurface::default();
    let the_pnindex = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let the_triangles = Handle::new(HArray2::<i32>::with_bounds(1, 2, 1, 2).unwrap());
    let object = StepVisualTriangulatedFace::new();
    object.init(
        &the_representation_item_name,
        &the_tessellated_face_coordinates,
        the_tessellated_face_pnmax,
        &the_tessellated_face_normals,
        the_has_tessellated_face_geometric_link,
        &the_tessellated_face_geometric_link,
        &the_pnindex,
        &the_triangles,
    );
    assert!(object.pnindex() == the_pnindex);
    assert!(object.triangles() == the_triangles);
}

#[test]
fn runtime_type_step_visual_triangulated_surface_set() {
    let object = Handle::new(StepVisualTriangulatedSurfaceSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTriangulatedSurfaceSet"
    );
    assert!(object.is_kind_name("StepVisualTessellatedSurfaceSet"));
}

#[test]
fn init_round_trip_step_visual_triangulated_surface_set() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_tessellated_surface_set_coordinates = Handle::new(StepVisualCoordinatesList::new());
    let the_tessellated_surface_set_pnmax = 7;
    let the_tessellated_surface_set_normals =
        Handle::new(HArray2::<f64>::with_bounds(1, 2, 1, 2).unwrap());
    let the_pnindex = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let the_triangles = Handle::new(HArray2::<i32>::with_bounds(1, 2, 1, 2).unwrap());
    let object = StepVisualTriangulatedSurfaceSet::new();
    object.init(
        &the_representation_item_name,
        &the_tessellated_surface_set_coordinates,
        the_tessellated_surface_set_pnmax,
        &the_tessellated_surface_set_normals,
        &the_pnindex,
        &the_triangles,
    );
    assert!(object.pnindex() == the_pnindex);
    assert!(object.triangles() == the_triangles);
}

#[test]
fn runtime_type_step_visual_view_volume() {
    let object = Handle::new(StepVisualViewVolume::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualViewVolume"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}
