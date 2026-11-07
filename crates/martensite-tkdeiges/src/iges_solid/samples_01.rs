#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_basic::entity_shape::boolean_tree_operands;
use crate::iges_basic::nested_arrays::nested_entity_lists;
use crate::iges_basic::{
    IgesBasicHArray1OfHArray1OfIgesEntity, IgesBasicHArray1OfHArray1OfInteger,
};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_data::{iges_array1_set, iges_new_array1};
use crate::iges_geom::samples_01::{
    sample_iges_geom_direction, sample_iges_geom_line, sample_iges_geom_point,
    sample_iges_geom_transformation_matrix,
};
use crate::iges_geom::{IgesGeomDirection, IgesGeomPoint, IgesGeomTransformationMatrix};
use crate::iges_solid::{
    IgesSolidBlock, IgesSolidBooleanTree, IgesSolidConeFrustum, IgesSolidConicalSurface,
    IgesSolidCylinder, IgesSolidCylindricalSurface, IgesSolidEdgeList, IgesSolidEllipsoid,
    IgesSolidFace, IgesSolidLoop, IgesSolidManifoldSolid, IgesSolidPlaneSurface,
    IgesSolidRightAngularWedge, IgesSolidSelectedComponent, IgesSolidShell, IgesSolidSolidAssembly,
    IgesSolidSolidInstance, IgesSolidSolidOfLinearExtrusion, IgesSolidSolidOfRevolution,
    IgesSolidSphere, IgesSolidSphericalSurface, IgesSolidToroidalSurface, IgesSolidTorus,
    IgesSolidVertexList,
};
use crate::iges_test_samples::{h_array1_of_h_array1_of_integer_sample, sample_list};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xyz;

