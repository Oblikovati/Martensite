//! Multi-document transactions: TDocStd_MultiTransactionManager synchronizes the transactions of several
//! documents, with nested transactions on (case 1) and off (case 2).
//!
//! The cases read the manager dump as a word list and check the index of each transaction name, the
//! document count after it and the index of `Last`; then every real attribute within 0.1 %.
//! Both checks are kept with their numbers. The dump is `*** Dump of MTM ***`, the
//! manager's `DumpTransaction` text and `***     End     ***`, as the dump command prints.
mod document_commands;
use document_commands::{get_real, new_document, set_real, word_at, word_index, word_list};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdoc_std::{TDocStdDocument, TDocStdMultiTransactionManager};

/// The shared manager of the `mtm*` commands.
struct Mtm(Handle<TDocStdMultiTransactionManager>);

impl Mtm {
    /// `mtmCreate limit`.
    fn create(limit: i32) -> Self {
        let manager = Handle::new(TDocStdMultiTransactionManager::new());
        manager.set_undo_limit(limit).expect("mtmCreate");
        Self(manager)
    }
    /// `mtmCommit name`.
    fn commit(&self, name: &str) {
        self.0
            .commit_command_named(&ExtendedString::new(name))
            .expect("mtmCommit");
    }
    /// `mtmDump` as a word list.
    fn dump(&self) -> Vec<String> {
        let mut text = b"*** Dump of MTM ***\n".to_vec();
        self.0.dump_transaction(&mut text).expect("mtmDump");
        text.extend_from_slice(b"\n***     End     ***\n");
        word_list(&String::from_utf8_lossy(&text))
    }
}

/// Each transaction name sits at its index, followed (after `-`) by
/// its document count, and `Last` at the last index.
fn check_dump(list: &[String], transactions: &[(&str, i64, &str)], last: i64) {
    for &(name, index, documents) in transactions {
        assert_eq!(word_index(list, name), index, "index of {name} in {list:?}");
        assert_eq!(word_at(list, index + 2), documents, "documents of {name}");
    }
    assert_eq!(word_index(list, "Last"), last, "index of Last in {list:?}");
}

/// The attribute exists and is within 0.1 % of `good`.
fn check_real(document: &Handle<TDocStdDocument>, entry: &str, good: f64) {
    let real = get_real(document, entry).expect("GetReal");
    let percent = (real - good).abs() / good * 100.0;
    assert!(percent <= 0.1, "{entry}: real = {real}, good_real = {good}");
}

fn check_reals(document: &Handle<TDocStdDocument>, values: &[(&str, f64)]) {
    for &(entry, good) in values {
        check_real(document, entry, good);
    }
}

/// Case 1 runs on three documents with a manager of undo limit 4 in nested mode.
struct Case1 {
    mtm: Mtm,
    doc1: Handle<TDocStdDocument>,
    doc2: Handle<TDocStdDocument>,
    doc3: Handle<TDocStdDocument>,
}

impl Case1 {
    fn new() -> Self {
        let case = Self {
            mtm: Mtm::create(4),
            doc1: new_document(),
            doc2: new_document(),
            doc3: new_document(),
        };
        case.mtm.0.set_nested_transaction_mode(true);
        case.mtm.0.add_document(&case.doc1).expect("mtmAdd doc1");
        case
    }
    /// The values every check after Transaction2 expects in doc2 and doc3.
    fn check_doc2_doc3(&self) {
        check_reals(&self.doc2, &[("0:2:1", 2.1), ("0:2:3", 22.1)]);
        check_reals(&self.doc3, &[("0:2:3", 22.1), ("0:2:4", 22.2)]);
    }
    fn check_doc1(&self, first: [f64; 3]) {
        let [a, b, c] = first;
        check_reals(&self.doc1, &[("0:1:1", a), ("0:1:3", b), ("0:1:4", c)]);
        check_reals(
            &self.doc1,
            &[("0:2:1", 2.2), ("0:2:2", 2.1), ("0:2:4", 2.3)],
        );
    }
}

