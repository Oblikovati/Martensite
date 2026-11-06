//! `StepControlActorWrite::transfer_shape` and what it builds on.
use super::StepControlActorWrite;
use super::actor_write_representation::ShapesToWrite;
use crate::de_step::StepControlStepModelType;
use crate::step_construct::StepConstructPart;
use crate::step_data::{StepDataFactors, StepDataStepModel};
use crate::step_entity::step_live;
use crate::step_geom::StepGeomAxis2Placement3d;
use crate::step_shape::{
    StepShapeNonManifoldSurfaceShapeRepresentation, StepShapeShapeDefinitionRepresentation,
    StepShapeShapeRepresentation,
};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Compound, Shape, ShapeIterator, Shell};
use martensite_tkbrep::toptools::IndexedDataMapOfShapeListOfShape;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_process::ShapeProcessShapeContext;
use martensite_tkxsbase::transfer::{
    TransferActorOps, TransferBinder, TransferFinder, TransferFinderProcess,
};
use martensite_tkxsbase::transfer_brep::{TransferBRep, TransferBRepShapeMapper};

/// `is_manifold_shape`: no edge of the direct non-compound children has more than two faces.
pub(super) fn is_manifold_shape(shape: &Shape) -> Result<bool, Failure> {
    let builder = BRepBuilder::default();
    let mut direct = Compound::default();
    builder.make_compound(&mut direct);
    let mut children = ShapeIterator::new(shape, true, true);
    while children.more() {
        let child = children.value()?;
        if child.shape_type()? != ShapeEnum::Compound {
            builder.add(&mut direct, child)?;
        }
        children.next();
    }
    let mut ancestors = IndexedDataMapOfShapeListOfShape::default();
    topexp::map_shapes_and_ancestors(&direct, ShapeEnum::Edge, ShapeEnum::Face, &mut ancestors)?;
    for rank in 1..=ancestors.extent() {
        let edge = topods::edge(ancestors.find_key(rank)?)?;
        if !BRepTool::degenerated(&edge)? && ancestors.find_from_index(rank)?.len() > 2 {
            return Ok(false);
        }
    }
    Ok(true)
}

/// `separateShapeToSoloVertex`.
fn separate_solo_vertices(shape: &Shape, vertices: &mut Vec<Shape>) -> Result<bool, Failure> {
    if shape.is_null() {
        return Ok(false);
    }
    match shape.shape_type()? {
        ShapeEnum::Compound => {
            let mut children = ShapeIterator::new(shape, true, true);
            while children.more() {
                if !separate_solo_vertices(children.value()?, vertices)? {
                    return Ok(false);
                }
                children.next();
            }
            Ok(true)
        }
        ShapeEnum::Vertex => {
            vertices.push(shape.clone());
            Ok(true)
        }
        _ => {
            vertices.clear();
            Ok(false)
        }
    }
}

const WHERE: &str = "StepControlActorWrite";

