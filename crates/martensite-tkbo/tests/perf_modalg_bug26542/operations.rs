//! Existing direct Fuse and copy makers retain the source command defaults.
use crate::common_operation;
use martensite_tkbo::bop_algo::{Operation, Options};
use martensite_tkbrep::{
    brep_tools::History,
    topods::{Shape, ShapeIterator},
};
use martensite_tkernel::{
    message::Gravity,
    osd::Timer,
    precision,
    standard::{Failure, Handle},
    storage::format_general,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};

/// Restore and plain-explode the authentic root; `let (first, second) = case_edges()`.
pub(super) fn case_edges() -> (Shape, Shape) {
    let shape = common_operation::restored_shape(
        include_bytes!(
            "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug26452_edges.brep"
        ),
        ShapeEnum::Compound,
    );
    let mut children = ShapeIterator::new(&shape, true, true);
    let first = Iterator::next(&mut children).expect("explode e: expected original binding e_1");
    let second = Iterator::next(&mut children).expect("explode e: expected original binding e_2");
    (first, second)
}

/// Copy the current binding with native defaults; `copy_binding(&edge)?` keeps mesh copying off.
pub(super) fn copy_binding(source: &Shape) -> Result<Shape, Failure> {
    let mut copier = BRepBuilderAPICopy::new();
    copier.perform(source, true, false)?;
    Ok(copier.shape()?.clone())
}

/// One source timer h and the current command-session history, both replaced in native order.
pub(super) struct FuseSession {
    timer: Timer,
    history: Option<Handle<History>>,
}
impl FuseSession {
    /// Start the actual per-case command context; `let mut commands = FuseSession::new()`.
    pub(super) fn new() -> Self {
        Self {
            timer: Timer::new(false),
            history: None,
        }
    }
    /// Time one actual Fuse and keep its history; `commands.timed_fuse(a, b, "COUNTER1")?`.
    pub(super) fn timed_fuse(
        &mut self,
        first: &Shape,
        second: &Shape,
        counter: &str,
    ) -> Result<Shape, Failure> {
        self.timer.restart();
        let result = direct_fuse(first, second, &mut self.history)?;
        self.timer.stop();
        // Draw::dchrono counter uses OSDChronometer::UserTimeCPU, never elapsed/system time.
        println!(
            "COUNTER {counter}: {}",
            format_general(self.timer.show_values().3, 17)
        );
        Ok(result)
    }
}

fn direct_fuse(
    first: &Shape,
    second: &Shape,
    history: &mut Option<Handle<History>>,
) -> Result<Shape, Failure> {
    let mut operation =
        common_operation::performed_direct(first, second, Operation::Fuse, precision::confusion());
    report_alerts(operation.options());
    // BOPTest::bsmt replaces session history before it tests HasErrors or publishes the result.
    *history = operation.history();
    if operation.options().has_errors() || operation.shape().is_null() {
        return Err(Failure::with_message(
            "Standard_NotDone",
            "bfuse: expected performed non-null result after actual reports",
        ));
    }
    Ok(operation.shape().clone())
}

fn report_alerts(options: &Options) {
    use common_operation::case_bop_alerts::gravity_alert_text;
    for (gravity, prefix) in [(Gravity::Warning, "Warning: "), (Gravity::Fail, "Error: ")] {
        print!("{}", gravity_alert_text(options.report(), gravity, prefix));
    }
}
