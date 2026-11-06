//! Original performance commands measure real V3d redraws and retain actual renderer failures.
use super::*;
use martensite_tkernel::{osd::Timer, storage::format_general};
use martensite_tkservice::graphic3d::{
    Graphic3dRenderingParams, Graphic3dRenderingParamsPerfCounters as Perf,
};
use martensite_vulkan::{Graphic3dRenderView, RenderRedrawCursor, RenderRedrawFailure};

const TIMED_REDRAWS: u32 = 100;

/// One actual frame-rate measurement; `session.measure_fps()?.command_text()`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FpsObservation {
    completed_frames: u32,
    elapsed_seconds: f64,
    user_cpu_seconds: f64,
}

impl FpsObservation {
    /// Successful timed publications, excluding warm-up; `observation.completed_frames()`.
    pub fn completed_frames(&self) -> u32 {
        self.completed_frames
    }

    /// Native command return, including its trailing newline; `observation.command_text()`.
    pub fn command_text(&self) -> String {
        let (fps, cpu_ms) = measurement_rates(self);
        format!(
            "FPS: {}\nCPU: {} msec\n",
            format_general(fps, 17),
            format_general(cpu_ms, 17)
        )
    }
}

/// Failure of the actual measurement, retaining stage/frame and the renderer cause.
#[derive(Debug)]
pub enum PerformanceFailure {
    /// Startup, warm-up or a particular timed source frame was refused.
    Redraw {
        /// One-based timed index, or None for startup/warm-up.
        timed_frame: Option<u32>,
        /// Actual view-publication failure.
        cause: RenderRedrawFailure,
    },
    /// The observed host-clock deltas cannot define finite FPS/CPU values.
    InvalidMeasurement {
        /// Number of successfully requested timed frames.
        frames: u32,
        /// Actual wall-clock delta before any validation.
        elapsed: f64,
        /// Actual process-user CPU delta, without system time.
        user_cpu: f64,
    },
    /// The source view has no actual Martensite render-view implementation.
    MissingRenderView {
        /// Actual source-view owner address.
        owner: usize,
    },
}

impl std::fmt::Display for PerformanceFailure {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            output,
            "performance operation {self:?}; expected 100 successful timed redraws after one warm-up on the actual view"
        )
    }
}

impl std::error::Error for PerformanceFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Redraw { cause, .. } => Some(cause),
            _ => None,
        }
    }
}

trait FpsRedraw {
    fn redraw(&mut self) -> Result<(), RenderRedrawFailure>;
}

struct SourceFpsRedraw {
    view: Handle<dyn V3dViewSource>,
    render: Handle<Graphic3dRenderView>,
    cursor: RenderRedrawCursor,
}

impl FpsRedraw for SourceFpsRedraw {
    fn redraw(&mut self) -> Result<(), RenderRedrawFailure> {
        self.view.redraw();
        self.cursor = self.render.check_redraw_since(&self.cursor)?;
        Ok(())
    }
}

impl ViewerSession {
    /// One warm-up and 100 measured source redraws; `session.measure_fps()?`.
    pub fn measure_fps(&self) -> Result<FpsObservation, PerformanceFailure> {
        let mut workload = source_fps_workload(self.view())?;
        measure_default_fps(&mut workload, &mut Timer::new(false))
    }

    /// Performance update interval; `session.set_perf_update_interval(0.)?`.
    pub fn set_perf_update_interval(&self, seconds: f32) -> Result<(), Failure> {
        self.update_view(|view| {
            view.change_rendering_params(&mut |params| set_interval(params, seconds));
            Ok(())
        })
    }

    /// Performance counters with exact parsed bits; `session.set_perf_counters(Perf::NONE)?`.
    pub fn set_perf_counters(&self, counters: Perf) -> Result<(), Failure> {
        self.update_view(|view| {
            view.change_rendering_params(&mut |params| set_counters(params, counters));
            Ok(())
        })
    }
}

fn source_fps_workload(
    view: &Handle<dyn V3dViewSource>,
) -> Result<SourceFpsRedraw, PerformanceFailure> {
    let render = view.view_base().view().cast::<Graphic3dRenderView>();
    let renderer = render.get().ok_or(PerformanceFailure::MissingRenderView {
        owner: view.object_address() as usize,
    })?;
    let cursor = renderer.redraw_cursor().map_err(untimed_failure)?;
    Ok(SourceFpsRedraw {
        view: view.clone(),
        render,
        cursor,
    })
}

fn untimed_failure(cause: RenderRedrawFailure) -> PerformanceFailure {
    PerformanceFailure::Redraw {
        timed_frame: None,
        cause,
    }
}

fn set_interval(params: &mut Graphic3dRenderingParams, seconds: f32) {
    params.stats.stats_update_interval = seconds;
}

fn set_counters(params: &mut Graphic3dRenderingParams, counters: Perf) {
    params.stats.collected_stats = counters;
    params.stats.to_show_stats = counters != Perf::NONE;
}

fn measure_default_fps(
    workload: &mut impl FpsRedraw,
    timer: &mut Timer,
) -> Result<FpsObservation, PerformanceFailure> {
    workload.redraw().map_err(untimed_failure)?;
    timer.start();
    let outcome = measure_redraws(workload);
    timer.stop();
    outcome?;
    checked_measurement(timer.elapsed_time(), timer.user_time_cpu())
}

fn measure_redraws(workload: &mut impl FpsRedraw) -> Result<(), PerformanceFailure> {
    for frame in 1..=TIMED_REDRAWS {
        workload
            .redraw()
            .map_err(|cause| PerformanceFailure::Redraw {
                timed_frame: Some(frame),
                cause,
            })?;
    }
    Ok(())
}

fn checked_measurement(elapsed: f64, user_cpu: f64) -> Result<FpsObservation, PerformanceFailure> {
    let observed = FpsObservation {
        completed_frames: TIMED_REDRAWS,
        elapsed_seconds: elapsed,
        user_cpu_seconds: user_cpu,
    };
    if !valid_measurement(&observed) {
        return Err(PerformanceFailure::InvalidMeasurement {
            frames: TIMED_REDRAWS,
            elapsed,
            user_cpu,
        });
    }
    Ok(observed)
}

fn valid_measurement(observed: &FpsObservation) -> bool {
    let (fps, cpu_ms) = measurement_rates(observed);
    observed.elapsed_seconds > 0.
        && observed.user_cpu_seconds >= 0.
        && [
            observed.elapsed_seconds,
            observed.user_cpu_seconds,
            fps,
            cpu_ms,
        ]
        .iter()
        .all(|value| value.is_finite())
}

fn measurement_rates(observed: &FpsObservation) -> (f64, f64) {
    let frames = f64::from(observed.completed_frames);
    (
        frames / observed.elapsed_seconds,
        1000. * (observed.user_cpu_seconds / frames),
    )
}

#[cfg(test)]
#[path = "performance_tests.rs"]
mod tests;
