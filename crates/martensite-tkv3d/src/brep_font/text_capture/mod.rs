//! Owned strict text capture; legacy rendering and discovery are unchanged.
mod access;
mod admission;
mod backend;
mod budget;
mod capture;
mod glyphs;
#[cfg(test)]
mod tests;
mod types;

pub use capture::capture_owned_font_text;
pub use types::*;
