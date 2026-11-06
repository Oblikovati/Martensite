//! Preserve analyzer result order and source side/group counters.
use super::{RetainedFullArgumentReport, output};
use martensite_tkbo::bop_algo::{BOPAlgoArgumentAnalyzer, CheckResult, CheckStatus};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::storage::format_general;
use std::fmt::Write;

#[derive(Default)]
pub(super) struct SideGroups {
    pub cases: [usize; 9],
    pub totals: [usize; 9],
}

pub(super) fn collect(
    analyzer: &BOPAlgoArgumentAnalyzer,
    report: &mut RetainedFullArgumentReport,
) -> Result<(), Failure> {
    let mut sides = [SideGroups::default(), SideGroups::default()];
    let mut unknown = false;
    for result in analyzer.check_result().iter() {
        collect_result(result, &mut sides, report, &mut unknown)?;
    }
    output::summaries(&sides, analyzer.shape2().is_null(), &mut report.output);
    if unknown {
        report
            .output
            .push_str("\nWARNING: The unexpected test break occurs!\n");
    }
    Ok(())
}

fn category(status: CheckStatus) -> Option<usize> {
    match status {
        CheckStatus::BadType => Some(0),
        CheckStatus::SelfIntersect => Some(1),
        CheckStatus::OperationAborted => Some(2),
        CheckStatus::TooSmallEdge => Some(3),
        CheckStatus::NonRecoverableFace => Some(4),
        CheckStatus::IncompatibilityOfVertex => Some(5),
        CheckStatus::IncompatibilityOfEdge => Some(6),
        CheckStatus::GeomAbsC0 => Some(7),
        CheckStatus::InvalidCurveOnSurface => Some(8),
        _ => None,
    }
}

fn collect_result(
    result: &CheckResult,
    sides: &mut [SideGroups; 2],
    report: &mut RetainedFullArgumentReport,
    unknown: &mut bool,
) -> Result<(), Failure> {
    let Some(group) = category(result.check_status()) else {
        *unknown |= result.check_status() != CheckStatus::IncompatibilityOfFace;
        return Ok(());
    };
    for side in 0..2 {
        collect_side(result, side, group, sides, report)?;
    }
    Ok(())
}

fn collect_side(
    result: &CheckResult,
    side: usize,
    group: usize,
    sides: &mut [SideGroups; 2],
    report: &mut RetainedFullArgumentReport,
) -> Result<(), Failure> {
    if [result.shape1(), result.shape2()][side].is_null() {
        return Ok(());
    }
    let shapes = [result.faulty_shapes1(), result.faulty_shapes2()][side];
    sides[side].cases[group] += 1;
    if group == 0 || group == 2 || shapes.is_empty() {
        return Ok(());
    }
    bind_faulty(result, side, group, shapes, &mut sides[side], report)
}

fn faulty_compound(shapes: &List<Shape>) -> Result<Shape, Failure> {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    for shape in shapes.iter() {
        builder.add(&mut compound, shape)?;
    }
    Ok(compound.into_shape())
}

fn bind_faulty(
    result: &CheckResult,
    side: usize,
    group: usize,
    shapes: &List<Shape>,
    counts: &mut SideGroups,
    report: &mut RetainedFullArgumentReport,
) -> Result<(), Failure> {
    const PREFIX: [&str; 9] = ["", "si", "", "se", "bf", "bv", "be", "C0", "COnS"];
    let name = format!("s{}{}_{}", side + 1, PREFIX[group], counts.cases[group]);
    let compound = faulty_compound(shapes)?;
    counts.totals[group] += shapes.iter().count();
    append_binding_line(result, side, group, &name, &mut report.output);
    report.bindings.push((name, compound));
    Ok(())
}

fn append_binding_line(
    result: &CheckResult,
    side: usize,
    group: usize,
    name: &str,
    output: &mut String,
) {
    write!(output, "Made faulty shape: {name}").unwrap();
    if group == 8 {
        write_distance(result, side, output);
    }
    output.push('\n');
}

fn write_distance(result: &CheckResult, side: usize, output: &mut String) {
    let distances = [result.max_distance1(), result.max_distance2()];
    let parameters = [result.max_parameter1(), result.max_parameter2()];
    write!(
        output,
        " (MaxDist = {}, MaxPar = {})",
        format_general(distances[side], 17),
        format_general(parameters[side], 17)
    )
    .unwrap();
}

#[cfg(test)]
#[path = "controls.rs"]
mod controls;
