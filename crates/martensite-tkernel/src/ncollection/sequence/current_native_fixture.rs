//! Real sequence metadata and live resource replacement for the heap fixture.
#![allow(unsafe_code)]
use crate::ncollection::{
    AllocatorDispatchCode, AllocatorInvocation, AllocatorObject, AllocatorSource, BaseAllocator,
    HeapAllocatorObject, Sequence,
};
use crate::standard::{Failure, Handle, SharedState, WeakHandle};
use std::{
    ptr::NonNull,
    sync::{Arc, Weak},
};
#[derive(Default)]
pub(super) struct NativeSequenceScene {
    pub(super) sequence: SharedState<Sequence<i32>>,
    pub(super) rows: SharedState<Vec<String>>,
    pub(super) retired: SharedState<bool>,
}
#[derive(Default)]
pub(super) struct CurrentSequenceHeap {
    parent: HeapAllocatorObject,
    scene: Weak<NativeSequenceScene>,
}
crate::impl_transient!(CurrentSequenceHeap,"Martensite_CurrentSequenceHeap",
    HeapAllocatorObject,interfaces:[dyn AllocatorSource]);
// SAFETY: both actual derived resources share the genuine compatible heap backend.
unsafe impl AllocatorSource for CurrentSequenceHeap {
    fn allocator_object_base(&self) -> &AllocatorObject {
        // The actual parent keeps allocation provenance and intrusive identity together.
        self.parent.allocator_object_base()
    }
    fn dispatch_code(&self) -> AllocatorDispatchCode {
        AllocatorDispatchCode {
            free: current_sequence_free,
            ..Default::default()
        }
    }
}
#[derive(Default)]
pub(super) struct ReplacingSequenceHeap {
    parent: HeapAllocatorObject,
    scene: Weak<NativeSequenceScene>,
    current: WeakHandle<CurrentSequenceHeap>,
}
crate::impl_transient!(ReplacingSequenceHeap,"Martensite_ReplacingSequenceHeap",
    HeapAllocatorObject,interfaces:[dyn AllocatorSource]);
// SAFETY: allocation always delegates to the genuine embedded heap backend.
unsafe impl AllocatorSource for ReplacingSequenceHeap {
    fn allocator_object_base(&self) -> &AllocatorObject {
        // This inherited base retains physical allocation provenance after owner loss.
        self.parent.allocator_object_base()
    }
    fn dispatch_code(&self) -> AllocatorDispatchCode {
        AllocatorDispatchCode {
            allocate: replacing_sequence_allocate,
            ..Default::default()
        }
    }
}
impl ReplacingSequenceHeap {
    fn record(&self, scene: &NativeSequenceScene, phase: &str, owners: usize) {
        let prefix = format!("{phase} {} {}", scene.sequence.read().length(), owners);
        let row = if phase == "allocate" {
            prefix
        } else {
            format!("{prefix} {}", self.current.ref_count())
        };
        scene.rows.write().push(row);
    }
}
impl Drop for ReplacingSequenceHeap {
    fn drop(&mut self) {
        let scene = self.scene.upgrade().unwrap();
        *scene.retired.write() = true;
        // Size comes from the actual sequence, never a parallel synthetic counter.
        self.record(&scene, "retired", 0);
    }
}
pub(super) fn create_sequence_resources(
    scene: &Arc<NativeSequenceScene>,
) -> (Handle<CurrentSequenceHeap>, Handle<ReplacingSequenceHeap>) {
    let current = Handle::new(CurrentSequenceHeap {
        scene: Arc::downgrade(scene),
        ..Default::default()
    });
    let original = Handle::new(ReplacingSequenceHeap {
        scene: Arc::downgrade(scene),
        current: current.downgrade(),
        parent: HeapAllocatorObject::default(),
    });
    install_sequence_resource(scene, &original);
    (current, original)
}

fn install_sequence_resource(
    scene: &NativeSequenceScene,
    original: &Handle<ReplacingSequenceHeap>,
) {
    // The real sequence owns one genuine source; physical node leases own none.
    scene
        .sequence
        .write()
        .clear(Some(BaseAllocator::from_handle(original)));
}

fn current_sequence_scene(source: &CurrentSequenceHeap) -> Arc<NativeSequenceScene> {
    source.scene.upgrade().unwrap()
}
fn record_current_sequence_free(scene: &NativeSequenceScene, owners: usize) {
    scene.rows.write().push(format!(
        "current-free {} {} {}",
        scene.sequence.read().length(),
        owners,
        i32::from(*scene.retired.read())
    ));
}
unsafe fn current_sequence_free(
    invocation: &AllocatorInvocation,
    pointer: *mut u8,
) -> Result<(), Failure> {
    let owners = invocation.ref_count();
    // SAFETY: plain trace mutation has no callbacks; current accepts restored original heap provenance.
    let scene = crate::standard::raise_native_failure(unsafe {
        invocation.capture(current_sequence_scene)
    });
    record_current_sequence_free(&scene, owners);
    // SAFETY: the caller passes a block this allocator's base allocated, which free_base
    // requires.
    unsafe { invocation.free_base(pointer) }
}
fn capture_replacing_sequence(
    source: &ReplacingSequenceHeap,
) -> (Arc<NativeSequenceScene>, WeakHandle<CurrentSequenceHeap>) {
    (source.scene.upgrade().unwrap(), source.current.clone())
}
fn current_sequence_pointer(source: &ReplacingSequenceHeap) -> WeakHandle<CurrentSequenceHeap> {
    source.current.clone()
}
fn replacing_sequence_allocate(
    invocation: &AllocatorInvocation,
    size: usize,
) -> Option<NonNull<u8>> {
    // SAFETY: selectors read actual fields and trace values; all resource callbacks run afterward.
    let (scene, current) = crate::standard::raise_native_failure(unsafe {
        invocation.capture(capture_replacing_sequence)
    });
    scene.rows.write().push(format!(
        "allocate {} {}",
        scene.sequence.read().length(),
        invocation.ref_count()
    ));
    let replacement = BaseAllocator::from_handle(&current.upgrade());
    scene.sequence.write().clear(Some(replacement));
    // SAFETY: record_replaced_sequence only writes the trace and runs no callback, so the
    // receiver stays alive while it is captured.
    let current = crate::standard::raise_native_failure(unsafe {
        invocation.capture(current_sequence_pointer)
    });
    scene.rows.write().push(format!(
        "replaced {} {} {}",
        scene.sequence.read().length(),
        invocation.ref_count(),
        current.ref_count()
    ));
    invocation.allocate_base(size)
}
