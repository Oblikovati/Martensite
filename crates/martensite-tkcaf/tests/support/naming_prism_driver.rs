//! Actual DNamingPrismDriver: planar basis conversion, validation, naming and recomputation.
use super::naming_function_values as values;
use super::naming_sweep_history::{cap_names, sweep_history};
use super::{
    naming_box_driver::{real_argument, result_label},
    naming_sweep_basis::{basis_context, planar_basis, sweep_axis, valid_result},
    naming_transform_arguments::argument_value,
};
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkcaf::tnaming::TNaming;
use martensite_tkernel::{
    impl_transient, precision,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    tdf::{TdfAttributeState, TdfLabel},
    tfunction::{TFunctionDriver, TFunctionDriverBase, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::gp::Vector3;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakePrism, BRepPrimAPIMakeSweep};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use values::optional_failure as failed;
/// Real document prism driver; `NamingPrismDriver::default()`.
#[derive(Default)]
pub struct NamingPrismDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
}
impl_transient!(NamingPrismDriver, "DNamingPrismDriver", dyn TFunctionDriver, transient, interfaces: [dyn TFunctionDriver]);
struct PrismInputs {
    basis: Shape,
    context: Shape,
    vector: Vector3,
}
impl PrismInputs {
    fn new(basis: Shape, context: Shape, vector: Vector3) -> Self {
        Self {
            basis,
            context,
            vector,
        }
    }
}
fn sweep_vector(
    function: &Handle<TFunctionFunction>,
    original: &Shape,
) -> Result<Option<Vector3>, Failure> {
    let height = real_argument(function, 2)?.get().unwrap().get();
    if height <= precision::confusion() {
        return failed(function, 14);
    }
    let mut axis = sweep_axis(original)?;
    if values::integer_argument(function, 3)?.get().unwrap().get() != 0 {
        axis.reverse();
    }
    Ok(Some(Vector3::from(axis.direction()) * height))
}
fn prism_inputs(function: &Handle<TFunctionFunction>) -> Result<Option<PrismInputs>, Failure> {
    let named = argument_value(function, 1)?;
    if named.is_null() || named.is_empty() {
        return failed(function, 14);
    }
    let original = named.get().unwrap().get()?;
    let Some(basis) = planar_basis(&original)? else {
        return failed(function, 14);
    };
    let Some(context) = basis_context(function, &basis)? else {
        return Ok(None);
    };
    let Some(vector) = sweep_vector(function, &original)? else {
        return Ok(None);
    };
    Ok(Some(PrismInputs::new(basis, context, vector)))
}
fn checked_prism(
    function: &Handle<TFunctionFunction>,
    inputs: &PrismInputs,
) -> Result<Option<BRepPrimAPIMakePrism>, Failure> {
    let mut maker = BRepPrimAPIMakePrism::new(&inputs.basis, inputs.vector, true, true)?;
    maker.build(&martensite_tkernel::message::ProgressRange::default())?;
    if !maker.is_done() {
        return failed(function, 11);
    }
    if !valid_result(maker.shape()?)? {
        return failed(function, 12);
    }
    Ok(Some(maker))
}
fn top_bottom_needed(basis: &Shape) -> Result<bool, Failure> {
    if basis.shape_type()? == ShapeEnum::Wire {
        return Ok(false);
    }
    if basis.shape_type()? != ShapeEnum::Compound {
        return Ok(true);
    }
    let first = Iterator::next(&mut ShapeIterator::new(basis, true, true));
    Ok(first.is_none_or(|shape| shape.shape_type().ok() != Some(ShapeEnum::Wire)))
}
fn load_names(
    result: &TdfLabel,
    maker: &mut BRepPrimAPIMakePrism,
    inputs: &PrismInputs,
) -> Result<(), Failure> {
    let orientations = sweep_history(result, maker, &inputs.basis, &inputs.context)?;
    if top_bottom_needed(&inputs.basis)? {
        cap_names(result, &maker.first_shape()?, &orientations)?;
        cap_names(result, &maker.last_shape()?, &orientations)?;
    }
    Ok(())
}
fn execute_function(
    function: &Handle<TFunctionFunction>,
    log: &Handle<TFunctionLogbook>,
) -> Result<i32, Failure> {
    let location = values::previous_location(function)?;
    let Some(inputs) = prism_inputs(function)? else {
        return Ok(-1);
    };
    let Some(mut maker) = checked_prism(function, &inputs)? else {
        return Ok(-1);
    };
    let result = result_label(function)?;
    load_names(&result, &mut maker, &inputs)?;
    if !location.is_identity() {
        TNaming::displace(&result, &location, true)?;
    }
    log.set_valid(&result, true)?;
    function.set_failure(0)?;
    Ok(0)
}
impl TFunctionDriver for NamingPrismDriver {
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
/// Execute the native driver with a fresh logbook; `execute_prism(&function)?`.
pub fn execute_prism(function: &Handle<TFunctionFunction>) -> Result<i32, Failure> {
    let driver = NamingPrismDriver::default();
    driver.init(&function.label());
    let mut log = TFunctionLogbook::set(&function.label())?;
    log.clear()?;
    driver.execute(&mut log)
}
