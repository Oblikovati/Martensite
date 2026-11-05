//! Actual DNaming Boolean driver fixture, including validity repair and native history.
mod history;
mod repair;
use super::naming_function_values as values;
use super::{
    naming_box_driver::{function_result, result_label},
    naming_transform_arguments::{argument_value, previous_function},
};
use martensite_tkbo::brep_algo_api::{
    BRepAlgoAPIBooleanOperation, BRepAlgoAPICommon, BRepAlgoAPICut, BRepAlgoAPIFuse,
    BRepAlgoAPISection,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tklcaf::{
    guid_from_text,
    tdf::TdfAttributeState,
    tfunction::{TFunctionDriver, TFunctionDriverBase, TFunctionFunction, TFunctionLogbook},
};
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer,
};
use values::optional_failure as failed;

/// Native document Boolean driver; `NamingBooleanDriver::default()`.
#[derive(Default)]
pub struct NamingBooleanDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
}
impl_transient!(NamingBooleanDriver, "DNamingBooleanOperationDriver", dyn TFunctionDriver, transient, interfaces: [dyn TFunctionDriver]);

fn input_pair(function: &Handle<TFunctionFunction>) -> Result<Option<(Shape, Shape)>, Failure> {
    let previous = previous_function(function)?;
    if previous.is_null() {
        return Ok(None);
    }
    let source = function_result(&previous)?;
    if source.is_null() || source.is_empty() {
        return failed(function, 14);
    }
    let tool = argument_value(function, 1)?;
    if tool.is_null() || tool.is_empty() {
        return failed(function, 14);
    }
    let source = source.get().unwrap().get()?;
    let tool = tool.get().unwrap().get()?;
    if source.is_null() || tool.is_null() {
        return failed(function, 14);
    }
    Ok(Some((source, tool)))
}
enum BooleanMaker {
    Cut(BRepAlgoAPICut),
    Fuse(BRepAlgoAPIFuse),
    Common(BRepAlgoAPICommon),
    Section(BRepAlgoAPISection),
}
impl BooleanMaker {
    fn operation(&self) -> &BRepAlgoAPIBooleanOperation {
        match self {
            Self::Cut(m) => m,
            Self::Fuse(m) => m,
            Self::Common(m) => m,
            Self::Section(m) => m,
        }
    }
    fn operation_mut(&mut self) -> &mut BRepAlgoAPIBooleanOperation {
        match self {
            Self::Cut(m) => m,
            Self::Fuse(m) => m,
            Self::Common(m) => m,
            Self::Section(m) => m,
        }
    }
}
fn requested_maker(
    function: &Handle<TFunctionFunction>,
    source: &Shape,
    tool: &Shape,
) -> Result<Option<BooleanMaker>, Failure> {
    let guid = function.get_driver_guid();
    for (suffix, operation) in [("48", 0), ("49", 1), ("4a", 2), ("4b", 3)] {
        if guid != guid_from_text(&format!("12e945{suffix}-6dbc-11d4-b9c8-0060b0ee281b")) {
            continue;
        }
        return Ok(Some(match operation {
            0 => BooleanMaker::Cut(BRepAlgoAPICut::from_shapes(source, tool)?),
            1 => BooleanMaker::Fuse(BRepAlgoAPIFuse::from_shapes(source, tool)?),
            2 => BooleanMaker::Common(BRepAlgoAPICommon::from_shapes(source, tool)?),
            _ => BooleanMaker::Section(BRepAlgoAPISection::from_shapes(source, tool, None, true)?),
        }));
    }
    failed(function, 15)
}
fn checked_shape(
    function: &Handle<TFunctionFunction>,
    maker: &mut BooleanMaker,
) -> Result<Option<Shape>, Failure> {
    let shape = maker.operation().shape().clone();
    if !maker.operation().is_done() || shape.is_null() {
        return failed(function, 11);
    }
    if shape.shape_type()? == martensite_tkg3d::top_abs::ShapeEnum::Compound
        && shape.nb_children() == 0
    {
        return failed(function, 16);
    }
    let mut analyzer = BRepCheckAnalyzer::new(&shape, true, false, false)?;
    if !analyzer.is_valid_shape(&shape)? && !repair::fix_same_parameter(&shape, &mut analyzer)? {
        return failed(function, 12);
    }
    Ok(Some(shape))
}
fn execute_function(
    function: &Handle<TFunctionFunction>,
    log: &Handle<TFunctionLogbook>,
) -> Result<i32, Failure> {
    let Some((source, tool)) = input_pair(function)? else {
        return Ok(-1);
    };
    let Some(mut maker) = requested_maker(function, &source, &tool)? else {
        return Ok(-1);
    };
    if checked_shape(function, &mut maker)?.is_none() {
        return Ok(-1);
    }
    let result = result_label(function)?;
    let section = matches!(maker, BooleanMaker::Section(_));
    history::load_names(&result, maker.operation_mut(), section)?;
    log.set_valid(&result, true)?;
    function.set_failure(0)?;
    Ok(0)
}
impl TFunctionDriver for NamingBooleanDriver {
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
        execute_function(&function, log)
    }
}
/// Execute the real document Boolean operation; `execute_boolean(&function)?`.
pub fn execute_boolean(function: &Handle<TFunctionFunction>) -> Result<i32, Failure> {
    let driver = NamingBooleanDriver::default();
    driver.init(&function.label());
    let mut log = TFunctionLogbook::set(&function.label())?;
    log.clear()?;
    driver.execute(&mut log)
}
