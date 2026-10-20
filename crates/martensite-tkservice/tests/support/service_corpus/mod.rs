//! The visualization service corpus operations, one module per package area.
pub mod array;
pub mod aspects;
pub mod camera;
pub mod clip_plane;
#[allow(dead_code, unused_imports)]
pub mod common;
pub mod culling;
pub mod driver;
pub mod font;
pub mod grid;
pub mod hid;
pub mod light;
pub mod marker;
pub mod material;
pub mod media;
pub mod mtex;
pub mod params;
pub mod pbr;
pub mod pixel;
#[allow(dead_code)]
pub mod print;
pub mod shader;
pub mod shader_manager;
pub mod stats;
pub mod texture;
pub mod tpers;
#[cfg(target_os = "linux")]
pub mod window;
use print::Output;

/// Encodes a failed call with the existing protocol exception token.
pub fn tried(output: &Output, result: Result<(), martensite_tkernel::standard::Failure>) {
    print::show(output, result, |()| ());
}

mod dispatch;
pub use dispatch::run_request;

#[cfg(test)]
mod contracts;
