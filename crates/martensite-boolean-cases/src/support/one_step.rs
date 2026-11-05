//! The one-step boolean commands, with the session's default options: each reports its alerts
//! to the case's stream and binds a result unless the operation failed, as the command does.
use super::session::Session;
use crate::common_operation::{performed_direct, report_result_alerts};
use martensite_tkbo::{
    bop_algo::{Glue, Operation},
    brep_algo_api::BRepAlgoAPISection,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::precision;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn operands(session: &dyn Session, first: &str, second: &str) -> Option<(Shape, Shape)> {
    Some((
        session.shape(first)?.clone(),
        session.shape(second)?.clone(),
    ))
}

fn one_step(s: &mut dyn Session, result: &str, first: &str, second: &str, kind: Operation) {
    let Some((first, second)) = operands(s, first, second) else {
        println!(" null shapes are not allowed ");
        return;
    };
    let operation = performed_direct(&first, &second, kind, precision::confusion());
    report_result_alerts(operation.options());
    if operation.options().has_errors() {
        return;
    }
    if operation.shape().is_null() {
        println!(" null shape");
        return;
    }
    s.bind(result, operation.shape());
}

/// `bcommon result first second`; `bcommon(s, "result", "b", "a")`.
pub fn bcommon(s: &mut dyn Session, result: &str, first: &str, second: &str) {
    one_step(s, result, first, second, Operation::Common);
}

/// `bfuse result first second`; `bfuse(s, "result", "b", "a")`.
pub fn bfuse(s: &mut dyn Session, result: &str, first: &str, second: &str) {
    one_step(s, result, first, second, Operation::Fuse);
}

/// `bcut result first second`; `bcut(s, "result", "b", "a")`.
pub fn bcut(s: &mut dyn Session, result: &str, first: &str, second: &str) {
    one_step(s, result, first, second, Operation::Cut);
}

/// `btuc result first second`; `btuc(s, "result", "b", "a")`.
pub fn btuc(s: &mut dyn Session, result: &str, first: &str, second: &str) {
    one_step(s, result, first, second, Operation::Cut21);
}

fn configured_section(first: &Shape, second: &Shape, pcurves: bool) -> BRepAlgoAPISection {
    let mut operation = BRepAlgoAPISection::from_shapes(first, second, None, false)
        .unwrap_or_else(|failure| panic!("bsection: {failure:?}"));
    operation.approximation(true);
    operation.compute_pcurve_on1(pcurves);
    operation.compute_pcurve_on2(pcurves);
    operation.set_fuzzy_value(precision::confusion());
    operation.set_run_parallel(false);
    operation.set_non_destructive(false);
    operation.set_glue(Glue::Off);
    operation.set_use_obb(false);
    operation
}

fn dump(write: impl FnOnce(&mut Vec<u8>) -> std::io::Result<()>) {
    let mut text = Vec::new();
    write(&mut text).unwrap();
    print!("{}", String::from_utf8_lossy(&text));
}

/// `bsection result first second`: warnings, then errors unless done, then the binding.
/// Example: `bsection(s, "r", "a", "f")`.
pub fn bsection(s: &mut dyn Session, result: &str, first: &str, second: &str) {
    section_with(s, result, first, second, true);
}

/// `bsection result first second -n2d`; `bsection_no_pcurves(s, "r", "a", "f")`.
pub fn bsection_no_pcurves(s: &mut dyn Session, result: &str, first: &str, second: &str) {
    section_with(s, result, first, second, false);
}

fn section_with(s: &mut dyn Session, result: &str, first: &str, second: &str, pcurves: bool) {
    let Some((first, second)) = operands(s, first, second) else {
        println!(" Null shapes are not allowed ");
        return;
    };
    let mut operation = configured_section(&first, &second, pcurves);
    operation.build().unwrap();
    if operation.has_warnings() {
        dump(|text| operation.dump_warnings(text));
    }
    if !operation.is_done() {
        dump(|text| operation.dump_errors(text));
        return;
    }
    if operation.shape().is_null() {
        println!(" null shape");
        return;
    }
    s.bind(result, operation.shape());
}
