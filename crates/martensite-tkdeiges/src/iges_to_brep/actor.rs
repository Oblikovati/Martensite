//! `IgesToBRepActor`: the actor of a transfer of IGES entities to shapes.
use super::{IgesToBRep, IgesToBRepCurveAndSurface};
use crate::iges_data::{IgesDataIgesEntity, IgesDataIgesModel, iges_live};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_extend::ShapeExtendExplorer;
use martensite_tkshhealing::shape_fix::ShapeFixShapeTolerance;
use martensite_tktopalgo::brep_lib;
use martensite_tkxsbase::interface::{
    InterfaceInterfaceModel, InterfaceModelEntities, InterfaceModelReports, InterfaceStatic,
};
use martensite_tkxsbase::transfer::{
    TransferActor, TransferActorOfTransientProcess, TransferActorOfTransientProcessOps,
    TransferActorOps, TransferBinder, TransferProcessOps, TransferShapeProcessing,
    TransferTransientKind, TransferTransientProcess, transferring_by_transient_process,
};
use martensite_tkxsbase::transfer_brep::TransferBRepShapeBinder;
use martensite_tkxsbase::xs_algo::{DEShapeFixParameters, XSAlgoShapeProcessor};
use std::ops::Deref;

const WHERE: &str = "IgesToBRepActor";

#[derive(Default)]
struct ActorState {
    model: Handle<dyn InterfaceInterfaceModel>,
    continuity: i32,
    eps: f64,
}

/// Transfers entities to shapes, in the mode of the settings.
///
/// ```
/// use martensite_tkdeiges::iges_to_brep::IgesToBRepActor;
/// let actor = IgesToBRepActor::new();
/// assert_eq!(actor.get_continuity(), 0);
/// assert_eq!(actor.used_tolerance(), 0.0001);
/// ```
pub struct IgesToBRepActor {
    base: TransferActorOfTransientProcess,
    state: SharedState<ActorState>,
}
impl_transient!(
    IgesToBRepActor,
    "IgesToBRepActor",
    TransferActorOfTransientProcess,
    base,
    interfaces: [dyn TransferActorOps<TransferTransientKind>, dyn TransferActorOfTransientProcessOps]
);

