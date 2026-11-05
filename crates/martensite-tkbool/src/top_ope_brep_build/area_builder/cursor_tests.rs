use super::*;
use crate::top_ope_brep_build::TopOpeBRepBuildLoopBase;
use martensite_tkbrep::topods::Shape;

fn loop_handle() -> Handle<dyn TopOpeBRepBuildLoop> {
    Handle::new(TopOpeBRepBuildLoopBase::from_shape(&Shape::default())).cast()
}
fn same_loop(left: &Handle<dyn TopOpeBRepBuildLoop>, right: &Handle<dyn TopOpeBRepBuildLoop>) {
    assert!(std::ptr::eq(left.get().unwrap(), right.get().unwrap()));
}
fn builder_with(first: &Handle<dyn TopOpeBRepBuildLoop>) -> TopOpeBRepBuildAreaBuilder {
    let area = [first.clone()].into_iter().collect();
    TopOpeBRepBuildAreaBuilder {
        my_area: [area].into_iter().collect(),
        ..Default::default()
    }
}
#[test]
fn prepending_an_area_does_not_rebind_current_area_node() {
    let first = loop_handle();
    let inserted = loop_handle();
    let mut builder = builder_with(&first);
    assert_eq!(builder.init_area(), 1);
    builder.my_area.prepend([inserted].into_iter().collect());
    builder.init_loop();
    same_loop(builder.loop_().unwrap(), &first);
}
#[test]
fn prepending_a_loop_does_not_rebind_current_loop_node() {
    let first = loop_handle();
    let mut builder = builder_with(&first);
    builder.init_area();
    builder.my_area.first_mut().unwrap().prepend(loop_handle());
    same_loop(builder.loop_().unwrap(), &first);
}
#[test]
fn exhausted_loop_cursor_does_not_reappear_after_append() {
    let first = loop_handle();
    let mut builder = builder_with(&first);
    builder.init_area();
    builder.next_loop();
    assert!(!builder.more_loop());
    builder.my_area.first_mut().unwrap().append(loop_handle());
    assert!(!builder.more_loop());
}
#[test]
fn ordinary_nested_traversal_keeps_order_and_counts() {
    let first = loop_handle();
    let second = loop_handle();
    let mut builder = builder_with(&first);
    builder
        .my_area
        .append([second.clone()].into_iter().collect());
    assert_eq!(builder.init_area(), 2);
    same_loop(builder.loop_().unwrap(), &first);
    builder.next_area();
    assert_eq!(builder.init_loop(), 1);
    same_loop(builder.loop_().unwrap(), &second);
    builder.next_area();
    assert!(!builder.more_area());
    assert!(!builder.more_loop());
}
