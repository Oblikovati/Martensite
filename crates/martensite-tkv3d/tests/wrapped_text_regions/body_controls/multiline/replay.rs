//! Recompute wrapped text against a saved selected face and replay complete features.
use super::*;
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorSurface,
    brep_tools::History,
    topexp::Explorer,
    topods::{self, Face, Shape},
};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tksketch::surface_sketch::attachment::{
    AttachmentFailureClass, CylinderFaceAttachment, FaceAttachmentId, SupportRevision,
};
use martensite_tksketch::surface_sketch::{
    CylinderFeatureKind, CylinderFeatureResult, CylinderFeatureSource,
};
use std::fmt::Write;
#[path = "replay/content.rs"]
mod content;

const FIRST: SupportRevision = SupportRevision(41);
const EDITED: SupportRevision = SupportRevision(42);
const ID: FaceAttachmentId = FaceAttachmentId(17);
const PREFIX: &str = "MULTILINE_BODY_REPLAY:";

#[test]
fn multiline_boss_replays_after_support_placement_and_depth_edits() {
    let first = original_support();
    let original = publish(
        &first,
        FIRST,
        multiline_request(NormalSense::Outward),
        [0., 4.],
        0.25,
        226802403. / 134217728.,
    );
    let revised = revise_support(&first);
    let edited = edited_publication(&revised);
    check_edited_publication(&first, &original, &edited);
    assert!(
        signature(&edited) == process_signature(),
        "edited BRep or authored history changed on replay"
    );
}

fn check_edited_publication(
    first: &CylinderFaceAttachment,
    original: &Publication,
    edited: &Publication,
) {
    assert_ne!(original.body, edited.body);
    assert_ne!(original.tool, edited.tool);
    assert_eq!(native_snapshot(&original.shape), original.body);
    assert_eq!(first.record().revision, FIRST);
    assert_eq!(original.authored_keys, edited.authored_keys);
}

#[test]
#[ignore = "worker for multiline boss process replay"]
fn process_worker() {
    let revised = revise_support(&original_support());
    println!("\n{PREFIX}{}", signature(&edited_publication(&revised)));
}

fn original_support() -> CylinderFaceAttachment {
    let body = multiline_support(10.);
    CylinderFaceAttachment::capture(
        ID,
        FIRST,
        &body,
        &selected_cylinder(&body),
        attachment_resolution(),
    )
    .unwrap()
}

fn revise_support(original: &CylinderFaceAttachment) -> CylinderFaceAttachment {
    let bytes = original.record().brep.clone();
    let current = original.export_native(FIRST).unwrap();
    let revised = multiline_support(12.);
    let history = History::new();
    history
        .add_modified(&current.face, &selected_cylinder(&revised))
        .unwrap();
    let changed = original
        .reattach(&current, &revised, EDITED, &history)
        .unwrap();
    assert_eq!(changed.record().id, ID);
    assert_eq!(original.record().brep, bytes);
    let stale = changed
        .extrusion_support(FIRST, 0., NormalSense::Outward)
        .unwrap_err();
    assert_eq!(stale.class(), AttachmentFailureClass::StaleRevision);
    CylinderFaceAttachment::restore(changed.record().clone()).unwrap()
}

