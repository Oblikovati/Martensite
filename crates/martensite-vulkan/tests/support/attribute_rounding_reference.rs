//! Lavapipe attribute-rounding residue (user decision, 2026-10-28, the design notes).
//!
//! Mesa's llvmpipe sets up interpolated attributes from the attachment's row 0 and from the
//! positive-area vertex order. GL rasterizes into y-up memory, the default TopLeftLogicalV2 frame
//! into y-down memory, so the plane coefficients round differently and textured interiors may
//! land one level apart. Such a pixel is accepted only when
//! - every RGB channel is within one level and alpha is equal,
//! - coverage is unchanged: native and Rust agree on background versus lit, and depth stays within
//!   the 1e-6 gate,
//! - a control capture of the same scene in the GL-equivalent bottom-up frame (LegacyBottomUpV1
//!   with last-vertex provoking) equals the native pixel, colour and depth.
//!
//! Accepted pixels are replaced by their native value; every other difference is left for the
//! exact-tie grader, which keeps it byte-exact unless it is a proven tie.
use super::exact_tie_reference::TieFrame;

const DEPTH_GATE: f32 = 1e-6;

/// Owned RGBA and depth after rounding-explained pixels took their native value.
#[derive(Clone, Debug, PartialEq)]
pub struct RoundedFrame {
    pub rgba: Vec<u8>,
    pub depth: Vec<f32>,
    pub rounded: usize,
}

impl RoundedFrame {
    /// Borrow for the exact-tie grader; `frame.as_tie_frame()`.
    pub fn as_tie_frame(&self) -> TieFrame<'_> {
        TieFrame {
            rgba: &self.rgba,
            depth: &self.depth,
        }
    }
}

/// Replace the differences explained by attribute rounding; `explain_attribute_rounding(r, n, c)`.
pub fn explain_attribute_rounding(
    rust: TieFrame,
    native: TieFrame,
    control: TieFrame,
) -> Result<RoundedFrame, String> {
    let frames = [rust, native, control];
    let pixels = equal_pixel_count(&frames)?;
    let mut result = RoundedFrame {
        rgba: rust.rgba.to_vec(),
        depth: rust.depth.to_vec(),
        rounded: 0,
    };
    let background = &native.rgba[..4];
    for index in (0..pixels).filter(|&i| rounding_explains(&frames, background, i)) {
        result.rgba[index * 4..index * 4 + 4].copy_from_slice(&native.rgba[index * 4..][..4]);
        result.rounded += 1;
    }
    Ok(result)
}

fn equal_pixel_count(frames: &[TieFrame; 3]) -> Result<usize, String> {
    let pixels = frames[1].depth.len();
    let complete = |f: &TieFrame| f.rgba.len() == pixels * 4 && f.depth.len() == pixels;
    if pixels == 0 || !frames.iter().all(complete) {
        return Err(format!(
            "received unequal RGBA/depth buffers; expected {pixels} pixels in all three"
        ));
    }
    Ok(pixels)
}

/// Whether pixel `index` of `[rust, native, control]` is an attribute-rounding residue.
fn rounding_explains(frames: &[TieFrame; 3], background: &[u8], index: usize) -> bool {
    let [rust, native, control] = [0, 1, 2].map(|k| sample(&frames[k], index));
    if rust.0 == native.0 {
        return false;
    }
    let within_level =
        (0..3).all(|k| rust.0[k].abs_diff(native.0[k]) <= 1) && rust.0[3] == native.0[3];
    let same_coverage = (rust.0 == background) == (native.0 == background)
        && (rust.1 - native.1).abs() <= DEPTH_GATE;
    let control_native = control.0 == native.0 && (control.1 - native.1).abs() <= DEPTH_GATE;
    within_level && same_coverage && control_native
}

fn sample<'a>(frame: &TieFrame<'a>, index: usize) -> (&'a [u8], f32) {
    (&frame.rgba[index * 4..index * 4 + 4], frame.depth[index])
}

#[cfg(test)]
#[path = "attribute_rounding_reference_tests.rs"]
mod tests;
