//! Real sphere naming driver used by the martensite-tkcaf document function registry.
use super::naming_transform_arguments::argument_value;
use martensite_tkbool::brep_algo;
use martensite_tkbrep::{brep::BRepTool, topexp, topods, toptools::IndexedMapOfShape};
use martensite_tkcaf::tnaming::{TNaming, TNamingNamedShape};
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    tdf::{TdfLabel, TdfTagSource},
    tfunction::{TFunctionDriver, TFunctionDriverBase, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::{
    brep_prim::BRepPrimOneAxis,
    brep_prim_api::{BRepPrimAPIMakeOneAxis, BRepPrimAPIMakeSphere},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

use super::naming_function_values as values;

/// Computes spheres and their stable face/meridian labels; `NamingSphereDriver::default()`.
#[derive(Default)]
pub struct NamingSphereDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
}
impl_transient!(NamingSphereDriver, "DNamingSphereDriver", dyn TFunctionDriver, transient,
    interfaces: [dyn TFunctionDriver]);

fn center_point(named: &Handle<TNamingNamedShape>) -> Result<Option<Point3>, Failure> {
    let shape = named.get().unwrap().get()?;
    if shape.is_null() {
        return Ok(None);
    }
    if shape.shape_type()? == ShapeEnum::Vertex {
        return Ok(Some(BRepTool::pnt(&topods::vertex(&shape)?)?));
    }
    // The driver deliberately uses the origin for a non-vertex named argument.
    Ok(Some(Point3::new(0.0, 0.0, 0.0)))
}

fn make_sphere(center: Point3, radius: f64) -> Result<BRepPrimAPIMakeSphere, Failure> {
    let axis = Axis2::with_x_direction(
        center,
        Direction3::new(0.0, 0.0, 1.0)?,
        Direction3::new(1.0, 0.0, 0.0)?,
    )?;
    let mut maker = BRepPrimAPIMakeSphere::with_axis(axis, radius)?;
    maker.build(&ProgressRange::default())?;
    Ok(maker)
}

fn generated_child(label: &TdfLabel, shape: &topods::Shape) -> Result<(), Failure> {
    values::record_primitive_shape(&TdfTagSource::new_child_of(label)?, shape)
}

fn load_meridian_names(label: &TdfLabel, face: &topods::Face) -> Result<(), Failure> {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(face, ShapeEnum::Edge, &mut edges)?;
    let mut good = Vec::new();
    for index in 1..=edges.extent() {
        let edge = topods::edge(edges.find_key(index)?)?;
        if !BRepTool::degenerated(&edge)? {
            good.push(edge);
        }
    }
    if good.len() != 1 {
        return Ok(());
    }
    generated_child(label, &good[0])?;
    load_meridian_vertices(label, &good[0])
}

fn load_meridian_vertices(label: &TdfLabel, edge: &topods::Edge) -> Result<(), Failure> {
    let mut vertices = topods::ShapeIterator::new(edge, true, true);
    while vertices.more() {
        generated_child(label, vertices.value()?)?;
        vertices.next();
    }
    Ok(())
}

/// Record every actual sphere primitive branch; `load_sphere_names(&label, &mut maker)?`.
pub(super) fn load_sphere_names(
    label: &TdfLabel,
    maker: &mut BRepPrimAPIMakeSphere,
) -> Result<(), Failure> {
    let tags = TdfTagSource::set(label)?;
    if tags.is_null() {
        return Ok(());
    }
    tags.set_value(0)?;
    values::record_primitive_shape(label, &maker.solid()?.into_shape())?;
    load_sphere_faces(label, maker)
}

fn load_sphere_faces(label: &TdfLabel, maker: &mut BRepPrimAPIMakeSphere) -> Result<(), Failure> {
    let sphere = maker.sphere();
    if sphere.has_bottom()? {
        generated_child(label, &sphere.bottom_face()?.into_shape())?;
    }
    if sphere.has_top()? {
        generated_child(label, &sphere.top_face()?.into_shape())?;
    }
    let lateral = sphere.lateral_face()?;
    generated_child(label, &lateral)?;
    if sphere.has_sides() {
        generated_child(label, &sphere.start_face()?.into_shape())?;
        generated_child(label, &sphere.end_face()?.into_shape())?;
    }
    load_meridian_names(label, &lateral)
}

fn checked_sphere(
    function: &Handle<TFunctionFunction>,
    center: Point3,
    radius: f64,
) -> Result<Option<BRepPrimAPIMakeSphere>, Failure> {
    let mut maker = make_sphere(center, radius)?;
    if !maker.is_done() {
        values::primitive_failure(function, 11)?;
        return Ok(None);
    }
    if !brep_algo::is_valid(&maker.solid()?.into_shape())? {
        values::primitive_failure(function, 12)?;
        return Ok(None);
    }
    Ok(Some(maker))
}

fn publish_sphere(
    function: &Handle<TFunctionFunction>,
    log: &mut Handle<TFunctionLogbook>,
    mut maker: BRepPrimAPIMakeSphere,
    location: &martensite_tkmath::toploc::Location,
) -> Result<i32, Failure> {
    let result = values::result_label(function)?;
    load_sphere_names(&result, &mut maker)?;
    if !location.is_identity() {
        TNaming::displace(&result, location, true)?;
    }
    log.set_valid(&result, true)?;
    function.set_failure(0)?;
    Ok(0)
}

fn execute_sphere(
    function: &Handle<TFunctionFunction>,
    log: &mut Handle<TFunctionLogbook>,
) -> Result<i32, Failure> {
    let radius = values::real_argument(function, 2)?.get().unwrap().get();
    let named = argument_value(function, 1)?;
    if named.is_null() || named.is_empty() {
        return values::primitive_failure(function, 14);
    }
    let location = values::previous_location(function)?;
    let Some(center) = center_point(&named)? else {
        return values::primitive_failure(function, 14);
    };
    let Some(maker) = checked_sphere(function, center, radius)? else {
        return Ok(-1);
    };
    publish_sphere(function, log, maker, &location)
}

impl TFunctionDriver for NamingSphereDriver {
    fn driver_base(&self) -> &TFunctionDriverBase {
        &self.base
    }
    fn validate(&self, _log: &Handle<TFunctionLogbook>) -> Result<(), Failure> {
        Ok(())
    }
    fn must_execute(&self, _log: &Handle<TFunctionLogbook>) -> Result<bool, Failure> {
        Ok(true)
    }
    fn execute(&self, log: &mut Handle<TFunctionLogbook>) -> Result<i32, Failure> {
        let Some(function) = values::function_on(&self.label())? else {
            return Ok(-1);
        };
        execute_sphere(&function, log)
    }
}
