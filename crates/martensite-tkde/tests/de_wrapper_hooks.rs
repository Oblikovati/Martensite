//! Virtual registry dispatch, explicit base delegation and callback ownership.
mod support;
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{
    DeProvider, DeProviderReadStreamList, DeProviderReadStreamNode, DeSectionGlobal, DeWrapper,
    DeWrapperHooks,
};
use martensite_tkernel::{
    message::ProgressRange,
    osd::{IStream, SeekDir},
    standard::{Failure, Handle, SharedState, exceptions},
    tcollection::AsciiString,
};
use std::sync::Arc;
use support::memory_stream::memory_istream;

type Trace = Arc<SharedState<Vec<String>>>;
struct RecordingHooks {
    trace: Trace,
}
impl RecordingHooks {
    fn record(&self, path: &AsciiString, event: String) {
        assert_eq!(path, &AsciiString::new("asset.ext"));
        self.trace.write().push(event);
    }
}
impl DeWrapperHooks for RecordingHooks {
    fn find_provider(
        &self,
        _: &DeWrapper,
        path: &AsciiString,
        importing: bool,
        _: &mut Handle<dyn DeProvider>,
    ) -> Result<bool, Failure> {
        self.record(path, format!("find:{importing}"));
        Ok(false)
    }
    fn find_read_provider(
        &self,
        _: &DeWrapper,
        path: &AsciiString,
        content: bool,
        _: &mut Handle<dyn DeProvider>,
    ) -> Result<bool, Failure> {
        self.record(path, format!("read:{content}"));
        Ok(false)
    }
    fn find_read_provider_from_stream(
        &self,
        _: &DeWrapper,
        path: &AsciiString,
        stream: &mut dyn IStream,
        _: &mut Handle<dyn DeProvider>,
    ) -> Result<bool, Failure> {
        self.record(path, format!("stream:{}", stream.tellg()));
        Ok(false)
    }
    fn find_write_provider(
        &self,
        _: &DeWrapper,
        path: &AsciiString,
        _: &mut Handle<dyn DeProvider>,
    ) -> bool {
        self.record(path, "write".to_owned());
        false
    }
    fn copy(&self, _: &DeWrapper) -> Handle<DeWrapper> {
        self.trace.write().push("copy".to_owned());
        let copy = Handle::new(DeWrapper::new());
        copy.set_global_parameters(DeSectionGlobal {
            length_unit: 42.,
            system_unit: 1.,
        });
        copy
    }
}
fn registry() -> (DeWrapper, Trace) {
    let trace = Arc::new(SharedState::new(Vec::new()));
    let wrapper = DeWrapper::with_hooks(Arc::new(RecordingHooks {
        trace: trace.clone(),
    }));
    (wrapper, trace)
}
fn path() -> AsciiString {
    AsciiString::new("asset.ext")
}
fn assert_trace(trace: &Trace, expected: &[&str]) {
    let actual: Vec<String> = expected.iter().map(|value| (*value).to_owned()).collect();
    assert_eq!(*trace.read(), actual);
}
fn direct_lookups(wrapper: &DeWrapper) {
    let mut provider = Handle::default();
    assert!(!wrapper.find_provider(&path(), true, &mut provider).unwrap());
    assert!(
        !wrapper
            .find_read_provider(&path(), false, &mut provider)
            .unwrap()
    );
    let mut stream = memory_istream(b"abcd");
    stream.seekg(2, SeekDir::Beg);
    assert!(
        !wrapper
            .find_read_provider_from_stream(&path(), &mut stream, &mut provider)
            .unwrap()
    );
    assert_eq!(stream.tellg(), 2);
    assert!(!wrapper.find_write_provider(&path(), &mut provider));
    assert!(provider.is_null());
}
#[test]
fn all_five_virtual_signatures_dispatch_to_owned_callbacks() {
    let (wrapper, trace) = registry();
    direct_lookups(&wrapper);
    assert_eq!(wrapper.copy().global_parameters().length_unit, 42.);
    assert_trace(
        &trace,
        &["find:true", "read:false", "stream:2", "write", "copy"],
    );
}
#[test]
fn explicit_base_direction_lookup_retains_nested_virtual_dispatch() {
    let (wrapper, trace) = registry();
    let mut provider = Handle::default();
    assert!(
        !wrapper
            .default_find_provider(&path(), true, &mut provider)
            .unwrap()
    );
    assert!(
        !wrapper
            .default_find_provider(&path(), false, &mut provider)
            .unwrap()
    );
    assert_trace(&trace, &["read:true", "write"]);
}
#[test]
fn path_and_stream_transfer_entrypoints_use_the_overrides() {
    let (wrapper, trace) = registry();
    let range = ProgressRange::default();
    let mut shape = Shape::default();
    assert!(!wrapper.read_shape(&path(), &mut shape, &range).unwrap());
    assert!(!wrapper.write_shape(&path(), &shape, &range).unwrap());
    let mut stream = memory_istream(b"abcd");
    stream.seekg(2, SeekDir::Beg);
    let mut streams = DeProviderReadStreamList::default();
    streams.append(DeProviderReadStreamNode::new(&path(), &mut stream));
    assert!(
        !wrapper
            .read_shape_streams(&mut streams, &mut shape, &range)
            .unwrap()
    );
    assert_trace(&trace, &["find:true", "find:false", "stream:2"]);
}
struct RejectingHooks;
impl DeWrapperHooks for RejectingHooks {
    fn find_provider(
        &self,
        _: &DeWrapper,
        _: &AsciiString,
        _: bool,
        _: &mut Handle<dyn DeProvider>,
    ) -> Result<bool, Failure> {
        Err(exceptions::domain_error("lookup rejected asset.ext"))
    }
}
#[test]
fn callback_failure_keeps_its_typed_cause_through_transfer() {
    let wrapper = DeWrapper::with_hooks(Arc::new(RejectingHooks));
    let mut shape = Shape::default();
    let failure = wrapper
        .read_shape(&path(), &mut shape, &ProgressRange::default())
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert_eq!(failure.what(), b"lookup rejected asset.ext");
    assert!(shape.is_null());
}
#[test]
fn base_copy_slices_callbacks_and_does_not_retain_the_hook_owner() {
    let trace = Arc::new(SharedState::new(Vec::new()));
    let hooks = Arc::new(RecordingHooks {
        trace: trace.clone(),
    });
    let weak = Arc::downgrade(&hooks);
    let wrapper = DeWrapper::with_hooks(hooks);
    let copy = wrapper.default_copy();
    assert_eq!(copy.global_parameters().length_unit, 1.);
    assert!(
        !copy
            .find_provider(&path(), false, &mut Handle::default())
            .unwrap()
    );
    assert!(trace.read().is_empty());
    assert!(weak.upgrade().is_some());
    drop(wrapper);
    assert!(weak.upgrade().is_none());
}
#[test]
fn registry_and_default_callbacks_have_send_sync_contracts() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<DeWrapper>();
    require_send_sync::<martensite_tkde::de::DeDefaultWrapperHooks>();
}

fn exercise_isolated_registry() {
    let (wrapper, trace) = registry();
    for _ in 0..100 {
        assert!(
            !wrapper
                .find_provider(&path(), true, &mut Handle::default())
                .unwrap()
        );
    }
    assert_eq!(trace.read().len(), 100);
    assert!(trace.read().iter().all(|event| event == "find:true"));
}

#[test]
fn concurrent_registries_keep_callback_state_local() {
    let threads: Vec<_> = (0..8)
        .map(|_| std::thread::spawn(exercise_isolated_registry))
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}
