//! The heap held after a document with a large shape closes. The heap counter includes what the
//! allocator keeps in its thread cache, which fills unevenly over the first iterations and differs
//! from run to run, so this case runs in a process with that cache off.
use crate::session::commands::CafSession;
use crate::session::hooks::{BUGS, ImageCheck::None as NoPicture, run_script};
use crate::support::heap_trend::{checktrend, meminfo_heap};
use std::process::Command;

const IDENTITY: &str = "heap_cases::bug23489";
const TUNABLES: &str = "GLIBC_TUNABLES";
const NO_THREAD_CACHE: &str = "glibc.malloc.tcache_count=0";

fn repeated_document_close(s: &mut CafSession) {
    println!("pload OCAF ");
    let mut samples = Vec::new();
    for _ in 1..10 {
        s.restore("occt-source-data/occ", "Bottom.brep", "a");
        s.new_document(Some("BinOcaf"));
        s.set_shape("0:1", "a");
        s.close_document();
        s.vertex_at("a", [0., 0., 0.]);
        println!("meminfo h ");
        samples.push(meminfo_heap());
        checktrend(&samples, 0., 256., "Memory leak detected");
    }
}

#[test]
fn bug23489() {
    if std::env::var_os(TUNABLES).is_some_and(|value| value == NO_THREAD_CACHE) {
        run_script(
            IDENTITY,
            "tests/bugs/caf/bug23489",
            &BUGS,
            NoPicture,
            repeated_document_close,
        );
        return;
    }
    let executable = std::env::current_exe().unwrap_or_else(|cause| panic!("{cause}"));
    let output = Command::new(executable)
        .args(["--exact", IDENTITY, "--test-threads=1"])
        .env(TUNABLES, NO_THREAD_CACHE)
        .output()
        .unwrap_or_else(|cause| panic!("{IDENTITY}: {cause}"));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success() && stdout.contains("test result: ok. 1 passed"),
        "{IDENTITY} with the thread cache off:\n{stdout}"
    );
}
