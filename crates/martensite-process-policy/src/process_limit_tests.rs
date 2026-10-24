use super::{ProcessLimitError, finish_process_policy};
use std::{cell::RefCell, error::Error};

#[derive(Default)]
struct RecordingProcessSetup {
    events: RefCell<Vec<&'static str>>,
    refuse_diagnostic: bool,
}

impl RecordingProcessSetup {
    fn report(&self, error: Option<&ProcessLimitError>) -> Result<(), &'static str> {
        let error = error.expect("the named control supplies an OS installation refusal");
        assert_eq!(error.raw_status(), -1);
        assert!(error.source().is_some());
        self.events.borrow_mut().push("diagnostic");
        if self.refuse_diagnostic {
            return Err("diagnostic output refused");
        }
        Ok(())
    }

    fn activate(&self) {
        self.events.borrow_mut().push("clock/watchdog");
    }
}

fn refused_installation() -> Result<(), ProcessLimitError> {
    Err(ProcessLimitError {
        seconds: Some(200),
        raw_status: -1,
        cause: std::io::Error::from_raw_os_error(1),
    })
}

#[test]
fn installation_refusal_reports_before_activation_and_retains_cause() {
    let control = RecordingProcessSetup::default();
    let result = finish_process_policy(
        refused_installation(),
        |e| control.report(e),
        || control.activate(),
    );
    let error = result.unwrap().unwrap_err();
    assert_eq!(&*control.events.borrow(), &["diagnostic", "clock/watchdog"]);
    assert!(error.to_string().contains("Some(200) seconds"));
    assert!(error.to_string().contains("returned -1, expected 0"));
    assert!(
        error
            .source()
            .unwrap()
            .downcast_ref::<std::io::Error>()
            .is_some()
    );
}

#[test]
fn diagnostic_refusal_prevents_clock_and_watchdog_activation() {
    let control = RecordingProcessSetup {
        refuse_diagnostic: true,
        ..Default::default()
    };
    let result = finish_process_policy(
        refused_installation(),
        |e| control.report(e),
        || control.activate(),
    );
    assert_eq!(result.unwrap_err(), "diagnostic output refused");
    assert_eq!(&*control.events.borrow(), &["diagnostic"]);
}
