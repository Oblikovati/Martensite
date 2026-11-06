//! `BRepToIgesBREntity`: the process and the model of a translation of shapes, and any shape.
use super::{BRepToIgesBRShell, BRepToIgesBRSolid, BRepToIgesBRWire};
use crate::iges_data::{IgesDataIgesEntity, IgesDataIgesModel, iges_live};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkxsbase::interface::InterfaceStatic;
use martensite_tkxsbase::transfer::{
    TransferBinder, TransferFinder, TransferFinderProcess, TransferSimpleBinderOfTransient,
    TransferTransientMapper,
};
use martensite_tkxsbase::transfer_brep::TransferBRepShapeMapper;

const WHERE: &str = "BRepToIgesBREntity";

/// What a shape of a face copy was in the original face.
pub type BRepToIgesOriginMap = DataMap<Shape, Shape>;

/// The process and the model of a translation.
///
/// ```
/// use martensite_tkdeiges::brep_to_iges::BRepToIgesBREntity;
/// assert_eq!(BRepToIgesBREntity::new().get_unit(), 1.0);
/// ```
#[derive(Clone)]
pub struct BRepToIgesBREntity {
    model: Handle<IgesDataIgesModel>,
    unit_factor: f64,
    conv_surface: bool,
    pcurve_mode: bool,
    process: Handle<TransferFinderProcess>,
}

impl Default for BRepToIgesBREntity {
    fn default() -> Self {
        Self::new()
    }
}

fn shape_finder(start: &Shape) -> Handle<dyn TransferFinder> {
    Handle::new(TransferBRepShapeMapper::new(start)).cast()
}

fn transient_finder(start: &Handle<StandardTransient>) -> Handle<dyn TransferFinder> {
    Handle::new(TransferTransientMapper::new(start)).cast()
}

