//! External composition edits loop state before publishing it through the loop trait.
use martensite_tkbool::top_ope_brep_build::{
    LoopExtensionState, LoopExtensionStateRef, TopOpeBRepBuildBlockIterator, TopOpeBRepBuildLoop,
    TopOpeBRepBuildLoopBase,
};
use martensite_tkbrep::topods::{Builder, Compound, Shape};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::Handle;

struct ComposedLoop {
    base: TopOpeBRepBuildLoopBase,
}
impl_transient!(ComposedLoop, "ComposedLoop", TopOpeBRepBuildLoopBase, base, interfaces: [dyn TopOpeBRepBuildLoop]);

impl ComposedLoop {
    fn retained_mode(&self) -> &bool {
        self.base.extension_state_ref().shape_mode()
    }
    fn retained_shape(&self) -> &Shape {
        self.base.extension_state_ref().shape()
    }
    fn retained_cursor(&self) -> &TopOpeBRepBuildBlockIterator {
        self.base.extension_state_ref().block_cursor()
    }
}

impl TopOpeBRepBuildLoop for ComposedLoop {
    fn is_shape(&self) -> bool {
        self.base.is_shape()
    }
    fn shape(&self) -> Shape {
        self.base.shape()
    }
    fn block_iterator(&self) -> TopOpeBRepBuildBlockIterator {
        self.base.block_iterator()
    }
}

fn distinct_shapes() -> [Shape; 2] {
    let mut first = Compound::default();
    let mut second = Compound::default();
    Builder.make_compound(&mut first);
    Builder.make_compound(&mut second);
    [first.into_shape(), second.into_shape()]
}

fn select_block(loop_record: &mut TopOpeBRepBuildLoopBase, replacement: &Shape) {
    let mut state = loop_record.extension_state();
    *state.shape_mode_mut() = false;
    *state.shape_mut() = replacement.clone();
    *state.block_cursor_mut() = TopOpeBRepBuildBlockIterator::new(3, 5);
    state.block_cursor_mut().next();
}

#[test]
fn external_composition_publishes_the_edited_mode_shape_and_cursor() {
    let [original, replacement] = distinct_shapes();
    let mut extension = ComposedLoop {
        base: TopOpeBRepBuildLoopBase::from_shape(&original),
    };
    select_block(&mut extension.base, &replacement);
    let published: Handle<dyn TopOpeBRepBuildLoop> = Handle::new(extension).cast();
    let record = published.get().expect("published loop");
    assert!(!record.is_shape());
    assert!(record.shape().is_same(&replacement));
    assert!(!record.shape().is_same(&original));
    assert_eq!(record.block_iterator().value(), 4);
    assert_eq!(record.block_iterator().extent(), 3);
}

#[test]
fn mode_switch_preserves_payloads_and_mutation_is_local_to_the_selected_record() {
    let [original, replacement] = distinct_shapes();
    let mut selected = TopOpeBRepBuildLoopBase::from_shape(&original);
    let untouched = TopOpeBRepBuildLoopBase::from_shape(&original);
    select_block(&mut selected, &replacement);
    *selected.extension_state().shape_mode_mut() = true;
    assert!(selected.is_shape());
    assert!(selected.shape().is_same(&replacement));
    assert_eq!(selected.block_iterator().value(), 4);
    assert!(untouched.is_shape());
    assert!(untouched.shape().is_same(&original));
    assert_eq!(untouched.block_iterator().value(), 0);
}

#[test]
fn cursor_rebinding_uses_the_live_retained_cursor_without_a_snapshot() {
    let cursor = TopOpeBRepBuildBlockIterator::new(7, 9);
    let mut record = TopOpeBRepBuildLoopBase::from_block(&cursor);
    record.extension_state().block_cursor_mut().next();
    assert_eq!(record.block_iterator().value(), 8);
    assert_eq!(cursor.value(), 7);
    record.extension_state().block_cursor_mut().initialize();
    assert_eq!(record.block_iterator().value(), 7);
    assert!(!record.is_shape());
    assert!(record.shape().is_null());
}

#[test]
fn extension_view_and_composed_loop_preserve_thread_traits() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<LoopExtensionState<'static>>();
    assert_send_sync::<LoopExtensionStateRef<'static>>();
    assert_send_sync::<TopOpeBRepBuildLoopBase>();
    assert_send_sync::<ComposedLoop>();
}

fn retained_addresses(
    record: &mut TopOpeBRepBuildLoopBase,
) -> (
    *const bool,
    *const Shape,
    *const TopOpeBRepBuildBlockIterator,
) {
    let mut state = record.extension_state();
    (
        std::ptr::from_mut(state.shape_mode_mut()).cast_const(),
        std::ptr::from_mut(state.shape_mut()).cast_const(),
        std::ptr::from_mut(state.block_cursor_mut()).cast_const(),
    )
}

#[test]
fn external_const_access_borrows_the_same_fields_as_mutable_extension_access() {
    let [original, replacement] = distinct_shapes();
    let mut extension = ComposedLoop {
        base: TopOpeBRepBuildLoopBase::from_shape(&original),
    };
    select_block(&mut extension.base, &replacement);
    let (mode, shape, cursor) = retained_addresses(&mut extension.base);
    assert!(std::ptr::eq(extension.retained_mode(), mode));
    assert!(std::ptr::eq(extension.retained_shape(), shape));
    assert!(std::ptr::eq(extension.retained_cursor(), cursor));
    assert!(!*extension.retained_mode());
    assert!(extension.retained_shape().is_same(&replacement));
    assert_eq!(extension.retained_cursor().value(), 4);
}

#[test]
fn shared_views_reborrow_updated_fields_instead_of_snapshot_payloads() {
    let [original, replacement] = distinct_shapes();
    let mut extension = ComposedLoop {
        base: TopOpeBRepBuildLoopBase::from_shape(&original),
    };
    let shape_address = std::ptr::from_ref(extension.retained_shape());
    let cursor_address = std::ptr::from_ref(extension.retained_cursor());
    assert!(extension.retained_shape().is_same(&original));
    assert_eq!(extension.retained_cursor().value(), 0);
    select_block(&mut extension.base, &replacement);
    assert!(std::ptr::eq(extension.retained_shape(), shape_address));
    assert!(std::ptr::eq(extension.retained_cursor(), cursor_address));
    assert!(extension.retained_shape().is_same(&replacement));
    assert_eq!(extension.retained_cursor().value(), 4);
}
