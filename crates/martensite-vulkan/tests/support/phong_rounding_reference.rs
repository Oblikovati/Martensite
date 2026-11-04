//! Lavapipe Phong compile-path residue (user decision 2026-10-28, "prove, then allow";
//! the design notes).
//!
//! The control (the recorded controls) replays a case's lit draws
//! through Mesa OpenGL on llvmpipe: our SPIR-V through GL_ARB_gl_spirv in the Vulkan frame equals
//! the Vulkan image, and our GLSL source through Mesa's GLSL front end in OpenGL's frame equals the
//! native image. The one-level lit-interior residue therefore comes from compiling the same
//! program through the two front ends. Only the tests that control recorded may use this grader,
//! and every differing pixel must
//! - be lit in both images (identical coverage mask; the background is the native corner pixel),
//! - differ by at most one level per RGB channel, with equal alpha.
//!
//! Example: `grade_phong_rounding("bug264_0", &rust, &native, RECORDED_PHONG_CONTROLS)?`.
//!
//! The same bound grades the lavapipe attribute-rounding residue of lit interiors (user decision
//! 2026-10-28) with `RECORDED_ATTRIBUTE_ROUNDING_CONTROLS`: there our SPIR-V through
//! GL_ARB_gl_spirv in OpenGL's own bottom-up frame, with OpenGL's last-vertex convention, equals
//! the native image at every residue pixel. The residue is llvmpipe's attribute setup order,
//! which follows the frame and the provoking vertex, not the shader or its compilation.

/// Tests whose compile-path control reproduced the Vulkan image (controls.json `recorded_tests`).
pub const RECORDED_PHONG_CONTROLS: &[&str] = &[
    "buc60707",
    "bug109",
    "bug20373",
    "bug21757_1",
    "bug21757_2",
    "bug21757",
    "bug22798_1",
    "bug22798_2",
    "bug22818_1",
    "bug22818_2",
    "bug22900",
    "bug22910_2",
    "bug23170",
    "bug23253",
    "bug23282_1",
    "bug23282_2",
    "bug23738",
    "bug23795",
    "bug24220",
    "bug24890",
    "bug25042",
    "bug25044_21",
    "bug25044_30",
    "bug25071",
    "bug25264",
    "bug25588_20",
    "bug25594",
    "bug25611_1",
    "bug25611_2",
    "bug258_1",
    "bug25982",
    "bug264_0",
    "bug264_2",
    "bug264_3",
    "bug264_4",
    "bug264_7",
    "bug26716",
    "bug26965",
    "bug27845",
    "bug28379",
    "bug292",
    "bug29807_b2",
    "bug29807_b4a",
    "bug29807_b4b",
    "bug31258",
    "bug31449_2",
    "bug32073",
    "bug6412",
    "bug810_1",
    "bug_polyline_inclusion",
    "manipulator",
    "rotate2",
];

/// Tests whose bottom-up control reproduced the native image at every residue pixel
/// (controls.json `attribute_rounding_tests`): our SPIR-V in OpenGL's frame, or the Vulkan
/// host run in the LegacyBottomUpV1 frame (vulkan_bottom_up_rows.json).
pub const RECORDED_ATTRIBUTE_ROUNDING_CONTROLS: &[&str] = &[
    "bug22196",
    "bug22304",
    "bug24307_1",
    "bug24867",
    "bug264_5",
    "bug26928",
    "bug27655",
    "bug2883_1",
    "bug31631",
    "bug5988",
    "drag_in_2d_view",
    "headlight",
];

/// The pixels a passing grade accepted; `grade.differing`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhongGrade {
    pub differing: usize,
}

/// Grade RGBA `rust` against RGBA `native` for `test`; `Err` names the first violation.
pub fn grade_phong_rounding(
    test: &str,
    rust: &[u8],
    native: &[u8],
    recorded: &[&str],
) -> Result<PhongGrade, String> {
    if !recorded.contains(&test) {
        return Err(format!(
            "test {test:?} has no recorded control; expected one of {} tests",
            recorded.len()
        ));
    }
    if rust.len() != native.len() || native.len() < 4 || !native.len().is_multiple_of(4) {
        return Err(format!(
            "received {} and {} RGBA bytes; expected equal non-empty multiples of 4",
            rust.len(),
            native.len()
        ));
    }
    let background = &native[..4];
    let mut differing = 0;
    for (index, (r, n)) in rust
        .as_chunks::<4>()
        .0
        .iter()
        .zip(native.as_chunks::<4>().0)
        .enumerate()
    {
        if r != n {
            residue_pixel(index, r, n, background)?;
            differing += 1;
        }
    }
    Ok(PhongGrade { differing })
}

/// One differing pixel: lit in both, RGB within one level, alpha equal.
fn residue_pixel(
    index: usize,
    rust: &[u8],
    native: &[u8],
    background: &[u8],
) -> Result<(), String> {
    if rust == background || native == background {
        return Err(format!(
            "pixel {index} changes coverage ({rust:?} vs native {native:?}); expected lit in both"
        ));
    }
    let within = (0..3).all(|k| rust[k].abs_diff(native[k]) <= 1) && rust[3] == native[3];
    if !within {
        return Err(format!(
            "pixel {index} is {rust:?} vs native {native:?}; expected RGB within one level, equal alpha"
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "phong_rounding_reference_tests.rs"]
mod tests;
