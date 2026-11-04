//! The cross-driver (AMD radeonsi versus RADV) residue bounds of tests/shadow_maps_native.rs.
//!
//! A differing pixel is proven when one of these holds at the surface under its centre:
//! - a tap within one 24-bit depth unit of its stored depth (support/shadow_flip_reference.rs);
//! - a tap coordinate within the sampler's sub-texel precision of a texel border across which
//!   the tap's decision changes (`admits_selection_flip`);
//! - a same-driver capture with the map raster shifted by at most 1/512 texel along one axis
//!   reproduces the native value (support/shadow_silhouette_reference.rs);
//! - a face-edge raster tie within the 2^-8 snap bound whose colours occur within one pixel in
//!   the other image;
//! - rule B (user decision 2026-10-28): the view raster's vertex grid changes a shadowed
//!   tap on the recorded triangle (support/shadow_view_snap.rs);
//! - rule C (widened by the user decision of 2026-10-28): a face edge within RADV's 2^-8 px snap
//!   bound of the pixel centre covers the pixel in one image only, and the covered colour is
//!   within one level of a lit colour within one pixel in the other image;
//! - rule D: inside a face, more than the snap bound from every edge, the pixel counts as lit in
//!   both images whatever its colour, so the one-level shading bound applies.
//!
//! Every bound here is cross-driver: tests/shadow_maps_native.rs applies them to AMD only.
use super::grading::{DumpReport, UnexplainedPixel};
use super::shadow_flip_reference::ShadowFlipScene;
use super::shadow_silhouette_reference::{admits_shifted_raster_flip, reads_caster_silhouette};
use super::shadow_view_snap::{admits_view_snap_flip, view_snap_json};

/// One AMD subpixel step: both AMD drivers snap window vertices to the 2^-8 pixel grid, so a
/// fill edge moves at most 2^-8 px along its minor axis (support/snapped_tie_grade.rs).
const SNAP_BOUND: f64 = 1. / 256.;

/// The evidence of one dump beyond its scene: our image, the native image, the shifted-raster
/// control captures, the window width and the device's sub-texel precision in texels.
pub struct ResidueEvidence<'a> {
    pub rust: &'a [u8],
    pub native: &'a [u8],
    pub controls: &'a [Vec<u8>],
    pub size: (usize, usize),
    pub sub_texel: f64,
}

/// The faces of the scene that cast: the box (the first six faces).
const CASTER_FACES: usize = 6;

/// Whether the colour of pixel (x, y) of `image` occurs within one pixel in `other`.
fn occurs_within_one_pixel(
    image: &[u8],
    other: &[u8],
    size: (usize, usize),
    (x, y): (usize, usize),
) -> bool {
    let colour = &image[3 * (y * size.0 + x)..][..3];
    let near = |d: usize| d.saturating_sub(1)..=d + 1;
    near(y).filter(|&ny| ny < size.1).any(|ny| {
        near(x)
            .filter(|&nx| nx < size.0)
            .any(|nx| &other[3 * (ny * size.0 + nx)..][..3] == colour)
    })
}

fn print_diagnostics(
    name: &str,
    scene: &ShadowFlipScene,
    evidence: &ResidueEvidence<'_>,
    (x, y): (usize, usize),
) {
    let silhouette = reads_caster_silhouette(scene, (x, y), CASTER_FACES);
    let at = 3 * (y * evidence.size.0 + x);
    let native = &evidence.native[at..at + 3];
    let reproduced: Vec<bool> = evidence
        .controls
        .iter()
        .map(|c| &c[at..at + 3] == native)
        .collect();
    let margin = scene.closest_margin(x, y);
    let tie = scene.closest_selection_tie(x, y);
    let edge = scene.edge_offset(x, y);
    // The reference's stored depths against the rendered map at the same taps.
    let rendered = scene.rendered_disagreement(x, y);
    let rust = &evidence.rust[at..at + 3];
    let snap = view_snap_json(scene, (x, y));
    println!(
        "{{\"shadow_residue\":{name:?},\"pixel\":[{x},{y}],\"native\":{native:?},\"rust\":{rust:?},\"margin_units\":{margin:?},\"selection_tie_texels\":{tie:?},\"edge_offset_px\":{edge},\"rendered_units\":{rendered:?},\"silhouette\":{silhouette},\"reproduced_by_shift\":{reproduced:?},\"view_snap_taps\":{snap}}}"
    );
}

