//! The parsing of an IGES file: its records split by section, then a model loaded from them.
mod parameters;
mod read;
mod records;
mod structure;

pub use read::{
    iges_file_read, iges_file_read_fnes, iges_file_read_stream, iges_file_read_stream_with,
    iges_file_read_with,
};
