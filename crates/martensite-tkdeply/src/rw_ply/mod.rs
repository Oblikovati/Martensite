//! Writing an exchange document or a stream of points into an ASCII PLY file.
mod caf_writer;
mod writer_context;
pub use caf_writer::{RwPlyCafWriter, face_color_bytes};
pub use writer_context::{RwPlyOutput, RwPlyPlyWriterContext};
