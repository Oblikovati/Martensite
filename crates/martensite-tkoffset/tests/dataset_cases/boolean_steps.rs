//! `bfuse`, `bcut`, `bcommon` and `bsection`, with the session's default options.
use crate::script::OffsetScript;
use crate::steps::{Bool, BopKind};
use crate::support::{
    common_operation::{
        configured_section_arguments, direct_with_fuzzy, intersections, performed_with_filler,
        report_result_alerts,
    },
    one_step::{bcommon, bcut, bfuse, bsection},
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkernel::standard::Failure;

/// Runs one boolean command; `apply(script, &Bool::fuse("r", "a", "b"))`.
pub fn apply(script: &mut OffsetScript<'_>, step: &Bool) -> Result<(), Failure> {
    if matches!(step, Bool::CutBlend(..) | Bool::FuseBlend(..)) {
        let mut commands = std::mem::take(&mut script.fillets);
        let outcome = commands.boolean(script, step);
        script.fillets = commands;
        return outcome;
    }
    if let Bool::Fuzzy(value) = step {
        script.extras.fuzzy = Some(*value);
        return Ok(());
    }
    if let Bool::Prepare(a, b) = step {
        let (a, b) = (script.required(a)?, script.required(b)?);
        let filler = intersections(&a, &b);
        script.prepared = Some((a, b, filler));
        return Ok(());
    }
    if let Bool::TwoStep(result, kind) = step {
        return two_step(script, result, *kind);
    }
    if let (Some(fuzzy), Bool::Fuse(result, a, b)) = (script.extras.fuzzy, step) {
        let (first, second) = (script.required(a)?, script.required(b)?);
        let (shape, _) = direct_with_fuzzy(&first, &second, Operation::Fuse, fuzzy);
        script.set_shape(result, shape);
        return Ok(());
    }
    let session = script.session();
    match step {
        Bool::Fuse(result, a, b) => bfuse(session, result, a, b),
        Bool::Cut(result, a, b) => bcut(session, result, a, b),
        Bool::Common(result, a, b) => bcommon(session, result, a, b),
        Bool::Section(result, a, b) => bsection(session, result, a, b),
        Bool::Fuzzy(_)
        | Bool::Prepare(..)
        | Bool::TwoStep(..)
        | Bool::CutBlend(..)
        | Bool::FuseBlend(..) => {
            unreachable!("handled above")
        }
    }
    Ok(())
}

/// `bopcut result` and the others, on the arguments and filler of the last `bop a b`.
fn two_step(
    script: &mut OffsetScript<'_>,
    result: &'static str,
    kind: BopKind,
) -> Result<(), Failure> {
    let Some((first, second, mut filler)) = script.prepared.take() else {
        return Err(crate::script::command_stopped(" prepare PaveFiller first"));
    };
    if matches!(kind, BopKind::Section) {
        let mut section = configured_section_arguments(&[first.clone(), second.clone()]);
        section.perform_with_filler(&mut filler);
        report_result_alerts(section.options());
        if !section.options().has_errors() && !section.shape().is_null() {
            script.set_shape(result, section.shape().clone());
            script.last_history = section.history();
        }
        script.prepared = Some((first, second, filler));
        return Ok(());
    }
    let operation_kind = match kind {
        BopKind::Common => Operation::Common,
        BopKind::Fuse => Operation::Fuse,
        BopKind::Cut => Operation::Cut,
        BopKind::Tuc => Operation::Cut21,
        BopKind::Section => Operation::Section,
    };
    let mut operation = performed_with_filler(&first, &second, operation_kind, &mut filler);
    report_result_alerts(operation.options());
    if !operation.options().has_errors() && !operation.shape().is_null() {
        script.set_shape(result, operation.shape().clone());
        script.last_history = operation.history();
    }
    script.prepared = Some((first, second, filler));
    Ok(())
}
