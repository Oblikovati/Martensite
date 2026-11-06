//! OBJ regressions: each script runs alone in a child process, reads and writes its files with the
//! library, shows the result on the real Vulkan renderer and prints the lines the script prints.
#![cfg(target_os = "linux")]
#![allow(clippy::too_many_lines)]

mod cad_model;
mod cases;
mod checks;
mod process;
mod scene;
mod wavefront;
