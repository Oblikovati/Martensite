//! The document commands of the scripts: `XNewDoc`, `read_step`, `WriteStep`, `XGetOneShape`, `close`,
//! on the session the scripts share.
use crate::support::{rules::EXCEPTION_PREFIX, session::Session};
use crate::xs_session::XsSession;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkdestep::{
    de_step::StepControlStepModelType,
    stepcaf_control::{STEPCAFControlReader, STEPCAFControlWriter},
};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle},
    tcollection::{AsciiString, ExtendedString},
    units_methods::UnitsMethods,
};
use martensite_tklcaf::{
    tdata_std::TDataStdName,
    tdoc_std::{TDocStdApplication, TDocStdDocument},
};
use martensite_tkxcaf::{
    xcaf_app::XCAFAppApplication,
    xcaf_doc::{XCAFDocDocumentTool, XCAFDocLengthUnit, XCAFDocShapeTool},
};
use martensite_tkxsbase::{if_select::IFSelectReturnStatus, xs_algo::XSAlgoShapeProcessor};

/// A command that returned 1, as `catch` sees it: what it wrote stays in the stream.
pub struct CommandFailed;

impl XsSession {
    /// `NewDocument name BinXCAF` or `XNewDoc name`: a new document, `document name created`.
    /// Example: `session.new_document("D_First", "XNewDoc")`.
    pub fn new_document(&mut self, name: &str) {
        let application = XCAFAppApplication::get_application();
        let mut document: Handle<TDocStdDocument> = Handle::default();
        application
            .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
            .unwrap_or_else(|failure| panic!("NewDocument {name}: {failure:?}"));
        let root = document.get().expect("a new document").get_data().root();
        TDataStdName::set(&root, &ExtendedString::from_utf8(name))
            .unwrap_or_else(|failure| panic!("NewDocument {name}: {failure:?}"));
        self.documents.insert(name.to_owned(), document);
        println!("document {name} created");
    }

    /// `ReadStep name file`, on the session's model. What the command itself writes reaches the stream
    /// when it returns, after the messages of the translation, as the interpreter's result does.
    /// Example: `session.read_step("D_First", "a.stp")`.
    pub fn read_step(&mut self, name: &str, file: &str) -> Result<(), CommandFailed> {
        let mut result = format!(" File STEP to read : {file}\n");
        let outcome = self.transfer_file(name, file, &mut result);
        match outcome {
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

    fn transfer_file(
        &mut self,
        name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<bool, Failure> {
        self.select_step_norm()?;
        let mut reader = STEPCAFControlReader::with_session(&self.work_session, true)?;
        if reader.read_file(file)? != IFSelectReturnStatus::RetDone {
            *result += &format!("Could not read file {file} , abandon\n");
            return Ok(false);
        }
        if !self.documents.contains_key(name) {
            self.new_document_silently(name);
        }
        self.set_processing(&mut reader)?;
        let document = self.documents[name].clone();
        if !reader.transfer(&document, &ProgressRange::default())? {
            *result += "Cannot read any relevant data from the STEP file\n";
            return Ok(false);
        }
        Ok(true)
    }

    fn new_document_silently(&mut self, name: &str) {
        let application = XCAFAppApplication::get_application();
        let mut document: Handle<TDocStdDocument> = Handle::default();
        application
            .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
            .unwrap_or_else(|failure| panic!("ReadStep {name}: {failure:?}"));
        self.documents.insert(name.to_owned(), document);
    }

    fn set_processing(&self, reader: &mut STEPCAFControlReader) -> Result<(), Failure> {
        let (mut parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("read.step.resource.name"),
            &AsciiString::from("read.step.sequence"),
        )?;
        for (key, value) in &self.resource_overrides {
            parameters.bind(
                AsciiString::from(key.as_str()),
                AsciiString::from(value.as_str()),
            );
        }
        reader.set_shape_fix_parameters(parameters)?;
        reader.set_shape_process_flags(flags)
    }

    /// `WriteStep name file`: the document to a file, in the session's model. What the command itself
    /// writes reaches the stream when it returns, after the translation's messages.
    /// Example: `session.write_step("D_First", "a.stp")`.
    pub fn write_step(&mut self, name: &str, file: &str) -> Result<(), CommandFailed> {
        let mut result = String::new();
        let outcome = self.transfer_to_file(name, file, &mut result);
        print!("{result}");
        outcome.map_err(|failure| {
            println!("{EXCEPTION_PREFIX}{failure:?}");
            CommandFailed
        })
    }

    fn transfer_to_file(
        &mut self,
        name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        self.select_step_norm()?;
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

    /// `XSetLengthUnit name unit`, with a unit's name.
    /// Example: `session.set_length_unit("D_First", "cm")`.
    pub fn set_length_unit(&mut self, name: &str, unit: &str) {
        println!("XSetLengthUnit {name} {unit} ");
        let document = &self.documents[name];
        let main = document
            .get()
            .expect("a document")
            .main()
            .expect("a main label");
        let kind = UnitsMethods::length_unit_from_string(unit.as_bytes(), false);
        let scale = UnitsMethods::get_length_factor_value(kind as i32) * 0.001;
        XCAFDocLengthUnit::set(
            &main.root().expect("a root label"),
            &AsciiString::from(unit),
            scale,
        )
        .unwrap_or_else(|failure| panic!("XSetLengthUnit {name}: {failure:?}"));
    }

    /// `XGetLengthUnit name`: the unit's name, as `dumpLengthUnit` writes it.
    /// Example: `session.get_length_unit("D_First")`.
    pub fn get_length_unit(&mut self, name: &str) -> String {
        println!("XGetLengthUnit {name} ");
        let document = &self.documents[name];
        let main = document
            .get()
            .expect("a document")
            .main()
            .expect("a main label");
        let root = main.root().expect("a root label");
        let mut unit: Handle<XCAFDocLengthUnit> = Handle::default();
        if !root
            .find_attribute_as(&XCAFDocLengthUnit::get_id(), &mut unit)
            .expect("a length unit")
        {
            println!("Error: Document doesn't contain a Length Unit");
            return String::new();
        }
        let name = unit.get().expect("a length unit").get_unit_name();
        let text = String::from_utf8_lossy(name.as_bytes()).into_owned();
        println!("{text}");
        text
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

    /// `read_file`'s new document: bound under its name, with the name on its root.
    pub(crate) fn new_named_document(&mut self, name: &str) {
        self.new_document_silently(name);
        let root = self.documents[name]
            .get()
            .expect("a new document")
            .get_data()
            .root();
        TDataStdName::set(&root, &ExtendedString::from_utf8(name))
            .unwrap_or_else(|failure| panic!("ReadFile {name}: {failure:?}"));
    }

    /// `XGetShape shape doc entry`: the shape on a label.
    /// Example: `session.get_shape_at("c", "doc", "0:1:1:1")`.
    pub fn get_shape_at(&mut self, shape_name: &str, name: &str, entry: &str) {
        let label = crate::script_values::label_of(self, name, entry);
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

fn shape_of(label: &martensite_tklcaf::tdf::TdfLabel) -> Shape {
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(label, &mut shape).expect("a label's shape");
    shape
}

fn one_shape(labels: &[martensite_tklcaf::tdf::TdfLabel]) -> Shape {
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
