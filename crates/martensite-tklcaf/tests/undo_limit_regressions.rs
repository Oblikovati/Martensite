//! Undo of committed transactions: a TDF_Transaction committed with its delta and undone through
//! `TDF_Data::Undo`, as the DDF commands `OpenTran`, `CommitTran D 1` and `DFUndo D 1` do.
//!
//! The undo removes the integer the committed transaction set, and the real attribute
//! stays. The tests assert that outcome: the integer is gone when the scenario reads it, the
//! real attribute is there when the scenario reads it, and every `DFUndo` is applicable.
mod document_commands;
use document_commands::{
    add_label, get_integer, get_real, new_named_document, set_integer, set_real,
};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tklcaf::tdf::{TdfDelta, TdfTransaction};
use martensite_tklcaf::tdoc_std::TDocStdDocument;

/// The DDF globals: the last delta of `CommitTran`/`DFUndo`.
struct Ddf {
    document: Handle<TDocStdDocument>,
    last_delta: Handle<TdfDelta>,
}

impl Ddf {
    /// `NewDocument D BinOcaf`, `UndoLimit D 100`, `Label D 0:2`.
    fn new() -> Self {
        let document = new_named_document("D", "BinOcaf");
        document.set_undo_limit(100).unwrap();
        add_label(&document, "0:2");
        Self {
            document,
            last_delta: Handle::default(),
        }
    }
    /// `OpenTran D`, `SetInteger D 0:2 5`, `CommitTran D 1`.
    fn committed_integer(&mut self) {
        let mut transaction =
            TdfTransaction::with_data(&self.document.get_data(), &AsciiString::default());
        transaction.open().unwrap();
        set_integer(&self.document, "0:2", 5);
        self.last_delta = transaction.commit(true).unwrap();
    }
    /// `DFUndo D 1`: the last delta must be applicable; it becomes the redo delta.
    fn undo(&mut self) {
        let data = self.document.get_data();
        let framework = data.get().unwrap();
        assert!(!self.last_delta.is_null(), "No undo to apply.");
        assert!(
            framework.is_applicable(&self.last_delta),
            "Undo not applicable HERE and NOW."
        );
        self.last_delta = framework.undo(&self.last_delta, true).unwrap();
    }
    /// `GetInteger D 0:2` fails: the `TODO` error line of the case.
    fn check_integer_missing(&self) {
        let missing = get_integer(&self.document, "0:2").unwrap_err();
        assert_eq!(missing, "attribute not found for entry : 0:2");
    }
    /// `GetReal D 0:2` succeeds: the `OK` line of the case.
    fn check_real_present(&self) {
        assert_eq!(get_real(&self.document, "0:2").unwrap(), 8.8);
    }
}

/// Case 1, Undo.
#[test]
fn undo_removes_the_committed_integer() {
    let mut ddf = Ddf::new();
    ddf.committed_integer();
    ddf.undo();
    ddf.check_integer_missing();
    set_real(&ddf.document, "0:2", 8.8);
    ddf.undo();
    ddf.check_real_present();
}

/// Case 3, SetUndoLimit.
#[test]
fn undo_after_limit_change_removes_the_committed_integer() {
    let mut ddf = Ddf::new();
    ddf.committed_integer();
    ddf.document.set_undo_limit(10).unwrap();
    set_real(&ddf.document, "0:2", 8.8);
    ddf.undo();
    ddf.check_integer_missing();
    ddf.check_real_present();
}
