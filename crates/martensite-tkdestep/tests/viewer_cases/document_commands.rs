//! The document commands that show a document or change its assemblies: `XShow`, `XDisplay`,
//! `XShowFaceBoundary`, `XRemoveComponent`, `XUpdateAssemblies`, `XSetShape`, `XAddShape`.
use crate::host::ViewerHost;
use crate::view_commands::SHADED;
use martensite_tkbrep::topods::Shape;
use martensite_tkdestep::de_step::StepControlStepModelType;
use martensite_tkdestep::stepcaf_control::{STEPCAFControlReader, STEPCAFControlWriter};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tklcaf::tdata_std::{TDataStdGenericExtString as _, TDataStdName};
use martensite_tklcaf::tdf::{TdfChildIterator, TdfLabel, TdfTool};
use martensite_tklcaf::tdoc_std::{TDocStdApplication as _, TDocStdDocument};
use martensite_tkmath::toploc::Location;
use martensite_tkservice::aspect::AspectTypeOfLine;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_tkv3d::ais::AisInteractiveObject;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use martensite_tkv3d::prs3d::Prs3dLineAspect;
use martensite_tkvcaf::tprs_std::{TPrsStdAISPresentation, TPrsStdAISViewer, TPrsStdDriverTable};
use martensite_tkxcaf::xcaf_app::XCAFAppApplication;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};
use martensite_tkxcaf::xcaf_prs::{XCAFPrsAISObject, XCAFPrsDriver};
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::xs_algo::XSAlgoShapeProcessor;
use std::collections::BTreeSet;
use std::sync::Once;

fn document_of(host: &ViewerHost, doc: &str) -> Handle<TDocStdDocument> {
    host.session
        .documents
        .get(doc)
        .unwrap_or_else(|| panic!("{doc} is not a document"))
        .clone()
}

fn shape_tool(document: &Handle<TDocStdDocument>) -> Handle<XCAFDocShapeTool> {
    XCAFDocDocumentTool::shape_tool(&document.get().unwrap().main().unwrap()).unwrap()
}

/// The label of an entry, as `TdfTool::label` finds it. Example: `label_at(&document, "0:1:1:1")`.
pub fn label_at(document: &Handle<TDocStdDocument>, entry: &str) -> TdfLabel {
    let mut label = TdfLabel::default();
    let data = document.get().unwrap().get_data();
    TdfTool::label_from_entry(&data, &AsciiString::from(entry), &mut label, false).unwrap();
    label
}

fn register_presentation_driver() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        // False when the application registered it first, which is as good.
        let _ = TPrsStdDriverTable::get().add_driver(
            &XCAFPrsDriver::get_id(),
            &Handle::new(XCAFPrsDriver::new()).cast(),
        );
    });
}

impl crate::xs_session::XsSession {
    /// A new document under the name, saying nothing, as the commands that make one on the way do.
    /// Example: `session.new_named_document_silently("D")`.
    pub fn new_named_document_silently(&mut self, name: &str) {
        let application = XCAFAppApplication::get_application();
        let mut document: Handle<TDocStdDocument> = Handle::default();
        application
            .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
            .unwrap();
        self.documents.insert(name.to_owned(), document);
    }
}

impl ViewerHost {
    /// `XShow doc`: every free shape, each as the document's presentation.
    pub fn xshow(&mut self, doc: &str) {
        let document = document_of(self, doc);
        let labels = shape_tool(&document).get_free_shapes().unwrap();
        println!("XShow {doc} ");
        self.show_labels(&document, &labels);
    }

    /// `XShow doc entry`.
    pub fn xshow_label(&mut self, doc: &str, entry: &str) {
        let document = document_of(self, doc);
        println!("XShow {doc} {entry} ");
        let labels = vec![label_at(&document, entry)];
        self.show_labels(&document, &labels);
    }

    fn show_labels(&mut self, document: &Handle<TDocStdDocument>, labels: &[TdfLabel]) {
        register_presentation_driver();
        let root = document.get().unwrap().get_data().root();
        let mut viewer = Handle::<TPrsStdAISViewer>::default();
        if !TPrsStdAISViewer::find(&root, &mut viewer).unwrap() {
            self.open_scene();
            TPrsStdAISViewer::new_with_context(&root, &self.live_scene().context).unwrap();
        }
        for label in labels {
            let mut presentation = Handle::<TPrsStdAISPresentation>::default();
            if !label
                .find_attribute_as(&TPrsStdAISPresentation::get_id(), &mut presentation)
                .unwrap()
            {
                presentation =
                    TPrsStdAISPresentation::set(label, &XCAFPrsDriver::get_id()).unwrap();
                presentation
                    .set_material(Graphic3dNameOfMaterial::Plastified)
                    .unwrap();
            }
            presentation.display(true).unwrap();
        }
        TPrsStdAISViewer::update_label(&root).unwrap();
    }

