//! `StepControlActorWrite`: one shape to its representation item.
use super::StepControlActorWrite;
use crate::de_step::StepControlStepModelType;
use crate::geom_to_step::empty_step_name;
use crate::step_data::{StepDataFactors, StepDataSelectType, StepDataStepModel};
use crate::step_entity::{step_array1_set, step_live, step_new_array1};
use crate::step_geom::StepGeomGeometricRepresentationItem;
use crate::step_repr::StepReprRepresentationItem;
use crate::step_shape::{
    StepShapeAdvancedBrepShapeRepresentation, StepShapeFacetedBrep,
    StepShapeFacetedBrepShapeRepresentation, StepShapeGeometricCurveSet,
    StepShapeGeometricSetSelect, StepShapeGeometricallyBoundedWireframeShapeRepresentation,
    StepShapeManifoldSolidBrep, StepShapeManifoldSurfaceShapeRepresentation,
    StepShapeShapeRepresentation, StepShapeShellBasedSurfaceModel, StepShapeVertexPoint,
};
use crate::step_visual::{StepVisualTessellatedItem, StepVisualTessellatedShapeRepresentation};
use crate::topo_ds_to_step::{
    TopoDSToStepFacetedError, TopoDSToStepFacetedTool, TopoDSToStepMakeBrepWithVoids,
    TopoDSToStepMakeFacetedBrep, TopoDSToStepMakeFacetedBrepAndBrepWithVoids,
    TopoDSToStepMakeGeometricCurveSet, TopoDSToStepMakeManifoldSolidBrep,
    TopoDSToStepMakeShellBasedSurfaceModel, TopoDSToStepMakeStepVertex, TopoDSToStepShapeMap,
    TopoDSToStepTool,
};
use martensite_tkbrep::brep::{BRepTEdge, BRepTFace};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape, ShapeIterator};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_analysis::ShapeAnalysisShapeTolerance;
use martensite_tkxsbase::interface::InterfaceMsg;
use martensite_tkxsbase::transfer::{
    TransferActorOps, TransferBinder, TransferFinder, TransferFinderProcess,
};
use martensite_tkxsbase::transfer_brep::TransferBRep;

/// `transferVertex`.
pub(super) fn transfer_vertex(
    fp: &Handle<TransferFinderProcess>,
    list: &Handle<HArray1<StepShapeGeometricSetSelect>>,
    vertex: &Shape,
    rank: i32,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    let model: Handle<StepDataStepModel> = step_live(fp, WHERE)?.model().cast();
    let mode = step_live(&model, WHERE)?
        .internal_parameters()
        .write_surface_cur_mode;
    let mut tool = TopoDSToStepTool::with_map(&TopoDSToStepShapeMap::default(), true, mode);
    let maker =
        TopoDSToStepMakeStepVertex::with_vertex(&topods::vertex(vertex)?, &mut tool, fp, factors)?;
    if !maker.is_done() {
        return Ok(());
    }
    let vertex_point: Handle<StepShapeVertexPoint> = tool.find(vertex)?.cast();
    let Some(live) = vertex_point.get() else {
        return Ok(());
    };
    let mut select = StepShapeGeometricSetSelect::default();
    select.set_value(&live.vertex_geometry().cast())?;
    step_array1_set(list, rank, select, WHERE)
}

/// The geometric_curve_set of the vertices of a compound or a vertex.
pub(super) fn vertices_curve_set(
    shape: &Shape,
    fp: &Handle<TransferFinderProcess>,
    factors: &StepDataFactors,
) -> Result<Handle<StepGeomGeometricRepresentationItem>, Failure> {
    let mut vertices = Vec::new();
    let mut exp = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    while exp.more() {
        vertices.push(exp.current().clone());
        exp.next()?;
    }
    if vertices.is_empty() {
        return Ok(Handle::default());
    }
    let list = step_new_array1::<StepShapeGeometricSetSelect>(1, vertices.len() as i32)?;
    let set = Handle::new(StepShapeGeometricCurveSet::new());
    set.set_name(&empty_step_name());
    for (index, vertex) in vertices.iter().enumerate() {
        transfer_vertex(fp, &list, vertex, index as i32 + 1, factors)?;
    }
    set.set_elements(&list);
    Ok(set.cast())
}

