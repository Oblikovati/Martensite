//! `IgesControlActorWrite`: the actor of a transfer of shapes and geometry to IGES entities.
use crate::brep_to_iges::BRepToIgesBREntity;
use crate::brep_to_iges_brep::BRepToIgesBRepEntity;
use crate::geom_to_iges::{GeomToIgesGeomCurve, GeomToIgesGeomSurface};
use crate::iges_data::{IgesDataIgesEntity, IgesDataIgesModel, iges_live};
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkxsbase::transfer::{
    TransferActor, TransferActorOfFinderProcess, TransferActorOfFinderProcessOps, TransferActorOps,
    TransferBinder, TransferFinder, TransferFinderKind, TransferFinderProcess, TransferProcessOps,
    TransferShapeProcessing, TransferTransientMapper, transferring_by_finder_process,
};
use martensite_tkxsbase::transfer_brep::TransferBRepShapeMapper;
use martensite_tkxsbase::xs_algo::{DEShapeFixParameters, XSAlgoShapeProcessor};

const WHERE: &str = "IgesControlActorWrite";

/// Transfers shapes, curves and surfaces in the mode of the transfer.
///
/// ```
/// use martensite_tkdeiges::iges_control::IgesControlActorWrite;
/// use martensite_tkxsbase::transfer::TransferActorOfFinderProcessOps;
/// let actor = IgesControlActorWrite::default();
/// assert_eq!(*actor.mode_trans(), 0);
/// ```
#[derive(Default)]
pub struct IgesControlActorWrite {
    base: TransferActorOfFinderProcess,
}
impl_transient!(
    IgesControlActorWrite,
    "IgesControlActorWrite",
    TransferActorOfFinderProcess,
    base,
    interfaces: [dyn TransferActorOps<TransferFinderKind>, dyn TransferActorOfFinderProcessOps]
);

impl IgesControlActorWrite {
    fn shape_entity(
        &self,
        mapper: &TransferBRepShapeMapper,
        model: &Handle<IgesDataIgesModel>,
        process: &Handle<TransferFinderProcess>,
        range: &ProgressRange<'_>,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let shape = mapper.value();
        if shape.is_null() {
            return Ok(Handle::default());
        }
        let processing = self.shape_processing();
        let mut processor = XSAlgoShapeProcessor::new(
            &processing.shape_fix_parameters(),
            &DEShapeFixParameters::default(),
        );
        let processed = processor.process_shape(shape, processing.flags().0, range)?;
        let entity = if *self.mode_trans() == 0 {
            let mut tool = BRepToIgesBREntity::new();
            tool.set_model(model)?;
            tool.set_transfer_process(process);
            tool.transfer_shape(&processed, range)?
        } else {
            let mut tool = BRepToIgesBRepEntity::new();
            tool.set_model(model)?;
            tool.set_transfer_process(process);
            tool.transfer_shape(&processed, range)?
        };
        processor.merge_transfer_info_finder(process)?;
        Ok(entity)
    }
    fn geometry_entity(
        mapper: &TransferTransientMapper,
        model: &Handle<IgesDataIgesModel>,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let value = mapper.value();
        let curve = value.cast::<dyn GeomCurve>();
        if let Some(live) = curve.get() {
            let mut tool = GeomToIgesGeomCurve::new();
            tool.set_model(model)?;
            return tool.transfer_curve(&curve, live.first_parameter(), live.last_parameter());
        }
        let surface = value.cast::<dyn GeomSurface>();
        if let Some(live) = surface.get() {
            let mut tool = GeomToIgesGeomSurface::new();
            tool.set_model(model)?;
            return tool.transfer_surface(&surface, live.bounds());
        }
        Ok(Handle::default())
    }
}

impl TransferActorOps<TransferFinderKind> for IgesControlActorWrite {
    fn actor(&self) -> &TransferActor<TransferFinderKind> {
        self.base.actor()
    }
    fn recognize(&self, start: &Handle<dyn TransferFinder>) -> bool {
        if start.cast::<TransferBRepShapeMapper>().is_some() {
            return true;
        }
        let mapper = start.cast::<TransferTransientMapper>();
        let Some(live) = mapper.get() else {
            return false;
        };
        let value = live.value();
        value.cast::<dyn GeomCurve>().is_some() || value.cast::<dyn GeomSurface>().is_some()
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

impl TransferActorOfFinderProcessOps for IgesControlActorWrite {
    fn shape_processing(&self) -> &TransferShapeProcessing {
        self.base.shape_processing()
    }
    fn mode_trans_state(&self) -> &martensite_tkernel::standard::SharedState<i32> {
        self.base.mode_trans_state()
    }
    /// The binder of the entity of a shape or of a geometry, null without one.
    fn transfer(
        &self,
        start: &Handle<dyn TransferFinder>,
        process: &Handle<TransferFinderProcess>,
        range: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        XSAlgoShapeProcessor::prepare_for_transfer();
        let model = iges_live(process, WHERE)?
            .model()
            .cast::<IgesDataIgesModel>();
        if model.is_null() || !(0..=1).contains(&*self.mode_trans()) {
            return Ok(self.null_result());
        }
        let shape_mapper = start.cast::<TransferBRepShapeMapper>();
        if let Some(live) = shape_mapper.get() {
            let entity = self.shape_entity(live, &model, process, range)?;
            if entity.is_some() {
                return Ok(self.transient_result(&entity.cast()).cast());
            }
        }
        let geometry_mapper = start.cast::<TransferTransientMapper>();
        if let Some(live) = geometry_mapper.get() {
            let entity = Self::geometry_entity(live, &model)?;
            if entity.is_some() {
                return Ok(self.transient_result(&entity.cast()).cast());
            }
        }
        Ok(self.null_result())
    }
}