fn selected_cylinder(body: &Shape) -> Face {
    let faces = Explorer::new(body, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let mut matches = faces.filter_map(|shape| {
        let face = topods::face(&shape).unwrap();
        let surface = BRepAdaptorSurface::with_face(&face, false).unwrap();
        (surface.get_type().unwrap() == SurfaceType::Cylinder).then_some(face)
    });
    let face = matches
        .next()
        .expect("fixture has one selected cylindrical face");
    assert!(matches.next().is_none());
    face
}

struct Publication {
    shape: Shape,
    body: Vec<u8>,
    archive: Vec<u8>,
    tool: Vec<u8>,
    authored_keys: Vec<WrappedHistoryKey>,
    history: String,
}

fn edited_publication(attachment: &CylinderFaceAttachment) -> Publication {
    // Exact Green area times the radius-12 radial integral at depth 3/8.
    let expected = (42000445. / 6291456.) * (195. / 512.);
    publish(
        attachment,
        EDITED,
        multiline_request(NormalSense::Outward),
        [0.5, 4.125],
        0.375,
        expected,
    )
}

fn publish(
    attachment: &CylinderFaceAttachment,
    revision: SupportRevision,
    mut request: WrappedNormalRequest,
    placement: [f64; 2],
    depth: f64,
    expected: f64,
) -> Publication {
    let normal = request.support.normal;
    request.support = attachment.extrusion_support(revision, 0., normal).unwrap();
    request.depths = [0., depth];
    place_developed(&mut request, placement);
    let source = attachment.export_native(revision).unwrap();
    let original = request.clone();
    let kind = match normal {
        NormalSense::Outward => CylinderFeatureKind::Boss,
        NormalSense::Inward => CylinderFeatureKind::Pocket,
    };
    let result = attachment
        .apply_wrapped_profile(revision, kind, &request)
        .unwrap();
    let native = result.export_native().unwrap();
    check_publication(&result, &request, expected);
    check_attachment_volume_direction(&native.body, &source.body, normal);
    assert_eq!(request, original);
    publication(native.body, &result)
}

fn place_developed(request: &mut WrappedNormalRequest, shift: [f64; 2]) {
    // Native axial coordinates start four units below the fixture's world layout.
    assert_eq!(request.support.origin, [0., 0., -4.]);
    for vertex in &mut request.profile.vertices {
        translate(&mut vertex.point, shift);
    }
    for curve in &mut request.profile.curves {
        if let ProfileCurve::Spline { poles, .. } = curve {
            poles.iter_mut().for_each(|point| translate(point, shift));
        }
    }
}
fn translate(point: &mut [f64; 2], [s, z]: [f64; 2]) {
    point[0] += s;
    point[1] += z;
}

fn check_publication(
    result: &CylinderFeatureResult,
    request: &WrappedNormalRequest,
    expected: f64,
) {
    let tool = result.tool().extrusion();
    assert_eq!(tool.reports().len(), request.selections.len());
    check_member_volume_oracle(tool.reports(), expected);
    assert!(result.validation().is_valid());
    assert_eq!(result.validation().census.len(), 1);
    check_depth_faces(result, request);
}

fn check_depth_faces(result: &CylinderFeatureResult, request: &WrappedNormalRequest) {
    let mut surviving = std::collections::BTreeSet::new();
    for (image, parents) in &result.history().images {
        if image.kind == ShapeEnum::Face {
            surviving.extend(parents.iter().map(|parent| parent.source));
        }
    }
    for selection in &request.selections {
        let key = WrappedHistoryKey {
            source: WrappedEntitySource::Region(selection.region),
            role: SurfaceHistoryRole::LastDepthFace,
        };
        assert!(surviving.contains(&CylinderFeatureSource::Tool(key)));
    }
}

fn publication(shape: Shape, result: &CylinderFeatureResult) -> Publication {
    let extrusion = result.tool().extrusion();
    let mut authored_keys: Vec<_> = extrusion
        .history()
        .iter()
        .map(|record| record.result)
        .filter(|key| {
            matches!(
                key.source,
                WrappedEntitySource::Region(_) | WrappedEntitySource::Vertex(_)
            )
        })
        .collect();
    authored_keys.sort_unstable();
    Publication {
        body: native_snapshot(&shape),
        shape,
        archive: result.encoded_brep().to_vec(),
        tool: extrusion.encoded_brep().to_vec(),
        authored_keys,
        history: format!("{:?} {:?}", result.history(), extrusion.history()),
    }
}

fn signature(publication: &Publication) -> String {
    let mut signature = publication.history.clone();
    for bytes in [&publication.archive, &publication.tool] {
        signature.push('|');
        for byte in bytes {
            write!(signature, "{byte:02x}").unwrap();
        }
    }
    signature
}

fn process_signature() -> String {
    let worker = concat!(module_path!(), "::process_worker")
        .split_once("::")
        .unwrap()
        .1;
    let run = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", worker, "--ignored", "--nocapture"])
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "worker: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    let stdout = String::from_utf8(run.stdout).unwrap();
    stdout
        .lines()
        .find_map(|line| line.strip_prefix(PREFIX))
        .expect("one executed replay worker")
        .to_owned()
}
