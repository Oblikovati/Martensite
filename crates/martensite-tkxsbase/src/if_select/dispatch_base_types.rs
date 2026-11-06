//! Named declared dispatch bases; abstract bases remain trait objects.
//!
//! ```
//! use martensite_tkernel::standard::Handle;
//! use martensite_tkxsbase::if_select::{IFSelectDispGlobal, IFSelectDispGlobalBaseType};
//! let base: Handle<IFSelectDispGlobalBaseType> = Handle::new(IFSelectDispGlobal::new()).cast();
//! assert!(!base.is_null());
//! ```
use super::IFSelectDispatch;

/// The declared base of `IFSelectDispGlobal`; use `Handle<IFSelectDispGlobalBaseType>` for its abstract interface.
pub type IFSelectDispGlobalBaseType = dyn IFSelectDispatch;
/// The declared base of `IFSelectDispPerOne`; use `Handle<IFSelectDispPerOneBaseType>` for its abstract interface.
pub type IFSelectDispPerOneBaseType = dyn IFSelectDispatch;
/// The declared base of `IFSelectDispPerCount`; use `Handle<IFSelectDispPerCountBaseType>` for its abstract interface.
pub type IFSelectDispPerCountBaseType = dyn IFSelectDispatch;
/// The declared base of `IFSelectDispPerFiles`; use `Handle<IFSelectDispPerFilesBaseType>` for its abstract interface.
pub type IFSelectDispPerFilesBaseType = dyn IFSelectDispatch;
/// The declared base of `IFSelectDispPerSignature`; use `Handle<IFSelectDispPerSignatureBaseType>` for its abstract interface.
pub type IFSelectDispPerSignatureBaseType = dyn IFSelectDispatch;
