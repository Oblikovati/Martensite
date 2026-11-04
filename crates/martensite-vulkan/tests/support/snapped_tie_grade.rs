//! RADV against radeonsi: one-pixel line columns decided by vertex snapping (D2 cross-driver
//! bound, the design notes; stated here for review).
//!
//! Both AMD drivers snap window vertices to the 2^-8 pixel grid above 1024 px (radeonsi
//! QUANT_MODE 16.8, RADV's fixed mode) and decide each column of a one-pixel line by the side of
//! the diamond corner the snapped line passes. Rounding moves each endpoint by at most 2^-9 per
//! axis, so along the minor axis the snapped line moves by at most 2^-9 (1 + |slope|) <= 2^-8 from
//! the submitted one. The drivers can therefore pick different pixels in a column only where the
//! submitted line passes within 2^-8 px of the corner (the y-up native window and the y-down
//! Vulkan framebuffer also break exact ties on opposite sides). Bound: every differing pixel lies
//! in such a column of a drawn line, and its colour occurs within one pixel in the other image.
//! The bound follows from the snapping argument and is not fitted to the measured pixels.
use martensite_vulkan::{PrimitiveDraw, PrimitiveTopology};
use std::io::Read;

/// One AMD subpixel step: the largest minor-axis move of a snapped line.
const SNAP_BOUND: f64 = 1. / 256.;

/// A one-pixel line in top-down window pixels.
#[derive(Clone, Copy, Debug)]
pub struct WindowLine {
    pub from: [f64; 2],
    pub to: [f64; 2],
}

impl WindowLine {
    /// The distance, along the minor axis, from the line to the corner between this pixel and
    /// its minor-axis neighbour that lies on the line's side (0 is an exact tie).
    fn corner_margin(&self, column: usize, row: usize) -> f64 {
        let (x, y) = (column as f64 + 0.5, row as f64 + 0.5);
        let (dx, dy) = (self.to[0] - self.from[0], self.to[1] - self.from[1]);
        let major = if dx.abs() >= dy.abs() { 0 } else { 1 };
        let along = [x, y][major];
        let (low, high) = (
            self.from[major].min(self.to[major]),
            self.from[major].max(self.to[major]),
        );
        if along < low - 1. || along > high + 1. {
            return f64::MAX;
        }
        let offset = (dy * (x - self.from[0]) - dx * (y - self.from[1])).abs();
        (offset / dx.abs().max(dy.abs()) - 0.5).abs()
    }
}

/// Top-down window coordinates of an object point under a column-major clip matrix.
fn window(clip: &[f32; 16], point: [f32; 3], size: f64) -> [f64; 2] {
    let row = |r: usize| {
        clip[r] * point[0] + clip[4 + r] * point[1] + clip[8 + r] * point[2] + clip[12 + r]
    };
    let w = row(3);
    let (x, y) = (f64::from(row(0) / w), f64::from(row(1) / w));
    [(x * 0.5 + 0.5) * size, (0.5 - y * 0.5) * size]
}

/// The window segment of an object segment under a column-major clip matrix.
pub fn window_line(clip: &[f32; 16], from: [f32; 3], to: [f32; 3], size: f64) -> WindowLine {
    WindowLine {
        from: window(clip, from, size),
        to: window(clip, to, size),
    }
}

/// The window segments of every one-pixel line among `draws`.
pub fn window_lines(draws: &[PrimitiveDraw], size: f64) -> Vec<WindowLine> {
    let lines = draws
        .iter()
        .filter(|d| d.topology() == PrimitiveTopology::Lines);
    lines
        .flat_map(|draw| {
            let point = |index: u32| draw.vertices()[index as usize].position;
            draw.indices().chunks(2).map(move |pair| WindowLine {
                from: window(draw.clip_transform(), point(pair[0]), size),
                to: window(draw.clip_transform(), point(pair[1]), size),
            })
        })
        .collect()
}

/// Decode a gzip-compressed binary PPM fixture into RGB bytes.
pub fn native_rgb(path: &std::path::Path) -> Vec<u8> {
    let file = std::fs::File::open(path).unwrap_or_else(|e| panic!("{path:?}: {e}"));
    let mut bytes = Vec::new();
    flate2::read::GzDecoder::new(file)
        .read_to_end(&mut bytes)
        .unwrap();
    let header = bytes.windows(4).position(|w| w == b"255\n").unwrap() + 4;
    bytes.split_off(header)
}

/// Grade a Vulkan RGB dump against the native RGB image (returns the differing pixel count),
/// handing a pixel beyond the snapping bound (with its corner margin) to `other`, a further
/// reviewed cross-driver bound; `grade_with(&rust, &native, 1040, &lines, &mut other)`.
pub fn grade_with(
    rust: &[u8],
    native: &[u8],
    size: usize,
    lines: &[WindowLine],
    other: &mut dyn FnMut((usize, usize), f64) -> Result<(), String>,
) -> Result<usize, String> {
    fn pixel(image: &[u8], size: usize, (c, r): (usize, usize)) -> &[u8] {
        &image[(r * size + c) * 3..][..3]
    }
    let near = |(c, r): (usize, usize)| {
        let rows = r.saturating_sub(1)..=(r + 1).min(size - 1);
        rows.flat_map(move |rr| {
            (c.saturating_sub(1)..=(c + 1).min(size - 1)).map(move |cc| (cc, rr))
        })
    };
    let shifted = |mine: &[u8], theirs: &[u8], p| {
        near(p).any(|q| pixel(mine, size, p) == pixel(theirs, size, q))
    };
    let mut differing = 0;
    for p in (0..size * size).map(|i| (i % size, i / size)) {
        if pixel(rust, size, p) == pixel(native, size, p) {
            continue;
        }
        differing += 1;
        let margin = lines
            .iter()
            .map(|l| l.corner_margin(p.0, p.1))
            .fold(f64::MAX, f64::min);
        if margin > SNAP_BOUND {
            other(p, margin)?;
            continue;
        }
        if !shifted(rust, native, p) || !shifted(native, rust, p) {
            return Err(format!(
                "pixel {p:?} colour has no match within 1 px; expected a shift"
            ));
        }
    }
    Ok(differing)
}
