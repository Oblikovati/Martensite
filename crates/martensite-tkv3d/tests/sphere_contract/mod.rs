//! Test-owned snapshots are Send/Sync bytes; live presentation objects remain thread-confined.
mod snapshot;
mod style;
use martensite_tkv3d::ais::{
    colored_sphere as vdraw_sphere_object, print_colored_sphere_info as print_vdraw_sphere_info,
};
use std::{fs, path::PathBuf, process::Command};

#[derive(Clone, Copy)]
pub(super) struct SphereCase {
    pub resolution: i32,
    pub center: [f64; 3],
    pub radius: f64,
    pub edges: bool,
}

pub(super) struct SphereSnapshot {
    mesh_style: Vec<u8>,
    statistics: Vec<u8>,
}

pub(super) fn cases() -> Vec<SphereCase> {
    [-3, 0, 3, 4, 10, 100]
        .into_iter()
        .flat_map(|resolution| {
            [
                ([0.; 3], 100., false),
                ([1.25, -2.5, 3.75], 0.625, true),
                ([0.; 3], 1., true),
                ([-16., 8., -4.], 100., false),
            ]
            .map(move |(center, radius, edges)| SphereCase {
                resolution,
                center,
                radius,
                edges,
            })
        })
        .collect()
}

fn identity(case: SphereCase) -> String {
    let center = case.center.map(f64::to_bits);
    format!(
        "r{}_c{:x}-{:x}-{:x}_s{:x}_e{}",
        case.resolution,
        center[0],
        center[1],
        center[2],
        case.radius.to_bits(),
        u8::from(case.edges)
    )
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sphere-factory-linux-x86_64-v1")
}

pub(super) fn capture(case: SphereCase) -> SphereSnapshot {
    let root = std::env::temp_dir().join(format!(
        "sphere-snapshot-{}-{}",
        std::process::id(),
        identity(case)
    ));
    fs::create_dir(&root).unwrap();
    let output = capture_child(case, &root);
    let result = SphereSnapshot {
        mesh_style: fs::read(root.join("snapshot.bin")).unwrap(),
        statistics: statistics(&output.stdout),
    };
    fs::remove_dir_all(root).unwrap();
    result
}

fn capture_child(case: SphereCase, root: &std::path::Path) -> std::process::Output {
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "sphere_snapshot_worker", "--nocapture"])
        .env("SPHERE_SNAPSHOT_CASE", encode_case(case))
        .env("SPHERE_SNAPSHOT_FILE", root.join("snapshot.bin"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "CPU snapshot worker failed: {:?}",
        output
    );
    output
}

fn encode_case(case: SphereCase) -> String {
    format!(
        "{} {} {} {} {} {}",
        case.resolution,
        case.center[0],
        case.center[1],
        case.center[2],
        case.radius,
        u8::from(case.edges)
    )
}

fn decode_case(encoded: &str) -> SphereCase {
    let values: Vec<&str> = encoded.split_whitespace().collect();
    assert_eq!(
        values.len(),
        6,
        "case {encoded:?}; expected resolution x y z radius edges"
    );
    SphereCase {
        resolution: values[0].parse().unwrap(),
        center: [
            values[1].parse().unwrap(),
            values[2].parse().unwrap(),
            values[3].parse().unwrap(),
        ],
        radius: values[4].parse().unwrap(),
        edges: values[5] == "1",
    }
}

fn statistics(stdout: &[u8]) -> Vec<u8> {
    let text = std::str::from_utf8(stdout).unwrap();
    let (_, rest) = text.split_once("SPHERE_STATISTICS_BEGIN\n").unwrap();
    let (content, _) = rest.split_once("SPHERE_STATISTICS_END").unwrap();
    content.as_bytes().to_vec()
}

pub(super) fn worker() {
    let Ok(encoded) = std::env::var("SPHERE_SNAPSHOT_CASE") else {
        assert_eq!(cases().len(), 24);
        return;
    };
    let case = decode_case(&encoded);
    let object =
        vdraw_sphere_object(case.center, case.resolution, case.radius, case.edges).unwrap();
    fs::write(
        std::env::var_os("SPHERE_SNAPSHOT_FILE").unwrap(),
        snapshot::object_bytes(&object),
    )
    .unwrap();
    println!("SPHERE_STATISTICS_BEGIN");
    print_vdraw_sphere_info(&object).unwrap();
    println!("SPHERE_STATISTICS_END");
}

pub(super) fn compare_or_record(case: SphereCase, actual: &SphereSnapshot) {
    assert_bounds::<SphereCase>();
    assert_bounds::<SphereSnapshot>();
    let capture = std::env::var_os("SPHERE_BASELINE_CAPTURE").map(PathBuf::from);
    let root = capture.as_ref().cloned().unwrap_or_else(fixture_root);
    for (suffix, bytes) in [("bin", &actual.mesh_style), ("stdout", &actual.statistics)] {
        let path = root.join(format!("{}.{}", identity(case), suffix));
        compare_file(&path, bytes, capture.is_some());
    }
}

fn assert_bounds<T: Send + Sync>() {
    let witness: fn(T) = std::mem::drop::<T>;
    std::hint::black_box(witness);
}

fn compare_file(path: &std::path::Path, bytes: &[u8], capture: bool) {
    if capture {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .unwrap();
        std::io::Write::write_all(&mut file, bytes).unwrap();
        return;
    }
    assert_eq!(
        fs::read(path).unwrap(),
        bytes,
        "owned baseline changed: {}",
        path.display()
    );
}
