mod bindings;
#[cfg(test)]
mod cpu;
mod geometry;
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomSurface;
type BoxError = Box<dyn std::error::Error>;
const TORUS_TODO: &str = "Faulty OCC604: Result of intersection operation is INcorrect !!!";
const TORUS_HEADING: &[&str] = &[
    "TODO OCC12345 ALL: Faulty OCC604: Result of intersection operation is INcorrect !!!",
    "========================",
    " OCC604",
    "========================",
    "",
];

fn nonshape_result_view(
    bindings: Option<&bindings::IntersectionBindings>,
    image: &'static str,
) -> Result<(), BoxError> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    // Shape lookup rejects raw geometry; never manufacture an edge to display.
    if bindings.is_some_and(|values| values.contains("result")) {
        println!("Error: object with name 'result' does not exist!");
    }
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    run.dump(image)?;
    run.finish()
}

/// Preserve actual located surfaces, the authored2×2 trim and non-shape display semantics.
/// Example: `trimmed_intersection_view()?`.
pub(crate) fn trimmed_intersection_view() -> Result<(), BoxError> {
    banner(&["============", "OCC22720", "============", ""]);
    let surfaces = geometry::trimmed_inputs()?;
    let bindings = geometry::actual_intersections(&surfaces)?;
    nonshape_result_view(Some(&bindings), "bug22720.png")
}

fn torus_intersection_view() -> Result<(), BoxError> {
    banner(TORUS_HEADING);
    let surfaces = geometry::torus_inputs()?;
    let result = geometry::actual_intersections(&surfaces);
    match &result {
        Ok(bindings) if bindings.authored_res_curve_count() == 1 => {
            println!("OCC604 OK : Result of intersection operation is CORRECT !!! ")
        }
        Ok(_) => println!("{TORUS_TODO}"),
        Err(_) => println!("Faulty OCC604: function INTERSECTION works with exception"),
    }
    nonshape_result_view(result.as_ref().ok(), "bug604.png")
}

/// Apply the actual lowalgos Faulty/common rules and authored TODO at CPU60.
/// Example: `run_torus()`.
pub(crate) fn run_torus() {
    let Some(report) =
        crate::case_opengl_process_case::observed_stream("bug604", 60, torus_intersection_view)
    else {
        return;
    };
    grade_torus_report(&report);
}

fn grade_torus_report(report: &str) {
    let failure =
        |line: &str| !crate::case_v3d_policy::accepted_report(&format!("{line}\nTEST COMPLETED"));
    let outcome = crate::case_todo::todo_outcome(report, &[TORUS_TODO], failure);
    assert_eq!(
        outcome,
        crate::case_todo::TodoOutcome::Todo,
        "lowalgos stream={report:?}; expected authoredTODO only"
    );
    assert!(
        report.lines().any(|line| line == "TEST COMPLETED"),
        "lowalgos stream={report:?}; expected completed body"
    );
}
