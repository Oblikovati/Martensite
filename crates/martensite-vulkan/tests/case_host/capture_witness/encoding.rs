//! Serialize IEEE values and actual source geometry; these are not GPU interpolation witnesses.
use super::{OwnedDumpSnapshot, PrimitiveDraw, io};

pub(super) struct WitnessPiece {
    pub name: &'static str,
    pub bytes: Vec<u8>,
}

/// Preserve owned input bits without a rendering claim; snapshot_pieces(snapshot)?.
pub(super) fn snapshot_pieces(snapshot: OwnedDumpSnapshot) -> io::Result<Vec<WitnessPiece>> {
    validate_capture(&snapshot.capture)?;
    let geometry = geometry_bytes(&snapshot.draws);
    let metadata = format!("{:#?}", snapshot.draws).into_bytes();
    let complete = completion_metadata(&snapshot);
    #[cfg(feature = "raytrace-input-diagnostics")]
    let complete = raytrace_completion(complete, snapshot.raytrace_inputs.as_deref());
    let pieces = vec![
        witness_piece("capture.rgba", snapshot.capture.rgba),
        witness_piece("capture.depth-f32le", f32_bytes(&snapshot.capture.depth)),
        witness_piece("submitted-geometry.bin", geometry),
        witness_piece("submitted-draws.txt", metadata),
        witness_piece(
            "camera-state.txt",
            super::camera_state::encode(&snapshot.camera),
        ),
        witness_piece("complete.txt", complete.into_bytes()),
    ];
    Ok(pieces)
}

#[cfg(feature = "raytrace-input-diagnostics")]
fn raytrace_completion(mut complete: String, inputs: Option<&str>) -> String {
    if let Some(json) = inputs {
        complete.push_str("additive_uploaded_inputs=raytrace_inputs_json\nraytrace_inputs_json=");
        complete.push_str(json);
        complete.push('\n');
    }
    complete
}

fn witness_piece(name: &'static str, bytes: Vec<u8>) -> WitnessPiece {
    WitnessPiece { name, bytes }
}

fn validate_capture(capture: &martensite_vulkan::FrameCapture) -> io::Result<()> {
    let pixels = (capture.width as usize).checked_mul(capture.height as usize);
    let valid = pixels.is_some_and(|n| {
        n > 0 && n.checked_mul(4) == Some(capture.rgba.len()) && n == capture.depth.len()
    });
    if valid {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "frame={}x{}, rgba={}, depth={}; expected positive packed RGBA8 and one f32 depth per pixel",
            capture.width,
            capture.height,
            capture.rgba.len(),
            capture.depth.len()
        ),
    ))
}

fn completion_metadata(snapshot: &OwnedDumpSnapshot) -> String {
    format!(
        "schema=martensite-original-frame-observation-v1\nwidth={}\nheight={}\ndraws={}\nbottom_up_requested={}\nfloat_order=IEEE754-little-endian\ncapture=last-accepted-no-redraw\ncamera=raw-state-and-F-matrices-from-same-view-at-dump\ngeometry=actual-PrimitiveDraw-values-normal-then-immediate\nhashes=frozen-controller-manifest\nlimits=no-uploaded-buffer-or-shader-interpolation-witness\ncoverage-tag-3=unsupported-polygon-point-raster\n",
        snapshot.capture.width,
        snapshot.capture.height,
        snapshot.draws.len(),
        std::env::var_os("MARTENSITE_CASE_BOTTOM_UP_CONTROL").is_some()
    )
}

fn f32_bytes(values: &[f32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_bits().to_le_bytes())
        .collect()
}

fn geometry_bytes(draws: &[PrimitiveDraw]) -> Vec<u8> {
    let mut encoded = b"MVWIT001".to_vec();
    encoded.extend_from_slice(&(draws.len() as u64).to_le_bytes());
    for draw in draws {
        append_draw(&mut encoded, draw);
    }
    encoded
}

fn append_draw(encoded: &mut Vec<u8>, draw: &PrimitiveDraw) {
    let normals = draw.lighting().map_or(&[][..], |light| light.normals());
    encoded.extend_from_slice(
        &raster_coverage_tag(draw.rasterizes_polygon_points(), draw.topology() as u32)
            .to_le_bytes(),
    );
    for count in [draw.vertices().len(), draw.indices().len(), normals.len()] {
        encoded.extend_from_slice(&(count as u64).to_le_bytes());
    }
    encoded.extend(f32_bytes(draw.clip_transform()));
    for vertex in draw.vertices() {
        encoded.extend(f32_bytes(&[
            vertex.position[0],
            vertex.position[1],
            vertex.position[2],
            vertex.weight,
        ]));
        encoded.extend(f32_bytes(&vertex.color));
    }
    encoded.extend(draw.indices().iter().flat_map(|index| index.to_le_bytes()));
    encoded.extend(normals.iter().flat_map(|normal| f32_bytes(normal)));
}

// Preserve assembly bytes, but never advertise filled triangle coverage for polygon-point raster.
fn raster_coverage_tag(polygon_points: bool, topology: u32) -> u32 {
    if polygon_points {
        return 3;
    }
    topology
}
#[cfg(test)]
mod raster_coverage_tests {
    use super::*;
    #[test]
    fn polygon_point_coverage_is_explicitly_unknown_instead_of_filled_triangle() {
        assert_eq!(raster_coverage_tag(true, 0), 3);
        for tag in [0, 1, 2] {
            assert_eq!(raster_coverage_tag(false, tag), tag);
        }
    }
}

#[cfg(all(test, feature = "raytrace-input-diagnostics"))]
mod raytrace_controls {
    use super::*;
    #[test]
    fn unrequested_metadata_preserves_existing_complete_bytes() {
        let original =
            "schema=existing\nlimits=no-uploaded-buffer-or-shader-interpolation-witness\n";
        assert_eq!(raytrace_completion(original.into(), None), original);
    }
    #[test]
    fn additive_inputs_stay_inside_the_existing_sealed_complete_piece() {
        let original = "schema=existing\n";
        let complete = raytrace_completion(original.into(), Some("{\"view_id\":7}"));
        assert!(complete.starts_with(original));
        let json = complete
            .lines()
            .find_map(|line| line.strip_prefix("raytrace_inputs_json="))
            .unwrap();
        let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(parsed["view_id"], 7);
        assert_eq!(complete.matches("raytrace_inputs_json=").count(), 1);
    }
}
