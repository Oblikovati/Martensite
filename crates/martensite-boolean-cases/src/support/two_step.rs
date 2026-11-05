//! `bop first second` followed by `bopcommon|bopcut|bopfuse|boptuc result`, with the reports
//! of both steps written to the case's stream.
//!
//! A fixture that returns only the shape drops these reports, so a warning never reaches the
//! case's stream; these commands are the scripts' path. A warning alone does not fail a case:
//! the stream is graded by the parse rules, and only those decide.
use super::{
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    properties::Prop,
    session::Session,
};
use crate::common_operation::{
    configured_intersections, performed_section, performed_with_filler, report_result_alerts,
};
use martensite_tkbo::bop_algo::{BOPAlgoPaveFiller, Operation};
use martensite_tkbrep::topods::Shape;

/// The prepared intersections a following result command reuses, as the session keeps them.
/// Example: `let mut two_step = TwoStepBoolean::default();`.
#[derive(Default)]
pub struct TwoStepBoolean {
    filler: Option<BOPAlgoPaveFiller>,
}

impl TwoStepBoolean {
    /// `bop first second`: replaces the prepared intersections and reports their alerts.
    /// Example: `two_step.bop(session, "b1", "b2");`.
    pub fn bop(&mut self, session: &dyn Session, first: &str, second: &str) {
        let (Some(first), Some(second)) = (session.shape(first), session.shape(second)) else {
            println!(" null shapes are not allowed ");
            return;
        };
        let mut filler = configured_intersections(first, second);
        filler.perform();
        report_result_alerts(filler.options());
        self.filler = Some(filler);
    }

    /// The result command of `kind`: reports its alerts and binds `name` unless it failed.
    /// Example: `two_step.result(session, "result", Operation::Cut21);`.
    pub fn result(&mut self, session: &mut dyn Session, name: &str, kind: Operation) {
        let Some((first, second)) = self.prepared_arguments() else {
            return;
        };
        let filler = self.filler.as_mut().expect("prepared intersections");
        let operation = performed_with_filler(&first, &second, kind, filler);
        report_result_alerts(operation.options());
        if operation.options().has_errors() {
            return;
        }
        if operation.shape().is_null() {
            println!(" null shape");
            return;
        }
        session.bind(name, operation.shape());
    }

    /// `bopsection name`: reports its alerts and binds `name` unless it failed.
    /// Example: `two_step.section(session, "result");`.
    pub fn section(&mut self, session: &mut dyn Session, name: &str) {
        let Some((first, second)) = self.prepared_arguments() else {
            return;
        };
        let filler = self.filler.as_mut().expect("prepared intersections");
        let operation = performed_section(&first, &second, filler);
        report_result_alerts(operation.options());
        if operation.options().has_errors() {
            return;
        }
        if operation.shape().is_null() {
            println!(" null shape");
            return;
        }
        session.bind(name, operation.shape());
    }

    /// The two arguments of intersections that succeeded; anything else writes its message.
    fn prepared_arguments(&self) -> Option<(Shape, Shape)> {
        let Some(filler) = self.filler.as_ref() else {
            println!(" prepare PaveFiller first");
            return None;
        };
        if filler.options().has_errors() {
            println!(" PaveFiller has not been done");
            return None;
        }
        let arguments: Vec<Shape> = filler.arguments().iter().cloned().collect();
        let [first, second] = arguments.as_slice() else {
            println!(" wrong number of arguments {}", arguments.len());
            return None;
        };
        Some((first.clone(), second.clone()))
    }
}

/// The last four lines of most scripts over `b1` and `b2`: `bop`, the result command of `kind`,
/// `checkprops result -s` and the view of the result or the operands.
/// Example: `operands_checked(s, Operation::Cut21, Expected::Mass(4.))`.
pub fn operands_checked(session: &mut dyn Session, kind: Operation, expected: Expected) {
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(session, "b1", "b2");
    two_step.result(session, "result", kind);
    checkprops(session, "result", Prop::Area, expected);
    let view = CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area);
    checkview(session, &view);
}