fn rgb_at(image: &[u8], size: (usize, usize), (x, y): (usize, usize)) -> [u8; 3] {
    let at = 3 * (y * size.0 + x);
    [image[at], image[at + 1], image[at + 2]]
}

fn within_one_level(a: [u8; 3], b: [u8; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 1)
}

/// Rule C: a face edge through the centre; one image shows the clear colour there and the
/// other's colour is within one level of a lit colour within one pixel in the first.
fn admits_centre_edge_tie(
    scene: &ShadowFlipScene,
    evidence: &ResidueEvidence<'_>,
    at: (usize, usize),
) -> bool {
    let (rust, native, size) = (evidence.rust, evidence.native, evidence.size);
    let clear = rgb_at(native, size, (0, 0));
    let (r, n) = (rgb_at(rust, size, at), rgb_at(native, size, at));
    let (covered, uncovered_image) = match (r == clear, n == clear) {
        (false, true) => (r, native),
        (true, false) => (n, rust),
        _ => return false,
    };
    let near = |d: usize, limit: usize| d.saturating_sub(1)..=(d + 1).min(limit - 1);
    let neighbour_matches = near(at.1, size.1).any(|y| {
        near(at.0, size.0).any(|x| {
            let colour = rgb_at(uncovered_image, size, (x, y));
            colour != clear && within_one_level(colour, covered)
        })
    });
    scene.edge_offset(at.0, at.1) <= SNAP_BOUND && neighbour_matches
}

/// Rule D: the centre lies on a face more than the snap bound from every edge, and every channel
/// is within one level.
fn admits_interior_one_level(
    scene: &ShadowFlipScene,
    evidence: &ResidueEvidence<'_>,
    at: (usize, usize),
) -> bool {
    let (r, n) = (
        rgb_at(evidence.rust, evidence.size, at),
        rgb_at(evidence.native, evidence.size, at),
    );
    let interior =
        scene.surface(at.0, at.1).is_some() && scene.edge_offset(at.0, at.1) > SNAP_BOUND;
    interior && within_one_level(r, n)
}

/// Whether one differing pixel is proven by one of the module's bounds.
fn proven(
    scene: &ShadowFlipScene,
    evidence: &ResidueEvidence<'_>,
    pixel: &UnexplainedPixel,
) -> bool {
    let at = (pixel.0, pixel.1);
    let (rust, native, size) = (evidence.rust, evidence.native, evidence.size);
    let raster_tie = scene.edge_offset(at.0, at.1) <= SNAP_BOUND
        && occurs_within_one_pixel(rust, native, size, at)
        && occurs_within_one_pixel(native, rust, size, at);
    let controls = (native, evidence.controls);
    let radv_rgb = rgb_at(rust, size, at);
    let native_rgb = rgb_at(native, size, at);
    scene.admits_flip(at.0, at.1)
        || scene.admits_selection_flip(at.0, at.1, evidence.sub_texel)
        || admits_shifted_raster_flip(at, controls, size.0)
        || raster_tie
        || admits_view_snap_flip(scene, at, &radv_rgb, &native_rgb)
        || admits_centre_edge_tie(scene, evidence, at)
        || admits_interior_one_level(scene, evidence, at)
}

/// Drop the differing pixels of `report` the cross-driver bounds prove.
pub fn without_proven_residues(
    report: DumpReport,
    scene: &ShadowFlipScene,
    evidence: &ResidueEvidence<'_>,
) -> DumpReport {
    let (name, differing, pixels) = report;
    pixels
        .iter()
        .for_each(|p| print_diagnostics(name, scene, evidence, (p.0, p.1)));
    let left = pixels
        .into_iter()
        .filter(|pixel| !proven(scene, evidence, pixel));
    (name, differing, left.collect())
}

#[cfg(test)]
mod tests {
    use super::super::shadow_flip_reference_tests::overhead;
    use super::{ResidueEvidence, admits_centre_edge_tie, admits_interior_one_level};

    const SIZE: (usize, usize) = (64, 64);

