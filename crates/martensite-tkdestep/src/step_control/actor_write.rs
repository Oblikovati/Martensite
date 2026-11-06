//! `StepControlActorWrite`: shapes to product and shape representation entities. This file holds
//! the actor's state, `recognize` and `transfer`; `transfer_shape`, `transfer_compound` and
//! `transfer_sub_shape` have files of their own.
use crate::de_step::StepControlStepModelType;
use crate::step_construct::{StepConstructContextTool, StepConstructPart};
use crate::step_data::{StepDataFactors, StepDataStepModel};
use crate::step_entity::step_live;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape, ShapeIterator};
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::HSequence;
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkg3d::geom::{GeomLine, GeomPlane};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkxsbase::transfer::{
    TransferActor, TransferActorOfFinderProcess, TransferActorOfFinderProcessOps, TransferActorOps,
    TransferBinder, TransferFinder, TransferFinderKind, TransferFinderProcess, TransferProcessOps,
    TransferShapeProcessing, transferring_by_finder_process,
};
use martensite_tkxsbase::transfer_brep::TransferBRepShapeMapper;
use martensite_tkxsbase::xs_algo::{XSAlgoShapeProcessor, xs_algo_units};

/// The fields of the actor.
pub(super) struct ActorWriteState {
    pub(super) mygroup: i32,
    pub(super) mytoler: f64,
    pub(super) my_context: StepConstructContextTool,
    /// The compounds to write as assemblies when an XDE document decides it, set by
    /// `STEPCAFControlActorWrite`, which overrides `is_assembly`. `None` is its standard mode.
    pub(crate) registered_assemblies: Option<martensite_tkbrep::toptools::MapOfShape>,
}

/// ```
/// use martensite_tkdestep::de_step::StepControlStepModelType;
/// use martensite_tkdestep::step_control::StepControlActorWrite;
/// let actor = StepControlActorWrite::new();
/// assert_eq!(actor.mode(), StepControlStepModelType::SHELL_BASED_SURFACE_MODEL);
/// ```
pub struct StepControlActorWrite {
    base: TransferActorOfFinderProcess,
    pub(super) state: SharedState<ActorWriteState>,
}
impl_transient!(StepControlActorWrite, "StepControlActorWrite", TransferActorOfFinderProcess, base, interfaces: [dyn TransferActorOps<TransferFinderKind>, dyn TransferActorOfFinderProcessOps]);

impl Default for StepControlActorWrite {
    fn default() -> Self {
        Self::new()
    }
}

/// Every face of the shape on a plane, every edge on a line.
fn is_planar_with_lines(shape: &Shape) -> Result<bool, Failure> {
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while faces.more() {
        let surface = BRepTool::surface(&topods::face(faces.current())?)?;
        if surface.cast::<GeomPlane>().is_null() {
            return Ok(false);
        }
        faces.next()?;
    }
    let mut edges = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while edges.more() {
        let (curve, _, _) = BRepTool::curve(&topods::edge(edges.current())?)?;
        if curve.cast::<GeomLine>().is_null() {
            return Ok(false);
        }
        edges.next()?;
    }
    Ok(true)
}

/// The members of the list of roots.
pub(super) fn roots_of(
    roots: &Handle<HSequence<Handle<StandardTransient>>>,
) -> Vec<Handle<StandardTransient>> {
    roots
        .get()
        .map(|live| live.sequence(|items| items.iter().cloned().collect()))
        .unwrap_or_default()
}

/// Whether the explorer of `to_find` avoiding `to_avoid` finds anything.
fn explores_to(shape: &Shape, to_find: ShapeEnum, to_avoid: ShapeEnum) -> Result<bool, Failure> {
    Ok(Explorer::new(shape, to_find, to_avoid)?.more())
}