impl StepControlActorWrite {
    /// `getNMSSRForGroup(shapeGroup, FP, isNMSSRCreated)`.
    pub(super) fn nmssr_for_group(
        &self,
        group: &Handle<martensite_tkernel::ncollection::HSequence<Shape>>,
        fp: &Handle<TransferFinderProcess>,
    ) -> Result<(Handle<StepShapeShapeRepresentation>, bool), Failure> {
        let process = step_live(fp, WHERE)?;
        let shapes = group
            .get()
            .map(|live| live.sequence(|items| items.iter().cloned().collect::<Vec<_>>()))
            .unwrap_or_default();
        let atype = StepShapeNonManifoldSurfaceShapeRepresentation::static_type();
        for shape in &shapes {
            let mapper = TransferBRep::shape_mapper(fp, shape)?;
            let mut found = Handle::<StepShapeNonManifoldSurfaceShapeRepresentation>::default();
            if process.find_typed_transient_as(&mapper.cast(), atype, &mut found) {
                return Ok((found.cast(), false));
            }
        }
        Ok((
            Handle::new(StepShapeNonManifoldSurfaceShapeRepresentation::new()).cast(),
            true,
        ))
    }
    /// `mergeInfoForNM(theFP, theInfo)`.
    pub(super) fn merge_info_for_nm(
        &self,
        fp: &Handle<TransferFinderProcess>,
        context: &Handle<ShapeProcessShapeContext>,
    ) -> Result<(), Failure> {
        let Some(live) = context.get() else {
            return Ok(());
        };
        let process = step_live(fp, WHERE)?;
        for (original, result) in live.map().items() {
            if original.shape_type()? != ShapeEnum::Face {
                continue;
            }
            let mapper = TransferBRep::shape_mapper(fp, original)?;
            let binder = process.find(&mapper.cast());
            if binder.is_null() {
                continue;
            }
            let result_mapper = TransferBRep::shape_mapper(fp, result)?;
            process.bind(&result_mapper.cast(), &binder)?;
        }
        Ok(())
    }
    /// `make_sdr(nullptr, GetProductName, GetAPD->Application, model)`.
    pub(super) fn make_part_sdr(
        &self,
        part: &mut StepConstructPart,
        model: &Handle<StepDataStepModel>,
    ) -> Result<(), Failure> {
        let (name, apd) = {
            let state = self.state.read();
            (
                state.my_context.get_product_name(),
                state.my_context.get_apd(),
            )
        };
        let application = step_live(&apd, WHERE)?.application();
        part.make_sdr(&Handle::default(), &name, &application, model)
    }
    /// The default axis of the context.
    pub(super) fn default_axis(&self) -> Result<Handle<StepGeomAxis2Placement3d>, Failure> {
        self.state.write().my_context.get_default_axis()
    }
    /// The non-manifold part of a compound written as one: its items transferred on `sdr`, and
    /// the binder to chain. `ok(Some(binder))` with `return_now` when nothing else is written.
    fn transfer_non_manifold_part(
        &self,
        shape: &mut Shape,
        sdr0: &Handle<StepShapeShapeDefinitionRepresentation>,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        is_manifold: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<(Handle<dyn TransferBinder>, bool), Failure> {
        let builder = BRepBuilder::default();
        let mut non_manifold = Compound::default();
        let mut manifold = Compound::default();
        builder.make_compound(&mut manifold);
        builder.make_compound(&mut non_manifold);
        let only_non_manifold = !is_manifold_shape(shape)?;
        if only_non_manifold {
            non_manifold = topods::compound(shape)?;
        } else {
            let mut pending = vec![shape.clone()];
            let mut manifold_shapes = Vec::new();
            while let Some(parent) = (!pending.is_empty()).then(|| pending.remove(0)) {
                let mut children = ShapeIterator::new(&parent, true, true);
                while children.more() {
                    let sub = children.value()?.clone();
                    children.next();
                    if sub.shape_type()? != ShapeEnum::Compound {
                        manifold_shapes.push(sub);
                    } else if !is_manifold_shape(&sub)? {
                        non_manifold = topods::compound(&sub)?;
                    } else {
                        pending.push(sub);
                    }
                }
            }
            for sub in &manifold_shapes {
                builder.add(&mut manifold, sub)?;
            }
        }
        *shape = manifold.into_shape();
        let mut items = Vec::new();
        let mut group = Vec::new();
        let mut children = ShapeIterator::new(&non_manifold, true, true);
        while children.more() {
            let sub = children.value()?.clone();
            children.next();
            let kind = sub.shape_type()?;
            if kind == ShapeEnum::Solid {
                let mut shells = ShapeIterator::new(&sub, true, true);
                while shells.more() {
                    let shell: Shell = topods::shell(shells.value()?)?;
                    shell.set_closed(true)?;
                    items.push(shell.clone().into_shape());
                    group.push(shell.into_shape());
                    shells.next();
                }
            } else if !is_manifold && kind == ShapeEnum::Shell {
                items.push(sub.clone());
                group.push(sub);
            } else {
                items.push(sub);
            }
        }
        if items.is_empty() {
            return Ok((Handle::default(), false));
        }
        let sdr = if only_non_manifold {
            sdr0.clone()
        } else {
            let model: Handle<StepDataStepModel> = step_live(fp, WHERE)?.model().cast();
            let mut part = StepConstructPart::new();
            self.make_part_sdr(&mut part, &model)?;
            part.sdr_value()
        };
        let binder: Handle<dyn TransferBinder> = self.transient_result(&sdr.cast()).cast();
        let group_handle = shared_sequence(group);
        let scope = ProgressScope::unnamed(progress, items.len() as f64, false);
        for item in &items {
            if !scope.more() {
                break;
            }
            let mapper: Handle<dyn TransferFinder> = TransferBRep::shape_mapper(fp, item)?.cast();
            self.transfer_shape(
                &mapper,
                &sdr,
                fp,
                factors,
                &group_handle,
                false,
                &scope.next(1.0),
            )?;
        }
        Ok((binder, only_non_manifold))
    }
    /// `transfer_shape(start, SDR0, FP, local_factors, shapeGroup, isManifold, progress)`.
    #[allow(clippy::too_many_arguments, reason = "the member takes each of these")]
    pub fn transfer_shape(
        &self,
        start: &Handle<dyn TransferFinder>,
        sdr0: &Handle<StepShapeShapeDefinitionRepresentation>,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        shape_group: &Handle<martensite_tkernel::ncollection::HSequence<Shape>>,
        is_manifold: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        let mapper: Handle<TransferBRepShapeMapper> = start.cast();
        let Some(live_mapper) = mapper.get() else {
            return Ok(Handle::default());
        };
        let mut the_shape = live_mapper.value().clone();
        if the_shape.is_null() {
            return Ok(Handle::default());
        }
        let process = step_live(fp, WHERE)?;
        let model: Handle<StepDataStepModel> = process.model().cast();
        let live_model = step_live(&model, WHERE)?;
        // The reads `binder` here and tests it twice; it is overwritten before its use, and
        // a known result does not stop the transfer.
        if self.is_assembly(&model, &mut the_shape)? {
            return self.transfer_compound(start, sdr0, fp, factors, progress);
        }
        let root = ProgressScope::unnamed(progress, 2.0, false);
        let nm_mode = live_model.internal_parameters().write_nonmanifold;
        let mut nm_binder: Handle<dyn TransferBinder> = Handle::default();
        if nm_mode && self.group_mode() == 0 && the_shape.shape_type()? == ShapeEnum::Compound {
            let range = root.next(1.0);
            let (binder, finished) = self.transfer_non_manifold_part(
                &mut the_shape,
                sdr0,
                fp,
                factors,
                is_manifold,
                &range,
            )?;
            nm_binder = binder;
            if finished {
                return Ok(nm_binder);
            }
        }
        if root.user_break() {
            return Ok(Handle::default());
        }
        let rep_items =
            match self.shapes_to_write(start, &mut the_shape, sdr0, fp, factors, &root)? {
                ShapesToWrite::Items(items) => items,
                ShapesToWrite::Done(binder) => return Ok(binder),
            };
        self.write_representation(
            start,
            sdr0,
            fp,
            factors,
            shape_group,
            is_manifold,
            &the_shape,
            rep_items,
            nm_binder,
            &root,
        )
    }
}

/// A shared sequence of shapes.
pub(super) fn shared_sequence(
    items: Vec<Shape>,
) -> Handle<martensite_tkernel::ncollection::HSequence<Shape>> {
    let sequence = Handle::new(martensite_tkernel::ncollection::HSequence::default());
    if let Some(live) = sequence.get() {
        for item in items {
            live.append(item);
        }
    }
    sequence
}

impl StepControlActorWrite {
    /// The free vertices of a compound: their compound first, and the compound without them.
    /// bug 23950. Returns whether the compound has only vertices.
    fn separate_free_vertices(shape: &mut Shape, items: &mut Vec<Shape>) -> Result<bool, Failure> {
        let builder = BRepBuilder::default();
        let mut rest = Compound::default();
        let mut vertices_compound = Compound::default();
        builder.make_compound(&mut rest);
        builder.make_compound(&mut vertices_compound);
        let mut vertices = Vec::new();
        let only_vertices = separate_solo_vertices(shape, &mut vertices)?;
        if !only_vertices {
            let mut children = ShapeIterator::new(shape, true, true);
            while children.more() {
                let child = children.value()?;
                let mut of_child = Vec::new();
                if separate_solo_vertices(child, &mut of_child)? {
                    vertices.append(&mut of_child);
                } else {
                    builder.add(&mut rest, child)?;
                }
                children.next();
            }
            *shape = rest.into_shape();
        }
        for vertex in &vertices {
            builder.add(&mut vertices_compound, vertex)?;
        }
        if !vertices.is_empty() {
            items.push(vertices_compound.into_shape());
        }
        Ok(only_vertices)
    }
    /// The solids, free shells and free faces of a compound, wires and edges too when as-is.
    fn compound_items(
        shape: &Shape,
        mode: StepControlStepModelType,
        only_vertices: bool,
        items: &mut Vec<Shape>,
    ) -> Result<(), Failure> {
        let mut collect = |find: ShapeEnum, avoid: ShapeEnum| -> Result<(), Failure> {
            let mut exp = Explorer::new(shape, find, avoid)?;
            while exp.more() {
                items.push(exp.current().clone());
                exp.next()?;
            }
            Ok(())
        };
        if mode != StepControlStepModelType::GEOMETRIC_CURVE_SET {
            collect(ShapeEnum::Solid, ShapeEnum::Shape)?;
            collect(ShapeEnum::Shell, ShapeEnum::Solid)?;
            collect(ShapeEnum::Face, ShapeEnum::Shell)?;
        } else if !only_vertices {
            items.push(shape.clone()); // :j1
        }
        if mode == StepControlStepModelType::AS_IS {
            let mut collect = |find: ShapeEnum, avoid: ShapeEnum| -> Result<(), Failure> {
                let mut exp = Explorer::new(shape, find, avoid)?;
                while exp.more() {
                    items.push(exp.current().clone());
                    exp.next()?;
                }
                Ok(())
            };
            collect(ShapeEnum::Wire, ShapeEnum::Face)?;
            collect(ShapeEnum::Edge, ShapeEnum::Wire)?;
        }
        Ok(())
    }
    /// The shapes to translate, per the kind of the shape.
    fn shapes_to_write(
        &self,
        start: &Handle<dyn TransferFinder>,
        shape: &mut Shape,
        sdr0: &Handle<StepShapeShapeDefinitionRepresentation>,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        root: &ProgressScope<'_>,
    ) -> Result<ShapesToWrite, Failure> {
        let process = step_live(fp, WHERE)?;
        let model: Handle<StepDataStepModel> = process.model().cast();
        let separate = step_live(&model, WHERE)?
            .internal_parameters()
            .write_vertex_mode
            .as_raw()
            == 0;
        let mode = self.mode();
        let mut items = Vec::new();
        let mut only_vertices = false;
        if shape.shape_type()? == ShapeEnum::Compound && separate {
            only_vertices = Self::separate_free_vertices(shape, &mut items)?;
        }
        match shape.shape_type()? {
            ShapeEnum::Compound => Self::compound_items(shape, mode, only_vertices, &mut items)?,
            ShapeEnum::Solid | ShapeEnum::Shell | ShapeEnum::Face => items.push(shape.clone()),
            ShapeEnum::CompSolid => {
                let text = "NonManifold COMPSOLID was translated like a set of SOLIDs";
                process.add_warning_text(start, text, "");
                if self.group_mode() > 0 {
                    let binder =
                        self.transfer_compound(start, sdr0, fp, factors, &root.next(1.0))?;
                    return Ok(ShapesToWrite::Done(binder));
                }
                let mut solids = Explorer::new(shape, ShapeEnum::Solid, ShapeEnum::Shape)?;
                while solids.more() {
                    items.push(solids.current().clone());
                    solids.next()?;
                }
            }
            _ if mode != StepControlStepModelType::GEOMETRIC_CURVE_SET
                && mode != StepControlStepModelType::AS_IS =>
            {
                process.add_fail_text(
                    start,
                    "The Shape is not a SOLID, nor a SHELL, nor a FACE",
                    "",
                );
                return Ok(ShapesToWrite::Done(process.find(start)));
            }
            _ => items.push(shape.clone()),
        }
        Ok(ShapesToWrite::Items(items))
    }
}