    /// The overhead floor with its left edge at window x = 16.5 + `offset` (column 16's centres
    /// plus `offset` px); the view maps world x to window x by 3.2 px per unit.
    fn floor_with_edge_at(offset: f64) -> super::ShadowFlipScene {
        let mut scene = overhead(0.5);
        for corner in [0, 3] {
            scene.faces[0].corners[corner][0] = ((16.5 + offset) / 32. - 1.) * 10.;
        }
        scene
    }

    fn floor_through_centres() -> super::ShadowFlipScene {
        floor_with_edge_at(0.)
    }

    /// A black image with the given pixels set.
    fn image(pixels: &[((usize, usize), [u8; 3])]) -> Vec<u8> {
        let mut rgb = vec![0; 3 * SIZE.0 * SIZE.1];
        for &((x, y), colour) in pixels {
            rgb[3 * (y * SIZE.0 + x)..][..3].copy_from_slice(&colour);
        }
        rgb
    }

    fn evidence<'a>(rust: &'a [u8], native: &'a [u8]) -> ResidueEvidence<'a> {
        ResidueEvidence {
            rust,
            native,
            controls: &[],
            size: SIZE,
            sub_texel: 1. / 256.,
        }
    }

    #[test]
    fn an_edge_through_the_centre_may_take_a_neighbours_colour_within_one_level() {
        let scene = floor_through_centres();
        let rust = image(&[((16, 30), [8, 0, 0]), ((17, 30), [9, 0, 0])]);
        let native = image(&[((17, 30), [9, 0, 0])]);
        assert!(admits_centre_edge_tie(
            &scene,
            &evidence(&rust, &native),
            (16, 30)
        ));
        let far = image(&[((17, 30), [11, 0, 0])]);
        assert!(
            !admits_centre_edge_tie(&scene, &evidence(&rust, &far), (16, 30)),
            "two levels"
        );
        let both = image(&[((16, 30), [9, 0, 0]), ((17, 30), [9, 0, 0])]);
        assert!(
            !admits_centre_edge_tie(&scene, &evidence(&rust, &both), (16, 30)),
            "covered in both"
        );
    }

    #[test]
    fn an_edge_within_the_snap_bound_counts_and_one_beyond_it_does_not() {
        let rust = image(&[((16, 30), [8, 0, 0]), ((17, 30), [9, 0, 0])]);
        let native = image(&[((17, 30), [9, 0, 0])]);
        let near = floor_with_edge_at(0.003);
        assert!(admits_centre_edge_tie(
            &near,
            &evidence(&rust, &native),
            (16, 30)
        ));
        let beyond = floor_with_edge_at(0.005);
        assert!(!admits_centre_edge_tie(
            &beyond,
            &evidence(&rust, &native),
            (16, 30)
        ));
    }

    #[test]
    fn an_edge_off_the_centre_is_no_centre_tie() {
        // The original floor's edges lie on window x = 16 and 48, between pixel centres.
        let scene = overhead(0.5);
        let rust = image(&[((16, 30), [8, 0, 0]), ((17, 30), [9, 0, 0])]);
        let native = image(&[((17, 30), [9, 0, 0])]);
        assert!(!admits_centre_edge_tie(
            &scene,
            &evidence(&rust, &native),
            (16, 30)
        ));
    }

    #[test]
    fn inside_a_face_one_level_passes_even_at_the_clear_colour() {
        let scene = floor_through_centres();
        let (rust, native) = (image(&[((30, 30), [1, 0, 0])]), image(&[]));
        assert!(admits_interior_one_level(
            &scene,
            &evidence(&rust, &native),
            (30, 30)
        ));
        let two = image(&[((30, 30), [2, 0, 0])]);
        assert!(
            !admits_interior_one_level(&scene, &evidence(&two, &native), (30, 30)),
            "two levels"
        );
        let edge = image(&[((16, 30), [1, 0, 0])]);
        assert!(
            !admits_interior_one_level(&scene, &evidence(&edge, &native), (16, 30)),
            "on the edge"
        );
        let off = image(&[((2, 2), [1, 0, 0])]);
        assert!(
            !admits_interior_one_level(&scene, &evidence(&off, &native), (2, 2)),
            "off every face"
        );
    }
}
