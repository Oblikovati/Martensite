#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn init_round_trip_step_visual_surface_style_control_grid() {
    let a_style_of_control_grid = Handle::new(StepVisualCurveStyle::new());
    let object = StepVisualSurfaceStyleControlGrid::new();
    object.init(&a_style_of_control_grid);
    assert!(object.style_of_control_grid() == a_style_of_control_grid);
}

#[test]
fn runtime_type_step_visual_surface_style_fill_area() {
    let object = Handle::new(StepVisualSurfaceStyleFillArea::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleFillArea"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_surface_style_fill_area() {
    let a_fill_area = Handle::new(StepVisualFillAreaStyle::new());
    let object = StepVisualSurfaceStyleFillArea::new();
    object.init(&a_fill_area);
    assert!(object.fill_area() == a_fill_area);
}

#[test]
fn runtime_type_step_visual_surface_style_parameter_line() {
    let object = Handle::new(StepVisualSurfaceStyleParameterLine::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleParameterLine"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_surface_style_parameter_line() {
    let a_style_of_parameter_lines = Handle::new(StepVisualCurveStyle::new());
    let a_direction_counts =
        Handle::new(HArray1::<StepVisualDirectionCountSelect>::with_bounds(1, 2).unwrap());
    let object = StepVisualSurfaceStyleParameterLine::new();
    object.init(&a_style_of_parameter_lines, &a_direction_counts);
    assert!(object.style_of_parameter_lines() == a_style_of_parameter_lines);
    assert!(object.direction_counts() == a_direction_counts);
}

#[test]
fn runtime_type_step_visual_surface_style_reflectance_ambient() {
    let object = Handle::new(StepVisualSurfaceStyleReflectanceAmbient::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleReflectanceAmbient"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_surface_style_reflectance_ambient() {
    let the_ambient_reflectance = 2.5;
    let object = StepVisualSurfaceStyleReflectanceAmbient::new();
    object.init(the_ambient_reflectance);
    assert!(object.ambient_reflectance() == the_ambient_reflectance);
}

#[test]
fn runtime_type_step_visual_surface_style_reflectance_ambient_diffuse() {
    let object = Handle::new(StepVisualSurfaceStyleReflectanceAmbientDiffuse::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleReflectanceAmbientDiffuse"
    );
    assert!(object.is_kind_name("StepVisualSurfaceStyleReflectanceAmbient"));
}

#[test]
fn init_round_trip_step_visual_surface_style_reflectance_ambient_diffuse() {
    let the_ambient_reflectance = 2.5;
    let the_diffuse_reflectance = 2.5;
    let object = StepVisualSurfaceStyleReflectanceAmbientDiffuse::new();
    object.init(the_ambient_reflectance, the_diffuse_reflectance);
    assert!(object.diffuse_reflectance() == the_diffuse_reflectance);
}

#[test]
fn runtime_type_step_visual_surface_style_reflectance_ambient_diffuse_specular() {
    let object = Handle::new(StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular"
    );
    assert!(object.is_kind_name("StepVisualSurfaceStyleReflectanceAmbientDiffuse"));
}

#[test]
fn init_round_trip_step_visual_surface_style_reflectance_ambient_diffuse_specular() {
    let the_ambient_reflectance = 2.5;
    let the_diffuse_reflectance = 2.5;
    let the_specular_reflectance = 2.5;
    let the_specular_exponent = 2.5;
    let the_specular_colour = Handle::new(StepVisualColour::new());
    let object = StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular::new();
    object.init(
        the_ambient_reflectance,
        the_diffuse_reflectance,
        the_specular_reflectance,
        the_specular_exponent,
        &the_specular_colour,
    );
    assert!(object.specular_reflectance() == the_specular_reflectance);
    assert!(object.specular_exponent() == the_specular_exponent);
    assert!(object.specular_colour() == the_specular_colour);
}

#[test]
fn runtime_type_step_visual_surface_style_rendering() {
    let object = Handle::new(StepVisualSurfaceStyleRendering::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleRendering"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_surface_style_rendering() {
    let the_rendering_method = StepVisualShadingSurfaceMethod::default();
    let the_surface_colour = Handle::new(StepVisualColour::new());
    let object = StepVisualSurfaceStyleRendering::new();
    object.init(the_rendering_method, &the_surface_colour);
    assert!(object.rendering_method() == the_rendering_method);
    assert!(object.surface_colour() == the_surface_colour);
}

#[test]
fn runtime_type_step_visual_surface_style_rendering_with_properties() {
    let object = Handle::new(StepVisualSurfaceStyleRenderingWithProperties::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleRenderingWithProperties"
    );
    assert!(object.is_kind_name("StepVisualSurfaceStyleRendering"));
}

#[test]
fn init_round_trip_step_visual_surface_style_rendering_with_properties() {
    let the_surface_style_rendering_rendering_method = StepVisualShadingSurfaceMethod::default();
    let the_surface_style_rendering_surface_colour = Handle::new(StepVisualColour::new());
    let the_properties =
        Handle::new(HArray1::<StepVisualRenderingPropertiesSelect>::with_bounds(1, 2).unwrap());
    let object = StepVisualSurfaceStyleRenderingWithProperties::new();
    object.init(
        the_surface_style_rendering_rendering_method,
        &the_surface_style_rendering_surface_colour,
        &the_properties,
    );
    assert!(object.properties() == the_properties);
}

#[test]
fn runtime_type_step_visual_surface_style_segmentation_curve() {
    let object = Handle::new(StepVisualSurfaceStyleSegmentationCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleSegmentationCurve"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_surface_style_segmentation_curve() {
    let a_style_of_segmentation_curve = Handle::new(StepVisualCurveStyle::new());
    let object = StepVisualSurfaceStyleSegmentationCurve::new();
    object.init(&a_style_of_segmentation_curve);
    assert!(object.style_of_segmentation_curve() == a_style_of_segmentation_curve);
}

#[test]
fn runtime_type_step_visual_surface_style_silhouette() {
    let object = Handle::new(StepVisualSurfaceStyleSilhouette::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleSilhouette"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_surface_style_silhouette() {
    let a_style_of_silhouette = Handle::new(StepVisualCurveStyle::new());
    let object = StepVisualSurfaceStyleSilhouette::new();
    object.init(&a_style_of_silhouette);
    assert!(object.style_of_silhouette() == a_style_of_silhouette);
}

#[test]
fn runtime_type_step_visual_surface_style_transparent() {
    let object = Handle::new(StepVisualSurfaceStyleTransparent::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleTransparent"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_surface_style_transparent() {
    let the_transparency = 2.5;
    let object = StepVisualSurfaceStyleTransparent::new();
    object.init(the_transparency);
    assert!(object.transparency() == the_transparency);
}

#[test]
fn runtime_type_step_visual_surface_style_usage() {
    let object = Handle::new(StepVisualSurfaceStyleUsage::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualSurfaceStyleUsage"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_visual_surface_style_usage() {
    let a_side = StepVisualSurfaceSide::default();
    let a_style = Handle::new(StepVisualSurfaceSideStyle::new());
    let object = StepVisualSurfaceStyleUsage::new();
    object.init(a_side, &a_style);
    assert!(object.side() == a_side);
    assert!(object.style() == a_style);
}

#[test]
fn runtime_type_step_visual_template() {
    let object = Handle::new(StepVisualTemplate::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepVisualTemplate");
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn runtime_type_step_visual_template_instance() {
    let object = Handle::new(StepVisualTemplateInstance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTemplateInstance"
    );
    assert!(object.is_kind_name("StepReprMappedItem"));
}

#[test]
fn runtime_type_step_visual_tessellated_annotation_occurrence() {
    let object = Handle::new(StepVisualTessellatedAnnotationOccurrence::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedAnnotationOccurrence"
    );
    assert!(object.is_kind_name("StepVisualStyledItem"));
}

#[test]
fn runtime_type_step_visual_tessellated_connecting_edge() {
    let object = Handle::new(StepVisualTessellatedConnectingEdge::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedConnectingEdge"
    );
    assert!(object.is_kind_name("StepVisualTessellatedEdge"));
}

#[test]
fn init_round_trip_step_visual_tessellated_connecting_edge() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_tessellated_edge_coordinates = Handle::new(StepVisualCoordinatesList::new());
    let the_has_tessellated_edge_geometric_link = true;
    let the_tessellated_edge_geometric_link = StepVisualEdgeOrCurve::default();
    let the_tessellated_edge_line_strip = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let the_smooth = StepDataLogical::default();
    let the_face1 = Handle::new(StepVisualTessellatedFace::new());
    let the_face2 = Handle::new(StepVisualTessellatedFace::new());
    let the_line_strip_face1 = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let the_line_strip_face2 = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let object = StepVisualTessellatedConnectingEdge::new();
    object.init(
        &the_representation_item_name,
        &the_tessellated_edge_coordinates,
        the_has_tessellated_edge_geometric_link,
        &the_tessellated_edge_geometric_link,
        &the_tessellated_edge_line_strip,
        the_smooth,
        &the_face1,
        &the_face2,
        &the_line_strip_face1,
        &the_line_strip_face2,
    );
    assert!(object.smooth() == the_smooth);
    assert!(object.face1() == the_face1);
    assert!(object.face2() == the_face2);
    assert!(object.line_strip_face1() == the_line_strip_face1);
    assert!(object.line_strip_face2() == the_line_strip_face2);
}

#[test]
fn runtime_type_step_visual_tessellated_curve_set() {
    let object = Handle::new(StepVisualTessellatedCurveSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedCurveSet"
    );
    assert!(object.is_kind_name("StepVisualTessellatedItem"));
}

#[test]
fn runtime_type_step_visual_tessellated_edge() {
    let object = Handle::new(StepVisualTessellatedEdge::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedEdge"
    );
    assert!(object.is_kind_name("StepVisualTessellatedStructuredItem"));
}

#[test]
fn init_round_trip_step_visual_tessellated_edge() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_coordinates = Handle::new(StepVisualCoordinatesList::new());
    let the_has_geometric_link = true;
    let the_geometric_link = StepVisualEdgeOrCurve::default();
    let the_line_strip = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let object = StepVisualTessellatedEdge::new();
    object.init(
        &the_representation_item_name,
        &the_coordinates,
        the_has_geometric_link,
        &the_geometric_link,
        &the_line_strip,
    );
    assert!(object.coordinates() == the_coordinates);
    assert!(object.has_geometric_link() == the_has_geometric_link);
    assert!(object.line_strip() == the_line_strip);
}

#[test]
fn runtime_type_step_visual_tessellated_face() {
    let object = Handle::new(StepVisualTessellatedFace::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepVisualTessellatedFace"
    );
    assert!(object.is_kind_name("StepVisualTessellatedStructuredItem"));
}

#[test]
fn init_round_trip_step_visual_tessellated_face() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_coordinates = Handle::new(StepVisualCoordinatesList::new());
    let the_pnmax = 7;
    let the_normals = Handle::new(HArray2::<f64>::with_bounds(1, 2, 1, 2).unwrap());
    let the_has_geometric_link = true;
    let the_geometric_link = StepVisualFaceOrSurface::default();
    let object = StepVisualTessellatedFace::new();
    object.init(
        &the_representation_item_name,
        &the_coordinates,
        the_pnmax,
        &the_normals,
        the_has_geometric_link,
        &the_geometric_link,
    );
    assert!(object.coordinates() == the_coordinates);
    assert!(object.pnmax() == the_pnmax);
    assert!(object.normals() == the_normals);
    assert!(object.has_geometric_link() == the_has_geometric_link);
}
