use super::*;

#[test]
fn direct_set_selected_retains_real_global_owner_and_repeat_identity() {
    let session = cpu_session();
    let context = session.context();
    let (_shape, object) = objects::anonymous_box(context).unwrap();
    let expected = object.global_sel_owner().object_address();
    for _ in 0..2 {
        context.set_selected_object(&object, true).unwrap();
        assert_eq!(context.nb_selected(), 1);
        context.init_selected();
        let selected = context.selected_owner().unwrap();
        assert_eq!(selected.object_address(), expected);
        assert_eq!(
            selected.selectable().object_address(),
            object.object_address()
        );
    }
}

#[test]
fn null_and_unregistered_object_cannot_invent_or_replace_selection() {
    let session = cpu_session();
    let context = session.context();
    selected_object::select_anonymous_box(context).unwrap();
    context.init_selected();
    let original = context.selected_owner().unwrap().object_address();
    let shape = crate::primitive_shapes::box_sized([2.; 3]).unwrap();
    let unregistered = AisShape::new(&shape).cast();
    context
        .set_selected_object(&Handle::default(), true)
        .unwrap();
    context.set_selected_object(&unregistered, true).unwrap();
    assert_eq!(context.nb_selected(), 1);
    context.init_selected();
    assert_eq!(context.selected_owner().unwrap().object_address(), original);
}
