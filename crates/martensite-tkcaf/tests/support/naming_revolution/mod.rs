//! Actual DNamingRevolutionDriver, with shared native basis/validation and complete history paths.
mod history;
mod topology;
use super::naming_function_values as values;
use super::{
    naming_box_driver::{real_argument, result_label},
    naming_sweep_basis::{basis_context, planar_basis, valid_result},
    naming_transform_arguments::{argument_value, axis_from_named},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::TNaming;
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    precision,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tklcaf::{
    guid_from_text,
    tdf::TdfAttributeState,
    tfunction::{TFunctionDriver, TFunctionDriverBase, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::gp::Axis1;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeRevol;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use values::optional_failure as failed;
/// Native full/sectioned revolution driver; `NamingRevolutionDriver::default()`.
#[derive(Default)]
pub struct NamingRevolutionDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
}
impl_transient!(NamingRevolutionDriver, "DNamingRevolutionDriver", dyn TFunctionDriver, transient, interfaces: [dyn TFunctionDriver]);
struct RevolutionInputs {
    basis: Shape,
    context: Shape,
    axis: Axis1,
}
impl RevolutionInputs {
    fn new(basis: Shape, context: Shape, axis: Axis1) -> Self {
        Self {
            basis,
            context,
            axis,
        }
    }
}
fn inputs(function: &Handle<TFunctionFunction>) -> Result<Option<RevolutionInputs>, Failure> {
    let named = argument_value(function, 1)?;
    if named.is_null() || named.is_empty() {
        return failed(function, 14);
    }
    let Some(basis) = planar_basis(&named.get().unwrap().get()?)? else {
        return failed(function, 14);
    };
    let Some(context) = basis_context(function, &basis)? else {
        return Ok(None);
    };
    let Some(axis) = axis_from_named(&argument_value(function, 2)?)? else {
        return failed(function, 14);
    };
    Ok(Some(RevolutionInputs::new(basis, context, axis)))
}
fn section_maker(
    function: &Handle<TFunctionFunction>,
    inputs: &RevolutionInputs,
) -> Result<Option<BRepPrimAPIMakeRevol>, Failure> {
    let angle = real_argument(function, 3)?.get().unwrap().get();
    if angle <= precision::confusion() {
        return failed(function, 14);
    }
    let mut axis = inputs.axis;
    if values::integer_argument(function, 4)?.get().unwrap().get() != 0 {
        axis.reverse();
    }
    BRepPrimAPIMakeRevol::new(&inputs.basis, axis, angle, true).map(Some)
}
fn requested_maker(
    function: &Handle<TFunctionFunction>,
    inputs: &RevolutionInputs,
) -> Result<Option<BRepPrimAPIMakeRevol>, Failure> {
    let guid = function.get_driver_guid();
    if guid == guid_from_text("12e94551-6dbc-11d4-b9c8-0060b0ee281b") {
        return BRepPrimAPIMakeRevol::full(&inputs.basis, inputs.axis, true).map(Some);
    }
    if guid == guid_from_text("12e94552-6dbc-11d4-b9c8-0060b0ee281b") {
        return section_maker(function, inputs);
    }
    failed(function, 15)
}
fn checked_maker(
    function: &Handle<TFunctionFunction>,
    inputs: &RevolutionInputs,
) -> Result<Option<BRepPrimAPIMakeRevol>, Failure> {
    let Some(mut maker) = requested_maker(function, inputs)? else {
        return Ok(None);
    };
    maker.build(&ProgressRange::default())?;
    if !maker.is_done() {
        return failed(function, 11);
    }
    if !valid_result(maker.shape()?)? {
        return failed(function, 12);
    }
    Ok(Some(maker))
}
fn execute_function(
    function: &Handle<TFunctionFunction>,
    log: &Handle<TFunctionLogbook>,
) -> Result<i32, Failure> {
    let location = values::previous_location(function)?;
    let Some(inputs) = inputs(function)? else {
        return Ok(-1);
    };
    let Some(mut maker) = checked_maker(function, &inputs)? else {
        return Ok(-1);
    };
    let result = result_label(function)?;
    history::load_names(&result, &mut maker, &inputs.basis, &inputs.context)?;
    if !location.is_identity() {
        TNaming::displace(&result, &location, true)?;
    }
    log.set_valid(&result, true)?;
    function.set_failure(0)?;
    Ok(0)
}
impl TFunctionDriver for NamingRevolutionDriver {
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
/// Execute the actual revolution driver; `execute_revolution(&function)?`.
pub fn execute_revolution(function: &Handle<TFunctionFunction>) -> Result<i32, Failure> {
    let driver = NamingRevolutionDriver::default();
    driver.init(&function.label());
    let mut log = TFunctionLogbook::set(&function.label())?;
    log.clear()?;
    driver.execute(&mut log)
}
