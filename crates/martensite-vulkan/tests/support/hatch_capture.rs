//! Optional owned raw captures bind mask evidence without affecting acceptance.
use super::FrameCapture;

/// Preserve raw color/depth and complete P6 rows; `record("hatch4", &capture)`.
pub fn record(name: &str, capture: &FrameCapture) {
    let Ok(folder) = std::env::var("MARTENSITE_HATCH_CAPTURE_DIR") else {
        return;
    };
    let folder = std::path::PathBuf::from(folder);
    std::fs::create_dir_all(&folder).unwrap_or_else(|error| {
        panic!("received {folder:?}: {error}; expected writable capture directory")
    });
    let depth: Vec<u8> = capture.depth.iter().flat_map(|v| v.to_le_bytes()).collect();
    write_capture_bytes(&folder.join(format!("{name}.rgba")), &capture.rgba);
    write_capture_bytes(&folder.join(format!("{name}.depth")), &depth);
    write_capture_bytes(
        &folder.join(format!("{name}.ppm")),
        &capture_rgb_rows(capture),
    );
}

fn capture_rgb_rows(capture: &FrameCapture) -> Vec<u8> {
    let mut ppm = format!("P6\n{} {}\n255\n", capture.width, capture.height).into_bytes();
    ppm.extend(
        capture
            .rgba
            .as_chunks::<4>()
            .0
            .iter()
            .flat_map(|v| v[..3].iter().copied()),
    );
    ppm
}

fn write_capture_bytes(path: &std::path::Path, bytes: &[u8]) {
    // Artifact failure must identify the owned output, not conceal missing evidence.
    std::fs::write(path, bytes).unwrap_or_else(|error| {
        panic!("received {path:?}: {error}; expected writable capture file")
    });
}

/// Bind the actual selected implementation; `record_identity(&renderer.driver_identity())`.
#[allow(dead_code)] // Only some targets that share this file use it.
pub fn record_identity(identity: &impl std::fmt::Debug) {
    let diagnostic = format!("{identity:?}");
    println!("{{\"hatch_device_identity\":{diagnostic:?}}}");
}
