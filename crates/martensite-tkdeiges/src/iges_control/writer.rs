//! `IgesControlWriter`: writes shapes and geometry to an IGES model and a file.
use super::IgesControlController;
use crate::brep_to_iges::BRepToIgesBREntity;
use crate::brep_to_iges_brep::BRepToIgesBRepEntity;
use crate::de_iges::xs_shape_fix_parameters;
use crate::geom_to_iges::{GeomToIgesGeomCurve, GeomToIgesGeomSurface};
use crate::iges_data::{
    IgesDataBasicEditor, IgesDataIgesEntity, IgesDataIgesModel, IgesDataIgesWriter, iges_live,
};
use crate::iges_select::IgesSelectWorkLibrary;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::geom::{GeomCurve, GeomGeometry, GeomSurface};
use martensite_tkg3d::geom_adaptor::{GeomAdaptorCurve, GeomAdaptorSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::bnd_lib::{Add3dCurve, AddSurface};
use martensite_tkmath::bnd::BoundingBox3 as BoundingBox;
use martensite_tkmath::gp::Xyz;
use martensite_tkshhealing::shape_analysis::ShapeAnalysisShapeTolerance;
use martensite_tkshhealing::shape_process::{ShapeProcessOperation, ShapeProcessOperationsFlags};
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use martensite_tkxsbase::interface::{InterfaceModelEntities, InterfaceStatic};
use martensite_tkxsbase::transfer::TransferFinderProcess;
use martensite_tkxsbase::xs_algo::{
    DEShapeFixParameters, XSAlgoParameterMap, XSAlgoShapeProcessor,
};
use std::fs::File;
use std::io::{BufWriter, Write};

const WHERE: &str = "IgesControlWriter";

/// The parameters and the flags of the processing of shapes before their translation.
#[derive(Default)]
struct WriterProcessing {
    parameters: XSAlgoParameterMap,
    flags: ShapeProcessOperationsFlags,
    flags_set: bool,
}

/// Translates shapes to a model and writes it.
///
/// ```no_run
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkdeiges::iges_control::IgesControlWriter;
/// use martensite_tkernel::message::ProgressRange;
/// let mut writer = IgesControlWriter::new().unwrap();
/// writer.add_shape(&Shape::default(), &ProgressRange::default()).unwrap();
/// writer.write_file("part.igs", false).unwrap();
/// ```
pub struct IgesControlWriter {
    process: Handle<TransferFinderProcess>,
    model: Handle<IgesDataIgesModel>,
    editor: IgesDataBasicEditor,
    write_mode: i32,
    is_computed: bool,
    processing: WriterProcessing,
}

impl IgesControlWriter {
    fn with_editor(editor: IgesDataBasicEditor, write_mode: i32) -> Self {
        Self {
            process: TransferFinderProcess::new(10000),
            model: editor.model(),
            editor,
            write_mode,
            is_computed: false,
            processing: WriterProcessing::default(),
        }
    }
    /// A writer on a new model, in the unit and the mode of the settings;
    /// `IgesControlWriter::new()?`.
    ///
    /// # Errors
    /// A failure of the init of the norm.
    pub fn new() -> Result<Self, Failure> {
        let unit = InterfaceStatic::c_val("write.iges.unit");
        let mode = InterfaceStatic::i_val("write.iges.brep.mode");
        Self::with_unit(unit.as_bytes(), mode)
    }
    /// A writer on a new model, in a unit and a mode (0 faces, 1 BRep);
    /// `IgesControlWriter::with_unit(b"MM", 0)?`.
    ///
    /// # Errors
    /// A failure of the init of the norm.
    pub fn with_unit(unit: &[u8], mode: i32) -> Result<Self, Failure> {
        IgesControlController::init()?;
        let mut editor = IgesDataBasicEditor::new();
        editor.init(&IgesSelectWorkLibrary::define_protocol());
        editor.set_unit_name(unit);
        editor.apply_unit(false);
        Ok(Self::with_editor(editor, mode))
    }
    /// A writer on a model, in a mode; `IgesControlWriter::with_model(&model, 0)`.
    pub fn with_model(model: &Handle<IgesDataIgesModel>, mode: i32) -> Self {
        let protocol = IgesSelectWorkLibrary::define_protocol();
        Self::with_editor(IgesDataBasicEditor::with_model(model, &protocol), mode)
    }
    /// The model; `writer.model()`.
    pub fn model(&self) -> Handle<IgesDataIgesModel> {
        self.model.clone()
    }
    /// The process; `writer.transfer_process()`.
    pub fn transfer_process(&self) -> Handle<TransferFinderProcess> {
        self.process.clone()
    }
    /// Sets the process; `writer.set_transfer_process(&process)`.
    pub fn set_transfer_process(&mut self, process: &Handle<TransferFinderProcess>) {
        self.process = process.clone();
    }
    /// Sets the parameters of the processing of shapes; `writer.set_shape_fix_parameters(map)`.
    pub fn set_shape_fix_parameters(&mut self, parameters: XSAlgoParameterMap) {
        self.processing.parameters = parameters;
    }
    /// Sets the parameters from the fix parameters of the exchange;
    /// `writer.set_shape_fix_parameters_from(&parameters, &additional)`.
    pub fn set_shape_fix_parameters_from(
        &mut self,
        parameters: &DEShapeFixParameters,
        additional: &XSAlgoParameterMap,
    ) {
        XSAlgoShapeProcessor::set_shape_fix_parameters(
            parameters,
            additional,
            &mut self.processing.parameters,
        );
    }
    /// The parameters of the processing of shapes; `writer.shape_fix_parameters()`.
    pub fn shape_fix_parameters(&self) -> &XSAlgoParameterMap {
        &self.processing.parameters
    }
    /// Sets the operations of the processing of shapes; `writer.set_shape_process_flags(flags)`.
    pub fn set_shape_process_flags(&mut self, flags: ShapeProcessOperationsFlags) {
        self.processing.flags = flags;
        self.processing.flags_set = true;
    }
    /// The operations of the processing of shapes; `writer.shape_process_flags()`.
    pub fn shape_process_flags(&self) -> ShapeProcessOperationsFlags {
        self.processing.flags
    }
    fn initialize_missing_parameters(&mut self) {
        if self.processing.parameters.is_empty() {
            self.set_shape_fix_parameters_from(
                &xs_shape_fix_parameters(),
                &XSAlgoParameterMap::default(),
            );
        }
        if !self.processing.flags_set {
            self.processing
                .flags
                .set(ShapeProcessOperation::DirectFaces);
            self.processing.flags_set = true;
        }
    }
    /// The resolution of the model with a new shape, by the mode of the settings.
    fn updated_resolution(
        &self,
        shape: &Shape,
        (old_resolution, old_nb, new_nb): (f64, i32, i32),
    ) -> Result<f64, Failure> {
        let mode = InterfaceStatic::i_val("write.precision.mode");
        if mode == 2 {
            return Ok(InterfaceStatic::r_val("write.precision.val"));
        }
        let mut analysis = ShapeAnalysisShapeTolerance::default();
        let vertices = analysis.tolerance(shape, mode, ShapeEnum::Vertex)?;
        let edges = analysis.tolerance(shape, mode, ShapeEnum::Edge)?;
        Ok(if mode == 0 {
            let added = (vertices + edges) / 2.0;
            (old_resolution * f64::from(old_nb) + added * f64::from(new_nb - old_nb))
                / f64::from(new_nb)
        } else if mode < 0 {
            let least = vertices.min(edges);
            if old_nb > 0 {
                old_resolution.min(least)
            } else {
                least
            }
        } else {
            let most = vertices.max(edges);
            if old_nb > 0 {
                old_resolution.max(most)
            } else {
                most
            }
        })
    }
    /// Takes a box in the model maximal coordinate of the header.
    fn extend_with_box(&self, boxed: &BoundingBox) -> Result<(), Failure> {
        let open = boxed.is_void()
            || boxed.is_open_xmax()
            || boxed.is_open_ymax()
            || boxed.is_open_zmax()
            || boxed.is_open_xmin()
            || boxed.is_open_ymin()
            || boxed.is_open_zmin();
        if open {
            return Ok(());
        }
        self.extend_header(boxed)
    }
    fn extend_header(&self, boxed: &BoundingBox) -> Result<(), Failure> {
        let live = iges_live(&self.model, WHERE)?;
        let mut section = live.global_section();
        let unit = section.unit_value();
        let [x_min, y_min, z_min, x_max, y_max, z_max] = boxed.get()?;
        section.max_max_coords(&Xyz::new(x_max / unit, y_max / unit, z_max / unit));
        section.max_max_coords(&Xyz::new(x_min / unit, y_min / unit, z_min / unit));
        live.set_global_section(&section);
        Ok(())
    }
    /// Translates a shape to the model; gives false for a null shape or without an entity;
    /// `writer.add_shape(&shape, &range)?`.
    ///
    /// # Errors
    /// A failure of the processing or of the translation of the shape.
    pub fn add_shape(&mut self, shape: &Shape, range: &ProgressRange<'_>) -> Result<bool, Failure> {
        if shape.is_null() {
            return Ok(false);
        }
        XSAlgoShapeProcessor::prepare_for_transfer();
        self.initialize_missing_parameters();
        let scope = ProgressScope::unnamed(range, 2.0, false);
        let mut processor = XSAlgoShapeProcessor::new(
            &self.processing.parameters,
            &DEShapeFixParameters::default(),
        );
        let processed = processor.process_shape(shape, self.processing.flags, &scope.next(1.0))?;
        if !scope.more() {
            return Ok(false);
        }
        let entity = self.shape_entity(&processed, &scope)?;
        if !scope.more() || entity.is_null() {
            return Ok(false);
        }
        processor.merge_transfer_info_finder(&self.process)?;
        let live = iges_live(&self.model, WHERE)?;
        let old_nb = live.nb_entities();
        let model = self.model.clone();
        let added = self.add_entity(&entity)?;
        let live = iges_live(&model, WHERE)?;
        let new_nb = live.nb_entities();
        let mut section = live.global_section();
        let resolution =
            self.updated_resolution(&processed, (section.resolution(), old_nb, new_nb))?;
        section.set_resolution(resolution / section.unit_value());
        live.set_global_section(&section);
        let mut boxed = BoundingBox::default();
        BRepBndLib::add(&processed, &mut boxed, true)?;
        self.extend_with_box(&boxed)?;
        Ok(added)
    }
    fn shape_entity(
        &self,
        shape: &Shape,
        scope: &ProgressScope<'_>,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if self.write_mode != 0 {
            let mut tool = BRepToIgesBRepEntity::new();
            tool.set_transfer_process(&self.process);
            tool.set_model(&self.model)?;
            return tool.transfer_shape(shape, &scope.next(1.0));
        }
        let mut tool = BRepToIgesBREntity::new();
        tool.set_transfer_process(&self.process);
        tool.set_model(&self.model)?;
        tool.transfer_shape(shape, &scope.next(1.0))
    }
    /// Translates a curve or a surface to the model; gives false for another object;
    /// `writer.add_geom(&geometry)?`.
    ///
    /// # Errors
    /// A failure of the translation of the geometry.
    pub fn add_geom(&mut self, geometry: &Handle<StandardTransient>) -> Result<bool, Failure> {
        if geometry.cast::<dyn GeomGeometry>().is_null() {
            return Ok(false);
        }
        let mut boxed = BoundingBox::default();
        let mut entity = Handle::default();
        let curve = geometry.cast::<dyn GeomCurve>();
        if let Some(live) = curve.get() {
            let mut tool = GeomToIgesGeomCurve::new();
            tool.set_model(&self.model)?;
            entity = tool.transfer_curve(&curve, live.first_parameter(), live.last_parameter())?;
            Add3dCurve::add(&GeomAdaptorCurve::with_curve(&curve)?, 0.0, &mut boxed)?;
        }
        let surface = geometry.cast::<dyn GeomSurface>();
        if let Some(live) = surface.get() {
            let mut tool = GeomToIgesGeomSurface::new();
            tool.set_model(&self.model)?;
            entity = tool.transfer_surface(&surface, live.bounds())?;
            AddSurface::add(
                &GeomAdaptorSurface::with_surface(&surface)?,
                0.0,
                &mut boxed,
            )?;
        }
        self.extend_header(&boxed)?;
        self.add_entity(&entity)
    }
    /// Adds an entity with those it refers to; gives false for a null entity;
    /// `writer.add_entity(&entity)?`.
    ///
    /// # Errors
    /// A failure of the reference walk.
    pub fn add_entity(&mut self, entity: &Handle<IgesDataIgesEntity>) -> Result<bool, Failure> {
        if entity.is_null() {
            return Ok(false);
        }
        let protocol = IgesSelectWorkLibrary::define_protocol();
        iges_live(&self.model, WHERE)?.add_with_refs_protocol(
            &entity.cast(),
            &protocol.cast(),
            0,
            false,
        )?;
        self.is_computed = false;
        Ok(true)
    }
    /// Computes the status of the entities and corrects them, once between changes;
    /// `writer.compute_model()?`.
    ///
    /// # Errors
    /// A failure of the status or of a correction.
    pub fn compute_model(&mut self) -> Result<(), Failure> {
        if !self.is_computed {
            self.editor.compute_status()?;
            let _ = self.editor.auto_correct_model()?;
            self.is_computed = true;
        }
        Ok(())
    }
    /// Writes the model to a stream; `fnes` is the coded form; false without an entity;
    /// `writer.write_stream(&mut stream, false)?`.
    ///
    /// # Errors
    /// A failure of the writer on the model.
    pub fn write_stream(&mut self, stream: &mut dyn Write, fnes: bool) -> Result<bool, Failure> {
        self.compute_model()?;
        if iges_live(&self.model, WHERE)?.nb_entities() == 0 {
            return Ok(false);
        }
        let mut writer = IgesDataIgesWriter::new(&self.model);
        writer.send_model(&IgesSelectWorkLibrary::define_protocol())?;
        if fnes {
            *writer.write_mode() = 10;
        }
        writer.print(stream)
    }
    /// Writes the model to a file; `writer.write_file("part.igs", false)?`.
    ///
    /// # Errors
    /// A failure of the writer on the model.
    pub fn write_file(&mut self, path: &str, fnes: bool) -> Result<bool, Failure> {
        let Ok(file) = File::create(path) else {
            return Ok(false);
        };
        let mut stream = BufWriter::new(file);
        let written = self.write_stream(&mut stream, fnes)?;
        Ok(stream.flush().is_ok() && written)
    }
}
