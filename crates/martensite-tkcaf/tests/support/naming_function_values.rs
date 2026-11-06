//! Shared real DNaming argument/result access for native driver fixtures.
#![allow(dead_code)] // Each host fixture uses only the relevant argument/result accessors.
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingNamedShape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tklcaf::{
    tdata_std::{TDataStdInteger, TDataStdReal},
    tdf::{TdfAttributeState, TdfLabel},
    tfunction::TFunctionFunction,
};
use martensite_tkmath::toploc::Location;

/// DNaming::GetReal creates an absent argument with zero; `real_argument(&f, 1)?`.
pub fn real_argument(
    function: &Handle<TFunctionFunction>,
    position: i32,
) -> Result<Handle<TDataStdReal>, Failure> {
    let arguments = function.label().find_child(1, true)?;
    let label = arguments.find_child(position, true)?;
    let mut found = Handle::default();
    if label.find_attribute_as(&TDataStdReal::get_id(), &mut found)? {
        return Ok(found);
    }
    TDataStdReal::set(&label, 0.0)
}

/// DNaming::GetFunctionResult's label; `result_label(&f)?`.
pub fn result_label(function: &Handle<TFunctionFunction>) -> Result<TdfLabel, Failure> {
    function.label().find_child(2, true)
}

/// Read the actual named shape result; `function_result(&f)?.is_null()`.
pub fn function_result(
    function: &Handle<TFunctionFunction>,
) -> Result<Handle<TNamingNamedShape>, Failure> {
    let mut named = Handle::default();
    result_label(function)?.find_attribute_as(&TNamingNamedShape::get_id(), &mut named)?;
    Ok(named)
}

/// Read a function attribute if present; `function_on(&label)?`.
pub fn function_on(label: &TdfLabel) -> Result<Option<Handle<TFunctionFunction>>, Failure> {
    let mut function = Handle::default();
    if !label.find_attribute_as(&TFunctionFunction::get_id(), &mut function)? {
        return Ok(None);
    }
    Ok(Some(function))
}

/// Preserve a primitive result location on recomputation; `previous_location(&function)?`.
pub(super) fn previous_location(function: &Handle<TFunctionFunction>) -> Result<Location, Failure> {
    let previous = function_result(function)?;
    if previous.is_null() || previous.is_empty() {
        return Ok(Location::default());
    }
    Ok(previous
        .get()
        .ok_or_else(|| Failure::new("null previous box; expected named shape"))?
        .get()?
        .location()
        .clone())
}

/// Record a generated primitive component; `record_primitive_shape(&label, &shape)?`.
pub(super) fn record_primitive_shape(label: &TdfLabel, shape: &Shape) -> Result<(), Failure> {
    TNamingBuilder::new(label)?.generated(shape)
}

/// Keep the native primitive failure code and return value together; `primitive_failure(&function, 13)?`.
pub(super) fn primitive_failure(
    function: &Handle<TFunctionFunction>,
    code: i32,
) -> Result<i32, Failure> {
    function.set_failure(code)?;
    Ok(-1)
}

/// DNaming::GetInteger creates an absent argument with zero; `integer_argument(&f, 3)?`.
pub fn integer_argument(
    function: &Handle<TFunctionFunction>,
    position: i32,
) -> Result<Handle<TDataStdInteger>, Failure> {
    let label = function
        .label()
        .find_child(1, true)?
        .find_child(position, true)?;
    let mut integer = Handle::default();
    if label.find_attribute_as(&TDataStdInteger::get_id(), &mut integer)? {
        return Ok(integer);
    }
    TDataStdInteger::set(&label, 0)
}

/// Keep optional operation failure codes explicit; `optional_failure::<Shape>(&function, 14)?`.
pub(super) fn optional_failure<T>(
    function: &Handle<TFunctionFunction>,
    code: i32,
) -> Result<Option<T>, Failure> {
    function.set_failure(code)?;
    Ok(None)
}
