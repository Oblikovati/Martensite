//! The document commands of the scripts: `XNewDoc`, `ReadIges`, `WriteIges`, `XAddShape`, `XSetColor`,
//! `XGetOneShape`, `Close`, on the session the scripts share.
use crate::support::{script_error::script_error, session::Session};
use crate::xs_session::XsSession;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkdeiges::igescaf_control::{IgesCafControlReader, IgesCafControlWriter};
use martensite_tkernel::{
    message::ProgressRange,
    quantity::{Color, ColorRgba},
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
    xcaf_doc::{
        XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool, XCAFDocEditor, XCAFDocLengthUnit,
        XCAFDocShapeTool,
    },
};
use martensite_tkxsbase::{if_select::IFSelectReturnStatus, xs_algo::XSAlgoShapeProcessor};

/// A command that returned 1, as `catch` sees it: what it wrote stays in the stream.
pub struct CommandFailed;

/// `TdfTool::label(doc, entry)`: the label of an entry; a null one when it does not exist.
pub fn label_of(document: &Handle<TDocStdDocument>, entry: &str) -> TdfLabel {
    let data = document.get().expect("a document").get_data();
    let mut label = TdfLabel::default();
    TdfTool::label_from_entry(&data, &AsciiString::from(entry), &mut label, false)
        .unwrap_or_else(|failure| panic!("{entry}: {failure:?}"));
    label
}

fn entry_of(label: &TdfLabel) -> String {
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    String::from_utf8_lossy(entry.as_bytes()).into_owned()
}

fn main_of(document: &Handle<TDocStdDocument>) -> TdfLabel {
    document
        .get()
        .expect("a document")
        .main()
        .expect("a main label")
}

fn new_bin_document(name: &str) -> Handle<TDocStdDocument> {
    let mut document: Handle<TDocStdDocument> = Handle::default();
    XCAFAppApplication::get_application()
        .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
        .unwrap_or_else(|failure| panic!("a new document {name}: {failure:?}"));
    document
}

fn name_root(document: &Handle<TDocStdDocument>, name: &str) {
    let root = document.get().expect("a new document").get_data().root();
    TDataStdName::set(&root, &ExtendedString::from_utf8(name))
        .unwrap_or_else(|failure| panic!("the name of {name}: {failure:?}"));
}

/// A colour the way the commands take one: a hexadecimal word, a name, or three components, 8-bit
/// and in sRGB when any of them is over 1. Example: `parse_color(&["BLUE"])`.
pub fn parse_color(words: &[&str]) -> Option<ColorRgba> {
    if let [word] = words {
        return ColorRgba::from_hex(word, true).or_else(|| ColorRgba::from_name(word));
    }
    let parts: Vec<f64> = words
        .iter()
        .map(|word| word.parse().ok())
        .collect::<Option<_>>()?;
    let [red, green, blue] = parts[..] else {
        return None;
    };
    if red.max(green).max(blue) > 1.0 {
        let linear = |part: f64| Color::srgb_to_linear_f32((part / 255.0) as f32);
        return ColorRgba::new(linear(red), linear(green), linear(blue), 1.0).ok();
    }
    ColorRgba::new(red as f32, green as f32, blue as f32, 1.0).ok()
}

impl XsSession {
    /// `XNewDoc name`: a new document. Example: `session.new_document("D")`.
    pub fn new_document(&mut self, name: &str) {
        println!("XNewDoc {name} ");
        let document = new_bin_document(name);
        name_root(&document, name);
        self.documents.insert(name.to_owned(), document);
        println!("document {name} created");
    }

    /// `NewDocument name format`: as `XNewDoc`, answering with the main label.
    /// Example: `session.new_document_with_format("D", "XCAF")`.
    pub fn new_document_with_format(&mut self, name: &str, format: &str) {
        println!("NewDocument {name} {format} ");
        let document = new_bin_document(name);
        name_root(&document, name);
        self.documents.insert(name.to_owned(), document);
        println!("document {name} created");
        println!("0:1");
    }

    /// `ReadIges name file`, on the session's model. What the command itself writes reaches the stream
    /// when it returns, after the messages of the translation.
    /// Example: `session.read_iges("D", "a.igs")`.
    pub fn read_iges(&mut self, name: &str, file: &str) -> Result<(), CommandFailed> {
        println!("ReadIges {name} {file} ");
        let mut result = format!(" File IGES to read : {file}\n");
        match self.transfer_file(name, file, &mut result) {
            Ok(true) => {
                println!("{result}Document saved with name {name}");
                Ok(())
            }
            Ok(false) => {
                print!("{result}");
                Err(CommandFailed)
            }
            Err(failure) => {
                print!("{result}");
                script_error(&format!("{failure:?}"));
            }
        }
    }

