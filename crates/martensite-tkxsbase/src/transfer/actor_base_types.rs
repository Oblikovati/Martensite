//! Named parent interfaces of concrete transfer actors.
//!
//! These aliases expose the parent's virtual interface; standalone parent objects are
//! constructed with `TransferActorOfProcessForFinder::new()` or
//! `TransferActorOfProcessForTransient::new()`.
//!
//! ```
//! use martensite_tkernel::standard::Handle;
//! use martensite_tkxsbase::transfer::{TransferActorOfFinderProcess, TransferActorOfFinderProcessBaseType};
//! let base: Handle<TransferActorOfFinderProcessBaseType> = Handle::new(TransferActorOfFinderProcess::new()).cast();
//! base.set_last(true);
//! assert!(base.is_last());
//! ```
use super::{TransferActorOps, TransferFinderKind, TransferTransientKind};

/// The declared parent interface of `TransferActorOfFinderProcess`; use `Handle<TransferActorOfFinderProcessBaseType>`.
pub type TransferActorOfFinderProcessBaseType = dyn TransferActorOps<TransferFinderKind>;
/// The declared parent interface of `TransferActorOfTransientProcess`; use `Handle<TransferActorOfTransientProcessBaseType>`.
pub type TransferActorOfTransientProcessBaseType = dyn TransferActorOps<TransferTransientKind>;