impl BRepToIgesBREntity {
    /// A tool with the modes of the settings; `BRepToIgesBREntity::new()`.
    pub fn new() -> Self {
        Self {
            model: Handle::default(),
            unit_factor: 1.0,
            conv_surface: InterfaceStatic::i_val("write.convertsurface.mode") != 0,
            pcurve_mode: InterfaceStatic::i_val("write.surfacecurve.mode") != 0,
            process: TransferFinderProcess::new(10000),
        }
    }
    /// Back to a new process, the unit at 1 and the modes of the settings; `tool.init()`.
    pub fn init(&mut self) {
        let model = self.model.clone();
        *self = Self::new();
        self.model = model;
    }
    /// Sets the model; a unit other than 1 of its global section is the unit;
    /// `tool.set_model(&model)?`.
    ///
    /// # Errors
    /// A null object failure for a null model.
    pub fn set_model(&mut self, model: &Handle<IgesDataIgesModel>) -> Result<(), Failure> {
        let unit = iges_live(model, WHERE)?.global_section().unit_value();
        self.model = model.clone();
        if unit != 1.0 {
            self.unit_factor = unit;
        }
        Ok(())
    }
    /// The model; `tool.get_model()`.
    pub fn get_model(&self) -> Handle<IgesDataIgesModel> {
        self.model.clone()
    }
    /// The unit; `tool.get_unit()`.
    pub fn get_unit(&self) -> f64 {
        self.unit_factor
    }
    /// Sets the process; `tool.set_transfer_process(&process)`.
    pub fn set_transfer_process(&mut self, process: &Handle<TransferFinderProcess>) {
        self.process = process.clone();
    }
    /// The process; `tool.get_transfer_process()`.
    pub fn get_transfer_process(&self) -> Handle<TransferFinderProcess> {
        self.process.clone()
    }
    /// Whether surfaces are converted to B-splines; `tool.get_convert_surface_mode()`.
    pub fn get_convert_surface_mode(&self) -> bool {
        self.conv_surface
    }
    /// Whether pcurves are written; `tool.get_p_curve_mode()`.
    pub fn get_p_curve_mode(&self) -> bool {
        self.pcurve_mode
    }
    /// The entity of any shape; `tool.transfer_shape(&shape, &range)?`.
    ///
    /// # Errors
    /// A failure of the geometry of the shape.
    pub fn transfer_shape(
        &self,
        start: &Shape,
        range: &ProgressRange,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let wire_tool = || BRepToIgesBRWire::with_entity(self);
        let shell_tool = || BRepToIgesBRShell::with_entity(self);
        let solid_tool = || BRepToIgesBRSolid::with_entity(self);
        match start.shape_type()? {
            ShapeEnum::Vertex => wire_tool().transfer_vertex(&topods::vertex(start)?),
            ShapeEnum::Edge => wire_tool().transfer_edge(
                &topods::edge(start)?,
                &BRepToIgesOriginMap::default(),
                false,
            ),
            ShapeEnum::Wire => wire_tool().transfer_wire(&topods::wire(start)?),
            ShapeEnum::Face => shell_tool().transfer_face(&topods::face(start)?, range),
            ShapeEnum::Shell => shell_tool().transfer_shell(&topods::shell(start)?, range),
            ShapeEnum::Solid => solid_tool().transfer_solid(&topods::solid(start)?, range),
            ShapeEnum::CompSolid => {
                solid_tool().transfer_comp_solid(&topods::comp_solid(start)?, range)
            }
            ShapeEnum::Compound => solid_tool().transfer_compound(&topods::compound(start)?, range),
            ShapeEnum::Shape => Ok(Handle::default()),
        }
    }
    /// Records a fail on a shape; `tool.add_fail(&shape, "text")?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn add_fail(&self, start: &Shape, text: &str) -> Result<(), Failure> {
        iges_live(&self.process, WHERE)?.add_fail_text(&shape_finder(start), text, "");
        Ok(())
    }
    /// Records a warning on a shape; `tool.add_warning(&shape, "text")?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn add_warning(&self, start: &Shape, text: &str) -> Result<(), Failure> {
        iges_live(&self.process, WHERE)?.add_warning_text(&shape_finder(start), text, "");
        Ok(())
    }
    /// Records a fail on an object; `tool.add_fail_transient(&object, "text")?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn add_fail_transient(
        &self,
        start: &Handle<StandardTransient>,
        text: &str,
    ) -> Result<(), Failure> {
        iges_live(&self.process, WHERE)?.add_fail_text(&transient_finder(start), text, "");
        Ok(())
    }
    /// Records a warning on an object; `tool.add_warning_transient(&object, "text")?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn add_warning_transient(
        &self,
        start: &Handle<StandardTransient>,
        text: &str,
    ) -> Result<(), Failure> {
        iges_live(&self.process, WHERE)?.add_warning_text(&transient_finder(start), text, "");
        Ok(())
    }
    fn found_binder(
        &self,
        finder: &Handle<dyn TransferFinder>,
    ) -> Result<Handle<TransferSimpleBinderOfTransient>, Failure> {
        Ok(iges_live(&self.process, WHERE)?.find(finder).cast())
    }
    fn found_result(
        &self,
        finder: &Handle<dyn TransferFinder>,
    ) -> Result<Handle<StandardTransient>, Failure> {
        let binder = self.found_binder(finder)?;
        Ok(match binder.get() {
            Some(live) if live.has_result() => live.result(),
            _ => Handle::default(),
        })
    }
    fn bound_result(
        &self,
        finder: &Handle<dyn TransferFinder>,
        result: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        let binder = Handle::new(TransferSimpleBinderOfTransient::new());
        iges_live(&binder, WHERE)?.set_result(result)?;
        iges_live(&self.process, WHERE)?.bind(finder, &binder.cast())
    }
    /// Whether a shape has a result; `tool.has_shape_result(&shape)?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn has_shape_result(&self, start: &Shape) -> Result<bool, Failure> {
        Ok(self
            .found_binder(&shape_finder(start))?
            .get()
            .is_some_and(|binder| binder.has_result()))
    }
    /// The result of a shape, null without one; `tool.get_shape_result(&shape)?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn get_shape_result(&self, start: &Shape) -> Result<Handle<StandardTransient>, Failure> {
        self.found_result(&shape_finder(start))
    }
    /// Records the result of a shape; `tool.set_shape_result(&shape, &entity)?`.
    ///
    /// # Errors
    /// A failure of the binding of the result.
    pub fn set_shape_result(
        &self,
        start: &Shape,
        result: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        self.bound_result(&shape_finder(start), result)
    }
    /// Whether an object has a result; `tool.has_transient_result(&object)?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn has_transient_result(&self, start: &Handle<StandardTransient>) -> Result<bool, Failure> {
        Ok(self
            .found_binder(&transient_finder(start))?
            .get()
            .is_some_and(|binder| binder.has_result()))
    }
    /// The result of an object, null without one; `tool.get_transient_result(&object)?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn get_transient_result(
        &self,
        start: &Handle<StandardTransient>,
    ) -> Result<Handle<StandardTransient>, Failure> {
        self.found_result(&transient_finder(start))
    }
    /// Records the result of an object; `tool.set_transient_result(&object, &entity)?`.
    ///
    /// # Errors
    /// A failure of the binding of the result.
    pub fn set_transient_result(
        &self,
        start: &Handle<StandardTransient>,
        result: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        self.bound_result(&transient_finder(start), result)
    }
}
