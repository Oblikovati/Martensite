//! Application overrides for the ten virtual reader-filter callbacks.
use super::PcdmReaderFilter;
use martensite_tkernel::{standard::RuntimeType, tcollection::AsciiString};

/// Override any predicate or traversal callback while retaining the default filter state.
///
/// ```
/// use martensite_tkcdf::pcdm::{PcdmReaderFilter, PcdmReaderFilterHooks};
/// struct PartialRead;
/// impl PcdmReaderFilterHooks for PartialRead {
///     fn is_part_tree(&self, _: &PcdmReaderFilter) -> bool { true }
/// }
/// let filter = PcdmReaderFilter::with_hooks(std::sync::Arc::new(PartialRead));
/// assert!(filter.is_part_tree());
/// ```
pub trait PcdmReaderFilterHooks: Send + Sync {
    /// Tests a descriptor through the virtual attribute-name predicate.
    /// Example: `hooks.is_passed_type(&filter, &descriptor)`.
    fn is_passed_type(&self, filter: &PcdmReaderFilter, attribute_type: &RuntimeType) -> bool {
        filter.is_passed_attr(&AsciiString::new(attribute_type.name()))
    }
    /// Tests the configured read/skip names. Example: `hooks.is_passed_attr(&filter, &name)`.
    fn is_passed_attr(&self, filter: &PcdmReaderFilter, attribute_type: &AsciiString) -> bool {
        filter.default_is_passed_attr(attribute_type)
    }
    /// Tests a label entry. Example: `hooks.is_passed_entry(&filter, &entry)`.
    fn is_passed_entry(&self, filter: &PcdmReaderFilter, entry: &AsciiString) -> bool {
        filter.default_is_passed_entry(entry)
    }
    /// Tests whether an entry contains a selected subtree.
    /// Example: `hooks.is_sub_passed_entry(&filter, &entry)`.
    fn is_sub_passed_entry(&self, filter: &PcdmReaderFilter, entry: &AsciiString) -> bool {
        filter.default_is_sub_passed_entry(entry)
    }
    /// Reports partial-tree selection. Example: `hooks.is_part_tree(&filter)`.
    fn is_part_tree(&self, filter: &PcdmReaderFilter) -> bool {
        filter.default_is_part_tree()
    }
    /// Restarts traversal at the root. Example: `hooks.start_iteration(&filter)`.
    fn start_iteration(&self, filter: &PcdmReaderFilter) {
        filter.default_start_iteration();
    }
    /// Visits the parent. Example: `hooks.up(&filter)`.
    fn up(&self, filter: &PcdmReaderFilter) {
        filter.default_up();
    }
    /// Visits a child tag. Example: `hooks.down(&filter, 3)`.
    fn down(&self, filter: &PcdmReaderFilter, tag: i32) {
        filter.default_down(tag);
    }
    /// Tests the current traversal node. Example: `hooks.is_passed(&filter)`.
    fn is_passed(&self, filter: &PcdmReaderFilter) -> bool {
        filter.default_is_passed()
    }
    /// Tests the current traversal depth. Example: `hooks.is_sub_passed(&filter)`.
    fn is_sub_passed(&self, filter: &PcdmReaderFilter) -> bool {
        filter.default_is_sub_passed()
    }
}

/// Explicit base callbacks for an override that delegates part of its work.
///
/// ```
/// use martensite_tkcdf::pcdm::{PcdmDefaultReaderFilterHooks, PcdmReaderFilterHooks};
/// let base = PcdmDefaultReaderFilterHooks;
/// # let filter = martensite_tkcdf::pcdm::PcdmReaderFilter::new();
/// assert!(!base.is_part_tree(&filter));
/// ```
#[derive(Default)]
pub struct PcdmDefaultReaderFilterHooks;
impl PcdmReaderFilterHooks for PcdmDefaultReaderFilterHooks {}
