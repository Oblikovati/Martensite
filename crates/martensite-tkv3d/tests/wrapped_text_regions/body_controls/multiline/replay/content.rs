//! An actual content edit removes punctuation while keeping all other authored owners.
use super::*;
use martensite_tksketch::surface_sketch::attachment::CylinderAttachmentRecord;
use std::{
    io::{Read, Write as IoWrite},
    process::{Command, Stdio},
};

const EXCLAMATION: u64 = 15;
const SEAM_SHIFT: f64 = -0.3125;
const EDITED_PREFIX: &str = "MULTILINE_POCKET_CONTENT_REPLAY:";

#[test]
fn multiline_pocket_replays_punctuation_removal_across_native_seam() {
    let attachment = original_support();
    let original = publish(
        &attachment,
        FIRST,
        multiline_request(NormalSense::Inward),
        [0., 4.],
        0.25,
        663607031. / 402653184.,
    );
    let edited = without_exclamation(&attachment);
    check_content_edit(&original, &edited);
    let expected = signature(&edited);
    let actual = replay_saved_support(attachment.record());
    assert!(
        actual == expected,
        "edited pocket BRep or authored history changed on replay"
    );
}

#[test]
fn native_trim_band_admits_full_multiline_outline_and_seam_edit() {
    let saved = original_support();
    let mut original = multiline_request(NormalSense::Inward);
    let mut edited = edited_request();
    assert_first_outline_crosses_seam(&edited);
    check_native_band(&saved, &mut original, [0., 4.], 18);
    check_native_band(&saved, &mut edited, [SEAM_SHIFT, 4.], 16);
}

fn check_native_band(
    attachment: &CylinderFaceAttachment,
    request: &mut WrappedNormalRequest,
    shift: [f64; 2],
    expected: usize,
) {
    request.support = attachment
        .extrusion_support(FIRST, 0., NormalSense::Inward)
        .unwrap();
    place_developed(request, shift);
    let report = attachment.certify_footprint(FIRST, request).unwrap();
    assert_eq!(report.angular, None);
    assert_eq!(report.axial, [0., 8.]);
    assert_eq!(report.regions, expected);
}

fn without_exclamation(attachment: &CylinderFaceAttachment) -> Publication {
    let request = edited_request();
    assert_first_outline_crosses_seam(&request);
    // Remaining exact Green area 1265699/196608, times inward depth integral 79/320.
    publish(
        attachment,
        FIRST,
        request,
        [SEAM_SHIFT, 4.],
        0.25,
        99990221. / 62914560.,
    )
}

fn edited_request() -> WrappedNormalRequest {
    let mut request = multiline_request(NormalSense::Inward);
    request.profile.vertices.retain(|vertex| keep(vertex.id.0));
    request.profile.curves.retain(|curve| match curve {
        ProfileCurve::Line { id, .. } | ProfileCurve::Spline { id, .. } => keep(id.0),
        unexpected => panic!("expected fixture line or spline, found {unexpected:?}"),
    });
    request
        .selections
        .retain(|selected| keep(selected.region.0));
    assert_eq!(request.profile.curves.len(), 229);
    assert_eq!(request.selections.len(), 16);
    request
}

fn keep(authored: u64) -> bool {
    authored / 1000 != EXCLAMATION
}

fn assert_first_outline_crosses_seam(request: &WrappedNormalRequest) {
    let points = request
        .profile
        .vertices
        .iter()
        .filter(|vertex| vertex.id.0 / 1000 == 1);
    assert!(
        points
            .clone()
            .any(|vertex| vertex.point[0] + SEAM_SHIFT < 0.)
    );
    assert!(
        points
            .clone()
            .any(|vertex| vertex.point[0] + SEAM_SHIFT > 0.)
    );
}

fn check_content_edit(original: &Publication, edited: &Publication) {
    assert_ne!(edited.body, original.body);
    assert_ne!(edited.tool, original.tool);
    assert_eq!(native_snapshot(&original.shape), original.body);
    let retained = original
        .authored_keys
        .iter()
        .copied()
        .filter(keep_key)
        .collect::<Vec<_>>();
    assert_eq!(edited.authored_keys, retained);
}

fn keep_key(key: &WrappedHistoryKey) -> bool {
    match key.source {
        WrappedEntitySource::Region(region) => keep(region.0),
        WrappedEntitySource::Vertex(vertex) => keep(vertex.0),
        unexpected => panic!("expected authored region or vertex, found {unexpected:?}"),
    }
}

#[test]
#[ignore = "worker for saved multiline pocket support replay"]
fn process_worker() {
    let mut brep = Vec::new();
    std::io::stdin().lock().read_to_end(&mut brep).unwrap();
    let attachment = CylinderFaceAttachment::restore(CylinderAttachmentRecord {
        id: ID,
        revision: FIRST,
        resolution: attachment_resolution(),
        brep,
    })
    .unwrap();
    println!(
        "\n{EDITED_PREFIX}{}",
        signature(&without_exclamation(&attachment))
    );
}

fn replay_saved_support(record: &CylinderAttachmentRecord) -> String {
    assert_eq!(record.id, ID);
    assert_eq!(record.revision, FIRST);
    assert_eq!(record.resolution, attachment_resolution());
    let worker = concat!(module_path!(), "::process_worker")
        .split_once("::")
        .unwrap()
        .1;
    let mut process = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", worker, "--ignored", "--nocapture"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    process
        .stdin
        .take()
        .unwrap()
        .write_all(&record.brep)
        .unwrap();
    let output = process.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "worker: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .find_map(|line| line.strip_prefix(EDITED_PREFIX))
        .expect("one executed content replay worker")
        .to_owned()
}