    fn transfer_file(
        &mut self,
        name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<bool, Failure> {
        self.select_iges_norm()?;
        let mut reader = IgesCafControlReader::with_session(&self.work_session, true)?;
        let only_visible =
            martensite_tkxsbase::interface::InterfaceStatic::i_val("read.iges.onlyvisible") == 1;
        reader.base_mut().set_read_visible(only_visible);
        use martensite_tkxsbase::xs_control::XSControlReaderOps as _;
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
            let document = new_bin_document(name);
            name_root(&document, name);
            self.documents.insert(name.to_owned(), document);
        }
        let document = self.documents[name].clone();
        if !reader.transfer(&document, &ProgressRange::default())? {
            *result += "Cannot read any relevant data from the IGES file\n";
            return Ok(false);
        }
        Ok(true)
    }

    /// `WriteIges name file`: the document to a file, in the session's model.
    /// Example: `session.write_iges("D", "a.igs")`.
    pub fn write_iges(&mut self, name: &str, file: &str) {
        println!("WriteIges {name} {file} ");
        let mut result = String::new();
        let outcome = self.transfer_to_file(name, file, &mut result);
        print!("{result}");
        if let Err(failure) = outcome {
            script_error(&format!("{failure:?}"));
        }
    }

    fn transfer_to_file(
        &mut self,
        name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        let Some(document) = self.documents.get(name).cloned() else {
            *result += &format!("{name} is not a document\n");
            script_error(result);
        };
        self.select_iges_norm()?;
        let mut writer = IgesCafControlWriter::with_session(&self.work_session)?;
        let (parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("write.iges.resource.name"),
            &AsciiString::from("write.iges.sequence"),
        )?;
        writer.base_mut().set_shape_fix_parameters(parameters);
        writer.base_mut().set_shape_process_flags(flags);
        writer.transfer_document(&document, &ProgressRange::default())?;
        *result += &format!("Writing IGES model to file {file}\n");
        if writer.base_mut().write_file(file, false)? {
            *result += " Write OK\n";
        } else {
            *result += " Write failed\n";
        }
        Ok(())
    }

    /// `XAddShape doc name [0|1]`: the shape on a new label, its entry the command's answer.
    /// Example: `session.add_shape("D", "s", true)`.
    pub fn add_shape(&mut self, doc: &str, name: &str, assembly: bool, words: &str) -> String {
        println!("XAddShape {doc} {name}{words} ");
        let shape = self.bound_shape(name);
        let main = main_of(&self.documents[doc]);
        let tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
        let label = tool
            .get()
            .expect("a shape tool")
            .add_shape(&shape, assembly, true)
            .unwrap_or_else(|failure| panic!("XAddShape {name}: {failure:?}"));
        let entry = entry_of(&label);
        println!("{entry}");
        entry
    }

    /// `XFindShape doc name`: the entry of the shape's label.
    /// Example: `session.find_shape("D", "pin")`.
    pub fn find_shape(&self, doc: &str, name: &str) -> String {
        println!("XFindShape {doc} {name} ");
        let main = main_of(&self.documents[doc]);
        let tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
        let mut label = TdfLabel::default();
        tool.get()
            .expect("a shape tool")
            .find_shape(&self.bound_shape(name), &mut label, false)
            .unwrap_or_else(|failure| panic!("XFindShape {name}: {failure:?}"));
        let entry = entry_of(&label);
        println!("{entry}");
        entry
    }

    /// `SetName doc entry name`. Example: `session.set_name("D", "0:1:1:1", "Pin")`.
    pub fn set_name(&self, doc: &str, entry: &str, name: &str) {
        println!("SetName {doc} {entry} {name} ");
        let label = label_of(&self.documents[doc], entry);
        TDataStdName::set(&label, &ExtendedString::from_utf8(name))
            .unwrap_or_else(|failure| panic!("SetName {entry}: {failure:?}"));
    }

