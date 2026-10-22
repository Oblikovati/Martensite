use super::{CaseFailure, CaseSpec, context::CaseContext};
use std::error::Error;

pub(super) trait PolicyInstaller {
    fn install(&self, seconds: i64) -> Result<(), Box<dyn Error>>;
}

struct OwnedProcessPolicy;

impl PolicyInstaller for OwnedProcessPolicy {
    fn install(&self, seconds: i64) -> Result<(), Box<dyn Error>> {
        crate::set_process_limit(Some(seconds))?;
        Ok(())
    }
}

pub(super) fn execute_prepared<T>(
    context: &CaseContext,
    spec: CaseSpec<'_>,
    prepare: impl FnOnce() -> Result<T, Box<dyn Error>>,
    body: impl FnOnce(T) -> Result<(), Box<dyn Error>>,
) -> Result<(), CaseFailure> {
    execute_prepared_with(context, spec, prepare, body, &OwnedProcessPolicy)
}

#[cfg(test)]
pub(super) fn execute_with(
    context: &CaseContext,
    spec: CaseSpec<'_>,
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
    installer: &dyn PolicyInstaller,
) -> Result<(), CaseFailure> {
    execute_prepared_with(context, spec, || Ok(()), |()| body(), installer)
}

pub(super) fn execute_prepared_with<T>(
    context: &CaseContext,
    spec: CaseSpec<'_>,
    prepare: impl FnOnce() -> Result<T, Box<dyn Error>>,
    body: impl FnOnce(T) -> Result<(), Box<dyn Error>>,
    installer: &dyn PolicyInstaller,
) -> Result<(), CaseFailure> {
    context.claim_entry()?;
    let prepared = prepare().map_err(CaseFailure::Preparation)?;
    context.record_prepared()?;
    installer
        .install(spec.seconds)
        .map_err(CaseFailure::Policy)?;
    body(prepared).map_err(CaseFailure::Body)?;
    context.complete()
}
