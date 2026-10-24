use super::{fixed, fixtures, support::property_bits};
use martensite_tkbrep::topods::Shape;
use martensite_tktopalgo::brep_gprop::{
    BRepGProp, SurfaceErrorEvidence, SurfacePropertyOptions, SurfacePropertyResult,
    SurfaceRepresentation,
};
use std::process::Command;
const WORKERS: &str = "MARTENSITE_SURFACE_POLICY_TEST_WORKERS";

fn policies() -> [SurfacePropertyOptions; 4] {
    let mesh = SurfaceRepresentation::PreferExistingMesh;
    [
        fixed(SurfaceRepresentation::PreferGeometry),
        fixed(mesh),
        SurfacePropertyOptions::default(),
        SurfacePropertyOptions {
            representation: mesh,
            ..Default::default()
        },
    ]
}

fn reports(shape: &Shape, workers: usize) -> Vec<SurfacePropertyResult> {
    if workers == 1 {
        return policies()
            .into_iter()
            .map(|options| BRepGProp::surface_properties_with_options(shape, options).unwrap())
            .collect();
    }
    std::thread::scope(|scope| {
        let tasks: Vec<_> = policies()
            .into_iter()
            .map(|options| {
                scope.spawn(move || {
                    BRepGProp::surface_properties_with_options(shape, options).unwrap()
                })
            })
            .collect();
        tasks.into_iter().map(|task| task.join().unwrap()).collect()
    })
}

fn emit_replay(workers: usize) {
    let (mesh, _) = fixtures::meshed_cylinder();
    let shape = fixtures::compound(&[fixtures::elliptic_prism_faces(), mesh.into()]);
    for (index, report) in reports(&shape, workers).iter().enumerate() {
        let estimate = match report.error_evidence() {
            SurfaceErrorEvidence::Unknown => None,
            SurfaceErrorEvidence::EstimatedRelative {
                max_face_relative_error,
            } => Some(max_face_relative_error.to_bits()),
        };
        println!(
            "SURFACE_POLICY {index} {:?} {:?} {} {} {:?}",
            property_bits(report.properties()),
            estimate,
            report.geometric_faces(),
            report.triangulated_faces(),
            report.effective_relative_target()
        );
    }
}

fn child_bytes(test_name: &str, workers: usize) -> Vec<u8> {
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", test_name, "--nocapture", "--test-threads=1"])
        .env(WORKERS, workers.to_string())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "workers={workers}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    let rows: Vec<_> = text
        .lines()
        .filter_map(|line| line.find("SURFACE_POLICY ").map(|start| &line[start..]))
        .collect();
    assert_eq!(rows.len(), 4);
    rows.join("\n").into_bytes()
}

#[test]
fn properties_and_evidence_replay_byte_for_byte_across_processes_and_workers() {
    if let Ok(workers) = std::env::var(WORKERS) {
        emit_replay(workers.parse().unwrap());
        return;
    }
    let thread = std::thread::current();
    let name = thread.name().unwrap();
    let first = child_bytes(name, 1);
    assert_eq!(first, child_bytes(name, 1));
    assert_eq!(first, child_bytes(name, 4));
}