/// The item and the tessellated item of one translated shape.
#[derive(Default)]
pub(super) struct TranslatedItem {
    pub(super) item: Handle<StepGeomGeometricRepresentationItem>,
    pub(super) tessellated: Handle<StepGeomGeometricRepresentationItem>,
}

macro_rules! translated_from {
    ($maker:expr, $tolerance:expr) => {{
        let mut maker = $maker;
        *maker.root.tolerance_mut() = $tolerance;
        if maker.is_done() {
            Some(TranslatedItem {
                item: maker.value()?.cast(),
                tessellated: maker.tessellated_value()?.cast(),
            })
        } else {
            None
        }
    }};
}

/// `ManifoldSolidBrep` mode on a solid: a brep_with_voids when it has more shells, else manifold.
pub(super) fn translate_manifold_solid(
    shape: &Shape,
    fp: &Handle<TransferFinderProcess>,
    factors: &StepDataFactors,
    tolerance: f64,
    progress: &ProgressRange<'_>,
) -> Result<TranslatedItem, Failure> {
    let solid = topods::solid(shape)?;
    let mut nb_shells = 0;
    let mut children = ShapeIterator::new(&solid, true, true);
    while children.more() {
        if children.value()?.shape_type()? == ShapeEnum::Shell {
            nb_shells += 1;
        }
        children.next();
    }
    if nb_shells > 1 {
        let made = translated_from!(
            TopoDSToStepMakeBrepWithVoids::new(&solid, fp, factors, progress)?,
            tolerance
        );
        match made {
            Some(found) => return Ok(found),
            None => nb_shells = 1, // smth went wrong; let it will be just Manifold
        }
    }
    if nb_shells == 1 {
        let made = translated_from!(
            TopoDSToStepMakeManifoldSolidBrep::with_solid(&solid, fp, factors, progress)?,
            tolerance
        );
        return Ok(made.unwrap_or_default());
    }
    Ok(TranslatedItem::default())
}

/// The faceted checks: `None` when the shape cannot be faceted, the failure recorded.
pub(super) fn check_faceted(
    shape: &Shape,
    start: &Handle<dyn TransferFinder>,
    fp: &Handle<TransferFinderProcess>,
) -> Result<bool, Failure> {
    let error = TopoDSToStepFacetedTool::check_topo_ds_shape(shape)?;
    if error == TopoDSToStepFacetedError::FacetedDone {
        return Ok(true);
    }
    let process = step_live(fp, WHERE)?;
    process.add_fail_text(start, "Error in Faceted Shape from TopoDS", "");
    if error == TopoDSToStepFacetedError::SurfaceNotPlane {
        process.add_fail_text(start, "-- The Face is not plane", "");
    } else if error == TopoDSToStepFacetedError::PCurveNotLinear {
        process.add_fail_text(start, "-- The Face contains non linear PCurves", "");
    }
    Ok(false)
}

