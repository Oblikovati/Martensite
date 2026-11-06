//! Two actual revolutions retain their authored process CPU counters and inherited final image.
use super::*;

#[test]
fn bug25264() {
    case_perf_process_case::run_perf_case(
        "v3d_revolution_perf_regressions::bug25264",
        mesh_view_cases::revolved_meshes,
    );
}

#[test]
fn revolution_perf_cpu_models_both_source_axes_and_real_meshes() {
    let wire = dataset_shape::read_dataset_shape("bug25264_wire.brep").unwrap();
    for (y, z) in [
        (14039.0208237378, 28443.2585934755),
        (14039.0208217527, 28443.25860033352),
    ] {
        let shape = mesh_view_cases::perf::revolved(&wire, y, z).unwrap();
        assert!(shape_checks::check_shape_valid(&shape).unwrap());
        mesh_view_cases::perf::timed_revolution_mesh(&shape, "CPU_control").unwrap();
        let summary = shape_mesh::TriangulationSummary::of(&shape).unwrap();
        assert!(summary.faces > 0 && summary.nodes > 0 && summary.triangles > 0);
    }
}
