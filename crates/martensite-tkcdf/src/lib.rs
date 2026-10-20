//! The document framework: documents, applications, storage drivers and their metadata.
//!
//! - [`cdm`]: documents, references between documents, metadata and the abstract application.
//! - [`cdf`]: the application that opens, retrieves and stores documents through drivers.
//! - [`ldom`]: the light DOM that reads and writes XML.
//! - [`pcdm`]: persistent-document interfaces (readers, writers, drivers, reader filter).
//!
//! See `docs/features/martensite-tkcdf/README.md` for the ownership model.
pub mod cdf;
pub mod cdm;
pub mod json_dump;
pub mod ldom;
mod live;
pub mod pcdm;
pub use live::{SharedResourceManager, live};
