//! Application-defined reader filters must dispatch every virtual predicate and cursor hook.
use martensite_tkcdf::pcdm::{
    PcdmDefaultReaderFilterHooks, PcdmReaderFilter, PcdmReaderFilterAppendMode,
    PcdmReaderFilterHooks,
};
use martensite_tkernel::{
    standard::{Handle, RuntimeType, RuntimeTyped, StandardTransient},
    tcollection::AsciiString,
};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct RecordingFilterHooks {
    calls: Mutex<Vec<&'static str>>,
}

impl RecordingFilterHooks {
    fn record(&self, call: &'static str, result: bool) -> bool {
        self.calls.lock().unwrap().push(call);
        result
    }
}

impl PcdmReaderFilterHooks for RecordingFilterHooks {
    fn is_passed_type(&self, _: &PcdmReaderFilter, descriptor: &RuntimeType) -> bool {
        assert_eq!(descriptor.name(), "Standard_Transient");
        self.record("type", false)
    }
    fn is_passed_attr(&self, _: &PcdmReaderFilter, name: &AsciiString) -> bool {
        assert_eq!(name, &AsciiString::new("Attribute"));
        self.record("attribute", false)
    }
    fn is_passed_entry(&self, _: &PcdmReaderFilter, entry: &AsciiString) -> bool {
        assert_eq!(entry, &AsciiString::new("0:7"));
        self.record("entry", false)
    }
    fn is_sub_passed_entry(&self, _: &PcdmReaderFilter, entry: &AsciiString) -> bool {
        assert_eq!(entry, &AsciiString::new("0:7"));
        self.record("sub-entry", false)
    }
    fn is_part_tree(&self, _: &PcdmReaderFilter) -> bool {
        self.record("partial", true)
    }
    fn start_iteration(&self, _: &PcdmReaderFilter) {
        self.record("start", false);
    }
    fn up(&self, _: &PcdmReaderFilter) {
        self.record("up", false);
    }
    fn down(&self, _: &PcdmReaderFilter, tag: i32) {
        assert_eq!(tag, 7);
        self.record("down", false);
    }
    fn is_passed(&self, _: &PcdmReaderFilter) -> bool {
        self.record("passed", true)
    }
    fn is_sub_passed(&self, _: &PcdmReaderFilter) -> bool {
        self.record("sub-passed", false)
    }
}

fn check_predicates(filter: &PcdmReaderFilter) {
    assert!(!filter.is_passed_type(StandardTransient::static_type()));
    assert!(!filter.is_passed_attr(&AsciiString::new("Attribute")));
    assert!(!filter.is_passed_entry(&AsciiString::new("0:7")));
    assert!(!filter.is_sub_passed_entry(&AsciiString::new("0:7")));
    assert!(filter.is_part_tree());
    assert!(filter.is_passed());
    assert!(!filter.is_sub_passed());
}

#[test]
fn all_ten_overrides_dispatch_through_the_concrete_reader_handle() {
    let hooks = Arc::new(RecordingFilterHooks::default());
    let filter = Handle::new(PcdmReaderFilter::with_hooks(hooks.clone()));
    filter.start_iteration();
    filter.down(7);
    filter.up();
    check_predicates(&filter);
    assert_eq!(
        *hooks.calls.lock().unwrap(),
        [
            "start",
            "down",
            "up",
            "type",
            "attribute",
            "entry",
            "sub-entry",
            "partial",
            "passed",
            "sub-passed"
        ]
    );
    filter.set_mode(PcdmReaderFilterAppendMode::Protect);
    assert!(filter.is_append_mode());
}

struct AttributeOnlyHooks;
impl PcdmReaderFilterHooks for AttributeOnlyHooks {
    fn is_passed_attr(&self, _: &PcdmReaderFilter, name: &AsciiString) -> bool {
        name != &AsciiString::new("Standard_Transient")
    }
}

#[test]
fn default_descriptor_predicate_calls_the_overridden_attribute_predicate() {
    let filter = PcdmReaderFilter::with_hooks(Arc::new(AttributeOnlyHooks));
    assert!(!filter.is_passed_type(StandardTransient::static_type()));
    assert!(filter.is_passed_attr(&AsciiString::new("Other")));
}

struct BaseDelegatingHooks;
impl PcdmReaderFilterHooks for BaseDelegatingHooks {
    fn is_passed_entry(&self, filter: &PcdmReaderFilter, entry: &AsciiString) -> bool {
        PcdmDefaultReaderFilterHooks.is_passed_entry(filter, entry)
    }
}

#[test]
fn partial_overrides_can_delegate_to_the_existing_tree_and_attribute_defaults() {
    let filter = PcdmReaderFilter::with_hooks(Arc::new(BaseDelegatingHooks));
    filter.add_path(&AsciiString::new("0:1:2"));
    filter.add_skipped(&AsciiString::new("Skip"));
    assert!(!filter.is_passed_attr(&AsciiString::new("Skip")));
    assert!(filter.is_passed_entry(&AsciiString::new("0:1:2:3")));
    assert!(!filter.is_passed_entry(&AsciiString::new("0:1:20")));
    assert!(filter.is_sub_passed_entry(&AsciiString::new("0:1")));
    assert!(filter.is_part_tree());
    filter.start_iteration();
    filter.down(1);
    filter.down(2);
    assert!(filter.is_passed() && filter.is_sub_passed());
    filter.up();
    assert!(!filter.is_passed() && filter.is_sub_passed());
}

#[test]
fn public_filter_and_hook_contracts_are_send_and_sync() {
    fn require_send_sync<T: Send + Sync + ?Sized>() {}
    require_send_sync::<PcdmReaderFilter>();
    require_send_sync::<dyn PcdmReaderFilterHooks>();
}