/// The doc1 block of Transaction1 and Transaction3: a value, an aborted command, a
/// committed command with an aborted nested one, two more values.
fn doc1_block(doc1: &Handle<TDocStdDocument>, values: [f64; 7]) {
    set_real(doc1, "0:1:1", values[0]);
    doc1.open_command().unwrap();
    set_real(doc1, "0:1:1", values[1]);
    doc1.abort_command().unwrap();
    doc1.open_command().unwrap();
    set_real(doc1, "0:1:3", values[2]);
    doc1.open_command().unwrap();
    set_real(doc1, "0:1:3", values[3]);
    doc1.abort_command().unwrap();
    set_real(doc1, "0:1:4", values[4]);
    doc1.commit_command().unwrap();
    set_real(doc1, "0:1:1", values[5]);
    set_real(doc1, "0:1:4", values[6]);
}

fn transaction1(case: &Case1) {
    case.mtm.0.open_command().unwrap();
    doc1_block(&case.doc1, [1.1, 11.0, 12.1, 121.0, 12.2, 1.2, 1.3]);
    case.mtm.commit("Transaction1");
    check_dump(&case.mtm.dump(), &[("Transaction1", 21, "1")], 29);
    check_reals(
        &case.doc1,
        &[("0:1:1", 1.2), ("0:1:3", 12.1), ("0:1:4", 1.3)],
    );
}

/// The commands of Transaction2 on the three documents.
fn transaction2_commands(case: &Case1) {
    let (doc1, doc2, doc3) = (&case.doc1, &case.doc2, &case.doc3);
    set_real(doc2, "0:2:1", 2.1);
    set_real(doc1, "0:2:2", 2.1);
    doc1.open_command().unwrap();
    doc2.open_command().unwrap();
    set_real(doc2, "0:2:1", 21.0);
    set_real(doc1, "0:2:2", 21.0);
    doc2.abort_command().unwrap();
    doc3.open_command().unwrap();
    doc2.open_command().unwrap();
    set_real(doc3, "0:2:3", 22.1);
    set_real(doc2, "0:2:3", 22.1);
    doc2.open_command().unwrap();
    set_real(doc2, "0:2:3", 221.0);
    doc2.abort_command().unwrap();
    set_real(doc3, "0:2:4", 22.2);
    doc2.commit_command().unwrap();
    doc3.commit_command().unwrap();
    doc1.abort_command().unwrap();
    set_real(doc1, "0:2:1", 2.2);
    set_real(doc1, "0:2:4", 2.3);
}

fn transaction2(case: &Case1) {
    case.mtm.0.add_document(&case.doc2).unwrap();
    case.mtm.0.add_document(&case.doc3).unwrap();
    case.mtm.0.open_command().unwrap();
    transaction2_commands(case);
    case.mtm.commit("Transaction2");
    let list = case.mtm.dump();
    check_dump(
        &list,
        &[("Transaction1", 23, "1"), ("Transaction2", 31, "3")],
        41,
    );
    case.check_doc1([1.2, 12.1, 1.3]);
    case.check_doc2_doc3();
}

const T123: [(&str, i64, &str); 3] = [
    ("Transaction1", 23, "1"),
    ("Transaction2", 31, "3"),
    ("Transaction3", 41, "1"),
];

fn transaction3_undo_abort_redo(case: &Case1) {
    case.mtm.0.open_command().unwrap();
    doc1_block(&case.doc1, [3.1, 31.0, 32.1, 321.0, 32.2, 3.2, 3.3]);
    case.mtm.commit("Transaction3");
    check_dump(&case.mtm.dump(), &T123, 49);
    case.check_doc1([3.2, 32.1, 3.3]);
    case.check_doc2_doc3();
    case.mtm.0.undo().unwrap();
    check_undone_transaction3(case);
    aborted_transaction(case);
    check_undone_transaction3(case);
    case.mtm.0.redo().unwrap();
    let list = case.mtm.dump();
    check_dump(&list, &T123, 49);
    assert_eq!(word_at(&list, 40), "Undo:");
    case.check_doc1([3.2, 32.1, 3.3]);
    case.check_doc2_doc3();
}

