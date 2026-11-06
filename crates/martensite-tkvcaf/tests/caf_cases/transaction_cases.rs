//! Every combination of attribute creation, change and removal, inside and outside transactions,
//! checked after the commit, the undo and the redo. The variant loops are the scripts' own.
use crate::session::commands::CafSession;
use crate::session::hooks::ImageCheck::None as NoPicture;
use crate::session::transactions::Attr;

const CREATE: i32 = 0;
const CREATE_FORGET: i32 = 1;
const CREATE_MODIFY: i32 = 2;
const NONE: i32 = 3;
const MODIFY: i32 = 4;
const MODIFY_FORGET: i32 = 5;
const FORGET: i32 = 6;
const FORGET_CREATE: i32 = 7;
/// The number of actions an attribute can take.
const ACTIONS: i32 = 8;
const AFTER_NONE: i32 = NONE + 1;
/// A first action whose attribute the undo took away; its label then holds none.
const UNDONE: i32 = 100;

/// Whether an attribute is on the label after this one action.
fn produces_one(action: i32) -> bool {
    matches!(action, CREATE | CREATE_MODIFY | MODIFY | FORGET_CREATE)
}

fn produces_two(first: i32, second: i32) -> bool {
    if first == UNDONE && (second == MODIFY || second == FORGET_CREATE) {
        return false;
    }
    if produces_one(second) {
        return true;
    }
    !matches!(second, FORGET | CREATE_FORGET | MODIFY_FORGET) && produces_one(first)
}

/// The value of the attribute two actions leave.
fn produced_value(first: i32, second: i32) -> i32 {
    if second == MODIFY {
        return if first == CREATE_MODIFY { 3 } else { 2 };
    }
    if second == CREATE_MODIFY || (first == CREATE_MODIFY && second != FORGET_CREATE) {
        return 2;
    }
    1
}

fn incremented(s: &CafSession, label: &str, attr: Attr) {
    let value = s.get_value(label, attr);
    s.set_value(label, attr, value + 1);
}

fn do_action(s: &CafSession, label: &str, id: i32, action: i32) {
    let attr = Attr::of(id);
    match action {
        CREATE => s.set_value(label, attr, 1),
        CREATE_FORGET => {
            s.set_value(label, attr, 1);
            s.forget_attribute(label, attr);
        }
        CREATE_MODIFY => {
            s.set_value(label, attr, 1);
            s.set_value(label, attr, 2);
        }
        MODIFY => incremented(s, label, attr),
        MODIFY_FORGET => {
            incremented(s, label, attr);
            s.forget_attribute(label, attr);
        }
        FORGET => s.forget_attribute(label, attr),
        FORGET_CREATE => {
            s.forget_attribute(label, attr);
            s.set_value(label, attr, 1);
        }
        _ => {}
    }
}

fn holds_no(s: &CafSession, label: &str, attr: Attr) {
    let wanted = format!("TDataStd_{}", attr.name());
    if s.attributes(label).contains(&wanted) {
        let name = attr.name();
        println!("Error : attribute {name} exists but it should not at the label {label}");
    }
}

fn check_value(s: &CafSession, label: &str, attr: Attr, first: i32, second: i32) {
    let value = s.get_value(label, attr);
    let expected = produced_value(first, second);
    if value != expected {
        let name = attr.name();
        println!(
            "Error : attribute {name} value {value} does not match the expected {expected} at the label {label}"
        );
    }
}

fn check_attribute(s: &CafSession, first: i32, second: i32, label: &str, id: i32) {
    let attr = Attr::of(id);
    if produces_two(first, second) {
        check_value(s, label, attr, first, second);
    } else {
        holds_no(s, label, attr);
    }
}

/// After a transaction, no transaction, undo and redo: changes come back, removals may not.
fn check_without_transaction(s: &CafSession, first: i32, second: i32, label: &str, id: i32) {
    let attr = Attr::of(id);
    if produces_two(first, second) {
        check_value(s, label, attr, first, second);
    } else if !produces_one(second) {
        holds_no(s, label, attr);
    }
}

fn changes_nothing(action: i32) -> bool {
    action == NONE || action == CREATE_FORGET
}

/// The range a second action may take: an existing attribute takes the ones that need it.
fn second_actions(first: i32) -> std::ops::Range<i32> {
    if produces_one(first) {
        NONE..ACTIONS
    } else {
        0..AFTER_NONE
    }
}

/// A variant of father and child labels with two transactions, per action of each attribute.
struct FamilyVariant {
    first: [i32; 3],
    second: [i32; 3],
}

impl FamilyVariant {
    fn check_all(&self, s: &CafSession, labels: [&str; 3], first: [i32; 3], second: [i32; 3]) {
        for (index, id) in [0, 0, 1].into_iter().enumerate() {
            check_attribute(s, first[index], second[index], labels[index], id);
        }
    }

    fn apply(&self, s: &CafSession, labels: [&str; 3], actions: [i32; 3]) {
        s.new_command();
        for (index, id) in [0, 0, 1].into_iter().enumerate() {
            do_action(s, labels[index], id, actions[index]);
        }
        s.commit_command();
    }

