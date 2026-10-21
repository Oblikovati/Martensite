use super::{
    CaseFailure, CaseRunError, CaseSpec,
    artifacts::{FilesystemReceipts, ReceiptRecorder, retain_logs, retain_result},
    context::CaseContext,
    failure::io_failure,
    harness,
    launch::{ChildLauncher, ChildObservation, ChildRequest, ExecutableChild},
    lifecycle::{ChildLifecycle, ChildWorkload},
    spec::{CHILD_ENV, FACTOR_ENV},
};
use std::{
    error::Error,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Mutex,
};

/// The retained invocation directory, including request, entry, completion and logs.
///
/// In the child this is a location after successful body execution; only the
/// parent finalizes `receipt.json` after observing a successful process exit.
/// Example: `println!("{}", receipt.directory.display());` references logs explicitly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaseReceipt {
    /// Caller-owned retained artifacts; no automatic cleanup or stdout forwarding.
    pub directory: PathBuf,
}

/// An owned test launcher with caller-provided artifact storage and serialization.
///
/// Share the same borrowed mutex among graphical cases in one host. It does not
/// coordinate independent host processes. This module owns no global/TLS state.
/// ```no_run
/// use martensite_process_policy::case_runner::{CaseRunner, CaseSpec, FactorProfile};
/// let root = std::env::temp_dir().join("case-receipts");
/// let runner = CaseRunner::new(&root, None);
/// runner.run(CaseSpec::new("cases::point", 600, FactorProfile::RequireUnset), || {
///     assert_eq!(1 + 2, 3);
///     Ok(())
/// })?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct CaseRunner<'a> {
    artifact_root: &'a Path,
    graphics_mutex: Option<&'a Mutex<()>>,
}

#[derive(Default)]
pub(crate) struct RunEnvironment {
    pub(crate) marker: Option<OsString>,
    pub(crate) factor: Option<OsString>,
}

impl RunEnvironment {
    fn actual() -> Self {
        Self {
            marker: std::env::var_os(CHILD_ENV),
            factor: std::env::var_os(FACTOR_ENV),
        }
    }
}

impl<'a> CaseRunner<'a> {
    /// Supply storage and optional host-owned graphics serialization explicitly.
    /// Example: `CaseRunner::new(std::path::Path::new("target/case-receipts"), None)`.
    pub fn new(artifact_root: &'a Path, graphics_mutex: Option<&'a Mutex<()>>) -> Self {
        Self {
            artifact_root,
            graphics_mutex,
        }
    }

    /// Run an unchanged body once in the exact matching child, before any fixture.
    ///
    /// The parent applies no process policy and retains raw nested harness logs.
    /// Example: `runner.run(spec, || { assert_eq!(2 + 2, 4); Ok(()) })?;`.
    pub fn run(
        &self,
        spec: CaseSpec<'_>,
        body: impl FnOnce() -> Result<(), Box<dyn Error>>,
    ) -> Result<CaseReceipt, CaseRunError> {
        self.run_with(
            spec,
            body,
            &RunEnvironment::actual(),
            &ExecutableChild,
            &FilesystemReceipts,
        )
    }

    pub(crate) fn run_with(
        &self,
        spec: CaseSpec<'_>,
        body: impl FnOnce() -> Result<(), Box<dyn Error>>,
        environment: &RunEnvironment,
        launcher: &dyn ChildLauncher,
        recorder: &dyn ReceiptRecorder,
    ) -> Result<CaseReceipt, CaseRunError> {
        let workload = ChildWorkload {
            prepare: || Ok(()),
            body: |()| body(),
            lifecycle: ChildLifecycle::Direct,
        };
        self.run_workload(spec, workload, environment, launcher, recorder)
    }

    /// Prepare owned child state before installing the policy, then consume it once.
    ///
    /// Neither callback runs in the parent. `T` remains child-local: it need not be
    /// Send or Sync. A preparation error installs no policy; a policy error drops T.
    /// ```no_run
    /// use martensite_process_policy::case_runner::{CaseRunner, CaseSpec, FactorProfile};
    /// let root = std::env::temp_dir().join("case-receipts");
    /// CaseRunner::new(&root, None).run_prepared(
    ///     CaseSpec::new("cases::prepared", 300, FactorProfile::RequireUnset),
    ///     || Ok(String::from("prepared before the budget")),
    ///     |owned| { assert!(!owned.is_empty()); Ok(()) },
    /// )?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn run_prepared<T>(
        &self,
        spec: CaseSpec<'_>,
        prepare: impl FnOnce() -> Result<T, Box<dyn Error>>,
        body: impl FnOnce(T) -> Result<(), Box<dyn Error>>,
    ) -> Result<CaseReceipt, CaseRunError> {
        let workload = ChildWorkload {
            prepare,
            body,
            lifecycle: ChildLifecycle::Prepared,
        };
        self.run_workload(
            spec,
            workload,
            &RunEnvironment::actual(),
            &ExecutableChild,
            &FilesystemReceipts,
        )
    }

    pub(super) fn run_workload<T, P, B>(
        &self,
        spec: CaseSpec<'_>,
        workload: ChildWorkload<P, B>,
        environment: &RunEnvironment,
        launcher: &dyn ChildLauncher,
        recorder: &dyn ReceiptRecorder,
    ) -> Result<CaseReceipt, CaseRunError>
    where
        P: FnOnce() -> Result<T, Box<dyn Error>>,
        B: FnOnce(T) -> Result<(), Box<dyn Error>>,
    {
        validate_environment(spec, environment).map_err(|e| CaseRunError::new(spec, e))?;
        if let Some(marker) = &environment.marker {
            return run_child(spec, marker, workload);
        }
        self.run_parent(spec, workload.lifecycle, launcher, recorder)
    }

    fn lock_graphics(&self) -> Result<Option<std::sync::MutexGuard<'_, ()>>, CaseFailure> {
        self.graphics_mutex
            .map(|lock| lock.lock())
            .transpose()
            .map_err(|e| harness::mismatch(e.to_string(), "unpoisoned caller graphics mutex"))
    }

    fn run_parent(
        &self,
        spec: CaseSpec<'_>,
        lifecycle: ChildLifecycle,
        launcher: &dyn ChildLauncher,
        recorder: &dyn ReceiptRecorder,
    ) -> Result<CaseReceipt, CaseRunError> {
        let _guard = self
            .lock_graphics()
            .map_err(|e| CaseRunError::new(spec, e))?;
        let context = CaseContext::create_for(self.artifact_root, spec, lifecycle)
            .map_err(|e| CaseRunError::new(spec, e))?;
        let request = ChildRequest {
            context: &context,
            spec,
        };
        let (output, failure) = observe_child(&request, launcher);
        finish_parent(&request, output.as_ref(), failure, recorder)
    }
}