/// After `mtmUndo` (and after the aborted transaction): Transaction3 is a redo.
fn check_undone_transaction3(case: &Case1) {
    let list = case.mtm.dump();
    let undone = [T123[0], T123[1], ("Transaction3", 44, "1")];
    check_dump(&list, &undone, 41);
    assert_eq!(word_at(&list, 43), "Redo:");
    case.check_doc1([1.2, 12.1, 1.3]);
    case.check_doc2_doc3();
}

/// `mtmOpen` ... `mtmAbort` with two commands still open in doc1.
fn aborted_transaction(case: &Case1) {
    let doc1 = &case.doc1;
    case.mtm.0.open_command().unwrap();
    set_real(doc1, "0:1:1", 4.1);
    doc1.open_command().unwrap();
    set_real(doc1, "0:1:1", 41.0);
    doc1.abort_command().unwrap();
    doc1.open_command().unwrap();
    set_real(doc1, "0:1:3", 42.1);
    doc1.open_command().unwrap();
    set_real(doc1, "0:1:3", 421.0);
    case.mtm.0.abort_command().unwrap();
}

/// Transaction5 commits with three commands of doc1 still open; it is undone, then
/// Transaction6 commits with two open commands and replaces the redo.
fn transactions5_and_6(case: &Case1) {
    let doc1 = &case.doc1;
    case.mtm.0.open_command().unwrap();
    set_real(doc1, "0:1:1", 5.1);
    doc1.open_command().unwrap();
    set_real(doc1, "0:1:1", 51.0);
    doc1.open_command().unwrap();
    set_real(doc1, "0:1:3", 52.1);
    doc1.open_command().unwrap();
    set_real(doc1, "0:1:3", 521.0);
    case.mtm.commit("Transaction5");
    check_dump(
        &case.mtm.dump(),
        &[T123[0], T123[1], T123[2], ("Transaction5", 49, "1")],
        57,
    );
    case.check_doc1([51.0, 521.0, 3.3]);
    case.check_doc2_doc3();
    case.mtm.0.undo().unwrap();
    let list = case.mtm.dump();
    check_dump(
        &list,
        &[T123[0], T123[1], T123[2], ("Transaction5", 52, "1")],
        49,
    );
    assert_eq!(word_at(&list, 51), "Redo:");
    case.check_doc1([3.2, 32.1, 3.3]);
    case.check_doc2_doc3();
    transaction6(case);
}

fn transaction6(case: &Case1) {
    let doc1 = &case.doc1;
    case.mtm.0.open_command().unwrap();
    set_real(doc1, "0:1:1", 6.1);
    doc1.open_command().unwrap();
    set_real(doc1, "0:1:1", 61.0);
    doc1.open_command().unwrap();
    set_real(doc1, "0:1:3", 62.1);
    case.mtm.commit("Transaction6");
    check_dump(
        &case.mtm.dump(),
        &[T123[0], T123[1], T123[2], ("Transaction6", 49, "1")],
        57,
    );
    case.check_doc1([61.0, 62.1, 3.3]);
    case.check_doc2_doc3();
}

/// Nested mode on.
#[test]
fn nested_transactions_synchronize_three_documents() {
    let case = Case1::new();
    transaction1(&case);
    transaction2(&case);
    transaction3_undo_abort_redo(&case);
    transactions5_and_6(&case);
}

/// Case 2: three documents, undo limit 4, nested mode off; `lindex list 7` is the
/// document count of the manager (`one` for a single document).
struct Case2 {
    mtm: Mtm,
    docs: [Handle<TDocStdDocument>; 3],
}

