//! The toolkit's scripts that end on a shaded view: each runs alone in a child process, builds its
//! model with the library, shows it on the real Vulkan renderer and is graded on its whole output,
//! with a clean validation report.
#![cfg(target_os = "linux")]
#![allow(dead_code)] // The shared contract holds more checks than these cases call.
#![allow(
    clippy::too_many_lines,
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::chunks_exact_to_as_chunks
)]

mod cases;
mod commands;
mod geometry;
mod grading;
mod modelling;
mod scene;
mod support;
mod sweeps;
mod variables;
