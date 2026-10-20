//! The application data framework: labels, attributes, documents and transactions.
//!
//! - [`tdf`]: labels, attributes, transactions, deltas and the copy tools (`TDF`).
//! - [`tdata_std`]: the standard attributes (values, strings, arrays, lists, maps, trees).
//! - [`tfunction`]: functions, their drivers, dependencies and execution order.
//! - [`tdoc_std`]: documents, applications, external links and multi-document transactions.
//!
//! See `docs/features/martensite-tklcaf/README.md` for the ownership model.
mod guid_text;
pub mod json;
pub mod tdata_std;
pub mod tdf;
pub mod tdoc_std;
pub mod tfunction;
pub use guid_text::guid_from_text;
pub use martensite_tkcdf::live;
