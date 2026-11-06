//! The document commands of the scripts: `XNewDoc`, `ReadIges`, `WriteIges`, `ReadStep`, `XGetOneShape`,
//! `Close`, on the session the scripts share.
use crate::iges_session::IgesSession;
use crate::support::{rules::EXCEPTION_PREFIX, session::Session};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkdeiges::igescaf_control::{IgesCafControlReader, IgesCafControlWriter};
use martensite_tkdestep::{
    de_step::StepControlStepModelType,
    stepcaf_control::{STEPCAFControlReader, STEPCAFControlWriter},
};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle},
    tcollection::{AsciiString, ExtendedString},
};
use martensite_tklcaf::{
    tdata_std::TDataStdName,
    tdf::{TdfLabel, TdfTool},
    tdoc_std::{TDocStdApplication, TDocStdDocument},
};
use martensite_tkxcaf::{
    xcaf_app::XCAFAppApplication,
    xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool},
};
use martensite_tkxsbase::{
    if_select::IFSelectReturnStatus, interface::InterfaceStatic, xs_algo::XSAlgoShapeProcessor,
    xs_control::XSControlReaderOps,
};

/// A command that returned 1, as `catch` sees it: what it wrote stays in the stream.
pub struct CommandFailed;

impl IgesSession {
    /// `NewDocument name BinXCAF` or `XNewDoc name`: a new document, `document name created`.
    /// Example: `session.new_document("D_First")`.
    pub fn new_document(&mut self, name: &str) {
        self.new_document_silently(name);
        println!("document {name} created");
    }

    fn new_document_silently(&mut self, name: &str) {
        let application = XCAFAppApplication::get_application();
        let mut document: Handle<TDocStdDocument> = Handle::default();
        application
            .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
            .unwrap_or_else(|failure| panic!("NewDocument {name}: {failure:?}"));
        let root = document.get().expect("a new document").get_data().root();
        TDataStdName::set(&root, &ExtendedString::from_utf8(name))
            .unwrap_or_else(|failure| panic!("NewDocument {name}: {failure:?}"));
        self.documents.insert(name.to_owned(), document);
    }

    /// `ReadIges name file`, on the session's model. What the command itself writes reaches the stream
    /// when it returns, after the messages of the translation, as the interpreter's result does.
    /// Example: `session.read_iges("D_First", "a.igs")`.
    pub fn read_iges(&mut self, name: &str, file: &str) -> Result<(), CommandFailed> {
        let mut result = format!(" File IGES to read : {file}\n");
        match self.transfer_iges(name, file, &mut result) {
            Ok(true) => {
                println!("{result}Document saved with name {name}");
                Ok(())
            }
            Ok(false) => {
                print!("{result}");
                Err(CommandFailed)
            }
            Err(failure) => {
                println!("{result}{EXCEPTION_PREFIX}{failure:?}");
                Err(CommandFailed)
            }
        }
    }

