//! The classes of IgesDraw: what their initialization keeps and their round through a file.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_test_samples::{
    assert_entity_round, sample_iges_draw_circ_array_subfigure, sample_iges_draw_connect_point,
    sample_iges_draw_drawing, sample_iges_draw_drawing_with_rotation,
    sample_iges_draw_label_display, sample_iges_draw_network_subfigure,
    sample_iges_draw_network_subfigure_def, sample_iges_draw_perspective_view,
    sample_iges_draw_planar, sample_iges_draw_rect_array_subfigure,
    sample_iges_draw_segmented_views_visible, sample_iges_draw_view,
    sample_iges_draw_views_visible, sample_iges_draw_views_visible_with_attr,
};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Xyz;

#[test]
fn class_circ_array_subfigure() -> Result<(), Failure> {
    let object = sample_iges_draw_circ_array_subfigure()?;
    assert_eq!(object.nb_locations(), 2);
    assert_eq!(object.circle_radius(), 4.5);
    assert_eq!(object.start_angle(), 5.5);
    assert_eq!(object.delta_angle(), 6.5);
    assert_entity_round(&object, "IgesDrawCircArraySubfigure")?;
    Ok(())
}

#[test]
fn class_connect_point() -> Result<(), Failure> {
    let object = sample_iges_draw_connect_point()?;
    assert_eq!(object.type_flag(), 2);
    assert_eq!(object.function_flag(), 2);
    assert_eq!(object.point_identifier(), 2);
    assert_eq!(object.function_code(), 2);
    assert_entity_round(&object, "IgesDrawConnectPoint")?;
    Ok(())
}

#[test]
fn class_drawing() -> Result<(), Failure> {
    let object = sample_iges_draw_drawing()?;
    assert_entity_round(&object, "IgesDrawDrawing")?;
    Ok(())
}

#[test]
fn class_drawing_with_rotation() -> Result<(), Failure> {
    let object = sample_iges_draw_drawing_with_rotation()?;
    assert_entity_round(&object, "IgesDrawDrawingWithRotation")?;
    Ok(())
}

#[test]
fn class_label_display() -> Result<(), Failure> {
    let object = sample_iges_draw_label_display()?;
    assert_entity_round(&object, "IgesDrawLabelDisplay")?;
    Ok(())
}

#[test]
fn class_network_subfigure() -> Result<(), Failure> {
    let object = sample_iges_draw_network_subfigure()?;
    assert_eq!(object.translation(), Xyz::new(1.25, 1.5, 2.5));
    assert_eq!(object.scale_factors(), Xyz::new(2.25, 1.5, 2.5));
    assert_eq!(object.type_flag(), 2);
    assert_entity_round(&object, "IgesDrawNetworkSubfigure")?;
    Ok(())
}

#[test]
fn class_network_subfigure_def() -> Result<(), Failure> {
    let object = sample_iges_draw_network_subfigure_def()?;
    assert_eq!(object.depth(), 2);
    assert_eq!(object.type_flag(), 2);
    assert_entity_round(&object, "IgesDrawNetworkSubfigureDef")?;
    Ok(())
}

#[test]
fn class_perspective_view() -> Result<(), Failure> {
    let object = sample_iges_draw_perspective_view()?;
    assert_eq!(object.view_number(), 2);
    assert_eq!(object.scale_factor(), 2.5);
    assert_eq!(object.view_plane_distance(), 7.5);
    assert_eq!(object.depth_clip(), 2);
    assert_eq!(object.back_plane_distance(), 11.5);
    assert_eq!(object.front_plane_distance(), 12.5);
    assert_entity_round(&object, "IgesDrawPerspectiveView")?;
    Ok(())
}

#[test]
fn class_planar() -> Result<(), Failure> {
    let object = sample_iges_draw_planar()?;
    assert_eq!(object.nb_matrices(), 1);
    assert_entity_round(&object, "IgesDrawPlanar")?;
    Ok(())
}

#[test]
fn class_rect_array_subfigure() -> Result<(), Failure> {
    let object = sample_iges_draw_rect_array_subfigure()?;
    assert_eq!(object.scale_factor(), 2.5);
    assert_eq!(object.nb_columns(), 2);
    assert_eq!(object.nb_rows(), 2);
    assert_eq!(object.column_separation(), 6.5);
    assert_eq!(object.row_separation(), 7.5);
    assert_eq!(object.rotation_angle(), 8.5);
    assert_entity_round(&object, "IgesDrawRectArraySubfigure")?;
    Ok(())
}

#[test]
fn class_segmented_views_visible() -> Result<(), Failure> {
    let object = sample_iges_draw_segmented_views_visible()?;
    assert_entity_round(&object, "IgesDrawSegmentedViewsVisible")?;
    Ok(())
}

#[test]
fn class_view() -> Result<(), Failure> {
    let object = sample_iges_draw_view()?;
    assert_eq!(object.view_number(), 2);
    assert_eq!(object.scale_factor(), 2.5);
    assert_entity_round(&object, "IgesDrawView")?;
    Ok(())
}

#[test]
fn class_views_visible() -> Result<(), Failure> {
    let object = sample_iges_draw_views_visible()?;
    assert_entity_round(&object, "IgesDrawViewsVisible")?;
    Ok(())
}

#[test]
fn class_views_visible_with_attr() -> Result<(), Failure> {
    let object = sample_iges_draw_views_visible_with_attr()?;
    assert_entity_round(&object, "IgesDrawViewsVisibleWithAttr")?;
    Ok(())
}