    /// `XShowFaceBoundary doc entry 1 r g b width style`, the colour in 0-255.
    pub fn xshow_face_boundary(
        &self,
        doc: &str,
        entry: &str,
        rgb: [f64; 3],
        width: f64,
        style: AspectTypeOfLine,
    ) {
        println!(
            "XShowFaceBoundary {doc} {entry} 1 {} {} {} {width} {} ",
            rgb[0],
            rgb[1],
            rgb[2],
            style as i32 - AspectTypeOfLine::Solid as i32
        );
        let document = document_of(self, doc);
        let label = label_at(&document, entry);
        let mut presentation = Handle::<TPrsStdAISPresentation>::default();
        assert!(
            label
                .find_attribute_as(&TPrsStdAISPresentation::get_id(), &mut presentation)
                .unwrap()
        );
        let object: Handle<dyn AisInteractiveObject> = presentation.get_ais();
        let attributes = object.get().unwrap().attributes();
        let drawer = attributes.get().unwrap();
        drawer.set_face_boundary_draw(true);
        let color = Color::new(
            rgb[0] / 255.0,
            rgb[1] / 255.0,
            rgb[2] / 255.0,
            ColorSpace::Rgb,
        )
        .unwrap();
        drawer.set_face_boundary_aspect(Handle::new(
            Prs3dLineAspect::new(color, style, width).unwrap(),
        ));
        let scene = self.live_scene();
        scene.context.redisplay(&object, true, false).unwrap();
    }

    /// `XDisplay -dispMode 1 doc [-explore]`: the free shapes under their names, each its own object.
    pub fn xdisplay_shaded(&mut self, doc: &str, explore: bool) {
        println!(
            "XDisplay -dispMode 1 {doc}{} ",
            if explore { " -explore" } else { "" }
        );
        let document = document_of(self, doc);
        let labels = shape_tool(&document).get_free_shapes().unwrap();
        let mut names = BTreeSet::new();
        let mut shown = String::new();
        for label in &labels {
            self.display_label(
                label,
                &format!("{doc}:"),
                &Location::default(),
                explore,
                &mut names,
                &mut shown,
            );
        }
        println!("{shown}");
    }

    fn display_label(
        &mut self,
        label: &TdfLabel,
        prefix: &str,
        parent: &Location,
        explore: bool,
        names: &mut BTreeSet<String>,
        shown: &mut String,
    ) {
        let name = format!("{prefix}{}", unique_name(label, names));
        let mut referred = label.clone();
        XCAFDocShapeTool::get_referred_shape(label, &mut referred).unwrap();
        if explore && XCAFDocShapeTool::is_assembly(&referred).unwrap() {
            let location = parent
                .multiplied(&XCAFDocShapeTool::get_location(label).unwrap())
                .unwrap();
            let mut children = TdfChildIterator::new(&referred, false).unwrap();
            while children.more() {
                self.display_label(
                    &children.value(),
                    &format!("{name}/"),
                    &location,
                    explore,
                    names,
                    shown,
                );
                children.next();
            }
            return;
        }
        let object = XCAFPrsAISObject::new(label).unwrap();
        if !parent.is_identity() {
            object
                .set_local_transformation(&parent.transformation())
                .unwrap();
        }
        let cast: Handle<dyn AisInteractiveObject> = object.cast();
        self.scene
            .as_mut()
            .expect("an open viewer")
            .display_object(&name, cast, Some(SHADED));
        shown.push_str(&name);
        shown.push(' ');
    }

    /// `XGetShape name doc entry`.
    pub fn xget_shape(&mut self, name: &str, doc: &str, entry: &str) {
        println!("XGetShape {name} {doc} {entry} ");
        let document = document_of(self, doc);
        let mut shape = Shape::default();
        XCAFDocShapeTool::get_shape(&label_at(&document, entry), &mut shape).unwrap();
        self.bind(name, &shape);
    }

    /// `XAddShape doc name 1`: an assembly from a compound.
    pub fn xadd_shape_assembly(&self, doc: &str, name: &str) {
        println!("XAddShape {doc} {name} 1 ");
        let document = document_of(self, doc);
        let label = shape_tool(&document)
            .add_shape(&self.shape(name), true, true)
            .unwrap();
        let mut entry = AsciiString::default();
        TdfTool::entry(&label, &mut entry);
        println!("{}", String::from_utf8_lossy(entry.as_bytes()));
    }

    /// `SetName doc entry name`.
    pub fn set_name(&self, doc: &str, entry: &str, name: &str) {
        println!("SetName {doc} {entry} {name} ");
        let document = document_of(self, doc);
        let label = label_at(&document, entry);
        TDataStdName::set(&label, &ExtendedString::from_utf8(name)).unwrap();
    }

