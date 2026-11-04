//! AMD native-driver SPIR-V control (user decision 2026-10-29, the design notes).
//!
//! The control replays the RADV frame's lit draws on native radeonsi: our exact SPIR-V through
//! GL_ARB_gl_spirv in the Vulkan top-left frame, with Vulkan's first-vertex convention
//! (the recorded controls, `spirv_control_rows` of
//! controls.json). On AMD only, a differing pixel passes when that control equals native there
//! with identical coverage. There is no colour bound: the residue is RADV's own compilation of
//! the same SPIR-V, which can move a steep shading pixel by tens of levels.
//!
//! Each proof states whether the control's coverage equals native at every pixel and lists the
//! differing pixels where the control equals native byte for byte. Every differing pixel must
//! - be listed by a proof whose control has identical coverage, or (non-lit draws, user decision
//!   2026-10-29) misses only text, line, point and marker draws and has identical coverage
//!   outside their pixels, where no pixel is listed,
//! - be lit in both images (identical coverage at the pixel; the background is the native
//!   corner pixel).
//!
//! These historical records lack execution/source/driver and genuine coverage evidence.
//! Nonexact comparisons are now refused.
//!
//! Example: `grade_spirv_control("AMD Radeon Graphics (RADV GFX1151)", &proof, &rust, &native, 409)?`.

/// One recorded proof: the control's coverage check and the pixels it proves.
#[derive(Clone, Copy, Debug)]
pub struct SpirvControlProof<'a> {
    pub test: &'a str,
    pub image: &'a str,
    pub coverage_identical: bool,
    /// Non-lit draws (user decision 2026-10-29): every draw the replay omits is a text, line,
    /// point or marker draw, and the control's coverage equals native outside their pixels.
    pub coverage_identical_outside_text_line_point: bool,
    #[allow(dead_code)] // Only some targets that share this file use it.
    pub pixels: &'a [(usize, usize)],
}
impl SpirvControlProof<'_> {
    /// Whether the control's coverage admits the proof; e.g. `proof.coverage_valid()`.
    pub fn coverage_valid(&self) -> bool {
        self.coverage_identical || self.coverage_identical_outside_text_line_point
    }
}

/// Grade RGBA `rust` against RGBA `native` (`width` pixels per row) under `proof` on `device`;
/// Legacy pixel lists lack source/build/driver and genuine coverage evidence: nonexact
/// images are refused until a qualified proof format is implemented.
pub fn grade_spirv_control(
    device: &str,
    proof: &SpirvControlProof<'_>,
    rust: &[u8],
    native: &[u8],
    width: usize,
) -> Result<usize, String> {
    check_preconditions(device, proof, rust, native, width)?;
    if rust == native {
        return Ok(0);
    }
    Err("legacy SPIR-V pixel lists are diagnostic only; expected qualified source/build/driver and coverage evidence".to_owned())
}

fn check_preconditions(
    device: &str,
    proof: &SpirvControlProof<'_>,
    rust: &[u8],
    native: &[u8],
    width: usize,
) -> Result<(), String> {
    if !device.contains("RADV") {
        return Err(format!("device {device:?}; expected RADV (AMD only)"));
    }
    if !proof.coverage_valid() {
        return Err(format!(
            "{} {}: the control's coverage differs from native; expected identical",
            proof.test, proof.image
        ));
    }
    let rows_of = |len: usize| width > 0 && len.is_multiple_of(4 * width) && len > 0;
    if rust.len() != native.len() || !rows_of(native.len()) {
        return Err(format!(
            "received {} and {} RGBA bytes; expected equal non-empty rows of {width} pixels",
            rust.len(),
            native.len()
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "amd_spirv_control_tests.rs"]
mod tests;