/// The items of one shape in one mode; `None` when the transfer stops with the failure recorded.
#[allow(clippy::too_many_arguments, reason = "the switch reads each of these")]
pub(super) fn translate_shape_item(
    trmode: StepControlStepModelType,
    shape: &Shape,
    start: &Handle<dyn TransferFinder>,
    fp: &Handle<TransferFinderProcess>,
    factors: &StepDataFactors,
    tolerance: f64,
    progress: &ProgressRange<'_>,
) -> Result<Option<TranslatedItem>, Failure> {
    let kind = shape.shape_type()?;
    let made = match trmode {
        StepControlStepModelType::MANIFOLD_SOLID_BREP => match kind {
            ShapeEnum::Solid => translate_manifold_solid(shape, fp, factors, tolerance, progress)?,
            ShapeEnum::Shell => translated_from!(
                TopoDSToStepMakeManifoldSolidBrep::with_shell(
                    &topods::shell(shape)?,
                    fp,
                    factors,
                    progress
                )?,
                tolerance
            )
            .unwrap_or_default(),
            _ => TranslatedItem::default(),
        },
        StepControlStepModelType::BREP_WITH_VOIDS if kind == ShapeEnum::Solid => translated_from!(
            TopoDSToStepMakeBrepWithVoids::new(&topods::solid(shape)?, fp, factors, progress)?,
            tolerance
        )
        .unwrap_or_default(),
        StepControlStepModelType::FACETED_BREP => {
            if !check_faceted(shape, start, fp)? {
                return Ok(None);
            }
            if kind == ShapeEnum::Solid {
                translated_from!(
                    TopoDSToStepMakeFacetedBrep::with_solid(
                        &topods::solid(shape)?,
                        fp,
                        factors,
                        progress
                    )?,
                    tolerance
                )
                .unwrap_or_default()
            } else {
                TranslatedItem::default()
            }
        }
        StepControlStepModelType::FACETED_BREP_AND_BREP_WITH_VOIDS => {
            if !check_faceted(shape, start, fp)? {
                return Ok(None);
            }
            if kind == ShapeEnum::Solid {
                translated_from!(
                    TopoDSToStepMakeFacetedBrepAndBrepWithVoids::new(
                        &topods::solid(shape)?,
                        fp,
                        factors,
                        progress
                    )?,
                    tolerance
                )
                .unwrap_or_default()
            } else {
                TranslatedItem::default()
            }
        }
        StepControlStepModelType::SHELL_BASED_SURFACE_MODEL => match kind {
            ShapeEnum::Solid => translated_from!(
                TopoDSToStepMakeShellBasedSurfaceModel::with_solid(
                    &topods::solid(shape)?,
                    fp,
                    factors,
                    progress
                )?,
                tolerance
            )
            .unwrap_or_default(),
            ShapeEnum::Shell => translated_from!(
                TopoDSToStepMakeShellBasedSurfaceModel::with_shell(
                    &topods::shell(shape)?,
                    fp,
                    factors,
                    progress
                )?,
                tolerance
            )
            .unwrap_or_default(),
            ShapeEnum::Face => translated_from!(
                TopoDSToStepMakeShellBasedSurfaceModel::with_face(
                    &topods::face(shape)?,
                    fp,
                    factors,
                    progress
                )?,
                tolerance
            )
            .unwrap_or_default(),
            _ => TranslatedItem::default(),
        },
        StepControlStepModelType::GEOMETRIC_CURVE_SET => {
            let mut maker = TopoDSToStepMakeGeometricCurveSet::new(shape, fp, factors)?;
            *maker.root.tolerance_mut() = tolerance;
            if maker.is_done() {
                TranslatedItem {
                    item: maker.value()?.cast(),
                    tessellated: Handle::default(),
                }
            } else if matches!(kind, ShapeEnum::Compound | ShapeEnum::Vertex) {
                TranslatedItem {
                    item: vertices_curve_set(shape, fp, factors)?,
                    tessellated: Handle::default(),
                }
            } else {
                TranslatedItem::default()
            }
        }
        _ => TranslatedItem::default(),
    };
    Ok(Some(made))
}

/// Binds `item` to the sub-shape: the start mapper for the shape itself, else its own.
pub(super) fn bind_item_result(
    actor: &StepControlActorWrite,
    item: &Handle<StepGeomGeometricRepresentationItem>,
    shape: &Shape,
    start: &Handle<dyn TransferFinder>,
    original: &Shape,
    fp: &Handle<TransferFinderProcess>,
) -> Result<(), Failure> {
    let process = step_live(fp, WHERE)?;
    let mapper: Handle<dyn TransferFinder> = if shape.is_same(original) {
        start.clone()
    } else {
        TransferBRep::shape_mapper(fp, shape)?.cast()
    };
    let binder = process.find(&mapper);
    let result: Handle<dyn TransferBinder> = actor.transient_result(&item.cast()).cast();
    match binder.get() {
        None => process.bind(&mapper, &result),
        Some(live) => {
            live.add_result(&result);
            Ok(())
        }
    }
}