impl StepControlActorWrite {
    /// `StepControlActorWrite()`: writing shell_based_surface_model by default, no group mode, no
    /// user tolerance.
    pub fn new() -> Self {
        let actor = Self {
            base: TransferActorOfFinderProcess::new(),
            state: SharedState::new(ActorWriteState {
                mygroup: 0,
                mytoler: -1.0,
                my_context: StepConstructContextTool::new(),
                registered_assemblies: None,
            }),
        };
        actor.set_mode(StepControlStepModelType::SHELL_BASED_SURFACE_MODEL);
        actor
    }
    /// `set_mode(M)`.
    pub fn set_mode(&self, mode: StepControlStepModelType) {
        let value = match mode {
            StepControlStepModelType::AS_IS | StepControlStepModelType::HYBRID => 0, // PAS IMPLEMENTE !!
            StepControlStepModelType::MANIFOLD_SOLID_BREP => 3,
            StepControlStepModelType::BREP_WITH_VOIDS => 5,
            StepControlStepModelType::FACETED_BREP => 1,
            StepControlStepModelType::FACETED_BREP_AND_BREP_WITH_VOIDS => 6,
            StepControlStepModelType::SHELL_BASED_SURFACE_MODEL => 2,
            StepControlStepModelType::GEOMETRIC_CURVE_SET => 4,
            _ => return,
        };
        *self.mode_trans() = value;
    }
    /// `Mode`.
    pub fn mode(&self) -> StepControlStepModelType {
        match *self.mode_trans() {
            1 => StepControlStepModelType::FACETED_BREP,
            2 => StepControlStepModelType::SHELL_BASED_SURFACE_MODEL,
            3 => StepControlStepModelType::MANIFOLD_SOLID_BREP,
            4 => StepControlStepModelType::GEOMETRIC_CURVE_SET,
            5 => StepControlStepModelType::BREP_WITH_VOIDS,
            6 => StepControlStepModelType::FACETED_BREP_AND_BREP_WITH_VOIDS,
            _ => StepControlStepModelType::AS_IS,
        }
    }
    /// `set_group_mode(mode)`: a negative mode is ignored.
    pub fn set_group_mode(&self, mode: i32) {
        if mode >= 0 {
            self.state.write().mygroup = mode;
        }
    }
    /// `group_mode`.
    pub fn group_mode(&self) -> i32 {
        self.state.read().mygroup
    }
    /// `set_tolerance(Tol)`.
    pub fn set_tolerance(&self, tolerance: f64) {
        self.state.write().mytoler = tolerance;
    }
    /// The `set_std_mode` of `STEPCAFControlActorWrite`: `None` is the standard mode.
    pub(crate) fn set_registered_assemblies(
        &self,
        map: Option<martensite_tkbrep::toptools::MapOfShape>,
    ) {
        self.state.write().registered_assemblies = map;
    }
    /// The `clear_map` of `STEPCAFControlActorWrite`: the mode stays.
    pub(crate) fn clear_registered_assemblies(&self) {
        if let Some(registered) = &mut self.state.write().registered_assemblies {
            *registered = martensite_tkbrep::toptools::MapOfShape::default();
        }
    }
    /// The `register_assembly` of `STEPCAFControlActorWrite`: nothing in standard mode.
    pub(crate) fn register_assembly_shape(&self, shape: &Shape) {
        if let Some(registered) = &mut self.state.write().registered_assemblies {
            registered.add(shape.clone());
        }
    }
    /// `is_assembly(model, S)`: `shape` narrows down a chain of single-child compounds.
    pub fn is_assembly(
        &self,
        model: &Handle<StepDataStepModel>,
        shape: &mut Shape,
    ) -> Result<bool, Failure> {
        if let Some(registered) = &self.state.read().registered_assemblies {
            return Ok(registered.contains(shape));
        }
        loop {
            if self.group_mode() == 0 || shape.shape_type()? != ShapeEnum::Compound {
                return Ok(false);
            }
            // bug 23950
            if step_live(model, "StepControlActorWrite")?
                .internal_parameters()
                .write_vertex_mode
                .as_raw()
                == 0
            {
                let mut children = ShapeIterator::new(shape, true, true);
                let mut only_vertices = true;
                while children.more() {
                    if children.value()?.shape_type()? != ShapeEnum::Vertex {
                        only_vertices = false;
                        break;
                    }
                    children.next();
                }
                if only_vertices {
                    return Ok(false);
                }
            }
            if self.group_mode() == 1 {
                return Ok(true);
            }
            let mut children = ShapeIterator::new(shape, true, true);
            if !children.more() {
                return Ok(false);
            }
            let first = children.value()?.clone();
            children.next();
            if children.more() {
                return Ok(true);
            }
            *shape = first;
        }
    }
    /// The model of the transfer, its units read, and the factors the shape is written with.
    fn prepare_model(&self, model: &Handle<StepDataStepModel>) -> Result<StepDataFactors, Failure> {
        let live = step_live(model, "StepControlActorWrite")?;
        {
            let mut state = self.state.write();
            state.my_context.set_model(model)?; // :abv 04.11.00: take APD from model
            state.my_context.add_apd(false)?; // update APD
            state.my_context.set_level(1); // set assembly level to 1 (to ensure)
        }
        if !live.is_initialized_unit() {
            XSAlgoShapeProcessor::prepare_for_transfer(); // update unit info
            live.set_local_length_unit(xs_algo_units().get_cascade_length_unit_default());
        }
        let length_factor = live.write_length_unit() / live.local_length_unit();
        let angle_mode = live.internal_parameters().angle_unit.as_raw();
        let angle = if angle_mode <= 1 {
            1.0
        } else {
            std::f64::consts::PI / 180.0
        };
        let mut factors = StepDataFactors::new();
        factors.initialize_factors(length_factor, angle, 1.0);
        Ok(factors)
    }
}

