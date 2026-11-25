//! Benchmarks for the public issue call. The independent native oracle runs in the text controls.
use super::*;

const SAMPLES: usize = 20;

#[test]
#[ignore = "local release-profile p95 and peak RSS for each normal on owned text"]
fn public_issue_latency_owned_239() {
    let stock = pinned::text_support();
    measure(&stock, pinned::text_request(&stock), "owned-239");
}

#[test]
#[ignore = "local release-profile p95 and peak RSS for each normal on captured desktop text"]
fn public_issue_latency_desktop_265() {
    let stock = desktop::stock();
    measure(&stock, desktop::request(&stock), "desktop-265");
}

fn measure(stock: &CylinderFaceAttachment, mut sketch: WrappedNormalRequest, workload: &str) {
    for (kind, normal) in [
        (CylinderFeatureKind::Boss, NormalSense::Outward),
        (CylinderFeatureKind::Pocket, NormalSense::Inward),
    ] {
        sketch.support = stock.extrusion_support(REVISION, 0., normal).unwrap();
        let mut samples = [std::time::Duration::ZERO; SAMPLES];
        let mut baseline: Option<CertifiedCylinderFeatureResult> = None;
        for sample in &mut samples {
            let start = Instant::now();
            let result = stock
                .apply_certified_wrapped_profile(REVISION, kind, &sketch, planar())
                .unwrap();
            *sample = start.elapsed();
            assert_ordered(result.validation());
            if let Some(expected) = &baseline {
                assert!(
                    result.encoded_brep() == expected.encoded_brep(),
                    "repeated cylinder archive changed"
                );
            } else {
                baseline = Some(result);
            }
        }
        check_permuted(stock, sketch.clone(), kind, baseline.as_ref().unwrap());
        samples.sort_unstable();
        let index = (SAMPLES * 95).div_ceil(100) - 1;
        eprintln!(
            "workload={workload} operation={kind:?} samples={SAMPLES} min_ms={:.3} p95_ms={:.3} max_ms={:.3}",
            samples[0].as_secs_f64() * 1000.,
            samples[index].as_secs_f64() * 1000.,
            samples[SAMPLES - 1].as_secs_f64() * 1000.
        );
    }
}

fn check_permuted(
    stock: &CylinderFaceAttachment,
    mut recipe: WrappedNormalRequest,
    kind: CylinderFeatureKind,
    initial: &CertifiedCylinderFeatureResult,
) {
    recipe.profile.curves.reverse();
    recipe.profile.vertices.reverse();
    recipe.selections.reverse();
    let replay = stock
        .apply_certified_wrapped_profile(REVISION, kind, &recipe, planar())
        .unwrap();
    assert!(
        replay.encoded_brep() == initial.encoded_brep(),
        "permuted authored cylinder archive changed"
    );
    assert_eq!(replay.history(), initial.history());
    assert_eq!(replay.authored_history(), initial.authored_history());
    assert_eq!(replay.validation(), initial.validation());
    assert_eq!(replay.attachment_report(), initial.attachment_report());
}
