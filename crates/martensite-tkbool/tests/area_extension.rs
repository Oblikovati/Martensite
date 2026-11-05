//! External composition retains native list nodes without exposing raw builder fields.
use martensite_tkbool::top_ope_brep_build::{
    AreaExtensionState, AreaExtensionStateMut, AreaList, AreaLoopList,
    TopOpeBRepBuildAreaBuilder as Builder, TopOpeBRepBuildLoop, TopOpeBRepBuildLoopBase,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{ncollection::ListCursor, standard::Handle};
use martensite_tkg3d::top_abs::State;

fn loop_handle() -> Handle<dyn TopOpeBRepBuildLoop> {
    Handle::new(TopOpeBRepBuildLoopBase::from_shape(&Shape::default())).cast()
}
fn area(handle: &Handle<dyn TopOpeBRepBuildLoop>) -> AreaLoopList {
    [handle.clone()].into_iter().collect()
}
fn same(left: &Handle<dyn TopOpeBRepBuildLoop>, right: &Handle<dyn TopOpeBRepBuildLoop>) {
    assert!(std::ptr::eq(left.get().unwrap(), right.get().unwrap()));
}
#[test]
fn views_are_send_sync_and_read_the_same_storage_they_mutate() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<Builder>();
    send_sync::<AreaExtensionState<'static>>();
    send_sync::<AreaExtensionStateMut<'static>>();
    let mut builder = Builder::new();
    let pointer = builder.area_extension_state().areas() as *const AreaList;
    assert_eq!(
        pointer,
        builder.area_extension_state_mut().areas_mut() as *const AreaList
    );
    let first = loop_handle();
    builder
        .area_extension_state_mut()
        .areas_mut()
        .append(area(&first));
    assert_eq!(builder.init_area(), 1);
    same(builder.loop_().unwrap(), &first);
}
#[test]
fn live_prepend_and_append_preserve_current_and_exhausted_positions() {
    let (first, second) = (loop_handle(), loop_handle());
    let mut builder = Builder::new();
    builder
        .area_extension_state_mut()
        .areas_mut()
        .append(area(&first));
    builder.init_area();
    builder
        .area_extension_state_mut()
        .areas_mut()
        .prepend(area(&second));
    builder.init_loop();
    same(builder.loop_().unwrap(), &first);
    builder
        .area_extension_state_mut()
        .loop_cursor_source_mut()
        .unwrap()
        .prepend(second);
    same(builder.loop_().unwrap(), &first);
    builder.next_loop();
    assert!(!builder.more_loop());
    builder
        .area_extension_state_mut()
        .loop_cursor_source_mut()
        .unwrap()
        .append(loop_handle());
    assert!(!builder.more_loop());
}
#[test]
fn owned_area_rebinding_preserves_node_identity_and_independent_loop_cursor() {
    let (first, other) = (loop_handle(), loop_handle());
    let mut builder = Builder::new();
    builder
        .area_extension_state_mut()
        .areas_mut()
        .append(area(&first));
    builder.init_area();
    let foreign: AreaList = [area(&other)].into_iter().collect();
    let cursor = foreign.cursor();
    builder
        .area_extension_state_mut()
        .bind_area_cursor(foreign, cursor)
        .unwrap();
    assert_eq!(
        builder.area_extension_state().area_cursor().current_node(),
        cursor.current_node()
    );
    same(builder.loop_().unwrap(), &first);
    builder.init_loop();
    same(builder.loop_().unwrap(), &other);
    assert_eq!(builder.area_extension_state().areas().extent(), 1);
}
#[test]
fn owned_loop_rebinding_survives_replacing_area_source_and_observes_node_updates() {
    let (first, other) = (loop_handle(), loop_handle());
    let mut builder = Builder::new();
    let foreign = area(&first);
    let cursor = foreign.cursor();
    builder
        .area_extension_state_mut()
        .bind_loop_cursor(foreign, cursor)
        .unwrap();
    assert_eq!(
        builder.area_extension_state().loop_cursor().current_node(),
        cursor.current_node()
    );
    *builder
        .area_extension_state_mut()
        .loop_cursor_source_mut()
        .unwrap()
        .first_mut()
        .unwrap() = other.clone();
    builder.area_extension_state_mut().areas_mut().clear();
    same(builder.loop_().unwrap(), &other);
    builder.next_loop();
    assert!(!builder.more_loop());
    builder
        .area_extension_state_mut()
        .loop_cursor_source_mut()
        .unwrap()
        .append(first);
    assert!(!builder.more_loop());
}
#[test]
fn cursor_reassignment_and_copy_do_not_follow_area_ordinals() {
    let (first, second) = (loop_handle(), loop_handle());
    let mut builder = Builder::new();
    builder
        .area_extension_state_mut()
        .areas_mut()
        .append([first, second.clone()].into_iter().collect());
    builder.init_area();
    builder.next_loop();
    let copy = builder.clone();
    same(copy.loop_().unwrap(), &second);
    assert_ne!(
        copy.area_extension_state().loop_cursor().current_node(),
        builder.area_extension_state().loop_cursor().current_node()
    );
    *builder.area_extension_state_mut().loop_cursor_mut() = ListCursor::default();
    assert!(!builder.more_loop());
    same(copy.loop_().unwrap(), &second);
    *builder.area_extension_state_mut().area_cursor_mut() = ListCursor::default();
    assert!(!builder.more_area());
}
#[test]
fn unknown_policy_changes_atomize_and_invalid_owner_is_rejected() {
    let mut builder = Builder::new();
    let mut state = State::Unknown;
    *builder.area_extension_state_mut().unknown_raises_mut() = true;
    assert!(*builder.area_extension_state().unknown_raises());
    assert!(builder.atomize(&mut state, State::In).is_err());
    *builder.area_extension_state_mut().unknown_raises_mut() = false;
    builder.atomize(&mut state, State::In).unwrap();
    assert_eq!(state, State::In);
    let first = area(&loop_handle());
    let second = area(&loop_handle());
    let wrong = first.cursor();
    assert!(
        builder
            .area_extension_state_mut()
            .bind_loop_cursor(second, wrong)
            .is_err()
    );
}

#[test]
fn empty_owned_sources_can_be_edited_and_reinitialized_without_copying() {
    let first = loop_handle();
    let mut builder = Builder::new();
    builder
        .area_extension_state_mut()
        .bind_area_cursor(AreaList::default(), ListCursor::default())
        .unwrap();
    builder
        .area_extension_state_mut()
        .area_cursor_source_mut()
        .unwrap()
        .append(area(&first));
    assert!(!builder.more_area());
    let cursor = builder
        .area_extension_state()
        .area_cursor_source()
        .unwrap()
        .cursor();
    *builder.area_extension_state_mut().area_cursor_mut() = cursor;
    builder.init_loop();
    same(builder.loop_().unwrap(), &first);
    builder
        .area_extension_state_mut()
        .bind_loop_cursor(AreaLoopList::default(), ListCursor::default())
        .unwrap();
    builder
        .area_extension_state_mut()
        .loop_cursor_source_mut()
        .unwrap()
        .append(first.clone());
    assert!(!builder.more_loop());
    let cursor = builder
        .area_extension_state()
        .loop_cursor_source()
        .unwrap()
        .cursor();
    *builder.area_extension_state_mut().loop_cursor_mut() = cursor;
    same(builder.loop_().unwrap(), &first);
}
