//! `XProgress +t`: an indicator that writes the command's `Progress` lines, with the names of the
//! scopes the library opened, and the check of `bug21264` and `bug30788` that each command reports
//! through its range.
use crate::support::session::Session;
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks, ProgressRange, ProgressScope},
    precision,
    storage::IoStream,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

/// The smallest advance the indicator writes, a hundredth of the whole.
const UPDATE_THRESHOLD: f64 = 0.01;

/// How many lines the indicator wrote, where the last one was, and every scope name it saw.
struct WrittenLines {
    lines: Arc<AtomicUsize>,
    last_position: Mutex<f64>,
    names: Arc<Mutex<Vec<String>>>,
}

fn scope_progress(scope: &ProgressScope<'_>) -> String {
    let value = scope.value();
    if !scope.is_infinite() {
        return format!("{value:.0} / {:.0}", scope.max_value());
    }
    if precision::is_infinite(value) {
        "finished".to_owned()
    } else {
        format!("{value:.0}")
    }
}

/// `Progress: N% name: value / max ...` for the named scopes from the root down.
fn progress_text(scope: &ProgressScope<'_>, position: f64) -> String {
    let mut chain = Vec::new();
    let mut current = Some(scope);
    while let Some(next) = current {
        chain.push(next);
        current = next.parent();
    }
    let mut text = format!("Progress: {:.0}%", 100.0 * position);
    for part in chain.iter().rev() {
        if let Some(name) = part.name() {
            let name = String::from_utf8_lossy(name);
            text.push_str(&format!(" {name}: {}", scope_progress(part)));
        }
    }
    text
}

impl ProgressIndicatorHooks for WrittenLines {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, force: bool) {
        let mut last = self.last_position.lock().unwrap();
        let too_small = (position - *last).abs() < UPDATE_THRESHOLD;
        if !force && (1.0 - position) > precision::confusion() && too_small {
            return;
        }
        *last = position;
        let text = progress_text(scope, position);
        self.names.lock().unwrap().push(text.clone());
        println!("{text}");
        self.lines.fetch_add(1, Ordering::SeqCst);
    }
}

/// Runs one command in a fresh range; its `isTracked` line unless it wrote a `Progress` line.
/// Example: `tracked(|range| filler.perform_with_progress(range));`.
pub fn tracked<T>(command: impl FnOnce(&ProgressRange<'_>) -> T) -> T {
    tracked_lines(command).0
}

/// [`tracked`], with the `Progress` lines it wrote; `tracked_lines(|range| run(range)).1`.
pub fn tracked_lines<T>(command: impl FnOnce(&ProgressRange<'_>) -> T) -> (T, Vec<String>) {
    let lines = Arc::new(AtomicUsize::new(0));
    let names = Arc::new(Mutex::new(Vec::new()));
    let hooks = WrittenLines {
        lines: lines.clone(),
        last_position: Mutex::new(-1.0),
        names: names.clone(),
    };
    let indicator = ProgressIndicator::new(Box::new(hooks));
    let range = indicator.start();
    let answer = command(&range);
    drop(range);
    if lines.load(Ordering::SeqCst) == 0 {
        println!("Error: progress is not tracked");
    }
    let written = names.lock().unwrap().clone();
    (answer, written)
}

/// `restore [locate_data_file file] name` with the indicator on: the read reports through its range.
/// Example: `restore_tracked(s, "bug29481_ex1.brep", "s")`.
pub fn restore_tracked(s: &mut dyn Session, file: &str, name: &str) {
    let root = std::env::var_os("MARTENSITE_TEST_DATA_DIR").map_or_else(
        || {
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../external/martensite-test-data")
        },
        std::path::PathBuf::from,
    );
    let path = root.join("dataset-7.9.0/brep").join(file);
    let bytes = std::fs::read(&path).unwrap_or_else(|cause| panic!("{path:?}: {cause}"));
    let end = bytes.iter().position(u8::is_ascii_whitespace).unwrap_or(0);
    let mut stream = IoStream::from_bytes(bytes[end..].to_vec());
    let mut shape = Shape::default();
    tracked(|range| brep_tools::read(&mut shape, &mut stream, &BRepBuilder::default(), range))
        .unwrap_or_else(|failure| panic!("restore {file}: {failure:?}"));
    // What the command answers after its progress.
    println!("n");
    s.bind(name, &shape);
}