    fn second_changes_nothing(&self) -> bool {
        self.second.into_iter().all(changes_nothing)
    }

    fn run(&self, s: &CafSession, label: &str, child: &str) {
        let labels = [label, child, child];
        self.apply(s, labels, self.first);
        self.apply(s, labels, self.second);
        self.check_all(s, labels, self.first, self.second);
        let undone = [NONE; 3];
        if !self.second_changes_nothing() {
            s.undo();
        }
        self.check_all(s, labels, undone, self.first);
        s.undo();
        s.redo();
        self.check_all(s, labels, undone, self.first);
        if !self.second_changes_nothing() {
            s.redo();
        }
        self.check_all(s, labels, self.first, self.second);
    }
}

/// Runs `body` for every second action of three attributes.
fn each_second(first: [i32; 3], mut body: impl FnMut([i32; 3])) {
    for a in second_actions(first[0]) {
        for b in second_actions(first[1]) {
            for c in second_actions(first[2]) {
                body([a, b, c]);
            }
        }
    }
}

fn run_family_variants(s: &CafSession) -> i32 {
    let (mut index, mut variants) = (1, 0);
    for a in 0..AFTER_NONE {
        for b in 0..AFTER_NONE {
            for c in 0..AFTER_NONE {
                let first = [a, b, c];
                each_second(first, |second| {
                    let label = s.label(&format!("0:{index}"));
                    s.forget_all(&label);
                    let child = s.label(&format!("{label}:1"));
                    s.forget_all(&child);
                    index += 1;
                    variants += 1;
                    // Too many labels would be too slow, as the script says.
                    if index == 11 {
                        index = 1;
                    }
                    FamilyVariant { first, second }.run(s, &label, &child);
                });
            }
        }
    }
    variants
}

caf_case!(
    bug31748_2,
    "tests/bugs/caf/bug31748_2",
    BUGS,
    NoPicture,
    |s| {
        println!("===========");
        println!("0031748: Application Framework - Efficient OCAF transactions");
        println!("===========");
        s.new_document(None);
        s.undo_limit(100);
        let variants = run_family_variants(s);
        println!("Checked {variants} variants");
    }
);

/// Each attribute's actions, given a whether-first-holds rule.
fn each_pair(mut body: impl FnMut([i32; 2], [i32; 2])) {
    for a in 0..AFTER_NONE {
        for b in 0..AFTER_NONE {
            for c in second_actions(a) {
                for d in second_actions(b) {
                    body([a, b], [c, d]);
                }
            }
        }
    }
}

fn pair_changes(actions: [i32; 2]) -> bool {
    !changes_nothing(actions[0]) || !changes_nothing(actions[1])
}

fn apply_pair(s: &CafSession, label: &str, actions: [i32; 2]) {
    do_action(s, label, 0, actions[0]);
    do_action(s, label, 1, actions[1]);
}

fn check_pair(s: &CafSession, label: &str, first: [i32; 2], second: [i32; 2]) {
    check_attribute(s, first[0], second[0], label, 0);
    check_attribute(s, first[1], second[1], label, 1);
}

fn no_transaction_then_transaction(s: &CafSession, index: &mut i32) {
    let mut variants = 0;
    each_pair(|first, second| {
        let label = s.label(&format!("0:{index}"));
        s.forget_all(&label);
        *index += 1;
        variants += 1;
        apply_pair(s, &label, first);
        s.new_command();
        apply_pair(s, &label, second);
        s.commit_command();
        check_pair(s, &label, first, second);
        if pair_changes(second) {
            s.undo();
            check_pair(s, &label, [NONE; 2], first);
            s.redo();
            check_pair(s, &label, first, second);
        }
    });
    println!("Checked {variants} variants in cycles no transaction->transaction");
}

/// What the undo leaves of a first action done in the undone transaction.
fn after_undo(first: i32) -> i32 {
    if first == CREATE_FORGET || first == NONE {
        first
    } else {
        UNDONE
    }
}

fn transaction_then_no_transaction(s: &CafSession, index: &mut i32) {
    let mut variants = 0;
    each_pair(|first, second| {
        let label = s.label(&format!("0:{index}"));
        *index += 1;
        variants += 1;
        s.new_command();
        apply_pair(s, &label, first);
        s.commit_command();
        apply_pair(s, &label, second);
        check_pair(s, &label, first, second);
        if pair_changes(second) {
            s.undo();
            check_pair(s, &label, first.map(after_undo), second);
            s.redo();
            check_without_transaction(s, first[0], second[0], &label, 0);
            check_without_transaction(s, first[1], second[1], &label, 1);
        }
    });
    println!("Checked {variants} variants in cycles transaction->no transaction");
}

caf_case!(
    bug31748_3,
    "tests/bugs/caf/bug31748_3",
    BUGS,
    NoPicture,
    |s| {
        println!("===========");
        println!("0031748: Application Framework - Efficient OCAF transactions");
        println!("===========");
        s.new_document(None);
        s.undo_limit(100);
        let mut index = 1;
        no_transaction_then_transaction(s, &mut index);
        transaction_then_no_transaction(s, &mut index);
    }
);
