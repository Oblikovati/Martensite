//! `IgesToBRepReader`: reads an IGES file and transfers it to shapes, without a session.
use super::{IgesToBRep, IgesToBRepActor, IgesToBRepCurveAndSurface, one_transfer_at_a_time};
use crate::de_iges::xs_shape_fix_parameters;
use crate::iges_appli::IgesAppli;
use crate::iges_data::{IgesDataIgesEntity, IgesDataIgesModel, iges_live, iges_msg};
use crate::iges_file::iges_file_read;
use crate::iges_select::IgesSelectWorkLibrary;
use crate::iges_solid::IgesSolid;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape};
use martensite_tkernel::message::{Gravity, Msg, ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_extend::ShapeExtendExplorer;
use martensite_tkshhealing::shape_process::{ShapeProcessOperation, ShapeProcessOperationsFlags};
use martensite_tkxsbase::interface::{
    InterfaceCheckIterator, InterfaceCheckTool, InterfaceInterfaceModel, InterfaceModelEntities,
    InterfaceProtocol, InterfaceShareFlags, InterfaceStatic,
};
use martensite_tkxsbase::transfer::{TransferActorOps, TransferOutput, TransferTransientProcess};
use martensite_tkxsbase::transfer_brep::TransferBRep;
use martensite_tkxsbase::xs_algo::{
    DEShapeFixParameters, XSAlgoParameterMap, XSAlgoShapeProcessor,
};
use std::time::Instant;

const WHERE: &str = "IgesToBRepReader";

/// The time since a start, as the messages show it.
fn elapsed_text(start: Instant) -> Vec<u8> {
    let total = start.elapsed().as_secs_f64();
    let hours = (total / 3600.0).floor();
    let minutes = ((total - hours * 3600.0) / 60.0).floor();
    let seconds = total - hours * 3600.0 - minutes * 60.0;
    let text = if hours > 0.0 {
        format!("{hours}h:{minutes}m:{seconds:.2}s")
    } else if minutes > 0.0 {
        format!("{minutes}m:{seconds:.2}s")
    } else {
        format!("{seconds:.2}s")
    };
    text.into_bytes()
}

/// The message of a key with its argument put in by `fill`.
fn with_arg(
    key: &str,
    fill: impl FnOnce(&mut Msg) -> Result<&mut Msg, Failure>,
) -> Result<Msg, Failure> {
    let mut message = iges_msg(key);
    let _ = fill(&mut message)?;
    Ok(message)
}

/// The key of the message on why a file does not open.
fn open_failure_key(code: Option<i32>) -> &'static str {
    match code {
        Some(2) => "XSTEP_3",
        Some(12) => "XSTEP_4",
        Some(13) => "XSTEP_5",
        Some(24) => "XSTEP_6",
        _ => "XSTEP_7",
    }
}

/// A protocol as the tools of the interface take it.
fn interface_protocol() -> Handle<dyn InterfaceProtocol> {
    IgesSelectWorkLibrary::define_protocol().cast()
}

/// The numbers of the failures and warnings of a list of checks.
fn count_checks(list: &InterfaceCheckIterator) -> Result<(i32, i32), Failure> {
    let (mut fails, mut warnings) = (0, 0);
    list.start();
    while list.more() {
        let check = list.value()?;
        let live = iges_live(&check, WHERE)?;
        fails += live.nb_fails();
        warnings += live.nb_warnings();
        list.next();
    }
    Ok((fails, warnings))
}

/// Reads a file into a model and its roots or one entity into shapes.
///
/// ```no_run
/// use martensite_tkdeiges::iges_to_brep::IgesToBRepReader;
/// use martensite_tkernel::message::ProgressRange;
/// let mut reader = IgesToBRepReader::new().unwrap();
/// assert_eq!(reader.load_file("part.igs").unwrap(), 0);
/// reader.transfer_roots(false, &ProgressRange::default()).unwrap();
/// assert!(reader.is_done());
/// ```
pub struct IgesToBRepReader {
    model: Handle<IgesDataIgesModel>,
    done: bool,
    shapes: Vec<Shape>,
    actor: Handle<IgesToBRepActor>,
    process: Handle<TransferTransientProcess>,
    parameters: XSAlgoParameterMap,
    flags: ShapeProcessOperationsFlags,
    flags_are_set: bool,
}