impl TransferActorOps<TransferFinderKind> for StepControlActorWrite {
    fn actor(&self) -> &TransferActor<TransferFinderKind> {
        self.base.actor()
    }
    fn recognize(&self, start: &Handle<dyn TransferFinder>) -> bool {
        let mapper: Handle<TransferBRepShapeMapper> = start.cast();
        let Some(live) = mapper.get() else {
            return false;
        };
        let mode = self.mode();
        if mode == StepControlStepModelType::AS_IS {
            return true;
        }
        recognize_shape(live.value(), mode).unwrap_or(false)
    }
    fn transferring(
        &self,
        start: &Handle<dyn TransferFinder>,
        tp: &Handle<dyn TransferProcessOps<TransferFinderKind>>,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        transferring_by_finder_process(self, start, tp, progress)
    }
}

/// `recognize` on a shape, in a mode other than as-is.
fn recognize_shape(shape: &Shape, mode: StepControlStepModelType) -> Result<bool, Failure> {
    let (mut solid, mut shell, mut face) = (false, false, false);
    match shape.shape_type()? {
        ShapeEnum::Compound => {
            solid = explores_to(shape, ShapeEnum::Solid, ShapeEnum::Shape)?;
            shell = explores_to(shape, ShapeEnum::Shell, ShapeEnum::Solid)?;
            face = explores_to(shape, ShapeEnum::Face, ShapeEnum::Shell)?;
        }
        ShapeEnum::Solid => solid = true,
        ShapeEnum::Shell => shell = true,
        ShapeEnum::Face => face = true,
        _ if mode != StepControlStepModelType::GEOMETRIC_CURVE_SET => return Ok(false),
        _ => {}
    }
    if matches!(
        mode,
        StepControlStepModelType::FACETED_BREP
            | StepControlStepModelType::FACETED_BREP_AND_BREP_WITH_VOIDS
    ) && !is_planar_with_lines(shape)?
    {
        return Ok(false);
    }
    Ok(match mode {
        StepControlStepModelType::MANIFOLD_SOLID_BREP => solid || shell,
        StepControlStepModelType::BREP_WITH_VOIDS
        | StepControlStepModelType::FACETED_BREP
        | StepControlStepModelType::FACETED_BREP_AND_BREP_WITH_VOIDS => solid,
        StepControlStepModelType::SHELL_BASED_SURFACE_MODEL => solid || shell || face,
        StepControlStepModelType::GEOMETRIC_CURVE_SET => true, // tout OK
        _ => false,
    })
}

impl TransferActorOfFinderProcessOps for StepControlActorWrite {
    fn shape_processing(&self) -> &TransferShapeProcessing {
        self.base.shape_processing()
    }
    fn mode_trans_state(&self) -> &SharedState<i32> {
        self.base.mode_trans_state()
    }
    fn transfer(
        &self,
        start: &Handle<dyn TransferFinder>,
        fp: &Handle<TransferFinderProcess>,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        if start.cast::<TransferBRepShapeMapper>().is_null() {
            return Ok(self.null_result());
        }
        let live_fp = step_live(fp, "StepControlActorWrite::Transfer")?;
        let model: Handle<StepDataStepModel> = live_fp.model().cast();
        let factors = self.prepare_model(&model)?;
        let mut part = StepConstructPart::new();
        self.make_part_sdr(&mut part, &model)?;
        let sdr = part.sdr_value();
        let result = self.transfer_shape(
            start,
            &sdr,
            fp,
            &factors,
            &Handle::default(),
            true,
            progress,
        )?;
        let (roots, apd) = {
            let mut state = self.state.write();
            let roots = state.my_context.get_roots_for_part(&part)?;
            (roots, state.my_context.get_apd())
        };
        let product: Handle<dyn TransferBinder> = self.transient_result(&apd.cast()).cast();
        for root in roots_of(&roots) {
            product.add_result(&self.transient_result(&root).cast());
        }
        product.add_result(&result);
        self.state.write().my_context.next_index();
        Ok(product)
    }
}
