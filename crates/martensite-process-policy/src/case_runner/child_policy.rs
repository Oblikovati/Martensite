use super::spec::RequestedProcessPolicy;
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
    install_requested_policy(spec, installer)?;
    body(prepared).map_err(CaseFailure::Body)?;
    context.complete()
}

fn install_requested_policy(
    spec: CaseSpec<'_>,
    installer: &dyn PolicyInstaller,
) -> Result<(), CaseFailure> {
    match spec.process_policy {
        RequestedProcessPolicy::Seconds(seconds) => {
            installer.install(seconds).map_err(CaseFailure::Policy)
        }
        RequestedProcessPolicy::Inherit => Ok(()),
    }
}
