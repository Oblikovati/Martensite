//! `IgesCafControlWriter`: writes a document to an IGES file, with colors, names and layers.
use crate::iges_control::IgesControlWriter;
use crate::iges_data::iges_live;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::units_methods::LengthUnit;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocLengthUnit, XCAFDocShapeTool};
use martensite_tkxsbase::xs_algo::{XSAlgoShapeProcessor, xs_algo_units};
use martensite_tkxsbase::xs_control::XSControlWorkSession;

const WHERE: &str = "IgesCafControlWriter";

/// Writes a document to an IGES file.
///
/// ```no_run
/// use martensite_tkdeiges::igescaf_control::IgesCafControlWriter;
/// let writer = IgesCafControlWriter::new().unwrap();
/// assert!(writer.get_color_mode());
/// ```
pub struct IgesCafControlWriter {
    pub(super) base: IgesControlWriter,
    color_mode: bool,
    name_mode: bool,
    layer_mode: bool,
}

impl IgesCafControlWriter {
    /// A writer with all modes; `IgesCafControlWriter::new()?`.
    ///
    /// # Errors
    /// A failure of the init of the norm.
    pub fn new() -> Result<Self, Failure> {
        Ok(Self {
            base: IgesControlWriter::new()?,
            color_mode: true,
            name_mode: true,
            layer_mode: true,
        })
    }
    /// A writer whose model and process the session takes; `IgesCafControlWriter::with_session(&ws)?`.
    ///
    /// # Errors
    /// A failure of the init of the norm or of the session.
    pub fn with_session(ws: &Handle<XSControlWorkSession>) -> Result<Self, Failure> {
        let writer = Self::new()?;
        let session = iges_live(ws, WHERE)?;
        session.set_model(&writer.base.model().cast(), false)?;
        session.set_map_writer(&writer.base.transfer_process())?;
        Ok(writer)
    }
    /// A writer in a unit whose model and process the session takes;
    /// `IgesCafControlWriter::with_unit(&ws, b"MM")?`.
    ///
    /// # Errors
    /// A failure of the init of the norm or of the session.
    pub fn with_unit(ws: &Handle<XSControlWorkSession>, unit: &[u8]) -> Result<Self, Failure> {
        let mode = martensite_tkxsbase::interface::InterfaceStatic::i_val("write.iges.brep.mode");
        let writer = Self {
            base: IgesControlWriter::with_unit(unit, mode)?,
            color_mode: true,
            name_mode: true,
            layer_mode: true,
        };
        let session = iges_live(ws, WHERE)?;
        session.set_model(&writer.base.model().cast(), false)?;
        session.set_map_writer(&writer.base.transfer_process())?;
        Ok(writer)
    }
    /// The writer of shapes under it; `writer.base()`.
    pub fn base(&self) -> &IgesControlWriter {
        &self.base
    }
    /// The writer of shapes under it; `writer.base_mut()`.
    pub fn base_mut(&mut self) -> &mut IgesControlWriter {
        &mut self.base
    }
    /// Sets whether colors are written; `writer.set_color_mode(true)`.
    pub fn set_color_mode(&mut self, mode: bool) {
        self.color_mode = mode;
    }
    /// Whether colors are written; `writer.get_color_mode()`.
    pub fn get_color_mode(&self) -> bool {
        self.color_mode
    }
    /// Sets whether names are written; `writer.set_name_mode(true)`.
    pub fn set_name_mode(&mut self, mode: bool) {
        self.name_mode = mode;
    }
    /// Whether names are written; `writer.get_name_mode()`.
    pub fn get_name_mode(&self) -> bool {
        self.name_mode
    }
    /// Sets whether layers are written; `writer.set_layer_mode(true)`.
    pub fn set_layer_mode(&mut self, mode: bool) {
        self.layer_mode = mode;
    }
    /// Whether layers are written; `writer.get_layer_mode()`.
    pub fn get_layer_mode(&self) -> bool {
        self.layer_mode
    }
    /// The unit of the model from the document, or from the settings.
    fn prepare_unit(&self, label: &TdfLabel) -> Result<(), Failure> {
        let model = self.base.model();
        let live = iges_live(&model, WHERE)?;
        let mut section = live.global_section();
        let mut unit = Handle::<XCAFDocLengthUnit>::default();
        let found = !label.is_null()
            && label
                .root()?
                .find_attribute_as(&XCAFDocLengthUnit::get_id(), &mut unit)?;
        if found {
            section.set_cascade_unit(iges_live(&unit, WHERE)?.get_unit_value() * 1000.0);
        } else {
            XSAlgoShapeProcessor::prepare_for_transfer();
            section
                .set_cascade_unit(xs_algo_units().get_cascade_length_unit(LengthUnit::Millimeter));
        }
        live.set_global_section(&section);
        Ok(())
    }
    /// Translates the free shapes of a document; `writer.transfer_document(&document, &range)?`.
    ///
    /// # Errors
    /// A failure of the document or of a translation.
    pub fn transfer_document(
        &mut self,
        document: &Handle<TDocStdDocument>,
        range: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let main = iges_live(document, WHERE)?.main()?;
        let tool = XCAFDocDocumentTool::shape_tool(&main)?;
        let Some(shapes) = tool.get() else {
            return Ok(false);
        };
        let labels = shapes.get_free_shapes()?;
        self.transfer_labels(&labels, range)
    }
    /// Translates the shape of a label; `writer.transfer_label(&label, &range)?`.
    ///
    /// # Errors
    /// A failure of the document or of a translation.
    pub fn transfer_label(
        &mut self,
        label: &TdfLabel,
        range: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.transfer_labels(std::slice::from_ref(label), range)
    }
    /// Translates the shapes of labels with their attributes, and computes the model; gives false
    /// without a label; `writer.transfer_labels(&labels, &range)?`.
    ///
    /// # Errors
    /// A failure of the document or of a translation.
    pub fn transfer_labels(
        &mut self,
        labels: &[TdfLabel],
        range: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let Some(first) = labels.first() else {
            return Ok(false);
        };
        self.prepare_unit(first)?;
        let scope = ProgressScope::with_literal(range, "Labels", labels.len() as f64, false);
        for label in labels {
            if !scope.more() {
                break;
            }
            let mut shape = Shape::default();
            let _ = XCAFDocShapeTool::get_shape(label, &mut shape)?;
            if !shape.is_null() {
                let _ = self.base.add_shape(&shape, &scope.next(1.0))?;
            }
        }
        if self.color_mode {
            let _ = self.write_attributes(labels)?;
        }
        if self.layer_mode {
            let _ = self.write_layers(labels)?;
        }
        if self.name_mode {
            let _ = self.write_names(labels)?;
        }
        self.base.compute_model()?;
        Ok(true)
    }
    /// Translates a document and writes the file; `writer.perform(&document, "a.igs", &range)?`.
    ///
    /// # Errors
    /// A failure of the document, a translation or the writing.
    pub fn perform(
        &mut self,
        document: &Handle<TDocStdDocument>,
        file_name: &str,
        range: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        if !self.transfer_document(document, range)? {
            return Ok(false);
        }
        self.base.write_file(file_name, false)
    }
}