fn observe_child(
    request: &ChildRequest<'_>,
    launcher: &dyn ChildLauncher,
) -> (Option<ChildObservation>, Option<CaseFailure>) {
    match launcher.launch(request) {
        Ok(observed) => {
            let failure =
                validate_observation(request.context, &observed, request.spec.identity).err();
            (Some(observed), failure)
        }
        Err(error) => (
            None,
            Some(io_failure("launch exact current executable", error)),
        ),
    }
}

fn validate_environment(
    spec: CaseSpec<'_>,
    environment: &RunEnvironment,
) -> Result<(), CaseFailure> {
    spec.validate()?;
    if let Some(factor) = &environment.factor {
        return Err(CaseFailure::FactorPresent(factor.clone()));
    }
    Ok(())
}

fn run_child<T, P, B>(
    spec: CaseSpec<'_>,
    marker: &std::ffi::OsStr,
    workload: ChildWorkload<P, B>,
) -> Result<CaseReceipt, CaseRunError>
where
    P: FnOnce() -> Result<T, Box<dyn Error>>,
    B: FnOnce(T) -> Result<(), Box<dyn Error>>,
{
    let context = CaseContext::load_for(marker.into(), spec, workload.lifecycle)
        .map_err(|e| CaseRunError::new(spec, e))?;
    let result =
        super::child_policy::execute_prepared(&context, spec, workload.prepare, workload.body);
    finish_child(spec, context, result)
}

fn finish_child(
    spec: CaseSpec<'_>,
    context: CaseContext,
    result: Result<(), CaseFailure>,
) -> Result<CaseReceipt, CaseRunError> {
    result.map_err(|failure| {
        let mut error = CaseRunError::new(spec, failure);
        error.artifacts = Some(context.directory.clone());
        error
    })?;
    Ok(CaseReceipt {
        directory: context.directory,
    })
}

fn validate_observation(
    context: &CaseContext,
    output: &ChildObservation,
    identity: &str,
) -> Result<(), CaseFailure> {
    if output.code != Some(0) || output.signal.is_some() {
        return Err(CaseFailure::ChildExit {
            code: output.code,
            signal: output.signal,
        });
    }
    context.require_record("entry.txt")?;
    context.require_record("complete.txt")?;
    if context.lifecycle == ChildLifecycle::Prepared {
        context.require_record("prepared.txt")?;
    }
    harness::validate(&output.stdout, identity)?;
    context.require_same_executable()
}

fn finish_parent(
    request: &ChildRequest<'_>,
    output: Option<&ChildObservation>,
    mut failure: Option<CaseFailure>,
    recorder: &dyn ReceiptRecorder,
) -> Result<CaseReceipt, CaseRunError> {
    let secondary = finish_recording(request, output, &mut failure, recorder);
    if let Some(failure) = failure {
        return Err(parent_failure(request, failure, secondary, output));
    }
    Ok(CaseReceipt {
        directory: request.context.directory.clone(),
    })
}

fn finish_recording(
    request: &ChildRequest<'_>,
    output: Option<&ChildObservation>,
    failure: &mut Option<CaseFailure>,
    recorder: &dyn ReceiptRecorder,
) -> Vec<std::io::Error> {
    let mut secondary = retain_logs(request.context, output, recorder);
    if failure.is_none() && !secondary.is_empty() {
        *failure = Some(io_failure("retain raw child logs", secondary.remove(0)));
    }
    if let Err(error) = retain_result(request, output, failure.as_ref(), recorder) {
        if failure.is_none() {
            *failure = Some(io_failure("write parent receipt", error));
        } else {
            secondary.push(error);
        }
    }
    secondary
}

fn parent_failure(
    request: &ChildRequest<'_>,
    failure: CaseFailure,
    secondary: Vec<std::io::Error>,
    output: Option<&ChildObservation>,
) -> CaseRunError {
    CaseRunError {
        identity: request.spec.identity.into(),
        seconds: request.spec.seconds,
        failure: Box::new(failure),
        artifacts: Some(request.context.directory.clone()),
        recording_failures: secondary,
        diagnostics: output.map(|o| Box::new((o.stdout.clone(), o.stderr.clone()))),
    }
}
