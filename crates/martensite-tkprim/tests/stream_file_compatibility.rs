//! Shared-buffer file adapters retain their established byte and caller-state contracts.
#[path = "common/geometry_io.rs"]
mod fakes;
#[path = "support/mod.rs"]
mod support;
#[path = "stream_file_compatibility/archive_format.rs"]
mod archive_format;
#[path = "stream_file_compatibility/contracts.rs"]
mod contracts;
#[path = "stream_file_compatibility/observed_files.rs"]
mod observed_files;
