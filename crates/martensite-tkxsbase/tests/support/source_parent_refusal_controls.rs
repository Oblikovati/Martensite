//! Actual descriptor writes test the same original caller boundary without weakening child failures.
use std::{any::Any, cell::Cell, error::Error, io::Write, panic::resume_unwind};

/// Select an actual descriptor/end mutation; `Incident::StdoutFaulty` never edits modelling inputs.
#[derive(Clone, Copy)]
pub(crate) enum Incident {
    StdoutFaulty,
    StderrFaulty,
    DuplicateEnd,
}

/// Invocation-local Cell state is intentionally not Sync; it is never a published algorithm result.
/// Retain whether this invocation is the real child; `StreamProbe::new("Bugs original:")`.
pub(crate) struct StreamProbe {
    executed: Cell<bool>,
    refusal_prefix: &'static str,
    required_events: &'static [&'static str],
}

impl StreamProbe {
    /// Name the exact parent policy refusal; `StreamProbe::new("IntSS original:")`.
    pub(crate) fn new(refusal_prefix: &'static str) -> Self {
        // Descriptor probes need no additional source-event subsequence.
        Self::new_with_events(refusal_prefix, &[])
    }

    /// Require source events before the exact refusal; `StreamProbe::new_with_events("Bugs original:", &["TEST COMPLETED"])`.
    pub(crate) fn new_with_events(
        refusal_prefix: &'static str,
        required_events: &'static [&'static str],
    ) -> Self {
        Self {
            executed: Cell::new(false),
            refusal_prefix,
            required_events,
        }
    }

    /// Mark and run the actual child body without injecting a new cause; `probe.execute(original)`.
    pub(crate) fn execute(
        &self,
        original: fn() -> Result<(), Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>> {
        self.executed.set(true);
        original()
    }

    /// Run the complete original before a real descriptor mutation; `probe.inject(body, Incident::StdoutFaulty)`.
    pub(crate) fn inject(
        &self,
        original: fn() -> Result<(), Box<dyn Error>>,
        incident: Incident,
    ) -> Result<(), Box<dyn Error>> {
        self.execute(original)?;
        emit_incident(incident)?;
        Ok(())
    }

    /// Execute a real child with no completion marker; `probe.missing_end()`.
    pub(crate) fn missing_end(&self) -> Result<(), Box<dyn Error>> {
        self.executed.set(true);
        println!("{{\"control\":\"actual child without source end\"}}");
        Ok(())
    }

    /// Return a named child failure instead of a policy refusal; `probe.failed_child()`.
    pub(crate) fn failed_child(&self) -> Result<(), Box<dyn Error>> {
        self.executed.set(true);
        let cause = std::io::Error::other("named full-stream child failure");
        Err(cause.into())
    }

    /// Accept only semantic parent refusal and preserve failed children; `probe.require_refusal(outcome)`.
    pub(crate) fn require_refusal(&self, outcome: std::thread::Result<()>) {
        if self.executed.get() {
            if let Err(cause) = outcome {
                resume_unwind(cause);
            }
            return;
        }
        let cause =
            outcome.expect_err("whole-stream control: expected exact parent policy refusal");
        let message = panic_message(cause.as_ref());
        assert!(
            message.starts_with(self.refusal_prefix),
            "parent refusal={message:?}: expected prefix {:?}",
            self.refusal_prefix
        );
        require_ordered_events(message, self.required_events);
    }
}

fn emit_incident(incident: Incident) -> std::io::Result<()> {
    let _: &(dyn Send + Sync) = &incident;
    let line = b"Faulty: actual descriptor source-policy control\n";
    match incident {
        Incident::StdoutFaulty => std::io::stdout().write_all(line),
        Incident::StderrFaulty => std::io::stderr().write_all(line),
        Incident::DuplicateEnd => std::io::stdout().write_all(b"TEST COMPLETED\n"),
    }
}

fn panic_message(cause: &(dyn Any + Send)) -> &str {
    if let Some(message) = cause.downcast_ref::<String>() {
        return message;
    }
    cause
        .downcast_ref::<&str>()
        .copied()
        .unwrap_or("<non-text panic>")
}

fn require_ordered_events(message: &str, events: &[&str]) {
    let mut cursor = 0;
    for event in events {
        let position = message[cursor..].find(event).unwrap_or_else(|| {
            panic!("parent cause={message:?}: expected later source event {event:?}")
        });
        cursor += position + event.len();
    }
}
