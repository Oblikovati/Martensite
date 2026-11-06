use super::*;
use martensite_tkernel::osd::{CpuTimes, WallClock};
use martensite_tkservice::null_driver::NullGraphicDriver;
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

struct NamedFpsClock {
    ticks: AtomicU32,
    user_step: f64,
}

impl CpuTimes for NamedFpsClock {
    fn process_cpu(&self) -> (f64, f64) {
        let ticks = f64::from(self.ticks.load(Ordering::Relaxed));
        (ticks * self.user_step, ticks * 5.)
    }

    fn thread_cpu(&self) -> (f64, f64) {
        (999., 999.)
    }
}

impl WallClock for NamedFpsClock {
    fn now(&self) -> f64 {
        2. * f64::from(self.ticks.load(Ordering::Relaxed))
    }
}

struct NamedFpsWorkload {
    clock: Arc<NamedFpsClock>,
    calls: u32,
    fail_at: Option<u32>,
}

impl FpsRedraw for NamedFpsWorkload {
    fn redraw(&mut self) -> Result<(), RenderRedrawFailure> {
        self.calls += 1;
        if self.fail_at == Some(self.calls) {
            return Err(RenderRedrawFailure::NoFreshPublication {
                before: 17,
                observed: 17,
            });
        }
        self.clock.ticks.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

fn named_measurement(fail_at: Option<u32>, user_step: f64) -> (NamedFpsWorkload, Timer) {
    let clock = Arc::new(NamedFpsClock {
        ticks: AtomicU32::new(0),
        user_step,
    });
    let timer = Timer::with_cpu(false, clock.clone(), clock.clone());
    (
        NamedFpsWorkload {
            clock,
            calls: 0,
            fail_at,
        },
        timer,
    )
}

#[test]
fn exactly_one_warmup_precedes_one_hundred_timed_process_user_frames() {
    let (mut workload, mut timer) = named_measurement(None, 1.);
    let observed = measure_default_fps(&mut workload, &mut timer).unwrap();
    assert_eq!(workload.calls, 101);
    assert_eq!(observed.completed_frames(), 100);
    assert_eq!(observed.elapsed_seconds, 200.);
    assert_eq!(observed.user_cpu_seconds, 100.);
    assert_eq!(observed.command_text(), "FPS: 0.5\nCPU: 1000 msec\n");
    assert!(!timer.is_started());
}

#[test]
fn failed_warmup_never_starts_the_measurement() {
    let (mut workload, mut timer) = named_measurement(Some(1), 1.);
    let failure = measure_default_fps(&mut workload, &mut timer).unwrap_err();
    assert!(matches!(
        failure,
        PerformanceFailure::Redraw {
            timed_frame: None,
            ..
        }
    ));
    assert_eq!(workload.calls, 1);
    assert!(!timer.is_started());
}

#[test]
fn failed_middle_frame_stops_and_returns_its_typed_cause() {
    let (mut workload, mut timer) = named_measurement(Some(52), 1.);
    let failure = measure_default_fps(&mut workload, &mut timer).unwrap_err();
    assert!(matches!(
        failure,
        PerformanceFailure::Redraw {
            timed_frame: Some(51),
            ..
        }
    ));
    assert_eq!(workload.calls, 52);
    assert!(!timer.is_started());
    assert!(std::error::Error::source(&failure).is_some());
    assert!(failure.to_string().contains("51"));
}

#[test]
fn native_double_precision_and_quantized_zero_cpu_are_preserved() {
    assert_eq!(
        checked_measurement(3., 0.1).unwrap().command_text(),
        "FPS: 33.333333333333336\nCPU: 1 msec\n"
    );
    let (mut workload, mut timer) = named_measurement(None, 0.);
    assert_eq!(
        measure_default_fps(&mut workload, &mut timer)
            .unwrap()
            .command_text(),
        "FPS: 0.5\nCPU: 0 msec\n"
    );
    assert_eq!(workload.clock.thread_cpu(), (999., 999.));
}

#[test]
fn invalid_clock_observations_are_refused_without_clamping() {
    for (wall, user) in [
        (0., 1.),
        (-1., 1.),
        (f64::NAN, 1.),
        (1., f64::INFINITY),
        (1., -1.),
        (f64::MIN_POSITIVE, 1.),
    ] {
        assert!(matches!(
            checked_measurement(wall, user),
            Err(PerformanceFailure::InvalidMeasurement { .. })
        ));
    }
}

#[test]
fn source_parameter_commands_keep_zero_interval_and_exact_counter_modes() {
    let session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 37, 19).unwrap();
    session.set_perf_update_interval(0.).unwrap();
    for bits in [0, 0x3fc, 0, 0x3fc] {
        session.set_perf_counters(Perf(bits)).unwrap();
        let observed = session.view().rendering_params().stats;
        assert_eq!(observed.collected_stats.0, bits);
        assert_eq!(observed.to_show_stats, bits != 0);
        assert_eq!(observed.stats_update_interval, 0.);
    }
}

#[test]
fn a_non_rendering_view_is_an_explicit_measurement_failure() {
    let session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 37, 19).unwrap();
    let failure = session.measure_fps().unwrap_err();
    assert!(matches!(
        failure,
        PerformanceFailure::MissingRenderView { .. }
    ));
    assert!(std::error::Error::source(&failure).is_none());
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<FpsObservation>();
}