    /// `XSetColor doc target color...`, a target being a label or a shape variable.
    /// Example: `session.set_color("D", "s", &["0", "0", "1"])`.
    pub fn set_color(&self, doc: &str, target: &str, words: &[&str]) {
        println!("XSetColor {doc} {target} {} ", words.join(" "));
        let Some(color) = parse_color(words) else {
            script_error(&format!("XSetColor: {words:?} is no colour"));
        };
        let main = main_of(&self.documents[doc]);
        let tool = XCAFDocDocumentTool::color_tool(&main).expect("a color tool");
        let tool = tool.get().expect("a color tool");
        let label = label_of(&self.documents[doc], target);
        if !label.is_null() {
            tool.set_color(&label, &color, XCAFDocColorType::ColorGen)
                .unwrap_or_else(|failure| panic!("XSetColor {target}: {failure:?}"));
            return;
        }
        let done = tool
            .set_shape_color(
                &self.bound_shape(target),
                &color,
                XCAFDocColorType::ColorGen,
            )
            .unwrap_or_else(|failure| panic!("XSetColor {target}: {failure:?}"));
        if !done {
            script_error(&format!("Syntax error: {target} is not a label nor shape"));
        }
    }

    /// `XExpand doc 0 entry`: the compound of a label made an assembly, not recursively.
    /// Example: `session.expand_label("D", "0:1:1:1")`.
    pub fn expand_label(&self, doc: &str, entry: &str) {
        println!("XExpand {doc} 0 {entry} ");
        let main = main_of(&self.documents[doc]);
        let label = label_of(&self.documents[doc], entry);
        let done = XCAFDocEditor::expand_shape(&main, &label, false)
            .unwrap_or_else(|failure| panic!("XExpand {entry}: {failure:?}"));
        println!(
            "{}",
            if done {
                ""
            } else {
                "No suitable labels to expand"
            }
        );
    }

    /// `XGetAllColors doc`: the colour names, each followed by a blank.
    /// Example: `session.all_colors("D")`.
    pub fn all_colors(&self, doc: &str) -> String {
        println!("XGetAllColors {doc} ");
        let main = main_of(&self.documents[doc]);
        let tool = XCAFDocDocumentTool::color_tool(&main).expect("a color tool");
        let mut text = String::new();
        for label in tool
            .get()
            .expect("a color tool")
            .get_colors()
            .expect("colors")
        {
            let color = XCAFDocColorTool::get_color(&label)
                .expect("a color")
                .expect("a colour label");
            text += Color::string_name(color.name());
            text.push(' ');
        }
        println!("{text}");
        text
    }

    /// `XGetLengthUnit name`: the unit's name, as `dumpLengthUnit` writes it.
    /// Example: `session.get_length_unit("D_First")`.
    pub fn get_length_unit(&self, name: &str) -> String {
        println!("XGetLengthUnit {name} ");
        let root = main_of(&self.documents[name]).root().expect("a root label");
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
    pub fn get_one_shape(&mut self, shape_name: &str, name: &str) {
        println!("XGetOneShape {shape_name} {name} ");
        let main = main_of(&self.documents[name]);
        let tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
        let labels = tool
            .get()
            .expect("a shape tool")
            .get_free_shapes()
            .expect("free shapes");
        if labels.is_empty() {
            println!("Document {name} is empty");
            script_error(&format!("XGetOneShape {name}"));
        }
        let shape = one_shape(&labels);
        if shape.is_null() {
            println!("No shape found in the document");
            script_error(&format!("XGetOneShape {name}"));
        }
        self.raster.bind(shape_name, &shape);
        println!("{shape_name}");
    }

    /// `read_file`'s new document: bound under its name, with the name on its root.
    pub(crate) fn new_named_document(&mut self, name: &str) {
        let document = new_bin_document(name);
        name_root(&document, name);
        self.documents.insert(name.to_owned(), document);
    }

    /// `Close name`: a name that is no document is only complained about.
    /// Example: `session.close_document("D_First")`.
    pub fn close_document(&mut self, name: &str) {
        println!("Close {name} ");
        let Some(document) = self.documents.remove(name) else {
            println!("{name} is not a Document");
            return;
        };
        XCAFAppApplication::get_application()
            .close_tdoc_std_document(&document)
            .unwrap_or_else(|failure| panic!("Close {name}: {failure:?}"));
    }

    /// `Close name -silent`: nothing said of a name that is no document.
    /// Example: `session.close_silently("D")`.
    pub fn close_silently(&mut self, name: &str) {
        println!("Close {name} -silent ");
        if let Some(document) = self.documents.remove(name) {
            XCAFAppApplication::get_application()
                .close_tdoc_std_document(&document)
                .unwrap_or_else(|failure| panic!("Close {name}: {failure:?}"));
        }
    }
}

fn shape_of(label: &TdfLabel) -> Shape {
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
