//! Only this source case's two mandatory known diagnostics may survive whole-stream grading.
const TODOS: [&str; 2] = [
    "TODO OCC23343 ALL: An exception was caught",
    "TODO OCC23343 ALL: Faulty OCC23343",
];
const CHECKED: &str = "An exception was caught Standard_NullObject: GeomPlate: the plate surface is null; expected a constructed object";
const FAULTY: &str =
    "Faulty OCC23343: there is crash (ACCESS VIOLATION) in GeomPlate_BuildPlateSurface";
const VIEW: &str = "SOURCE23343_VIEW_SAVED";
const OUTCOME: &str = "{\"runtime_outcome\":\"KnownProblemReproduced\",\"source_policy_status\":\"BAD (known problem)\",\"case_assertions_all_passed\":false,\"native_failure\":\"OSD_SIGSEGV\",\"rust_failure\":\"Standard_NullObject\"}";

const REPORT_STEPS: [&str; 7] = [
    TODOS[0],
    TODOS[1],
    CHECKED,
    FAULTY,
    VIEW,
    "TEST COMPLETED",
    OUTCOME,
];

/// Print the original mandatory declarations before construction; `start_source()`.
pub fn start_source() {
    for declaration in TODOS {
        println!("{declaration}");
    }
    println!(
        "============\n0023343: Crash (A C C E S S  V I O L A T I O N) in GeomPlate_BuildPlateSurface / MakeApprox if init Surface is set\n============\n"
    );
}

/// Emit the checked command failure without fabricating a native signal; `caught_failure(&error)`.
pub fn caught_failure(error: &martensite_tkernel::standard::Failure) {
    let description = format!(
        "An exception was caught {}: {}",
        error.exception_type(),
        error.message()
    );
    println!("{description}");
    println!("{FAULTY}");
    // The outcome separately discloses native signal versus checked Rust failure.
    assert_eq!(description, CHECKED);
}

/// Grade only after the actual saved view and inherited end; `finish_source()`.
pub fn finish_source() {
    println!("{VIEW}");
    println!("TEST COMPLETED");
    println!("{OUTCOME}");
}

/// Match the fixed ordered native-known failure vector, never arbitrary caller failures.
/// Example: `assert!(!known_problem_report("TEST COMPLETED\n"));`.
pub fn known_problem_report(reports: &str) -> bool {
    let observed: Vec<&str> = reports
        .lines()
        .filter(|line| REPORT_STEPS.contains(line))
        .collect();
    if observed != REPORT_STEPS {
        return false;
    }
    let other = reports
        .lines()
        .filter(|line| !REPORT_STEPS.contains(line))
        .collect::<Vec<_>>()
        .join("\n");
    crate::case_bugs_bop_rules::accepted_bugs_bop_report(&other)
}
