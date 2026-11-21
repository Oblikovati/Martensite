//! Timing, signals and package statics: timers, chronometers and `Osd`.
mod chronometer;
mod clocks;
mod cpu_native;
mod osd_statics;
mod real_text;
#[cfg(test)]
mod signal_capabilities;
mod signal_native;
mod signals;
mod sleep;
mod timer;
mod wall_native;

pub use chronometer::Chronometer;
pub(crate) use clocks::NativeWallClock;
pub use clocks::{CpuTimes, NativeCpuTimes, WallClock};
pub use osd_statics::Osd;
pub use signals::{NativeSignalHost, SignalHost, SignalState};
pub use sleep::{
    NativeSleeper, SleepRequest, Sleeper, milli_sec_sleep_duration, sec_sleep_duration,
};
pub use timer::Timer;

#[cfg(unix)]
pub(super) use signal_native::install_process_cpu_exit;
