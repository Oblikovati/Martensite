use super::*;

#[test]
fn classification_flags_remain_independent_and_drive_existing_query() {
    let mut selected = builder();
    let untouched = builder();
    *selected.extension_state_mut().classification_value_mut() = false;
    assert!(!selected.classify());
    assert!(!selected.extension_state().classification_defined());
    *selected.extension_state_mut().classification_defined_mut() = true;
    assert!(*selected.extension_state().classification_defined());
    assert!(!selected.extension_state().classification_value());
    assert!(!untouched.extension_state().classification_defined());
    assert!(*untouched.extension_state().classification_value());
    selected.change_classify(true);
    assert!(*selected.extension_state().classification_value());
}

#[test]
fn operation_control_slots_keep_independent_values() {
    let mut selected = builder();
    let untouched = builder();
    let mut edit = selected.extension_state_mut();
    *edit.special_case_mut() = -17;
    *edit.shape_set_index_mut() = 23;
    *edit.process_on_mut() = true;
    edit.diagnostic_name_mut().push_str("extension");
    assert_eq!(*selected.extension_state().special_case(), -17);
    assert_eq!(*selected.extension_state().shape_set_index(), 23);
    assert!(*selected.extension_state().process_on());
    assert_eq!(selected.extension_state().diagnostic_name(), "extension");
    assert_eq!(*untouched.extension_state().special_case(), 0);
    assert_eq!(*untouched.extension_state().shape_set_index(), 0);
    assert!(!untouched.extension_state().process_on());
    assert!(untouched.extension_state().diagnostic_name().is_empty());
}

#[test]
fn section_completion_controls_existing_section_query_and_reset() {
    let mut selected = builder();
    let retained = distinct_shape();
    let mut edit = selected.extension_state_mut();
    edit.section_shapes_mut().append(retained.clone());
    *edit.section_complete_mut() = true;
    *edit.section_edges_complete_mut() = true;
    let section = selected.section().unwrap();
    assert!(section.first().unwrap().is_same(&retained));
    assert_eq!(selected.extension_state().section_shapes().extent(), 1);
    assert!(*selected.extension_state().section_complete());
    assert!(*selected.extension_state().section_edges_complete());
    selected.init_section();
    assert!(selected.extension_state().section_shapes().is_empty());
    assert!(!selected.extension_state().section_complete());
    assert!(!selected.extension_state().section_edges_complete());
}

#[test]
fn classifier_view_mutates_the_retained_classifier_not_a_copy() {
    let mut selected = builder();
    let untouched = builder();
    selected
        .extension_state_mut()
        .classifier_mut()
        .set_same_domain(17);
    assert_eq!(selected.extension_state().classifier().same_domain(), 17);
    assert_eq!(untouched.extension_state().classifier().same_domain(), 0);
    selected
        .extension_state_mut()
        .classifier_mut()
        .clear_current();
    assert_eq!(selected.extension_state().classifier().same_domain(), -1);
}
