//! Loading a service implementation identified by a GUID.
//!
//! Loading shared libraries of compiled code is out of scope, so libraries are
//! a seam ([`PluginLibraries`]) where factories are registered by library file name; the
//! resource lookup, library naming and `Plugin_Failure` messages are fixed.
use crate::standard::exceptions::define_standard_exceptions;
mod libraries;
mod loader;
pub use libraries::{PluginFactory, PluginLibraries, StaticPluginLibraries, library_file_name};
pub use loader::PluginLoader;

define_standard_exceptions! {
    PLUGIN_HIERARCHY;
    failure => "Plugin_Failure" : "Standard_Failure";
}
