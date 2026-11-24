//! Paired performance cases for the Vulkan renderer against OCCT's OpenGL renderer.
//!
//! Every case here has a C++ twin of the same name in `occt/`. The twin builds the same scene
//! through OCCT's `V3d` viewer on `TKOpenGl` and must report the same checksum: the primitive
//! counts of the renderer's own frame statistics. `cargo xtask occt-parity --toolkit vulkan`
//! builds both programs, runs them in turn on the same CPUs and compares the medians.
//!
//! Both programs draw into a mapped, unmanaged 1280x720 X11 window with vertical sync off, so
//! they need a display and a GPU. `cargo bench -p martensite-vulkan --bench occt_parity`
//! measures (Cargo passes `--bench`); without `--bench` every case draws two frames, as a
//! smoke test. `--filter <text>` keeps the cases whose name contains the text, and
//! `--frames <n>` changes the frames (operations) of one timed run from 100.
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
fn main() {
    linux::run();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("the OCCT parity cases draw into an X11 window; expected a Linux host");
    std::process::exit(2);
}
