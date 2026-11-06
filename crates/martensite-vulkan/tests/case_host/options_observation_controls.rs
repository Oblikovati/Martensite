use super::*;
#[derive(Default)]
struct NamedUnavailableWriter {
    pieces: Vec<(String, Vec<u8>)>,
    stale_ties: bool,
}
impl DumpWitnessWriter for NamedUnavailableWriter {
    fn write_piece(&mut self, name: &str, bytes: &[u8]) -> io::Result<()> {
        if self.pieces.iter().any(|(existing, _)| existing == name) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("fake piece={name}; expected exclusive new output"),
            ));
        }
        self.pieces.push((name.to_owned(), bytes.to_vec()));
        Ok(())
    }
}

#[test]
fn explicit_nontiled_extents_do_not_inherit_the_physical_window() {
    assert_eq!(observation_extent((409, 409), None), (409, 409));
    for size in [412, 500] {
        let options = V3dImageDumpOptions {
            width: size,
            height: size,
            ..Default::default()
        };
        assert_eq!(observation_extent((409, 409), Some(&options)), (size, size));
        assert_eq!(incomplete_reason(false, Some(&options)), None);
    }
}

#[test]
fn actual_tile_or_shadow_projection_refuses_whole_view_reference_eligibility() {
    let shadow = V3dImageDumpOptions {
        buffer_type: Graphic3dBufferType::ShadowMap,
        ..Default::default()
    };
    assert_eq!(
        incomplete_reason(true, None),
        Some(IncompleteObservation::TiledLastPassOnly)
    );
    assert_eq!(
        incomplete_reason(false, Some(&shadow)),
        Some(IncompleteObservation::ShadowMapProjection)
    );
}

#[test]
fn optional_strict_diagnostics_emit_only_explicit_unavailability() {
    for reason in [
        IncompleteObservation::TiledLastPassOnly,
        IncompleteObservation::ShadowMapProjection,
    ] {
        let mut writer = NamedUnavailableWriter::default();
        unavailable_observation(reason, (500, 500), None, &mut writer).unwrap();
        assert_eq!(writer.pieces.len(), 1);
        assert_eq!(writer.pieces[0].0, "unavailable.json");
        let text = String::from_utf8(writer.pieces[0].1.clone()).unwrap();
        assert!(text.contains("\"reference_eligible\":false"));
        assert!(text.contains("\"whole_frame_geometry\":false"));
    }
}

#[test]
fn required_whole_frame_witness_is_incomplete_after_case_end() {
    let mut writer = NamedUnavailableWriter::default();
    let error = unavailable_observation(
        IncompleteObservation::TiledLastPassOnly,
        (412, 412),
        Some(OsStr::new("1")),
        &mut writer,
    )
    .unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::Unsupported);
    let case_end = std::cell::Cell::new(false);
    let source = {
        case_end.set(true);
        Ok(())
    };
    assert!(capture_witness::after_end(source, Some(error)).is_err());
    assert!(case_end.get());
}

#[test]
fn shadow_required_witness_and_stale_diagnostic_fail_closed() {
    let mut writer = NamedUnavailableWriter::default();
    let reason = IncompleteObservation::ShadowMapProjection;
    let required = unavailable_observation(reason, (409, 409), Some(OsStr::new("1")), &mut writer)
        .unwrap_err();
    assert_eq!(required.kind(), io::ErrorKind::Unsupported);
    let stale = unavailable_observation(reason, (409, 409), None, &mut writer).unwrap_err();
    assert_eq!(stale.kind(), io::ErrorKind::AlreadyExists);
}

impl UnavailableDumpWriter for NamedUnavailableWriter {
    fn require_no_ties(&self) -> io::Result<()> {
        if self.stale_ties {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "fake stale ties; expected missing geometry proof",
            ));
        }
        Ok(())
    }
}

#[test]
fn preexisting_geometry_proof_cannot_survive_unavailable_observation_as_success() {
    let mut writer = NamedUnavailableWriter {
        stale_ties: true,
        ..Default::default()
    };
    let error = unavailable_observation(
        IncompleteObservation::TiledLastPassOnly,
        (500, 500),
        None,
        &mut writer,
    )
    .unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
    assert!(writer.pieces.is_empty());
}
