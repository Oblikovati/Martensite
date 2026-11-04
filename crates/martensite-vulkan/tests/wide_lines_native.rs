//! Aliased wide lines on real devices, including widths beyond RADV's lineWidthRange [0, 8].
//!
//! Native fingerprints come from the recorded wide_lines capture on llvmpipe
//! and AMD radeonsi, which agree byte for byte; kept in fixtures/wide_lines/native.txt (build and
//! run as the recorded depth_prepass capture). Horizontal, vertical and diagonal segments
//! of widths 3, 9 and 12 are drawn one at a time, 1 world unit per pixel.
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "support/rgb_fingerprint.rs"]
mod rgb_fingerprint;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkservice::graphic3d::Graphic3dStructureSource;
use rgb_fingerprint::{FingerprintScene, native_fingerprints};

const NATIVE: &str = include_str!("fixtures/wide_lines/native.txt");

#[path = "support/wide_line_scene.rs"]
mod wide_line_scene;
use wide_line_scene::{SEGMENTS, show_segment};

fn assert_widths(widths: &[f32], lines: &[&str]) {
    let scene = FingerprintScene::new();
    for &width in widths {
        for (name, from, to) in SEGMENTS.iter().filter(|s| lines.contains(&s.0)) {
            let structure =
                show_segment(&scene.scene.viewer.structure_manager(), width, [*from, *to]);
            let actual = scene.dump(&mut |_| {});
            let label = format!("width={width} line={name}");
            let [sw, host] = native_fingerprints(NATIVE, &label);
            assert_eq!(sw, host, "native drivers disagree on {label}");
            assert_eq!(actual, sw, "{label}");
            structure.erase();
        }
    }
}

#[test]
fn wide_lines_inside_the_device_range_match_native() {
    assert_widths(&[3.], &["h", "v", "d"]);
}

#[test]
fn wide_lines_beyond_the_device_range_match_native() {
    // 9 and 12 px exceed RADV's range [0, 8]; lavapipe draws them natively.
    assert_widths(&[9.], &["h", "v", "d"]);
    assert_widths(&[12.], &["h", "v"]);
}

#[test]
fn even_wide_sloped_line_beyond_the_device_range_matches_native() {
    assert_widths(&[12.], &["d"]);
}