pub(crate) fn sample_iges_solid_block() -> Result<Handle<IgesSolidBlock>, Failure> {
    let object = Handle::new(IgesSolidBlock::new());
    let a_size: Xyz = Xyz::new(1.5, 2.5, 3.5);
    let a_corner: Xyz = Xyz::new(2.5, 3.5, 4.5);
    let a_x_axis: Xyz = Xyz::new(3.5, 4.5, 5.5);
    let a_z_axis: Xyz = Xyz::new(4.5, 5.5, 6.5);
    object.init(a_size, a_corner, a_x_axis, a_z_axis);
    Ok(object)
}
pub(crate) fn sample_iges_solid_boolean_tree() -> Result<Handle<IgesSolidBooleanTree>, Failure> {
    let object = Handle::new(IgesSolidBooleanTree::new());
    let operands: Handle<HArray1<Handle<IgesDataIgesEntity>>> = boolean_tree_operands()?;
    let operations: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, 3).and_then(|list| {
        iges_array1_set(&list, 3, 1, "sample")?;
        Ok(list)
    })?;
    object.init(&operands, &operations)?;
    Ok(object)
}
pub(crate) fn sample_iges_solid_cone_frustum() -> Result<Handle<IgesSolidConeFrustum>, Failure> {
    let object = Handle::new(IgesSolidConeFrustum::new());
    let ht: f64 = 1.5;
    let r1: f64 = 2.5;
    let r2: f64 = 3.5;
    let center: Xyz = Xyz::new(4.5, 5.5, 6.5);
    let an_axis: Xyz = Xyz::new(5.5, 6.5, 7.5);
    object.init(ht, r1, r2, center, an_axis);
    Ok(object)
}
pub(crate) fn sample_iges_solid_conical_surface() -> Result<Handle<IgesSolidConicalSurface>, Failure>
{
    let object = Handle::new(IgesSolidConicalSurface::new());
    let a_location: Handle<IgesGeomPoint> = sample_iges_geom_point()?;
    let an_axis: Handle<IgesGeomDirection> = sample_iges_geom_direction()?;
    let a_radius: f64 = 3.5;
    let an_angle: f64 = 4.5;
    let a_refdir: Handle<IgesGeomDirection> = sample_iges_geom_direction()?;
    object.init(&a_location, &an_axis, a_radius, an_angle, &a_refdir);
    Ok(object)
}
pub(crate) fn sample_iges_solid_cylinder() -> Result<Handle<IgesSolidCylinder>, Failure> {
    let object = Handle::new(IgesSolidCylinder::new());
    let a_height: f64 = 1.5;
    let a_radius: f64 = 2.5;
    let a_center: Xyz = Xyz::new(3.5, 4.5, 5.5);
    let an_axis: Xyz = Xyz::new(4.5, 5.5, 6.5);
    object.init(a_height, a_radius, a_center, an_axis);
    Ok(object)
}
pub(crate) fn sample_iges_solid_cylindrical_surface()
-> Result<Handle<IgesSolidCylindricalSurface>, Failure> {
    let object = Handle::new(IgesSolidCylindricalSurface::new());
    let a_location: Handle<IgesGeomPoint> = sample_iges_geom_point()?;
    let an_axis: Handle<IgesGeomDirection> = sample_iges_geom_direction()?;
    let a_radius: f64 = 3.5;
    let a_refdir: Handle<IgesGeomDirection> = sample_iges_geom_direction()?;
    object.init(&a_location, &an_axis, a_radius, &a_refdir);
    Ok(object)
}
pub(crate) fn sample_iges_solid_edge_list() -> Result<Handle<IgesSolidEdgeList>, Failure> {
    let object = Handle::new(IgesSolidEdgeList::new());
    let curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> = sample_list([
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
    ])?;
    let start_vertex_list: Handle<HArray1<Handle<IgesSolidVertexList>>> = sample_list([
        sample_iges_solid_vertex_list()?,
        sample_iges_solid_vertex_list()?,
    ])?;
    let start_vertex_index: Handle<HArray1<i32>> = sample_list([3, 4])?;
    let end_vertex_list: Handle<HArray1<Handle<IgesSolidVertexList>>> = sample_list([
        sample_iges_solid_vertex_list()?,
        sample_iges_solid_vertex_list()?,
    ])?;
    let end_vertex_index: Handle<HArray1<i32>> = sample_list([3, 4])?;
    object.init(
        &curves,
        &start_vertex_list,
        &start_vertex_index,
        &end_vertex_list,
        &end_vertex_index,
    )?;
    Ok(object)
}
pub(crate) fn sample_iges_solid_ellipsoid() -> Result<Handle<IgesSolidEllipsoid>, Failure> {
    let object = Handle::new(IgesSolidEllipsoid::new());
    let a_size: Xyz = Xyz::new(1.5, 2.5, 3.5);
    let a_center: Xyz = Xyz::new(2.5, 3.5, 4.5);
    let an_x_axis: Xyz = Xyz::new(3.5, 4.5, 5.5);
    let an_z_axis: Xyz = Xyz::new(4.5, 5.5, 6.5);
    object.init(a_size, a_center, an_x_axis, an_z_axis);
    Ok(object)
}
pub(crate) fn sample_iges_solid_face() -> Result<Handle<IgesSolidFace>, Failure> {
    let object = Handle::new(IgesSolidFace::new());
    let a_surface: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let outer_loop_flag: bool = true;
    let loops: Handle<HArray1<Handle<IgesSolidLoop>>> =
        sample_list([sample_iges_solid_loop()?, sample_iges_solid_loop()?])?;
    object.init(&a_surface, outer_loop_flag, &loops)?;
    Ok(object)
}
pub(crate) fn sample_iges_solid_loop() -> Result<Handle<IgesSolidLoop>, Failure> {
    let object = Handle::new(IgesSolidLoop::new());
    let types: Handle<HArray1<i32>> = sample_list([1, 1])?;
    let edges: Handle<HArray1<Handle<IgesDataIgesEntity>>> = sample_list([
        sample_iges_solid_vertex_list()?.cast::<IgesDataIgesEntity>(),
        sample_iges_solid_vertex_list()?.cast::<IgesDataIgesEntity>(),
    ])?;
    let index: Handle<HArray1<i32>> = sample_list([1, 2])?;
    let orient: Handle<HArray1<i32>> = sample_list([1, 0])?;
    let nb_parameter_curves: Handle<HArray1<i32>> = sample_list([2, 2])?;
    let isoparametric_flags: Handle<IgesBasicHArray1OfHArray1OfInteger> =
        h_array1_of_h_array1_of_integer_sample()?;
    let curves: Handle<IgesBasicHArray1OfHArray1OfIgesEntity> = nested_entity_lists()?;
    object.init(
        &types,
        &edges,
        &index,
        &orient,
        &nb_parameter_curves,
        &isoparametric_flags,
        &curves,
    )?;
    Ok(object)
}
pub(crate) fn sample_iges_solid_manifold_solid() -> Result<Handle<IgesSolidManifoldSolid>, Failure>
{
    let object = Handle::new(IgesSolidManifoldSolid::new());
    let a_shell: Handle<IgesSolidShell> = sample_iges_solid_shell()?;
    let shellflag: bool = true;
    let void_shells: Handle<HArray1<Handle<IgesSolidShell>>> =
        sample_list([sample_iges_solid_shell()?, sample_iges_solid_shell()?])?;
    let void_shell_flags: Handle<HArray1<i32>> = sample_list([3, 4])?;
    object.init(&a_shell, shellflag, &void_shells, &void_shell_flags)?;
    Ok(object)
}
pub(crate) fn sample_iges_solid_plane_surface() -> Result<Handle<IgesSolidPlaneSurface>, Failure> {
    let object = Handle::new(IgesSolidPlaneSurface::new());
    let a_location: Handle<IgesGeomPoint> = sample_iges_geom_point()?;
    let a_normal: Handle<IgesGeomDirection> = sample_iges_geom_direction()?;
    let refdir: Handle<IgesGeomDirection> = sample_iges_geom_direction()?;
    object.init(&a_location, &a_normal, &refdir);
    Ok(object)
}
pub(crate) fn sample_iges_solid_right_angular_wedge()
-> Result<Handle<IgesSolidRightAngularWedge>, Failure> {
    let object = Handle::new(IgesSolidRightAngularWedge::new());
    let a_size: Xyz = Xyz::new(1.5, 2.5, 3.5);
    let low_x: f64 = 2.5;
    let a_corner: Xyz = Xyz::new(3.5, 4.5, 5.5);
    let an_x_axis: Xyz = Xyz::new(4.5, 5.5, 6.5);
    let an_z_axis: Xyz = Xyz::new(5.5, 6.5, 7.5);
    object.init(a_size, low_x, a_corner, an_x_axis, an_z_axis);
    Ok(object)
}
pub(crate) fn sample_iges_solid_selected_component()
-> Result<Handle<IgesSolidSelectedComponent>, Failure> {
    let object = Handle::new(IgesSolidSelectedComponent::new());
    let an_entity: Handle<IgesSolidBooleanTree> = sample_iges_solid_boolean_tree()?;
    let select_pnt: Xyz = Xyz::new(2.5, 3.5, 4.5);
    object.init(&an_entity, select_pnt);
    Ok(object)
}
pub(crate) fn sample_iges_solid_shell() -> Result<Handle<IgesSolidShell>, Failure> {
    let object = Handle::new(IgesSolidShell::new());
    let faces: Handle<HArray1<Handle<IgesSolidFace>>> =
        sample_list([sample_iges_solid_face()?, sample_iges_solid_face()?])?;
    let orient: Handle<HArray1<i32>> = sample_list([3, 4])?;
    object.init(&faces, &orient)?;
    Ok(object)
}
pub(crate) fn sample_iges_solid_solid_assembly() -> Result<Handle<IgesSolidSolidAssembly>, Failure>
{
    let object = Handle::new(IgesSolidSolidAssembly::new());
    let items: Handle<HArray1<Handle<IgesDataIgesEntity>>> = sample_list([
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
    ])?;
    let matrices: Handle<HArray1<Handle<IgesGeomTransformationMatrix>>> = sample_list([
        sample_iges_geom_transformation_matrix()?,
        sample_iges_geom_transformation_matrix()?,
    ])?;
    object.init(&items, &matrices)?;
    Ok(object)
}
pub(crate) fn sample_iges_solid_solid_instance() -> Result<Handle<IgesSolidSolidInstance>, Failure>
{
    let object = Handle::new(IgesSolidSolidInstance::new());
    let an_entity: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    object.init(&an_entity);
    Ok(object)
}
pub(crate) fn sample_iges_solid_solid_of_linear_extrusion()
-> Result<Handle<IgesSolidSolidOfLinearExtrusion>, Failure> {
    let object = Handle::new(IgesSolidSolidOfLinearExtrusion::new());
    let a_curve: Handle<IgesDataIgesEntity> = sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let length: f64 = 2.5;
    let direction: Xyz = Xyz::new(3.5, 4.5, 5.5);
    object.init(&a_curve, length, direction);
    Ok(object)
}
pub(crate) fn sample_iges_solid_solid_of_revolution()
-> Result<Handle<IgesSolidSolidOfRevolution>, Failure> {
    let object = Handle::new(IgesSolidSolidOfRevolution::new());
    let a_curve: Handle<IgesDataIgesEntity> = sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let fract: f64 = 2.5;
    let axis_pnt: Xyz = Xyz::new(3.5, 4.5, 5.5);
    let direction: Xyz = Xyz::new(4.5, 5.5, 6.5);
    object.init(&a_curve, fract, axis_pnt, direction);
    Ok(object)
}
pub(crate) fn sample_iges_solid_sphere() -> Result<Handle<IgesSolidSphere>, Failure> {
    let object = Handle::new(IgesSolidSphere::new());
    let a_radius: f64 = 1.5;
    let a_center: Xyz = Xyz::new(2.5, 3.5, 4.5);
    object.init(a_radius, a_center);
    Ok(object)
}
pub(crate) fn sample_iges_solid_spherical_surface()
-> Result<Handle<IgesSolidSphericalSurface>, Failure> {
    let object = Handle::new(IgesSolidSphericalSurface::new());
    let a_center: Handle<IgesGeomPoint> = sample_iges_geom_point()?;
    let a_radius: f64 = 2.5;
    let an_axis: Handle<IgesGeomDirection> = sample_iges_geom_direction()?;
    let a_refdir: Handle<IgesGeomDirection> = sample_iges_geom_direction()?;
    object.init(&a_center, a_radius, &an_axis, &a_refdir);
    Ok(object)
}
pub(crate) fn sample_iges_solid_toroidal_surface()
-> Result<Handle<IgesSolidToroidalSurface>, Failure> {
    let object = Handle::new(IgesSolidToroidalSurface::new());
    let a_center: Handle<IgesGeomPoint> = sample_iges_geom_point()?;
    let an_axis: Handle<IgesGeomDirection> = sample_iges_geom_direction()?;
    let maj_radius: f64 = 3.5;
    let min_radius: f64 = 4.5;
    let refdir: Handle<IgesGeomDirection> = sample_iges_geom_direction()?;
    object.init(&a_center, &an_axis, maj_radius, min_radius, &refdir);
    Ok(object)
}
pub(crate) fn sample_iges_solid_torus() -> Result<Handle<IgesSolidTorus>, Failure> {
    let object = Handle::new(IgesSolidTorus::new());
    let r1: f64 = 1.5;
    let r2: f64 = 2.5;
    let point: Xyz = Xyz::new(3.5, 4.5, 5.5);
    let axisdir: Xyz = Xyz::new(4.5, 5.5, 6.5);
    object.init(r1, r2, point, axisdir);
    Ok(object)
}
pub(crate) fn sample_iges_solid_vertex_list() -> Result<Handle<IgesSolidVertexList>, Failure> {
    let object = Handle::new(IgesSolidVertexList::new());
    let vertices: Handle<HArray1<Xyz>> =
        sample_list([Xyz::new(1.5, 2.5, 0.5), Xyz::new(3.5, 4.5, 0.5)])?;
    object.init(&vertices)?;
    Ok(object)
}
