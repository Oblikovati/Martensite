//! Declared filter parents name actual live interfaces rather than detached list snapshots.
use super::{SelectMgrCompositionFilter, SelectMgrFilter};

/// The composition framework's declared parent shares the actual filter object.
///
/// ```
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkv3d::select_mgr::{SelectMgrAndFilter, SelectMgrCompositionFilterBaseType};
/// let owner = Handle::new(SelectMgrAndFilter::default());
/// let parent: Handle<SelectMgrCompositionFilterBaseType> = owner.cast();
/// assert_eq!(owner.object_address(), parent.object_address());
/// ```
pub type SelectMgrCompositionFilterBaseType = dyn SelectMgrFilter;
/// The AND filter's declared parent; use `Handle<SelectMgrAndFilterBaseType>` for live composition methods.
pub type SelectMgrAndFilterBaseType = dyn SelectMgrCompositionFilter;
/// The OR filter's declared parent; use `Handle<SelectMgrOrFilterBaseType>` for live composition methods.
pub type SelectMgrOrFilterBaseType = dyn SelectMgrCompositionFilter;
/// The configurable filter's declared parent; use `Handle<SelectMgrAndOrFilterBaseType>` for live composition methods.
pub type SelectMgrAndOrFilterBaseType = dyn SelectMgrCompositionFilter;
