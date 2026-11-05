//! DNamingSelectionDriver from its package at reference.
use super::naming_function_values as values;
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingNamedShape, TNamingSelector, TNamingTool};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    tdf::{TdfLabel, TdfLabelMap, TdfTool},
    tfunction::{TFunctionDriver, TFunctionDriverBase, TFunctionLogbook},
};
use std::sync::Arc;

/// Boundary for the driver's native warning lines; tests inject a named recorder.
pub trait NamingSelectionWarnings: Send + Sync {
    /// Emit one complete warning line; `sink.warning("selection failed")`.
    fn warning(&self, message: &str);
}

struct StdoutSelectionWarnings;
impl NamingSelectionWarnings for StdoutSelectionWarnings {
    fn warning(&self, message: &str) {
        println!("{message}");
    }
}

/// Real selector driver with a replaceable output boundary, not a mocked selector.
pub struct NamingSelectionDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
    warnings: Arc<dyn NamingSelectionWarnings>,
}
impl_transient!(NamingSelectionDriver, "DNamingSelectionDriver", dyn TFunctionDriver, transient,
    interfaces: [dyn TFunctionDriver]);
impl Default for NamingSelectionDriver {
    fn default() -> Self {
        Self::with_warnings(Arc::new(StdoutSelectionWarnings))
    }
}
impl NamingSelectionDriver {
    /// Inject only external warning I/O; `NamingSelectionDriver::with_warnings(sink)`.
    pub fn with_warnings(warnings: Arc<dyn NamingSelectionWarnings>) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: TFunctionDriverBase::default(),
            warnings,
        }
    }
    fn mark_unsolved(
        &self,
        function: &Handle<martensite_tklcaf::tfunction::TFunctionFunction>,
    ) -> Result<i32, Failure> {
        function.set_failure(9999)?;
        self.warnings
            .warning("%%%WARNING: DNamingSelectionDriver::Execute: Selection is Not solved !!!");
        Ok(1)
    }
    fn execute_selection(
        &self,
        function: &Handle<martensite_tklcaf::tfunction::TFunctionFunction>,
        log: &Handle<TFunctionLogbook>,
    ) -> Result<i32, Failure> {
        let result = values::result_label(function)?;
        if result.is_null() {
            return Ok(-1);
        }
        let previous = previous_type(&result)?;
        let Some(valid) = self.solve_result(&result, log)? else {
            return self.mark_unsolved(function);
        };
        if self.finish_result(&result, &valid, previous)? {
            function.set_failure(0)?;
        }
        Ok(0)
    }
    fn solve_result(
        &self,
        result: &TdfLabel,
        log: &Handle<TFunctionLogbook>,
    ) -> Result<Option<TdfLabelMap>, Failure> {
        let mut valid = TdfLabelMap::default();
        log.get_valid_into(&mut valid);
        if !TNamingSelector::new(result).solve(&mut valid)? {
            return Ok(None);
        }
        log.set_valid(result, false)?;
        Ok(Some(valid))
    }
    fn warn_result(
        &self,
        result: &TdfLabel,
        named: &Handle<TNamingNamedShape>,
        found: bool,
    ) -> bool {
        if !found {
            return self.warn_message("not found");
        }
        if named.is_null() {
            return self.warn_message("NULL");
        }
        if named.is_empty() {
            return self.warn_empty(result);
        }
        false
    }
    fn warn_message(&self, reason: &str) -> bool {
        self.warnings.warning(&format!(
            "%%%WARNING: DNamingSelectionDriver::NamedShape is {reason}"
        ));
        true
    }
    fn warn_empty(&self, result: &TdfLabel) -> bool {
        let mut entry = AsciiString::default();
        TdfTool::entry(result, &mut entry);
        self.warnings.warning(&format!(
            "%%%WARNING: DNamingSelectionDriver::NamedShape is EMPTY on Label = {}",
            String::from_utf8_lossy(entry.as_bytes())
        ));
        true
    }
    fn finish_result(
        &self,
        result: &TdfLabel,
        valid: &TdfLabelMap,
        previous: ShapeEnum,
    ) -> Result<bool, Failure> {
        let mut named = Handle::default();
        let found = result.find_attribute_as(&TNamingNamedShape::get_id(), &mut named)?;
        if self.warn_result(result, &named, found) {
            return Ok(false);
        }
        if previous != ShapeEnum::Wire
            || named.get().unwrap().get()?.shape_type()? != ShapeEnum::Compound
        {
            return Ok(true);
        }
        repair_wire(result, valid, &named)?;
        Ok(true)
    }
}

fn repair_wire(
    result: &TdfLabel,
    valid: &TdfLabelMap,
    named: &Handle<TNamingNamedShape>,
) -> Result<(), Failure> {
    let mut wire = Shape::default();
    TNamingTool::find_shape(valid, valid, named, &mut wire)?;
    TNamingBuilder::new(result)?.select(&wire, &wire)
}

fn previous_type(result: &TdfLabel) -> Result<ShapeEnum, Failure> {
    let mut named: Handle<TNamingNamedShape> = Handle::default();
    if !result.find_attribute_as(&TNamingNamedShape::get_id(), &mut named)?
        || named.is_null()
        || named.is_empty()
    {
        return Ok(ShapeEnum::Shape);
    }
    named.get().unwrap().get()?.shape_type()
}

impl TFunctionDriver for NamingSelectionDriver {
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
        self.execute_selection(&function, log)
    }
}