    fn transfer_iges(
        &mut self,
        name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<bool, Failure> {
        self.select_norm("IGES")?;
        let mut reader = IgesCafControlReader::with_session(&self.work_session, true)?;
        reader
            .base_mut()
            .set_read_visible(InterfaceStatic::i_val("read.iges.onlyvisible") == 1);
        if reader.base_mut().read_file(file)? != IFSelectReturnStatus::RetDone {
            *result += &format!("Could not read file {file} , abandon\n");
            return Ok(false);
        }
        let (parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("read.iges.resource.name"),
            &AsciiString::from("read.iges.sequence"),
        )?;
        reader
            .base_mut()
            .xs_reader()
            .set_shape_fix_parameters(parameters);
        reader.base_mut().xs_reader().set_shape_process_flags(flags);
        if !self.documents.contains_key(name) {
            self.new_document_silently(name);
        }
        let document = self.documents[name].clone();
        if !reader.transfer(&document, &ProgressRange::default())? {
            *result += "Cannot read any relevant data from the IGES file\n";
            return Ok(false);
        }
        Ok(true)
    }

    /// `WriteIges name file`: the document to a file, in the session's model. What the command itself
    /// writes reaches the stream when it returns, after the translation's messages.
    /// Example: `session.write_iges("D_First", "a.igs")`.
    pub fn write_iges(&mut self, name: &str, file: &str) -> Result<(), CommandFailed> {
        let mut result = String::new();
        let outcome = self.transfer_to_iges(name, file, &mut result);
        print!("{result}");
        outcome.map_err(|failure| {
            println!("{EXCEPTION_PREFIX}{failure:?}");
            CommandFailed
        })
    }

    fn transfer_to_iges(
        &mut self,
        name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        self.select_norm("IGES")?;
        let mut writer = IgesCafControlWriter::with_session(&self.work_session)?;
        let (parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("write.iges.resource.name"),
            &AsciiString::from("write.iges.sequence"),
        )?;
        writer.base_mut().set_shape_fix_parameters(parameters);
        writer.base_mut().set_shape_process_flags(flags);
        let document = self.documents[name].clone();
        writer.transfer_document(&document, &ProgressRange::default())?;
        *result += &format!("Writing IGES model to file {file}\n");
        *result += if writer.base_mut().write_file(file, false)? {
            " Write OK\n"
        } else {
            " Write failed\n"
        };
        Ok(())
    }

    /// `ReadStep name file`, on the session's model.
    /// Example: `session.read_step("d", "a.stp")`.
    pub fn read_step(&mut self, name: &str, file: &str) -> Result<(), CommandFailed> {
        let mut result = format!(" File STEP to read : {file}\n");
        match self.transfer_step(name, file, &mut result) {
            Ok(true) => {
                println!("{result}Document saved with name {name}");
                Ok(())
            }
            Ok(false) => {
                print!("{result}");
                Err(CommandFailed)
            }
            Err(failure) => {
                println!("{result}{EXCEPTION_PREFIX}{failure:?}");
                Err(CommandFailed)
            }
        }
    }

    fn transfer_step(
        &mut self,
        name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<bool, Failure> {
        self.select_norm("STEP")?;
        let mut reader = STEPCAFControlReader::with_session(&self.work_session, true)?;
        if reader.read_file(file)? != IFSelectReturnStatus::RetDone {
            *result += &format!("Could not read file {file} , abandon\n");
            return Ok(false);
        }
        if !self.documents.contains_key(name) {
            self.new_document_silently(name);
        }
        let (parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("read.step.resource.name"),
            &AsciiString::from("read.step.sequence"),
        )?;
        reader.set_shape_fix_parameters(parameters)?;
        reader.set_shape_process_flags(flags)?;
        let document = self.documents[name].clone();
        if !reader.transfer(&document, &ProgressRange::default())? {
            *result += "Cannot read any relevant data from the STEP file\n";
            return Ok(false);
        }
        Ok(true)
    }

    /// `WriteStep name file`: the document to a file, in the session's model.
    /// Example: `session.write_step("D", "a.stp")`.
    pub fn write_step(&mut self, name: &str, file: &str) -> Result<(), CommandFailed> {
        let mut result = String::new();
        let outcome = self.transfer_to_step(name, file, &mut result);
        print!("{result}");
        outcome.map_err(|failure| {
            println!("{EXCEPTION_PREFIX}{failure:?}");
            CommandFailed
        })
    }

    fn transfer_to_step(
        &mut self,
        name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        self.select_norm("STEP")?;
        let mut writer = STEPCAFControlWriter::with_session(&self.work_session, true)?;
        let (parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("write.step.resource.name"),
            &AsciiString::from("write.step.sequence"),
        )?;
        writer.set_shape_fix_parameters(parameters)?;
        writer.set_shape_process_flags(flags)?;
        *result += &format!("Translating document {name} to STEP\n");
        let document = self.documents[name].clone();
        if !writer.transfer_document(
            &document,
            StepControlStepModelType::AS_IS,
            None,
            &ProgressRange::default(),
        )? {
            *result += "Error: The document cannot be translated or gives no result\n";
        }
        *result += &format!("Writing STEP file {file}\n");
        *result += &match writer.write(file)? {
            IFSelectReturnStatus::RetVoid => "Error: no file written".to_owned(),
            IFSelectReturnStatus::RetDone => format!("File {file} written\n"),
            _ => "Error on writing file".to_owned(),
        };
        Ok(())
    }

    /// `XGetOneShape shape name`: the document's only free shape, or a compound of all of them.
    /// Example: `session.get_one_shape("res_1", "D_First")`.
    pub fn get_one_shape(&mut self, shape_name: &str, name: &str) -> Result<(), CommandFailed> {
        let document = self.documents[name].clone();
        let main = document
            .get()
            .expect("a document")
            .main()
            .expect("a main label");
        let tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
        let labels = tool.get_free_shapes().expect("free shapes");
        if labels.is_empty() {
            println!("Document {name} is empty");
            return Err(CommandFailed);
        }
        let shape = one_shape(&labels);
        if shape.is_null() {
            println!("No shape found in the document");
            return Err(CommandFailed);
        }
        self.raster.bind(shape_name, &shape);
        println!("{shape_name}");
        Ok(())
    }

    /// `XGetShape shape doc entry`: the shape on a label.
    /// Example: `session.get_shape_at("c", "doc", "0:1:1:1")`.
    pub fn get_shape_at(&mut self, shape_name: &str, name: &str, entry: &str) {
        let label = label_of(self, name, entry);
        self.raster.bind(shape_name, &shape_of(&label));
    }

    /// `Close name -silent`: nothing said of a name that is no document.
    /// Example: `session.close_silently("D")`.
    pub fn close_silently(&mut self, name: &str) {
        println!("Close {name} -silent ");
        if self.documents.contains_key(name) {
            self.close_document(name);
        }
    }

    /// `Close name`: a name that is no document is only complained about.
    /// Example: `session.close_document("D_First")`.
    pub fn close_document(&mut self, name: &str) {
        let Some(document) = self.documents.remove(name) else {
            println!("{name} is not a Document");
            return;
        };
        XCAFAppApplication::get_application()
            .close_tdoc_std_document(&document)
            .unwrap_or_else(|failure| panic!("Close {name}: {failure:?}"));
    }
}

/// `TdfTool::label(doc, entry)`: the label of an entry; a null one when it does not exist.
/// Example: `label_of(&session, "D", "0:1:1:1")`.
pub fn label_of(session: &IgesSession, doc: &str, entry: &str) -> TdfLabel {
    let data = session.documents[doc].get().expect("a document").get_data();
    let mut label = TdfLabel::default();
    TdfTool::label_from_entry(&data, &AsciiString::from(entry), &mut label, false)
        .unwrap_or_else(|failure| panic!("{entry}: {failure:?}"));
    label
}

/// `TdfTool::entry(label)`. Example: `entry_of(&label)`.
pub fn entry_of(label: &TdfLabel) -> String {
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    String::from_utf8_lossy(entry.as_bytes()).into_owned()
}

/// The shape on a label. Example: `shape_of(&label)`.
pub fn shape_of(label: &TdfLabel) -> Shape {
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(label, &mut shape).expect("a label's shape");
    shape
}

fn one_shape(labels: &[TdfLabel]) -> Shape {
    if let [only] = labels {
        return shape_of(only);
    }
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    for label in labels {
        builder
            .add(&mut compound, &shape_of(label))
            .expect("a compound");
    }
    compound.into_shape()
}
