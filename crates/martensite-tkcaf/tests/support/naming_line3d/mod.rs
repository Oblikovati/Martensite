//! Real DNamingLine3DDriver: native per-segment checks, wire construction and naming failures.
mod history;
use super::naming_function_values as values;
use super::naming_transform_arguments::argument_value;
use martensite_tkbool::brep_algo;
use martensite_tkbrep::topods::{self, Shape, Wire};
use martensite_tkcaf::tnaming::TNaming;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tfunction::{
    TFunctionDriver, TFunctionDriverBase, TFunctionFunction, TFunctionLogbook,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
/// Builds named open/closed polylines from actual point references; `NamingLine3dDriver::default()`.
#[derive(Default)]
pub struct NamingLine3dDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
}
impl_transient!(NamingLine3dDriver,"DNamingLine3DDriver",dyn TFunctionDriver,transient,interfaces:[dyn TFunctionDriver]);
fn referenced_shape(function: &Handle<TFunctionFunction>, position: i32) -> Result<Shape, Failure> {
    let named = argument_value(function, position)?;
    if named.is_null() {
        return Err(Failure::new(&format!(
            "Line3D argument {position} has null named attribute; expected existing named point (native null dereference boundary)"
        )));
    }
    named.get().unwrap().get()
}
fn point_pair(
    function: &Handle<TFunctionFunction>,
    index: usize,
) -> Result<Option<(Shape, Shape)>, Failure> {
    let first = referenced_shape(function, index as i32 + 2)?;
    let second = referenced_shape(function, index as i32 + 3)?;
    if first.is_null() || second.is_null() {
        return values::optional_failure(function, 14);
    }
    if first.shape_type()? != ShapeEnum::Vertex || second.shape_type()? != ShapeEnum::Vertex {
        return values::optional_failure(function, 14);
    }
    Ok(Some((first, second)))
}
fn append_segment(
    wire: &mut BRepBuilderAPIMakeWire,
    first: &Shape,
    second: &Shape,
) -> Result<bool, Failure> {
    let edge =
        BRepBuilderAPIMakeEdge::from_vertices(&topods::vertex(first)?, &topods::vertex(second)?)?;
    if !edge.is_done() {
        return Ok(false);
    }
    wire.add_edge(&edge.edge()?)?;
    Ok(true)
}
fn constructed_wire(
    function: &Handle<TFunctionFunction>,
    length: usize,
    closed: bool,
) -> Result<Option<(Wire, Vec<Shape>)>, Failure> {
    let mut maker = BRepBuilderAPIMakeWire::new();
    let mut vertices = vec![Shape::default(); length];
    let mut last = Shape::default();
    for index in 0..length - 1 {
        let Some((first, second)) = point_pair(function, index)? else {
            return Ok(None);
        };
        if append_segment(&mut maker, &first, &second)? {
            vertices[index] = first;
            vertices[index + 1] = second.clone();
        }
        last = second;
    }
    if closed {
        append_segment(&mut maker, &last, &referenced_shape(function, 2)?)?;
    }
    checked_wire(function, maker, vertices)
}
fn checked_wire(
    function: &Handle<TFunctionFunction>,
    maker: BRepBuilderAPIMakeWire,
    vertices: Vec<Shape>,
) -> Result<Option<(Wire, Vec<Shape>)>, Failure> {
    if !maker.is_done() {
        return values::optional_failure(function, 11);
    }
    let wire = maker.wire()?;
    if wire.is_null() {
        return values::optional_failure(function, 11);
    }
    if !brep_algo::is_valid(&wire)? {
        return values::optional_failure(function, 12);
    }
    Ok(Some((wire, vertices)))
}
fn execute_line(
    function: &Handle<TFunctionFunction>,
    log: &mut Handle<TFunctionLogbook>,
) -> Result<i32, Failure> {
    function.set_failure(9999)?;
    let location = values::previous_location(function)?;
    let closed = values::integer_argument(function, 1)?.get().unwrap().get() != 0;
    let length = values::integer_argument(function, 2)?.get().unwrap().get();
    if length < 2 {
        return values::primitive_failure(function, 14);
    }
    let Some((wire, vertices)) = constructed_wire(function, length as usize, closed)? else {
        return Ok(-1);
    };
    let result = values::result_label(function)?;
    if history::load_line_names(&result, &wire, &vertices, closed).is_err() {
        return values::primitive_failure(function, 15);
    }
    if !location.is_identity() {
        TNaming::displace(&result, &location, true)?;
    }
    log.set_valid(&result, true)?;
    function.set_failure(0)?;
    Ok(0)
}
impl TFunctionDriver for NamingLine3dDriver {
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
        execute_line(&function, log)
    }
}
