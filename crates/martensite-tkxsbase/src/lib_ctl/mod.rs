//! `LibCtl`: the generic library of (module, protocol) pairs that dispatches an object to
//! the module of the protocol that recognises it (xsbase-design.md section 2).
//!
//! The template is instantiated natively per library (`InterfaceGeneralLib`,
//! `InterfaceReaderLib`, and in the format toolkits `StepDataWriterLib`...). Here one
//! generic [`LibCtlLibrary`] serves all of them: a [`LibCtlKind`] names the module and
//! protocol types of an instance and its process-wide [`LibCtlRegistry`] (the native
//! static `THE_GLOBAL`). The `*_in` members take an explicit registry instead.
//!
//! The generated node classes (`*_GlobalNodeOf*Lib`, `*_NodeOf*Lib`) are the generic
//! [`LibCtlGlobalNode`] and [`LibCtlNode`], shared through `Arc` (natively handles).
mod base_type;
mod library;
mod nodes;

pub use base_type::LibCtlNodeBaseType;
pub use library::{LibCtlKind, LibCtlLibrary, LibCtlProtocol, LibCtlRegistry};
pub use nodes::{LibCtlGlobalNode, LibCtlNode};

#[cfg(test)]
mod node_runtime_tests;
