//! Both cases require FPS, the ref dump and the counter-bearing dump in source order.
use super::prelude::*;
use martensite_tkservice::graphic3d::Graphic3dRenderingParamsPerfCounters as Perf;

pub(super) const VERBOSE_WITHOUT_TIMING: Perf =
    Perf(Perf::EXTENDED.0 & !(Perf::FRAME_RATE.0 | Perf::CPU.0));

trait CullingPhaseCommands {
    fn print_fps(&mut self) -> Result<(), Box<dyn Error>>;
    fn counters(&mut self, flags: Perf) -> Result<(), Box<dyn Error>>;
    fn dump(&mut self, name: &'static str) -> Result<(), Box<dyn Error>>;
    fn displayed(&mut self, kind: &str);
}

impl CullingPhaseCommands for SessionRun {
    fn print_fps(&mut self) -> Result<(), Box<dyn Error>> {
        println!("{}", self.session().measure_fps()?.command_text());
        Ok(())
    }

    fn counters(&mut self, flags: Perf) -> Result<(), Box<dyn Error>> {
        self.session().set_perf_counters(flags)?;
        Ok(())
    }

    fn dump(&mut self, name: &'static str) -> Result<(), Box<dyn Error>> {
        SessionRun::dump(self, name)
    }

    fn displayed(&mut self, kind: &str) {
        println!("All {kind} were displayed.");
    }
}

/// Execute one actual original phase; `observe_culling_phase(run, off_ref, off_counters, "spheres")?`.
pub(super) fn observe_culling_phase(
    run: &mut SessionRun,
    reference: &'static str,
    counters: &'static str,
    kind: &str,
) -> Result<(), Box<dyn Error>> {
    source_culling_phase(run, reference, counters, kind)
}

fn source_culling_phase(
    commands: &mut impl CullingPhaseCommands,
    reference: &'static str,
    counters: &'static str,
    kind: &str,
) -> Result<(), Box<dyn Error>> {
    commands.print_fps()?;
    commands.counters(Perf::NONE)?;
    commands.dump(reference)?;
    commands.counters(VERBOSE_WITHOUT_TIMING)?;
    commands.dump(counters)?;
    commands.displayed(kind);
    Ok(())
}

#[cfg(test)]
#[path = "culling_phase_tests.rs"]
mod tests;
