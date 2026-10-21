use super::{AreaLimit, EdgeBuilder, State};
use crate::hlr_algo::Intersection;
use martensite_tkernel::standard::{Handle, WeakHandle};

fn owned_limit() -> Handle<AreaLimit> {
    Handle::new(AreaLimit::new(
        &Intersection::new(),
        true,
        false,
        State::Out,
        State::In,
        State::Out,
        State::In,
    ))
}
fn cyclic_builder() -> (EdgeBuilder, WeakHandle<AreaLimit>, WeakHandle<AreaLimit>) {
    let first = owned_limit();
    let second = owned_limit();
    first.set_next(&second);
    first.set_previous(&second);
    second.set_next(&first);
    second.set_previous(&first);
    let weak = (first.downgrade(), second.downgrade());
    let builder = EdgeBuilder {
        to_build: State::In,
        limits: first.clone(),
        left: first,
        right: second,
        current: 1,
    };
    (builder, weak.0, weak.1)
}
#[test]
fn builder_drop_breaks_both_strong_cycle_links_without_external_owners() {
    let (builder, first, second) = cyclic_builder();
    assert!(!first.upgrade().is_null());
    assert!(!second.upgrade().is_null());
    #[allow(
        drop_bounds,
        reason = "requires an explicit destructor body, not only owned fields"
    )]
    fn explicit_drop<T: Drop>() {}
    explicit_drop::<EdgeBuilder>();
    drop(builder);
    assert!(first.upgrade().is_null());
    assert!(second.upgrade().is_null());
}
#[test]
fn retained_limit_survives_builder_drop_with_states_intact_and_links_cleared() {
    let (builder, first, second) = cyclic_builder();
    let retained = first.upgrade();
    drop(builder);
    assert!(retained.previous().is_null() && retained.next().is_null());
    assert_eq!(retained.state_before(), State::Out);
    assert_eq!(retained.state_after(), State::In);
    assert!(second.upgrade().is_null());
    drop(retained);
    assert!(first.upgrade().is_null());
}
#[test]
fn destroy_then_drop_and_unwind_each_release_cyclic_limits_once() {
    let (mut builder, first, second) = cyclic_builder();
    builder.destroy();
    builder.destroy();
    drop(builder);
    assert!(first.upgrade().is_null() && second.upgrade().is_null());
    let (builder, first, second) = cyclic_builder();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let _owner = builder;
            panic!("builder ownership unwind");
        }))
        .is_err()
    );
    assert!(first.upgrade().is_null() && second.upgrade().is_null());
}
