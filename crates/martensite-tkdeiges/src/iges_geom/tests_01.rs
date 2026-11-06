#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_geom::samples_01::{
    sample_iges_geom_b_spline_curve, sample_iges_geom_b_spline_surface, sample_iges_geom_boundary,
    sample_iges_geom_bounded_surface, sample_iges_geom_circular_arc,
    sample_iges_geom_composite_curve, sample_iges_geom_conic_arc, sample_iges_geom_copious_data,
    sample_iges_geom_curve_on_surface, sample_iges_geom_direction, sample_iges_geom_flash,
    sample_iges_geom_line, sample_iges_geom_offset_curve, sample_iges_geom_offset_surface,
    sample_iges_geom_plane, sample_iges_geom_point, sample_iges_geom_ruled_surface,
    sample_iges_geom_spline_curve, sample_iges_geom_spline_surface,
    sample_iges_geom_surface_of_revolution, sample_iges_geom_tabulated_cylinder,
    sample_iges_geom_transformation_matrix, sample_iges_geom_trimmed_surface,
};
use crate::iges_test_samples::assert_entity_round;
use martensite_tkernel::standard::Failure;

#[test]
fn class_iges_geom_b_spline_curve() -> Result<(), Failure> {
    let object = sample_iges_geom_b_spline_curve()?;
    assert!(object.is_planar());
    assert!(object.is_closed());
    assert!(object.is_periodic());
    assert_eq!(object.u_min(), 10.5);
    assert_eq!(object.u_max(), 11.5);
    assert_entity_round(&object, "IgesGeomBSplineCurve")?;
    Ok(())
}
#[test]
fn class_iges_geom_b_spline_surface() -> Result<(), Failure> {
    let object = sample_iges_geom_b_spline_surface()?;
    assert!(object.is_closed_u());
    assert!(object.is_closed_v());
    assert!(object.is_periodic_u());
    assert!(object.is_periodic_v());
    assert_eq!(object.u_min(), 14.5);
    assert_eq!(object.u_max(), 15.5);
    assert_eq!(object.v_min(), 16.5);
    assert_eq!(object.v_max(), 17.5);
    assert_entity_round(&object, "IgesGeomBSplineSurface")?;
    Ok(())
}
#[test]
fn class_iges_geom_boundary() -> Result<(), Failure> {
    let object = sample_iges_geom_boundary()?;
    assert_eq!(object.boundary_type(), 2);
    assert_eq!(object.preference_type(), 3);
    assert_eq!((object.type_number(), object.form_number()), (141, 0));
    assert_entity_round(&object, "IgesGeomBoundary")?;
    Ok(())
}
#[test]
fn class_iges_geom_bounded_surface() -> Result<(), Failure> {
    let object = sample_iges_geom_bounded_surface()?;
    assert_eq!(object.representation_type(), 2);
    assert_eq!((object.type_number(), object.form_number()), (143, 0));
    assert_entity_round(&object, "IgesGeomBoundedSurface")?;
    Ok(())
}
#[test]
fn class_iges_geom_circular_arc() -> Result<(), Failure> {
    let object = sample_iges_geom_circular_arc()?;
    assert_eq!(object.z_plane(), 1.5);
    assert_eq!((object.type_number(), object.form_number()), (100, 0));
    assert_entity_round(&object, "IgesGeomCircularArc")?;
    Ok(())
}
#[test]
fn class_iges_geom_composite_curve() -> Result<(), Failure> {
    let object = sample_iges_geom_composite_curve()?;
    assert_eq!((object.type_number(), object.form_number()), (102, 0));
    assert_entity_round(&object, "IgesGeomCompositeCurve")?;
    Ok(())
}
#[test]
fn class_iges_geom_conic_arc() -> Result<(), Failure> {
    let object = sample_iges_geom_conic_arc()?;
    assert_eq!(object.z_plane(), 7.5);
    assert_entity_round(&object, "IgesGeomConicArc")?;
    Ok(())
}
#[test]
fn class_iges_geom_copious_data() -> Result<(), Failure> {
    let object = sample_iges_geom_copious_data()?;
    assert_eq!(object.data_type(), 2);
    assert_eq!(object.z_plane(), 2.5);
    assert_entity_round(&object, "IgesGeomCopiousData")?;
    Ok(())
}
#[test]
fn class_iges_geom_curve_on_surface() -> Result<(), Failure> {
    let object = sample_iges_geom_curve_on_surface()?;
    assert_eq!(object.creation_mode(), 2);
    assert_eq!(object.preference_mode(), 6);
    assert_eq!((object.type_number(), object.form_number()), (142, 0));
    assert_entity_round(&object, "IgesGeomCurveOnSurface")?;
    Ok(())
}
#[test]
fn class_iges_geom_direction() -> Result<(), Failure> {
    let object = sample_iges_geom_direction()?;
    assert_eq!((object.type_number(), object.form_number()), (123, 0));
    assert_entity_round(&object, "IgesGeomDirection")?;
    Ok(())
}
#[test]
fn class_iges_geom_flash() -> Result<(), Failure> {
    let object = sample_iges_geom_flash()?;
    assert_eq!(object.dimension1(), 2.5);
    assert_eq!(object.dimension2(), 3.5);
    assert_eq!(object.rotation(), 4.5);
    assert_entity_round(&object, "IgesGeomFlash")?;
    Ok(())
}
#[test]
fn class_iges_geom_line() -> Result<(), Failure> {
    let object = sample_iges_geom_line()?;
    assert_eq!((object.type_number(), object.form_number()), (110, 0));
    assert_entity_round(&object, "IgesGeomLine")?;
    Ok(())
}
#[test]
fn class_iges_geom_offset_curve() -> Result<(), Failure> {
    let object = sample_iges_geom_offset_curve()?;
    assert_eq!(object.offset_type(), 3);
    assert_eq!(object.function_parameter(), 5);
    assert_eq!(object.tapered_offset_type(), 6);
    assert_eq!(object.first_offset_distance(), 6.5);
    assert_eq!(object.arc_length1(), 7.5);
    assert_eq!(object.second_offset_distance(), 8.5);
    assert_eq!(object.arc_length2(), 9.5);
    assert_eq!(object.start_parameter(), 11.5);
    assert_eq!(object.end_parameter(), 12.5);
    assert_eq!((object.type_number(), object.form_number()), (130, 0));
    assert_entity_round(&object, "IgesGeomOffsetCurve")?;
    Ok(())
}
#[test]
fn class_iges_geom_offset_surface() -> Result<(), Failure> {
    let object = sample_iges_geom_offset_surface()?;
    assert_eq!(object.distance(), 2.5);
    assert_eq!((object.type_number(), object.form_number()), (140, 0));
    assert_entity_round(&object, "IgesGeomOffsetSurface")?;
    Ok(())
}
#[test]
fn class_iges_geom_plane() -> Result<(), Failure> {
    let object = sample_iges_geom_plane()?;
    assert_eq!(object.symbol_size(), 7.5);
    assert_entity_round(&object, "IgesGeomPlane")?;
    Ok(())
}
#[test]
fn class_iges_geom_point() -> Result<(), Failure> {
    let object = sample_iges_geom_point()?;
    assert_eq!((object.type_number(), object.form_number()), (116, 0));
    assert_entity_round(&object, "IgesGeomPoint")?;
    Ok(())
}
#[test]
fn class_iges_geom_ruled_surface() -> Result<(), Failure> {
    let object = sample_iges_geom_ruled_surface()?;
    assert_eq!(object.direction_flag(), 4);
    assert_entity_round(&object, "IgesGeomRuledSurface")?;
    Ok(())
}
#[test]
fn class_iges_geom_spline_curve() -> Result<(), Failure> {
    let object = sample_iges_geom_spline_curve()?;
    assert_eq!(object.spline_type(), 2);
    assert_eq!(object.degree(), 3);
    assert_eq!(object.nb_dimensions(), 4);
    assert_eq!((object.type_number(), object.form_number()), (112, 0));
    assert_entity_round(&object, "IgesGeomSplineCurve")?;
    Ok(())
}
#[test]
fn class_iges_geom_spline_surface() -> Result<(), Failure> {
    let object = sample_iges_geom_spline_surface()?;
    assert_eq!(object.boundary_type(), 2);
    assert_eq!(object.patch_type(), 3);
    assert_eq!((object.type_number(), object.form_number()), (114, 0));
    assert_entity_round(&object, "IgesGeomSplineSurface")?;
    Ok(())
}
#[test]
fn class_iges_geom_surface_of_revolution() -> Result<(), Failure> {
    let object = sample_iges_geom_surface_of_revolution()?;
    assert_eq!(object.start_angle(), 3.5);
    assert_eq!(object.end_angle(), 4.5);
    assert_eq!((object.type_number(), object.form_number()), (120, 0));
    assert_entity_round(&object, "IgesGeomSurfaceOfRevolution")?;
    Ok(())
}
#[test]
fn class_iges_geom_tabulated_cylinder() -> Result<(), Failure> {
    let object = sample_iges_geom_tabulated_cylinder()?;
    assert_eq!((object.type_number(), object.form_number()), (122, 0));
    assert_entity_round(&object, "IgesGeomTabulatedCylinder")?;
    Ok(())
}
#[test]
fn class_iges_geom_transformation_matrix() -> Result<(), Failure> {
    let object = sample_iges_geom_transformation_matrix()?;
    assert_entity_round(&object, "IgesGeomTransformationMatrix")?;
    Ok(())
}
#[test]
fn class_iges_geom_trimmed_surface() -> Result<(), Failure> {
    let object = sample_iges_geom_trimmed_surface()?;
    assert_eq!(object.outer_boundary_type(), 3);
    assert_eq!((object.type_number(), object.form_number()), (144, 0));
    assert_entity_round(&object, "IgesGeomTrimmedSurface")?;
    Ok(())
}
