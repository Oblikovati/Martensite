//! Actual DNamingTransformationDriver, including its source-bound naming history.
mod history;
use super::naming_box_driver::{function_result, real_argument, result_label};
use super::naming_function_values as values;
use super::naming_transform_arguments::{
    argument_value, axis_from_named, plane_from_named, previous_function,
};
use martensite_tkcaf::tnaming::TNamingNamedShape;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tklcaf::{
    guid_from_text,
    tfunction::{TFunctionDriver, TFunctionDriverBase, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::gp::{Transform3, Vector3};

/// Real transform driver fixture; `NamingTransformDriver::default()`.
#[derive(Default)]
pub struct NamingTransformDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
}
impl_transient!(NamingTransformDriver, "DNamingTransformationDriver", dyn TFunctionDriver, transient,
    interfaces: [dyn TFunctionDriver]);

fn real_value(function: &Handle<TFunctionFunction>, position: i32) -> Result<f64, Failure> {
    Ok(real_argument(function, position)?.get().unwrap().get())
}
fn axis_transform(
    function: &Handle<TFunctionFunction>,
    rotation: bool,
) -> Result<Transform3, Failure> {
    let value = argument_value(function, 2)?;
    let axis = axis_from_named(&value)?
        .ok_or_else(|| Failure::new("invalid axis selection; expected line EDGE or WIRE"))?;
    let mut transform = Transform3::default();
    if rotation {
        transform.set_rotation(axis, real_value(function, 1)?)?;
    } else {
        transform.set_translation(
            Vector3::from(axis.direction()).normalized()? * real_value(function, 1)?,
        );
    }
    Ok(transform)
}
fn requested_transform(
    function: &Handle<TFunctionFunction>,
) -> Result<Option<Transform3>, Failure> {
    let guid = function.get_driver_guid();
    if guid == guid_from_text("12e94557-6dbc-11d4-b9c8-0060b0ee281b") {
        return axis_transform(function, false).map(Some);
    }
    if guid == guid_from_text("12e94558-6dbc-11d4-b9c8-0060b0ee281b") {
        return axis_transform(function, true).map(Some);
    }
    if guid == guid_from_text("12e94556-6dbc-11d4-b9c8-0060b0ee281b") {
        return xyz_transform(function).map(Some);
    }
    if guid == guid_from_text("12e94559-6dbc-11d4-b9c8-0060b0ee281b") {
        return mirror_transform(function).map(Some);
    }

    Ok(None)
}
fn xyz_transform(function: &Handle<TFunctionFunction>) -> Result<Transform3, Failure> {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(
        real_value(function, 1)?,
        real_value(function, 2)?,
        real_value(function, 3)?,
    ));
    Ok(transform)
}
fn mirror_transform(function: &Handle<TFunctionFunction>) -> Result<Transform3, Failure> {
    let named = argument_value(function, 1)?;
    let plane = plane_from_named(&named)?
        .ok_or_else(|| Failure::new("invalid mirror selection; expected planar FACE"))?;
    let mut transform = Transform3::default();
    transform.set_mirror_plane(plane.position().ax2()?);
    Ok(transform)
}

fn source_context(
    function: &Handle<TFunctionFunction>,
) -> Result<Option<Handle<TNamingNamedShape>>, Failure> {
    let previous = previous_function(function)?;
    if previous.is_null() {
        return Ok(None);
    }
    Ok(Some(function_result(&previous)?))
}
fn transform_or_failure(
    function: &Handle<TFunctionFunction>,
) -> Result<Option<Transform3>, Failure> {
    match requested_transform(function) {
        Ok(Some(transform)) => Ok(Some(transform)),
        Ok(None) => {
            function.set_failure(15)?;
            Ok(None)
        }
        Err(_) => {
            function.set_failure(14)?;
            Ok(None)
        }
    }
}

fn apply_requested(
    function: &Handle<TFunctionFunction>,
    source: &Handle<TNamingNamedShape>,
    log: &Handle<TFunctionLogbook>,
) -> Result<i32, Failure> {
    let Some(transform) = transform_or_failure(function)? else {
        return Ok(-1);
    };
    let result = result_label(function)?;
    history::load_transformed_names(&result, source, transform)?;
    log.set_valid(&result, true)?;
    function.set_failure(0)?;
    Ok(0)
}
impl TFunctionDriver for NamingTransformDriver {
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
        let Some(source) = source_context(&function)? else {
            return Ok(-1);
        };
        if source.is_null() || source.is_empty() {
            function.set_failure(14)?;
            return Ok(-1);
        }
        apply_requested(&function, &source, log)
    }
}
mod fixtures;
pub use fixtures::{
    execute_transform, make_along_line, make_mirror, make_rotation, make_translation,
};
