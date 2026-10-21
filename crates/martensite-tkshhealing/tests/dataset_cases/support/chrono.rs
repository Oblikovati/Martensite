//! Draw `dchrono name restart` / `dchrono name stop counter text` (Draw_BasicCommands.cxx):
//! a process CPU chronometer whose stop prints `COUNTER text: cpu-seconds`.
use super::log;
use martensite_tkernel::osd::Chronometer;

/// One named scenario chronometer.
pub struct Dchrono(Chronometer);

impl Dchrono {
    /// `dchrono name restart`: reset and start.
    pub fn restart() -> Self {
        let mut chronometer = Chronometer::new(false);
        chronometer.restart();
        Self(chronometer)
    }

    /// `dchrono name stop counter text`: stop, then print the user CPU time (`%.17g`).
    pub fn stop_counter(mut self, text: &str) {
        self.0.stop();
        let (user, _system) = self.0.show_cpu();
        log::puts(&format!("COUNTER {text}: {}", super::printed::g17(user)));
    }
}
