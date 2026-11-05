//! Exact four transformation construction helpers used by TNamingTool tests E2.
use super::super::{
    naming_box_driver::real_argument, naming_function_graph::add_function,
    naming_transform_arguments::set_object_argument,
};
use super::NamingTransformDriver;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::TDataStdUAttribute,
    tdf::TdfAttributeState,
    tfunction::{TFunctionDriver, TFunctionFunction, TFunctionLogbook},
};

/// Execute an actual transform driver with the native fresh logbook; `execute_transform(&f)?`.
pub fn execute_transform(function: &Handle<TFunctionFunction>) -> Result<i32, Failure> {
    let driver = NamingTransformDriver::default();
    driver.init(&function.label());
    let mut log = TFunctionLogbook::set(&function.label())?;
    log.clear()?;
    driver.execute(&mut log)
}

/// Native MakeNamingTranslation, with unchanged XYZ arguments; `make_translation(&object, [150.,40.,90.])?`.
pub fn make_translation(
    object: &Handle<TDataStdUAttribute>,
    vector: [f64; 3],
) -> Result<Handle<TFunctionFunction>, Failure> {
    let function = add_function(
        object,
        &guid_from_text("12e94556-6dbc-11d4-b9c8-0060b0ee281b"),
    )?;
    for (index, value) in vector.into_iter().enumerate() {
        real_argument(&function, index as i32 + 1)?.set_value(value)?;
    }
    assert_eq!(execute_transform(&function)?, 0);
    Ok(function)
}
fn make_axis_transform(
    object: &Handle<TDataStdUAttribute>,
    line: &Handle<TDataStdUAttribute>,
    value: f64,
    guid: &str,
) -> Result<Handle<TFunctionFunction>, Failure> {
    let function = add_function(object, &guid_from_text(guid))?;
    set_object_argument(&function, 2, line)?;
    real_argument(&function, 1)?.set_value(value)?;
    assert_eq!(execute_transform(&function)?, 0);
    Ok(function)
}

/// Native MakeNamingAlongLine; `make_along_line(&object, &line, 210.)?`.
pub fn make_along_line(
    object: &Handle<TDataStdUAttribute>,
    line: &Handle<TDataStdUAttribute>,
    offset: f64,
) -> Result<Handle<TFunctionFunction>, Failure> {
    make_axis_transform(object, line, offset, "12e94557-6dbc-11d4-b9c8-0060b0ee281b")
}

/// Native MakeNamingRotation stores the supplied value unchanged; `make_rotation(&object, &line, 120.)?`.
pub fn make_rotation(
    object: &Handle<TDataStdUAttribute>,
    line: &Handle<TDataStdUAttribute>,
    angle: f64,
) -> Result<Handle<TFunctionFunction>, Failure> {
    make_axis_transform(object, line, angle, "12e94558-6dbc-11d4-b9c8-0060b0ee281b")
}

/// Native MakeNamingMirror; `make_mirror(&object, &plane)?`.
pub fn make_mirror(
    object: &Handle<TDataStdUAttribute>,
    plane: &Handle<TDataStdUAttribute>,
) -> Result<Handle<TFunctionFunction>, Failure> {
    let function = add_function(
        object,
        &guid_from_text("12e94559-6dbc-11d4-b9c8-0060b0ee281b"),
    )?;
    set_object_argument(&function, 1, plane)?;
    assert_eq!(execute_transform(&function)?, 0);
    Ok(function)
}