impl Deref for IgesToBRepActor {
    type Target = TransferActorOfTransientProcess;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl Default for IgesToBRepActor {
    fn default() -> Self {
        Self::new()
    }
}

/// Whether an entity is one a transfer can give a shape for.
fn has_geometry(entity: &Handle<IgesDataIgesEntity>) -> bool {
    let Some(live) = entity.get() else {
        return false;
    };
    let (type_number, form) = (live.type_number(), live.form_number());
    IgesToBRep::is_curve_and_surface(entity)
        || (type_number == 402 && matches!(form, 1 | 7 | 14 | 15))
        || type_number == 408
        || type_number == 308
}

/// The regularity of the edges, under the setting; a failure is not one of the transfer.
pub(super) fn encode_regularity(shape: &Shape) {
    let angle = InterfaceStatic::r_val("read.encoderegularity.angle");
    if !shape.is_null() && angle > 0.0 {
        let _ = brep_lib::encode_regularity(shape, angle);
    }
}

pub(super) fn trim_tolerances(shape: &Shape, tolerance: f64) -> Result<(), Failure> {
    if InterfaceStatic::i_val("read.maxprecision.mode") == 1 {
        let limit = tolerance.max(InterfaceStatic::r_val("read.maxprecision.val"));
        let _ = ShapeFixShapeTolerance.limit_tolerance(shape, 0.0, limit, ShapeEnum::Shape)?;
    }
    Ok(())
}

impl IgesToBRepActor {
    /// An actor at continuity 0 and tolerance 1e-4; `IgesToBRepActor::new()`.
    pub fn new() -> Self {
        Self {
            base: TransferActorOfTransientProcess::default(),
            state: SharedState::new(ActorState {
                eps: 0.0001,
                ..ActorState::default()
            }),
        }
    }
    /// Sets the model and takes its resolution as tolerance; `actor.set_model(&model)?`.
    ///
    /// # Errors
    /// A null object failure when the model is not an IGES one.
    pub fn set_model(&self, model: &Handle<dyn InterfaceInterfaceModel>) -> Result<(), Failure> {
        let iges = model.cast::<IgesDataIgesModel>();
        let resolution = iges_live(&iges, WHERE)?.global_section().resolution();
        let mut state = self.state.write();
        state.model = model.clone();
        state.eps = resolution;
        Ok(())
    }
    /// Sets the continuity to try; `actor.set_continuity(1)`.
    pub fn set_continuity(&self, continuity: i32) {
        self.state.write().continuity = continuity;
    }
    /// The continuity to try; `actor.get_continuity()`.
    pub fn get_continuity(&self) -> i32 {
        self.state.read().continuity
    }
    /// The tolerance of the last transfer; `actor.used_tolerance()`.
    pub fn used_tolerance(&self) -> f64 {
        self.state.read().eps
    }
    fn prepared_settings(
        &self,
        model: &Handle<IgesDataIgesModel>,
        process: &Handle<TransferTransientProcess>,
    ) -> Result<IgesToBRepCurveAndSurface, Failure> {
        let mut settings = IgesToBRepCurveAndSurface::new();
        settings.set_model(model)?;
        settings.set_continuity(self.get_continuity());
        settings.set_transfer_process(process);
        let eps = if InterfaceStatic::i_val("read.precision.mode") == 0 {
            iges_live(model, WHERE)?.global_section().resolution()
        } else {
            InterfaceStatic::r_val("read.precision.val")
        };
        settings.set_mode_approx(InterfaceStatic::i_val("read.iges.bspline.approxd1.mode") > 0);
        settings.set_surface_curve(InterfaceStatic::i_val("read.surfacecurve.mode"));
        if eps > 1.0e-8 {
            settings.set_eps_geom(eps);
            self.state.write().eps = eps * settings.get_unit_factor();
        }
        Ok(settings)
    }
    fn transferred_shape(
        &self,
        entity: &Handle<IgesDataIgesEntity>,
        model: &Handle<IgesDataIgesModel>,
        process: &Handle<TransferTransientProcess>,
        range: &ProgressRange,
    ) -> Result<Shape, Failure> {
        let scope = ProgressScope::with_literal(range, "Transfer stage", 2.0, false);
        XSAlgoShapeProcessor::prepare_for_transfer();
        let mut settings = self.prepared_settings(model, process)?;
        let nb_mapped = iges_live(process, WHERE)?.nb_mapped();
        let step = scope.next(1.0);
        // A failure of one entity is no shape for it, on its check; the other entities still transfer.
        let shape = settings
            .transfer_geometry(entity, &step)
            .unwrap_or_else(|failure| {
                if let Some(live) = process.get() {
                    live.add_fail_text(&entity.cast(), &failure.to_string(), "");
                }
                Shape::default()
            });
        let mut parameters = self.shape_processing().shape_fix_parameters();
        XSAlgoShapeProcessor::set_parameter_real(
            "FixShape.Tolerance3d",
            self.used_tolerance(),
            true,
            &mut parameters,
        );
        XSAlgoShapeProcessor::set_parameter_real(
            "FixShape.MaxTolerance3d",
            settings.get_max_tol(),
            true,
            &mut parameters,
        );
        let mut processor =
            XSAlgoShapeProcessor::new(&parameters, &DEShapeFixParameters::default());
        let step = scope.next(1.0);
        let shape = processor.process_shape(&shape, self.shape_processing().flags().0, &step)?;
        processor.merge_transfer_info(process, nb_mapped)?;
        Ok(shape)
    }
}

impl TransferActorOps<TransferTransientKind> for IgesToBRepActor {
    fn actor(&self) -> &TransferActor<TransferTransientKind> {
        self.base.actor()
    }
    fn recognize(&self, start: &Handle<StandardTransient>) -> bool {
        has_geometry(&start.cast())
    }
    fn transferring(
        &self,
        start: &Handle<StandardTransient>,
        tp: &Handle<dyn TransferProcessOps<TransferTransientKind>>,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        transferring_by_transient_process(self, start, tp, progress)
    }
}

impl TransferActorOfTransientProcessOps for IgesToBRepActor {
    fn shape_processing(&self) -> &TransferShapeProcessing {
        self.base.shape_processing()
    }
    /// The binder of the shape of an entity, null without a shape.
    fn transfer(
        &self,
        start: &Handle<StandardTransient>,
        process: &Handle<TransferTransientProcess>,
        range: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        let model = self.state.read().model.cast::<IgesDataIgesModel>();
        let entity: Handle<IgesDataIgesEntity> = start.cast();
        let (Some(live_model), true) = (model.get(), entity.is_some()) else {
            return Ok(Handle::default());
        };
        let number = live_model.number(start);
        if InterfaceStatic::i_val("read.iges.faulty.entities") == 0
            && live_model.is_error_entity(number)
        {
            return Ok(Handle::default());
        }
        let mut shape = Shape::default();
        if has_geometry(&entity) {
            shape = self.transferred_shape(&entity, &model, process, range)?;
        }
        if ShapeExtendExplorer.shape_type(&shape, true)? != ShapeEnum::Shape && !shape.is_null() {
            encode_regularity(&shape);
            trim_tolerances(&shape, self.used_tolerance())?;
        }
        if shape.is_null() {
            return Ok(Handle::default());
        }
        Ok(Handle::new(TransferBRepShapeBinder::with_result(&shape)).cast())
    }
}
