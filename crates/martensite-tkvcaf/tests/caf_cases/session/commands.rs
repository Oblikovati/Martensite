//! The document commands the scripts call: each writes the line the script's log has for it, then
//! does the same library calls, and answers what the script reads back.
use super::hooks::{ImageCheck, kept_image};
use super::viewer::ViewerHost;
use crate::support::script_error::script_error;
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::TNamingBuilder;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tklcaf::tdf::{TdfLabel, TdfTool};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tklcaf::tfunction::TFunctionDriverTable;
use martensite_tkvcaf::tprs_std::TPrsStdAISViewer;
use std::collections::HashMap;

/// One script's document, its viewer once it opens one, and its named shapes.
#[derive(Default)]
pub struct CafSession {
    document: Option<Handle<TDocStdDocument>>,
    pub(super) host: Option<ViewerHost>,
    pub(super) drivers: Option<Handle<TFunctionDriverTable>>,
    shapes: HashMap<String, Shape>,
}

impl CafSession {
    /// The open document; `session.document()`.
    pub fn document(&self) -> &Handle<TDocStdDocument> {
        self.document
            .as_ref()
            .expect("the script opened a document")
    }

    /// `NewDocument D [format]`, without a format not handled by an application.
    /// Example: `session.new_document(Some("BinOcaf"));`.
    pub fn new_document(&mut self, format: Option<&str>) {
        let document = match format {
            Some(name) => {
                println!("NewDocument D {name} \ndocument D created");
                TDocStdDocument::new(&ExtendedString::from_utf8(name)).unwrap()
            }
            None => {
                println!("NewDocument D \ndocument (not handled by application)  D created");
                TDocStdDocument::new(&ExtendedString::from_utf8("dummy")).unwrap()
            }
        };
        println!("{}", entry_of(&document.main().unwrap()));
        self.document = Some(document);
    }

    /// `Close D`, on a document with no viewer.
    /// Example: `session.close_document();`.
    pub fn close_document(&mut self) {
        println!("Close D ");
        self.document = None;
    }

    /// `UndoLimit D limit`.
    /// Example: `session.undo_limit(100);`.
    pub fn undo_limit(&self, limit: i32) {
        println!("UndoLimit D {limit} ");
        let document = self.document();
        document.set_undo_limit(limit).unwrap();
        let (undos, redos) = (
            document.get_available_undos(),
            document.get_available_redos(),
        );
        println!("{} {undos} {redos}", document.get_undo_limit());
    }

    /// `NewCommand D`.
    /// Example: `session.new_command();`.
    pub fn new_command(&self) {
        println!("NewCommand D ");
        self.document().new_command().unwrap();
    }

    /// `OpenCommand D`.
    /// Example: `session.open_command();`.
    pub fn open_command(&self) {
        println!("OpenCommand D ");
        self.document().open_command().unwrap();
    }

    /// `CommitCommand D`.
    /// Example: `session.commit_command();`.
    pub fn commit_command(&self) {
        println!("CommitCommand D ");
        self.document().commit_command().unwrap();
    }

    /// `Undo D`: undoes one command and redraws the document's viewer.
    /// Example: `session.undo();`.
    pub fn undo(&self) {
        println!("Undo D ");
        if !self.document().undo().unwrap() {
            println!("Undo not done");
        }
        self.redraw_document_viewer();
    }

    /// `Redo D`: redoes one command and redraws the document's viewer.
    /// Example: `session.redo();`.
    pub fn redo(&self) {
        println!("Redo D ");
        if !self.document().redo().unwrap() {
            println!("Redo not done");
        }
        self.redraw_document_viewer();
    }

    fn redraw_document_viewer(&self) {
        let mut context = Handle::default();
        let main = self.document().main().unwrap();
        if TPrsStdAISViewer::find_context(&main, &mut context).unwrap() {
            context.update_current_viewer();
        }
    }

    /// The label of `entry`, created when absent; `session.add_label("0:2")`.
    pub fn add_label(&self, entry: &str) -> TdfLabel {
        let mut label = TdfLabel::new();
        let data = self.document().get_data();
        let text = AsciiString::from(entry);
        TdfTool::label_from_entry(&data, &text, &mut label, true).unwrap();
        label
    }

    /// The label of `entry`; a script error when absent; `session.find_label("0:2")`.
    pub fn find_label(&self, entry: &str) -> TdfLabel {
        let mut label = TdfLabel::new();
        let data = self.document().get_data();
        let text = AsciiString::from(entry);
        TdfTool::label_from_entry(&data, &text, &mut label, false).unwrap();
        if label.is_null() {
            script_error(&format!("label {entry}: expected it in the document"));
        }
        label
    }

    /// Binds `shape` to `name` without a line, for commands with their own echo.
    /// Example: `session.bind_shape("aBox1", shape);`.
    pub fn bind_shape(&mut self, name: &str, shape: Shape) {
        self.shapes.insert(name.to_owned(), shape);
    }

    /// The shape bound to `name`; a script error when none is.
    /// Example: `session.shape("aBox1")`.
    pub fn shape(&self, name: &str) -> &Shape {
        self.shapes
            .get(name)
            .unwrap_or_else(|| script_error(&format!("shape {name}: expected it bound")))
    }

    /// `SetShape D entry name`: the shape as a generated named shape of the label.
    /// Example: `session.set_shape("0:2", "aBox1");`.
    pub fn set_shape(&self, entry: &str, name: &str) {
        println!("SetShape D {entry} {name} ");
        let label = self.add_label(entry);
        TNamingBuilder::new(&label)
            .unwrap()
            .generated(self.shape(name))
            .unwrap();
    }

    /// Dumps the view the script's end hook photographs, and checks what its words promise.
    /// Example: `session.checkview_screenshot("bug_cases::a", ImageCheck::Drawn);`.
    pub fn checkview_screenshot(&self, identity: &str, check: ImageCheck) {
        println!("vdump {identity} ");
        let host = self.host.as_ref().expect("the script opened a viewer");
        let drawn = host.dump(kept_image(identity).as_deref());
        let holds = match check {
            ImageCheck::Empty => drawn == 0,
            ImageCheck::Drawn => drawn > 0,
            ImageCheck::None => true,
        };
        if !holds {
            println!("Error: picture of {identity} has {drawn} drawn pixels; expected {check:?}");
        }
    }

    /// Retires the script's view; validation errors go to the graded stream.
    /// Example: `session.retire_viewer();`.
    pub fn retire_viewer(&mut self) {
        let Some(host) = self.host.take() else {
            return;
        };
        for error in host.close() {
            println!("Error: Vulkan validation: {error}");
        }
    }
}

/// The entry of `label`, as a script reads it; `entry_of(&label) == "0:2"`.
pub fn entry_of(label: &TdfLabel) -> String {
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    String::from_utf8_lossy(entry.as_bytes()).into_owned()
}