impl Case2 {
    fn check(&self, count: &str, transactions: &[(&str, i64, &str)], last: i64) {
        let list = self.mtm.dump();
        assert_eq!(
            word_at(&list, 7),
            count,
            "manager document count in {list:?}"
        );
        check_dump(&list, transactions, last);
    }
    /// `mtmOpen`, `SetReal` on (document, entry, value), `mtmCommit name`.
    fn transaction(&self, name: &str, sets: &[(usize, &str, f64)]) {
        self.mtm.0.open_command().unwrap();
        for &(document, entry, value) in sets {
            set_real(&self.docs[document], entry, value);
        }
        self.mtm.commit(name);
    }
    fn check_reals(&self, values: &[(usize, &str, f64)]) {
        for &(document, entry, good) in values {
            check_real(&self.docs[document], entry, good);
        }
    }
}

fn case2_first_documents(case: &Case2) {
    case.mtm.0.add_document(&case.docs[0]).unwrap();
    case.transaction("tr1", &[(0, "0:1", 1.0)]);
    case.check("one", &[("tr1", 21, "1")], 29);
    case.check_reals(&[(0, "0:1", 1.0)]);
    case.mtm.0.add_document(&case.docs[1]).unwrap();
    case.check("2", &[("tr1", 22, "1")], 30);
    case.check_reals(&[(0, "0:1", 1.0)]);
    case.transaction("tr2", &[(1, "0:1", 2.0)]);
    case.check("2", &[("tr1", 22, "1"), ("tr2", 30, "1")], 38);
    case.check_reals(&[(0, "0:1", 1.0), (1, "0:1", 2.0)]);
}

fn case2_third_document(case: &Case2) {
    set_real(&case.docs[2], "0:1", 1.0);
    case.mtm.0.add_document(&case.docs[2]).unwrap();
    case.check("3", &[("tr1", 23, "1"), ("tr2", 31, "1")], 39);
    case.check_reals(&[(0, "0:1", 1.0), (1, "0:1", 2.0)]);
    case.transaction("tr3", &[(0, "0:2", 2.0), (2, "0:2", 2.0)]);
    let three = [("tr1", 23, "1"), ("tr2", 31, "1"), ("tr3", 39, "2")];
    case.check("3", &three, 48);
    case.check_reals(&[
        (0, "0:1", 1.0),
        (0, "0:2", 2.0),
        (1, "0:1", 2.0),
        (2, "0:2", 2.0),
    ]);
    case.transaction("tr4", &[(1, "0:2", 2.0), (2, "0:3", 3.0)]);
    case.check("3", &[three[0], three[1], three[2], ("tr4", 48, "2")], 57);
    case.check_reals(&[
        (0, "0:1", 1.0),
        (0, "0:2", 2.0),
        (1, "0:1", 2.0),
        (1, "0:2", 2.0),
    ]);
    case.check_reals(&[(2, "0:2", 2.0), (2, "0:3", 3.0)]);
}

/// tr5 pushes tr1 out of the undo limit 4.
fn case2_undo_limit(case: &Case2) {
    case.transaction("tr5", &[(0, "0:3", 3.0), (1, "0:3", 3.0)]);
    let four = [
        ("tr2", 23, "1"),
        ("tr3", 31, "2"),
        ("tr4", 40, "2"),
        ("tr5", 49, "2"),
    ];
    case.check("3", &four, 58);
    case.check_reals(&[(0, "0:1", 1.0), (0, "0:2", 2.0), (0, "0:3", 3.0)]);
    case.check_reals(&[(1, "0:1", 2.0), (1, "0:2", 2.0), (1, "0:3", 3.0)]);
    case.check_reals(&[(2, "0:2", 2.0), (2, "0:3", 3.0)]);
}

/// Nested mode off.
#[test]
fn flat_transactions_synchronize_three_documents() {
    let case = Case2 {
        mtm: Mtm::create(4),
        docs: [new_document(), new_document(), new_document()],
    };
    case2_first_documents(&case);
    case2_third_document(&case);
    case2_undo_limit(&case);
}
