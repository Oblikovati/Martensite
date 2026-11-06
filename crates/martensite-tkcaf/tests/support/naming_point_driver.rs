//! Real DNamingPointDriver from its package, including Cartesian and relative point modes.
use super::naming_function_values as values;
use super::naming_transform_arguments::argument_value;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkcaf::tnaming::TNaming;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tklcaf::{
    guid_from_text,
    tfunction::{TFunctionDriver, TFunctionDriverBase, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex};
/// Computes an actual named Cartesian or relative vertex; `NamingPointDriver::default()`.
#[derive(Default)]
pub struct NamingPointDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
}
impl_transient!(NamingPointDriver,"DNamingPointDriver",dyn TFunctionDriver,transient,interfaces:[dyn TFunctionDriver]);
fn point_offsets(function: &Handle<TFunctionFunction>) -> Result<[f64; 3], Failure> {
    Ok([
        values::real_argument(function, 1)?.get().unwrap().get(),
        values::real_argument(function, 2)?.get().unwrap().get(),
        values::real_argument(function, 3)?.get().unwrap().get(),
    ])
}
fn point_position(
    function: &Handle<TFunctionFunction>,
    offsets: [f64; 3],
) -> Result<Option<Point3>, Failure> {
    let [dx, dy, dz] = offsets;
    if function.get_driver_guid() != guid_from_text("12e9455d-6dbc-11d4-b9c8-0060b0ee281b") {
        return Ok(Some(Point3::new(dx, dy, dz)));
    }
    let reference = argument_value(function, 4)?;
    if reference.is_null() || reference.is_empty() {
        return values::optional_failure(function, 14);
    }
    let point = BRepTool::pnt(&topods::vertex(&reference.get().unwrap().get()?)?)?;
    Ok(Some(Point3::new(
        point.x() + dx,
        point.y() + dy,
        point.z() + dz,
    )))
}
fn execute_point(
    function: &Handle<TFunctionFunction>,
    log: &mut Handle<TFunctionLogbook>,
) -> Result<i32, Failure> {
    let offsets = point_offsets(function)?;
    let location = values::previous_location(function)?;
    let Some(point) = point_position(function, offsets)? else {
        return Ok(-1);
    };
    let mut maker = BRepBuilderAPIMakeVertex::new(point)?;
    if !maker.is_done() {
        return values::primitive_failure(function, 11);
    }
    let result = values::result_label(function)?;
    values::record_primitive_shape(&result, maker.shape()?)?;
    if !location.is_identity() {
        TNaming::displace(&result, &location, true)?;
    }
    log.set_valid(&result, true)?;
    function.set_failure(0)?;
    Ok(0)
}
impl TFunctionDriver for NamingPointDriver {
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
        execute_point(&function, log)
    }
}
