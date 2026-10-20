//! Abort after every action combination:
//! every combination of create/modify/forget actions on one label and its sub-label in two
//! TDocStd_Document commands, the attributes checked after the second command and again
//! after it is aborted.
mod document_commands;
use document_commands::{
    add_label, attributes, entry, forget_all, forget_attribute, get_integer, get_real,
    new_document, set_integer, set_real,
};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::tdata_std::{TDataStdInteger, TDataStdReal};
use martensite_tklcaf::tdoc_std::TDocStdDocument;

// The action ids, in the scenario's order (first the actions that need no attribute
// on the label, then none, then those that need one).
const CREATE: i32 = 0;
const CREATE_FORGET: i32 = 1;
const CREATE_MODIFY: i32 = 2;
const NONE: i32 = 3;
const MODIFY: i32 = 4;
const MODIFY_FORGET: i32 = 5;
const FORGET: i32 = 6;
const FORGET_CREATE: i32 = 7;
const ACTIONS: i32 = 8;
const AFTER_NONE: i32 = NONE + 1;

/// `produces_attribute1`: an attribute exists after this action.
fn produces_attribute1(action: i32) -> bool {
    matches!(action, CREATE | CREATE_MODIFY | MODIFY | FORGET_CREATE)
}

/// `produces_attribute2`: an attribute exists after the two actions.
fn produces_attribute2(action1: i32, action2: i32) -> bool {
    if produces_attribute1(action2) {
        return true;
    }
    !matches!(action2, FORGET | CREATE_FORGET | MODIFY_FORGET) && produces_attribute1(action1)
}

/// `produces_value`: the value of the attribute produced by the two actions (its
/// `$action1==$act_none && $action1==$act_createmodify` branch can never hold).
fn produces_value(action1: i32, action2: i32) -> i32 {
    if action2 == MODIFY {
        return if action1 == CREATE_MODIFY { 3 } else { 2 };
    }
    if action2 == CREATE_MODIFY || (action1 == CREATE_MODIFY && action2 != FORGET_CREATE) {
        return 2;
    }
    1
}

/// One attribute kind of the case: 0 is TDataStd_Integer, 1 TDataStd_Real.
fn get_value(document: &Handle<TDocStdDocument>, label: &str, attr: i32) -> f64 {
    if attr == 0 {
        f64::from(get_integer(document, label).unwrap())
    } else {
        get_real(document, label).unwrap()
    }
}

fn set_value(document: &Handle<TDocStdDocument>, label: &str, attr: i32, value: f64) {
    if attr == 0 {
        set_integer(document, label, value as i32);
    } else {
        set_real(document, label, value);
    }
}

/// `ForgetAtt D label <guid of the kind>`.
fn forget(document: &Handle<TDocStdDocument>, label: &str, attr: i32) {
    let id = if attr == 0 {
        TDataStdInteger::get_id()
    } else {
        TDataStdReal::get_id()
    };
    forget_attribute(document, label, &id);
}

/// `do_action label attr action`.
fn do_action(document: &Handle<TDocStdDocument>, label: &str, attr: i32, action: i32) {
    match action {
        CREATE => set_value(document, label, attr, 1.0),
        CREATE_FORGET => {
            set_value(document, label, attr, 1.0);
            forget(document, label, attr);
        }
        CREATE_MODIFY => {
            set_value(document, label, attr, 1.0);
            set_value(document, label, attr, 2.0);
        }
        MODIFY | MODIFY_FORGET => {
            let value = get_value(document, label, attr);
            set_value(document, label, attr, value + 1.0);
            if action == MODIFY_FORGET {
                forget(document, label, attr);
            }
        }
        FORGET => forget(document, label, attr),
        FORGET_CREATE => {
            forget(document, label, attr);
            set_value(document, label, attr, 1.0);
        }
        _ => {}
    }
}

/// `check_attribute action1 action2 lab attr`.
fn check_attribute(
    document: &Handle<TDocStdDocument>,
    actions: (i32, i32),
    label: &str,
    attr: i32,
) {
    let name = if attr == 0 { "Integer" } else { "Real" };
    if produces_attribute2(actions.0, actions.1) {
        let value = get_value(document, label, attr);
        let expected = f64::from(produces_value(actions.0, actions.1));
        assert_eq!(
            value, expected,
            "Error : attribute {name} value at the label {label} {actions:?}"
        );
        return;
    }
    let present = attributes(document, label).contains(&format!("TDataStd_{name}"));
    assert!(
        !present,
        "Error : attribute {name} exists but it should not at the label {label} {actions:?}"
    );
}

/// The second-command range of one attribute: after an action that leaves an attribute,
/// the actions from `none` on; otherwise those up to `none`.
fn second_actions(first: i32) -> std::ops::Range<i32> {
    if produces_attribute1(first) {
        NONE..ACTIONS
    } else {
        0..AFTER_NONE
    }
}

/// One variant: three attributes (label integer, sub-label integer and real), two commands,
/// the checks after the second, an abort and the checks after it.
fn run_variant(
    document: &Handle<TDocStdDocument>,
    lab_index: i32,
    first: [i32; 3],
    second: [i32; 3],
) {
    let lab = entry(&add_label(document, &format!("0:{lab_index}")));
    forget_all(document, &lab);
    let sublab = entry(&add_label(document, &format!("{lab}:1")));
    forget_all(document, &sublab);
    let slots = [(&lab, 0), (&sublab, 0), (&sublab, 1)];
    document.new_command().unwrap();
    for (slot, action) in slots.iter().zip(first) {
        do_action(document, slot.0, slot.1, action);
    }
    document.commit_command().unwrap();
    document.new_command().unwrap();
    for (slot, action) in slots.iter().zip(second) {
        do_action(document, slot.0, slot.1, action);
    }
    for (index, slot) in slots.iter().enumerate() {
        check_attribute(document, (first[index], second[index]), slot.0, slot.1);
    }
    document.abort_command().unwrap();
    for (index, slot) in slots.iter().enumerate() {
        check_attribute(document, (NONE, first[index]), slot.0, slot.1);
    }
}

/// Runs every variant whose first actions are `first`; returns the next label index.
fn run_first_actions(
    document: &Handle<TDocStdDocument>,
    first: [i32; 3],
    lab_index: &mut i32,
) -> i32 {
    let mut variants = 0;
    for a in second_actions(first[0]) {
        for b in second_actions(first[1]) {
            for c in second_actions(first[2]) {
                run_variant(document, *lab_index, first, [a, b, c]);
                variants += 1;
                *lab_index = if *lab_index == 10 { 1 } else { *lab_index + 1 };
            }
        }
    }
    variants
}

/// `Checked 5832 variants` without an error line.
#[test]
fn every_create_modify_forget_combination_aborts_cleanly() {
    let document = new_document();
    document.set_undo_limit(100).unwrap();
    let (mut lab_index, mut variants) = (1, 0);
    for a in 0..AFTER_NONE {
        for b in 0..AFTER_NONE {
            for c in 0..AFTER_NONE {
                variants += run_first_actions(&document, [a, b, c], &mut lab_index);
            }
        }
    }
    assert_eq!(variants, 5832);
}
