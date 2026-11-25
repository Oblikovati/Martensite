use super::*;
use crate::surface_sketch::{WrappedRegionSelection, attachment::CylinderAttachmentRecord};
use martensite_tkbo::profile_extrusion::{
    DisplayTolerance, LoopUse, PlanarProofWorkLimit, ProfileCurve, ProfileCurveId, ProfileLoop,
    ProfilePlane, ProfileVertex, ProfileVertexId, Resolution, SketchProfile, ToleranceCap,
};
use std::time::Instant;

mod desktop;
mod pinned;

#[test]
#[ignore = "release timing and independent native search for the 239-curve reference"]
fn certified_cylinder_one_lift_owned_239_curve_text() {
    let stock = pinned::text_support();
    let mut sketch = pinned::text_request(&stock);
    check(
        &stock,
        &mut sketch,
        "owned-239",
        226802403. / 134217728.,
        444,
        CylinderFeatureKind::Boss,
    );
    check(
        &stock,
        &mut sketch,
        "owned-239",
        -226802403. / 134217728. * (79. / 81.),
        444,
        CylinderFeatureKind::Pocket,
    );
}

#[test]
#[ignore = "release timing and independent native search for the 265-curve desktop attachment"]
fn certified_cylinder_seam_crossing_desktop_265_curve_text() {
    let stock = desktop::stock();
    let mut sketch = desktop::request(&stock);
    let area = desktop::developed_area(&sketch);
    let radius = sketch.support.radius;
    let depth = sketch.depths[1];
    check(
        &stock,
        &mut sketch,
        "desktop-265",
        area * depth * (1. + depth / (2. * radius)),
        270,
        CylinderFeatureKind::Boss,
    );
    check(
        &stock,
        &mut sketch,
        "desktop-265",
        -area * depth * (1. - depth / (2. * radius)),
        270,
        CylinderFeatureKind::Pocket,
    );
}

fn check(
    stock: &CylinderFaceAttachment,
    sketch: &mut WrappedNormalRequest,
    workload: &str,
    expected: f64,
    sides: usize,
    kind: CylinderFeatureKind,
) {
    let normal = match kind {
        CylinderFeatureKind::Boss => NormalSense::Outward,
        CylinderFeatureKind::Pocket => NormalSense::Inward,
    };
    sketch.support = stock.extrusion_support(REVISION, 0., normal).unwrap();
    let original = stock.record().brep.clone();
    let feature = timed_issue(stock, sketch, workload, kind);
    assert_eq!(feature.attachment_report().side_faces, sides);
    assert!(feature.attachment_report().seam_residual_upper <= stock.record().resolution.sew);
    assert_eq!(stock.record().brep, original);
    assert_ordered(feature.validation());
    let native = feature.export_native().unwrap();
    let body = stock.export_native(REVISION).unwrap().body;
    assert!((volume(&native.body) - volume(&body) - expected).abs() < 1e-5);
    assert_history(sketch, feature.history(), sides);
    let start = Instant::now();
    let oracle = validate_body(&native.body, &stock.record().resolution).unwrap();
    eprintln!(
        "workload={workload} operation={kind:?} native_oracle_seconds={:.6}",
        start.elapsed().as_secs_f64()
    );
    assert!(oracle.is_valid());
    assert_eq!(oracle.census, feature.validation().census);
}

fn timed_issue(
    stock: &CylinderFaceAttachment,
    sketch: &WrappedNormalRequest,
    workload: &str,
    kind: CylinderFeatureKind,
) -> CertifiedCylinderFeatureResult {
    let attempt = Attempt {
        attachment: stock.record().id,
        revision: REVISION,
        kind,
    };
    let start = Instant::now();
    let mut previous = start;
    let feature = execute(stock, attempt, sketch, planar(), &mut || false, |stage| {
        let now = Instant::now();
        eprintln!(
            "workload={workload} operation={kind:?} stage={stage:?} stage_seconds={:.6}",
            (now - previous).as_secs_f64()
        );
        previous = now;
    })
    .unwrap();
    eprintln!(
        "workload={workload} operation={kind:?} certified_seconds={:.6}",
        start.elapsed().as_secs_f64()
    );
    feature
}

fn assert_history(sketch: &WrappedNormalRequest, history: &CylinderFeatureHistory, sides: usize) {
    let mut wall_count = 0;
    let mut far = std::collections::BTreeSet::new();
    for parents in history.images.values() {
        for parent in parents {
            let CylinderFeatureSource::Tool(key) = parent.source else {
                continue;
            };
            match (key.role, key.source) {
                (HistoryRole::SideFace, _) => wall_count += 1,
                (HistoryRole::LastDepthFace, WrappedEntitySource::Region(region)) => {
                    assert!(far.insert(region));
                }
                _ => (),
            }
        }
    }
    assert_eq!(wall_count, sides);
    assert_eq!(
        far,
        sketch
            .selections
            .iter()
            .map(|selection| selection.region)
            .collect()
    );
}
