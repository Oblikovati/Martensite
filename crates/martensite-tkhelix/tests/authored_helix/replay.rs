// SPDX-License-Identifier: GPL-2.0-only
use super::*;

fn replay_fit() -> FittedHelix {
    let radius = HelixRadius::Cone {
        opening: 4.,
        closing: 3.,
    };
    fitted(ConstantHelixRequest {
        lead: HelixLead::PitchAndHeight {
            pitch: 3.,
            height: 0.5,
        },
        ..request(radius, HelixRotation::Negative)
    })
}

fn retained_archive(fit: &FittedHelix) -> Vec<u8> {
    use martensite_tkbrep::bintools::{self, WriteOptions};
    use martensite_tkernel::{message::ProgressRange, storage::IoStream};
    let retained = fit.export_brep(|| false).unwrap();
    let mut stream = IoStream::from_bytes(Vec::new());
    let options = WriteOptions {
        with_triangles: false,
        with_normals: false,
        ..WriteOptions::default()
    };
    bintools::write(
        &retained.wire,
        &mut stream,
        options,
        &ProgressRange::default(),
    )
    .unwrap();
    let mut bytes = vec![0; usize::try_from(stream.tellp()).unwrap()];
    stream.seekg(0).read(&mut bytes);
    assert!(!stream.fail());
    bytes
}

#[test]
fn concurrent_fits_and_detached_exports_repeat_raw_native_bytes() {
    fn assert_owned<T: Send + Sync>() {}
    assert_owned::<ConstantHelix>();
    assert_owned::<FittedHelix>();
    assert_owned::<DetachedHelix>();
    assert_owned::<HelixFitReceipt>();
    assert_owned::<HelixFailure>();

    let fit = replay_fit();
    let expected = retained_archive(&fit);
    std::thread::scope(|scope| {
        let workers = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    for _ in 0..4 {
                        assert_eq!(retained_archive(&fit), expected);
                    }
                    let independent = replay_fit();
                    assert_eq!(independent.receipt(), fit.receipt());
                    assert_eq!(retained_archive(&independent), expected);
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

#[test]
fn replay_retained_bytes_in_separate_processes() {
    const MARKER: &str = "AUTHORED_HELIX_BREP=";
    let argv = [
        "--exact",
        "authored_helix::replay::replay_retained_bytes_in_separate_processes",
        "--nocapture",
    ];
    if std::env::var_os(MARKER.trim_end_matches('=')).is_some() {
        let fit = replay_fit();
        let bytes = retained_archive(&fit);
        let encoded = bytes
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        println!("{MARKER}{encoded}");
        return;
    }
    let executable = std::env::current_exe().unwrap();
    let run = || {
        let output = std::process::Command::new(&executable)
            .args(argv)
            .env(MARKER.trim_end_matches('='), "1")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        let encoded = stdout.split_once(MARKER).unwrap().1.lines().next().unwrap();
        assert!(encoded.len() > 100);
        encoded.to_owned()
    };
    assert_eq!(run(), run());
}