    /// `XRemoveComponent doc entry`.
    pub fn xremove_component(&self, doc: &str, entry: &str) {
        println!("XRemoveComponent {doc} {entry} ");
        let document = document_of(self, doc);
        shape_tool(&document)
            .remove_component(&label_at(&document, entry))
            .unwrap();
    }

    /// `XUpdateAssemblies doc`.
    pub fn xupdate_assemblies(&self, doc: &str) {
        println!("XUpdateAssemblies {doc} ");
        shape_tool(&document_of(self, doc))
            .update_assemblies()
            .unwrap();
    }

    /// `XSetShape doc entry name`.
    pub fn xset_shape(&self, doc: &str, entry: &str, name: &str) {
        println!("XSetShape {doc} {entry} {name} ");
        let document = document_of(self, doc);
        shape_tool(&document)
            .set_shape(&label_at(&document, entry), &self.shape(name))
            .unwrap();
    }

    /// `ReadStep doc file -stream`: the same read as the file's, through a stream.
    pub fn read_step_stream(&mut self, doc: &str, file: &str) {
        println!("ReadStep {doc} {file} -stream ");
        println!(" File STEP to read : {file}");
        self.session.select_step_norm().unwrap();
        let mut reader =
            STEPCAFControlReader::with_session(&self.session.work_session, true).unwrap();
        let bytes = std::fs::read(file).unwrap_or_else(|cause| panic!("{file}: {cause}"));
        let short = file.rsplit('/').next().unwrap_or(file);
        let status = reader.read_stream(short, &mut bytes.as_slice()).unwrap();
        assert_eq!(status, IFSelectReturnStatus::RetDone, "ReadStep {file}");
        self.session.new_named_document_silently(doc);
        let (fix, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("read.step.resource.name"),
            &AsciiString::from("read.step.sequence"),
        )
        .unwrap();
        reader.set_shape_fix_parameters(fix).unwrap();
        reader.set_shape_process_flags(flags).unwrap();
        let document = self.session.documents[doc].clone();
        assert!(
            reader
                .transfer(&document, &ProgressRange::default())
                .unwrap(),
            "ReadStep {file}"
        );
        println!("Document saved with name {doc}");
    }

    /// `WriteStep doc file -stream`: the same write as the file's, through a stream.
    pub fn write_step_stream(&mut self, doc: &str, file: &str) {
        println!("WriteStep {doc} {file} -stream ");
        self.session.select_step_norm().unwrap();
        let mut writer =
            STEPCAFControlWriter::with_session(&self.session.work_session, true).unwrap();
        let (fix, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("write.step.resource.name"),
            &AsciiString::from("write.step.sequence"),
        )
        .unwrap();
        writer.set_shape_fix_parameters(fix).unwrap();
        writer.set_shape_process_flags(flags).unwrap();
        let mut result = format!("Translating document {doc} to STEP\n");
        let document = self.session.documents[doc].clone();
        let translated = writer
            .transfer_document(
                &document,
                StepControlStepModelType::AS_IS,
                None,
                &ProgressRange::default(),
            )
            .unwrap();
        assert!(translated, "WriteStep {doc}: the document gives no result");
        result += &format!("Writing STEP file {file}\n");
        let mut bytes = Vec::new();
        assert_eq!(
            writer.write_stream(&mut bytes).unwrap(),
            IFSelectReturnStatus::RetDone
        );
        std::fs::write(file, bytes).unwrap_or_else(|cause| panic!("{file}: {cause}"));
        println!("{result}File {file} written");
    }

    /// `fromshape name`: what the pilot command writes about where the shape came from.
    pub fn fromshape(&self, name: &str) -> String {
        let vars = self.session.work_session.get().unwrap().vars();
        vars.set_shape(name, &self.shape(name));
        self.session.pilot_command(&format!("fromshape {name}"))
    }
}

/// The label's name or its definition's, else its entry, numbered on a collision.
fn unique_name(label: &TdfLabel, names: &mut BTreeSet<String>) -> String {
    let base = label_name(label);
    let mut name = base.clone();
    let mut index = 1;
    while !names.insert(name.clone()) {
        name = format!("{base}_{index}");
        index += 1;
    }
    name
}

fn label_name(label: &TdfLabel) -> String {
    if let Some(own) = stored_name(label) {
        return own;
    }
    let mut referred = TdfLabel::default();
    if XCAFDocShapeTool::get_referred_shape(label, &mut referred).unwrap()
        && let Some(definition) = stored_name(&referred)
    {
        return definition;
    }
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    String::from_utf8_lossy(entry.as_bytes()).into_owned()
}

fn stored_name(label: &TdfLabel) -> Option<String> {
    let mut name = Handle::<TDataStdName>::default();
    let found = label
        .find_attribute_as(&TDataStdName::get_id(), &mut name)
        .unwrap();
    let text = found.then(|| name.get().unwrap().get().to_string())?;
    (!text.is_empty()).then_some(text)
}
