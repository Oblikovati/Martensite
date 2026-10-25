//! The existing process-wide CPU cap and elapsed watchdog, independent of command parsing.
use crate::cpu_limits as os_limits;
use martensite_tkernel::osd::Timer;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

/// The elapsed-time clock of `cpulimit` (`static OSD_Timer aTimer`).
static ELAPSED: Mutex<Option<Timer>> = Mutex::new(None);
/// `static int aFirst`: the watchdog thread is started by the first call only.
static WATCHDOG_STARTED: AtomicBool = AtomicBool::new(false);

/// A failed operating-system cap installation; the elapsed policy still restarts.
/// Usage: `eprintln!("{}", error.raw_status());`.
#[derive(Debug)]
pub struct ProcessLimitError {
    seconds: Option<i64>,
    raw_status: i32,
    cause: std::io::Error,
}

impl ProcessLimitError {
    /// The C installation status; zero is the expected successful value.
    /// Usage: `assert_ne!(error.raw_status(), 0);`.
    pub fn raw_status(&self) -> i32 {
        self.raw_status
    }
}

impl std::fmt::Display for ProcessLimitError {
    fn fmt(&self, text: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            text,
            "process limit {:?} seconds: OS setup returned {}, expected 0: {}",
            self.seconds, self.raw_status, self.cause
        )
    }
}

impl std::error::Error for ProcessLimitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.cause)
    }
}

/// Install the existing process-global CPU/elapsed policy and restart its clock.
///
/// This changes the whole process, installs a signal handler on Unix and starts one
/// permanent five-second watchdog. It is not an operation-local or RAII deadline.
/// `None` removes the CPU cap; signed values retain the command's legacy cast to
/// the OS unsigned limit. On non-Unix platforms only the elapsed policy applies.
/// Even an OS refusal retains the reported limit and restarts the elapsed clock.
/// Usage in a dedicated process:
/// ```no_run
/// # fn main() -> Result<(), martensite_process_policy::ProcessLimitError> {
/// martensite_process_policy::set_process_limit(Some(200))?;
/// # Ok(())
/// # }
/// ```
pub fn set_process_limit(seconds: Option<i64>) -> Result<(), ProcessLimitError> {
    match apply_process_limit(seconds, |_| Ok::<_, std::convert::Infallible>(())) {
        Ok(result) => result,
        Err(never) => match never {},
    }
}

/// The command emits an installation diagnostic before starting the elapsed watchdog.
/// A failed output aborts exactly where the original command's Append did.
///
/// The outer result is diagnostic-output success; the inner result is OS setup.
/// Use in a dedicated process:
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// martensite_process_policy::apply_process_limit(Some(200), |refusal| {
///     if let Some(error) = refusal { eprintln!("{error}"); }
///     Ok::<_, std::io::Error>(())
/// })??;
/// # Ok(())
/// # }
/// ```
pub fn apply_process_limit<E>(
    seconds: Option<i64>,
    report: impl FnOnce(Option<&ProcessLimitError>) -> Result<(), E>,
) -> Result<Result<(), ProcessLimitError>, E> {
    let result = install_cpu_cap(seconds);
    finish_process_policy(result, report, start_process_watchdog)
}

fn finish_process_policy<E>(
    result: Result<(), ProcessLimitError>,
    report: impl FnOnce(Option<&ProcessLimitError>) -> Result<(), E>,
    activate: impl FnOnce(),
) -> Result<Result<(), ProcessLimitError>, E> {
    report(result.as_ref().err())?;
    activate();
    Ok(result)
}

fn start_process_watchdog() {
    restart_elapsed_clock();
    if !WATCHDOG_STARTED.swap(true, Ordering::SeqCst) {
        std::thread::spawn(watch_elapsed_time);
    }
}

fn install_cpu_cap(seconds: Option<i64>) -> Result<(), ProcessLimitError> {
    os_limits::set_cpu_rlimit(seconds).map_err(|cause| ProcessLimitError {
        seconds,
        raw_status: cause.raw_os_error().unwrap_or(-1),
        cause,
    })
}

/// Restarts the elapsed-time clock the watchdog reads.
fn restart_elapsed_clock() {
    let mut clock = ELAPSED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let timer = clock.get_or_insert_with(|| Timer::new(false));
    timer.reset();
    timer.start();
}

/// The watchdog: every 5 s, ends the process (status 2) once the elapsed time reached
/// a positive limit.
fn watch_elapsed_time() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        let elapsed = ELAPSED
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_ref()
            .map_or(0.0, Timer::elapsed_time);
        let limit = os_limits::reported_cpu_limit();
        if limit > 0 && elapsed as i64 >= limit {
            println!("Process killed by elapsed limit  ({limit} sec)");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
#[path = "process_limit_tests.rs"]
mod tests;
