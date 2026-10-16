//! Martensite foundation toolkit: errors, memory, collections, strings, OS services,
//! messages, units and resources.

mod c_format;
mod error;
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod native_calendar;
pub mod quantity;
pub use error::{Error, ErrorKind};
pub mod fsd;
pub mod message;
pub mod ncollection;
pub mod osd;
pub mod plugin;
pub mod precision;
pub mod resource;
pub mod standard;
pub mod std_fail;
pub mod stl;
pub mod storage;
pub mod tcol_std;
pub mod tcollection;
pub mod units;
pub mod units_api;
pub mod units_methods;
