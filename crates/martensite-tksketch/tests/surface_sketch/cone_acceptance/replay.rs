//! Cone replay is separate so the established cylinder signature stays unchanged.
use super::{construct, sector};
use martensite_tksketch::surface_sketch::NormalSense;

fn signature() -> String {
    let mut request = sector();
    request.support.chart_origin = -0.3;
    [NormalSense::Outward, NormalSense::Inward]
        .into_iter()
        .map(|normal| {
            request.support.normal = normal;
            let result = construct(&request, [-1., 2.]);
            format!(
                "{:?}|{:?}|{:?}|{:?}|{}",
                result.history(),
                result.volume(),
                result.volume_error_estimates(),
                result.validation(),
                crate::replay::geometry_bytes::signature(&result.export_brep().unwrap())
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn assert_parallel_matches_sequential() {
    let expected = signature();
    let results = std::thread::scope(|scope| {
        let tasks: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| (0..2).map(|_| signature()).collect::<Vec<_>>()))
            .collect();
        tasks
            .into_iter()
            .flat_map(|task| task.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert!(results.iter().all(|actual| actual == &expected));
}

#[test]
fn cone_child_signature() {
    if std::env::var_os("MARTENSITE_CONE_REPLAY_CHILD").is_some() {
        println!("CONE_REPLAY={:?}", signature());
    }
}

#[test]
fn independent_cone_processes_match_geometry_bindings_and_metadata() {
    let expected = format!("CONE_REPLAY={:?}", signature());
    for _ in 0..2 {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "cone_acceptance::replay::cone_child_signature",
                "--nocapture",
            ])
            .env("MARTENSITE_CONE_REPLAY_CHILD", "1")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(
            stdout.lines().find(|line| line.starts_with("CONE_REPLAY=")),
            Some(expected.as_str())
        );
    }
}
