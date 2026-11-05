//! Keep base initialization separate from the genuinely specialized CheckerSI initializer.
use super::BOPAlgoPaveFiller;
use crate::bop_algo::alerts::alert;
use crate::bop_algo::{AlertNullInputShapes, AlertTooFewArguments};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};

impl BOPAlgoPaveFiller {
    /// Prepares the DS, context, broad-phase iterator, and automatic safety mode.
    /// `filler.initialize()?; assert!(filler.data_structure().is_some());`.
    pub fn initialize(&mut self) -> Result<(), martensite_tkernel::standard::Failure> {
        let range = ProgressRange::default();
        self.initialize_with_progress(&range)
    }

    /// Preserve native Init scope placement without adding cancellation polls.
    /// Example: `filler.initialize_with_progress(range)?` for the base receiver only.
    pub(in crate::bop_algo::pave_filler) fn initialize_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        if !self.has_initialization_arguments() {
            return Ok(());
        }
        let _scope = ProgressScope::with_literal(
            range,
            "Initialization of Intersection algorithm",
            1.0,
            false,
        );
        if !self.has_non_null_arguments() {
            return Ok(());
        }
        self.initialize_valid_arguments()
    }

    fn has_initialization_arguments(&self) -> bool {
        if self.arguments.is_empty() {
            self.options.add_error(&alert(AlertTooFewArguments));
            return false;
        }
        true
    }

    fn has_non_null_arguments(&self) -> bool {
        if self.arguments.iter().any(Shape::is_null) {
            self.options.add_error(&alert(AlertNullInputShapes));
            return false;
        }
        true
    }

    fn initialize_valid_arguments(&mut self) -> Result<(), Failure> {
        self.clear();
        self.initialize_data_structure()?;
        self.initialize_context_and_iterator()?;
        self.set_non_destructive_from_arguments()?;
        Ok(())
    }
}
