// SPDX-License-Identifier: GPL-2.0-only
use super::{PARAMETERS, fit, fit_id, pid, point, recipe, stations};
use martensite_tksketch3d::{Edit, FitSplineGeometry, SketchFitSpline, Snapshot};
use std::{process::Command, sync::Barrier, thread};

const RECEIPT: &str = "{\"fit_spline_replay\":";

#[test]
fn independent_processes_repeat_interpolation_and_derivative_bits() {
    let first = fit_receipt();
    let second = fit_receipt();
    assert_eq!(first, second);
}

fn fit_receipt() -> String {
    let result = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "fit_splines::replay::emit_fit_receipt",
            "--nocapture",
            "--test-threads=1",
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let text = String::from_utf8(result.stdout).unwrap();
    text.split_once(RECEIPT)
        .unwrap()
        .1
        .lines()
        .next()
        .unwrap()
        .to_owned()
}

#[test]
fn emit_fit_receipt() {
    let source = fit(PARAMETERS.map(stations), recipe()).unwrap();
    let curve = source.fit_spline(fit_id()).unwrap();
    let mut receipt = vec![curve.recipe().id.get().get()];
    receipt.extend(curve.recipe().parameters.values().map(f64::to_bits));
    receipt.extend(
        curve
            .poles()
            .into_iter()
            .flat_map(|point| point.coordinates().map(f64::to_bits)),
    );
    receipt.extend(curve.station_miss_upper().map(f64::to_bits));
    receipt.push(curve.pole_rounding_upper().to_bits());
    receipt.push(curve.distortion().ratio_upper.to_bits());
    receipt.extend(
        curve
            .station_weights(0.5)
            .unwrap()
            .into_iter()
            .flatten()
            .map(f64::to_bits),
    );
    println!("{RECEIPT}{receipt:?}}}");
}

#[test]
fn accepted_fit_rebuilds_share_the_fixed_operator_across_threads() {
    fn send_sync<T: Send + Sync>() {}
    fn copy<T: Copy>() {}
    send_sync::<Snapshot>();
    send_sync::<FitSplineGeometry>();
    copy::<Edit>();
    copy::<SketchFitSpline>();
    let source = fit(PARAMETERS.map(stations), recipe()).unwrap();
    let edit = Edit::MovePoint {
        point: pid(2),
        position: point([0.75, 0.25, 0.015625]),
    };
    let expected = source.apply(&[edit], || false).unwrap();
    let start = Barrier::new(4);
    thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                start.wait();
                for _ in 0..16 {
                    let rebuilt = source.apply(&[edit], || false).unwrap();
                    assert_eq!(expected, rebuilt);
                    let a = expected.fit_spline(fit_id()).unwrap();
                    let b = rebuilt.fit_spline(fit_id()).unwrap();
                    assert_eq!(
                        a.station_weights(0.5).unwrap(),
                        b.station_weights(0.5).unwrap()
                    );
                    assert_eq!(a.jet(0.5).unwrap(), b.jet(0.5).unwrap());
                }
            });
        }
    });
}
