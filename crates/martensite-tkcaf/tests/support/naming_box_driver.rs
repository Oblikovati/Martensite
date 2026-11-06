//! Native DNamingBoxDriver fixture, from its package at reference.
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::{TNaming, TNamingBuilder};
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tklcaf::{
    tdf::TdfLabel,
    tfunction::{TFunctionDriver, TFunctionDriverBase, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer,
};

/// The real box computation driver, not a mock geometry/history producer.
#[derive(Default)]
pub struct NamingBoxDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
}
impl_transient!(NamingBoxDriver, "DNamingBoxDriver", dyn TFunctionDriver, transient,
    interfaces: [dyn TFunctionDriver]);

fn argument_sizes(function: &Handle<TFunctionFunction>) -> Result<[f64; 3], Failure> {
    Ok([
        real_argument(function, 1)?.get().unwrap().get(),
        real_argument(function, 2)?.get().unwrap().get(),
        real_argument(function, 3)?.get().unwrap().get(),
    ])
}

fn checked_box(
    function: &Handle<TFunctionFunction>,
) -> Result<Option<BRepPrimAPIMakeBox>, Failure> {
    let [dx, dy, dz] = argument_sizes(function)?;
    let mut maker = BRepPrimAPIMakeBox::from_sizes(dx, dy, dz)?;
    maker.build(&ProgressRange::default())?;
    if !maker.is_done() {
        function.set_failure(11)?;
        return Ok(None);
    }
    let solid = maker.solid()?.into_shape();
    if !BRepCheckAnalyzer::new(&solid, true, false, false)?.is_valid_shape(&solid)? {
        function.set_failure(12)?;
        return Ok(None);
    }
    Ok(Some(maker))
}

fn generated_on(label: &TdfLabel, shape: Shape) -> Result<(), Failure> {
    TNamingBuilder::new(label)?.generated(&shape)
}

fn load_box_names(label: &TdfLabel, maker: &mut BRepPrimAPIMakeBox) -> Result<(), Failure> {
    generated_on(label, maker.solid()?.into_shape())?;
    let faces = [
        maker.bottom_face()?,
        maker.top_face()?,
        maker.front_face()?,
        maker.right_face()?,
        maker.back_face()?,
        maker.left_face()?,
    ];
    for (index, face) in faces.into_iter().enumerate() {
        generated_on(
            &label.find_child(index as i32 + 1, true)?,
            face.into_shape(),
        )?;
    }
    Ok(())
}

impl TFunctionDriver for NamingBoxDriver {
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
        let Some(function) = function_on(&self.label())? else {
            return Ok(-1);
        };
        let location = previous_location(&function)?;
        let Some(mut maker) = checked_box(&function)? else {
            return Ok(-1);
        };
        let result = result_label(&function)?;
        load_box_names(&result, &mut maker)?;
        if !location.is_identity() {
            TNaming::displace(&result, &location, true)?;
        }
        log.set_valid(&result, true)?;
        function.set_failure(0)?;
        Ok(0)
    }
}

use super::naming_function_values as values;
use values::{function_on, previous_location};
pub use values::{function_result, real_argument, result_label};
