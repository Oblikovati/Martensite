//! The offset family of scripts: a surface read, made one shell by `sewing` or `shape_divide`, offset
//! by `offsetperform` under `catch`, then checked. One procedure, a row per script.
use crate::runner::{BUGS_CPU_SECONDS, BUGS_RULES, hold_against_reference};
use crate::steps::run_command;
use crate::support::{
    checkview::{CheckView, checkview},
    process::verdict_of,
    script_error::run_script,
    session::Session as _,
};
use crate::table::OffsetCase;
use crate::xs_session::XsSession;
use martensite_tkernel::{message::ProgressRange, standard::Failure};
use martensite_tkmath::geom_abs::JoinType;
use martensite_tkoffset::brep_offset::{BRepOffsetMakeOffset, BRepOffsetMode, MakeOffsetOptions};

/// The tolerance `myoffset` gives `offsetparameter`.
const OFFSET_TOLERANCE: f64 = 1.0e-7;

/// `offsetparameter`, `offsetload` and `offsetperform`: the shape when the builder made one.
fn offset(session: &mut XsSession, case: &OffsetCase) -> Result<(), Failure> {
    let join = if case.intersection {
        JoinType::Intersection
    } else {
        JoinType::Arc
    };
    println!(
        "offsetparameter 1e-7 p {} ",
        if case.intersection { "i" } else { "a" }
    );
    println!("offsetload {} {} ", case.names.1, case.value_word);
    println!("offsetperform result ");
    let mut builder = BRepOffsetMakeOffset::new();
    builder.initialize(
        &session.bound_shape(case.names.1),
        case.value,
        OFFSET_TOLERANCE,
        MakeOffsetOptions {
            mode: BRepOffsetMode::Skin,
            intersection: false,
            self_inter: false,
            join,
            thickening: false,
            remove_int_edges: false,
        },
    );
    builder.make_offset_shape(&ProgressRange::default())?;
    if builder.is_done() {
        session.raster.bind("result", builder.shape());
    } else {
        println!("ERROR. offsetperform operation not done.");
    }
    Ok(())
}

fn script(session: &mut XsSession, case: &OffsetCase) {
    for line in case.heading {
        println!("{line}");
    }
    if case.cpu_seconds != BUGS_CPU_SECONDS {
        println!("cpulimit {} ", case.cpu_seconds);
        println!(
            "CPU and elapsed time limit set to {} seconds",
            case.cpu_seconds
        );
    }
    run_command(session, "testreadiges", &[case.file, case.names.0]);
    let making = if case.divided {
        "shape_divide"
    } else {
        "sewing"
    };
    run_command(session, making, &[case.names.1, case.names.0]);
    println!(
        "***** offsetting with join type {}",
        if case.intersection {
            "Intersection"
        } else {
            "Arc"
        }
    );
    println!("   {}", case.said.1);
    if let Err(failure) = offset(session, case) {
        println!("An exception was caught {failure}");
        println!("Offset case: Faulty (1)");
    } else {
        run_command(session, "checkprops", &["result", "-s", case.area]);
        run_command(session, "checkshape", &["result"]);
        let mut words = vec!["result"];
        words.extend_from_slice(case.counts);
        run_command(session, "checknbshapes", &words);
    }
    let image = case.source.rsplit('/').next().expect("a script name");
    checkview(
        &mut session.raster,
        &CheckView::of("result").image_named(image),
    );
}

/// Runs a row against the reference's verdict.
/// Example: `run_offset_case("cases::bugs_modalg_4::bug8842_1", &row)`.
pub fn run_offset_case(identity: &str, case: &OffsetCase) {
    let body = || {
        println!("Executing {}...", case.source);
        let mut session = XsSession::for_source(&identity.replace("::", "_"), case.source);
        run_script(|| {
            println!("CPU and elapsed time limit set to {BUGS_CPU_SECONDS} seconds");
            script(&mut session, case);
            println!("TEST COMPLETED");
        });
    };
    let verdict = verdict_of(identity, case.cpu_seconds, BUGS_RULES, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        hold_against_reference(identity, &verdict, case.recorded);
    }
}
