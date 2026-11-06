#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_solid::samples_01::{
    sample_iges_solid_block, sample_iges_solid_boolean_tree, sample_iges_solid_cone_frustum,
    sample_iges_solid_conical_surface, sample_iges_solid_cylinder,
    sample_iges_solid_cylindrical_surface, sample_iges_solid_edge_list,
    sample_iges_solid_ellipsoid, sample_iges_solid_face, sample_iges_solid_loop,
    sample_iges_solid_manifold_solid, sample_iges_solid_plane_surface,
    sample_iges_solid_right_angular_wedge, sample_iges_solid_selected_component,
    sample_iges_solid_shell, sample_iges_solid_solid_assembly, sample_iges_solid_solid_instance,
    sample_iges_solid_solid_of_linear_extrusion, sample_iges_solid_solid_of_revolution,
    sample_iges_solid_sphere, sample_iges_solid_spherical_surface,
    sample_iges_solid_toroidal_surface, sample_iges_solid_torus, sample_iges_solid_vertex_list,
};
use crate::iges_test_samples::assert_entity_round;
use martensite_tkernel::standard::Failure;

#[test]
fn class_iges_solid_block() -> Result<(), Failure> {
    let object = sample_iges_solid_block()?;
    assert_eq!((object.type_number(), object.form_number()), (150, 0));
    assert_entity_round(&object, "IgesSolidBlock")?;
    Ok(())
}
#[test]
fn class_iges_solid_boolean_tree() -> Result<(), Failure> {
    let object = sample_iges_solid_boolean_tree()?;
    assert_eq!((object.type_number(), object.form_number()), (180, 0));
    assert_entity_round(&object, "IgesSolidBooleanTree")?;
    Ok(())
}
#[test]
fn class_iges_solid_cone_frustum() -> Result<(), Failure> {
    let object = sample_iges_solid_cone_frustum()?;
    assert_eq!(object.height(), 1.5);
    assert_eq!(object.larger_radius(), 2.5);
    assert_eq!(object.smaller_radius(), 3.5);
    assert_eq!((object.type_number(), object.form_number()), (156, 0));
    assert_entity_round(&object, "IgesSolidConeFrustum")?;
    Ok(())
}
#[test]
fn class_iges_solid_conical_surface() -> Result<(), Failure> {
    let object = sample_iges_solid_conical_surface()?;
    assert_eq!(object.radius(), 3.5);
    assert_eq!(object.semi_angle(), 4.5);
    assert_entity_round(&object, "IgesSolidConicalSurface")?;
    Ok(())
}
#[test]
fn class_iges_solid_cylinder() -> Result<(), Failure> {
    let object = sample_iges_solid_cylinder()?;
    assert_eq!(object.height(), 1.5);
    assert_eq!(object.radius(), 2.5);
    assert_eq!((object.type_number(), object.form_number()), (154, 0));
    assert_entity_round(&object, "IgesSolidCylinder")?;
    Ok(())
}
#[test]
fn class_iges_solid_cylindrical_surface() -> Result<(), Failure> {
    let object = sample_iges_solid_cylindrical_surface()?;
    assert_eq!(object.radius(), 3.5);
    assert_entity_round(&object, "IgesSolidCylindricalSurface")?;
    Ok(())
}
#[test]
fn class_iges_solid_edge_list() -> Result<(), Failure> {
    let object = sample_iges_solid_edge_list()?;
    assert_eq!((object.type_number(), object.form_number()), (504, 1));
    assert_entity_round(&object, "IgesSolidEdgeList")?;
    Ok(())
}
#[test]
fn class_iges_solid_ellipsoid() -> Result<(), Failure> {
    let object = sample_iges_solid_ellipsoid()?;
    assert_eq!((object.type_number(), object.form_number()), (168, 0));
    assert_entity_round(&object, "IgesSolidEllipsoid")?;
    Ok(())
}
#[test]
fn class_iges_solid_face() -> Result<(), Failure> {
    let object = sample_iges_solid_face()?;
    assert_eq!((object.type_number(), object.form_number()), (510, 1));
    assert_entity_round(&object, "IgesSolidFace")?;
    Ok(())
}
#[test]
fn class_iges_solid_loop() -> Result<(), Failure> {
    let object = sample_iges_solid_loop()?;
    assert_eq!((object.type_number(), object.form_number()), (508, 1));
    assert_entity_round(&object, "IgesSolidLoop")?;
    Ok(())
}
#[test]
fn class_iges_solid_manifold_solid() -> Result<(), Failure> {
    let object = sample_iges_solid_manifold_solid()?;
    assert!(object.orientation_flag());
    assert_eq!((object.type_number(), object.form_number()), (186, 0));
    assert_entity_round(&object, "IgesSolidManifoldSolid")?;
    Ok(())
}
#[test]
fn class_iges_solid_plane_surface() -> Result<(), Failure> {
    let object = sample_iges_solid_plane_surface()?;
    assert_entity_round(&object, "IgesSolidPlaneSurface")?;
    Ok(())
}
#[test]
fn class_iges_solid_right_angular_wedge() -> Result<(), Failure> {
    let object = sample_iges_solid_right_angular_wedge()?;
    assert_eq!(object.x_small_length(), 2.5);
    assert_eq!((object.type_number(), object.form_number()), (152, 0));
    assert_entity_round(&object, "IgesSolidRightAngularWedge")?;
    Ok(())
}
#[test]
fn class_iges_solid_selected_component() -> Result<(), Failure> {
    let object = sample_iges_solid_selected_component()?;
    assert_eq!((object.type_number(), object.form_number()), (182, 0));
    assert_entity_round(&object, "IgesSolidSelectedComponent")?;
    Ok(())
}
#[test]
fn class_iges_solid_shell() -> Result<(), Failure> {
    let object = sample_iges_solid_shell()?;
    assert_eq!((object.type_number(), object.form_number()), (514, 1));
    assert_entity_round(&object, "IgesSolidShell")?;
    Ok(())
}
#[test]
fn class_iges_solid_solid_assembly() -> Result<(), Failure> {
    let object = sample_iges_solid_solid_assembly()?;
    assert_eq!((object.type_number(), object.form_number()), (184, 0));
    assert_entity_round(&object, "IgesSolidSolidAssembly")?;
    Ok(())
}
#[test]
fn class_iges_solid_solid_instance() -> Result<(), Failure> {
    let object = sample_iges_solid_solid_instance()?;
    assert_eq!((object.type_number(), object.form_number()), (430, 0));
    assert_entity_round(&object, "IgesSolidSolidInstance")?;
    Ok(())
}
#[test]
fn class_iges_solid_solid_of_linear_extrusion() -> Result<(), Failure> {
    let object = sample_iges_solid_solid_of_linear_extrusion()?;
    assert_eq!(object.extrusion_length(), 2.5);
    assert_eq!((object.type_number(), object.form_number()), (164, 0));
    assert_entity_round(&object, "IgesSolidSolidOfLinearExtrusion")?;
    Ok(())
}
#[test]
fn class_iges_solid_solid_of_revolution() -> Result<(), Failure> {
    let object = sample_iges_solid_solid_of_revolution()?;
    assert_eq!(object.fraction(), 2.5);
    assert_entity_round(&object, "IgesSolidSolidOfRevolution")?;
    Ok(())
}
#[test]
fn class_iges_solid_sphere() -> Result<(), Failure> {
    let object = sample_iges_solid_sphere()?;
    assert_eq!(object.radius(), 1.5);
    assert_eq!((object.type_number(), object.form_number()), (158, 0));
    assert_entity_round(&object, "IgesSolidSphere")?;
    Ok(())
}
#[test]
fn class_iges_solid_spherical_surface() -> Result<(), Failure> {
    let object = sample_iges_solid_spherical_surface()?;
    assert_eq!(object.radius(), 2.5);
    assert_entity_round(&object, "IgesSolidSphericalSurface")?;
    Ok(())
}
#[test]
fn class_iges_solid_toroidal_surface() -> Result<(), Failure> {
    let object = sample_iges_solid_toroidal_surface()?;
    assert_eq!(object.major_radius(), 3.5);
    assert_eq!(object.minor_radius(), 4.5);
    assert_entity_round(&object, "IgesSolidToroidalSurface")?;
    Ok(())
}
#[test]
fn class_iges_solid_torus() -> Result<(), Failure> {
    let object = sample_iges_solid_torus()?;
    assert_eq!(object.major_radius(), 1.5);
    assert_eq!(object.disc_radius(), 2.5);
    assert_eq!((object.type_number(), object.form_number()), (160, 0));
    assert_entity_round(&object, "IgesSolidTorus")?;
    Ok(())
}
#[test]
fn class_iges_solid_vertex_list() -> Result<(), Failure> {
    let object = sample_iges_solid_vertex_list()?;
    assert_eq!((object.type_number(), object.form_number()), (502, 1));
    assert_entity_round(&object, "IgesSolidVertexList")?;
    Ok(())
}