impl IgesToBRepReader {
    /// A reader with a new actor and process; `IgesToBRepReader::new()?`.
    ///
    /// # Errors
    /// A failure of the set-up of the entity packages.
    pub fn new() -> Result<Self, Failure> {
        IgesToBRep::init()?;
        IgesAppli::try_init()?;
        IgesSolid::try_init()?;
        Ok(Self {
            model: Handle::default(),
            done: false,
            shapes: Vec::new(),
            actor: Handle::new(IgesToBRepActor::new()),
            process: TransferTransientProcess::new(10000),
            parameters: XSAlgoParameterMap::default(),
            flags: ShapeProcessOperationsFlags::default(),
            flags_are_set: false,
        })
    }
    fn send(&self, message: &Msg) -> Result<(), Failure> {
        iges_live(&self.process, WHERE)?
            .messenger()
            .send_extended(message.value(), Gravity::Info);
        Ok(())
    }
    /// Reads a file to the model; gives the status of the reading, 0 when done;
    /// `reader.load_file("part.igs")?`.
    ///
    /// # Errors
    /// A failure of the reading or of the checks of the model.
    pub fn load_file(&mut self, file_name: &str) -> Result<i32, Failure> {
        if self.process.is_null() {
            self.process = TransferTransientProcess::new(10000);
        }
        let trace = iges_live(&self.process, WHERE)?.trace_level();
        self.send(&with_arg("IGES_2000", |m| {
            m.arg_text(file_name.as_bytes())
        })?)?;
        self.send(&with_arg("IGES_2005", |m| m.arg_integer(trace))?)?;
        let start = Instant::now();
        let model = IgesDataIgesModel::new();
        let status = iges_file_read(file_name, &model, &IgesSelectWorkLibrary::define_protocol())?;
        if status != 0 {
            let code = std::io::Error::last_os_error().raw_os_error();
            self.send(&iges_msg("XSTEP_2"))?;
            self.send(&iges_msg(open_failure_key(code)))?;
        }
        let generic: Handle<dyn InterfaceInterfaceModel> = model.cast();
        let mut tool = InterfaceCheckTool::with_protocol(&generic, &interface_protocol())?;
        let (fails, warnings) = count_checks(&tool.complete_check_list()?)?;
        self.send(&with_arg("XSTEP_25", |m| m.arg_integer(fails))?)?;
        self.send(&with_arg("XSTEP_26", |m| m.arg_integer(warnings))?)?;
        self.send(&with_arg("XSTEP_8", |m| m.arg_text(&elapsed_text(start)))?)?;
        self.set_model(&model)?;
        Ok(status)
    }
    /// Sets the model and clears the results; `reader.set_model(&model)?`.
    ///
    /// # Errors
    /// A failure for a null model when a new process is needed.
    pub fn set_model(&mut self, model: &Handle<IgesDataIgesModel>) -> Result<(), Failure> {
        self.model = model.clone();
        self.done = false;
        self.shapes.clear();
        if self.process.is_null() {
            let size = iges_live(model, WHERE)?.nb_entities();
            self.process = TransferTransientProcess::new(size);
        } else {
            iges_live(&self.process, WHERE)?.clear();
        }
        Ok(())
    }
    /// The model read; `reader.model()`.
    pub fn model(&self) -> Handle<IgesDataIgesModel> {
        self.model.clone()
    }
    /// Sets the process of the transfers; `reader.set_transient_process(&process)`.
    pub fn set_transient_process(&mut self, process: &Handle<TransferTransientProcess>) {
        self.process = process.clone();
    }
    /// The process of the transfers; `reader.transient_process()`.
    pub fn transient_process(&self) -> Handle<TransferTransientProcess> {
        self.process.clone()
    }
    /// The actor of the transfers of roots; `reader.actor()`.
    pub fn actor(&self) -> Handle<IgesToBRepActor> {
        self.actor.clone()
    }
    /// Forgets the results; `reader.clear()`.
    pub fn clear(&mut self) {
        self.done = false;
        self.shapes.clear();
    }
    /// Whether the model is free of failures, optionally printing its checks to the messages;
    /// `reader.check(false)?`.
    ///
    /// # Errors
    /// A failure of the checks of the model.
    pub fn check(&self, with_print: bool) -> Result<bool, Failure> {
        let generic: Handle<dyn InterfaceInterfaceModel> = self.model.cast();
        let mut tool = InterfaceCheckTool::with_protocol(&generic, &interface_protocol())?;
        let list = tool.complete_check_list()?;
        if let (true, Some(process)) = (with_print, self.process.get()) {
            let mut printed = Vec::new();
            if tool.print_list(&list, &mut printed).is_ok() {
                process.messenger().send_c_str(&printed, Gravity::Info);
            }
        }
        Ok(list.is_empty(true))
    }
    /// Whether the last transfer gave something; `reader.is_done()`.
    pub fn is_done(&self) -> bool {
        self.done
    }
    /// Tells the settings of a transfer; gives the setting of the precision.
    fn send_settings(&self) -> Result<i32, Failure> {
        let mode = InterfaceStatic::i_val("read.precision.mode");
        self.send(&with_arg("IGES_2035", |m| m.arg_integer(mode))?)?;
        Ok(mode)
    }
    fn send_curve_settings(&self, continuity: i32) -> Result<(), Failure> {
        let surface_curve = InterfaceStatic::i_val("read.surfacecurve.mode");
        self.send(&with_arg("IGES_2045", |m| m.arg_integer(continuity))?)?;
        self.send(&with_arg("IGES_2050", |m| m.arg_integer(surface_curve))?)
    }
    /// Transfers one root; a failure is told and leaves it without a shape.
    fn transfer_root(
        &mut self,
        output: &TransferOutput,
        entity: &Handle<IgesDataIgesEntity>,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.done = true;
        let result = output
            .transfer(&entity.cast(), range)
            .and_then(|()| TransferBRep::shape_result_of(&self.process, &entity.cast()));
        let shape = match result {
            Ok(shape) => shape,
            Err(_) => return self.send(&iges_msg("IGES_1005")),
        };
        if shape.is_null() {
            return self.send(&iges_msg("IGES_2076"));
        }
        if ShapeExtendExplorer.shape_type(&shape, true)? != ShapeEnum::Shape {
            let tolerance = iges_live(&self.actor, WHERE)?.used_tolerance();
            super::actor::encode_regularity(&shape);
            super::actor::trim_tolerances(&shape, tolerance)?;
            self.shapes.push(shape);
        }
        Ok(())
    }
    /// Transfers the entities no other one refers to, the visible ones only when asked;
    /// `reader.transfer_roots(false, &range)?`.
    ///
    /// # Errors
    /// A failure of the settings or of the structure of the model.
    pub fn transfer_roots(
        &mut self,
        only_visible: bool,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let _only_one = one_transfer_at_a_time();
        let (model_handle, process_handle) = (self.model.clone(), self.process.clone());
        let (Some(model), Some(process)) = (model_handle.get(), process_handle.get()) else {
            return Ok(());
        };
        self.send(&iges_msg("IGES_2030"))?;
        let start = Instant::now();
        self.done = false;
        self.shapes.clear();
        let trace = process.trace_level();
        process.set_error_handle(true);
        process.set_root_management(true);
        let generic: Handle<dyn InterfaceInterfaceModel> = self.model.cast();
        let actor_handle = self.actor.clone();
        let actor = iges_live(&actor_handle, WHERE)?;
        actor.set_model(&generic)?;
        let continuity = InterfaceStatic::i_val("read.iges.bspline.continuity");
        actor.set_continuity(continuity);
        process.set_model(&generic);
        process.set_actor(&self.actor.cast());
        let output = TransferOutput::with_process(&self.process, &generic);
        let shares = InterfaceShareFlags::with_protocol(&generic, &interface_protocol())?;
        let number = model.nb_entities();
        if self.send_settings()? == 1 {
            let value = InterfaceStatic::r_val("read.precision.val");
            self.send(&with_arg("IGES_2040", |m| m.arg_real(value))?)?;
        }
        self.send_curve_settings(continuity)?;
        InterfaceStatic::set_i_val("read.iges.onlyvisible", i32::from(only_visible))?;
        let scope = ProgressScope::with_literal(range, "Root", f64::from(number), false);
        for rank in 1..=number {
            if !scope.more() {
                break;
            }
            let part = scope.next(1.0);
            let entity = model.entity(rank);
            if shares.is_shared(&entity.cast())? || !actor.recognize(&entity.cast()) {
                continue;
            }
            if trace > 0 {
                let kind = iges_live(&entity, WHERE)?.type_number();
                let mut message = iges_msg("IGES_2070");
                let _ = message.arg_integer(2 * rank - 1)?.arg_integer(kind)?;
                self.send(&message)?;
            }
            if !only_visible || iges_live(&entity, WHERE)?.blank_status() == 0 {
                self.transfer_root(&output, &entity, &part)?;
            }
        }
        self.send(&with_arg("IGES_2065", |m| {
            m.arg_text(&elapsed_text(start))
        })?)
    }
    /// The geometry tool of one entity on the settings; gives it with the precision used.
    fn entity_tool(&self) -> Result<(IgesToBRepCurveAndSurface, f64), Failure> {
        let mut tool = IgesToBRepCurveAndSurface::new();
        tool.set_model(&self.model)?;
        let eps = if self.send_settings()? == 0 {
            iges_live(&self.model, WHERE)?.global_section().resolution()
        } else {
            let value = InterfaceStatic::r_val("read.precision.val");
            self.send(&with_arg("IGES_2040", |m| m.arg_real(value))?)?;
            value
        };
        tool.set_mode_approx(InterfaceStatic::i_val("read.iges.bspline.approxd1.mode") > 0);
        let continuity = InterfaceStatic::i_val("read.iges.bspline.continuity");
        tool.set_continuity(continuity);
        tool.set_surface_curve(InterfaceStatic::i_val("read.surfacecurve.mode"));
        self.send_curve_settings(continuity)?;
        if eps > 1.0e-8 {
            tool.set_eps_geom(eps);
        }
        tool.set_transfer_process(&self.process);
        Ok((tool, eps))
    }
    /// Heals the geometry of an entity and keeps the result.
    fn keep_healed(
        &mut self,
        entity: &Handle<IgesDataIgesEntity>,
        shape: &Shape,
        (first_item, resolution, max_tolerance): (i32, f64, f64),
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.initialize_missing_parameters();
        XSAlgoShapeProcessor::set_parameter_real(
            "FixShape.Tolerance3d",
            resolution,
            true,
            &mut self.parameters,
        );
        XSAlgoShapeProcessor::set_parameter_real(
            "FixShape.MaxTolerance3d",
            max_tolerance,
            true,
            &mut self.parameters,
        );
        let mut healing =
            XSAlgoShapeProcessor::new(&self.parameters, &DEShapeFixParameters::default());
        let healed = healing.process_shape(shape, self.flags, range)?;
        healing.merge_transfer_info(&self.process, first_item)?;
        if ShapeExtendExplorer.shape_type(&healed, true)? == ShapeEnum::Shape {
            return Ok(());
        }
        TransferBRep::set_shape_result(&self.process, &entity.cast(), &healed)?;
        iges_live(&self.process, WHERE)?.set_root(&entity.cast());
        if !healed.is_null() {
            self.done = true;
            super::actor::encode_regularity(&healed);
            super::actor::trim_tolerances(&healed, max_tolerance)?;
            self.shapes.push(healed);
        }
        Ok(())
    }
    /// Transfers one entity; gives false without a model, out of range or on an interruption;
    /// `reader.transfer(1, &range)?`.
    ///
    /// # Errors
    /// A failure of the settings or of the healing.
    pub fn transfer(&mut self, number: i32, range: &ProgressRange<'_>) -> Result<bool, Failure> {
        let _only_one = one_transfer_at_a_time();
        self.done = false;
        let model_handle = self.model.clone();
        let Some(model) = model_handle.get() else {
            self.send(&iges_msg("IGES_2031"))?;
            return Ok(false);
        };
        if number <= 0 || number > model.nb_entities() {
            self.send(&with_arg("IGES_2032", |m| m.arg_integer(number))?)?;
            return Ok(false);
        }
        self.send(&iges_msg("IGES_2030"))?;
        let start = Instant::now();
        let entity = model.entity(number);
        let scope = ProgressScope::with_literal(range, "OneEnt", 2.0, false);
        XSAlgoShapeProcessor::prepare_for_transfer();
        let (mut tool, eps) = self.entity_tool()?;
        let first_item = iges_live(&self.process, WHERE)?.nb_mapped();
        match tool.transfer_geometry(&entity, &scope.next(1.0)) {
            Ok(shape) => {
                if scope.user_break() {
                    return Ok(false);
                }
                let limits = (first_item, eps * tool.get_unit_factor(), tool.get_max_tol());
                self.keep_healed(&entity, &shape, limits, &scope.next(1.0))?;
                if scope.user_break() {
                    return Ok(false);
                }
            }
            Err(_) => self.send(&iges_msg("IGES_1015"))?,
        }
        self.send(&with_arg("IGES_2065", |m| {
            m.arg_text(&elapsed_text(start))
        })?)?;
        Ok(true)
    }
    /// The tolerance of the last transfer of roots; `reader.used_tolerance()`.
    pub fn used_tolerance(&self) -> f64 {
        self.actor
            .get()
            .map_or(0.0, IgesToBRepActor::used_tolerance)
    }
    /// The number of shapes; `reader.nb_shapes()`.
    pub fn nb_shapes(&self) -> i32 {
        self.shapes.len() as i32
    }
    /// A shape by its rank from 1, a null shape out of range; `reader.shape(1)`.
    pub fn shape(&self, number: i32) -> Shape {
        usize::try_from(number - 1)
            .ok()
            .and_then(|rank| self.shapes.get(rank))
            .cloned()
            .unwrap_or_default()
    }
    /// All the shapes as one: none, the one, or a compound; `reader.one_shape()?`.
    ///
    /// # Errors
    /// A failure of the builder.
    pub fn one_shape(&self) -> Result<Shape, Failure> {
        match self.shapes.as_slice() {
            [] => Ok(Shape::default()),
            [one] => Ok(one.clone()),
            many => {
                let builder = BRepBuilder::default();
                let mut compound = Compound::default();
                builder.make_compound(&mut compound);
                for shape in many {
                    builder.add(&mut compound, shape)?;
                }
                Ok(compound.into_shape())
            }
        }
    }
    /// Sets the healing parameters; `reader.set_shape_fix_parameters(parameters)`.
    pub fn set_shape_fix_parameters(&mut self, parameters: XSAlgoParameterMap) {
        self.parameters = parameters;
    }
    /// Sets the healing parameters from a set of modes and additional ones;
    /// `reader.set_shape_fix_parameters_from(&modes, &additional)`.
    pub fn set_shape_fix_parameters_from(
        &mut self,
        parameters: &DEShapeFixParameters,
        additional: &XSAlgoParameterMap,
    ) {
        XSAlgoShapeProcessor::set_shape_fix_parameters(
            parameters,
            additional,
            &mut self.parameters,
        );
    }
    /// The healing parameters; `reader.shape_fix_parameters()`.
    pub fn shape_fix_parameters(&self) -> &XSAlgoParameterMap {
        &self.parameters
    }
    /// Sets the healing operations; `reader.set_shape_process_flags(flags)`.
    pub fn set_shape_process_flags(&mut self, flags: ShapeProcessOperationsFlags) {
        self.flags = flags;
        self.flags_are_set = true;
    }
    /// The healing operations; `reader.shape_process_flags()`.
    pub fn shape_process_flags(&self) -> ShapeProcessOperationsFlags {
        self.flags
    }
    fn initialize_missing_parameters(&mut self) {
        if self.parameters.is_empty() {
            self.set_shape_fix_parameters_from(&xs_shape_fix_parameters(), &Default::default());
        }
        if !self.flags_are_set {
            self.flags.set(ShapeProcessOperation::FixShape);
            self.flags_are_set = true;
        }
    }
}
