//! Callers may observe real task entry; geometry and default executor stay unchanged.
use crate::{bop_algo::context_tasks::execute_with_launcher, int_tools::Context};
use martensite_tkernel::{osd::Launcher, standard::Failure};

type EntryObserver<'a> = dyn Fn(&Context) -> Result<(), Failure> + Sync + 'a;

/// Borrow one operation-local entry observer and optional owned reservations.
/// Example: `ClassificationExecution { observer: &record, launcher: Some(&launcher) }`.
pub(super) struct ClassificationExecution<'a> {
    pub observer: &'a EntryObserver<'a>,
    pub launcher: Option<&'a Launcher<'a>>,
}

impl Default for ClassificationExecution<'_> {
    fn default() -> Self {
        let observer = &unobserved_entry;
        Self {
            observer,
            launcher: None,
        }
    }
}

impl ClassificationExecution<'_> {
    /// Execute the same actual task callback on reserved or default workers.
    /// Example: `execution.run(tasks, context, classify)?`.
    pub fn run<T: Send>(
        &self,
        tasks: &mut [T],
        context: &Context,
        perform: impl Fn(&mut T, &Context) -> Result<(), Failure> + Sync,
    ) -> Result<(), Failure> {
        if let Some(launcher) = self.launcher {
            return execute_with_launcher(tasks, context, launcher, Context::default, perform);
        }
        crate::bop_algo::run_launcher_context_tasks(tasks, context, true, Context::default, perform)
    }
}

fn unobserved_entry(_context: &Context) -> Result<(), Failure> {
    // Production has no observer side effects or additional cancellation polls.
    Ok(())
}
