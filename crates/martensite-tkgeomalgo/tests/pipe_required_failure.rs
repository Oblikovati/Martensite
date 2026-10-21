//! Complete original bugs/modalg_7/bug31361.
//! The original REQUIRED directive expects the actual failed construction message.
#[path = "common/isolated_case.rs"]
mod isolated_case;
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCircle, GeomCurve};
use martensite_tkgeomalgo::geom_fill::GeomFillPipe;
#[path = "support/curve_segment.rs"]
mod curve_segment;
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Axis2, Direction3, Point3},
};

fn original_spine() -> Handle<dyn GeomCurve> {
    let poles = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(100.0, 0.0, 0.0),
        Point3::new(100.0, 100.0, 0.0),
        Point3::new(0.0, 100.0, 0.0),
    ];
    Handle::new(GeomBezierCurve::new(&Array1::from_shared(&poles, 1)).unwrap()).cast()
}

fn circular_section(center: Point3) -> Handle<dyn GeomCurve> {
    let frame = Axis2::new(center, Direction3::Z).unwrap();
    Handle::new(GeomCircle::new(frame, 10.0).unwrap()).cast()
}

fn perform_original_tuyau(pipe: &mut GeomFillPipe) -> String {
    pipe.perform(1.0e-4, false, Shape::C2, 11, 30).unwrap();
    if !pipe.is_done() {
        return "Error: GeomFill_Pipe cannot make a surface\n".to_owned();
    }
    let accuracy = pipe.error_on_surf();
    let _published_surface = pipe.surface();
    format!("Accuracy of approximation = {accuracy}\n")
}

#[test]
fn bug31361_original_evolving_pipe_requires_failure_message() {
    run_pipe_original(
        "pipe_required_failure::bug31361_original_evolving_pipe_requires_failure_message",
        bug31361_original_evolving_pipe_requires_failure_message_body,
    );
}

fn bug31361_original_evolving_pipe_requires_failure_message_body() {
    let source_first_section = circular_section(Point3::default());
    let source_last_section = circular_section(Point3::new(0.0, 100.0, 0.0));
    let source_spine = original_spine();
    let mut pipe = GeomFillPipe::new();
    pipe.set_generate_particular_case(true);
    pipe.init_two_sections(&source_spine, &source_first_section, &source_last_section)
        .unwrap();
    let output = perform_original_tuyau(&mut pipe);
    // The log rules match REQUIRED messages before the general Error rule.
    assert!(
        output.contains("Error: GeomFill_Pipe cannot make a surface"),
        "original REQUIRED failure message is missing from actual command output: {output}"
    );
}

#[test]
fn pipe_command_success_branch_publishes_regular_straight_sweep() {
    let origin = Point3::default();
    let end = Point3::new(0.0, 0.0, 5.0);
    let path = curve_segment::extrema_segment(origin, end);
    let mut pipe = GeomFillPipe::new();
    pipe.set_generate_particular_case(true);
    pipe.init_two_sections(&path, &circular_section(origin), &circular_section(end))
        .unwrap();
    let output = perform_original_tuyau(&mut pipe);
    assert!(
        output.starts_with("Accuracy of approximation = "),
        "actual success output: {output}"
    );
    assert!(pipe.is_done());
    assert!(!pipe.surface().is_null());
}

// bugs/begin installs CPU/elapsed1000 before even the independent section fixtures.
fn run_pipe_original(identity: &str, body: fn()) {
    isolated_case::run_isolated(identity, 1000, || {
        body();
        println!("TEST COMPLETED");
    });
}
