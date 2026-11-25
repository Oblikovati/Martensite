//! Compare the complete frozen archive, history and validation across independent processes.
use super::*;
use std::{fmt::Write, process::Command};

const PREFIX: &str = "CYLINDER_ATTACHMENT_REPLAY=";
const WORKER: &str = "surface_sketch::wrapped_profile::api::footprint::feature::cylinder_certified::tests::replay::replay_worker";

#[test]
fn independent_processes_replay_cylinder_body_history_and_certificates() {
    let expected = signature();
    for _ in 0..2 {
        let output = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", WORKER, "--ignored", "--nocapture"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "cylinder replay worker failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        let actual = stdout
            .lines()
            .find_map(|line| line.strip_prefix(PREFIX))
            .unwrap();
        assert!(
            actual == expected,
            "complete independent cylinder archive or certificate differs"
        );
    }
}

#[test]
#[ignore = "invoked by the independent process replay control"]
fn replay_worker() {
    println!("{PREFIX}{}", signature());
}

fn signature() -> String {
    let stock = indirect_extruded_circle();
    let mut sketch = request(&stock, [-0.25, 0.25], [1., 2.]);
    let mut signature = String::new();
    append_bytes(&mut signature, &stock.record().brep);
    for (kind, normal) in [
        (CylinderFeatureKind::Boss, NormalSense::Outward),
        (CylinderFeatureKind::Pocket, NormalSense::Inward),
    ] {
        sketch.support.normal = normal;
        let result = stock
            .apply_certified_wrapped_profile(REVISION, kind, &sketch, planar())
            .unwrap();
        append(&mut signature, &result);
    }
    signature
}

fn append(signature: &mut String, result: &CertifiedCylinderFeatureResult) {
    write!(
        signature,
        "{:?}|{:?}|{:?}|{:?}|",
        result.history(),
        result.authored_history(),
        result.validation(),
        result.attachment_report()
    )
    .unwrap();
    append_bytes(signature, result.encoded_brep());
}

fn append_bytes(signature: &mut String, bytes: &[u8]) {
    signature.reserve(bytes.len() * 2);
    for byte in bytes {
        write!(signature, "{byte:02x}").unwrap();
    }
}
