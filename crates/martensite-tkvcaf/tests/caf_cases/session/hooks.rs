//! What every script inherits: its group's budget, parse rules, start and end, and whether the
//! script's picture is expected to show anything.
use super::commands::CafSession;
use crate::support::process::{KEEP_ARTIFACTS_ENV, run_case};
use crate::support::rules::{BUG_RULES, DOCUMENT_RULES, Rule, VIEWER_RULES};
use crate::support::script_error::run_script as stopping_on_script_error;
use std::path::PathBuf;

/// What a script's group gives it.
pub struct ScriptGroup {
    pub cpu_seconds: i64,
    pub rules: &'static [Rule],
    pub begin: fn(&mut CafSession),
    pub end: fn(&mut CafSession, &str, ImageCheck),
}

/// What the end of a script checks on the picture it dumps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageCheck {
    /// The script takes no picture at its end.
    None,
    /// The script says its viewer is empty: every pixel is the background.
    Empty,
    /// The script shows a shape: at least one pixel is not the background.
    Drawn,
}

/// The presentation grid: the document opened with an undo limit and one command open.
pub const PRESENTATION: ScriptGroup = ScriptGroup {
    cpu_seconds: 300,
    rules: DOCUMENT_RULES,
    begin: presentation_begin,
    end: picture_end,
};

/// The document bug scripts, which open their documents themselves.
pub const BUGS: ScriptGroup = ScriptGroup {
    cpu_seconds: 1000,
    rules: BUG_RULES,
    begin: bugs_begin,
    end: picture_end,
};

/// The viewer bug scripts.
pub const VIEWER_BUGS: ScriptGroup = ScriptGroup {
    cpu_seconds: 1000,
    rules: VIEWER_RULES,
    begin: viewer_begin,
    end: picture_end,
};

/// Runs one script in its child process and grades its stream.
/// Example: `run_script("bug_cases::a", "tests/bugs/caf/a", &BUGS, ImageCheck::None, |_| {})`.
pub fn run_script(
    identity: &str,
    source: &str,
    group: &ScriptGroup,
    image: ImageCheck,
    body: fn(&mut CafSession),
) {
    run_case(identity, group.cpu_seconds, group.rules, || {
        stopping_on_script_error(|| {
            let budget = martensite_process_policy::reported_process_limit();
            assert_eq!(budget, group.cpu_seconds, "group begin: CPU budget");
            let mut session = CafSession::default();
            (group.begin)(&mut session);
            println!("Executing {source}...");
            body(&mut session);
            (group.end)(&mut session, identity, image);
            session.retire_viewer();
        });
    });
}

fn budget_lines(seconds: i64) {
    println!("cpulimit {seconds} ");
    println!("CPU and elapsed time limit set to {seconds} seconds");
}

fn presentation_begin(session: &mut CafSession) {
    println!("pload OCAF \n1\nchrono qat start ");
    session.new_document(Some("BinOcaf"));
    budget_lines(300);
    session.undo_limit(100);
    session.new_command();
    println!("pload TOPTEST VISUALIZATION ");
}

fn bugs_begin(_session: &mut CafSession) {
    println!("pload TOPTEST \npload VISUALIZATION ");
    budget_lines(1000);
    println!("pload OCAF \n1");
}

fn viewer_begin(_session: &mut CafSession) {
    budget_lines(1000);
}

/// A dump with the check its script's words ask for, then the one completion line.
fn picture_end(session: &mut CafSession, identity: &str, image: ImageCheck) {
    if image != ImageCheck::None {
        session.checkview_screenshot(identity, image);
    }
    println!("TEST COMPLETED");
}

/// Where a kept run writes the picture of `identity`, when it keeps them.
/// Example: `kept_image("bug_cases::a")`.
pub fn kept_image(identity: &str) -> Option<PathBuf> {
    let folder = std::env::var_os(KEEP_ARTIFACTS_ENV)?;
    std::fs::create_dir_all(&folder).unwrap();
    Some(PathBuf::from(folder).join(identity.replace("::", "_") + ".png"))
}
