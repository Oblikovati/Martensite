//! Runs each case in its own child process and keeps what it printed for inspection.
use martensite_process_policy::case_runner::{CaseReceipt, CaseRunner, CaseSpec, FactorProfile};
use martensite_tkernel::message::{
    Gravity, Message, Printer, PrinterBase, PrinterHandle, PrinterType,
};
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped};
use martensite_tkernel::tcollection::AsciiString;
use std::cell::Cell;
use std::path::PathBuf;
use std::sync::Arc;

/// The folder of kept images and streams, when a run asks for them.
pub const KEEP_ENV: &str = "MARTENSITE_GEOMETRY_ARTIFACT_DIR";

/// Writes messages to the case's stream, in order with what the script prints.
struct CasePrinter {
    base: PrinterBase,
}

impl Printer for CasePrinter {
    fn printer_base(&self) -> &PrinterBase {
        &self.base
    }
    fn dynamic_type(&self) -> &'static RuntimeType {
        PrinterType::static_type()
    }
    fn send(&self, text: &AsciiString, _gravity: Gravity) {
        println!("{}", String::from_utf8_lossy(text.as_bytes()));
    }
}

fn print_messages_to_the_stream() {
    let messenger = Message::default_messenger();
    messenger.printers_mut().clear();
    let printer = Arc::new(CasePrinter {
        base: PrinterBase::default(),
    });
    messenger.add_printer(printer as PrinterHandle);
}

/// Runs a script under the limit of its grids; the parent checks the script finished.
/// Example: `run_case("de_mesh_obj_read::prism", "tests/de_mesh/obj_read/prism", prism)`.
pub fn run_case(identity: &str, source: &str, script: fn()) {
    let spec = CaseSpec::new(identity, 1000, FactorProfile::RequireUnset);
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("obj-case-receipts");
    let executed = Cell::new(false);
    let receipt = CaseRunner::new(&root, None)
        .run(spec, || {
            executed.set(true);
            print_messages_to_the_stream();
            println!("Executing {source}...");
            println!("CPU and elapsed time limit set to 1000 seconds");
            script();
            println!("TEST COMPLETED");
            Ok(())
        })
        .unwrap_or_else(|failure| panic!("{identity}: {failure}"));
    if executed.get() {
        return;
    }
    let stream = stream_of(&receipt);
    assert!(
        stream.contains("TEST COMPLETED"),
        "{identity}: unfinished stream {stream}"
    );
    assert!(
        !has_bad_line(&stream),
        "{identity}: a line reports an error:\n{stream}"
    );
    keep_stream(identity, &stream);
    std::fs::remove_dir_all(&receipt.directory).unwrap();
}

fn has_bad_line(stream: &str) -> bool {
    stream.lines().any(|line| {
        let lower = line.to_ascii_lowercase();
        lower.starts_with("error") || lower.contains("faulty") || lower.contains("exception")
    })
}

fn stream_of(receipt: &CaseReceipt) -> String {
    let read =
        |file: &str| std::fs::read_to_string(receipt.directory.join(file)).unwrap_or_default();
    let stdout = read("stdout.log");
    let body = stdout
        .split_once(" stdout ----\n")
        .map_or(stdout.as_str(), |parts| parts.1);
    let body = body.split("\n\nsuccesses:\n").next().unwrap_or(body);
    format!("{body}\n{}", read("stderr.log"))
}

fn keep_stream(identity: &str, stream: &str) {
    let Some(folder) = std::env::var_os(KEEP_ENV) else {
        return;
    };
    std::fs::create_dir_all(&folder).unwrap();
    let file = identity.replace("::", "_") + ".stream.txt";
    std::fs::write(PathBuf::from(folder).join(file), stream).unwrap();
}

/// One `#[test]` per script, in a module named after its group and grid.
#[macro_export]
macro_rules! obj_case {
    ($name:ident, $source:literal, $script:expr) => {
        #[test]
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            let module = module.rsplit("::").next().unwrap_or(module);
            let identity = format!("cases::{module}::{}", stringify!($name));
            $crate::process::run_case(&identity, $source, $script);
        }
    };
}