/// The transfer mode of one shape when writing as-is.
pub(super) fn as_is_mode(shape: &Shape) -> Result<StepControlStepModelType, Failure> {
    Ok(match shape.shape_type()? {
        ShapeEnum::Solid => StepControlStepModelType::MANIFOLD_SOLID_BREP,
        ShapeEnum::Shell | ShapeEnum::Face => StepControlStepModelType::SHELL_BASED_SURFACE_MODEL,
        _ => StepControlStepModelType::GEOMETRIC_CURVE_SET,
    })
}

/// The shape_representation class of an item of a split AP203 representation.
pub(super) fn representation_of_item(
    item: &Handle<StepReprRepresentationItem>,
) -> Handle<StepShapeShapeRepresentation> {
    if item.cast::<StepShapeManifoldSolidBrep>().is_some() {
        Handle::new(StepShapeAdvancedBrepShapeRepresentation::new()).cast()
    } else if item.cast::<StepShapeShellBasedSurfaceModel>().is_some() {
        Handle::new(StepShapeManifoldSurfaceShapeRepresentation::new()).cast()
    } else if item.cast::<StepShapeGeometricCurveSet>().is_some() {
        Handle::new(StepShapeGeometricallyBoundedWireframeShapeRepresentation::new()).cast()
    } else if item.cast::<StepShapeFacetedBrep>().is_some() {
        Handle::new(StepShapeFacetedBrepShapeRepresentation::new()).cast()
    } else if item.cast::<StepVisualTessellatedItem>().is_some() {
        Handle::new(StepVisualTessellatedShapeRepresentation::new()).cast()
    } else {
        Handle::new(StepShapeShapeRepresentation::new()).cast()
    }
}

/// `hasGeometry`: a curve or surface behind every edge and face of the shape.
pub(super) fn has_geometry(shape: &Shape) -> Result<bool, Failure> {
    match shape.shape_type()? {
        ShapeEnum::Vertex => Ok(true),
        ShapeEnum::Edge => {
            let edge: Handle<BRepTEdge> = shape.tshape().cast();
            let live = step_live(&edge, WHERE)?;
            for representation in live.curves() {
                let live_rep = step_live(&representation, WHERE)?;
                let has_curve3d =
                    live_rep.is_curve3d() && live_rep.curve3d().is_ok_and(|c| c.is_some());
                let has_polygon =
                    live_rep.is_polygon3d() && live_rep.polygon3d().is_ok_and(|p| p.is_some());
                if !(has_curve3d
                    || live_rep.is_curve_on_surface()
                    || live_rep.is_regularity()
                    || has_polygon
                    || live_rep.is_polygon_on_triangulation()
                    || live_rep.is_polygon_on_surface())
                {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        ShapeEnum::Face => {
            let face: Handle<BRepTFace> = shape.tshape().cast();
            Ok(step_live(&face, WHERE)?.surface().is_some())
        }
        _ => {
            let mut children = ShapeIterator::new(shape, false, false);
            while children.more() {
                if !has_geometry(children.value()?)? {
                    return Ok(false);
                }
                children.next();
            }
            Ok(true)
        }
    }
}

/// `used_tolerance`.
pub(super) fn used_tolerance(
    model: &StepDataStepModel,
    mytoler: f64,
    shape: &Shape,
) -> Result<f64, Failure> {
    let mut tolerance = mytoler;
    let (mode, value) = {
        let parameters = model.internal_parameters();
        (
            parameters.write_precision_mode.as_raw(),
            parameters.write_precision_val,
        )
    };
    if tolerance <= 0.0 && mode == 2 {
        tolerance = value;
    }
    if tolerance <= 0.0 {
        let mut analysis = ShapeAnalysisShapeTolerance::new();
        let measured = analysis.tolerance(shape, mode, ShapeEnum::Shape)?;
        // arrondi a 1 2 5 ...
        tolerance = InterfaceMsg::intervalled(measured * 1.5, 3, false);
    }
    if tolerance == 0.0 {
        tolerance = 1.0e-07; // minimum ...
    }
    Ok(tolerance)
}

const WHERE: &str = "StepControlActorWrite";
