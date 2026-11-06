//! Lavapipe user-program compile-path residue (user decision 2026-10-28,
//! the design notes): the Phong compile-path rule extended to user-program draws,
//! proven per pixel.
//!
//! the recorded controls (`user_program_rows` of controls.json)
//! replays each frame's lit and user-program draws through Mesa OpenGL on llvmpipe: the
//! program's SPIR-V through GL_ARB_gl_spirv in the Vulkan frame equals the Vulkan image, and the
//! program's GLSL (as the recording tool wraps it) through Mesa's GLSL front end in
//! OpenGL's frame equals the native image, at every residue pixel. Only the pixels listed in
//! `PROVEN_PIXELS` may use this grader, only on llvmpipe, and each must still be lit in both
//! images (the background is the command host's black clear) and within one level per channel.
//!
//! Example: `compile_path_explained("llvmpipe", "bug21753", &pixel, PROVEN_PIXELS)`.

/// The residue pixels the controls proved: (dump, (column, row)).
pub type ProvenPixel = (&'static str, (usize, usize));

/// controls.json `user_program_rows`: the dumped residue pixels other graders leave. The 3d-*
/// pixels are the texture_3d volume (user_program_texture_3d_native.rs), replayed with its
/// GL_TEXTURE_3D upload (2026-10-29).
pub const PROVEN_PIXELS: &[ProvenPixel] = &[
    ("bug21753", (40, 67)),
    ("bug21753", (82, 97)),
    ("fuse-ph1", (144, 123)),
    ("fuse-ph2", (144, 123)),
    ("up-multi", (144, 21)),
    ("up-multi", (177, 24)),
    ("up-multi", (178, 41)),
    ("up-multi", (104, 139)),
    ("up-multi", (96, 159)),
    ("3d-s0", (89, 48)),
    ("3d-s0", (230, 111)),
    ("3d-s0", (175, 128)),
    ("3d-s0", (188, 176)),
    ("3d-s0", (155, 202)),
    ("3d-s1", (84, 36)),
    ("3d-s1", (89, 48)),
    ("3d-s1", (191, 92)),
    ("3d-s1", (230, 111)),
    ("3d-s1", (205, 194)),
    ("3d-s1", (155, 202)),
    ("3d-s2", (89, 48)),
    ("3d-s2", (230, 111)),
    ("3d-s2", (175, 128)),
    ("3d-s2", (188, 176)),
    ("3d-s2", (155, 202)),
    ("3d-s3", (84, 36)),
    ("3d-s3", (89, 48)),
    ("3d-s3", (191, 92)),
    ("3d-s3", (230, 111)),
    ("3d-s3", (205, 194)),
    ("3d-s3", (155, 202)),
    ("3d-s4", (89, 48)),
    ("3d-s4", (230, 111)),
    ("3d-s4", (175, 128)),
    ("3d-s4", (188, 176)),
    ("3d-s4", (155, 202)),
    ("3d-s5", (84, 36)),
    ("3d-s5", (89, 48)),
    ("3d-s5", (191, 92)),
    ("3d-s5", (230, 111)),
    ("3d-s5", (205, 194)),
    ("3d-s5", (155, 202)),
    ("3d-def", (84, 36)),
    ("3d-def", (89, 48)),
    ("3d-def", (220, 86)),
    ("3d-def", (191, 92)),
    ("3d-def", (218, 108)),
    ("3d-def", (211, 130)),
    ("3d-def", (184, 146)),
];

/// The default view background, the clear colour of every controlled frame.
const BACKGROUND: [u8; 3] = [0, 0, 0];

/// Whether one differing pixel `(column, row, rust RGB, native RGB)` of `dump` on `device` is a
/// proven compile-path residue: llvmpipe, listed, lit in both images, RGB within one level.
pub fn compile_path_explained(
    device: &str,
    dump: &str,
    (column, row, rust, native): (usize, usize, &[u8], &[u8]),
    proven: &[ProvenPixel],
) -> bool {
    let listed = proven.contains(&(dump, (column, row)));
    let lit = rust != BACKGROUND && native != BACKGROUND;
    let within = rust.len() == 3 && native.len() == 3 && within_one_level(rust, native);
    device.contains("llvmpipe") && listed && lit && within
}

fn within_one_level(rust: &[u8], native: &[u8]) -> bool {
    rust.iter().zip(native).all(|(r, n)| r.abs_diff(*n) <= 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PIXEL: (usize, usize, &[u8], &[u8]) = (40, 67, &[120, 80, 40], &[121, 80, 40]);

    #[test]
    fn a_proven_one_level_lit_pixel_passes_on_llvmpipe() {
        assert!(compile_path_explained(
            "llvmpipe (LLVM 20.1.2, 128 bits)",
            "bug21753",
            PIXEL,
            PROVEN_PIXELS
        ));
    }

    #[test]
    fn unlisted_pixels_dumps_and_devices_are_refused() {
        let (column, row, rust, native) = PIXEL;
        let lavapipe = "llvmpipe";
        assert!(!compile_path_explained(
            lavapipe,
            "bug21753",
            (column + 1, row, rust, native),
            PROVEN_PIXELS
        ));
        assert!(!compile_path_explained(
            lavapipe,
            "fuse-ph1",
            PIXEL,
            PROVEN_PIXELS
        ));
        assert!(!compile_path_explained(
            "AMD Radeon RX 7900 XTX (RADV NAVI31)",
            "bug21753",
            PIXEL,
            PROVEN_PIXELS
        ));
    }

    #[test]
    fn two_levels_or_a_coverage_change_are_refused() {
        let (column, row, rust, _) = PIXEL;
        let device = "llvmpipe";
        let two_levels: &[u8] = &[122, 80, 40];
        assert!(!compile_path_explained(
            device,
            "bug21753",
            (column, row, rust, two_levels),
            PROVEN_PIXELS
        ));
        let uncovered: &[u8] = &[0, 0, 0];
        let near_black: &[u8] = &[1, 0, 0];
        assert!(!compile_path_explained(
            device,
            "bug21753",
            (column, row, near_black, uncovered),
            PROVEN_PIXELS
        ));
        assert!(!compile_path_explained(
            device,
            "bug21753",
            (column, row, uncovered, near_black),
            PROVEN_PIXELS
        ));
    }
}
